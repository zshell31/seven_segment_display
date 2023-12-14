use std::fmt::{Binary, Debug};

use ferrum_hdl::{
    array::{Array, ArrayExt},
    bitpack::BitPack,
    bitvec::BitVec,
    cast::Cast,
    const_helpers::{Assert, ConstConstr, IsTrue},
    domain::{Clock, ClockDomain},
    index::{idx_constr, Idx},
    signal::{reg_en, Enable, Reset, Signal, SignalValue},
    unsigned::Unsigned,
};

#[derive(Clone, SignalValue)]
pub struct RoundRobin<const N: usize>(Idx<N>)
where
    ConstConstr<{ idx_constr(N) }>:;

impl<const N: usize> Debug for RoundRobin<N>
where
    ConstConstr<{ idx_constr(N) }>:,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.0, f)
    }
}

impl<const N: usize> Binary for RoundRobin<N>
where
    ConstConstr<{ idx_constr(N) }>:,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Binary::fmt(&self.0, f)
    }
}

impl<const N: usize> RoundRobin<N>
where
    ConstConstr<{ idx_constr(N) }>:,
{
    fn new() -> Self {
        Self(Idx::new())
    }

    fn next(self) -> Self {
        RoundRobin(self.0.succ())
    }

    pub fn signal<D: ClockDomain>(
        clk: Clock<D>,
        rst: &Reset<D>,
        next: &Enable<D>,
    ) -> Signal<D, Self> {
        reg_en(clk, rst, next, &Self::new(), |rr| rr.next())
    }

    pub fn selector(&self) -> Array<N, bool>
    where
        Assert<{ N <= 128 }>: IsTrue,
        Array<N, bool>: BitPack<Packed = BitVec<N>>,
    {
        type U<const N: usize> = Unsigned<N>;

        let offset = (N - 1).cast::<U<N>>() - self.index().val().cast::<U<N>>();
        let val = 1_u8.cast::<U<N>>() << offset;
        val.repack()
    }

    pub fn index(&self) -> Idx<N> {
        self.0.clone()
    }

    pub fn mux<T: SignalValue>(&self, inputs: &Array<N, T>) -> T
    where
        Assert<{ idx_constr(N) <= usize::BITS as usize }>: IsTrue,
    {
        inputs.idx(self.index())
    }
}
