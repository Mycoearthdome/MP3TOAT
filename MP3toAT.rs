use std::fs::File;
use std::io::{BufReader, Result};
use hound::{WavWriter, WavSpec, SampleFormat};
use puremp3::Mp3Decoder;

fn main() -> Result<()> {
    let mp3_file = "Chapter0000.mp3"; // Path to the input MP3 file
    let wav_file = "morse_code.wav"; // Path to the output WAV file

    // Convert MP3 to WAV
    convert_mp3_to_wav(mp3_file, wav_file)?;

    // Extract audio data from WAV file
    let audio_data = extract_audio_data(wav_file)?;

    // Generate ATDT command from audio data
    let atdt_command = generate_atdt_command(&audio_data)?;

    println!("ATDT command: {}", atdt_command);

    Ok(())
}

fn convert_mp3_to_wav(mp3_file: &str, wav_file: &str) -> Result<()> {
    let mp3_file = File::open(mp3_file)?;
    let mut decoder = Mp3Decoder::new(BufReader::new(mp3_file));

    // Create WAV writer with appropriate specifications
    let mut wav_writer = WavWriter::create(wav_file, WavSpec {
        channels: 1,
        sample_rate: 11025,
        bits_per_sample: 32,
        sample_format: SampleFormat::Float,
    }).unwrap();

    // Decode MP3 frames and write to WAV
    while let Ok(frame) = decoder.next_frame() {
        // Write the samples to the WAV file
        for sample in frame.samples {
            wav_writer.write_sample(sample[0]).unwrap();
        }
    }

    Ok(())
}

fn extract_audio_data(wav_file: &str) -> Result<Vec<f32>> {
    let wav_file = File::open(wav_file)?;
    let mut wav_reader = hound::WavReader::new(BufReader::new(wav_file)).unwrap();

    let mut audio_data = Vec::new();
    for sample in wav_reader.samples::<f32>() {
        match sample {
            Ok(x) => audio_data.push(x),
            Err(e) => eprintln!("Error reading sample: {}", e),
        }
    }

    Ok(audio_data)
}

fn generate_atdt_command(audio_data: &[f32]) -> Result<String> {
    let mut atdt_command = String::new();

    // Define Morse code mapping
    let morse_code_mapping = ['.', '-'];

    let sample_threshold_min = -0.0001;
    let sample_threshold_max = 0.0001;

    let sampling_threshold_min = -0.005;
    let sampling_threshold_max = 0.005;

    let mut previous_sampling = 0.0;

    let mut nb_matched_samples = 0;
    let mut previous_jump_value = 0.0;
    let mut jump_value = 0.0;
    let mut impulse = 0.0;
    let mut previous_impulse = 0.0;
    let mut impulse_bridge = 0.0;

    let mut bridge_vector:Vec<(f32, f32)> = Vec::new();
    // Iterate over audio data and generate ATDT command
    for sample in audio_data {
        let mut sampling = *sample;
        if *sample > sample_threshold_min && *sample < sample_threshold_max{
            sampling = 0.0;
        }
        if previous_sampling + sampling == sampling * 2.0 && sampling != 0.0{
            nb_matched_samples = nb_matched_samples + 1;
            if jump_value == 0.0{
                jump_value = jump_value + sampling;
                if sampling < sampling_threshold_min && sampling > sampling_threshold_max{
                    previous_impulse = impulse;
                    impulse = previous_jump_value + jump_value;
                    
                } else {
                    impulse = sampling;
                    impulse_bridge = impulse + previous_impulse;
                }
                previous_jump_value = jump_value;
            }
        } else {
            jump_value = 0.0;
            previous_impulse = impulse;
            impulse = 0.0;
            impulse_bridge = 0.0;
        }

        bridge_vector.push((sampling, impulse_bridge));

        //println!("SAMPLE={} DIFFERENCE={} MATCHED={} JUMP={} IMPULSE={} BRIDGE={}", sampling, previous_sampling+sampling, nb_matched_samples, jump_value, impulse, impulse_bridge);
        
        previous_sampling = sampling;

    }
    let mut negative: bool = false;
    let mut positive: bool = false; //for the first dash --TODO: better than this.
    let mut diagonal;
    let mut skip_count= 0;
    let mut skip : bool = false;

    //let sampling_second_threshold_filter = 0.005;

    for (sampling, bridge) in bridge_vector {
        if bridge != 0.0 {//{&& sampling >= sampling_second_threshold_filter{
            if skip == false { //to trace the diagonal to a dash
                println!("SAMPLING={} BRIDGE={}", sampling, bridge);
                if bridge > 0.0 {
                    if positive == true{
                        diagonal = false;
                    } else {
                        diagonal = true
                    }
                    positive = true;
                    negative = false
                } else {
                    if negative == true{
                        diagonal = false;
                    } else {
                        diagonal = true;
                    }
                    negative = true;
                    positive = false;
                }
                if diagonal == true{
                    atdt_command.push(morse_code_mapping[1]); //dash
                    skip = true;
                    skip_count = 0;
                } else {
                    atdt_command.push(morse_code_mapping[0]); //dot
                }
            }
        }
        if skip == true {
            skip_count = skip_count + 1;
            if skip_count == 3{
                skip = false;
            }
        }
    }

    Ok(atdt_command)
}
