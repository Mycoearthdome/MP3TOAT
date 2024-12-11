use hound::{SampleFormat, WavSpec, WavWriter};
use std::fs;
use std::io::{BufReader, Result, Write};
use std::collections::HashMap;
use std::process::exit;
use clap::Parser;
use base64::{Engine as _, engine::general_purpose};

#[derive(Parser)]
struct Args {
    #[clap(short, long)]
    input: String,
}

fn main() -> Result<()> {

    let args = Args::parse();

    let base64_morse_code_map: HashMap<char, &'static str> = [
    // Uppercase letters
    ('A', ".-"),
    ('B', "-..."),
    ('C', "-.-."),
    ('D', "-.."),
    ('E', "."),         // Original
    ('F', "..-."),
    ('G', "--."),
    ('H', "...."),
    ('I', ".."),
    ('J', ".---"),
    ('K', "-.-"),
    ('L', ".-.."),     // Original
    ('M', "--"),
    ('N', "-."),
    ('O', "---"),
    ('P', ".--."),
    ('Q', "--.-"),
    ('R', ".-."),
    ('S', "..."),
    ('T', "-"),
    ('U', "..-"),
    ('V', "...-"),
    ('W', ".--"),
    ('X', "-..-"),
    ('Y', "-.--"),
    ('Z', "--.."),

    // Digits (0-9)
    ('0', "-----"),
    ('1', ".----"),
    ('2', "..---"),
    ('3', "...--"),
    ('4', "....-"),
    ('5', "....."),
    ('6', "-...."),
    ('7', "--..."),
    ('8', "---..-"),    // Updated for '8'
    ('9', "----."),

    // Special characters
    ('.', ".-.-.-"),
    (',', "--..--"),
    ('?', "..--.."),
    ('\'', ".----."),
    ('!', "-.-.--"),
    ('/', "-..-."),  // Standard BASE64 '/'
    ('-', "-....-"), // URL-safe BASE64 '-'
    ('(', "-.--."),
    (')', "-.--.-"),
    ('&', ".-..."),
    (':', "---..."),
    (';', "-.-.-."),
    ('=', "-...-"),
    ('+', ".-.-."),  // Standard BASE64 '+'
    ('_', "..--.-"), // URL-safe BASE64 '_'
    ('"', "..-..--"), // Updated for '"'
    ('$', "...-..-"),
    ('@', ".--.-."), // Remains the same

    // Lowercase letters with unique Morse code
    ('a', "--.-."),     // Unique Morse for lowercase 'a'
    ('b', "-.-.-"),     // Unique Morse for lowercase 'b'
    ('c', ".--.."),     // Unique Morse for lowercase 'c'
    ('d', "-..-.."),    // Unique Morse for lowercase 'd'
    ('e', "..-..."),     // Updated for lowercase 'e'
    ('f', "---.-"),     // Unique Morse for lowercase 'f'
    ('g', ".---."),     // Unique Morse for lowercase 'g'
    ('h', "..-.-"),     // Unique Morse for lowercase 'h'
    ('i', "..-..-."),   // Updated for lowercase 'i'
    ('j', "..-.."),     // Unique Morse for lowercase 'j'
    ('k', "-.-..-"),    // Updated for lowercase 'k'
    ('l', "---..-."),    // Updated for lowercase 'l'
    ('m', ".-.-.."),    // Unique Morse for lowercase 'm'
    ('n', "-..-.-"),    // Unique Morse for lowercase 'n'
    ('o', "---.."),     // Updated for lowercase 'o'
    ('p', "-..--."),    // Unique Morse for lowercase 'p'
    ('q', "..-.-."),    // Unique Morse for lowercase 'q'
    ('r', ".-..-"),     // Unique Morse for lowercase 'r'
    ('s', "--.--."),    // Unique Morse for lowercase 's'
    ('t', "..--.-."),   // Unique Morse for lowercase 't'
    ('u', ".-.-..-"),   // Unique Morse for lowercase 'u'
    ('v', "-...-."),    // Unique Morse for lowercase 'v'
    ('w', ".--..-."),   // Updated for lowercase 'w'
    ('x', "--..-."),    // Unique Morse for lowercase 'x'
    ('y', "..-.---"),    // Unique Morse for lowercase 'y'
    ('z', ".-.-.-."),   // Unique Morse for lowercase 'z'
].iter().cloned().collect();

    let inverted_base64_map: HashMap<&&str, &char> = base64_morse_code_map.iter().map(|(k, v)| (v, k)).collect();
    if inverted_base64_map.len() < base64_morse_code_map.len(){
        for (key, value) in base64_morse_code_map.clone(){
            if **inverted_base64_map.get(&value).unwrap() != key{
                println!("MISING-->{}",key);
            }
        }
    }

    let input_filename = &args.input;

    if input_filename.find(".wav") == None{
        base64_to_wav(input_filename, base64_morse_code_map.clone());
    } else {
        extract_audio_data(input_filename, inverted_base64_map);
    }
   
    Ok(())
}

