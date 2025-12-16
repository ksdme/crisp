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

    // TODO: Check for bounds.
    pub fn get_mem_u8(&self, addr: u32) -> Result<u8, Error> {
        Ok(self.memory[addr as usize])
    }

    // Get a 2 byte value from memory starting from the base address assuming
    // little endian-ness.
    // TODO: Check for alignment.
    // TODO: Check for bounds.
    pub fn get_mem_u16(&self, base_addr: u32) -> Result<u16, Error> {
        Ok(u16::from_le_bytes([
            self.get_mem_u8(base_addr)?,
            self.get_mem_u8(base_addr + 1)?,
        ]))
    }

    // Get a 4 byte value from memory starting from the base address assuming
    // little endian-ness.
    // TODO: Check for alignment.
    // TODO: Check for bounds.
    pub fn get_mem_u32(&self, base_addr: u32) -> Result<u32, Error> {
        Ok(u32::from_le_bytes([
            self.get_mem_u8(base_addr)?,
            self.get_mem_u8(base_addr + 1)?,
            self.get_mem_u8(base_addr + 2)?,
            self.get_mem_u8(base_addr + 3)?,
        ]))
    }

    // TODO: Check for bounds.
    pub fn set_mem_u8(&mut self, addr: u32, val: u8) -> Result<(), Error> {
        self.memory[addr as usize] = val;
        Ok(())
    }

    // Set a 2 byte value in memory starting at the base address with little
    // endian-ness.
    // TODO: Check for bounds.
    pub fn set_mem_u16(&mut self, base_addr: u32, val: u16) -> Result<(), Error> {
        let [a, b] = val.to_le_bytes();

        self.set_mem_u8(base_addr, a)?;
        self.set_mem_u8(base_addr + 1, b)?;

        Ok(())
    }

    // Set a 4 byte value in memory starting at the base address with little
    // endian-ness.
    // TODO: Check for bounds.
    pub fn set_mem_u32(&mut self, base_addr: u32, val: u32) -> Result<(), Error> {
        let [a, b, c, d] = val.to_le_bytes();

        self.set_mem_u8(base_addr, a)?;
        self.set_mem_u8(base_addr + 1, b)?;
        self.set_mem_u8(base_addr + 2, c)?;
        self.set_mem_u8(base_addr + 3, d)?;

        Ok(())
    }
}
