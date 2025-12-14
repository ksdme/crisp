use thiserror::Error;

use crate::machine::state::{self, State};

#[derive(Debug)]
pub enum Inst {
    Add { rd: u8, rs1: u8, rs2: u8 },
    AddI { rd: u8, rs1: u8, imm: u16 },
    Sub { rd: u8, rs1: u8, rs2: u8 },
    LoadWord { rd: u8, rs1: u8, imm: u16 },
}

#[derive(Debug, Error)]
pub enum InstError {
    #[error(transparent)]
    State(#[from] state::Error),
}

impl Inst {
    pub fn execute<const M: usize>(self, state: &mut State<M>) -> Result<(), InstError> {
        match self {
            Inst::Add { rd, rs1, rs2 } => {
                let a = state.get_r(rs1)?;
                let b = state.get_r(rs2)?;
                state.set_r(rd, a.wrapping_add(b))?;
            }

            Inst::AddI { rd, rs1, imm } => {
                todo!();
            }

            Inst::Sub { rd, rs1, rs2 } => {
                todo!();
            }

            Inst::LoadWord { rd, rs1, imm } => {
                let a = state.get_r(rs1)?;
                let b = sign_extend_12(imm);
                let val = state.get_mem_u32(a.wrapping_add(b))?;
                state.set_r(rd, val)?;
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
