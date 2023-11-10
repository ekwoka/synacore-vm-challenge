use std::ops::Add;

use super::_retrieve_register;

/*
- all numbers are unsigned integers 0..32767 (15-bit)
- all math is modulo 32768; 32758 + 15 => 5
*/

pub struct SynacoreValue(pub u16);

impl From<u16> for SynacoreValue {
    fn from(value: u16) -> Self {
        Self(_retrieve_register(value))
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

pub fn _safe_add(a: SynacoreValue, b: SynacoreValue) -> u16 {
    (a + b).into()
}

pub fn _safe_mul(a: u16, b: u16) -> u16 {
    (_retrieve_register(a) as u32 * _retrieve_register(b) as u32) as u16 % 32768
}

pub fn _safe_mod(a: u16, b: u16) -> u16 {
    _retrieve_register(a) % _retrieve_register(b)
}

pub fn _safe_and(a: u16, b: u16) -> u16 {
    (_retrieve_register(a) & _retrieve_register(b)) % 32768
}

pub fn _safe_or(a: u16, b: u16) -> u16 {
    (_retrieve_register(a) | _retrieve_register(b)) % 32768
}

pub fn _safe_not(a: u16) -> u16 {
    !_retrieve_register(a) % 32768
}
