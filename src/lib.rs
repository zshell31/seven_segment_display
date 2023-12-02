#![allow(incomplete_features)]
#![allow(clippy::let_and_return)]
#![allow(clippy::type_complexity)]
#![feature(generic_const_exprs)]
#![feature(const_trait_impl)]
#![feature(generic_arg_infer)]
pub mod active;
pub mod counter;
pub mod signal_ext;
pub mod ss_display;
pub mod system;

use active::{Active, High, Low};
use counter::Counter;
use ferrum_hdl::{
    array::Array,
    bitpack::BitPack,
    cast::Cast,
    domain::Clock,
    signal::{reg_en, Reset, Signal},
    unsigned::{u, Unsigned},
};
use signal_ext::rise_rate;
use system::System;

pub fn top_module(
    clk: Clock<System>,
    rst: Reset<System>,
    btn: Signal<System, Array<8, bool>>,
) -> (
    Signal<System, Array<4, Active<High>>>,
    Signal<System, Array<7, Active<High>>>,
    Signal<System, Active<High>>,
) {
    let seg = Signal::lift(u::MAX.repack::<Array<_, bool>>().cast());
    let dp = Signal::lift(false.cast());

    let fast = rise_rate::<System, 512>(clk, rst.clone());

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

    let anodes = reg_en(
        clk,
        rst.clone(),
        slow.clone(),
        Counter::<_>::new(),
        |cnt: Counter<4>| {
            let (cnt, _) = cnt.succ();
            cnt
        },
    )
    .map(|cnt| cnt.one_hot().cast());

    (anodes, seg, dp)
}
