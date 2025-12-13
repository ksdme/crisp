use thiserror::Error;

use crate::machine::state::{self, State};

#[derive(Debug)]
pub enum Inst {
    Add { rd: u8, rs1: u8, rs2: u8 },
    AddI { rd: u8, rs1: u8, imm: u16 },
    Sub { rd: u8, rs1: u8, rs2: u8 },
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
                let o1 = state.get_r(rs1)?;
                let o2 = state.get_r(rs2)?;
                Ok(state.set_r(rd, o1 + o2)?)
            }

            Inst::AddI { rd, rs1, imm } => {
                todo!();
            }

            Inst::Sub { rd, rs1, rs2 } => {
                todo!();
            }
        }
    }
}
