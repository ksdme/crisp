use thiserror::Error;

use crate::machine::instructions::Inst;

#[derive(Debug, Error)]
pub enum Error {
    #[error("unknown instruction")]
    UnknownInst,
}

pub fn decode(inst: u32) -> Result<Inst, Error> {
    match inst & 0b1_111_111 {
        // R instructions.
        0b0_110_011 => {
            let rd = select(inst, 7, 5) as u8;
            let f3 = select(inst, 12, 3) as u8;
            let rs1 = select(inst, 15, 5) as u8;
            let rs2 = select(inst, 20, 5) as u8;
            let f7 = select(inst, 25, 7) as u8;

            match (f3, f7) {
                (0, 0) => Ok(Inst::Add { rd, rs1, rs2 }),
                (0, 0b100_000) => Ok(Inst::Sub { rd, rs1, rs2 }),
                _ => Err(Error::UnknownInst),
            }
        }

        // I instructions.
        0b0_010_011 => {
            let rd = select(inst, 7, 5) as u8;
            let f3 = select(inst, 12, 3) as u8;
            let rs1 = select(inst, 15, 5) as u8;
            let imm = select(inst, 20, 12) as u16;

            match f3 {
                0 => Ok(Inst::AddI { rd, rs1, imm }),
                0b010 => Ok(Inst::LoadWord { rd, rs1, imm }),
                _ => Err(Error::UnknownInst),
            }
        }
        _ => Err(Error::UnknownInst),
    }
}

#[inline]
fn select(n: u32, shift: u8, width: u8) -> u32 {
    (n >> shift) & ((1 << width) - 1)
}
