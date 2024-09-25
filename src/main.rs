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

// TODO: Create helper functions that return strings of formatted decimal, octal, and ascii respectively
// Create unit tests for each of the helper functions...
struct DefaultHexadecimal<'a> {
    idx: usize,
    length: usize,
    bytes: &'a Vec<u8>,
}

impl<'a> DefaultHexadecimal<'a> {
    fn new(length: usize, bytes: &'a Vec<u8>) -> Self {
        DefaultHexadecimal {idx: 0, length, bytes}
    }
}

impl<'a> Iterator for DefaultHexadecimal<'a> {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        if self.idx == self.length { // nothing left to print
            return None
        }
        let mut out: String = format!("{:07x} ", self.idx);
        if 8 <= (self.length-self.idx)/2 { 
            for _ in 0..8 {
                out.push_str(&format!("{:02x}{:02x} ", self.bytes[self.idx+1], self.bytes[self.idx]));
                self.idx += 2;
            }
        } else { // last line to be printed
            while self.idx+1 < self.length {
                out.push_str(&format!("{:02x}{:02x} ", self.bytes[self.idx+1], self.bytes[self.idx]));
                self.idx += 2;
            }
            if self.idx != self.length { // deal with leftover odd
                out.push_str(&format!("{:04x} ", self.bytes[self.idx]));
                self.idx += 1;
            }
        }
        Some(out)
    }
}


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
        let mut out: String = format!("{:07x}    ", self.idx);
        if 8 <= (self.length-self.idx)/2 { 
            for _ in 0..8 {
                out.push_str(&format!("{:02x}{:02x}    ", self.bytes[self.idx+1], self.bytes[self.idx]));
                self.idx += 2;
            }
        } else { // last line to be printed
            while self.idx+1 < self.length {
                out.push_str(&format!("{:02x}{:02x}    ", self.bytes[self.idx+1], self.bytes[self.idx]));
                self.idx += 2;
            }
            if self.idx != self.length { // deal with leftover odd
                out.push_str(&format!("{:04x}    ", self.bytes[self.idx]));
                self.idx += 1;
            }
        }
        Some(out)
    }
}


struct TwoByteDecimal<'a> {
    idx: usize,
    length: usize,
    bytes: &'a Vec<u8>,
}

impl<'a> TwoByteDecimal<'a> {
    fn new(length: usize, bytes: &'a Vec<u8>) -> Self {
        TwoByteDecimal {idx: 0, length, bytes}
    }
}

impl<'a> Iterator for TwoByteDecimal<'a> {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        if self.idx == self.length { // nothing left to print
            return None
        }
        let mut out: String = format!("{:07x}   ", self.idx);
        if 8 <= (self.length-self.idx)/2 { 
            for _ in 0..8 {
                out.push_str(&format!("{:05}   ", ((self.bytes[self.idx+1] as u16) << 8)+(self.bytes[self.idx] as u16)));
                self.idx += 2;
            }
        } else { // last line to be printed
            while self.idx+1 < self.length {
                out.push_str(&format!("{:05}   ", ((self.bytes[self.idx+1] as u16) << 8)+(self.bytes[self.idx] as u16)));
                self.idx += 2;
            }
            if self.idx != self.length { // deal with leftover odd
                out.push_str(&format!("{:05}   ", self.bytes[self.idx]));
                self.idx += 1;
            }
        }
        Some(out)
    }
}


struct OneByteOctal<'a> {
    idx: usize,
    length: usize,
    bytes: &'a Vec<u8>,
}

impl<'a> OneByteOctal<'a> {
    fn new(length: usize, bytes: &'a Vec<u8>) -> Self {
        OneByteOctal {idx: 0, length, bytes}
    }
}

impl<'a> Iterator for OneByteOctal<'a> {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        if self.idx == self.length { // nothing left to print
            return None
        }
        let mut out: String = format!("{:07x} ", self.idx);
        for _ in 0..min(16, self.length-self.idx) {
            out.push_str(&format!("{:03o} ", self.bytes[self.idx]));
            self.idx += 1;
        }
        Some(out)
    }
}

struct OneByteChar<'a> {
    idx: usize,
    length: usize,
    bytes: &'a Vec<u8>,
}

impl<'a> OneByteChar<'a> {
    fn new(length: usize, bytes: &'a Vec<u8>) -> Self {
        OneByteChar {idx: 0, length, bytes}
    }
}

