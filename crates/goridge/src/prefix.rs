use crate::bit_operations::{put_u64_be, put_u64_le, read_be, read_le};
use crate::errors::Error;
use std::ops::BitAnd;

pub type Prefix = [u8; 17];

const PAYLOAD_EMPTY: u8 = 2;
const PAYLOAD_RAW: u8 = 4;
const PAYLOAD_ERROR: u8 = 8;
const PAYLOAD_CONTROL: u8 = 16;

pub trait Checker {
    fn to_string(&self) -> String;
    fn flags(&self) -> u8;
    fn has_flag(&self, flag: u8) -> bool;
    fn valid(&self) -> Result<(), Error>;
    fn size(&self) -> usize;
    fn has_payload(&self) -> bool;
    fn with_flag(&mut self, flag: u8) -> Self;
    fn with_flags(&mut self, flags: u8) -> Self;
    fn with_size(&mut self, size: usize) -> Self;
}

impl Checker for Prefix {
    fn to_string(&self) -> String {
        unimplemented!()
    }

    fn flags(&self) -> u8 {
        self[0]
    }

    fn has_flag(&self, flag: u8) -> bool {
        self[0].bitand(flag) == flag
    }

    fn valid(&self) -> Result<(), Error> {
        if read_le(&self[1..]) == read_be(&self[9..]) {
            return Ok(());
        }

        Err(Error::PrefixValidationError {
            cause: "read_le and read_be do not match".to_string(),
        })
    }

    fn size(&self) -> usize {
        if self.has_flag(PAYLOAD_EMPTY) {
            return 0;
        }
        // if p.HasFlag(PayloadEmpty) {
        //     return 0
        // }
        //
        // return binary.LittleEndian.Uint64(p[1:])
        return read_le(&self[1..]) as usize;
    }

    fn has_payload(&self) -> bool {
        !self.is_empty()
    }

    fn with_flag(&mut self, flag: u8) -> Self {
        self[0] |= flag;
        *self
    }

    fn with_flags(&mut self, flags: u8) -> Self {
        self[0] = flags;
        *self
    }

    fn with_size(&mut self, size: usize) -> Self {
        put_u64_le(&mut self[1..], size);
        put_u64_be(&mut self[9..], size);
        *self
    }
}
