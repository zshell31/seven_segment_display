use ferrum_hdl::{
    const_helpers::ConstConstr,
    domain::{hz_to_period, Clock, ClockDomain, MILLISECOND, SECOND},
    signal::{Enable, Reset},
};

use crate::{rise_period_constr, signal_ext::rise_period};

pub struct TestSystem;

impl ClockDomain for TestSystem {
    const FREQ: usize = 160;
}

pub struct ZynqMiniDom;

impl ClockDomain for ZynqMiniDom {
    const FREQ: usize = 50_000_000;
}

#[cfg(test)]
pub(crate) type System = TestSystem;

#[cfg(not(test))]
pub(crate) type System = ZynqMiniDom;

pub trait EnSignals<const N: usize>: ClockDomain + Sized {
    const RR_PERIOD: usize;
    const SR_PERIOD: usize;

    #[inline]
    fn sr_enable(clk: Clock<Self>, rst: &Reset<Self>) -> Enable<Self>
    where
        ConstConstr<{ rise_period_constr!(Self, Self::SR_PERIOD) }>:,
    {
        rise_period::<_, { Self::SR_PERIOD }>(clk, rst)
    }

    #[inline]
    fn rr_enable(clk: Clock<Self>, rst: &Reset<Self>) -> Enable<Self>
    where
        ConstConstr<{ rise_period_constr!(Self, Self::RR_PERIOD) }>:,
    {
        rise_period::<_, { Self::RR_PERIOD }>(clk, rst)
    }
}

impl<const N: usize> EnSignals<N> for ZynqMiniDom {
    const RR_PERIOD: usize = MILLISECOND;
    const SR_PERIOD: usize = SECOND;
}

const RR_PERIOD: usize = hz_to_period(80);

impl<const N: usize> EnSignals<N> for TestSystem {
    const RR_PERIOD: usize = RR_PERIOD;
    const SR_PERIOD: usize = N * RR_PERIOD;
}
