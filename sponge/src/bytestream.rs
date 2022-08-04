#![warn(missing_docs)]
//! An in-order byte stream.

//! Bytes are written on the "input" side and read from the "output"
//! side.  The byte stream is finite: the writer can end the input,
//! and then no more bytes can be written.
use std::collections::VecDeque;
/// The ByteStream struct
pub struct ByteStream {
  buffer: VecDeque<char>,
  capacity: usize,
  bytes_written: usize,
  bytes_read: usize,
  is_ended: bool,
  is_error: bool,
}

impl ByteStream {
  /// Construct a stream with room for `capacity` bytes.
  pub fn new(size: usize) -> ByteStream {
    ByteStream {
      buffer: VecDeque::new(),
      capacity: size,
      bytes_written: 0,
      bytes_read: 0,
      is_ended: false,
      is_error: false,
    }
  }

  /// Write a string of bytes into the stream. Write as many
  /// as will fit, and return how many were written.
  pub fn write(&mut self, data: &str) -> usize {
    let num = data.len().min(self.remaining_capacity());
    self.buffer.extend(data[0..num].chars());
    self.bytes_written += num;
    num
  }

  /// Returns the number of additional bytes that the stream has space for
  pub fn remaining_capacity(&self) -> usize {
    self.capacity - self.buffer.len()
  }

  /// Signal that the byte stream has reached its ending
  pub fn end_input(&mut self) {
    self.is_ended = true;
  }

  /// Indicate that the stream suffered an error.
  pub fn set_error(&mut self) {
    self.is_error = true;
  }

  /// Peek at next "len" bytes of the stream
  pub fn peek_output(&self, len: usize) -> String {
    let num = len.min(self.buffer_size());
    self.buffer.range(..num).collect()
  }
  /// Remove bytes from the buffer
  pub fn pop_output(&mut self, len: usize) {
    let num = len.min(self.buffer_size());
    self.buffer.drain(..num);
    self.bytes_read += num;
  }

  /// Read (i.e., copy and then pop) the next "len" bytes of the stream
  pub fn read(&mut self, len: usize) -> String {
    let readout = self.peek_output(len);
    self.pop_output(len);
    readout
  }

  /// Returns `true` if the stream input has ended
  pub fn input_ended(&self) -> bool {
    self.is_ended
  }

  /// Returns `true` if the output has reached the ending
  pub fn eof(&self) -> bool {
    self.is_ended && self.buffer_size() == 0
  }

  /// Returns `true` if the stream has suffered an error
  pub fn error(&self) -> bool {
    self.is_error
  }

  /// Returns the maximum amount that can currently be peeked/read
  pub fn buffer_size(&self) -> usize {
    self.buffer.len()
  }

  ///\Rturns `true` if the buffer is empty
  pub fn buffer_empty(&self) -> bool {
    self.buffer.is_empty()
  }

  /// Total number of bytes written
  pub fn bytes_written(&self) -> usize {
    self.bytes_written
  }

