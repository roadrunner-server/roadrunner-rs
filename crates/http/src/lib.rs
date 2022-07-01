use std::io;
use std::net::TcpListener;

#[cfg(test)]
mod tests {
    use crate::Plugin;

    #[test]
    fn it_works() {
        let p = Plugin::new("127.0.0.1:7878").unwrap();
    }
}

#[derive(Default)]
pub struct Plugin {

}

impl Plugin {
    pub fn new(val: &str) -> Result<Plugin, io::Error> {
        let listener = TcpListener::bind(val)?;

        for stream in listener.incoming() {
            let stream = stream?;

            println!("connection established!");
        }

        Ok(Plugin{})
    }

}
