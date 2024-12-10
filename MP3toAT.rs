use hound::{SampleFormat, WavSpec, WavWriter};
use std::fs;
use std::io::{BufReader, Result};
use std::collections::HashMap;
use std::io::Write;
use std::process::exit;


fn main() -> Result<()> {
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

    let input_base64_filename = "BASE64_Convert.txt";
    let wav_file = "morse_code.wav"; // Path to the output WAV file
    let (morse, base64_content) = base64_to_wav(input_base64_filename, wav_file, base64_morse_code_map.clone());
    let not_so_hidden_base64 = extract_audio_data(wav_file, inverted_base64_map);
    
    let mut f = fs::File::create("OUTPUT.morse")?;
    f.write_all(morse.as_bytes())?;

    if base64_content == not_so_hidden_base64{
        println!("{}", not_so_hidden_base64);
    } else {
        println!("IRRECOVERABLE!");
    }

    Ok(())
}

fn base64_to_wav(base64_file: &str, wav_file: &str,base64_hashmap:HashMap<char, &'static str>) -> (String, String) {
    // Create WAV writer with appropriate specifications
    let mut wav_writer = WavWriter::create(
        wav_file,
        WavSpec {
            channels: 1,
            sample_rate: 48000,
            bits_per_sample: 32,
            sample_format: SampleFormat::Float,
        },
    )
    .unwrap();
    let mut final_morse_string  = String::new();
    let base64_contents = fs::read_to_string(base64_file).unwrap().replace("\n", "");
    for char in base64_contents.chars(){
        match base64_hashmap.get(&char) {
            Some(morse_code_char_value) => {
                for morse_char in morse_code_char_value.chars(){
                    if morse_char == '-'{
                        wav_writer.write_sample(0.5).unwrap();
                    } else {
                        wav_writer.write_sample(0.0).unwrap();
                    }
                    final_morse_string.push(morse_char);
               }
               wav_writer.write_sample(-0.5).unwrap();
               final_morse_string.push(' ');
            }
            None => {
                // handle the case where the key is not present
                println!("The key {} is not present in the map", char);
                exit(1);
            }
        }

    }
   (final_morse_string, base64_contents)
}


fn extract_audio_data(wav_file: &str, base64_hashmap:HashMap<&&str, &char>) -> String {
    let wav_file = fs::File::open(wav_file).unwrap();
    let mut wav_reader = hound::WavReader::new(BufReader::new(wav_file)).unwrap();
    let mut not_so_hidden_base64 = String::new();
    let mut char = String::new();
    for sample in wav_reader.samples::<f32>() {
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
    }

    not_so_hidden_base64
}

