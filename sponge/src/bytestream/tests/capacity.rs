use super::byte_stream_harness::*;
#[test]
fn overwrite() {
  let mut test = ByteStreamTestHarness::with_capacity(2);

  test.execute(ActionWrite::new(b"cat").with_bytes_written(2));

  test.execute(ExpectInputEnded::new(false));
  test.execute(ExpectBufferEmpty::new(false));
  test.execute(ExpectEof::new(false));
  test.execute(ExpectBytesRead::new(0));
  test.execute(ExpectBytesWritten::new(2));
  test.execute(ExpectRemainingCapacity::new(0));
  test.execute(ExpectBufferSize::new(2));
  test.execute(ExpectPeek::new(b"ca"));

  test.execute(ActionWrite::new(b"t").with_bytes_written(0));

  test.execute(ExpectInputEnded::new(false));
  test.execute(ExpectBufferEmpty::new(false));
  test.execute(ExpectEof::new(false));
  test.execute(ExpectBytesRead::new(0));
  test.execute(ExpectBytesWritten::new(2));
  test.execute(ExpectRemainingCapacity::new(0));
  test.execute(ExpectBufferSize::new(2));
  test.execute(ExpectPeek::new(b"ca"));
}

#[test]
fn overwrite_clear_overwrite() {
  let mut test = ByteStreamTestHarness::with_capacity(2);

  test.execute(ActionWrite::new(b"cat").with_bytes_written(2));
  test.execute(ActionPop::new(2));
  test.execute(ActionWrite::new(b"tac").with_bytes_written(2));

  test.execute(ExpectInputEnded::new(false));
  test.execute(ExpectBufferEmpty::new(false));
  test.execute(ExpectEof::new(false));
  test.execute(ExpectBytesRead::new(2));
  test.execute(ExpectBytesWritten::new(4));
  test.execute(ExpectRemainingCapacity::new(0));
  test.execute(ExpectBufferSize::new(2));
  test.execute(ExpectPeek::new(b"ta"));
}

#[test]
fn overwrite_pop_overwrite() {
  let mut test = ByteStreamTestHarness::with_capacity(2);

  test.execute(ActionWrite::new(b"cat").with_bytes_written(2));
  test.execute(ActionPop::new(1));
  test.execute(ActionWrite::new(b"tac").with_bytes_written(1));

  test.execute(ExpectInputEnded::new(false));
  test.execute(ExpectBufferEmpty::new(false));
  test.execute(ExpectEof::new(false));
  test.execute(ExpectBytesRead::new(1));
  test.execute(ExpectBytesWritten::new(3));
  test.execute(ExpectRemainingCapacity::new(0));
  test.execute(ExpectBufferSize::new(2));
  test.execute(ExpectPeek::new(b"at"));
}

#[test]
fn long_stream() {
  let mut test = ByteStreamTestHarness::with_capacity(3);

  test.execute(ActionWrite::new(b"abcdef").with_bytes_written(3));
  test.execute(ExpectPeek::new(b"abc"));
  test.execute(ActionPop::new(1));

  for _ in 0..99997 {
    test.execute(ExpectRemainingCapacity::new(1));
    test.execute(ExpectBufferSize::new(2));
    test.execute(ActionWrite::new(b"abc").with_bytes_written(1));
    test.execute(ExpectRemainingCapacity::new(0));
    test.execute(ExpectPeek::new(b"bca"));
    test.execute(ActionPop::new(1));

    test.execute(ExpectRemainingCapacity::new(1));
    test.execute(ExpectBufferSize::new(2));
    test.execute(ActionWrite::new(b"bca").with_bytes_written(1));
    test.execute(ExpectRemainingCapacity::new(0));
    test.execute(ExpectPeek::new(b"cab"));
    test.execute(ActionPop::new(1));

    test.execute(ExpectRemainingCapacity::new(1));
    test.execute(ExpectBufferSize::new(2));
    test.execute(ActionWrite::new(b"cab").with_bytes_written(1));
    test.execute(ExpectRemainingCapacity::new(0));
    test.execute(ExpectPeek::new(b"abc"));
    test.execute(ActionPop::new(1));
  }

  test.execute(ActionEndInput);
  test.execute(ExpectPeek::new(b"bc"));
  test.execute(ActionPop::new(2));
  test.execute(ExpectEof::new(true));
}
