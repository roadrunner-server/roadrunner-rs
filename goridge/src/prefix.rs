use crate::errors::Error;
use std::ops::Shl;

pub type Prefix = [u8; 17];

pub trait Checker {
    fn to_string(&self) -> String;
    fn valid(&self) -> Result<(), Error>;
}

fn read_le(data: &[u8]) -> u64 {
    (
        data[0] |
            data[1].shl(8) |
            data[2].shl(16) |
            data[3].shl(24) |
            data[4].shl(32) |
            data[5].shl(40) |
            data[6].shl(48) |
            data[7].shl(58)
    ) as u64
}

fn read_be(data: &[u8]) -> u64 {
    (
        data[7] |
            data[6].shl(8) |
            data[5].shl(16) |
            data[4].shl(24) |
            data[3].shl(32) |
            data[2].shl(40) |
            data[1].shl(48) |
            data[0].shl(58)
    ) as u64
}

impl Checker for Prefix {
    fn to_string(&self) -> String {
        unimplemented!()
    }

    fn valid(&self) -> Result<(), Error> {
        if read_le(&self[1..]) == read_be(&self[9..]) {
            return Ok(());
        }

        Err(Error::PrefixValidationError { cause: "read_le and read_be do not match".to_string() })
    }
}