use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("invalid register")]
    InvalidRegister,

    #[error("illegal operation")]
    IllegalOperation,

    #[error("invalid memory access")]
    InvalidMemoryAccess,
}

pub struct State<const M: usize> {
    // TODO: Does PC have to be aligned?
    pc: u32,

    // The general purpose 32 bit registers. All the registers are treated equal
    // on the machine level. The compiler is responsible for assigning special
    // meaning.
    // registers: [Register; 31],
    registers: [u32; 31],

    // The main memory of the machine in bytes.
    memory: [u8; M],
}

impl<const M: usize> Default for State<M> {
    fn default() -> Self {
        Self {
            pc: 0,
            registers: [0; 31],
            memory: [0; M],
        }
    }
}

impl<const M: usize> State<M> {
    // Get the program counter.
    pub fn get_pc(&self) -> u32 {
        self.pc
    }

    // Get the program counter.
    // TODO: Should we check for alignment here?
    pub fn set_pc(&mut self, value: u32) {
        self.pc = value;
    }

    // Get the value on a general register.
    pub fn get_r(&self, name: u8) -> Result<u32, Error> {
        match name {
            0 => Ok(0),
            name if name > 31 => Err(Error::InvalidRegister),
            name => Ok(self.registers[name as usize]),
        }
    }

    // Set the value on a general register.
    pub fn set_r(&mut self, name: u8, value: u32) -> Result<(), Error> {
        match name {
            0 => Err(Error::IllegalOperation),
            name if name > 31 => Err(Error::InvalidRegister),
            name => {
                self.registers[name as usize] = value;
                Ok(())
            }
        }
    }

    // Get a 4 byte value from memory starting from the base address assuming
    // little endian-ness.
    // TODO: Check for alignment.
    // TODO: Check for bounds.
    pub fn get_mem_u32(&self, base_addr: u32) -> Result<u32, Error> {
        let addr = base_addr as usize;
        let cast: Result<[u8; 4], _> = self.memory[addr..addr + 4].try_into();
        match cast {
            Ok(byts) => Ok(u32::from_le_bytes(byts)),
            Err(_) => Err(Error::InvalidMemoryAccess),
        }
    }

    // Set a 4 byte value in memory starting at the base address with little
    // endian-ness.
    // TODO: Check for bounds.
    pub fn set_mem_u32(&mut self, base_addr: u32, val: u32) -> Result<(), Error> {
        let addr = base_addr as usize;

        if let Some(slice) = self.memory.get_mut(addr..addr + 4) {
            let mut byts = val.to_le_bytes();
            slice.swap_with_slice(&mut byts);
        }

        Ok(())
    }
}
