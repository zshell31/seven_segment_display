use ferrum_hdl::{
    domain::{clk_divider, hz_to_period, Clock, ClockDomain},
    signal::{reg, Reset, Signal},
};

use crate::counter::{counter, Counter};

#[inline]
pub fn rise_every<D: ClockDomain, const PS: usize>(
    clk: Clock<D>,
    rst: Reset<D>,
) -> Signal<D, bool>
where
    [(); counter(PS)]:,
{
    reg(
        clk,
        rst,
        (Counter::<PS>::default(), false),
        |(counter, _)| counter.succ(),
    )
    .map(|(_, en)| en)
}

#[inline]
pub fn rise_period<D: ClockDomain, const PS: usize>(
    clk: Clock<D>,
    rst: Reset<D>,
) -> Signal<D, bool>
where
    [(); counter(clk_divider::<D>(PS))]:,
{
    rise_every::<D, { clk_divider::<D>(PS) }>(clk, rst)
}

#[inline]
pub fn rise_rate<D: ClockDomain, const RATE: usize>(
    clk: Clock<D>,
    rst: Reset<D>,
) -> Signal<D, bool>
where
    [(); counter(clk_divider::<D>(hz_to_period(RATE)))]:,
{
    rise_period::<D, { hz_to_period(RATE) }>(clk, rst)
}
