use std::fs;
use std::cmp::min;
use clap::Parser;

use rust_hexdump::*;

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

    /// Print Two-Byte Hexadecimal
    #[arg(short = 'x', long = "two-bytes-hex", action = clap::ArgAction::Count)]
    pub x: u8,

    /// Print Two-Byte Decimal
    #[arg(short = 'd', long = "two-bytes-decimal", action = clap::ArgAction::Count)]
    pub d: u8,

    /// Print One-Byte Octal
    #[arg(short = 'b', long = "one-byte-octal", action = clap::ArgAction::Count)]
    pub b: u8,

    /// Print One-Byte Char (ascii)
    #[arg(short = 'c', long = "one-byte-char", action = clap::ArgAction::Count)]
    pub c: u8,
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
        
        // Fetch Repr Flags
        let mut iterators: Vec<Box<dyn Iterator<Item = String>>> = vec![];
        for arg in std::env::args() {
            match arg.as_str() {
                "-x" | "--two-bytes-hex" => iterators.push(Box::new(TwoByteHexadecimal::new(length, &bytes))),
                "-d" | "--two-bytes-decimal" => iterators.push(Box::new(TwoByteDecimal::new(length, &bytes))),
                "-b" | "--one-byte-octal" => iterators.push(Box::new(OneByteOctal::new(length, &bytes))),
                "-c" | "--one-byte-char" => iterators.push(Box::new(OneByteChar::new(length, &bytes))), 
                _ => {},
            }
        }
        // No repr option flags passed, use default
        if iterators.len() == 0 {
            iterators.push(Box::new(DefaultHexadecimal::new(length, &bytes)));
        }

        // Printing Work
        let mut stop: bool = false;
        loop {
            for iterator in &mut iterators {
                if let Some(line) = iterator.next() {
                    println!("{line}");
                } else { // all other iterators should be done as well
                    println!("{:07x}", length);
                    stop = true;
                    break;
                }
            }
            if stop { break };
        }
    } else {
        eprintln!("Failed to read file {}.", args.file);
    }
}
