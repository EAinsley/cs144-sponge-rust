use super::byte_stream_harness::*;

#[test]
fn write_end_pop() {
  let mut test = ByteStreamTestHarness::with_capacity(15);

  test.execute(ActionWrite::new(b"cat"));

  test.execute(ExpectInputEnded::new(false));
  test.execute(ExpectBufferEmpty::new(false));
  test.execute(ExpectEof::new(false));
  test.execute(ExpectBytesRead::new(0));
  test.execute(ExpectBytesWritten::new(3));
  test.execute(ExpectRemainingCapacity::new(12));
  test.execute(ExpectBufferSize::new(3));
  test.execute(ExpectPeek::new(b"cat"));

  test.execute(ActionEndInput);

  test.execute(ExpectInputEnded::new(true));
  test.execute(ExpectBufferEmpty::new(false));
  test.execute(ExpectEof::new(false));
  test.execute(ExpectBytesRead::new(0));
  test.execute(ExpectBytesWritten::new(3));
  test.execute(ExpectRemainingCapacity::new(12));
  test.execute(ExpectBufferSize::new(3));
  test.execute(ExpectPeek::new(b"cat"));

  test.execute(ActionPop::new(3));

  test.execute(ExpectInputEnded::new(true));
  test.execute(ExpectBufferEmpty::new(true));
  test.execute(ExpectEof::new(true));
  test.execute(ExpectBytesRead::new(3));
  test.execute(ExpectBytesWritten::new(3));
  test.execute(ExpectRemainingCapacity::new(15));
  test.execute(ExpectBufferSize::new(0));
}

#[test]
fn write_pop_end() {
  let mut test = ByteStreamTestHarness::with_capacity(15);

  test.execute(ActionWrite::new(b"cat"));

  test.execute(ExpectInputEnded::new(false));
  test.execute(ExpectBufferEmpty::new(false));
  test.execute(ExpectEof::new(false));
  test.execute(ExpectBytesRead::new(0));
  test.execute(ExpectBytesWritten::new(3));
  test.execute(ExpectRemainingCapacity::new(12));
  test.execute(ExpectBufferSize::new(3));
  test.execute(ExpectPeek::new(b"cat"));

  test.execute(ActionPop::new(3));

  test.execute(ExpectInputEnded::new(false));
  test.execute(ExpectBufferEmpty::new(true));
  test.execute(ExpectEof::new(false));
  test.execute(ExpectBytesRead::new(3));
  test.execute(ExpectBytesWritten::new(3));
  test.execute(ExpectRemainingCapacity::new(15));
  test.execute(ExpectBufferSize::new(0));

  test.execute(ActionEndInput);

  test.execute(ExpectInputEnded::new(true));
  test.execute(ExpectBufferEmpty::new(true));
  test.execute(ExpectEof::new(true));
  test.execute(ExpectBytesRead::new(3));
  test.execute(ExpectBytesWritten::new(3));
  test.execute(ExpectRemainingCapacity::new(15));
  test.execute(ExpectBufferSize::new(0));
}

#[test]
fn write_pop2_end() {
  let mut test = ByteStreamTestHarness::with_capacity(15);

  test.execute(ActionWrite::new(b"cat"));

  test.execute(ExpectInputEnded::new(false));
  test.execute(ExpectBufferEmpty::new(false));
  test.execute(ExpectEof::new(false));
  test.execute(ExpectBytesRead::new(0));
  test.execute(ExpectBytesWritten::new(3));
  test.execute(ExpectRemainingCapacity::new(12));
  test.execute(ExpectBufferSize::new(3));
  test.execute(ExpectPeek::new(b"cat"));

  test.execute(ActionPop::new(1));

  test.execute(ExpectInputEnded::new(false));
  test.execute(ExpectBufferEmpty::new(false));
  test.execute(ExpectEof::new(false));
  test.execute(ExpectBytesRead::new(1));
  test.execute(ExpectBytesWritten::new(3));
  test.execute(ExpectRemainingCapacity::new(13));
  test.execute(ExpectBufferSize::new(2));
  test.execute(ExpectPeek::new(b"at"));

  test.execute(ActionPop::new(2));

  test.execute(ExpectInputEnded::new(false));
  test.execute(ExpectBufferEmpty::new(true));
  test.execute(ExpectEof::new(false));
  test.execute(ExpectBytesRead::new(3));
  test.execute(ExpectBytesWritten::new(3));
  test.execute(ExpectRemainingCapacity::new(15));
  test.execute(ExpectBufferSize::new(0));

  test.execute(ActionEndInput);

  test.execute(ExpectInputEnded::new(true));
  test.execute(ExpectBufferEmpty::new(true));
  test.execute(ExpectEof::new(true));
  test.execute(ExpectBytesRead::new(3));
  test.execute(ExpectBytesWritten::new(3));
  test.execute(ExpectRemainingCapacity::new(15));
  test.execute(ExpectBufferSize::new(0));
}
