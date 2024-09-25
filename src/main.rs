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
        // DETERMINE LENGTH
        let length: usize;
        if let Some(n) = args.length {
            length = min(n, bytes.len());
        } else {
            length = bytes.len();
        }

        // PRINT WORK
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

// How to deal with multi arg printing? Ex: 
// ➜ hexdump -n 32 -b -c html.txt
// 0000000 131 157 165 040 143 141 156 047 164 040 160 141 162 163 145 040
// 0000000   Y   o   u       c   a   n   '   t       p   a   r   s   e
// 0000010 133 130 135 110 124 115 114 040 167 151 164 150 040 162 145 147
// 0000010   [   X   ]   H   T   M   L       w   i   t   h       r   e   g
// 0000020
