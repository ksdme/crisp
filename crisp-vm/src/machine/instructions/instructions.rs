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

    // I - Load Byte
    // Loads a byte at the address rs1 + sign extended imm, sign extends it and stores it in rd.
    LB { rs1: u8, rd: u8, imm: u16 },

    // I - Load Half-word
    // Loads half word (2 bytes) starting at the address rs1 + sign extended imm, sign extends it
    // and stores it in rd. Uses little endian.
    LH { rs1: u8, rd: u8, imm: u16 },

    // I - Load Word
    // Loads 4 byte value from memory starting at rs1 + sign extended imm, then, stores it in rd.
    // Uses little endian.
    LW { rd: u8, rs1: u8, imm: u16 },

    // I - Load Byte Unsigned
    // Loads a byte from the address rs1 + sign extended imm and zero extends the value
    // before storing it in rd.
    LBU { rd: u8, rs1: u8, imm: u16 },

    // I - Load Half-word Unsigned
    // Loads 2 bytes starting from the address rs1 + sign extended imm and zero extends the value
    // before storing it in rd.
    LHU { rd: u8, rs1: u8, imm: u16 },

    // S - Store Byte
    // Stores the lower byte from rs2 in memory at rs1 + sign extended imm.
    SB { rs1: u8, rs2: u8, imm: u16 },

    // S - Store Half-word
    // Stores the lower 2 bytes from rs2 in memory starting at rs1 + sign extended imm.
    // Uses little endian.
    SH { rs1: u8, rs2: u8, imm: u16 },

    // S - Store Half-word
    // Stores the 4 bytes from rs2 in memory starting at rs1 + sign extended imm.
    SW { rs1: u8, rs2: u8, imm: u16 },

    // I - Add Immediate
    // Add sign-extended 12-bit immediate to rs1, and store the result in rd.
    ADDI { rd: u8, rs1: u8, imm: u16 },

    // I - Set Less Than Immediate
    // Set 1 to rd if rs1 < sign extended imm with signed comparison.
    SLTI { rd: u8, rs1: u8, imm: u16 },

    // I - Set Less Than Immediate Unsigned
    // Set 1 to rd if rs1 < sign extended imm with unsigned comparison.
    SLTIU { rd: u8, rs1: u8, imm: u16 },

    // I - XOR Immediate
    // Stores the value of bitwise XOR of rs1 and sign extended imm in rd.
    XORI { rd: u8, rs1: u8, imm: u16 },

    // I - OR Immediate
    // Stores the value of bitwise OR of rs1 and sign extended imm in rd.
    ORI { rd: u8, rs1: u8, imm: u16 },

    // I - AND Immediate
    // Stores the value of bitwise AND of rs1 and sign extended imm in rd.
    ANDI { rd: u8, rs1: u8, imm: u16 },

    // I - Shift Left Logical Immediate
    // Shifts the value in rs1 left by shamt and stores the value in rd.
    SLLI { rd: u8, rs1: u8, shamt: u8 },

    // I - Shift Right Logical Immediate
    // Shifts the value in rs1 right by shamt and stores the value in rd.
    SRLI { rd: u8, rs1: u8, shamt: u8 },

    // I - Shift Right Arithmetic Immediate
    // Right shifts the value of rs1 by shamt while retaining the sign bit and
    // stores that value in rd.
    SRAI { rd: u8, rs1: u8, shamt: u8 },

    // R - Add
    // Add rs2 to register rs1, and store the result in register rd.
    ADD { rd: u8, rs1: u8, rs2: u8 },

    // R - Subtract
    // Subtract rs2 from register rs1, and store the result in register rd.
    SUB { rd: u8, rs1: u8, rs2: u8 },

    // R - Shift Left Logic
    // Shit the value in rs1 to left by *rs2.
    SLL { rd: u8, rs1: u8, rs2: u8 },

    // R - Shift Right Logic
    // Shift the value in rs1 right by rs2 bits.
    SRL { rd: u8, rs1: u8, rs2: u8 },

    // R - Shift Right Arithmetic
    // Shift the value in rs1 right by rs2 bits using the original sign bit for filling
    // the vacancies.
    SRA { rd: u8, rs1: u8, rs2: u8 },

    // R - Set Less Than
    // Set 1 in rd if *rs1 < *rs2 during signed comparison else set 0.
    SLT { rd: u8, rs1: u8, rs2: u8 },

    // R - Set Less Than Unsigned
    // Set 1 in rd if *rs1 < *rs2 during unsigned comparison else set 0.
    SLTU { rd: u8, rs1: u8, rs2: u8 },

    // R - XOR
    // Stores the result of XORing *rs1 and *rs2 result in rd.
    XOR { rd: u8, rs1: u8, rs2: u8 },

    // R - OR
    // Store the value of *rs1 | *rs2 in rd.
    OR { rd: u8, rs1: u8, rs2: u8 },

    // R - AND
    // Store the value of *rs1 & *rs2 in rd.
    AND { rd: u8, rs1: u8, rs2: u8 },

    // I - ECALL
    // Trigger a trap into the runtime.
    ECALL,

    // All the CSRR* instructions.
    IGNORE,
}

