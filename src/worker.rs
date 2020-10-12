use crate::errors::Error;
use crate::state::WorkerState;
use goridge_rs::relay::Relay;
use std::process::{Command, ChildStdin, ChildStdout, ChildStderr};
use std::time::Instant;
use std::process::Child;
use std::io::{BufWriter, BufReader};

pub trait Worker<T: Relay> {
    // time in unix nano format
    fn created(&mut self) -> std::time::Instant;

    // channel WorkerEvents
    fn events(&self);

    fn pid(&self) -> u16;

    fn state(&self) -> WorkerState;

    fn start(&mut self) -> Result<(), Error>;

    fn wait(&mut self) -> Result<(), Error>;

    fn stop(&self) -> Result<(), Error>;

    fn kill(&self) -> Result<(), Error>;

    // fn relay(&self) -> T;

    fn attach_relay(&mut self, rl: T);
}

struct ChildProcess {
    stdin: Option<BufWriter<ChildStdin>>,
    stdout: Option<ChildStdout>,
    stderr: Option<ChildStderr>,
}

pub struct WorkerProcess<T: Relay> {
    created: std::time::Instant,
    // events channel
    state: WorkerState,
    cmd: std::process::Command,
    pid: u16,
    child: Option<Child>,
    child_fds: ChildProcess,
    // errbuffer
    // endstate
    relay: T,
}

impl<T: Relay> WorkerProcess<T> {
    fn new(rl: T, command: Command) -> Self {
        WorkerProcess {
            created: Instant::now(),
            state: WorkerState::default(),
            cmd: command,
            child_fds: ChildProcess {
                stdin:,
                stdout: None,
                stderr: None,
            },
            pid: 0,
            relay: rl,
            child: None,
        }
    }
}

impl<T> Worker<T> for WorkerProcess<T>
    where
        T: Relay,
{
    fn created(&mut self) -> std::time::Instant {
        self.created
    }

    // channel with events???
    fn events(&self) {
        unimplemented!()
    }

    fn pid(&self) -> u16 {
        self.pid
    }

    fn state(&self) -> WorkerState {
        self.state
    }

    fn start(&mut self) -> Result<(), Error> {
        let mut spawned = self.cmd.spawn()?;
        let stdin = BufWriter::new(spawned.stdin.take().unwrap());
        let stdout = BufReader::new(spawned.stdout.take().unwrap());
        let stderr = BufReader::new(spawned.stderr.take().unwrap());
        self.child_fds.stdin = Option::from(stdin);
        Ok(())
    }

    fn wait(&mut self) -> Result<(), Error> {
        if let Some(mut ch) = self.child.take() {
            let res = ch.wait();
            if let Ok(data) = res {
                println!("data: {}", data.to_string());
                return Ok(());
            }
        }

        Err(Error::WaitError { cause: "some error".to_string() })
    }

    fn stop(&self) -> Result<(), Error> {
        unimplemented!()
    }

    fn kill(&self) -> Result<(), Error> {
        unimplemented!()
    }

    // fn relay(&self) -> T {
    //     self.relay
    // }

    fn attach_relay(&mut self, rl: T) {
        self.relay = rl;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use goridge_rs::pipe::PipeRelay;

    #[test]
    fn test_init_worker() {
        let command = Command::new("ls");
        let pipe_relay = PipeRelay::new_relay(None, None);
        let mut worker = WorkerProcess::new(pipe_relay, command);
        let a = worker.start();
        let b = worker.wait();
    }
}