use std::fs;
use clap::Parser;
// use clap::ValueEnum;

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
            length = n;
        } else {
            length = bytes.len();
        }

        // PRINT WORK
        let mut idx: usize = 0;
        while idx < length {
            print!("{:07x} ", idx);
            if 8 <= (length-idx)/2 { 
                for _ in 0..8 {
                    print!("{:x}{:x} ", bytes[idx+1], bytes[idx]);
                    idx += 2;
                }
            } else { // last line to be printed
                while idx+1 < length {
                    print!("{:x}{:x} ", bytes[idx+1], bytes[idx]);
                    idx += 2;
                }
                if idx != length { // deal with leftover odd
                    print!("00{:x}", bytes[idx]);
                    idx += 1;
                }
            }
            println!();
        }
        println!("{:07x}", idx);
    } else {
        eprintln!("Failed to read file {}.", args.file);
    }
}

// TODO: Create helper functions that return strings of formatted decimal, octal, and ascii respectively
// Create unit tests for each of the helper functions...

// How to deal with multi arg printing? Ex: 
// ➜ hexdump -n 32 -b -c html.txt
// 0000000 131 157 165 040 143 141 156 047 164 040 160 141 162 163 145 040
// 0000000   Y   o   u       c   a   n   '   t       p   a   r   s   e
// 0000010 133 130 135 110 124 115 114 040 167 151 164 150 040 162 145 147
// 0000010   [   X   ]   H   T   M   L       w   i   t   h       r   e   g
// 0000020
