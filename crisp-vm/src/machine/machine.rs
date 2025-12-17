use thiserror::Error;

use crate::machine::{
    instructions::{self, decode},
    state,
};

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    State(#[from] state::Error),

    #[error(transparent)]
    Decode(#[from] decode::Error),

    #[error(transparent)]
    Execute(#[from] instructions::InstError),
}

pub struct Machine<const M: usize> {
    pub state: state::State<M>,
}

impl<const M: usize> Machine<M> {
    pub fn new(state: state::State<M>) -> Self {
        Machine { state }
    }

    pub fn fetch_decode(&self) -> Result<instructions::Inst, Error> {
        let inst = self.state.get_mem_u32(self.state.get_pc())?;
        Ok(instructions::decode(inst)?)
    }

    pub fn log_r(&self) {
        for i in 0..31 {
            log::info!(
                target: "stat",
                "x{:02} {:x}",
                i + 1,
                self.state.get_r(i + 1).expect("could not fetch value"),
            );
        }
    }

    pub fn run(&mut self) -> Result<(), Error> {
        log::debug!(target: "loop", "running machine",);

        let mut cycles = 0;
        loop {
            cycles += 1;
            log::debug!(target: "loop", "--------- {} ---------", cycles);

            let pc = self.state.get_pc();

            log::debug!(target: "loop", "fetch_decode pc:{:x}", pc);
            let inst = self.fetch_decode()?;

            if matches!(inst, instructions::Inst::ECALL) {
                self.log_r();
            }

            match inst.execute(&mut self.state)? {
                Some(pc) => self.state.set_pc(pc),
                None => self.state.set_pc(pc + 4),
            }
        }
    }
}
