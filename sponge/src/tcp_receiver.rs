use crate::tcp_helpers::TCPSegment;
use crate::ByteStream;

use super::{StreamReassembler, WrappingInt32};
struct TCPReceiver {
  reassembler: StreamReassembler,
  capacity: usize,
  seqno: u64,
  isn: Option<WrappingInt32>,
  fin_seq: Option<u64>,
}

impl TCPReceiver {
  /// Construct a TCP receiver
  /// capacity: the maximu number of bytes that the receiver
  /// will store in its buffers at any given time
  pub fn new(capacity: usize) -> TCPReceiver {
    TCPReceiver {
      reassembler: StreamReassembler::new(capacity),
      capacity,
      seqno: 0,
      isn: None,
      fin_seq: None,
    }
  }
  /// The ackno that should be sent to the peer
  /// returns empty if no SYN has been received
  //
  // This is the beginning of the receiver's window, or in other words, the
  // sequence number of the first byte in the stream that the receiver hasn't
  // received.
  pub fn ackno(&self) -> Option<WrappingInt32> {
    todo!()
  }
  /// The window size that should be sent to the peer
  ///
  /// Operationally: the capacity minus the number of bytes that the
  /// TCPReceiver is holding in its byte stream (those that have been
  /// reassembled, but not consumed).
  ///
  /// Formally: the difference between (a) the sequence number of
  /// the first byte that falls after the window (and will not be
  /// accepted by the receiver) and (b) the sequence number of the
  /// beginning of the window (the ackno).
  pub fn window_size(&self) -> usize {
    todo!()
  }

  /// number of bytes stored but not yet reassembled
  pub fn unassembled_bytes(&self) -> usize {
    self.reassembler.unassembled_bytes()
  }

  /// handle an inbound segment
  pub fn segment_received(seg: &TCPSegment) {
    todo!();
  }

  pub fn as_stream(&self) -> &ByteStream {
    self.reassembler.as_stream()
  }
  pub fn as_mut_stream(&mut self) -> &mut ByteStream {
    self.reassembler.as_mut_stream()
  }
}
