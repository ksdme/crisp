use crate::machine::state::State;

mod machine;

fn main() {
    env_logger::init();

    let state = State::<1_048_576>::from(include_bytes!("../blobs/rv32ui-p-add.bin"));
    let mut machine = machine::Machine::new(state);
    machine.run().expect("could not run machine");
}
