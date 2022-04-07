use crate::errors::Error;
use crate::frame::Frame;
use crate::relay::Relay;
use std::io::{BufReader, BufWriter, Read, Write};
use std::process::{ChildStderr, ChildStdin, ChildStdout, Command, Stdio};

pub struct Pipes {
    stdin: Option<ChildStdin>,
    stdout: Option<ChildStdout>,
    stderr: Option<ChildStderr>,
}

impl Relay<Frame> for Pipes {
    fn send(&mut self, frame: &mut Frame) -> Result<(), Error> {
        let stderr = self.stdin.as_mut();

        match stderr {
            None => Err(Error::PipeError {
                cause: "".to_string(),
            }),
            Some(child) => {
                let mut buf = BufWriter::new(child);
                match buf.write_all(&frame.bytes()) {
                    Ok(_) => Ok(()),
                    Err(err) => Err(Error::PipeError {
                        cause: err.to_string(),
                    }),
                }
            }
        }
    }

    fn receive(&mut self) -> Result<Vec<u8>, Error> {
        let stderr = self.stderr.as_mut();
        match stderr {
            None => Err(Error::PipeError {
                cause: "".to_string(),
            }),
            Some(stdout) => {
                let mut buf = BufReader::new(stdout);

                let mut data = vec![];
                match buf.read_to_end(&mut data) {
                    Ok(_) => Ok(data),
                    Err(err) => Err(Error::PipeError {
                        cause: err.to_string(),
                    }),
                }
            }
        }
    }

    fn receive_string(&mut self) -> Result<String, Error> {
        todo!()
    }

    fn close(&self) {
        todo!()
    }
}

impl Pipes {
    fn new(cmd: &str) -> Result<Self, std::io::Error> {
        let command = Command::new(cmd)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;

        Ok(Pipes {
            stderr: command.stderr,
            stdout: command.stdout,
            stdin: command.stdin,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::pipe::Pipes;
    use crate::relay::Relay;
    use std::process::Stdio;

    #[test]
    fn test1() {
        let cmd: &str = "ls";

        let mut p = Pipes::new("ls").unwrap();
        let data = p.receive().unwrap();
        let data2 = p.receive().unwrap();

        println!("{:?}", data);
    }
}
