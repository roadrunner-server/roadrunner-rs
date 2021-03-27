#![allow(dead_code)]

use crate::errors::Error;
use crate::prefix::{Checker, Prefix};
use crate::relay::Relay;
use std::io::{Read, Write};
use std::process::{ChildStdin, ChildStdout};
use async_trait::async_trait;

struct Message {
    flags: u8,
    data: [u8],
}

// TODO send raw fd?
const BUFFER_SIZE: u64 = 65536;
const PREFIX_SIZE: usize = 17;

pub struct PipeRelay {
    buf_size: u64,
    child_stdin: Option<ChildStdin>,
    child_stdout: Option<ChildStdout>,
}

// TODO generic
impl PipeRelay {
    pub fn new_relay(ch_std_in: Option<ChildStdin>, ch_std_out: Option<ChildStdout>) -> Self {
        PipeRelay {
            buf_size: BUFFER_SIZE,
            child_stdin: ch_std_in,
            child_stdout: ch_std_out,
        }
    }
}

#[async_trait]
impl Relay for PipeRelay {
    fn send(&mut self, data: &[u8], flags: u8) -> Result<usize, Error> {
        let mut d = Vec::with_capacity(data.len() + PREFIX_SIZE);

        let mut p = Prefix::default();
        p.with_flags(flags).with_size(data.len());
        d.extend_from_slice(&p);
        d.extend_from_slice(data);

        if let Some(mut stdin) = self.child_stdin.take() {
            stdin.write_all(&d)?;
        }
        Ok(d.len())
    }

    fn receive(&mut self) -> Result<Vec<u8>, Error> {
        let mut p = Prefix::default();
        if let Some(mut stdout) = self.child_stdout.take() {
            stdout.read_exact(&mut p)?; // read exact prefix data
        }

        // validation step
        p.valid()?;
        if !p.has_payload() {
            return Err(Error::PipeError {
                cause: "has no payload".to_string(),
            });
        }

        // Read the rest data
        let mut data = vec![];
        if let Some(mut stdout) = self.child_stdout.take() {
            stdout.read_to_end(&mut data)?;
        }

        Ok(data)
    }

    fn receive_string(&mut self) -> Result<String, Error> {
        let mut s = String::new();
        if let Some(mut stdout) = self.child_stdout.take() {
            stdout.read_to_string(&mut s)?;
        }
        Ok(s)
    }

    fn close(&self) {
        unimplemented!()
    }
}

#[allow(soft_unstable)]
#[cfg(test)]
mod tests {
    use crate::pipe::{PipeRelay, Relay};
    use std::process::{Command, Stdio};

    #[test]
    fn test_pipe_init() {
        let process = match Command::new("php")
            .arg("./test/hello.php")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .current_dir(".")
            .spawn()
        {
            Err(why) => panic!("couldn't spawn php: {}", why.to_string()),
            Ok(process) => process,
        };

        let mut relay = PipeRelay::new_relay(process.stdin, process.stdout);

        if let Ok(data) = relay.receive_string() {
            assert_eq!("hello1 - test".to_string(), data);
        }
    }
}
