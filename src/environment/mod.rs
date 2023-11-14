use std::sync::RwLock;

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
    pub fn write(&self, address: u16, value: u16) {
        self.memory.write().unwrap()[address as usize] = value
    }
    pub fn read(&self, address: u16) -> u16 {
        self.memory.read().unwrap()[address as usize]
    }
}

/* == architecture ==
- three storage regions
  - memory with 15-bit address space storing 16-bit values
*/

pub static MEMORY: Memory<MEMORY_SIZE> = Memory::new();

pub fn load_into_memory(iter: impl Iterator<Item = u16>) {
    for (i, byte) in iter.enumerate() {
        MEMORY.write(i.try_into().expect("data should only be u16 long"), byte);
    }
}
/*
- eight registers
*/

pub static REGISTER: Memory<8> = Memory::new();

/*
- an unbounded stack which holds individual 16-bit values
*/

pub static STACK: Stack = Stack {
    stack: RwLock::new(Vec::new()),
};

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
