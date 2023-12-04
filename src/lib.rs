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
    signal::{reg_en, Reset, Signal},
    unsigned::u,
};
use round_robin::RoundRobin;
use signal_ext::rise_rate;
use system::{Params, System};

pub fn top_module(
    clk: Clock<System>,
    rst: Reset<System>,
    btn: Signal<System, Array<8, bool>>,
) -> (
    Signal<System, Array<4, Active<High>>>,
    Signal<System, Array<7, Active<High>>>,
    Signal<System, Active<High>>,
)
where
    [bool; 4]: BitPack<Packed = BitVec<4>>,
{
    let seg = Signal::lift(u::MAX.repack::<Array<_, bool>>().cast());
    let dp = Signal::lift(false.cast());

    let fast = rise_rate::<System, { <System as Params>::RATE }>(clk, rst.clone());

    let cnt = {
        let rst = rst.clone();
        let fast = fast.clone();

        btn.and_then(move |btn| {
            let speed = btn.value().repack();
            reg_en(clk, rst, fast, 0_u8, move |cnt| {
                if cnt >= speed {
                    0_u8
                } else {
                    cnt + 1
                }
            })
        })
    };
    let slow = fast.clone().and(cnt.eq(0));

    let anodes = RoundRobin::<_>::signal(clk, rst.clone(), slow.clone())
        .map(|cnt| cnt.selector().cast());

    (anodes, seg, dp)
}
