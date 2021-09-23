use crate::errors::Error;
use crate::state::WorkerState;
use goridge_rs::relay::Relay;
use std::io::{BufReader, BufWriter};
use std::process::Child;
use std::process::{ChildStderr, ChildStdin, ChildStdout, Command};
use std::time::Instant;

pub trait Worker<T: Relay<T>> {
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

    fn attach_relay(&mut self, rl: T);

    // state
    fn value() -> i64;
    fn set(value: u64);
    fn num_execs() -> u64;
    fn is_active() -> bool;
    fn register_exec();
    fn set_last_used(lu: u64);
    fn last_used() -> u64;
}

struct ChildProcess {
    stdin: Option<BufWriter<ChildStdin>>,
    stdout: Option<ChildStdout>,
    stderr: Option<ChildStderr>,
}

pub struct WorkerProcess<T: Relay<T>> {
    created: std::time::Instant,
    // events channel
    state: WorkerState,
    cmd: std::process::Command,
    pid: u16,
    child: Option<Child>,
    child_fds: ChildProcess,
    relay: T,
}

impl<T: Relay<T>> WorkerProcess<T> {
    fn new(rl: T, command: Command) -> Self {
        WorkerProcess {
            created: Instant::now(),
            state: WorkerState::default(),
            cmd: command,
            child_fds: ChildProcess {
                stdin: None,
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
    T: Relay<T>,
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

        Err(Error::WaitError {
            cause: "some error".to_string(),
        })
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

    fn value() -> i64 {
        todo!()
    }

    fn set(value: u64) {
        todo!()
    }

    fn num_execs() -> u64 {
        todo!()
    }

    fn is_active() -> bool {
        todo!()
    }

    fn register_exec() {
        todo!()
    }

    fn set_last_used(lu: u64) {
        todo!()
    }

    fn last_used() -> u64 {
        todo!()
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
