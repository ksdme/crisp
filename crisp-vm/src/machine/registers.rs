pub enum Error {
    Illegal,
}

#[derive(Debug, Clone, Copy)]
pub struct Register {
    value: u32,
    writeable: bool,
}

impl Register {
    pub fn new(initial: u32, writeable: bool) -> Self {
        Register {
            value: initial,
            writeable: writeable,
        }
    }

    pub fn get(&self) -> u32 {
        self.value
    }

    pub fn set(&mut self, value: u32) -> Result<(), Error> {
        if self.writeable {
            self.value = value;
            Ok(())
        } else {
            Err(Error::Illegal)
        }
    }
}
