use super::super::*;
use std::io::Write;
use std::{error::Error, fmt::Display};
pub trait BytestreamTestStep {
  const MSG: &'static str;
  fn execute(
    &self,
    bytestream: &mut ByteStream,
  ) -> Result<(), Box<dyn Error>>;
  fn to_string(&self) -> String {
    String::from(format!("{}: {}", Self::MSG, self.description()))
  }
  fn description(&self) -> String;
}
#[derive(Debug)]
pub struct BytestreamExpectationViolation {
  msg: String,
}

impl BytestreamExpectationViolation {
  pub fn new(msg: String) -> BytestreamExpectationViolation {
    BytestreamExpectationViolation { msg }
  }
  pub fn with_property<T>(
    property_name: String,
    expected: &T,
    actual: &T,
  ) -> BytestreamExpectationViolation
  where
    T: Display,
  {
    BytestreamExpectationViolation {
      msg: format!(
        "The ByteStream should have had \
        {} equal to {} but instead it was {}",
        property_name, expected, actual
      ),
    }
  }
}

impl Display for BytestreamExpectationViolation {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.msg)
  }
}

impl Error for BytestreamExpectationViolation {}

pub struct ActionEndInput;

impl BytestreamTestStep for ActionEndInput {
  const MSG: &'static str = "Action";

  fn execute(
    &self,
    bytestream: &mut ByteStream,
  ) -> Result<(), Box<dyn Error>> {
    bytestream.end_input();
    Ok(())
  }

  fn description(&self) -> String {
    String::from("end input")
  }
}

pub struct ActionWrite<'a> {
  data: &'a [u8],
  bytes_written: Option<usize>,
}

impl<'a> ActionWrite<'a> {
  pub fn new(data: &[u8]) -> ActionWrite {
    ActionWrite {
      data,
      bytes_written: None,
    }
  }
  pub fn with_bytes_written(
    self,
    bytes_written: usize,
  ) -> ActionWrite<'a> {
    ActionWrite {
      data: self.data,
      bytes_written: Some(bytes_written),
    }
  }
}

impl<'a> BytestreamTestStep for ActionWrite<'a> {
  const MSG: &'static str = "Action";

  fn execute(
    &self,
    bytestream: &mut ByteStream,
  ) -> Result<(), Box<dyn Error>> {
    let actual_bw = bytestream.write(self.data)?;
    if let Some(expected_bw) = self.bytes_written {
      if expected_bw != actual_bw {
        return Err(Box::new(
          BytestreamExpectationViolation::with_property(
            String::from("bytes_written"),
            &expected_bw,
            &actual_bw,
          ),
        ));
      }
    }
    Ok(())
  }

  fn description(&self) -> String {
    format!(
      "write {} to the stream",
      String::from_utf8(self.data.to_vec()).unwrap()
    )
  }
}

pub struct ActionPop {
  len: usize,
}
impl ActionPop {
  pub fn new(len: usize) -> ActionPop {
    ActionPop { len }
  }
}
impl BytestreamTestStep for ActionPop {
  const MSG: &'static str = "Action";

  fn execute(
    &self,
    bytestream: &mut ByteStream,
  ) -> Result<(), Box<dyn Error>> {
    bytestream.pop_output(self.len);
    Ok(())
  }

  fn description(&self) -> String {
    format!("pop {}", self.len)
  }
}

pub struct ExpectInputEnded {
  input_ended: bool,
}

impl ExpectInputEnded {
  pub fn new(input_ended: bool) -> ExpectInputEnded {
    ExpectInputEnded { input_ended }
  }
}

impl BytestreamTestStep for ExpectInputEnded {
  const MSG: &'static str = "Expectation";

  fn execute(
    &self,
    bytestream: &mut ByteStream,
  ) -> Result<(), Box<dyn Error>> {
    let actual_ie = bytestream.input_ended();
    if actual_ie != self.input_ended {
      Err(Box::new(BytestreamExpectationViolation::with_property(
        String::from("input_ended"),
        &self.input_ended,
        &actual_ie,
      )))
    } else {
      Ok(())
    }
  }

  fn description(&self) -> String {
    format!("input_ended: {}", self.input_ended)
  }
}

pub struct ExpectBufferEmpty {
  buffer_empty: bool,
}

impl ExpectBufferEmpty {
  pub fn new(buffer_empty: bool) -> ExpectBufferEmpty {
    ExpectBufferEmpty { buffer_empty }
  }
}

impl BytestreamTestStep for ExpectBufferEmpty {
  const MSG: &'static str = "Expectation";

  fn execute(
    &self,
    bytestream: &mut ByteStream,
  ) -> Result<(), Box<dyn Error>> {
    let actual_be = bytestream.buffer_empty();
    if actual_be != self.buffer_empty {
      Err(Box::new(BytestreamExpectationViolation::with_property(
        String::from("buffer_empty"),
        &self.buffer_empty,
        &actual_be,
      )))
    } else {
      Ok(())
    }
  }

  fn description(&self) -> String {
    format!("buffer_empty: {}", self.buffer_empty)
  }
}

pub struct ExpectEof {
  eof: bool,
}

impl ExpectEof {
  pub fn new(eof: bool) -> ExpectEof {
    ExpectEof { eof }
  }
}

impl BytestreamTestStep for ExpectEof {
  const MSG: &'static str = "Expectation";

  fn execute(
    &self,
    bytestream: &mut ByteStream,
  ) -> Result<(), Box<dyn Error>> {
    let actual_eof = bytestream.eof();
    if actual_eof != self.eof {
      Err(Box::new(BytestreamExpectationViolation::with_property(
        String::from("eof"),
        &self.eof,
        &actual_eof,
      )))
    } else {
      Ok(())
    }
  }

