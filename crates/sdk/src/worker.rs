use crate::payload::Payload;
use crate::state::WorkerState;
use anyhow::anyhow;
use goridge_rs::frame::Frame;
use goridge_rs::frame::frame_flags::Flag::{CodecProto, Error as GoridgeErrorBit};
use goridge_rs::pipe::Pipes;
use log::debug;
use std::time::Instant;

pub struct WorkerProcess {
    created: Instant,
    state: WorkerState,
    pid: u32,
    pipes: Pipes,
}

impl WorkerProcess {
    async fn new(cmd: &[&str]) -> anyhow::Result<Self> {
        debug!("worker_created");

        let mut p = Pipes::new(cmd).await?;
        let id = p.id().await?;

        Ok(WorkerProcess {
            created: Instant::now(),
            state: WorkerState::default(),
            pid: id,
            pipes: p,
        })
    }
}

impl WorkerProcess {
    fn created(&mut self) -> Instant {
        debug!("created called: {:?}", self.created);
        self.created
    }

    fn pid(&self) -> u32 {
        self.pid
    }

    fn state(&self) -> WorkerState {
        self.state
    }

    async fn exec(&mut self, p: Payload) -> anyhow::Result<Payload> {
        assert!(!p.body.is_empty());
        assert!(!p.context.is_empty());

        let mut frame = Frame::default();
        frame.write_version(1);
        frame.write_flags(&[CodecProto]);
        frame.write_options(&[p.context.len() as u32]);

        let mut tmp = vec![];
        tmp.extend_from_slice(&p.context);
        tmp.extend_from_slice(&p.body);
        frame.write_payload(&tmp);
        frame.write_crc();

        self.pipes.send(&mut frame).await?;

        let mut frame_receive = self.pipes.receive_stdout().await?;

        let flags = frame_receive.read_flags();
        if flags & (GoridgeErrorBit as u8) != 0 {
            return Err(anyhow!(""));
        }

        let options = frame_receive.read_options();
        match options {
            None => Err(anyhow!("options length should be equal 1 (body offset)")),
            Some(opts) => {
                if opts.len() != 1 {
                    return Err(anyhow!("options length should be equal 1 (body offset)"));
                }

                if frame_receive.payload().len() < opts[0] as usize {
                    return Err(anyhow!("bad payload"));
                }

                Ok(Payload::new(
                    flags,
                    frame_receive.payload()[..(opts[0] as usize)].to_owned(),
                    frame_receive.payload()[(opts[0] as usize)..].to_owned(),
                ))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use tokio::io::AsyncReadExt;
    use tokio::process::ChildStderr;
    use tokio::spawn;

    use crate::payload::Payload;
    use crate::worker::WorkerProcess;
    use log::{debug, error, info, warn};
    use std::str::from_utf8;
    use std::time::Instant;

    #[tokio::test]
    async fn test_init_worker() {
        env_logger::init();
        let mut wp = WorkerProcess::new(&["php", "../../tests/worker.php"])
            .await
            .unwrap();

        let mut stderr: ChildStderr = wp.pipes.take_stderr().unwrap();
        let stderr_task = spawn(async move {
            use tokio::io::BufReader;
            let mut reader = BufReader::new(&mut stderr);
            // https://linux.die.net/man/7/pipe
            let mut data = Vec::with_capacity(65536);
            loop {
                match reader.read_buf(&mut data).await {
                    Ok(0) => {
                        warn!("EOF on stderr");
                        break;
                    } // EOF
                    Ok(_) => {
                        if let Ok(s) = from_utf8(&data) {
                            info!("{}", s);
                        }
                        data.clear();
                    }
                    Err(e) => {
                        error!("Error reading stderr: {}", e);
                        break;
                    }
                }
            }
        });

        let timer: Instant = Instant::now();
        for _ in 0..2 {
            let p = Payload::new(1, b"hello", b"hello");

            let res = wp.exec(p).await.unwrap();
            let body_utf8 = from_utf8(&res.body).unwrap();
            println!("{:?}", body_utf8);
        }

        tokio::time::sleep(std::time::Duration::from_secs(2)).await;
        stderr_task.abort();
        println!("done in: {}", timer.elapsed().as_millis());
    }
}
