use std::io;

trait Plugin {
    fn init() -> Result<(), io::Error>;
    fn serve() -> String;
    fn stop() -> String;
}