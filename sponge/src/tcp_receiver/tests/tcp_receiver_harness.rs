use super::super::*;
use crate::{tcp_helpers::TCPState, utils::Buffer};
use std::{error::Error, fmt::Display};
pub trait ReceiverTestStep {
  const MSG: &'static str;
  fn execute(
    &self,
    receiver: &mut TCPReceiver,
  ) -> Result<(), ReceiverExpectationViolation>;
  fn to_string(&self) -> String {
    String::from(format!("{}: {}", Self::MSG, self.description()))
  }
  fn description(&self) -> String;
}
#[derive(Debug)]
pub struct ReceiverExpectationViolation {
  msg: String,
}

impl Display for ReceiverExpectationViolation {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.msg)
  }
}

impl Error for ReceiverExpectationViolation {}

pub struct ExpectState {
  pub state: &'static str,
}
impl ReceiverTestStep for ExpectState {
  const MSG: &'static str = "Expectation";
  fn execute(
    &self,
    receiver: &mut TCPReceiver,
  ) -> Result<(), ReceiverExpectationViolation> {
    if TCPState::state_summary(receiver) != self.state {
      Err(ReceiverExpectationViolation {
        msg: format!(
          "The TCPReceiver was in state \"{}\", \
          but it was expected to be in state \"{}\"",
          TCPState::state_summary(receiver),
          self.state
        ),
      })
    } else {
      Ok(())
    }
  }
  fn description(&self) -> String {
    format!("in state '{}'", self.state)
  }
}

pub struct ExpectAckno {
  pub ackno: Option<WrappingInt32>,
}

impl ReceiverTestStep for ExpectAckno {
  const MSG: &'static str = "Expectation";
  fn description(&self) -> String {
    if let Some(value) = self.ackno {
      "ackno ".to_owned() + &value.to_string()
    } else {
      "no ackno available".to_owned()
    }
  }
  fn execute(
    &self,
    receiver: &mut TCPReceiver,
  ) -> Result<(), ReceiverExpectationViolation> {
    if receiver.ackno() != self.ackno {
      let reported = if let Some(value) = receiver.ackno() {
        value.to_string()
      } else {
        "none".to_owned()
      };
      let expected = if let Some(value) = self.ackno {
        value.to_string()
      } else {
        "none".to_owned()
      };
      Err(ReceiverExpectationViolation {
        msg: format!(
          "The TCPReceiver reported ackno \"{}\", \
          but it was expected to be \"{}\"",
          reported, expected
        ),
      })
    } else {
      Ok(())
    }
  }
}

pub struct ExpectWindow {
  pub window: usize,
}
impl ReceiverTestStep for ExpectWindow {
  const MSG: &'static str = "Expectation";
  fn description(&self) -> String {
    format!("window: {}", self.window.to_string())
  }
  fn execute(
    &self,
    receiver: &mut TCPReceiver,
  ) -> Result<(), ReceiverExpectationViolation> {
    if receiver.window_size() != self.window {
      Err(ReceiverExpectationViolation {
        msg: format!(
          "The TCPReceiver reported window \"{}\", \
          but it was expected to be \"{}\"",
          &receiver.window_size().to_string(),
          &self.window.to_string()
        ),
      })
    } else {
      Ok(())
    }
  }
}

pub struct ExpectUnassembledBytes {
  pub n_bytes: usize,
}

impl ReceiverTestStep for ExpectUnassembledBytes {
  const MSG: &'static str = "Expectation";
  fn description(&self) -> String {
    self.n_bytes.to_string() + " unassembled bytes"
  }
  fn execute(
    &self,
    receiver: &mut TCPReceiver,
  ) -> Result<(), ReceiverExpectationViolation> {
    if receiver.unassembled_bytes() != self.n_bytes {
      Err(ReceiverExpectationViolation {
        msg: format!(
          "The TCPReceiver reported \"{}\" unassembled bytes, \
          but there was expected to be \"{}\" unassembled bytes",
          receiver.unassembled_bytes(),
          self.n_bytes
        ),
      })
    } else {
      Ok(())
    }
  }
}

