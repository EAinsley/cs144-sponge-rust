#![warn(missing_docs)]
//! An in-order byte stream.

//! Bytes are written on the "input" side and read from the "output"
//! side.  The byte stream is finite: the writer can end the input,
//! and then no more bytes can be written.
use std::collections::VecDeque;
/// The ByteStream struct
pub struct ByteStream {
  buffer: VecDeque<u8>,
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

  /// Write one character into the stream.
  pub fn write_char(&mut self, data: u8) -> bool {
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

  /// Remove bytes from the buffer
  pub fn pop_output(&mut self, len: usize) {
    let num = len.min(self.buffer_size());
    self.buffer.drain(..num);
    self.bytes_read += num;
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

impl std::io::Read for ByteStream {
  /// Read the bytes of the stream to buffer
  /// and return the number of bytes read.
  fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
    let num = buf.len().min(self.buffer_size());
    let readout = self.buffer.read(buf)?;
    self.bytes_read += readout;
    Ok(readout)
  }
}

impl std::io::Write for ByteStream {
  /// Write a string of bytes into the stream. Write as many
  /// as will fit, and return how many were written.
  fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
    let num = buf.len().min(self.remaining_capacity());
    self.buffer.extend(buf[0..num].iter());
    self.bytes_written += num;
    Ok(num)
  }

  fn flush(&mut self) -> std::io::Result<()> {
    self.buffer.flush()
  }
}

#[cfg(test)]
mod tests;
