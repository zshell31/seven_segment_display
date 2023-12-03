use ferrum_hdl::{
    const_helpers::ConstConstr,
    domain::{clk_divider, hz_to_period, Clock, ClockDomain},
    index::{idx_constr, Idx},
    signal::{reg, Reset, Signal},
};

pub fn rise_every_constr(n: usize) -> usize {
    idx_constr(n)
}

macro_rules! rise_every_constr {
    ($n:expr) => {
        idx_constr($n)
    };
}

#[inline]
pub fn rise_every<D: ClockDomain, const PS: usize>(
    clk: Clock<D>,
    rst: Reset<D>,
) -> Signal<D, bool>
where
    ConstConstr<{ rise_every_constr!(PS) }>:,
{
    reg(clk, rst, (Idx::<PS>::new(), false), |(idx, _)| {
        (idx.clone().succ(), idx.is_max())
    })
    .map(|(_, en)| en)
}

macro_rules! rise_period_constr {
    ($domain:ident, $period:expr) => {
        rise_every_constr!(clk_divider::<$domain>($period))
    };
}

#[inline]
pub fn rise_period<D: ClockDomain, const PS: usize>(
    clk: Clock<D>,
    rst: Reset<D>,
) -> Signal<D, bool>
where
    ConstConstr<{ rise_period_constr!(D, PS) }>:,
{
    rise_every::<D, { clk_divider::<D>(PS) }>(clk, rst)
}

macro_rules! rise_rate_constr {
    ($domain:ident, $rate:expr) => {
        rise_period_constr!($domain, hz_to_period($rate))
    };
}

#[inline]
pub fn rise_rate<D: ClockDomain, const RATE: usize>(
    clk: Clock<D>,
    rst: Reset<D>,
) -> Signal<D, bool>
where
    ConstConstr<{ rise_rate_constr!(D, RATE) }>:,
{
    rise_period::<D, { hz_to_period(RATE) }>(clk, rst)
}
