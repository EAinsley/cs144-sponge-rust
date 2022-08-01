use std::collections::VecDeque;
pub struct ByteStream {
  buffer: VecDeque<char>,
  capacity: usize,
  bytes_written: usize,
  bytes_read: usize,
  is_ended: bool,
  is_error: bool,
}

impl ByteStream {
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
  pub fn write(&mut self, data: &str) -> usize {
    let num = data.len().min(self.remaining_capacity());
    self.buffer.extend(data[0..num].chars());
    self.bytes_written += num;
    num
  }
  pub fn remaining_capacity(&self) -> usize {
    self.capacity - self.buffer.len()
  }
  pub fn end_input(&mut self) {
    self.is_ended = true;
  }
  pub fn set_error(&mut self) {
    self.is_error = true;
  }
  pub fn peek_output(&self, len: usize) -> String {
    let num = len.min(self.buffer_size());
    self.buffer.range(..num).collect()
  }
  pub fn pop_output(&mut self, len: usize) {
    let num = len.min(self.buffer_size());
    self.buffer.drain(..num);
    self.bytes_read += num;
  }
  pub fn read(&mut self, len: usize) -> String {
    let readout = self.peek_output(len);
    self.pop_output(len);
    readout
  }
  pub fn input_ended(&self) -> bool {
    self.is_ended
  }
  pub fn eof(&self) -> bool {
    self.is_ended && self.buffer_size() == 0
  }
  pub fn error(&self) -> bool {
    self.is_error
  }
  /// the maximum amount that can currently be peeked/read
  pub fn buffer_size(&self) -> usize {
    self.buffer.len()
  }
  pub fn buffer_empty(&self) -> bool {
    self.buffer.is_empty()
  }
  pub fn bytes_written(&self) -> usize {
    self.bytes_written
  }
  pub fn bytes_read(&self) -> usize {
    self.bytes_read
  }
}
