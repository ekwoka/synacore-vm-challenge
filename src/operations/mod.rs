pub mod address;
pub mod math;

use std::{process::exit, sync::RwLock};

use crate::environment::{MEMORY, REGISTER, STACK};

use self::{
    address::{Address, Mem, Reg},
    math::SynacoreValue,
};

/**
 * These structs allow us to easily retrieve arguments from memory in a dynamic manner
 * The caller can simply pass in the position in memory of the OpCode,
 * and the function will automatically retrieve the related arguments
 */
pub struct SingleArg<F = u16>(F)
where
    F: From<u16>;

impl<F> From<&mut u16> for SingleArg<F>
where
    F: From<u16>,
{
    fn from(position: &mut u16) -> Self {
        Self(MEMORY.read(*position + 1).into())
    }
}

pub struct DoubleArg<F, S = F>(F, S)
where
    F: From<u16>,
    S: From<u16>;
impl<F, S> From<&mut u16> for DoubleArg<F, S>
where
    F: From<u16>,
    S: From<u16>,
{
    fn from(position: &mut u16) -> Self {
        let mem = &MEMORY;
        Self(
            mem.read(*position + 1).into(),
            mem.read(*position + 2).into(),
        )
    }
}
pub struct TripleArg<F = u16, S = F, T = S>(F, T, S)
where
    F: From<u16>,
    S: From<u16>,
    T: From<u16>;
impl<F, S, T> From<&mut u16> for TripleArg<F, S, T>
where
    F: From<u16>,
    S: From<u16>,
    T: From<u16>,
{
    fn from(position: &mut u16) -> Self {
        let mem = &MEMORY;
        Self(
            mem.read(*position + 1).into(),
            mem.read(*position + 2).into(),
            mem.read(*position + 3).into(),
        )
    }
}
#[derive(Debug)]
pub enum OpCode {
    Halt,
    Set,
    Push,
    Pop,
    Equals,
    GreaterThan,
    JumpTo,
    JumpIfTruthy,
    JumpIfFalsy,
    Add,
    Multiply,
    Modulo,
    And,
    Or,
    Not,
    ReadMemoryTo,
    WriteMemoryFrom,
    Call,
    Return,
    Out,
    In,
    Noop,
    Unknown,
}

impl From<u16> for OpCode {
    fn from(position: u16) -> Self {
        match MEMORY.read(position) {
            0 => OpCode::Halt,
            1 => OpCode::Set,
            2 => OpCode::Push,
            3 => OpCode::Pop,
            4 => OpCode::Equals,
            5 => OpCode::GreaterThan,
            6 => OpCode::JumpTo,
            7 => OpCode::JumpIfTruthy,
            8 => OpCode::JumpIfFalsy,
            9 => OpCode::Add,
            10 => OpCode::Multiply,
            11 => OpCode::Modulo,
            12 => OpCode::And,
            13 => OpCode::Or,
            14 => OpCode::Not,
            15 => OpCode::ReadMemoryTo,
            16 => OpCode::WriteMemoryFrom,
            17 => OpCode::Call,
            18 => OpCode::Return,
            19 => OpCode::Out,
            20 => OpCode::In,
            21 => OpCode::Noop,
            _ => OpCode::Unknown,
        }
    }
}

