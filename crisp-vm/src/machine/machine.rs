use crate::machine::registers::Register;

pub struct Machine {
    pc: Register,
    registers: [Register; 32],
}

impl Machine {
    pub fn new() -> Self {
        let mut registers = [Register::new(0, true); 32];
        registers[0] = Register::new(0, false); // x0 is hardwired to 0.

        return Machine {
            pc: Register::new(0, true),
            registers,
        };
    }
}
