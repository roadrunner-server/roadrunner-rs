#![allow(dead_code)]

use std::process::{ChildStdin, ChildStdout};
use std::io::{Read, Write};
use crate::errors::Error;
use crate::prefix::{Prefix, Checker};

trait Relay {
    fn send(&mut self, data: &[u8], flags: u8) -> Result<usize, Error>;
    fn receive(&mut self) -> Result<Vec<u8>, Error>;
    fn receive_string(&mut self) -> Result<String, Error>;
    fn close(&self);
}

struct Message {
    flags: u8,
    data: [u8],
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
        }
        Ok(data.len())
    }

    fn receive(&mut self) -> Result<Vec<u8>, Error> {
        let mut p = Prefix::default();
        if let Some(mut stdout) = self.child_stdout.take() {
            stdout.read_exact(&mut p)?; // read exact prefix data
        }

        p.valid()?;


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
    use std::process::{Command, Stdio};
    use crate::pipe::{PipeRelay, Relay};

    #[test]
    fn test_pipe_init() {
        let process = match Command::new("/usr/bin/php")
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