use std::fs::File;
use std::io::Read;

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
    I: Default + Clone + Copy,
{
    type Item = [T::Item; N];

    fn next(&mut self) -> Option<Self::Item> {
        let mut arr = [Default::default(); N];
        for i in arr.iter_mut() {
            *i = self.iter.next()?;
        }
        Some(arr)
    }
}

pub fn read_binary() -> Box<dyn Iterator<Item = u16>> {
    let challenge_bin = File::open("challenge.bin").unwrap();

    Box::new(
        challenge_bin
            .bytes()
            .map(|b| b.unwrap_or(0))
            .into_chunks::<2>()
            .map(u16::from_le_bytes),
    )
}

#[test]
fn reads_the_challenge_binary() {
    let initial_bytes = read_binary().take(10).collect::<Vec<u16>>();
    assert_eq!(
        initial_bytes,
        vec![21, 21, 19, 87, 19, 101, 19, 108, 19, 99]
    );
}

#[test]
fn chunks_iterator_to_arrays() {
    let mut chunks = vec![1, 2, 3, 4, 5, 6, 7, 8, 9]
        .into_iter()
        .into_chunks::<3>();
    assert_eq!(chunks.next(), Some([1, 2, 3]));
    assert_eq!(chunks.next(), Some([4, 5, 6]));
    assert_eq!(chunks.next(), Some([7, 8, 9]));
}
