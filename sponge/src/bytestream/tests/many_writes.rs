use super::byte_stream_harness::*;
use rand::{distributions::Alphanumeric, prelude::*};

const NREPS: usize = 1000;
const MIN_WRITE: usize = 10;
const MAX_WRITE: usize = 200;
const CAPACITY: usize = NREPS * MAX_WRITE;

#[test]
fn many_writes() {
  let mut test = ByteStreamTestHarness::with_capacity(CAPACITY);
  let mut rd = thread_rng();
  let mut acc = 0usize;
  for _ in 0..NREPS {
    let size = rd.gen_range(MIN_WRITE..=MAX_WRITE);
    let d: Vec<u8> =
      (&mut rd).sample_iter(Alphanumeric).take(size).collect();
    test.execute(
      ActionWrite::new(d.as_slice()).with_bytes_written(size),
    );
    acc += size;

    test.execute(ExpectInputEnded::new(false));
    test.execute(ExpectBufferEmpty::new(false));
    test.execute(ExpectEof::new(false));
    test.execute(ExpectBytesRead::new(0));
    test.execute(ExpectBytesWritten::new(acc));
    test.execute(ExpectRemainingCapacity::new(CAPACITY - acc));
    test.execute(ExpectBufferSize::new(acc));
  }
}
