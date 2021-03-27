#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
struct Frame {
    header: [u8; 12],
    payload: Vec<u8>,
}

impl Frame {
    fn new() -> Self {
        let mut f = Frame {
            header: [0; 12],
            payload: vec![],
        };
        f.default_hl();
        f
    }

    fn write_hl(&mut self, hl: u8) {
        self.header[0] |= hl;
    }

    fn default_hl(&mut self) {
        self.write_hl(3);
    }

    // fn read_header(data: &[u8]) -> Self {
    //     Frame {}
    // }
}


#[cfg(test)]
mod tests {
    use crate::frame::Frame;

    #[test]
    fn test1() {
        let mut ff = Frame::new();
        ff.write_hl(3);
        println!("{:?}", ff);
    }
}