use std::process::exit;

use crate::environment::{ReadWrite, _get_register, get_memory};

use self::math::SynacoreValue;
pub mod math;

pub fn _retrieve_register(address: u16) -> u16 {
    if address < 32768 {
        address
    } else {
        _get_register().read(address % 8)
    }
}

/**
 * These structs allow us to easily retrieve arguments from memory in a dynamic manner
 * The caller can simply pass in the position in memory of the OpCode,
 * and the function will automatically retrieve the related arguments
 */
pub struct SingleArg<F = u16>(F)
where
    F: From<u16>;

impl<F> From<u16> for SingleArg<F>
where
    F: From<u16>,
{
    fn from(position: u16) -> Self {
        let mem = get_memory();
        Self(mem.read(position + 1).into())
    }
}

pub struct DoubleArg<F, S = F>(F, S)
where
    F: From<u16>,
    S: From<u16>;
impl<F, S> From<u16> for DoubleArg<F, S>
where
    F: From<u16>,
    S: From<u16>,
{
    fn from(position: u16) -> Self {
        let mem = get_memory();
        Self(mem.read(position + 1).into(), mem.read(position + 2).into())
    }
}
pub struct TripleArg<F = u16, S = F, T = S>(F, T, S)
where
    F: From<u16>,
    S: From<u16>,
    T: From<u16>;
impl<F, S, T> From<u16> for TripleArg<F, S, T>
where
    F: From<u16>,
    S: From<u16>,
    T: From<u16>,
{
    fn from(position: u16) -> Self {
        let mem = get_memory();
        Self(
            mem.read(position + 1).into(),
            mem.read(position + 2).into(),
            mem.read(position + 3).into(),
        )
    }
}

pub struct OpCode {}

#[allow(non_upper_case_globals)]
impl OpCode {
    pub const Halt: u16 = 0;
    pub const Out: u16 = 19;
    pub const Noop: u16 = 21;
}

/* == opcode listing ==
halt: 0
  stop execution and terminate the program */
pub fn halt() {
    exit(0);
}

/*
  set: 1 a b
  set register <a> to the value of <b>
*/

/* push: 2 a
 push <a> onto the stack
*/

/* pop: 3 a
 remove the top element from the stack and write it into <a>; empty stack = error
*/
/*
eq: 4 a b c
  set <a> to 1 if <b> is equal to <c>; set it to 0 otherwise
 */
/*
gt: 5 a b c
  set <a> to 1 if <b> is greater than <c>; set it to 0 otherwise
 */
/*
jmp: 6 a
  jump to <a>
 */
/*
jt: 7 a b
  if <a> is nonzero, jump to <b>
 */
/*
jf: 8 a b
  if <a> is zero, jump to <b>
 */
/*
add: 9 a b c
  assign into <a> the sum of <b> and <c> (modulo 32768)
 */
/*
mult: 10 a b c
  store into <a> the product of <b> and <c> (modulo 32768)
 */
/*
mod: 11 a b c
  store into <a> the remainder of <b> divided by <c>
 */
/*
and: 12 a b c
  stores into <a> the bitwise and of <b> and <c>
 */
/*
or: 13 a b c
  stores into <a> the bitwise or of <b> and <c>
 */
/*
not: 14 a b
  stores 15-bit bitwise inverse of <b> in <a>
 */
/*
rmem: 15 a b
  read memory at address <b> and write it to <a>
 */
/*
wmem: 16 a b
  write the value from <b> into memory at address <a>
 */
/*
call: 17 a
  write the address of the next instruction to the stack and jump to <a>
 */
/*
ret: 18
  remove the top element from the stack and jump to it; empty stack = halt
 */
/*
out: 19 a
  write the character represented by ascii code <a> to the terminal
 */
pub fn out(args: SingleArg<SynacoreValue>) {
    print!("{}", char::from(args.0));
}
/*
in: 20 a
  read a character from the terminal and write its ascii code to <a>; it can be assumed that once input starts, it will continue until a newline is encountered; this means that you can safely read whole lines from the keyboard and trust that they will be fully read
 */
/*
noop: 21
  no operation
  NOTE: just don't implement it
 */
