use thiserror::Error;

use crate::machine::state::{self, State};

// https://docs.openhwgroup.org/projects/cva6-user-manual/01_cva6_user/RISCV_Instructions_RV32I.html
#[derive(Debug)]
pub enum Inst {
    // U - Load Upper Immediate
    // Place the immediate value in the top 20 bits of the destination register rd, filling in the
    // lowest 12 bits with zeros.
    LUI { rd: u8, imm: u32 },

    // U - Add Upper Immediate to PC
    // Froms a 32-bit offset from the 20-bit immediate, filling in the lowest 12 bits with zeros,
    // adds this offset to the pc, then place the result in register rd.
    AUIPC { rd: u8, imm: u32 },

    // J - Jump and Link
    // Offset is sign-extended and added to the pc to form the jump target address, then
    // setting the least-significant bit of the result to zero (for easy 2 byte alignment),
    // and store the address of instruction following the jump (pc+4) into register rd.
    JAL { rd: u8, imm: u32 },

    // I - Jump and Link Register
    // Like JAL but gets the address to jump to by adding the sign extended imm to rs1 and
    // setting the return address in rd. The least-significant bit of the address can be force
    // set to 0 to assume alignment.
    JALR { rd: u8, rs1: u8, imm: u16 },

    // B - Branch if Equal
    // Takes the branch (pc + sign extended imm) if the registers rs1 and rs2 are equal.
    // The imm is shifted to have an implicit LSB making the imm 13 bits long (to account for the alignment).
    BEQ { rs1: u8, rs2: u8, imm: u16 },

    // B - Branch Not Equal
    // Takes the branch (pc + sign extended imm) if the registers rs1 and rs2 are not equal.
    // The imm is shifted to have an implicit LSB making the imm 13 bits long (to account for the alignment).
    BNE { rs1: u8, rs2: u8, imm: u16 },

    // B - Branch Less Than
    // Takes the branch (pc + sign extended imm) if the registers rs1 < rs2 on signed comparison.
    // The imm is shifted to have an implicit LSB making the imm 13 bits long (to account for the alignment).
    BLT { rs1: u8, rs2: u8, imm: u16 },

    // B - Branch Less Than Unsigned
    // Takes the branch (pc + sign extended imm) if the registers rs1 < rs2 on unsigned comparison.
    // The imm is shifted to have an implicit LSB making the imm 13 bits long (to account for the alignment).
    BLTU { rs1: u8, rs2: u8, imm: u16 },

    // B - Branch Greater Than
    // Takes the branch (pc + sign extended imm) if the registers rs1 >= rs2 on signed comparison.
    // The imm is shifted to have an implicit LSB making the imm 13 bits long (to account for the alignment).
    BGE { rs1: u8, rs2: u8, imm: u16 },

    // B - Branch Greater Than Unsigned
    // Takes the branch (pc + sign extended imm) if the registers rs1 >= rs2 on unsigned comparison.
    // The imm is shifted to have an implicit LSB making the imm 13 bits long (to account for the alignment).
    BGEU { rs1: u8, rs2: u8, imm: u16 },

    // R - Add
    // Add rs2 to register rs1, and store the result in register rd.
    ADD { rd: u8, rs1: u8, rs2: u8 },

    // I - Add Immediate
    // Add sign-extended 12-bit immediate to register rs1, and store the result in register rd.
    ADDI { rd: u8, rs1: u8, imm: u16 },

    // R - Subtract
    // Subtract rs2 from register rs1, and store the result in register rd.
    SUB { rd: u8, rs1: u8, rs2: u8 },

    // I - Load Word
    // Loads a 32-bit value from memory, then storing in rd (rd is calculated using signed arithmetic).
    // The effective address is obtained by adding register rs1 to the sign-extended 12-bit offset
    LW { rd: u8, rs1: u8, imm: u16 },

    // S - Store Word
    // Stores a 32-bit value from register rs2 to memory, the effective address is obtained by adding
    // register rs1 to the sign-extended 12-bit offset imm.
    SW { rs1: u8, rs2: u8, imm: u16 },
}

