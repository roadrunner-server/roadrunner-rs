use crate::errors::Error;
use crate::frame::Frame;
use crate::relay::Relay;
use std::io::{BufReader, Read, Write};
use std::process::{Command, Stdio};

pub struct Pipes {
    stdout: Box<dyn Write>,
    stdin: Box<dyn Read>,
}

impl Relay<Frame> for Pipes {
    fn send(&mut self, frame: &mut Frame) -> Result<(), Error> {
        let v: Vec<u8> = frame.into();
        self.stdout.write_all(&v[..])?;
        Ok(())
    }

    fn receive(&mut self) -> Result<Vec<u8>, Error> {
        let mut reader = BufReader::new(&mut self.stdin);
        let mut data: Vec<u8> = vec![];
        reader.read_to_end(&mut data)?;
        Ok(data)
    }

    fn receive_string(&mut self) -> Result<String, Error> {
        todo!()
    }

    fn close(&self) {
        todo!()
    }
}

impl Pipes {
    fn new(cmd: &str) -> Self {
        let mut command = Command::new(cmd)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn();

        Pipes {}
    }
}
