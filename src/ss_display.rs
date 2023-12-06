use std::fmt::{self, Display};

use ferrum_hdl::{
    array::ArrayExt,
    bitpack::BitPack,
    cast::{Cast, CastFrom},
    index::Idx,
    signal::SignalValue,
    unsigned::{u, Unsigned},
};

#[derive(Debug, Clone, SignalValue, BitPack)]
pub struct SSDisplay {
    pub a: bool,
    pub b: bool,
    pub c: bool,
    pub d: bool,
    pub e: bool,
    pub f: bool,
    pub g: bool,
}

fn horiz(b: bool) -> &'static str {
    match b {
        true => " ##### ",
        false => " ..... ",
    }
}

fn vert(b1: bool, b2: bool) -> &'static str {
    match (b1, b2) {
        (false, false) => ".     .",
        (true, false) => "#     .",
        (false, true) => ".     #",
        (true, true) => "#     #",
    }
}

impl Display for SSDisplay {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", horiz(self.a))?;
        writeln!(f, "{}", vert(self.f, self.b))?;
        writeln!(f, "{}", vert(self.f, self.b))?;
        writeln!(f, "{}", vert(self.f, self.b))?;
        writeln!(f, "{}", horiz(self.g))?;
        writeln!(f, "{}", vert(self.e, self.c))?;
        writeln!(f, "{}", vert(self.e, self.c))?;
        writeln!(f, "{}", vert(self.e, self.c))?;
        writeln!(f, "{}", horiz(self.d))?;

        Ok(())
    }
}

impl SSDisplay {
    pub fn five() -> SSDisplay {
        Self::encode(5_u8.cast())
    }

    pub fn encode(n: Unsigned<4>) -> Self {
        // const CODES: [u8; 16] = [
        //     0b1111110_u8,
        //     0b0110000,
        //     0b1101101,
        //     0b1111001,
        //     0b0110011,
        //     0b1011011,
        //     0b1011111,
        //     0b1110000,
        //     0b1111111,
        //     0b1111011,
        //     0b1110111,
        //     0b0011111,
        //     0b1001110,
        //     0b0111101,
        //     0b1001111,
        //     0b1000111,
        // ];
        // let idx: Idx<16> = Idx::<_>::from(n);
        // CODES.idx(idx).cast::<Unsigned<7>>().repack()
        (match u::<4>::cast_from(n) {
            u(0x0) => 0b1111110_u8,
            u(0x1) => 0b0110000,
            u(0x2) => 0b1101101,
            u(0x3) => 0b1111001,
            u(0x4) => 0b0110011,
            u(0x5) => 0b1011011,
            u(0x6) => 0b1011111,
            u(0x7) => 0b1110000,
            u(0x8) => 0b1111111,
            u(0x9) => 0b1111011,
            u(0xa) => 0b1110111,
            u(0xb) => 0b0011111,
            u(0xc) => 0b1001110,
            u(0xd) => 0b0111101,
            u(0xe) => 0b1001111,
            u(0xf) => 0b1000111,
            _ => 0b0000000,
        })
        .cast::<Unsigned<7>>()
        .repack()
    }
}
