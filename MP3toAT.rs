use std::fs::File;
use std::io::{BufReader, Result};
use hound::{WavWriter, WavSpec, SampleFormat};
use puremp3::Mp3Decoder;

fn main() -> Result<()> {
    let mp3_file = "Chapter0000.mp3"; // Path to the input MP3 file
    let wav_file = "morse_code.wav"; // Path to the output WAV file

    // Convert MP3 to WAV
    let audio_data = convert_mp3_to_wav(mp3_file, wav_file)?;

    // Extract audio data from WAV file
    //let audio_data = extract_audio_data(wav_file)?;

    // Generate ATDT command from audio data
    let atdt_command = generate_atdt_command(audio_data)?;

    println!("ATDT command: {}", atdt_command);

    Ok(())
}

fn convert_mp3_to_wav(mp3_file: &str, wav_file: &str) -> Result<Vec<[Vec<f64>; 1]>> {
    let mp3_file = File::open(mp3_file)?;
    let mut decoder = Mp3Decoder::new(BufReader::new(mp3_file));

    // Create WAV writer with appropriate specifications
    let mut wav_writer = WavWriter::create(wav_file, WavSpec {
        channels: 1,
        sample_rate: 11025,
        bits_per_sample: 32,
        sample_format: SampleFormat::Float,
    }).unwrap();

    let mut audio_segments = Vec::new();
    let mut data = Vec::new();
    // Decode MP3 frames and write to WAV
    while let Ok(frame) = decoder.next_frame() {
        // Write the samples to the WAV file
         let mut skip:bool = false;
        for mut sample in frame.samples {
            if !skip{
                if sample[0] > -0.001 && sample[0] < 0.001{
                    sample[0] = 0.0;
                }
                if sample[0] > 0.0{
                    data.push(0.50);
                    wav_writer.write_sample(0.50).unwrap();
                } else if sample[0] < 0.0{
                    data.push(-0.50);
                    wav_writer.write_sample(-0.50).unwrap();
                } else{
                    wav_writer.write_sample(0.0).unwrap();
                    //segment between zeros
                    audio_segments.push([data.clone()]);
                    data.clear();
                }
            }
            if !skip{
                skip = true;
            } else {
                skip = false;
            }
        }
    }

    Ok(audio_segments)
}
/*
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
*/

fn check_dash_presence(audio:&Vec<f64>,morse_code_mapping:[char; 2]) -> String{
    let mut previous_sample = 0.0;
    let mut count_to_zero_dash = 0;
    let mut atdt_command:String = String::new();

    for sample in audio{
        //DASH conditions
        if *sample != previous_sample && *sample > 0.0{
            count_to_zero_dash = count_to_zero_dash + 1;
        }

        if count_to_zero_dash >= 2{
            atdt_command.push(morse_code_mapping[1]);
            count_to_zero_dash = 0;
        }
        previous_sample = *sample;
    }
    atdt_command
}


fn generate_atdt_command(audio_segment: Vec<[Vec<f64>; 1]>) -> Result<String> {
    let mut atdt_command = String::new();

    // Define Morse code mapping
    let morse_code_mapping: [char; 2] = ['.', '-'];
    let mut added_atdt;
    // Iterate over audio data and generate ATDT command
    for segment in audio_segment {
        for audio in segment{
            added_atdt = atdt_command.len();
            atdt_command += &check_dash_presence(&audio, morse_code_mapping);
            if atdt_command.len() == added_atdt{ //no dash present add dots.
                //println!("Added {} dots", audio.len());
                for _sample in audio{
                    atdt_command.push(morse_code_mapping[0]);
                }
            } else {
                println!("Added {} dash", atdt_command.len() - added_atdt)
            }
        }
    }
    Ok(atdt_command)
}