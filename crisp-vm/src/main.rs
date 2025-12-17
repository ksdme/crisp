use crate::machine::state::State;

mod machine;

fn main() {
    env_logger::init();

    let state = State::<1_048_576>::default();
    let mut machine = machine::Machine::new(state);
    machine.run().expect("could not run machine");
}

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use std::path::PathBuf;

    use crate::machine::{Error, Machine, instructions, state::State};

    fn run_riscv_test(bytes: &[u8]) {
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

    #[rstest]
    fn test_riscv_tests_test(#[files("tests/**/*.bin")] path: PathBuf) {
        let bin = std::fs::read(path).expect("could not read bin");
        run_riscv_test(bin.as_slice());
    }
}
