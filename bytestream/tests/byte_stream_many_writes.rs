use bytestream::ByteStream;
use rand::{distributions::Alphanumeric, prelude::*};

const NREPS: usize = 1000;
const MIN_WRITE: usize = 10;
const MAX_WRITE: usize = 200;
const CAPACITY: usize = NREPS * MAX_WRITE;

#[test]
fn many_writes() {
  let mut rd = thread_rng();
  let mut stream = ByteStream::new(CAPACITY);
  let mut acc = 0usize;
  for _ in 0..NREPS {
    let size = rd.gen_range(MIN_WRITE..=MAX_WRITE);
    let d: String = (&mut rd)
      .sample_iter(Alphanumeric)
      .take(size)
      .map(char::from)
      .collect();
    assert_eq!(stream.write(&d), size);
    acc += size;

    assert_eq!(stream.input_ended(), false);
    assert_eq!(stream.buffer_empty(), false);
    assert_eq!(stream.eof(), false);
    assert_eq!(stream.bytes_read(), 0);
    assert_eq!(stream.bytes_written(), acc);
    assert_eq!(stream.remaining_capacity(), CAPACITY - acc);
    assert_eq!(stream.buffer_size(), acc);
  }
}
