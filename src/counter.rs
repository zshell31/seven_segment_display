use std::fmt::{Binary, Debug};

use ferrum_hdl::{
    array::Array,
    bitpack::BitPack,
    bitvec::BitVec,
    cast::{Cast, CastFrom},
    const_functions::clog2,
    const_helpers::{Assert, IsTrue},
    signal::SignalValue,
    unsigned::Unsigned,
};

pub const fn counter(n: usize) -> usize {
    clog2(n)
}

#[derive(Clone)]
pub struct Counter<const N: usize>(Unsigned<{ counter(N) }>)
where
    [(); counter(N)]:;

impl<const N: usize> Debug for Counter<N>
where
    [(); counter(N)]:,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.0, f)
    }
}

impl<const N: usize> Binary for Counter<N>
where
    [(); counter(N)]:,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Binary::fmt(&self.0, f)
    }
}

impl<const N: usize> Default for Counter<N>
where
    [(); counter(N)]:,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<const N: usize> SignalValue for Counter<N> where [(); counter(N)]: {}

impl<const N: usize> Counter<N>
where
    [(); counter(N)]:,
    Assert<{ N <= 8 }>: IsTrue,
{
    const MAX: u8 = 1 << ((N - 1) as u8);
}

impl<const N: usize> Counter<N>
where
    [(); counter(N)]:,
{
    pub fn new() -> Self {
        Self(0_u8.cast())
    }

    pub fn one_hot(self) -> Array<N, bool>
    where
        Assert<{ N <= 8 }>: IsTrue,
        Array<N, bool>: BitPack<Packed = BitVec<N>>,
    {
        (Self::MAX >> self.0.cast::<u8>())
            .cast::<Unsigned<N>>()
            .repack()
    }

    #[inline]
    pub fn is_max(&self) -> bool {
        let max = self.0 == Unsigned::cast_from(N - 1);
        max
    }

    #[inline]
    pub fn is_min(&self) -> bool {
        let min = self.0 == 0;
        min
    }

    pub fn succ(self) -> (Self, bool) {
        let (value, succ) = if self.is_max() {
            (0_u8.cast(), true)
        } else {
            (self.0 + 1_u8, false)
        };
        (Self(value), succ)
    }

    pub fn pred(self) -> (Self, bool) {
        let (value, pred) = if self.is_min() {
            ((N as u128).cast(), true)
        } else {
            (self.0 - 1_u8, false)
        };
        (Self(value), pred)
    }
}
