use log;
use std::fs::File;
use std::error::Error;
use std::str;
use std::io::{self, BufRead, BufReader, Write};


fn merge_encodings(input: Vec<Vec<u8>>) -> Vec<u8> {
    let mut output = Vec::new();

    for mut enc in input {
        if enc.len() < 2 {
            continue;
        }

        let output_len = output.len();

        if output_len < 2 {
            output.append(&mut enc);
            continue;
        }

        if output[output_len - 2] == enc[0] {
            output[output_len - 1] = output[output_len - 1] + enc[1];
            enc.drain(0..=1);
        }

        output.append(&mut enc);
    }

    return output
}


fn encode_files(files: Vec<String>) -> Vec<u8> {
    const BUFFER_SIZE: usize = 4096;

    let mut output: Vec<Vec<u8>> = Vec::new();

    for file in files {
        let file       = File::open(file).unwrap();
        let mut reader = BufReader::with_capacity(BUFFER_SIZE, file);
        let mut buffer = reader.fill_buf().unwrap();

        output.push(encode_str(str::from_utf8(&buffer).unwrap()));

        let length = buffer.len();
        buffer.consume(length);
    }
 
    return merge_encodings(output);
}

fn encode_str(input: &str) -> Vec<u8> {
    let mut output: Vec<u8> = Vec::new();
    let mut prevchar: char  = input.chars().nth(0).unwrap();
    let mut charcount: u8   = 0;

    for c in input.chars() {
        if c == prevchar { 
            charcount = charcount + 1;
            continue;
        }

        output.push(prevchar as u8);
        output.push(charcount);
        prevchar = c;
        charcount = 1;
    }

    if charcount > 0 { // push last character count into output
        output.push(prevchar as u8);
        output.push(charcount);
    }

    return output;
}

pub fn run_encoder(files: Vec<String>, jobs: u8) -> Result<(), Box<dyn Error>> {
    if jobs == 1 {
        let enc: Vec<u8> = encode_files(files);
        log::trace!("{:?}", &enc);

        io::stdout().write(&enc).unwrap();
        Ok(())
    }
    else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::remove_file;

    #[test]
    fn test_encode_str() {
        assert_eq!(encode_str("aaaaabbbbcccdda"), [97, 5, 98, 4, 99, 3, 100, 2, 97, 1]);
    }

    #[test]
    fn test_merge_encodings() {
        assert_eq!(merge_encodings(vec!(vec!(98, 4, 97, 1), vec!(97, 1, 98, 4))), [98, 4, 97, 2, 98, 4]);
    }

    #[test]
    fn test_encode_files() {
        let mut file1 = File::create("foo.txt").unwrap();
        let mut file2 = File::create("bar.txt").unwrap();
        let mut file3 = File::create("baz.txt").unwrap();

        file1.write(b"aaaaabb").unwrap();
        file2.write(b"bbcccd").unwrap();
        file3.write(b"da").unwrap();

        let files = vec!("foo.txt".to_string(), "bar.txt".to_string(), "baz.txt".to_string());

        assert_eq!(encode_files(files), [97, 5, 98, 4, 99, 3, 100, 2, 97, 1]);

        remove_file("foo.txt").unwrap();
        remove_file("bar.txt").unwrap();
        remove_file("baz.txt").unwrap();
    }
}