impl<'a> Iterator for OneByteChar<'a> {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        if self.idx == self.length { // nothing left to print
            return None
        }
        let mut out: String = format!("{:07x}   ", self.idx);
        for _ in 0..min(16, self.length-self.idx) {
            out.push_str(&format!("{}   ", self.bytes[self.idx] as char));
            self.idx += 1;
        }
        Some(out)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_default_hexadecimal() {
        let bytes: Vec<u8> = "orange juice and banana peel! apple juice and lemon rind!".as_bytes().to_vec();
        // Multiple Lines, Odd Last Line
        let mut mult_iterator = DefaultHexadecimal::new(bytes.len(), &bytes);
        assert_eq!(mult_iterator.next().unwrap(), "0000000 726f 6e61 6567 6a20 6975 6563 6120 646e ".to_string());
        assert_eq!(mult_iterator.next().unwrap(), "0000010 6220 6e61 6e61 2061 6570 6c65 2021 7061 ".to_string());
        assert_eq!(mult_iterator.next().unwrap(), "0000020 6c70 2065 756a 6369 2065 6e61 2064 656c ".to_string());
        assert_eq!(mult_iterator.next().unwrap(), "0000030 6f6d 206e 6972 646e 0021 ".to_string());
        assert_eq!(mult_iterator.next(), None);
        // Full Last Line
        let mut full_iterator = DefaultHexadecimal::new(16, &bytes);
        assert_eq!(full_iterator.next().unwrap(), "0000000 726f 6e61 6567 6a20 6975 6563 6120 646e ".to_string());
        assert_eq!(full_iterator.next(), None);
        // Even Last Line
        let mut even_iterator = DefaultHexadecimal::new(30, &bytes);
        assert_eq!(even_iterator.next().unwrap(), "0000000 726f 6e61 6567 6a20 6975 6563 6120 646e ".to_string());
        assert_eq!(even_iterator.next().unwrap(), "0000010 6220 6e61 6e61 2061 6570 6c65 2021 ".to_string());
        assert_eq!(even_iterator.next(), None);
    }

    #[test]
    fn test_two_byte_hexadecimal() {
        let bytes: Vec<u8> = "orange juice and banana peel! apple juice and lemon rind!".as_bytes().to_vec();
        // Multiple Lines, Odd Last Line
        let mut mult_iterator = TwoByteHexadecimal::new(bytes.len(), &bytes);
        assert_eq!(mult_iterator.next().unwrap(), "0000000    726f    6e61    6567    6a20    6975    6563    6120    646e    ".to_string());
        assert_eq!(mult_iterator.next().unwrap(), "0000010    6220    6e61    6e61    2061    6570    6c65    2021    7061    ".to_string());
        assert_eq!(mult_iterator.next().unwrap(), "0000020    6c70    2065    756a    6369    2065    6e61    2064    656c    ".to_string());
        assert_eq!(mult_iterator.next().unwrap(), "0000030    6f6d    206e    6972    646e    0021    ".to_string());
        assert_eq!(mult_iterator.next(), None);
        // Full Last Line
        let mut full_iterator = TwoByteHexadecimal::new(16, &bytes);
        assert_eq!(full_iterator.next().unwrap(), "0000000    726f    6e61    6567    6a20    6975    6563    6120    646e    ".to_string());
        assert_eq!(full_iterator.next(), None);
        // Even Last Line
        let mut even_iterator = TwoByteHexadecimal::new(30, &bytes);
        assert_eq!(even_iterator.next().unwrap(), "0000000    726f    6e61    6567    6a20    6975    6563    6120    646e    ".to_string());
        assert_eq!(even_iterator.next().unwrap(), "0000010    6220    6e61    6e61    2061    6570    6c65    2021    ".to_string());
        assert_eq!(even_iterator.next(), None);
    }