#[derive(Debug, Error)]
pub enum InstError {
    #[error(transparent)]
    State(#[from] state::Error),

    #[error("suspend")]
    Suspend,
}

// Sign extends a number to be a negative value with a different bit size if the original
// value was negative.
macro_rules! sign_extend {
    ($size:expr, $val:expr) => {
        if ($val >> ($size - 1)) == 1 {
            (u32::MAX << $size) | $val as u32
        } else {
            $val as u32
        }
    };
}

macro_rules! add {
    ($a:expr, $b:expr) => {
        $a.wrapping_add($b)
    };
}

macro_rules! shl {
    ($a:expr, $b:expr) => {
        $a.wrapping_shl($b)
    };
}

macro_rules! shr {
    ($a:expr, $b:expr) => {
        $a.wrapping_shr($b)
    };
}

impl Inst {
    // Executes the instruction on the state and returns a Result with the updated value of
    // PC. If None was passed, it is expected that the machine increments to the next instruction.
    pub fn execute<const M: usize>(self, state: &mut State<M>) -> Result<Option<u32>, InstError> {
        match self {
            // Upper immediates.
            Inst::LUI { rd, imm } => {
                log::debug!(target: "exec", "lui rd:{:x} imm:{:x}", rd, imm);

                state.set_r(rd, imm)?;

                Ok(None)
            }

            Inst::AUIPC { rd, imm } => {
                log::debug!(target: "exec", "auipc rd:{:x} imm:{:x}", rd, imm);

                let val = add!(state.get_pc(), imm);
                state.set_r(rd, val)?;

                Ok(None)
            }

            // Jumps.
            // TODO: Check for alignment and throw exception.
            Inst::JAL { rd, imm } => {
                log::debug!(target: "exec", "jal rd:{:x} imm:{:x}", rd, imm);

                let current_pc = state.get_pc();
                state.set_r(rd, current_pc + 4)?;

                let loc = add!(current_pc, sign_extend!(21, imm));
                Ok(Some(loc))
            }

            // TODO: Check for alignment and throw exception even though we assume it.
            Inst::JALR { rd, rs1, imm } => {
                log::debug!(target: "exec", "jalr rd:{:x} rs1:{:x} imm:{:x}", rd, rs1, imm);

                let addr = add!(state.get_r(rs1)?, sign_extend!(12, imm));
                let addr = addr >> 1 << 1;

                let current_pc = state.get_pc();
                state.set_r(rd, current_pc + 4)?;

                Ok(Some(addr))
            }

            // Branches.
            Inst::BEQ { rs1, rs2, imm } => {
                log::debug!(target: "exec", "beq rs1:{:x} rs2:{:x} imm:{:x}", rs1, rs2, imm);
                branch(state, rs1, rs2, imm, |a, b| a == b)
            }

            Inst::BNE { rs1, rs2, imm } => {
                log::debug!(target: "exec", "bne rs1:{:x} rs2:{:x} imm:{:x}", rs1, rs2, imm);
                branch(state, rs1, rs2, imm, |a, b| a != b)
            }

            Inst::BLT { rs1, rs2, imm } => {
                log::debug!(target: "exec", "blt rs1:{:x} rs2:{:x} imm:{:x}", rs1, rs2, imm);
                branch(state, rs1, rs2, imm, |a, b| signed_cmp_lt(a, b))
            }

            Inst::BLTU { rs1, rs2, imm } => {
                log::debug!(target: "exec", "bltu rs1:{:x} rs2:{:x} imm:{:x}", rs1, rs2, imm);
                branch(state, rs1, rs2, imm, |a, b| a < b)
            }

            Inst::BGE { rs1, rs2, imm } => {
                log::debug!(target: "exec", "bge rs1:{:x} rs2:{:x} imm:{:x}", rs1, rs2, imm);
                branch(state, rs1, rs2, imm, |a, b| signed_cmp_gt(a, b))
            }

            Inst::BGEU { rs1, rs2, imm } => {
                log::debug!(target: "exec", "bgeu rs1:{:x} rs2:{:x} imm:{:x}", rs1, rs2, imm);
                branch(state, rs1, rs2, imm, |a, b| !(a < b))
            }

            // Loads
            Inst::LB { rs1, rd, imm } => {
                log::debug!(target: "exec", "lb rd:{:x} rs1:{:x} imm:{:x}", rd, rs1, imm);

                let addr = add!(state.get_r(rs1)?, sign_extend!(12, imm));
                let val = state.get_mem_u8(addr)?;
                state.set_r(rd, sign_extend!(8, val))?;

                Ok(None)
            }

            Inst::LH { rs1, rd, imm } => {
                log::debug!(target: "exec", "lh rd:{:x} rs1:{:x} imm:{:x}", rd, rs1, imm);

                let base_addr = add!(state.get_r(rs1)?, sign_extend!(12, imm));
                let val = state.get_mem_u16(base_addr)?;
                state.set_r(rd, sign_extend!(16, val))?;

                Ok(None)
            }

            Inst::LW { rd, rs1, imm } => {
                log::debug!(target: "exec", "lw rd:{:x} rs1:{:x} imm:{:x}", rd, rs1, imm);

                let base_addr = add!(state.get_r(rs1)?, sign_extend!(12, imm));
                let val = state.get_mem_u32(base_addr)?;
                state.set_r(rd, val)?;

                Ok(None)
            }

            Inst::LBU { rd, rs1, imm } => {
                log::debug!(target: "exec", "lbu rd:{:x} rs1:{:x} imm:{:x}", rd, rs1, imm);

                let addr = add!(state.get_r(rs1)?, sign_extend!(12, imm));
                state.set_r(rd, state.get_mem_u8(addr)? as u32)?;

                Ok(None)
            }

            Inst::LHU { rd, rs1, imm } => {
                log::debug!(target: "exec", "lhu rd:{:x} rs1:{:x} imm:{:x}", rd, rs1, imm);

                let base_addr = add!(state.get_r(rs1)?, sign_extend!(12, imm));
                state.set_r(rd, state.get_mem_u16(base_addr)? as u32)?;

                Ok(None)
            }

            // Stores
            Inst::SB { rs1, rs2, imm } => {
                log::debug!(target: "exec", "sb rs1:{:x} rs2:{:x} imm:{:x}", rs1, rs2, imm);

                let addr = add!(state.get_r(rs1)?, sign_extend!(12, imm));
                state.set_mem_u8(addr, state.get_r(rs2)? as u8)?;

                Ok(None)
            }

            Inst::SH { rs1, rs2, imm } => {
                log::debug!(target: "exec", "sh rs1:{:x} rs2:{:x} imm:{:x}", rs1, rs2, imm);

                let base_addr = add!(state.get_r(rs1)?, sign_extend!(12, imm));
                state.set_mem_u16(base_addr, state.get_r(rs2)? as u16)?;

                Ok(None)
            }

            Inst::SW { rs1, rs2, imm } => {
                log::debug!(target: "exec", "sw rs1:{:x} rs2:{:x} imm:{:x}", rs1, rs2, imm);

                let base_addr = add!(state.get_r(rs1)?, sign_extend!(12, imm));
                state.set_mem_u32(base_addr, state.get_r(rs2)?)?;

                Ok(None)
            }

            // Immediate Logical and arithematics.
            Inst::ADDI { rd, rs1, imm } => {
                log::debug!(target: "exec", "addi rd:{:x} rs1:{:x} imm:{:x}", rd, rs1, imm);

                let val = add!(state.get_r(rs1)?, sign_extend!(12, imm));
                state.set_r(rd, val)?;

                Ok(None)
            }

            Inst::SLTI { rd, rs1, imm } => {
                log::debug!(target: "exec", "slti rd:{:x} rs1:{:x} imm:{:x}", rd, rs1, imm);

                let lt = signed_cmp_lt(state.get_r(rs1)?, sign_extend!(12, imm));
                state.set_r(rd, if lt { 1 } else { 0 })?;

                Ok(None)
            }

            Inst::SLTIU { rd, rs1, imm } => {
                log::debug!(target: "exec", "sltiu rd:{:x} rs1:{:x} imm:{:x}", rd, rs1, imm);

                let lt = state.get_r(rs1)? < sign_extend!(12, imm);
                state.set_r(rd, if lt { 1 } else { 0 })?;

                Ok(None)
            }

            Inst::XORI { rd, rs1, imm } => {
                log::debug!(target: "exec", "xori rd:{:x} rs1:{:x} imm:{:x}", rd, rs1, imm);

                let val = state.get_r(rs1)? ^ sign_extend!(12, imm);
                state.set_r(rd, val)?;

                Ok(None)
            }

            Inst::ORI { rd, rs1, imm } => {
                log::debug!(target: "exec", "ori rd:{:x} rs1:{:x} imm:{:x}", rd, rs1, imm);

                let val = state.get_r(rs1)? | sign_extend!(12, imm);
                state.set_r(rd, val)?;

                Ok(None)
            }

            Inst::ANDI { rd, rs1, imm } => {
                log::debug!(target: "exec", "andi rd:{:x} rs1:{:x} imm:{:x}", rd, rs1, imm);

                let val = state.get_r(rs1)? & sign_extend!(12, imm);
                state.set_r(rd, val)?;

                Ok(None)
            }

            // Shifts
            Inst::SLLI { rd, rs1, shamt } => {
                log::debug!(target: "exec", "slli rd:{:x} rs1:{:x} shamt:{:x}", rd, rs1, shamt);

                let val = shl!(state.get_r(rs1)?, shamt as u32);
                state.set_r(rd, val)?;

                Ok(None)
            }

            Inst::SRLI { rd, rs1, shamt } => {
                log::debug!(target: "exec", "srli rd:{:x} rs1:{:x} shamt:{:x}", rd, rs1, shamt);

                let val = shr!(state.get_r(rs1)?, shamt as u32);
                state.set_r(rd, val)?;

                Ok(None)
            }

            Inst::SRAI { rd, rs1, shamt } => {
                log::debug!(target: "exec", "srai rd:{:x} rs1:{:x} shamt:{:x}", rd, rs1, shamt);

                let val = right_shift_arithmetic(state.get_r(rs1)?, shamt as u32);
                state.set_r(rd, val)?;

                Ok(None)
            }

            // Register logical and arithematics.
            Inst::ADD { rd, rs1, rs2 } => {
                log::debug!(target: "exec", "add rd:{:x} rs1:{:x} rs2:{:x}", rd, rs1, rs2);

                let val = add!(state.get_r(rs1)?, state.get_r(rs2)?);
                state.set_r(rd, val)?;

                Ok(None)
            }

            Inst::SUB { rd, rs1, rs2 } => {
                log::debug!(target: "exec", "sub rd:{:x} rs1:{:x} rs2:{:x}", rd, rs1, rs2);

                // Two's complement rs2 and add it to rs1.
                let b = add!(!state.get_r(rs2)?, 1);
                let val = add!(state.get_r(rs1)?, b);
                state.set_r(rd, val)?;

                Ok(None)
            }

            Inst::SLT { rd, rs1, rs2 } => {
                log::debug!(target: "exec", "slt rd:{:x} rs1:{:x} rs2:{:x}", rd, rs1, rs2);

                let lt = signed_cmp_lt(state.get_r(rs1)?, state.get_r(rs2)?);
                state.set_r(rd, if lt { 1 } else { 0 })?;

                Ok(None)
            }

            Inst::SLTU { rd, rs1, rs2 } => {
                log::debug!(target: "exec", "sltu rd:{:x} rs1:{:x} rs2:{:x}", rd, rs1, rs2);

                let lt = state.get_r(rs1)? < state.get_r(rs2)?;
                state.set_r(rd, if lt { 1 } else { 0 })?;

                Ok(None)
            }

            Inst::SLL { rd, rs1, rs2 } => {
                log::debug!(target: "exec", "sll rd:{:x} rs1:{:x} rs2:{:x}", rd, rs1, rs2);

                let val = shl!(state.get_r(rs1)?, state.get_r(rs2)?);
                state.set_r(rd, val)?;

                Ok(None)
            }

            Inst::SRL { rd, rs1, rs2 } => {
                log::debug!(target: "exec", "srl rd:{:x} rs1:{:x} rs2:{:x}", rd, rs1, rs2);

                let val = shr!(state.get_r(rs1)?, state.get_r(rs2)?);
                state.set_r(rd, val)?;

                Ok(None)
            }

            Inst::SRA { rd, rs1, rs2 } => {
                log::debug!(target: "exec", "sra rd:{:x} rs1:{:x} rs2:{:x}", rd, rs1, rs2);

                let val = right_shift_arithmetic(state.get_r(rs1)?, state.get_r(rs2)?);
                state.set_r(rd, val)?;

                Ok(None)
            }

            Inst::XOR { rd, rs1, rs2 } => {
                log::debug!(target: "exec", "xor rd:{:x} rs1:{:x} rs2:{:x}", rd, rs1, rs2);

                let val = state.get_r(rs1)? ^ state.get_r(rs2)?;
                state.set_r(rd, val)?;

                Ok(None)
            }

            Inst::OR { rd, rs1, rs2 } => {
                log::debug!(target: "exec", "or rd:{:x} rs1:{:x} rs2:{:x}", rd, rs1, rs2);

                let val = state.get_r(rs1)? | state.get_r(rs2)?;
                state.set_r(rd, val)?;

                Ok(None)
            }

            Inst::AND { rd, rs1, rs2 } => {
                log::debug!(target: "exec", "and rd:{:x} rs1:{:x} rs2:{:x}", rd, rs1, rs2);

                let val = state.get_r(rs1)? & state.get_r(rs2)?;
                state.set_r(rd, val)?;

                Ok(None)
            }

            // Indicate that we want to suspend execution in some manner here.
            Inst::ECALL => {
                log::debug!(target: "exec", "ecall");
                Err(InstError::Suspend)
            }

            // Fence, FenceI & CSR
            Inst::IGNORE => {
                log::debug!(target: "exec", "ignore");
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
        let addr = add!(state.get_pc(), sign_extend!(13, imm));
        Ok(Some(addr))
    } else {
        Ok(None)
    }
}

// Even in case of negative numbers, the two's complement of a smaller number
// will still be smaller than the other number.
#[inline]
fn signed_cmp_lt(a: u32, b: u32) -> bool {
    if (a >> 31) == (b >> 31) {
        a < b
    } else {
        (a >> 31) == 1
    }
}

// Even in case of negative numbers, the two's complement of a smaller number
// will still be smaller than the other number.
#[inline]
fn signed_cmp_gt(a: u32, b: u32) -> bool {
    if (a >> 31) == (b >> 31) {
        !(a < b)
    } else {
        (a >> 31) == 0
    }
}

// TODO: I bet this can be optimized more.
// Right shift the value while moving in sign bit of the original value instead.
#[inline]
fn right_shift_arithmetic(val: u32, amount: u32) -> u32 {
    let amount = amount % 32;

    if ((val >> 31) == 1) && (amount > 0) {
        (val >> amount) | (u32::MAX << (32 - amount))
    } else {
        val >> amount
    }
}