  /// Total number of bytes popped
  pub fn bytes_read(&self) -> usize {
    self.bytes_read
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  // Bytestream construction
  #[test]
  fn byte_stream_construction() {
    let stream = ByteStream::new(15);

    assert_eq!(stream.input_ended(), false);
    assert_eq!(stream.buffer_empty(), true);
    assert_eq!(stream.eof(), false);
    assert_eq!(stream.bytes_read(), 0);
    assert_eq!(stream.bytes_written(), 0);
    assert_eq!(stream.remaining_capacity(), 15);
    assert_eq!(stream.buffer_size(), 0);
  }
  #[test]
  fn byte_stream_construction_end() {
    let mut stream = ByteStream::new(15);
    stream.end_input();
    assert_eq!(stream.input_ended(), true);
    assert_eq!(stream.buffer_empty(), true);
    assert_eq!(stream.eof(), true);
    assert_eq!(stream.bytes_read(), 0);
    assert_eq!(stream.bytes_written(), 0);
    assert_eq!(stream.remaining_capacity(), 15);
    assert_eq!(stream.buffer_size(), 0);
  }
  // Bytestream one write
  #[test]
  fn write_end_pop() {
    let mut stream = ByteStream::new(15);

    assert_eq!(stream.write("cat"), 3);

    assert_eq!(stream.input_ended(), false);
    assert_eq!(stream.buffer_empty(), false);
    assert_eq!(stream.eof(), false);
    assert_eq!(stream.bytes_read(), 0);
    assert_eq!(stream.bytes_written(), 3);
    assert_eq!(stream.remaining_capacity(), 12);
    assert_eq!(stream.buffer_size(), 3);
    assert_eq!(stream.peek_output(3), "cat");

    stream.end_input();

    assert_eq!(stream.input_ended(), true);
    assert_eq!(stream.buffer_empty(), false);
    assert_eq!(stream.eof(), false);
    assert_eq!(stream.bytes_read(), 0);
    assert_eq!(stream.bytes_written(), 3);
    assert_eq!(stream.remaining_capacity(), 12);
    assert_eq!(stream.buffer_size(), 3);
    assert_eq!(stream.peek_output(3), "cat");

    stream.pop_output(3);

    assert_eq!(stream.input_ended(), true);
    assert_eq!(stream.buffer_empty(), true);
    assert_eq!(stream.eof(), true);
    assert_eq!(stream.bytes_read(), 3);
    assert_eq!(stream.bytes_written(), 3);
    assert_eq!(stream.remaining_capacity(), 15);
    assert_eq!(stream.buffer_size(), 0);
  }

  #[test]
  fn write_pop_end() {
    let mut stream = ByteStream::new(15);

    assert_eq!(stream.write("cat"), 3);

    assert_eq!(stream.input_ended(), false);
    assert_eq!(stream.buffer_empty(), false);
    assert_eq!(stream.eof(), false);
    assert_eq!(stream.bytes_read(), 0);
    assert_eq!(stream.bytes_written(), 3);
    assert_eq!(stream.remaining_capacity(), 12);
    assert_eq!(stream.buffer_size(), 3);
    assert_eq!(stream.peek_output(3), "cat");

    stream.pop_output(3);

    assert_eq!(stream.input_ended(), false);
    assert_eq!(stream.buffer_empty(), true);
    assert_eq!(stream.eof(), false);
    assert_eq!(stream.bytes_read(), 3);
    assert_eq!(stream.bytes_written(), 3);
    assert_eq!(stream.remaining_capacity(), 15);
    assert_eq!(stream.buffer_size(), 0);

    stream.end_input();

    assert_eq!(stream.input_ended(), true);
    assert_eq!(stream.buffer_empty(), true);
    assert_eq!(stream.eof(), true);
    assert_eq!(stream.bytes_read(), 3);
    assert_eq!(stream.bytes_written(), 3);
    assert_eq!(stream.remaining_capacity(), 15);
    assert_eq!(stream.buffer_size(), 0);
  }

  #[test]
  fn write_pop2_end() {
    let mut stream = ByteStream::new(15);

    assert_eq!(stream.write("cat"), 3);

    assert_eq!(stream.input_ended(), false);
    assert_eq!(stream.buffer_empty(), false);
    assert_eq!(stream.eof(), false);
    assert_eq!(stream.bytes_read(), 0);
    assert_eq!(stream.bytes_written(), 3);
    assert_eq!(stream.remaining_capacity(), 12);
    assert_eq!(stream.buffer_size(), 3);
    assert_eq!(stream.peek_output(3), "cat");

    stream.pop_output(1);

    assert_eq!(stream.input_ended(), false);
    assert_eq!(stream.buffer_empty(), false);
    assert_eq!(stream.eof(), false);
    assert_eq!(stream.bytes_read(), 1);
    assert_eq!(stream.bytes_written(), 3);
    assert_eq!(stream.remaining_capacity(), 13);
    assert_eq!(stream.buffer_size(), 2);
    assert_eq!(stream.peek_output(2), "at");

    stream.pop_output(2);

    assert_eq!(stream.input_ended(), false);
    assert_eq!(stream.buffer_empty(), true);
    assert_eq!(stream.eof(), false);
    assert_eq!(stream.bytes_read(), 3);
    assert_eq!(stream.bytes_written(), 3);
    assert_eq!(stream.remaining_capacity(), 15);
    assert_eq!(stream.buffer_size(), 0);

    stream.end_input();

    assert_eq!(stream.input_ended(), true);
    assert_eq!(stream.buffer_empty(), true);
    assert_eq!(stream.eof(), true);
    assert_eq!(stream.bytes_read(), 3);
    assert_eq!(stream.bytes_written(), 3);
    assert_eq!(stream.remaining_capacity(), 15);
    assert_eq!(stream.buffer_size(), 0);
  }
  // Bytestream two writes
  #[test]
  fn write_write_end_pop_pop() {
    let mut stream = ByteStream::new(15);

    assert_eq!(stream.write("cat"), 3);

    assert_eq!(stream.input_ended(), false);
    assert_eq!(stream.buffer_empty(), false);
    assert_eq!(stream.eof(), false);
    assert_eq!(stream.bytes_read(), 0);
    assert_eq!(stream.bytes_written(), 3);
    assert_eq!(stream.remaining_capacity(), 12);
    assert_eq!(stream.buffer_size(), 3);
    assert_eq!(stream.peek_output(3), "cat");

    assert_eq!(stream.write("tac"), 3);

    assert_eq!(stream.input_ended(), false);
    assert_eq!(stream.buffer_empty(), false);
    assert_eq!(stream.eof(), false);
    assert_eq!(stream.bytes_read(), 0);
    assert_eq!(stream.bytes_written(), 6);
    assert_eq!(stream.remaining_capacity(), 9);
    assert_eq!(stream.buffer_size(), 6);
    assert_eq!(stream.peek_output(6), "cattac");

    stream.end_input();

    assert_eq!(stream.input_ended(), true);
    assert_eq!(stream.buffer_empty(), false);
    assert_eq!(stream.eof(), false);
    assert_eq!(stream.bytes_read(), 0);
    assert_eq!(stream.bytes_written(), 6);
    assert_eq!(stream.remaining_capacity(), 9);
    assert_eq!(stream.buffer_size(), 6);
    assert_eq!(stream.peek_output(6), "cattac");

    stream.pop_output(2);

    assert_eq!(stream.input_ended(), true);
    assert_eq!(stream.buffer_empty(), false);
    assert_eq!(stream.eof(), false);
    assert_eq!(stream.bytes_read(), 2);
    assert_eq!(stream.bytes_written(), 6);
    assert_eq!(stream.remaining_capacity(), 11);
    assert_eq!(stream.buffer_size(), 4);
    assert_eq!(stream.peek_output(4), "ttac");

    stream.pop_output(4);

    assert_eq!(stream.input_ended(), true);
    assert_eq!(stream.buffer_empty(), true);
    assert_eq!(stream.eof(), true);
    assert_eq!(stream.bytes_read(), 6);
    assert_eq!(stream.bytes_written(), 6);
    assert_eq!(stream.remaining_capacity(), 15);
    assert_eq!(stream.buffer_size(), 0);
  }
  #[test]
  fn write_pop_write_end_pop() {
    let mut stream = ByteStream::new(15);

    assert_eq!(stream.write("cat"), 3);

    assert_eq!(stream.input_ended(), false);
    assert_eq!(stream.buffer_empty(), false);
    assert_eq!(stream.eof(), false);
    assert_eq!(stream.bytes_read(), 0);
    assert_eq!(stream.bytes_written(), 3);
    assert_eq!(stream.remaining_capacity(), 12);
    assert_eq!(stream.buffer_size(), 3);
    assert_eq!(stream.peek_output(3), "cat");

    stream.pop_output(2);

    assert_eq!(stream.input_ended(), false);
    assert_eq!(stream.buffer_empty(), false);
    assert_eq!(stream.eof(), false);
    assert_eq!(stream.bytes_read(), 2);
    assert_eq!(stream.bytes_written(), 3);
    assert_eq!(stream.remaining_capacity(), 14);
    assert_eq!(stream.buffer_size(), 1);
    assert_eq!(stream.peek_output(1), "t");

    assert_eq!(stream.write("tac"), 3);

    assert_eq!(stream.input_ended(), false);
    assert_eq!(stream.buffer_empty(), false);
    assert_eq!(stream.eof(), false);
    assert_eq!(stream.bytes_read(), 2);
    assert_eq!(stream.bytes_written(), 6);
    assert_eq!(stream.remaining_capacity(), 11);
    assert_eq!(stream.buffer_size(), 4);
    assert_eq!(stream.peek_output(4), "ttac");

    stream.end_input();

    assert_eq!(stream.input_ended(), true);
    assert_eq!(stream.buffer_empty(), false);
    assert_eq!(stream.eof(), false);
    assert_eq!(stream.bytes_read(), 2);
    assert_eq!(stream.bytes_written(), 6);
    assert_eq!(stream.remaining_capacity(), 11);
    assert_eq!(stream.buffer_size(), 4);
    assert_eq!(stream.peek_output(4), "ttac");

    stream.pop_output(4);

    assert_eq!(stream.input_ended(), true);
    assert_eq!(stream.buffer_empty(), true);
    assert_eq!(stream.eof(), true);
    assert_eq!(stream.bytes_read(), 6);
    assert_eq!(stream.bytes_written(), 6);
    assert_eq!(stream.remaining_capacity(), 15);
    assert_eq!(stream.buffer_size(), 0);
  }
  // Bytestream many writes
  #[test]
  fn many_writes() {
    use rand::{distributions::Alphanumeric, prelude::*};
    const NREPS: usize = 1000;
    const MIN_WRITE: usize = 10;
    const MAX_WRITE: usize = 200;
    const CAPACITY: usize = NREPS * MAX_WRITE;

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
  // Bytestream capacity

  #[test]
  fn overwrite() {
    let mut stream = ByteStream::new(2);

    assert_eq!(stream.write("cat"), 2);

    assert_eq!(stream.input_ended(), false);
    assert_eq!(stream.buffer_empty(), false);
    assert_eq!(stream.eof(), false);
    assert_eq!(stream.bytes_read(), 0);
    assert_eq!(stream.bytes_written(), 2);
    assert_eq!(stream.remaining_capacity(), 0);
    assert_eq!(stream.buffer_size(), 2);
    assert_eq!(stream.peek_output(2), "ca");

    assert_eq!(stream.write("t"), 0);

    assert_eq!(stream.input_ended(), false);
    assert_eq!(stream.buffer_empty(), false);
    assert_eq!(stream.eof(), false);
    assert_eq!(stream.bytes_read(), 0);
    assert_eq!(stream.bytes_written(), 2);
    assert_eq!(stream.remaining_capacity(), 0);
    assert_eq!(stream.buffer_size(), 2);
    assert_eq!(stream.peek_output(2), "ca");
  }

  #[test]
  fn overwrite_clear_overwrite() {
    let mut stream = ByteStream::new(2);

    assert_eq!(stream.write("cat"), 2);
    stream.pop_output(2);
    assert_eq!(stream.write("cat"), 2);

    assert_eq!(stream.input_ended(), false);
    assert_eq!(stream.buffer_empty(), false);
    assert_eq!(stream.eof(), false);
    assert_eq!(stream.bytes_read(), 2);
    assert_eq!(stream.bytes_written(), 4);
    assert_eq!(stream.remaining_capacity(), 0);
    assert_eq!(stream.buffer_size(), 2);
    assert_eq!(stream.peek_output(2), "ca");
  }

  #[test]
  fn overwrite_pop_overwrite() {
    let mut stream = ByteStream::new(2);

    assert_eq!(stream.write("cat"), 2);
    stream.pop_output(1);
    assert_eq!(stream.write("tac"), 1);

    assert_eq!(stream.input_ended(), false);
    assert_eq!(stream.buffer_empty(), false);
    assert_eq!(stream.eof(), false);
    assert_eq!(stream.bytes_read(), 1);
    assert_eq!(stream.bytes_written(), 3);
    assert_eq!(stream.remaining_capacity(), 0);
    assert_eq!(stream.buffer_size(), 2);
    assert_eq!(stream.peek_output(2), "at");
  }

  #[test]
  fn long_stream() {
    let mut stream = ByteStream::new(3);

    assert_eq!(stream.write("abcdef"), 3);
    assert_eq!(stream.peek_output(3), "abc");
    stream.pop_output(1);

    for _ in 0..99997 {
      assert_eq!(stream.remaining_capacity(), 1);
      assert_eq!(stream.buffer_size(), 2);
      assert_eq!(stream.write("abc"), 1);
      assert_eq!(stream.remaining_capacity(), 0);
      assert_eq!(stream.peek_output(3), "bca");
      stream.pop_output(1);

      assert_eq!(stream.remaining_capacity(), 1);
      assert_eq!(stream.buffer_size(), 2);
      assert_eq!(stream.write("bca"), 1);
      assert_eq!(stream.remaining_capacity(), 0);
      assert_eq!(stream.peek_output(3), "cab");
      stream.pop_output(1);

      assert_eq!(stream.remaining_capacity(), 1);
      assert_eq!(stream.buffer_size(), 2);
      assert_eq!(stream.write("cab"), 1);
      assert_eq!(stream.remaining_capacity(), 0);
      assert_eq!(stream.peek_output(3), "abc");
      stream.pop_output(1);
    }

    stream.end_input();
    assert_eq!(stream.peek_output(2), "bc");
    stream.pop_output(2);
    assert_eq!(stream.eof(), true);
  }
}
