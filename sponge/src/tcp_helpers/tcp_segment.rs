use super::TCPHeader;
use crate::utils::Buffer;
pub struct TCPSegment {
  header: TCPHeader,
  payload: Buffer,
}
impl TCPSegment {
  pub fn new() -> TCPSegment {
    TCPSegment {
      header: TCPHeader::new(),
      payload: Buffer::new(),
    }
  }
  pub fn header(&self) -> &TCPHeader {
    &self.header
  }
  pub fn header_mut(&mut self) -> &mut TCPHeader {
    &mut self.header
  }
  pub fn length_in_sequence_space(&self) -> usize {
    todo!();
  }
  pub fn payload(&self) -> &Buffer {
    &self.payload
  }
  pub fn payload_mut(&mut self) -> &mut Buffer {
    &mut self.payload
  }
}
