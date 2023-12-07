#![allow(incomplete_features)]
#![allow(clippy::let_and_return)]
#![allow(clippy::type_complexity)]
#![feature(generic_const_exprs)]
#![feature(generic_arg_infer)]
pub mod active;
pub mod round_robin;
pub mod signal_ext;
pub mod ss_display;
pub mod system;

use std::fmt::Display;

use active::{Active, High};
use ferrum_hdl::{
    array::{Array, ArrayExt},
    bitpack::BitPack,
    bitvec::BitVec,
    cast::Cast,
    const_helpers::ConstConstr,
    domain::{Clock, ClockDomain},
    index::idx_constr,
    signal::{reg_en, Bundle, Enable, Reset, Signal, SignalValue},
    unsigned::Unsigned,
};
use round_robin::RoundRobin;
use signal_ext::rise_period;
use system::{Params, System};

use crate::ss_display::SSDisplay;

pub fn shift_register<D: ClockDomain, const N: usize, T: SignalValue>(
    clk: Clock<D>,
    rst: Reset<D>,
    next: Enable<D>,
    init: T,
    data: Signal<D, T>,
) -> Signal<D, Array<N, T>>
where
    ConstConstr<{ idx_constr(N) }>:,
{
    let mut prev = data;

    Array::<N, Signal<D, T>>::make(|_| {
        let block =
            prev.and_then(|data| reg_en(clk, &rst, &next, &init, move |_| data.value()));
        prev = block.clone();
        block
    })
    .bundle()
}

pub fn top_module(
    clk: Clock<System>,
    rst: Reset<System>,
) -> (
    Signal<System, Array<4, Active<High>>>,
    Signal<System, Array<7, Active<High>>>,
    Signal<System, Active<High>>,
)
where
    [bool; 4]: BitPack<Packed = BitVec<4>>,
{
    let digits: Array<4, Unsigned<4>> = [1_u8, 2, 3, 4].cast();

    let period = rise_period::<System, { <System as Params>::PERIOD }>(clk, &rst);
    let rr = RoundRobin::signal(clk, &rst, &period);

    let seg = rr.map(|rr| rr.selector().cast());
    let anodes = rr.map(move |rr| SSDisplay::encode(rr.mux(&digits)).repack());
    let dp = Signal::lift(false.cast());

    (seg, anodes, dp)
}
