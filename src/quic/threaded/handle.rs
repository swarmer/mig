pub type Handle = u64;

#[derive(Debug, Default, PartialEq)]
pub struct HandleGenerator {
    last_handle: Handle,
}

impl HandleGenerator {
    pub fn new() -> HandleGenerator {
        HandleGenerator::default()
    }

    pub fn generate(&mut self) -> Handle {
        self.last_handle += 1;
        self.last_handle
    }
}
