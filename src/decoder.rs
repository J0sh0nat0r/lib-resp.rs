use nom::Err;
use std::io::{BufRead, Error, ErrorKind, Result};

use super::{Parser, Value};

/// Streaming decoder implementation for [BufRead](std::io::BufRead) implementors
pub struct Decoder<T: BufRead> {
    src: T,
    multi_buf: Vec<Value>,
}

impl<T> Decoder<T>
where
    T: BufRead,
{
    pub fn new(src: T) -> Self {
        Decoder {
            src,
            multi_buf: Vec::new(),
        }
    }

    /// Attempts to read a single value from the stream, then parse it.
    pub fn decode(&mut self) -> Result<Option<Value>> {
        let (ret, consumed) = {
            let buf = self.src.fill_buf()?;

            match Parser::parse(buf) {
                Ok((i, o)) => (Ok(Some(o)), buf.len() - i.len()),

                Err(Err::Incomplete(_)) => (Ok(None), 0),

                Err(_) => (
                    Err(Error::new(ErrorKind::InvalidData, "Invalid RESP")),
                    buf.len(),
                ),
            }
        };

        if consumed != 0 {
            self.src.consume(consumed);
        }

        ret
    }

    /// Attempts to read *all* values from the stream, then parse them.
    ///
    /// NOTE: Parsed values will be stored until the stream is empty.
    pub fn decode_all(&mut self) -> Result<Option<Vec<Value>>> {
        loop {
            match self.decode()? {
                None => break,
                Some(value) => self.multi_buf.push(value),
            }
        }

        if self.src.fill_buf()?.len() > 0 {
            return Ok(None);
        }

        Ok(Some(::std::mem::replace(&mut self.multi_buf, Vec::new())))
    }
}
