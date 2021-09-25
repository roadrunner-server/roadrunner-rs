use crate::errors::Error;
use crate::frame::Frame;
use crate::relay::Relay;
use std::io::Write;

pub struct Pipes {
    stdin: std::io::Stdin,
    stdout: std::io::Stdout,
}

impl Relay<Frame> for Pipes {
    fn send(&mut self, frame: &mut Frame) -> Result<(), Error> {
        let v: Vec<u8> = frame.into();
        self.stdout.write_all(&v[..])?;
        Ok(())
    }

    fn receive(&mut self) -> Result<Vec<u8>, Error> {
        todo!()
    }

    fn receive_string(&mut self) -> Result<String, Error> {
        todo!()
    }

    fn close(&self) {
        todo!()
    }
}
