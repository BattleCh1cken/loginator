use anyhow::Result;
use regex::Regex;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DecodeError {
    #[error("The length of the encoded data was incorrect.")]
    InvalidLength(),
}

/// This struct reads a stream of COBS encoded data and decodes it.
/// You can read more about COBS encoding here: https://www.wikipedia.org/
/// PROS emits COBS encoded data by default which helps combat incomplete packets.
#[derive(Debug, Clone, Default)]
pub struct Decoder {
    buffer: Vec<u8>,
    pointer: usize, // this is the index of the next zero
    is_parsing: bool,
}

impl Decoder {
    /// Adds a single byte for the decoder to decode.
    pub fn feed(&mut self, byte: u8) -> Result<Option<Vec<u8>>, DecodeError> {
        if !self.is_parsing {
            //println!("found next zero: {byte}");
            self.pointer = byte as usize; // The first pointer is always the overhead byte.
            self.is_parsing = true;
        } else if byte == 0 {
            //println!("found the end");
            // If the byte being received is zero then we know that we have all of our data
            let result = self.buffer.clone();

            if !(self.buffer.len() + 1 == self.pointer) {
                //println!("the end isn't where it should be");
                self.buffer = vec![];
                return Err(DecodeError::InvalidLength());
            }

            self.buffer = vec![];
            return Ok(Some(result));
        } else if self.buffer.len() + 1 == self.pointer {
            // If the current index is pointer then the current value was originally zero, but was changed during encoding.
            // We have to add 1 to the length to account for the overhead byte.
            self.buffer.push(0);
            self.pointer = self.buffer.len() + byte as usize;
            //println!("found zero: {byte}");
            //println!("new pointer len: {}", self.pointer);
        } else {
            // If none of the other conditions are true then the data does not need to be modified at all.
            //println!("pushing byte: {byte}");
            self.buffer.push(byte);
        }

        Ok(None)
    }

    /// Sends multiple bytes for the decoder to decode
    pub fn push(&mut self, data: Vec<u8>) -> Result<Option<Vec<u8>>, DecodeError> {
        for byte in data {
            match self.feed(byte) {
                Ok(Some(value)) => return Ok(Some(value)),
                Ok(None) => {} // The data is valid so far, but is not complete yet
                Err(err) => return Err(err),
            }
        }
        Ok(None)
    }
    /// Expects UTF-8 as input. The data cannot be COBS encoded.
    pub fn parse(data: Vec<u8>) -> Result<Vec<f32>> {
        let mut buffer = String::new();

        for byte in data {
            buffer.push(byte as char);
        }

        let regex = Regex::new(r"[-+]?\d*\.\d+|[-+]?\d+").unwrap();

        let mut result = vec![];

        for capture in regex.captures_iter(&buffer) {
            let number: f32 = capture.get(0).unwrap().as_str().parse()?;
            result.push(number);
        }

        Ok(result)
    }
}
