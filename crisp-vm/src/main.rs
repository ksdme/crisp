use crate::machine::state::State;

mod machine;

fn main() {
    let mut state = State::<1_048_576>::default();
    state.set_r(1, 1).expect("could not set r.1");
    state.set_r(2, 1).expect("could not set r.2");
    state
        .set_mem_u32(0, 0b0000000_00001_00010_000_00011_0110011)
        .expect("could not set add");

    let mut machine = machine::Machine::new(state);
    dbg!(machine.fetch_decode().expect("could not decode add"))
        .execute(&mut machine.state)
        .expect("could not execute add");

    dbg!(
        machine
            .state
            .get_r(0b00011)
            .expect("could not read register")
    );
}
