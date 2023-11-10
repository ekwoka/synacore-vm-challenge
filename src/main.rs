mod environment;
mod operations;
mod reader;

use environment::{get_memory, load_into_memory, ReadWrite};
use operations::{halt, out, OpCode};
use reader::read_binary;

fn main() {
    let byte_stream = read_binary();

    load_into_memory(byte_stream);

    let mut position = 0;

    loop {
        position = match get_memory().read(position) {
            OpCode::Halt => {
                halt();
                position + 1
            }
            OpCode::Out => {
                out(position.into());
                position + 2
            }
            OpCode::Noop => position + 1,
            unk => {
                println!("unknown opcode: {}", unk);
                position + 1
            }
        }
    }
}
