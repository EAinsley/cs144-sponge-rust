use super::super::utils::parser::{NetParser, ParserResult};
use super::super::WrappingInt32;

#[derive(PartialEq, Eq)]
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
  sport: u16,
  dport: u16,
  seqno: WrappingInt32,
  ackno: WrappingInt32,
  doff: u8,
  urg: bool,
  ack: bool,
  psh: bool,
  rst: bool,
  syn: bool,
  fin: bool,
  win: u16,
  cksum: u16,
  uptr: u16,
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