    #[test]
    fn test_two_byte_decimal() {
        let bytes: Vec<u8> = "orange juice and banana peel! apple juice and lemon rind!".as_bytes().to_vec();
        // Multiple Lines, Odd Last Line
        let mut mult_iterator = TwoByteDecimal::new(bytes.len(), &bytes);
        assert_eq!(mult_iterator.next().unwrap(), "0000000   29295   28257   25959   27168   26997   25955   24864   25710   ".to_string());
        assert_eq!(mult_iterator.next().unwrap(), "0000010   25120   28257   28257   08289   25968   27749   08225   28769   ".to_string());
        assert_eq!(mult_iterator.next().unwrap(), "0000020   27760   08293   30058   25449   08293   28257   08292   25964   ".to_string());
        assert_eq!(mult_iterator.next().unwrap(), "0000030   28525   08302   26994   25710   00033   ".to_string());
        assert_eq!(mult_iterator.next(), None);
        // Full Last Line
        let mut full_iterator = TwoByteDecimal::new(16, &bytes);
        assert_eq!(full_iterator.next().unwrap(), "0000000   29295   28257   25959   27168   26997   25955   24864   25710   ".to_string());
        assert_eq!(full_iterator.next(), None);
        // Even Last Line
        let mut even_iterator = TwoByteDecimal::new(30, &bytes);
        assert_eq!(even_iterator.next().unwrap(), "0000000   29295   28257   25959   27168   26997   25955   24864   25710   ".to_string());
        assert_eq!(even_iterator.next().unwrap(), "0000010   25120   28257   28257   08289   25968   27749   08225   ".to_string());
        assert_eq!(even_iterator.next(), None);
    }

    #[test]
    fn test_one_byte_octal() {
        let bytes: Vec<u8> = "orange juice and banana peel! apple juice and lemon rind!".as_bytes().to_vec();
        // Multiple Lines, Odd Last Line
        let mut mult_iterator = OneByteOctal::new(bytes.len(), &bytes);
        assert_eq!(mult_iterator.next().unwrap(), "0000000 157 162 141 156 147 145 040 152 165 151 143 145 040 141 156 144 ".to_string());
        assert_eq!(mult_iterator.next().unwrap(), "0000010 040 142 141 156 141 156 141 040 160 145 145 154 041 040 141 160 ".to_string());
        assert_eq!(mult_iterator.next().unwrap(), "0000020 160 154 145 040 152 165 151 143 145 040 141 156 144 040 154 145 ".to_string());
        assert_eq!(mult_iterator.next().unwrap(), "0000030 155 157 156 040 162 151 156 144 041 ".to_string());
        assert_eq!(mult_iterator.next(), None);
        // Full Last Line
        let mut full_iterator = OneByteOctal::new(16, &bytes);
        assert_eq!(full_iterator.next().unwrap(), "0000000 157 162 141 156 147 145 040 152 165 151 143 145 040 141 156 144 ".to_string());
        assert_eq!(full_iterator.next(), None);
        // Even Last Line
        let mut even_iterator = OneByteOctal::new(30, &bytes);
        assert_eq!(even_iterator.next().unwrap(), "0000000 157 162 141 156 147 145 040 152 165 151 143 145 040 141 156 144 ".to_string());
        assert_eq!(even_iterator.next().unwrap(), "0000010 040 142 141 156 141 156 141 040 160 145 145 154 041 040 ".to_string());
        assert_eq!(even_iterator.next(), None);
    }

    #[test]
    fn test_one_byte_char() {
        let bytes: Vec<u8> = "orange juice and banana peel! apple juice and lemon rind!".as_bytes().to_vec();
        // Multiple Lines, Odd Last Line
        let mut mult_iterator = OneByteChar::new(bytes.len(), &bytes);
        assert_eq!(mult_iterator.next().unwrap(), "0000000   o   r   a   n   g   e       j   u   i   c   e       a   n   d   ".to_string());
        assert_eq!(mult_iterator.next().unwrap(), "0000010       b   a   n   a   n   a       p   e   e   l   !       a   p   ".to_string());
        assert_eq!(mult_iterator.next().unwrap(), "0000020   p   l   e       j   u   i   c   e       a   n   d       l   e   ".to_string());
        assert_eq!(mult_iterator.next().unwrap(), "0000030   m   o   n       r   i   n   d   !   ".to_string());
        assert_eq!(mult_iterator.next(), None);
        // Full Last Line
        let mut full_iterator = OneByteChar::new(16, &bytes);
        assert_eq!(full_iterator.next().unwrap(), "0000000   o   r   a   n   g   e       j   u   i   c   e       a   n   d   ".to_string());
        assert_eq!(full_iterator.next(), None);
        // Even Last Line
        let mut even_iterator = OneByteChar::new(30, &bytes);
        assert_eq!(even_iterator.next().unwrap(), "0000000   o   r   a   n   g   e       j   u   i   c   e       a   n   d   ".to_string());
        assert_eq!(even_iterator.next().unwrap(), "0000010       b   a   n   a   n   a       p   e   e   l   !       ".to_string());
        assert_eq!(even_iterator.next(), None);
    }

}