  fn description(&self) -> String {
    format!("eof: {}", self.eof)
  }
}

pub struct ExpectBufferSize {
  buffer_size: usize,
}

impl ExpectBufferSize {
  pub fn new(buffer_size: usize) -> ExpectBufferSize {
    ExpectBufferSize { buffer_size }
  }
}

impl BytestreamTestStep for ExpectBufferSize {
  const MSG: &'static str = "Expectation";

  fn execute(
    &self,
    bytestream: &mut ByteStream,
  ) -> Result<(), Box<dyn Error>> {
    let actual_bs = bytestream.buffer_size();
    if actual_bs != self.buffer_size {
      Err(Box::new(BytestreamExpectationViolation::with_property(
        String::from("buffer_size"),
        &self.buffer_size,
        &actual_bs,
      )))
    } else {
      Ok(())
    }
  }

  fn description(&self) -> String {
    format!("buffer_size: {}", self.buffer_size)
  }
}

pub struct ExpectBytesWritten {
  pub bytes_written: usize,
}

impl ExpectBytesWritten {
  pub fn new(bytes_written: usize) -> ExpectBytesWritten {
    ExpectBytesWritten { bytes_written }
  }
}

impl BytestreamTestStep for ExpectBytesWritten {
  const MSG: &'static str = "Expectation";

  fn execute(
    &self,
    bytestream: &mut ByteStream,
  ) -> Result<(), Box<dyn Error>> {
    let actual_bw = bytestream.bytes_written();
    if actual_bw != self.bytes_written {
      Err(Box::new(BytestreamExpectationViolation::with_property(
        String::from("bytes_written"),
        &self.bytes_written,
        &actual_bw,
      )))
    } else {
      Ok(())
    }
  }

  fn description(&self) -> String {
    format!("bytes_written: {}", self.bytes_written)
  }
}

pub struct ExpectBytesRead {
  pub bytes_read: usize,
}

impl ExpectBytesRead {
  pub fn new(bytes_read: usize) -> ExpectBytesRead {
    ExpectBytesRead { bytes_read }
  }
}

impl BytestreamTestStep for ExpectBytesRead {
  const MSG: &'static str = "Expectation";

  fn execute(
    &self,
    bytestream: &mut ByteStream,
  ) -> Result<(), Box<dyn Error>> {
    let actual_br = bytestream.bytes_read();
    if actual_br != self.bytes_read {
      Err(Box::new(BytestreamExpectationViolation::with_property(
        String::from("bytes_read"),
        &self.bytes_read,
        &actual_br,
      )))
    } else {
      Ok(())
    }
  }

  fn description(&self) -> String {
    format!("bytes_read: {}", self.bytes_read)
  }
}

pub struct ExpectRemainingCapacity {
  pub remaining_capacity: usize,
}

impl ExpectRemainingCapacity {
  pub fn new(remaining_capacity: usize) -> ExpectRemainingCapacity {
    ExpectRemainingCapacity { remaining_capacity }
  }
}

impl BytestreamTestStep for ExpectRemainingCapacity {
  const MSG: &'static str = "Expectation";

  fn execute(
    &self,
    bytestream: &mut ByteStream,
  ) -> Result<(), Box<dyn Error>> {
    let actual_rc = bytestream.remaining_capacity();
    if actual_rc != self.remaining_capacity {
      Err(Box::new(BytestreamExpectationViolation::with_property(
        String::from("remaining_capactiy"),
        &self.remaining_capacity,
        &actual_rc,
      )))
    } else {
      Ok(())
    }
  }

  fn description(&self) -> String {
    format!("remaining_capacity: {}", self.remaining_capacity)
  }
}

pub struct ExpectPeek<'a> {
  pub output: &'a [u8],
}

impl<'a> ExpectPeek<'a> {
  pub fn new(output: &'a [u8]) -> ExpectPeek {
    ExpectPeek { output }
  }
}

impl<'a> BytestreamTestStep for ExpectPeek<'a> {
  const MSG: &'static str = "Expectation";

  fn execute(
    &self,
    bytestream: &mut ByteStream,
  ) -> Result<(), Box<dyn Error>> {
    let actual_o = bytestream.peek_output(self.output.len());
    if actual_o.as_slice() != self.output {
      Err(Box::new(BytestreamExpectationViolation::new(format!(
        "Expected \"{}\" at the front of  the stream, \
      buf found \"{}\"",
        String::from_utf8(self.output.to_vec()).unwrap(),
        String::from_utf8(actual_o).unwrap(),
      ))))
    } else {
      Ok(())
    }
  }

  fn description(&self) -> String {
    format!(
      "\"{}\" at the front of the stream",
      String::from_utf8(self.output.to_vec()).unwrap()
    )
  }
}

pub struct ByteStreamTestHarness {
  bytes_stream: ByteStream,
  steps_executed: Vec<String>,
}

impl ByteStreamTestHarness {
  pub fn with_capacity(capacity: usize) -> ByteStreamTestHarness {
    ByteStreamTestHarness {
      bytes_stream: ByteStream::new(capacity),
      steps_executed: vec![format!(
        "Initialized with (capacity={})",
        capacity
      )],
    }
  }
  pub fn execute<T>(&mut self, step: T)
  where
    T: BytestreamTestStep,
  {
    if let Err(e) = step.execute(&mut self.bytes_stream) {
      let mut em = format!(
        "Thest Failure on expectation:\n\t{}",
        step.to_string()
      );
      em += &format!("\nFailure message:\n\t{}", e);
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
