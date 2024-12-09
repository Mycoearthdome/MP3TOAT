use hound::{SampleFormat, WavSpec, WavWriter};
use puremp3::Mp3Decoder;
use std::fs::File;
use std::io::{BufReader, Result};

const MATCH_MORSE_CODE: &str = "-...............--....------.-.......-....-.....--."; //This is a test morse file

fn main() -> Result<()> {
    let mp3_file = "Chapter0000.mp3"; // Path to the input MP3 file
    let wav_file = "morse_code.wav"; // Path to the output WAV file
    let match_morse_code = MATCH_MORSE_CODE.to_string();
    let morse = convert_mp3_to_wav(mp3_file, wav_file);

    if morse.contains(&match_morse_code){
        println!("MATCH!!!--->OK!");
    } else {
        println!("NO MATCH!");
    }
    //println!("[{}] <--> [{}]", morse, match_morse_code);

    Ok(())
}

fn convert_mp3_to_wav(mp3_file: &str, wav_file: &str) -> String {
    let mp3_file = File::open(mp3_file).unwrap();
    let mut decoder = Mp3Decoder::new(BufReader::new(mp3_file));

    // Create WAV writer with appropriate specifications
    let mut wav_writer = WavWriter::create(
        wav_file,
        WavSpec {
            channels: 1,
            sample_rate: 11025,
            bits_per_sample: 32,
            sample_format: SampleFormat::Float,
        },
    )
    .unwrap();

    //let mut empty = Vec::new();
    let mut sample_buffer = Vec::new();
    // Decode MP3 frames and write to WAV
    while let Ok(frame) = decoder.next_frame() {
        // Write the samples to the WAV file
        //let mut skip:bool = false;
        //if !denied {

        let mut start_sample = frame.samples[0][0]; //MONO CHANNEL. CH1  START

        if start_sample > 0.0 {
            start_sample = 0.50;
        } else if start_sample < 0.0 {
            start_sample = -0.50; //-
        } else {
            //data.push(0.0);
            //wav_writer.write_sample(-0.5).unwrap();
            //audio_segments.push([empty.clone()]);
            start_sample = 0.0;
        }

        sample_buffer.push(start_sample);
    }

    let morse = vector_to_morse(sample_buffer);

    println!("MORSE={}", morse);

    morse
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

fn vector_to_morse(data: Vec<f32>) -> String {
    let mut result = String::new();

    for (idx, value) in data.iter().enumerate() {
        for i in idx..data.len()-1{
            if *value > 0.0 {
                if idx >= 1 && idx < data.len() - 1 {
                    if (data[idx - 1] < *value && *value > data[idx + 1])
                        || (data[idx - 1] > *value && *value < data[idx + 1]){
                    
                        result.push('-');
                    }

                    if (data[i - 1] < *value && *value > data[i + 1])
                    || (data[i - 1] > *value && *value < data[i + 1]){
                            
                        result.push('.');
                    }
                } 
            }
        }
    }
    result
}
