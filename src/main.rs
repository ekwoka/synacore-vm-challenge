mod environment;
mod operations;
mod reader;

use environment::load_into_memory;
use operations::OpCode;
use reader::read_binary;

fn main() {
    let byte_stream = read_binary();

    load_into_memory(byte_stream);

    let mut position: u16 = 0;

    loop {
        OpCode::from(position).execute(&mut position);
    }
}
