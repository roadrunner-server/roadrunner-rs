#![allow(dead_code)]

use std::process::{ChildStdin, ChildStdout, Command, Stdio};
use std::io::{Read, Write};
use crate::errors::Error;

trait Relay {
    fn send(&mut self, data: &[u8], flags: u8) -> Result<usize, Error>;
    fn receive(&self, data: &[u8]);
    fn close(&self);
}

// TODO send raw fd?
const BUFFER_SIZE: u64 = 65536;

struct PipeRelay {
    buf_size: u64,
    child_stdin: Option<ChildStdin>,
    child_stdout: Option<ChildStdout>,
}

impl PipeRelay {
    pub fn new_relay(ch_std_in: Option<ChildStdin>, ch_std_out: Option<ChildStdout>) -> Self {
        PipeRelay {
            buf_size: BUFFER_SIZE,
            child_stdin: ch_std_in,
            child_stdout: ch_std_out,
        }
    }
}

impl Relay for PipeRelay {
    fn send(&mut self, data: &[u8], flags: u8) -> Result<usize, Error> {
        if let Some(mut stdin) = self.child_stdin.take() {
            stdin.write_all(data)?;
        };
        Err(Error::PipeError { cause: "failed to write to stdin".to_string() })
    }

    fn receive(&self, data: &[u8]) {
        unimplemented!()
    }

    fn close(&self) {
        unimplemented!()
    }
}
