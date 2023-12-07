use ferrum_hdl::{
    cast::Cast,
    domain::{Clock, TestSystem4},
    signal::Signal,
    simulation::Simulate,
    unsigned::Unsigned,
};
use seven_segment_display::shift_register;

fn main() {
    let clk = Clock::new();
    let rst = Signal::reset();
    let (next, next_signal) = Signal::source(false);
    let (data, data_signal) = Signal::source(0_u8.cast());

    let mut sr4 = shift_register::<TestSystem4, 4, Unsigned<3>>(
        clk,
        rst,
        next_signal.watch("next"),
        0_u8.cast(),
        data_signal.watch("data"),
    )
    .simulate();

    for values in sr4.by_ref().take(4) {
        println!("values: {:?}", values);
    }

    next.revert();

    println!("values: {:?}", sr4.next_cycle());
    data.with(|value| value + 1_u8);

    println!("values: {:?}", sr4.next_cycle());
    data.with(|value| value + 1_u8);

    println!("values: {:?}", sr4.next_cycle());
    data.with(|value| value + 1_u8);

    println!("values: {:?}", sr4.next_cycle());
    data.with(|value| value + 1_u8);

    println!("values: {:?}", sr4.next_cycle());
    data.with(|value| value + 1_u8);

    println!("values: {:?}", sr4.next_cycle());
    data.with(|value| value + 1_u8);

    next.revert();
    for values in sr4.by_ref().take(4) {
        data.with(|value| value + 1_u8);
        println!("values: {:?}", values);
    }

    next.revert();
    println!("values: {:?}", sr4.next_cycle());
    println!("values: {:?}", sr4.next_cycle());
}
