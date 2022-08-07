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
  /// Write one character into the stream.
  pub fn write_char(&mut self, data: char) -> bool {
    if self.remaining_capacity() == 0 {
      return false;
    }
    self.buffer.push_back(data);
    self.bytes_written += 1;
    true
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
mod tests;