pub struct ExpectAssembledBytes {
  pub n_bytes: usize,
}

impl ReceiverTestStep for ExpectAssembledBytes {
  const MSG: &'static str = "Expectation";
  fn description(&self) -> String {
    self.n_bytes.to_string() + " assembled bytes, in total"
  }
  fn execute(
    &self,
    receiver: &mut TCPReceiver,
  ) -> Result<(), ReceiverExpectationViolation> {
    if receiver.as_stream().bytes_written() != self.n_bytes {
      Err(ReceiverExpectationViolation {
        msg: format!(
          "The TCPReceiver reported \"{}\" bytes written, \
          but there was expected to be \"{}\" bytes written (in total)",
          receiver.as_stream().bytes_written(),
          self.n_bytes
        ),
      })
    } else {
      Ok(())
    }
  }
}

pub struct ExpectEof;

impl ReceiverTestStep for ExpectEof {
  const MSG: &'static str = "Expectation";

  fn execute(
    &self,
    receiver: &mut TCPReceiver,
  ) -> Result<(), ReceiverExpectationViolation> {
    if !receiver.as_stream().eof() {
      Err(ReceiverExpectationViolation {
        msg: String::from(
          "The TCPReceiver stream reported eof() == false, \
          but was expected to be true",
        ),
      })
    } else {
      Ok(())
    }
  }

  fn description(&self) -> String {
    String::from("receiver.stream_out().eof() == true")
  }
}

pub struct ExpectInputNotEnded;

impl ReceiverTestStep for ExpectInputNotEnded {
  const MSG: &'static str = "Expectation";

  fn execute(
    &self,
    receiver: &mut TCPReceiver,
  ) -> Result<(), ReceiverExpectationViolation> {
    if receiver.as_stream().input_ended() {
      Err(ReceiverExpectationViolation {
        msg: String::from(
          "The TCPReceiver stream reported input_ended() == true, \
          but was expected to be false",
        ),
      })
    } else {
      Ok(())
    }
  }

  fn description(&self) -> String {
    String::from("receiver.stream_out().input_ended() == false")
  }
}

pub struct ExpectBytes {
  pub bytes: String,
}
impl ReceiverTestStep for ExpectBytes {
  const MSG: &'static str = "Expectation";
  fn description(&self) -> String {
    format!("bytes available: \"{}\"", self.bytes)
  }
  fn execute(
    &self,
    receiver: &mut TCPReceiver,
  ) -> Result<(), ReceiverExpectationViolation> {
    let stream = receiver.as_mut_stream();
    if stream.buffer_size() != self.bytes.len() {
      return Err(ReceiverExpectationViolation {
        msg: format!(
          "The TCPReceiver reported \"{}\" bytes available, \
          but there were expectated to be \"{}\" bytes available",
          stream.buffer_size(),
          self.bytes.len()
        ),
      });
    }
    let bytes = stream.read(self.bytes.len());
    if !bytes.eq(&self.bytes) {
      return Err(ReceiverExpectationViolation {
        msg: format!(
          "the TCPReceiver assembled \"{}\", \
          but was expected to assemble \"{}\"",
          bytes, self.bytes
        ),
      });
    }
    todo!()
  }
}
#[derive(PartialEq, Clone, Copy)]
#[allow(non_camel_case_types)]
pub enum SegmentResult {
  NOT_SYN,
  OK,
}
pub struct SegmentArrives {
  ack: bool,
  rst: bool,
  syn: bool,
  fin: bool,
  seqno: WrappingInt32,
  ackno: WrappingInt32,
  win: u16,
  data: String,
  result: Option<SegmentResult>,
}

