use super::byte_stream_harness::*;
#[test]
fn write_write_end_pop_pop() {
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

  test.execute(ActionWrite::new(b"tac"));

  test.execute(ExpectInputEnded::new(false));
  test.execute(ExpectBufferEmpty::new(false));
  test.execute(ExpectEof::new(false));
  test.execute(ExpectBytesRead::new(0));
  test.execute(ExpectBytesWritten::new(6));
  test.execute(ExpectRemainingCapacity::new(9));
  test.execute(ExpectBufferSize::new(6));
  test.execute(ExpectPeek::new(b"cattac"));

  test.execute(ActionEndInput);

  test.execute(ExpectInputEnded::new(true));
  test.execute(ExpectBufferEmpty::new(false));
  test.execute(ExpectEof::new(false));
  test.execute(ExpectBytesRead::new(0));
  test.execute(ExpectBytesWritten::new(6));
  test.execute(ExpectRemainingCapacity::new(9));
  test.execute(ExpectBufferSize::new(6));
  test.execute(ExpectPeek::new(b"cattac"));

  test.execute(ActionPop::new(2));

  test.execute(ExpectInputEnded::new(true));
  test.execute(ExpectBufferEmpty::new(false));
  test.execute(ExpectEof::new(false));
  test.execute(ExpectBytesRead::new(2));
  test.execute(ExpectBytesWritten::new(6));
  test.execute(ExpectRemainingCapacity::new(11));
  test.execute(ExpectBufferSize::new(4));
  test.execute(ExpectPeek::new(b"ttac"));

  test.execute(ActionPop::new(4));

  test.execute(ExpectInputEnded::new(true));
  test.execute(ExpectBufferEmpty::new(true));
  test.execute(ExpectEof::new(true));
  test.execute(ExpectBytesRead::new(6));
  test.execute(ExpectBytesWritten::new(6));
  test.execute(ExpectRemainingCapacity::new(15));
  test.execute(ExpectBufferSize::new(0));
}
#[test]
fn write_pop_write_end_pop() {
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

  test.execute(ActionPop::new(2));

  test.execute(ExpectInputEnded::new(false));
  test.execute(ExpectBufferEmpty::new(false));
  test.execute(ExpectEof::new(false));
  test.execute(ExpectBytesRead::new(2));
  test.execute(ExpectBytesWritten::new(3));
  test.execute(ExpectRemainingCapacity::new(14));
  test.execute(ExpectBufferSize::new(1));
  test.execute(ExpectPeek::new(b"t"));

  test.execute(ActionWrite::new(b"tac"));

  test.execute(ExpectInputEnded::new(false));
  test.execute(ExpectBufferEmpty::new(false));
  test.execute(ExpectEof::new(false));
  test.execute(ExpectBytesRead::new(2));
  test.execute(ExpectBytesWritten::new(6));
  test.execute(ExpectRemainingCapacity::new(11));
  test.execute(ExpectBufferSize::new(4));
  test.execute(ExpectPeek::new(b"ttac"));

  test.execute(ActionEndInput);

  test.execute(ExpectInputEnded::new(true));
  test.execute(ExpectBufferEmpty::new(false));
  test.execute(ExpectEof::new(false));
  test.execute(ExpectBytesRead::new(2));
  test.execute(ExpectBytesWritten::new(6));
  test.execute(ExpectRemainingCapacity::new(11));
  test.execute(ExpectBufferSize::new(4));
  test.execute(ExpectPeek::new(b"ttac"));

  test.execute(ActionPop::new(4));

  test.execute(ExpectInputEnded::new(true));
  test.execute(ExpectBufferEmpty::new(true));
  test.execute(ExpectEof::new(true));
  test.execute(ExpectBytesRead::new(6));
  test.execute(ExpectBytesWritten::new(6));
  test.execute(ExpectRemainingCapacity::new(15));
  test.execute(ExpectBufferSize::new(0));
}
