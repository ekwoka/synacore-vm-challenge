use std::sync::{OnceLock, RwLock};

const MEMORY_SIZE: usize = 32768;

pub struct Memory<const N: usize> {
    memory: RwLock<[u16; N]>,
}

impl<const N: usize> Memory<N> {
    const fn new() -> Memory<N> {
        Self {
            memory: RwLock::new([0; N]),
        }
    }
}

pub trait ReadWrite<const N: usize> {
    fn write(&self, address: u16, value: u16);
    fn read(&self, address: u16) -> u16;
}

impl ReadWrite<MEMORY_SIZE> for Memory<MEMORY_SIZE> {
    fn write(&self, address: u16, value: u16) {
        self.memory.write().unwrap()[address as usize] = value
    }
    fn read(&self, address: u16) -> u16 {
        self.memory.read().unwrap()[address as usize]
    }
}
impl ReadWrite<8> for Memory<8> {
    fn write(&self, address: u16, value: u16) {
        self.memory.write().unwrap()[address as usize] = value
    }
    fn read(&self, address: u16) -> u16 {
        self.memory.read().unwrap()[address as usize]
    }
}

/* == architecture ==
- three storage regions
  - memory with 15-bit address space storing 16-bit values
*/

pub fn get_memory() -> &'static Memory<MEMORY_SIZE> {
    static MEMORY: Memory<MEMORY_SIZE> = Memory::new();

    &MEMORY
}

pub fn load_into_memory(iter: impl Iterator<Item = u16>) {
    let memory = get_memory();
    for (i, byte) in iter.enumerate() {
        memory.write(i.try_into().expect("data should only be u16 long"), byte);
    }
}
/*
- eight registers
*/

pub fn get_register() -> &'static Memory<8> {
    static REGISTER: Memory<8> = Memory::new();

    &REGISTER
}
/*
- an unbounded stack which holds individual 16-bit values
*/

pub fn get_stack() -> &'static Stack {
    static STACK: OnceLock<Stack> = OnceLock::new();

    STACK.get_or_init(|| Stack {
        stack: RwLock::new(Vec::new()),
    })
}

pub struct Stack {
    stack: RwLock<Vec<u16>>,
}

impl Stack {
    pub fn push(&self, value: u16) {
        self.stack.write().unwrap().push(value)
    }
    pub fn pop(&self) -> Option<u16> {
        self.stack.write().unwrap().pop()
    }
}
