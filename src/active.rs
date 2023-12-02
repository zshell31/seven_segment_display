use std::marker::PhantomData;

use derive_where::derive_where;
use ferrum_hdl::{bit::Bit, bitpack::BitPack, signal::SignalValue};

pub trait Polarity {
    fn bit(bit: Bit) -> Bit;
}

pub struct High;

impl Polarity for High {
    #[inline]
    fn bit(bit: Bit) -> Bit {
        bit
    }
}

pub struct Low;

impl Polarity for Low {
    #[inline]
    fn bit(bit: Bit) -> Bit {
        !bit
    }
}

#[derive_where(Debug, Clone, Copy)]
#[derive(SignalValue, BitPack)]
#[signal_value(bound = "P: Polarity + 'static")]
#[bitpack(bound = "P: Polarity")]
pub struct Active<P: Polarity> {
    bit: Bit,
    _polarity: PhantomData<P>,
}

impl<P: Polarity> Active<P> {
    #[inline]
    pub fn new(bit: Bit) -> Self {
        Self {
            bit: P::bit(bit),
            _polarity: PhantomData,
        }
    }

    #[inline]
    pub fn bit(&self) -> Bit {
        P::bit(self.bit)
    }
}

impl<P: Polarity> From<Bit> for Active<P> {
    #[inline]
    fn from(bit: Bit) -> Self {
        Self::new(bit)
    }
}

impl<P: Polarity> From<Active<P>> for Bit {
    #[inline]
    fn from(active: Active<P>) -> Self {
        active.bit()
    }
}

impl<P: Polarity> From<bool> for Active<P> {
    #[inline]
    fn from(bit: bool) -> Self {
        Self::new(bit.into())
    }
}

impl<P: Polarity> From<Active<P>> for bool {
    #[inline]
    fn from(active: Active<P>) -> Self {
        active.bit().into()
    }
}
