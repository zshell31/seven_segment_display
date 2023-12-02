use std::marker::PhantomData;

use derive_where::derive_where;
use ferrum_hdl::{
    cast::{Cast, CastFrom},
    signal::SignalValue,
};

pub trait Polarity {
    fn bit(bit: bool) -> bool;
}

pub struct High;

impl Polarity for High {
    #[inline]
    fn bit(bit: bool) -> bool {
        bit
    }
}

pub struct Low;

impl Polarity for Low {
    #[inline]
    fn bit(bit: bool) -> bool {
        !bit
    }
}

#[derive_where(Debug, Clone, Copy)]
#[derive(SignalValue)]
#[signal_value(bound = "P: Polarity + 'static")]
// #[bitpack(bound = "P: Polarity")]
pub struct Active<P: Polarity> {
    bit: bool,
    _polarity: PhantomData<P>,
}

impl<P: Polarity> Active<P> {
    #[inline]
    pub fn new(bit: bool) -> Self {
        Self {
            bit: P::bit(bit),
            _polarity: PhantomData,
        }
    }

    #[inline]
    pub fn bit(&self) -> bool {
        P::bit(self.bit)
    }
}

impl<P: Polarity> CastFrom<bool> for Active<P> {
    #[inline]
    fn cast_from(bit: bool) -> Self {
        Self::new(bit.cast())
    }
}

impl<P: Polarity> CastFrom<Active<P>> for bool {
    #[inline]
    fn cast_from(active: Active<P>) -> Self {
        active.bit().cast()
    }
}
