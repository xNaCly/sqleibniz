pub struct Tracer {
    pub indent: usize,
}

impl Tracer {
    pub fn new() -> Self {
        Self { indent: 0 }
    }

    pub fn call(&mut self, name: &str) {
        self.indent += 1;
        println!("{}↳ Parser::{}()", " ".repeat(self.indent), name);
    }
}
