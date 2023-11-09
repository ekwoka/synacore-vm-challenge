mod environment;
mod operations;
mod reader;

use reader::read_binary;
use std::process::exit;

fn main() {
    let mut byte_stream = read_binary();

    loop {
        if let Some(byte) = byte_stream.next() {
            match byte {
                0 => exit(0),
                19 => print!("{}", char::from(byte_stream.next().unwrap_or(0) as u8)),
                21 => {}
                _ => {}
            }
        }
    }
}
