use std::fs;
use std::cmp::min;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "rust-hexdump: hexdump command written in Rust")]
#[command(version = "1.0.1")]
#[command(about = "display file contents in hexadecimal, decimal, octal, or ascii", long_about = None)]
pub struct Args {
    /// Filepath of Rust code
    #[arg(short, long, required = true)]
    pub file: String,

    /// Toggle length of file to print
    #[arg(short = 'n', long = "length")]
    pub length: Option<usize>, // especially large positive integers will fail
}

fn main() {
    let args = Args::parse();
    if let Ok(bytes) = fs::read(args.file.clone()) {
        // Determine Length
        let length: usize;
        if let Some(n) = args.length {
            length = min(n, bytes.len());
        } else {
            length = bytes.len();
        }
        // Return immediately if the file is empty
        if length == 0 {
            return
        }

        // Printing Work
        // let mut idx: usize = 0;
        let mut two_byte_hexadecimal = TwoByteHexadecimal::new(length, &bytes);
        loop {
            if let Some(line) = two_byte_hexadecimal.next() {
                println!("{line}");
            } else {
                println!("{:07x}", two_byte_hexadecimal.idx);
                break;
            }
        }
    } else {
        eprintln!("Failed to read file {}.", args.file);
    }
}

// TODO: Create helper functions that return strings of formatted decimal, octal, and ascii respectively
// Create unit tests for each of the helper functions...
struct TwoByteHexadecimal<'a> {
    idx: usize,
    length: usize,
    bytes: &'a Vec<u8>,
}

impl<'a> TwoByteHexadecimal<'a> {
    fn new(length: usize, bytes: &'a Vec<u8>) -> Self {
        TwoByteHexadecimal {idx: 0, length, bytes}
    }
}

impl<'a> Iterator for TwoByteHexadecimal<'a> {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        if self.idx == self.length { // nothing left to print
            return None
        }
        let mut out: String = format!("{:07x} ", self.idx);
        if 8 <= (self.length-self.idx)/2 { 
            for _ in 0..8 {
                out.push_str(&format!("{:x}{:x} ", self.bytes[self.idx+1], self.bytes[self.idx]));
                self.idx += 2;
            }
        } else { // last line to be printed
            while self.idx+1 < self.length {
                out.push_str(&format!("{:x}{:x} ", self.bytes[self.idx+1], self.bytes[self.idx]));
                self.idx += 2;
            }
            if self.idx != self.length { // deal with leftover odd
                out.push_str(&format!("00{:x}", self.bytes[self.idx]));
                self.idx += 1;
            }
        }
        Some(out)
    }
}

// TODO: Add tests for each struct

#[cfg(test)]
mod tests {
    use crate::TwoByteHexadecimal;
    #[test]
    fn test_two_byte_hexadecimal() {
        let bytes: Vec<u8> = "orange juice and banana peel! apple juice and lemon rind!".as_bytes().to_vec();
        // Multiple Lines, Odd Last Line
        let mut mult_iterator = TwoByteHexadecimal::new(bytes.len(), &bytes);
        assert_eq!(mult_iterator.next().unwrap(), "0000000 726f 6e61 6567 6a20 6975 6563 6120 646e ".to_string());
        assert_eq!(mult_iterator.next().unwrap(), "0000010 6220 6e61 6e61 2061 6570 6c65 2021 7061 ".to_string());
        assert_eq!(mult_iterator.next().unwrap(), "0000020 6c70 2065 756a 6369 2065 6e61 2064 656c ".to_string());
        assert_eq!(mult_iterator.next().unwrap(), "0000030 6f6d 206e 6972 646e 0021".to_string());
        assert_eq!(mult_iterator.next(), None);
        // Full Last Line
        let mut full_iterator = TwoByteHexadecimal::new(16, &bytes);
        assert_eq!(full_iterator.next().unwrap(), "0000000 726f 6e61 6567 6a20 6975 6563 6120 646e ".to_string());
        assert_eq!(full_iterator.next(), None);
        // Even Last Line
        let mut even_iterator = TwoByteHexadecimal::new(30, &bytes);
        assert_eq!(even_iterator.next().unwrap(), "0000000 726f 6e61 6567 6a20 6975 6563 6120 646e ".to_string());
        assert_eq!(even_iterator.next().unwrap(), "0000010 6220 6e61 6e61 2061 6570 6c65 2021 ".to_string());
        assert_eq!(even_iterator.next(), None);
    }
}