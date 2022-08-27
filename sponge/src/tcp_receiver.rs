use crate::tcp_helpers::TCPSegment;
use crate::ByteStream;

use super::{StreamReassembler, WrappingInt32};
pub struct TCPReceiver {
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
    if let Some(isn) = self.isn {
      Some(WrappingInt32::wrap(self.seqno, isn))
    } else {
      None
    }
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
    self.capacity
      - (self.reassembler.as_stream().bytes_written()
        - self.reassembler.as_stream().bytes_read())
  }

  /// number of bytes stored but not yet reassembled
  pub fn unassembled_bytes(&self) -> usize {
    self.reassembler.unassembled_bytes()
  }

  /// handle an inbound segment
  pub fn segment_received(&mut self, seg: &TCPSegment) {
    // check syn
    if seg.header().syn {
      self.isn = Some(seg.header().seqno);
    }
    if self.isn == None {
      return;
    }

    // check fin
    let seqno_unwrap = WrappingInt32::unwrap(
      seg.header().seqno,
      self.isn.unwrap(),
      self.seqno,
    );
    if seg.header().fin {
      self.fin_seq =
        Some(seqno_unwrap + seg.length_in_sequence_space() as u64);
    }

    // compute index(absolute seqno)
    let index = if seg.header().syn {
      seqno_unwrap
    } else {
      seqno_unwrap - 1
    };
    self.reassembler.push_substring(
      seg.payload().copy().as_str(),
      index as usize,
      seg.header().fin,
    );
    // update the seqno
    self.seqno =
      (self.reassembler.as_stream().bytes_written() + 1) as u64;
    if self.fin_seq == Some(self.seqno + 1) {
      self.seqno += 1;
    }
  }

  pub fn as_stream(&self) -> &ByteStream {
    self.reassembler.as_stream()
  }
  pub fn as_mut_stream(&mut self) -> &mut ByteStream {
    self.reassembler.as_mut_stream()
  }
}

#[cfg(test)]
mod tests;
