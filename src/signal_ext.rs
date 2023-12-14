use ferrum_hdl::{
    const_helpers::ConstConstr,
    domain::{clk_divider, hz_to_period, Clock, ClockDomain},
    index::{idx_constr, Idx},
    signal::{reg, Reset, Signal},
};

pub fn rise_every_constr(n: usize) -> usize {
    idx_constr(n)
}

#[macro_export]
macro_rules! rise_every_constr {
    ($n:expr) => {
        ferrum_hdl::index::idx_constr($n)
    };
}

#[inline]
pub fn rise_every<D: ClockDomain, const PS: usize>(
    clk: Clock<D>,
    rst: &Reset<D>,
) -> Signal<D, bool>
where
    ConstConstr<{ rise_every_constr!(PS) }>:,
{
    reg(clk, rst, &(Idx::<PS>::new(), false), |(idx, _)| {
        (idx.clone().succ(), idx.is_max())
    })
    .map(|(_, en)| en)
}

#[macro_export]
macro_rules! rise_period_constr {
    ($domain:ident, $period:expr) => {
        rise_every_constr!(ferrum_hdl::domain::clk_divider::<$domain>($period))
    };
}

pub(crate) use rise_period_constr;

#[inline]
pub fn rise_period<D: ClockDomain, const PS: usize>(
    clk: Clock<D>,
    rst: &Reset<D>,
) -> Signal<D, bool>
where
    ConstConstr<{ rise_period_constr!(D, PS) }>:,
{
    rise_every::<D, { clk_divider::<D>(PS) }>(clk, rst)
}

#[macro_export]
macro_rules! rise_rate_constr {
    ($domain:ident, $rate:expr) => {
        rise_period_constr!($domain, ferrum_hdl::domain::hz_to_period($rate))
    };
}

#[inline]
pub fn rise_rate<D: ClockDomain, const RATE: usize>(
    clk: Clock<D>,
    rst: &Reset<D>,
) -> Signal<D, bool>
where
    ConstConstr<{ rise_rate_constr!(D, RATE) }>:,
{
    rise_period::<D, { hz_to_period(RATE) }>(clk, rst)
}
