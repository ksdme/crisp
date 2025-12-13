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
}
