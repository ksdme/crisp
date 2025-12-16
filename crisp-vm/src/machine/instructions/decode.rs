use thiserror::Error;

use crate::machine::instructions::Inst;

#[derive(Debug, Error)]
pub enum Error {
    #[error("unknown instruction")]
    UnknownInst,
}

// TODO: Both 0 and u32::MAX are illegal instructions.
pub fn decode(inst: u32) -> Result<Inst, Error> {
    match inst & 0b1_111_111 {
        // U instuctions.
        0b0_110_111 => {
            let rd = select(inst, 7, 5) as u8;
            let imm = select(inst, 12, 20) << 12;
            Ok(Inst::LUI { rd, imm })
        }

        0b0_010_111 => {
            let rd = select(inst, 7, 5) as u8;
            let imm = select(inst, 12, 20) << 12;
            Ok(Inst::AUIPC { rd, imm })
        }

        // J instructions.
        0b1_101_111 => {
            let rd = select(inst, 7, 5) as u8;

            // imm[20|10:1|11|19:12]
            let imm = ((inst >> 31) << 20)
                | (((inst >> 12) & 0b11_111_111) << 12)
                | (((inst >> 20) & 0b1) << 11)
                | (((inst >> 21) & 0b1_111_111_111) << 1);

            Ok(Inst::JAL { rd, imm })
        }

        // R instructions.
        0b0_110_011 => {
            let rd = select(inst, 7, 5) as u8;
            let f3 = select(inst, 12, 3) as u8;
            let rs1 = select(inst, 15, 5) as u8;
            let rs2 = select(inst, 20, 5) as u8;
            let f7 = select(inst, 25, 7) as u8;

            match (f3, f7) {
                (0, 0) => Ok(Inst::ADD { rd, rs1, rs2 }),
                (0, 0b100_000) => Ok(Inst::SUB { rd, rs1, rs2 }),
                _ => Err(Error::UnknownInst),
            }
        }

        // I instructions.
        0b1_100_111 => {
            let (rd, f3, rs1, imm) = unpack_i(inst);

            match f3 {
                0 => Ok(Inst::JALR { rd, rs1, imm }),
                _ => Err(Error::UnknownInst),
            }
        }

        // B instructions.
        0b1_100_011 => {
            let f3 = select(inst, 12, 3) as u8;
            let rs1 = select(inst, 15, 5) as u8;
            let rs2 = select(inst, 20, 5) as u8;
            let imm = (((inst >> 31) << 12)
                | (((inst >> 7) & 1) << 11)
                | (((inst >> 25) & 0b111_111) << 5)
                | ((inst >> 8) & 0b1_111) << 1) as u16;

            match f3 {
                0 => Ok(Inst::BEQ { rs1, rs2, imm }),
                1 => Ok(Inst::BNE { rs1, rs2, imm }),
                0b100 => Ok(Inst::BLT { rs1, rs2, imm }),
                0b101 => Ok(Inst::BGE { rs1, rs2, imm }),
                0b110 => Ok(Inst::BLTU { rs1, rs2, imm }),
                0b111 => Ok(Inst::BGEU { rs1, rs2, imm }),
                _ => Err(Error::UnknownInst),
            }
        }

        0b0_010_011 => {
            let (rd, f3, rs1, imm) = unpack_i(inst);

            match f3 {
                0 => Ok(Inst::ADDI { rd, rs1, imm }),
                0b010 => Ok(Inst::LW { rd, rs1, imm }),
                _ => Err(Error::UnknownInst),
            }
        }

        // S instructions.
        0b0_100_011 => {
            let imm = (((inst >> 25) << 5) | ((inst >> 7) & 0b11_111)) as u16;
            let f3 = select(inst, 12, 3) as u8;
            let rs1 = select(inst, 15, 5) as u8;
            let rs2 = select(inst, 20, 5) as u8;

            match f3 {
                0b010 => Ok(Inst::SW { rs1, rs2, imm }),
                _ => Err(Error::UnknownInst),
            }
        }

        _ => Err(Error::UnknownInst),
    }
}

// Unpacks an I type instruction.
#[inline]
fn unpack_i(inst: u32) -> (u8, u8, u8, u16) {
    (
        select(inst, 7, 5) as u8,
        select(inst, 12, 3) as u8,
        select(inst, 15, 5) as u8,
        select(inst, 20, 12) as u16,
    )
}

#[inline]
fn select(n: u32, shift: u8, width: u8) -> u32 {
    (n >> shift) & ((1 << width) - 1)
}
