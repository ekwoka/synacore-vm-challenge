use crate::environment::{get_memory, get_register, ReadWrite};

use super::math::SynacoreValue;

pub enum Address {
    Reg { address: u16 },
    Mem { address: u16 },
}

impl Address {
    pub fn write(&self, value: u16) {
        match self {
            Self::Reg { address } => {
                let register = get_register();
                register.write(*address, value);
            }
            Self::Mem { address } => {
                let memory = get_memory();
                memory.write(*address, value);
            }
        }
    }
}

impl From<u16> for Address {
    fn from(address: u16) -> Self {
        if address < 32768 {
            Self::Mem { address }
        } else {
            Self::Reg {
                address: address % 8,
            }
        }
    }
}

pub struct Reg {
    pub address: u16,
}

impl From<u16> for Reg {
    fn from(address: u16) -> Self {
        Self {
            address: address % 8,
        }
    }
}

pub struct Mem {
    pub address: u16,
}

impl Mem {
    pub fn write(&self, value: u16) {
        let memory = get_memory();
        memory.write(self.address, value);
    }
    pub fn read(&self) -> u16 {
        let memory = get_memory();
        memory.read(self.address)
    }
}

impl From<u16> for Mem {
    fn from(address: u16) -> Self {
        Self {
            address: SynacoreValue::from(address).into(),
        }
    }
}
