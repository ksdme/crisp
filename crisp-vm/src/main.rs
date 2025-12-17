use crate::machine::{instructions, state::State};

mod machine;

fn main() {
    env_logger::init();

    let state = State::<1_048_576>::from(include_bytes!("../blobs/rv32ui-p-add.bin"));
    let mut machine = machine::Machine::new(state);
    machine.run().expect("could not run machine");
}

#[cfg(test)]
mod tests {
    use crate::machine::{Error, Machine, instructions, state::State};

    fn run_riscv_test<const B: usize>(bytes: &[u8; B]) {
        let state = State::<10_000>::from(bytes);
        let mut machine = Machine::new(state);

        assert!(matches!(
            machine.run(),
            Err(Error::Execute(instructions::InstError::Suspend)),
        ));
        assert_eq!(machine.state.get_r(3).expect("could not gp"), 1);
        assert_eq!(machine.state.get_r(10).expect("could not a0"), 0);
        assert_eq!(machine.state.get_r(17).expect("could not a7"), 93);
    }

    #[test]
    fn test_add() {
        run_riscv_test(include_bytes!("../blobs/rv32ui-p-add.bin"));
    }
}
