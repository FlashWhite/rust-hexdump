mod tests;

use std::cmp::min;

// TODO: Create helper functions that return strings of formatted decimal, octal, and ascii respectively
// Create unit tests for each of the helper functions...
pub struct DefaultHexadecimal<'a> {
    idx: usize,
    length: usize,
    bytes: &'a Vec<u8>,
}

impl<'a> DefaultHexadecimal<'a> {
    pub fn new(length: usize, bytes: &'a Vec<u8>) -> Self {
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


pub struct TwoByteHexadecimal<'a> {
    idx: usize,
    length: usize,
    bytes: &'a Vec<u8>,
}

impl<'a> TwoByteHexadecimal<'a> {
    pub fn new(length: usize, bytes: &'a Vec<u8>) -> Self {
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


pub struct TwoByteDecimal<'a> {
    idx: usize,
    length: usize,
    bytes: &'a Vec<u8>,
}

impl<'a> TwoByteDecimal<'a> {
    pub fn new(length: usize, bytes: &'a Vec<u8>) -> Self {
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


pub struct TwoByteOctal<'a> {
    idx: usize,
    length: usize,
    bytes: &'a Vec<u8>,
}

impl<'a> TwoByteOctal<'a> {
    pub fn new(length: usize, bytes: &'a Vec<u8>) -> Self {
        TwoByteOctal {idx: 0, length, bytes}
    }
}

impl<'a> Iterator for TwoByteOctal<'a> {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        if self.idx == self.length { // nothing left to print
            return None
        }
        let mut out: String = format!("{:07x}  ", self.idx);
        if 8 <= (self.length-self.idx)/2 { 
            for _ in 0..8 {
                out.push_str(&format!("{:06o}  ", ((self.bytes[self.idx+1] as u16) << 8)+(self.bytes[self.idx] as u16)));
                self.idx += 2;
            }
        } else { // last line to be printed
            while self.idx+1 < self.length {
                out.push_str(&format!("{:06o}  ", ((self.bytes[self.idx+1] as u16) << 8)+(self.bytes[self.idx] as u16)));
                self.idx += 2;
            }
            if self.idx != self.length { // deal with leftover odd
                out.push_str(&format!("{:06o}  ", self.bytes[self.idx]));
                self.idx += 1;
            }
        }
        Some(out)
    }
}


pub struct OneByteOctal<'a> {
    idx: usize,
    length: usize,
    bytes: &'a Vec<u8>,
}

impl<'a> OneByteOctal<'a> {
    pub fn new(length: usize, bytes: &'a Vec<u8>) -> Self {
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

pub struct OneByteChar<'a> {
    idx: usize,
    length: usize,
    bytes: &'a Vec<u8>,
}

impl<'a> OneByteChar<'a> {
    pub fn new(length: usize, bytes: &'a Vec<u8>) -> Self {
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

