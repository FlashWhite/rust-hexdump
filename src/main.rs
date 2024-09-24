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

    /// Toggle matching with numeric variables
    #[arg(short = 'n', long = "length")]
    pub length: Option<u32>, // especially large positive integers will fail
}

fn main() {
    let args = Args::parse();
    if let Ok(bytes) = fs::read(args.file.clone()) {
        if let Some(length) = args.length {
            println!("Printing {} from {}...", length, args.file);
        } else {
            println!("Printing EVERYTHING from {}!", args.file);
        }
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
