use crate::errors::Error;
use crate::frame::Frame;
use crate::relay::Relay;
use std::io::{BufReader, Read, Write};
use std::process::{ChildStderr, ChildStdin, ChildStdout, Command, Stdio};

pub struct Pipes {
    // stdout: Box<dyn Write>,
    stderr: Box<dyn Write>,
    // stdin: Box<dyn Read>,
    // stderr: Option<ChildStderr>,
    // stdin: Option<ChildStdin>,
    // stdout: Option<ChildStdout>,
}

impl Relay<Frame> for Pipes {
    fn send(&mut self, frame: &mut Frame) -> Result<(), Error> {
        let v: Vec<u8> = frame.into();
        // self.stdin.unwrap().write_all(&v[..]);

        // self.stdout.unwrap().write_all(&v[..])?;
        Ok(())
    }

    fn receive(&mut self) -> Result<Vec<u8>, Error> {
        // let mut reader = BufReader::new(&mut self.stdin);
        let mut data: Vec<u8> = vec![];
        // reader.read_to_end(&mut data)?;
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
    fn new(cmd: &str, stderr: Box<dyn std::io::Write>) -> Result<Self, std::io::Error> {
        // let mut command = Command::new(cmd)
        //     .stdin(Stdio::piped())
        //     .stdout(Stdio::piped())
        //     .stderr(Stdio::piped())
        //     .spawn()?;
        //
        Ok(Pipes {
            stderr: stderr,
            // stdout: command.stdout,
            // stdin: command.stdin,
        })
    }
}


#[cfg(test)]
mod tests {
    use std::process::{Command, Stdio};
    use crate::pipe::Pipes;

    #[test]
    fn test1() {
        let cmd: &str = "ls";
        let mut command = Command::new(cmd)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn().unwrap();

        Pipes::new("ls", Box::new(command.stderr.into()));
    }
}