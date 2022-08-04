use super::bytestream::ByteStream;

pub struct StreamReassembler {}

impl StreamReassembler {
  /// This function accepts a substring (aka a segment) of bytes,
  /// possibly out-of-order, from the logical stream, and assembles any newly
  /// contiguous substrings and writes them into the output stream in order.
  pub fn new(capacity: usize) -> StreamReassembler {
    todo!();
  }
  pub fn push_substring(
    &mut self,
    data: &str,
    index: usize,
    eof: bool,
  ) {
    todo!();
  }
  pub fn unassembled_bytes(&self) -> usize {
    todo!();
  }
  pub fn empty(&self) -> bool {
    todo!();
  }
}
