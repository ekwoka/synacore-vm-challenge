use super::_retrieve_register;

/*
- all numbers are unsigned integers 0..32767 (15-bit)
- all math is modulo 32768; 32758 + 15 => 5
*/

pub fn _safe_add(a: u16, b: u16) -> u16 {
    (_retrieve_register(a) as u32 + _retrieve_register(b) as u32) as u16 % 32768
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
