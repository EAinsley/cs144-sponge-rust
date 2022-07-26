use crate::TCPReceiver;

pub struct TCPReceiverSummary;
#[allow(dead_code)]
impl TCPReceiverSummary {
  pub const ERROR: &'static str = "error (connection was reset)";
  pub const LISTEN: &'static str = "waiting for SYN: ackno is empty";
  pub const SYN_RECV: &'static str =
    "SYN received (ackno exists), and input to stream hasn't ended";
  pub const FIN_RECV: &'static str = "input to stream has ended";
}

#[allow(dead_code)]
pub struct TCPState;
#[allow(dead_code)]
impl TCPState {
  pub fn state_summary(receiver: &TCPReceiver) -> &'static str {
    if receiver.as_stream().error() {
      TCPReceiverSummary::ERROR
    } else if receiver.ackno() == None {
      TCPReceiverSummary::LISTEN
    } else if receiver.as_stream().input_ended() {
      TCPReceiverSummary::FIN_RECV
    } else {
      TCPReceiverSummary::SYN_RECV
    }
  }
}
