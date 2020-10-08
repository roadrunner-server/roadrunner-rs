pub type Prefix = [u8; 17];

pub trait Checker {
    fn to_string() -> String;
}

impl Checker for Prefix {
    fn to_string() -> String {
        unimplemented!()
    }
}