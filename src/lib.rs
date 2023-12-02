#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
#![feature(const_trait_impl)]
pub mod active;
pub mod ss_display;
pub mod system;

use active::{Active, High, Low};
use ferrum_hdl::{array::Array, bitpack::BitPack, signal::Signal};
use ss_display::SSDisplay;
use system::System;

#[allow(clippy::let_and_return)]
#[allow(clippy::type_complexity)]
pub fn top_module() -> (
    Signal<System, Array<4, Active<High>>>,
    Signal<System, Array<7, Active<Low>>>,
    Signal<System, Active<Low>>,
) {
    let anodes = Signal::lift([false, false, false, true].repack());
    let seg = Signal::lift(SSDisplay::five().repack());
    let dp = Signal::lift(false.into());

    (anodes, seg, dp)
}
