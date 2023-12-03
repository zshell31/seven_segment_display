use ferrum_hdl::domain::ClockDomain;

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
}

impl Params for ZynqMiniDom {
    const RATE: usize = 512;
}

impl Params for TestSystem {
    const RATE: usize = 20;
}
