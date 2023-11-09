use std::sync::OnceLock;

const MEMORY_SIZE: usize = 32768;

pub struct Memory<const N: usize> {
    memory: [u16; N],
}

pub trait ReadWrite<const N: usize> {
    fn get_memory(&self) -> [u16; N];

    fn write(&self, address: u16, value: u16) {
        let mut mem = self.get_memory();
        mem[address as usize] = value;
    }
    fn read(&self, address: u16) -> u16 {
        self.get_memory()[address as usize]
    }
}

impl ReadWrite<MEMORY_SIZE> for Memory<MEMORY_SIZE> {
    fn get_memory(&self) -> [u16; MEMORY_SIZE] {
        self.memory
    }
}
impl ReadWrite<8> for Memory<8> {
    fn get_memory(&self) -> [u16; 8] {
        self.memory
    }
}

/* == architecture ==
- three storage regions
  - memory with 15-bit address space storing 16-bit values
*/

pub fn _get_memory() -> &'static Memory<MEMORY_SIZE> {
    static MEMORY: OnceLock<Memory<MEMORY_SIZE>> = OnceLock::new();

    MEMORY.get_or_init(|| Memory {
        memory: [0; MEMORY_SIZE],
    })
}

pub fn _load_into_memory(iter: impl Iterator<Item = u16>) {
    let memory = _get_memory();
    for (i, byte) in iter.enumerate() {
        memory.write(i.try_into().expect("data should only be u16 long"), byte)
    }
}
/*
- eight registers
*/

pub fn _get_register() -> &'static Memory<8> {
    static REGISTER: OnceLock<Memory<8>> = OnceLock::new();

    REGISTER.get_or_init(|| Memory { memory: [0; 8] })
}
/*
- an unbounded stack which holds individual 16-bit values
*/

pub fn _get_stack() -> &'static Vec<u16> {
    static STACK: OnceLock<Vec<u16>> = OnceLock::new();

    STACK.get_or_init(Vec::new)
}
