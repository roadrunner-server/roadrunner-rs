use std::rc::Rc;

#[cfg(test)]
mod tests {
    use crate::Plugin;

    #[test]
    fn it_works() {
        let p = Plugin::new();
    }
}

#[derive(Default)]
struct Plugin {

}

impl Plugin {
    fn new() -> Plugin {
        let s = Rc::new("foo".to_string());
        let t = s;
        let u = t;
        Self{}
    }

    // fn start() -> Result<>
}


pub struct SpiderRobot {
    species: String,
    web_enabled:bool,
}