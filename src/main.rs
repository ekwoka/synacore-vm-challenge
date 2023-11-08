use std::fs::File;
use std::io::Read;
use std::process::exit;

trait ArrayChunks<T> {
  fn into_chunks(self, size: usize) -> ArrayChunksIter<T>;
}

impl<T> ArrayChunks<T> for T
where
  T: Iterator,
{
  fn into_chunks(self, size: usize) -> ArrayChunksIter<T> {
    ArrayChunksIter { iter: self, size }
  }
}

#[derive(Debug, Clone)]
struct ArrayChunksIter<T> {
  iter: T,
  size: usize,
}

impl<T> Iterator for ArrayChunksIter<T>
where
  T: Iterator,
{
  type Item = Vec<T::Item>;

  fn next(&mut self) -> Option<Self::Item> {
    let vec = self.iter.by_ref().take(self.size).collect::<Vec<T::Item>>();

    Some(vec)
  }
}

trait LittleEndian {
  fn to_little_endian(self) -> u16;
}

impl LittleEndian for Vec<u8> {
  fn to_little_endian(self: Vec<u8>) -> u16 {

    let result = self.iter().enumerate().fold(0, |mut result, (i, byte)| {
      result |= (*byte as u16) << (i * 8);
      result
    });

    result
  }
}

fn read_binary()-> Box<dyn Iterator<Item = u16>> {
  let challenge_bin = File::open("challenge.bin").unwrap();

  Box::new(challenge_bin.bytes().map(|b| b.unwrap_or(0)).into_chunks(2).map(|b| b.to_little_endian()))
}

fn main() {
  let mut byte_stream = read_binary();

  loop {
  if let Some(byte) = byte_stream.next() {
    match byte {
      0 => exit(0),
      19 => print!("{}", char::from(byte_stream.next().unwrap_or(0) as u8)),
      21 => {},
      _ => {}
    }
  }}
}