#[derive(Debug, Error)]
pub enum InstError {
    #[error(transparent)]
    State(#[from] state::Error),
}

impl Inst {
    // Executes the instruction on the state and returns a Result with the updated value of
    // PC. If None was passed, it is expected that the machine increments to the next instruction.
    pub fn execute<const M: usize>(self, state: &mut State<M>) -> Result<Option<u32>, InstError> {
        match self {
            Inst::LUI { rd, imm } => {
                state.set_r(rd, imm)?;

                Ok(None)
            }

            Inst::AUIPC { rd, imm } => {
                let a = state.get_pc();
                state.set_r(rd, add(a, imm))?;

                Ok(None)
            }

            // TODO: Check for alignment and throw exception.
            Inst::JAL { rd, imm } => {
                let current_pc = state.get_pc();

                let offset = sign_extend_21(imm);
                let loc = add(current_pc, offset);

                state.set_r(rd, current_pc + 4)?;
                Ok(Some(loc))
            }

            // TODO: Check for alignment and throw exception even though we assume it.
            Inst::JALR { rd, rs1, imm } => {
                let current_pc = state.get_pc();

                let base = state.get_r(rs1)?;
                let offset = sign_extend_12(imm);
                let addr = add(base, offset) >> 1 << 1;

                state.set_r(rd, current_pc + 4)?;
                Ok(Some(addr))
            }

            Inst::BEQ { rs1, rs2, imm } => branch(state, rs1, rs2, imm, |a, b| a == b),

            Inst::BNE { rs1, rs2, imm } => branch(state, rs1, rs2, imm, |a, b| a != b),

            Inst::BLT { rs1, rs2, imm } => branch(state, rs1, rs2, imm, |a, b| {
                // Even in case of negative numbers, the two's complement of a smaller number
                // will still be smaller than the other number.
                if (a >> 31) == (b >> 31) {
                    a < b
                } else {
                    (a >> 31) == 1
                }
            }),

            Inst::BLTU { rs1, rs2, imm } => branch(state, rs1, rs2, imm, |a, b| a < b),

            Inst::BGE { rs1, rs2, imm } => branch(state, rs1, rs2, imm, |a, b| {
                // Even in case of negative numbers, the two's complement of a smaller number
                // will still be smaller than the other number.
                if (a >> 31) == (b >> 31) {
                    !(a < b)
                } else {
                    (a >> 31) == 0
                }
            }),

            Inst::BGEU { rs1, rs2, imm } => branch(state, rs1, rs2, imm, |a, b| !(a < b)),

            Inst::ADD { rd, rs1, rs2 } => {
                let a = state.get_r(rs1)?;
                let b = state.get_r(rs2)?;
                state.set_r(rd, add(a, b))?;

                Ok(None)
            }

            Inst::ADDI { rd, rs1, imm } => {
                let a = state.get_r(rs1)?;
                let b = sign_extend_12(imm);
                state.set_r(rd, add(a, b))?;

                Ok(None)
            }

            Inst::SUB { rd, rs1, rs2 } => {
                todo!();
            }

            Inst::LW { rd, rs1, imm } => {
                let a = state.get_r(rs1)?;
                let b = sign_extend_12(imm);

                let val = state.get_mem_u32(add(a, b))?;
                state.set_r(rd, val)?;

                Ok(None)
            }

            Inst::SW { rs1, rs2, imm } => {
                // Stores *rs2 at rs1 + imm.
                let a = state.get_r(rs1)?;
                let b = sign_extend_12(imm);
                let addr = add(a, b);

                let val = state.get_r(rs2)?;
                state.set_mem_u32(addr, val)?;

                Ok(None)
            }
        }
    }
}

// TODO: Check for alignment of the jump address if the branch will be taken.
// It needs to be on a 4 byte boundary.
fn branch<const M: usize, C: Fn(u32, u32) -> bool>(
    state: &State<M>,
    rs1: u8,
    rs2: u8,
    imm: u16,
    cmp: C,
) -> Result<Option<u32>, InstError> {
    let a = state.get_r(rs1)?;
    let b = state.get_r(rs2)?;
    if cmp(a, b) {
        let offset = sign_extend_13(imm);
        let addr = add(state.get_pc(), offset);
        Ok(Some(addr))
    } else {
        Ok(None)
    }
}

#[inline]
fn add(a: u32, b: u32) -> u32 {
    a.wrapping_add(b)
}

#[inline]
fn sign_extend_12(val: u16) -> u32 {
    if (val >> 11) == 1 {
        (u32::MAX << 12) | val as u32
    } else {
        val as u32
    }
}

#[inline]
fn sign_extend_13(val: u16) -> u32 {
    if (val >> 12) == 1 {
        (u32::MAX << 13) | val as u32
    } else {
        val as u32
    }
}

#[inline]
fn sign_extend_21(val: u32) -> u32 {
    if (val >> 20) == 1 {
        (u32::MAX << 21) | val as u32
    } else {
        val as u32
    }
}
