use crate::machine::registers::Register;

pub struct Machine {
    pc: Register,

    // The general purpose 32 bit registers. All the registers are treated equal
    // on the machine level. The compiler is responsible for assigning special
    // meaning.
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
