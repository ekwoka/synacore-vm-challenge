use std::fs::File;
use std::io::Read;
use std::process::exit;

trait ArrayChunks<T> {
    fn into_chunks<const N: usize>(self) -> ArrayChunksIter<T, N>;
}

impl<T> ArrayChunks<T> for T
where
    T: Iterator,
{
    fn into_chunks<const N: usize>(self) -> ArrayChunksIter<T, N> {
        ArrayChunksIter { iter: self }
    }
}

#[derive(Debug, Clone)]
struct ArrayChunksIter<I, const N: usize> {
    iter: I,
}

impl<T, const N: usize, I> Iterator for ArrayChunksIter<T, N>
where
    T: Iterator<Item = I>,
    I: Default + Clone + Copy
{
    type Item = [T::Item;N];

    fn next(&mut self) -> Option<Self::Item> {
        let mut arr = [Default::default(); N];
        arr.iter_mut().for_each(|x| *x = self.iter.next().unwrap());
        Some(arr)
    }
}

fn read_binary() -> Box<dyn Iterator<Item = u16>> {
    let challenge_bin = File::open("challenge.bin").unwrap();

    Box::new(
        challenge_bin
            .bytes()
            .map(|b| b.unwrap_or(0))
            .into_chunks::<2>()
            .map(u16::from_le_bytes),
    )
}

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


#[test]
fn reads_the_challenge_binary() {
  let initial_bytes = read_binary().take(10).collect::<Vec<u16>>();
  assert_eq!(initial_bytes, vec![21, 21, 19, 87, 19, 101, 19, 108, 19, 99]);
}
