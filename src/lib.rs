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

use active::{Active, High};
use ferrum_hdl::{
    array::Array,
    bitpack::BitPack,
    bitvec::BitVec,
    cast::Cast,
    domain::Clock,
    signal::{Reset, Signal},
    unsigned::Unsigned,
};
use round_robin::RoundRobin;
use signal_ext::rise_period;
use system::{Params, System};

use crate::ss_display::SSDisplay;

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

    let period = rise_period::<System, { <System as Params>::PERIOD }>(clk, rst.clone());
    let rr = RoundRobin::signal(clk, rst.clone(), period);

    let seg = rr.clone().map(|rr| rr.selector().cast());
    let anodes = rr
        .clone()
        .map(move |rr| SSDisplay::encode(rr.mux(&digits)).repack());
    let dp = Signal::lift(false.cast());

    (seg, anodes, dp)
}