impl OpCode {
    pub fn execute(self, position: &mut u16) {
        match self {
            OpCode::Halt => halt(),
            OpCode::Set => set(position.into(), position),
            OpCode::Push => push(position.into(), position),
            OpCode::Pop => pop(position.into(), position),
            OpCode::Equals => eq(position.into(), position),
            OpCode::GreaterThan => gt(position.into(), position),
            OpCode::JumpTo => jmp(position.into(), position),
            OpCode::JumpIfTruthy => jt(position.into(), position),
            OpCode::JumpIfFalsy => jf(position.into(), position),
            OpCode::Add => add(position.into(), position),
            OpCode::Multiply => mult(position.into(), position),
            OpCode::Modulo => modulo(position.into(), position),
            OpCode::And => and(position.into(), position),
            OpCode::Or => or(position.into(), position),
            OpCode::Not => not(position.into(), position),
            OpCode::ReadMemoryTo => rmem(position.into(), position),
            OpCode::WriteMemoryFrom => wmem(position.into(), position),
            OpCode::Call => call(position.into(), position),
            OpCode::Return => ret(position),
            OpCode::Out => out(position.into(), position),
            OpCode::In => inp(position.into(), position),
            OpCode::Noop => noop(position),
            _ => {
                println!("unknown opcode: {:?}: {}", self, MEMORY.read(*position));
                *position += 1;
            }
        }
    }
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
fn set(DoubleArg(destination, value): DoubleArg<Reg, SynacoreValue>, position: &mut u16) {
    REGISTER.write(destination.address, value.into());
    *position += 3;
}

/* push: 2 a
 push <a> onto the stack
*/
fn push(SingleArg(value): SingleArg<SynacoreValue>, position: &mut u16) {
    STACK.push(value.into());
    *position += 2;
}

/* pop: 3 a
 remove the top element from the stack and write it into <a>; empty stack = error
*/
fn pop(SingleArg(destination): SingleArg<Address>, position: &mut u16) {
    STACK
        .pop()
        .map(|value| {
            destination.write(value);
        })
        .expect("Stack should not be empty");
    *position += 2;
}
/*
eq: 4 a b c
  set <a> to 1 if <b> is equal to <c>; set it to 0 otherwise
 */
fn eq(TripleArg(destination, lhs, rhs): TripleArg<Address, SynacoreValue>, position: &mut u16) {
    destination.write((lhs == rhs).into());
    *position += 4;
}
/*
gt: 5 a b c
  set <a> to 1 if <b> is greater than <c>; set it to 0 otherwise
 */
fn gt(TripleArg(destination, lhs, rhs): TripleArg<Address, SynacoreValue>, position: &mut u16) {
    destination.write((lhs > rhs).into());
    *position += 4;
}
/*
jmp: 6 a
  jump to <a>
 */
fn jmp(SingleArg(destination): SingleArg, position: &mut u16) {
    *position = destination;
}
/*
jt: 7 a b
  if <a> is nonzero, jump to <b>
 */
pub fn jt(DoubleArg(check, target): DoubleArg<SynacoreValue>, position: &mut u16) {
    if u16::from(check) != 0 {
        *position = target.into();
    } else {
        *position += 3;
    }
}
/*
jf: 8 a b
  if <a> is zero, jump to <b>
 */
fn jf(DoubleArg(check, target): DoubleArg<SynacoreValue>, position: &mut u16) {
    if u16::from(check) == 0 {
        *position = target.into();
    } else {
        *position += 3;
    }
}
/*
add: 9 a b c
  assign into <a> the sum of <b> and <c> (modulo 32768)
 */
fn add(TripleArg(destination, lhs, rhs): TripleArg<Address, SynacoreValue>, position: &mut u16) {
    destination.write((lhs + rhs).into());
    *position += 4;
}
/*
mult: 10 a b c
  store into <a> the product of <b> and <c> (modulo 32768)
 */
fn mult(TripleArg(destination, lhs, rhs): TripleArg<Address, SynacoreValue>, position: &mut u16) {
    destination.write((lhs * rhs).into());
    *position += 4;
}
/*
mod: 11 a b c
  store into <a> the remainder of <b> divided by <c>
 */
fn modulo(TripleArg(destination, lhs, rhs): TripleArg<Address, SynacoreValue>, position: &mut u16) {
    destination.write((lhs % rhs).into());
    *position += 4;
}
/*
and: 12 a b c
  stores into <a> the bitwise and of <b> and <c>
 */
fn and(TripleArg(destination, lhs, rhs): TripleArg<Address, SynacoreValue>, position: &mut u16) {
    destination.write((lhs & rhs).into());
    *position += 4;
}
/*
or: 13 a b c
  stores into <a> the bitwise or of <b> and <c>
 */
fn or(TripleArg(destination, lhs, rhs): TripleArg<Address, SynacoreValue>, position: &mut u16) {
    destination.write((lhs | rhs).into());
    *position += 4;
}
/*
not: 14 a b
  stores 15-bit bitwise inverse of <b> in <a>
 */
fn not(DoubleArg(destination, value): DoubleArg<Address, SynacoreValue>, position: &mut u16) {
    destination.write((!value).into());
    *position += 3;
}
/*
rmem: 15 a b
  read memory at address <b> and write it to <a>
 */
fn rmem(DoubleArg(destination, target): DoubleArg<Address, Mem>, position: &mut u16) {
    destination.write(target.read());
    *position += 3;
}
/*
wmem: 16 a b
  write the value from <b> into memory at address <a>
 */
fn wmem(DoubleArg(destination, value): DoubleArg<Mem, SynacoreValue>, position: &mut u16) {
    destination.write(value.into());
    *position += 3;
}
/*
call: 17 a
  write the address of the next instruction to the stack and jump to <a>
 */

fn call(SingleArg(destination): SingleArg<SynacoreValue>, position: &mut u16) {
    STACK.push(*position + 2);
    *position = destination.into();
}
/*
ret: 18
  remove the top element from the stack and jump to it; empty stack = halt
 */
fn ret(position: &mut u16) {
    STACK
        .pop()
        .map(|value| *position = value)
        .expect("Stack should not be empty");
}
/*
out: 19 a
  write the character represented by ascii code <a> to the terminal
 */
pub fn out(SingleArg(char_code): SingleArg<SynacoreValue>, position: &mut u16) {
    print!("{}", char::from(char_code));
    *position += 2;
}
/*
in: 20 a
  read a character from the terminal and write its ascii code to <a>; it can be assumed that once input starts, it will continue until a newline is encountered; this means that you can safely read whole lines from the keyboard and trust that they will be fully read
 */
pub fn inp(SingleArg(destination): SingleArg<Address>, position: &mut u16) {
    static INPUT_BUFFER: RwLock<Vec<u16>> = RwLock::new(Vec::new());
    if INPUT_BUFFER.read().unwrap().is_empty() {
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("stdout should be hooked up");

        INPUT_BUFFER.write().unwrap().extend(
            input
                .trim()
                .chars()
                .chain(std::iter::once('\n'))
                .map(|c| c as u16),
        )
    }
    let next_char = INPUT_BUFFER.write().unwrap().remove(0);
    destination.write(next_char);
    *position += 2;
}
/*
noop: 21
  no operation
 */
fn noop(position: &mut u16) {
    *position += 1;
}
