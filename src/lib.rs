#![allow(incomplete_features)]
#![allow(clippy::let_and_return)]
#![allow(clippy::type_complexity)]
#![allow(trivial_bounds)]
#![feature(generic_const_exprs)]
#![feature(generic_arg_infer)]
#![feature(trivial_bounds)]
pub mod active;
pub mod round_robin;
#[macro_use]
pub mod signal_ext;
pub mod shift_register;
pub mod ss_display;
pub mod system;

use active::{Active, High};
use ferrum_hdl::{
    array::{Array, ArrayExt},
    bitpack::BitPack,
    bitvec::BitVec,
    cast::Cast,
    const_helpers::{Assert, ConstConstr, IsTrue},
    domain::Clock,
    index::{idx_constr, Idx},
    signal::{reg_en, Reset, Signal},
    unsigned::Unsigned,
};
use round_robin::RoundRobin;
use shift_register::shift_register;
use system::{EnSignals, System};

use crate::ss_display::SSDisplay;

const DIGITS: usize = 4;

// fn priority_encoder<const N: usize>(data: Unsigned<N>) -> Unsigned<{ idx_constr(N) }>
// where
//     ConstConstr<{ idx_constr(N) }>:,
//     Assert<{ N - 1 < N }>: IsTrue,
// {
//     let mut block = 0_u8.cast::<Unsigned<_>>();

//     let blocks = Array::<N, _>::make(|idx| {
//         let bit = data.idx(idx.rev());
//         block = if bit { idx.cast() } else { block.clone() };
//         block.clone()
//     });
//     let last = Idx::from::<{ N - 1 }>();
//     blocks.idx(last)
// }

// pub fn top_module(data: Unsigned<8>) -> Unsigned<3> {
//     priority_encoder(data)
// }

pub fn top_module(
    clk: Clock<System>,
    rst: Reset<System>,
) -> (
    Signal<System, Array<DIGITS, Active<High>>>,
    Signal<System, Array<7, Active<High>>>,
    Signal<System, Active<High>>,
)
where
    [bool; DIGITS]: BitPack<Packed = BitVec<DIGITS>>,
{
    let sr_enable = <System as EnSignals<DIGITS>>::sr_enable(clk, &rst);
    let data = reg_en(clk, &rst, &sr_enable, &0_u8.cast(), |counter| {
        counter + 1_u8
    });
    let digits = shift_register(clk, &rst, &sr_enable, 0_u8.cast(), &data);

    let rr_enable = <System as EnSignals<DIGITS>>::rr_enable(clk, &rst);
    let rr = RoundRobin::signal(clk, &rst, &rr_enable);

    let seg = rr.map(|rr| rr.selector().cast());
    let anodes = digits.and_then(|digits| {
        rr.map(move |rr| {
            let digit: Unsigned<4> = rr.mux(&digits.value());
            SSDisplay::encode(digit.clone()).repack()
        })
    });
    let dp = Signal::lift(false.cast());

    (seg, anodes, dp)
}