impl SegmentArrives {
  const fn result_name(res: SegmentResult) -> &'static str {
    match res {
      SegmentResult::OK => "(no SYN received, so no ackno available)",
      SegmentResult::NOT_SYN => "(SYN received, so ackno available)",
    }
  }

  pub fn with_ack_wrapping(
    &mut self,
    ackno_: WrappingInt32,
  ) -> &SegmentArrives {
    self.ack = true;
    self.ackno = ackno_;
    self
  }
  pub fn with_ack_u32(&mut self, ackno_: u32) -> &SegmentArrives {
    self.with_ack_wrapping(WrappingInt32::new(ackno_))
  }
  pub fn with_rst(&mut self) -> &SegmentArrives {
    self.rst = true;
    self
  }
  pub fn with_syn(&mut self) -> &SegmentArrives {
    self.syn = true;
    self
  }
  pub fn with_fin(&mut self) -> &SegmentArrives {
    self.fin = true;
    self
  }
  pub fn with_sequo_Wrapping(
    &mut self,
    seqno_: WrappingInt32,
  ) -> &SegmentArrives {
    self.seqno = seqno_;
    self
  }
  pub fn with_seqno_u32(&mut self, seqno_: u32) -> &SegmentArrives {
    self.with_sequo_Wrapping(WrappingInt32::new(seqno_))
  }
  pub fn with_win(&mut self, win_: u16) -> &SegmentArrives {
    self.win = win_;
    self
  }
  pub fn with_data(&mut self, data_: String) -> &SegmentArrives {
    self.data = data_;
    self
  }
  pub fn with_result(
    &mut self,
    result_: SegmentResult,
  ) -> &SegmentArrives {
    self.result = Some(result_);
    self
  }
  pub fn build_segment(&self) -> TCPSegment {
    let mut seg = TCPSegment::new();
    *seg.payload_mut() = Buffer::from_string(self.data.clone());
    seg.header_mut().ack = self.ack;
    seg.header_mut().fin = self.fin;
    seg.header_mut().syn = self.syn;
    seg.header_mut().rst = self.rst;
    seg.header_mut().ackno = self.ackno;
    seg.header_mut().seqno = self.seqno;
    seg.header_mut().win = self.win;
    seg
  }
}
impl ReceiverTestStep for SegmentArrives {
  const MSG: &'static str = "Action";

  fn execute(
    &self,
    receiver: &mut TCPReceiver,
  ) -> Result<(), ReceiverExpectationViolation> {
    let seg = self.build_segment();
    let mut o = seg.header().summary();
    if self.data.len() > 0 {
      o += &format!(" with data \"{}\"", self.data);
    }
    receiver.segment_received(&seg);
    let res = if receiver.ackno().is_some() {
      SegmentResult::OK
    } else {
      SegmentResult::NOT_SYN
    };
    if let Some(value) = self.result {
      if value != res {
        return Err(ReceiverExpectationViolation {
          msg: format!(
            "TCPReceiver::segment_received() reported \"{}\" \
            in response to \"{}\", \
            but it was expected to report \"{}\"",
            SegmentArrives::result_name(res),
            o,
            SegmentArrives::result_name(value)
          ),
        });
      };
    }
    Ok(())
  }

  fn description(&self) -> String {
    let seg = self.build_segment();
    let mut o = format!("segment arrives {}", seg.header().summary());
    if self.data.len() > 0 {
      o += &format!(" with data \"{}\"", self.data);
    }
    o
  }
}

pub struct TCPReceiverTestHarness {
  receiver: TCPReceiver,
  steps_executed: Vec<String>,
}

impl TCPReceiverTestHarness {
  pub fn with_capacity(capacity: usize) -> TCPReceiverTestHarness {
    TCPReceiverTestHarness {
      receiver: TCPReceiver::new(capacity),
      steps_executed: vec![format!(
        "Initialized with (capacity={})",
        capacity
      )],
    }
  }
  pub fn execute<T>(&mut self, step: &T)
  where
    T: ReceiverTestStep,
  {
    if let Err(ReceiverExpectationViolation { msg }) =
      step.execute(&mut self.receiver)
    {
      let mut em = format!(
        "Thest Failure on expectation:\n\t{}",
        step.to_string()
      );
      em += &format!("\nFailure message:\n\t{}", msg);
      em += "\nList of steps that executed successfully:";
      for s in self.steps_executed.iter() {
        em += &format!("\n\t{}", s);
      }
      em += "\n";
      panic!("{}", em);
    }
    self.steps_executed.push(step.to_string());
  }
}
