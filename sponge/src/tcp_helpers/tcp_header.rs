use super::super::utils::parser::{NetParser, ParserResult};
use super::super::WrappingInt32;

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct TCPHeader {
  /// # TCPHeader
  ///   0                   1                   2                   3
  ///   0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
  ///  +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
  ///  |          Source Port          |       Destination Port        |
  ///  +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
  ///  |                        Sequence Number                        |
  ///  +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
  ///  |                    Acknowledgment Number                      |
  ///  +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
  ///  |  Data |           |U|A|P|R|S|F|                               |
  ///  | Offset| Reserved  |R|C|S|S|Y|I|            Window             |
  ///  |       |           |G|K|H|T|N|N|                               |
  ///  +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
  ///  |           Checksum            |         Urgent Pointer        |
  ///  +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
  ///  |                    Options                    |    Padding    |
  ///  +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
  ///  |                             data                              |
  ///  +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
  pub sport: u16,
  pub dport: u16,
  pub seqno: WrappingInt32,
  pub ackno: WrappingInt32,
  pub doff: u8,
  pub urg: bool,
  pub ack: bool,
  pub psh: bool,
  pub rst: bool,
  pub syn: bool,
  pub fin: bool,
  pub win: u16,
  pub cksum: u16,
  pub uptr: u16,
}

impl TCPHeader {
  const _LENGTH: usize = 20;
  pub fn new() -> TCPHeader {
    TCPHeader {
      sport: 0,
      dport: 0,
      seqno: WrappingInt32::new(0),
      ackno: WrappingInt32::new(0),
      doff: TCPHeader::_LENGTH as u8 / 4,
      urg: false,
      ack: false,
      psh: false,
      rst: false,
      syn: false,
      fin: false,
      win: 0,
      cksum: 0,
      uptr: 0,
    }
  }

  pub fn parse(p: &NetParser) -> ParserResult {
    todo!()
  }

  pub fn serialize() -> String {
    todo!()
  }

  pub fn as_string() -> String {
    todo!()
  }

  pub fn summary() -> String {
    todo!()
  }
}
