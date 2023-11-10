mod environment;
mod operations;
mod reader;

use environment::{get_memory, load_into_memory, ReadWrite};
use reader::read_binary;
use std::process::exit;

fn main() {
    let byte_stream = read_binary();

    load_into_memory(byte_stream);

    let mut position = 0;

    loop {
        match get_memory().read(position) {
            0 => exit(0),
            19 => {
                print!("{}", char::from(get_memory().read(position + 1) as u8));
                position += 2;
            }
            21 => position += 1,
            _ => position += 1,
        }
    }
}
