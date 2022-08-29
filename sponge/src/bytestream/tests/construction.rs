use super::byte_stream_harness::*;

#[test]
fn byte_stream_construction() {
  let mut test = ByteStreamTestHarness::with_capacity(15);
  test.execute(ExpectInputEnded::new(false));
  test.execute(ExpectBufferEmpty::new(true));
  test.execute(ExpectEof::new(false));
  test.execute(ExpectBytesRead::new(0));
  test.execute(ExpectBytesWritten::new(0));
  test.execute(ExpectRemainingCapacity::new(15));
  test.execute(ExpectBufferSize::new(0));
}
#[test]
fn byte_stream_construction_end() {
  let mut test = ByteStreamTestHarness::with_capacity(15);
  test.execute(ActionEndInput);
  test.execute(ExpectInputEnded::new(true));
  test.execute(ExpectBufferEmpty::new(true));
  test.execute(ExpectEof::new(true));
  test.execute(ExpectBytesRead::new(0));
  test.execute(ExpectBytesWritten::new(0));
  test.execute(ExpectRemainingCapacity::new(15));
  test.execute(ExpectBufferSize::new(0));
}
