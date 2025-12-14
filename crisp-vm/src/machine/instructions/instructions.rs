use thiserror::Error;

use crate::machine::state::{self, State};

// https://docs.openhwgroup.org/projects/cva6-user-manual/01_cva6_user/RISCV_Instructions_RV32I.html
#[derive(Debug)]
pub enum Inst {
    // Add rs2 to register rs1, and store the result in register rd.
    ADD { rd: u8, rs1: u8, rs2: u8 },

    // Add sign-extended 12-bit immediate to register rs1, and store the result in register rd.
    ADDI { rd: u8, rs1: u8, imm: u16 },

    // Subtract rs2 from register rs1, and store the result in register rd.
    SUB { rd: u8, rs1: u8, rs2: u8 },

    // Loads a 32-bit value from memory, then storing in rd (rd is calculated using signed arithmetic).
    // The effective address is obtained by adding register rs1 to the sign-extended 12-bit offset
    LW { rd: u8, rs1: u8, imm: u16 },

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
    pub fn execute<const M: usize>(self, state: &mut State<M>) -> Result<(), InstError> {
        match self {
            Inst::ADD { rd, rs1, rs2 } => {
                let a = state.get_r(rs1)?;
                let b = state.get_r(rs2)?;
                state.set_r(rd, a.wrapping_add(b))?;
            }

            Inst::ADDI { rd, rs1, imm } => {
                todo!();
            }

            Inst::SUB { rd, rs1, rs2 } => {
                todo!();
            }

            Inst::LW { rd, rs1, imm } => {
                let a = state.get_r(rs1)?;
                let b = sign_extend_12(imm);
                let val = state.get_mem_u32(a.wrapping_add(b))?;
                state.set_r(rd, val)?;
            }

            Inst::SW { rs1, rs2, imm } => {
                // Stores *rs2 at rs1 + imm.
                let a = state.get_r(rs1)?;
                let b = sign_extend_12(imm);
                let addr = a.wrapping_add(b);

                let val = state.get_r(rs2)?;
                state.set_mem_u32(addr, val)?;
            }
        }

        Ok(())
    }
}

#[inline]
fn sign_extend_12(val: u16) -> u32 {
    if (val >> 11) == 1 {
        0b11111_11111_11111_11111_00000_00000_00 | val as u32
    } else {
        val as u32
    }
}
