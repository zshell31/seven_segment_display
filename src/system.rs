use ferrum_hdl::domain::{hz_to_period, ClockDomain, MILLISECOND};

pub struct TestSystem;

impl ClockDomain for TestSystem {
    const FREQ: usize = 80;
}

pub struct ZynqMiniDom;

impl ClockDomain for ZynqMiniDom {
    const FREQ: usize = 50_000_000;
}

pub(crate) type System = TestSystem;

pub trait Params {
    const RATE: usize;
    const PERIOD: usize;
}

impl Params for ZynqMiniDom {
    const RATE: usize = 512;
    const PERIOD: usize = MILLISECOND;
}

impl Params for TestSystem {
    const RATE: usize = 20;
    const PERIOD: usize = hz_to_period(Self::RATE);
}