fn base64_to_wav(input_file: &str,base64_hashmap:HashMap<char, &'static str>){
    let mut wav_file = input_file.to_string();
    let sample_rate = 44100;
    wav_file.push_str(".wav");


    // Create WAV writer with appropriate specifications
    let mut wav_writer = WavWriter::create(
        &wav_file,
        WavSpec {
            channels: 1,
            sample_rate: sample_rate, //44100
            bits_per_sample: 32,
            sample_format: SampleFormat::Float,
        },
    )
    .unwrap();
    //let mut final_morse_string  = String::new();
    let input_contents = fs::read(input_file).unwrap();
    let handshake = fax_handshake(sample_rate);

    //FAX handshake
    for shakes in handshake{
        wav_writer.write_sample(shakes).unwrap()
    }

    let base64_contents = general_purpose::STANDARD.encode(input_contents);
    for char in base64_contents.chars(){
        match base64_hashmap.get(&char) {
            Some(morse_code_char_value) => {
                for morse_char in morse_code_char_value.chars(){
                    if morse_char == '-'{
                        wav_writer.write_sample(0.5).unwrap();
                    } else {
                        wav_writer.write_sample(0.0).unwrap();
                    }
                    //final_morse_string.push(morse_char);
               }
               wav_writer.write_sample(-0.5).unwrap();
               //final_morse_string.push(' ');
            }
            None => {
                // handle the case where the key is not present
                println!("The key {} is not present in the map", char);
                exit(1);
            }
        }

    }
}

fn extract_audio_data(wav_filename: &str, base64_hashmap:HashMap<&&str, &char>) {
    let wav_file = fs::File::open(wav_filename).unwrap();
    let mut wav_reader = hound::WavReader::new(BufReader::new(wav_file)).unwrap();
    let mut not_so_hidden_base64 = String::new();
    let mut char = String::new();
    let mut handshake_length = 308700;
    for sample in wav_reader.samples::<f32>() {
        if handshake_length == 0{
            match sample {
                Ok(sampling) => {

                    if sampling > 0.0{
                        char.push('-');
                    } else if sampling < 0.0{
                        //process char back
                        match base64_hashmap.get(&char.as_str()){
                            Some(base64) =>{
                                not_so_hidden_base64.push(**base64);
                            }
                            None =>{
                                println!("File is irrecoverable -- send again");
                                exit(1)
                            }

                        }
                        char.clear();
                    } else{
                        char.push('.');
                    }

                },
                Err(e) => eprintln!("Error reading sample: {}", e),
            }
        } else {
            handshake_length = handshake_length -1;
        }
    }

    let recovered_file = general_purpose::STANDARD.decode(not_so_hidden_base64).unwrap();
    let mut recovered_filename = wav_filename[..(wav_filename.len()-4)].to_string();
    recovered_filename.push_str(".recovered");
    let mut recovered = std::fs::File::create(recovered_filename).unwrap();
    recovered.write_all(&recovered_file).unwrap();
    
    
}

fn fax_handshake(sample_rate: u32) -> Vec<f32> {
    let mut handshake = Vec::new();

    // CED (Calling tone) - 1100 Hz
    let ced_freq = 1100.0;
    let ced_duration = 1.0; // seconds
    let ced_samples = (ced_duration * sample_rate as f32) as usize;
    for i in 0..ced_samples {
        let t = i as f32 / sample_rate as f32;
        let sample = (t * ced_freq * 2.0 * std::f32::consts::PI).sin();
        handshake.push(sample);
    }

    // CNG (Calling tone) - 1100 Hz, 3 seconds
    let cng_freq = 1100.0;
    let cng_duration = 3.0; // seconds
    let cng_samples = (cng_duration * sample_rate as f32) as usize;
    for i in 0..cng_samples {
        let t = i as f32 / sample_rate as f32;
        let sample = (t * cng_freq * 2.0 * std::f32::consts::PI).sin();
        handshake.push(sample);
    }

    // Silence - 1 second
    let silence_duration = 1.0; // seconds
    let silence_samples = (silence_duration * sample_rate as f32) as usize;
    for _ in 0..silence_samples {
        handshake.push(0.0);
    }

    // CED (Answer tone) - 2100 Hz
    let ced_ans_freq = 2100.0;
    let ced_ans_duration = 1.0; // seconds
    let ced_ans_samples = (ced_ans_duration * sample_rate as f32) as usize;
    for i in 0..ced_ans_samples {
        let t = i as f32 / sample_rate as f32;
        let sample = (t * ced_ans_freq * 2.0 * std::f32::consts::PI).sin();
        handshake.push(sample);
    }

    // V.21 (Answer tone) - 2100 Hz, 400 Hz
    let v21_ans_freq1 = 2100.0;
    let v21_ans_freq2 = 400.0;
    let v21_ans_duration = 1.0; // seconds
    let v21_ans_samples = (v21_ans_duration * sample_rate as f32) as usize;
    for i in 0..v21_ans_samples {
        let t = i as f32 / sample_rate as f32;
        let sample = (t * v21_ans_freq1 * 2.0 * std::f32::consts::PI).sin() +
                     (t * v21_ans_freq2 * 2.0 * std::f32::consts::PI).sin();
        handshake.push(sample);
    }

    handshake
}

