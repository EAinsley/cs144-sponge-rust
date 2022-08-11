//! A class that assembles a series of excerpts from a byte stream
//! (possibly out of order, possibly overlapping) into an in-order byte stream.

use super::ByteStream;
use std::vec;

pub struct StreamReassembler {
  buffer: Vec<Option<char>>,
  buffer_header: usize,
  // unassembled_bytes: usize,
  eof_byte: Option<usize>,
  output: ByteStream,
  capacity: usize,
}

impl StreamReassembler {
  /// Construct a `StreamReassembler` that will store up to
  /// `capacity` bytes. This capacity limits both the bytes that
  /// have been reassembled, and those that have not yet been
  /// reassembled.
  pub fn new(capacity: usize) -> StreamReassembler {
    StreamReassembler {
      buffer: vec![None; capacity],
      buffer_header: 0,
      // unassembled_bytes: 0,
      eof_byte: None,
      output: ByteStream::new(capacity),
      capacity,
    }
  }

  /// Receive a substring and write any newly contiguous bytes
  /// into the stream.
  ///
  /// The StreamReassembler will stay within the memory limits of the
  /// `capacity`. Bytes that would exceed the capacity are silently
  /// discarded.
  pub fn push_substring(
    &mut self,
    data: &str,
    index: usize,
    eof: bool,
  ) {
    if self.output.input_ended() {
      return;
    }
    let data_end = data.len() + index;

    // check for eof
    if eof {
      self.eof_byte = Some(data_end);
    }

    // calculate the range
    let max_index = self.output.bytes_read() + self.capacity;
    let index_start = index.max(self.output.bytes_written());
    let index_end = self.eof_byte.map_or_else(
      || max_index.min(data_end),
      |i| i.min(max_index).min(data_end),
    );

    // buffer the data
    let data = data.as_bytes();
    for write_index in index_start..index_end {
      let buffer_index = (self.buffer_header + write_index
        - self.output.bytes_written())
        % self.capacity;
      let ch = data[write_index - index] as char;
      assert_eq!(self.buffer[buffer_index].get_or_insert(ch), &ch);
    }

    // write buffer
    while let Some(ch) = self.buffer[self.buffer_header].take() {
      self.output.write_char(ch);
      self.buffer_header = (self.buffer_header + 1) % self.capacity;
    }

    // check eof
    if self
      .eof_byte
      .map_or(false, |b| self.output.bytes_written() >= b)
    {
      self.output.end_input();
    }
  }

  pub fn as_stream(&self) -> &ByteStream {
    &self.output
  }
  pub fn as_mut_stream(&mut self) -> &mut ByteStream {
    &mut self.output
  }

  /// The number of bytes in the substrings stored but not yet
  /// reassembled
  ///
  /// If the byte at a particular index has been pushed more than
  /// once, it should only be counted once for the purpose of this
  /// function.
  pub fn unassembled_bytes(&self) -> usize {
    (&self.buffer).into_iter().filter(|x| x.is_some()).count()
  }

  /// Is the internal state empty (other than the output stream)?
  /// `true` if no substrings are waiting to be assembled
  pub fn empty(&self) -> bool {
    self.unassembled_bytes() == 0
  }
}

#[cfg(test)]
mod tests;
