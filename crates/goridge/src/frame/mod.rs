use std::convert::{TryInto};
use crate::errors::Error;
use std::ops::BitAnd;

const WORD: u8 = 4;
const FRAME_OPTIONS_MAX_SIZE: u8 = 40;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
struct Frame {
    header: [u8; 12],
    payload: Vec<u8>,
}

impl Frame {
    fn new() -> Self {
        let mut f = Frame {
            header: [0; 12],
            payload: vec![],
        };
        f.default_hl();
        f
    }

    #[inline]
    fn write_hl(&mut self, hl: u8) {
        self.header[0] |= hl;
    }

    #[inline]
    fn default_hl(&mut self) {
        self.write_hl(3);
    }

    #[inline]
    pub  fn read_header(&self, data: &[u8]) -> Result<Self, Error> {
        if data.len() < 12 {
            return Err(Error::HeaderLenError { cause: "len is less than 12".to_string() });
        }
        Ok(Frame {
            header: data[..12].try_into().expect("slice with incorrect length"),
            payload: vec![],
        })
    }

    pub fn read_frame(&self, data: &[u8]) -> Self {
        // get options bits
        let opt = data[0].bitand(0x0F);

        if opt > 3 {
            return Self {
                header: data[..(opt * WORD) as usize].try_into().expect("array with incorrect length"),
                payload: vec![],
            };
        }

        let mut frame = Frame {
            header: data[..12_usize].try_into().expect("array with incorrect length"),
            payload: data[12_usize..].to_vec(),//.expect("can't transform slice into vector"),
        };

        frame.header[10] = 0;
        frame.header[11] = 0;

        frame
    }

    #[inline]
    pub fn version(&self) -> u8 {
        self.header[0] >> 4
    }

    #[inline]
    pub fn write_version(&mut self, version: u8) {
        if version > 15 {
            panic!("version should be less than 2 bytes (15)")
        }

        self.header[0] |= version << 4
    }


}


#[cfg(test)]
mod tests {
    use crate::frame::Frame;

    #[test]
    fn test1() {
        let mut ff = Frame::new();
        ff.write_hl(3);
        println!("{:?}", ff);
        ff.read_header(&[0; 11]).expect("errjor");
    }
}