use super::buffer::Buffer;
use num::traits::{FromPrimitive, PrimInt, Unsigned, Zero};

use std::mem;
#[derive(Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub enum ParserResult {
  NoError,
  BadChecksum,
  PacketTooShort,
  WrongIPVersion,
  HeaderTooShort,
  TruncatedPacket,
  Unsupported,
}

pub struct NetParser {
  buffer: Buffer,
  error: ParserResult,
}
#[allow(dead_code)]
impl NetParser {
  // Check that there is sufficient data to parse the next tocken
  fn check_size(&mut self, size: usize) {
    if size > self.buffer.size() {
      self.set_error(ParserResult::PacketTooShort)
    }
  }
  // Generic integer parsing method used by (u32, u16, u8)
  fn parse_int<T>(&mut self) -> T
  where
    T: PrimInt + Unsigned + Zero + FromPrimitive,
  {
    let len: usize = mem::size_of::<T>();
    self.check_size(len);
    if self.error() {
      return T::zero();
    }
    let mut ret: T = T::zero();
    for i in 0..len {
      ret = ret << 8;
      ret = ret + T::from_u8(self.buffer.at(i)).unwrap();
    }
    self.buffer.remove_prefix(len);
    ret
  }
  // public functions
  pub fn new_with_buffer(buffer: Buffer) -> NetParser {
    NetParser {
      buffer,
      error: ParserResult::NoError,
    }
  }
  pub fn buffer(&self) -> Buffer {
    self.buffer.clone()
  }
  pub fn get_error(&self) -> ParserResult {
    self.error
  }
  pub fn set_error(&mut self, _res: ParserResult) {
    todo!()
  }
  pub fn error(&self) -> bool {
    self.get_error() != ParserResult::NoError
  }
  pub fn u32(&mut self) -> u32 {
    self.parse_int::<u32>()
  }
  pub fn u16(&self) -> u16 {
    todo!()
  }
  pub fn u8(&self) -> u8 {
    todo!()
  }
  pub fn remove_prefix(&mut self, n: usize) {
    self.check_size(n);
    if self.error() {
      return;
    }
    self.buffer.remove_prefix(n);
  }
}
impl ToString for ParserResult {
  fn to_string(&self) -> String {
    static NAMES: [&str; 6] = [
      "NoError",
      "BadChecksum",
      "PacketTooShort",
      "WrongIPVersion",
      "HeaderTooShort",
      "TruncatedPacket",
    ];
    NAMES[*self as usize].to_owned()
  }
}
