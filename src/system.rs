use ferrum_hdl::domain::ClockDomain;

pub struct TestSystem;

impl ClockDomain for TestSystem {
    const FREQ: usize = 40;
}

pub struct ZynqMiniDom;

impl ClockDomain for ZynqMiniDom {
    const FREQ: usize = 50_000_000;
}

pub(crate) type System = ZynqMiniDom;
