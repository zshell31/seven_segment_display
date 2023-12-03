use std::fmt::{Binary, Debug};

use ferrum_hdl::{
    array::Array,
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
pub struct Counter<const N: usize>(Idx<N>)
where
    ConstConstr<{ idx_constr(N) }>:;

impl<const N: usize> Debug for Counter<N>
where
    ConstConstr<{ idx_constr(N) }>:,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.0, f)
    }
}

impl<const N: usize> Binary for Counter<N>
where
    ConstConstr<{ idx_constr(N) }>:,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Binary::fmt(&self.0, f)
    }
}

impl<const N: usize> Counter<N>
where
    ConstConstr<{ idx_constr(N) }>:,
{
    fn new() -> Self {
        Self(Idx::new())
    }

    #[inline]
    fn succ(self) -> Self {
        Counter(self.0.succ())
    }

    #[inline]
    pub fn signal<D: ClockDomain>(
        clk: Clock<D>,
        rst: Reset<D>,
        en: Enable<D>,
    ) -> Signal<D, Self> {
        reg_en(clk, rst, en, Self::new(), |counter| counter.succ())
    }

    pub fn one_hot(self) -> Array<N, bool>
    where
        Assert<{ N <= 128 }>: IsTrue,
        Array<N, bool>: BitPack<Packed = BitVec<N>>,
    {
        let val = 1_u8.cast::<Unsigned<N>>()
            << ((N - 1).cast::<Unsigned<N>>() - self.0.val().cast::<Unsigned<N>>());
        val.repack()
    }
}
