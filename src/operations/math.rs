use std::ops::{Add, BitAnd, BitOr, Mul, Not, Rem};

use crate::environment::REGISTER;

use super::address::Address;

/*
- all numbers are unsigned integers 0..32767 (15-bit)
- all math is modulo 32768; 32758 + 15 => 5
*/
#[derive(Debug, Clone, Copy)]
pub struct SynacoreValue(pub u16);

impl From<u16> for SynacoreValue {
    fn from(value: u16) -> Self {
        if let Address::Reg { address } = value.into() {
            Self(REGISTER.read(address))
        } else {
            Self(value)
        }
    }
}

impl From<SynacoreValue> for u16 {
    fn from(value: SynacoreValue) -> Self {
        value.0
    }
}
impl From<SynacoreValue> for u8 {
    fn from(value: SynacoreValue) -> Self {
        value.0 as u8
    }
}
impl From<SynacoreValue> for char {
    fn from(value: SynacoreValue) -> Self {
        Self::from(value.0 as u8)
    }
}

impl Add<SynacoreValue> for SynacoreValue {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self((self.0 + other.0) % 32768)
    }
}

impl Mul<SynacoreValue> for SynacoreValue {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self((self.0 as u32 * other.0 as u32) as u16 % 32768)
    }
}

impl Rem<SynacoreValue> for SynacoreValue {
    type Output = Self;

    fn rem(self, other: Self) -> Self {
        Self(self.0 % other.0)
    }
}

impl PartialEq<SynacoreValue> for SynacoreValue {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl PartialOrd<SynacoreValue> for SynacoreValue {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl BitAnd<SynacoreValue> for SynacoreValue {
    type Output = Self;

    fn bitand(self, other: Self) -> Self {
        Self((self.0 & other.0) % 32768)
    }
}

impl BitOr<SynacoreValue> for SynacoreValue {
    type Output = Self;

    fn bitor(self, other: Self) -> Self {
        Self((self.0 | other.0) % 32768)
    }
}

impl Not for SynacoreValue {
    type Output = Self;

    fn not(self) -> Self {
        Self(!self.0 & 0b0111_1111_1111_1111)
    }
}
