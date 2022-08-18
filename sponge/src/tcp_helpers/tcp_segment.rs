use super::TCPHeader;
use crate::utils::Buffer;
pub struct TCPSegment {
  header: TCPHeader,
  payload: Buffer,
}
impl TCPSegment {
  pub fn header(&self) -> TCPHeader {
    self.header
  }
  pub fn length_in_sequence_space(&self) -> usize {
    todo!();
  }
  pub fn payload(&self) -> &Buffer {
    &self.payload
  }
}
