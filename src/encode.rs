use log;
use std::fs::File;
use std::error::Error;
use std::io::{self, Write};
use memmap2::MmapOptions;
use memmap2::Mmap;
use rayon::prelude::*;

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

fn threaded_encode_files(files: Vec<String>, jobs: u8) -> Vec<u8> {
    rayon::ThreadPoolBuilder::new()
        .num_threads(jobs as usize)
        .build_global()
        .unwrap();

    const BUFFER_SIZE: usize = 4096;

    let mut output: Vec<Vec<u8>> = Vec::new();

    for file in files {
        let file = File::open(file).unwrap();
        let mmap = unsafe { MmapOptions::new().map(&file).unwrap() };

        log::debug!("mmap: {:?}", &mmap.len());

        let buffer: Vec<usize> = (0..mmap.len())
            .step_by(BUFFER_SIZE)
            .collect();

        log::debug!("buffer: {:?}", &buffer);

        let par_results: Vec<Vec<u8>> = buffer
            .into_par_iter()
            .map(|x| encode_mmap(&mmap, x as usize, x + BUFFER_SIZE as usize))
            .collect();

        output.push(merge_encodings(par_results));
    }
 
    return merge_encodings(output);
}

fn encode_files(files: Vec<String>) -> Vec<u8> {
    let mut output: Vec<Vec<u8>> = Vec::new();

    for file in files {
        let file = File::open(file).unwrap();
        let mmap = unsafe { MmapOptions::new().map(&file).unwrap() };

        output.push(encode_mmap(&mmap, 0, mmap.len()));
    }
 
    return merge_encodings(output);
}

fn encode_mmap(input: &Mmap, from: usize, to: usize) -> Vec<u8> {
    let mut to = to;
    if to > input.len() { to = input.len(); }

    log::trace!("from-to: {:?}", &input[from..to]);

    let mut output: Vec<u8> = Vec::new();
    let mut prevchar: &u8   = &input[from];
    let mut charcount: u8   = 0;

    for c in input[from..to].iter() {
        if c == prevchar { 
            charcount = charcount + 1;
            continue;
        }

        output.push(*prevchar);
        output.push(charcount);
        prevchar = c;
        charcount = 1;
    }

    if charcount > 0 { // push last character count into output
        output.push(*prevchar);
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
        let enc: Vec<u8> = threaded_encode_files(files, jobs);
        log::trace!("{:?}", &enc);

        io::stdout().write(&enc).unwrap();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::remove_file;

    #[test]
    fn test_encode_mmap() {
        let mut file1 = File::create("boo.txt").unwrap();
        file1.write(b"aaaaabbbbcccdda").unwrap();
        drop(file1);

        let file1 = File::open("boo.txt").unwrap();
        let mmap = unsafe { MmapOptions::new().map(&file1).unwrap() };

        assert_eq!(encode_mmap(&mmap, 0, mmap.len()), [97, 5, 98, 4, 99, 3, 100, 2, 97, 1]);

        remove_file("boo.txt").unwrap();
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

        drop(file1);
        drop(file2);
        drop(file3);

        let files = vec!("foo.txt".to_string(), "bar.txt".to_string(), "baz.txt".to_string());

        assert_eq!(encode_files(files), [97, 5, 98, 4, 99, 3, 100, 2, 97, 1]);

        remove_file("foo.txt").unwrap();
        remove_file("bar.txt").unwrap();
        remove_file("baz.txt").unwrap();
    }

    #[test]
    fn test_threaded_encode_files() {
        env_logger::init();
        let mut file1 = File::create("zoo.txt").unwrap();
        let mut file2 = File::create("zar.txt").unwrap();
        let mut file3 = File::create("zaz.txt").unwrap();

        file1.write(b"aaaaabb").unwrap();
        file2.write(b"bbcccd").unwrap();
        file3.write(b"da").unwrap();

        drop(file1);
        drop(file2);
        drop(file3);

        let files = vec!("zoo.txt".to_string(), "zar.txt".to_string(), "zaz.txt".to_string());

        assert_eq!(threaded_encode_files(files, 4), [97, 5, 98, 4, 99, 3, 100, 2, 97, 1]);

        remove_file("zoo.txt").unwrap();
        remove_file("zar.txt").unwrap();
        remove_file("zaz.txt").unwrap();
    }
}
