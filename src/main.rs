mod environment;
mod operations;
mod reader;

use environment::{get_memory, load_into_memory, ReadWrite};
use operations::{halt, out};
use reader::read_binary;

fn main() {
    let byte_stream = read_binary();

    load_into_memory(byte_stream);

    let mut position = 0;

    loop {
        position = match get_memory().read(position) {
            0 => {
                halt();
                position + 1
            }
            19 => {
                out(get_memory().read(position + 1));
                position + 2
            }
            21 => position + 1,
            unk => {
                println!("unknown opcode: {}", unk);
                position + 1
            }
        }
    }
}
