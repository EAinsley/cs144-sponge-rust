use super::tcp_receiver_harness::*;
use crate::tcp_helpers::TCPReceiverSummary;
use crate::WrappingInt32;
#[test]
fn connect_0() {
  let mut test = TCPReceiverTestHarness::with_capacity(4000);
  test.execute(ExpectWindow::new(4000));
  test.execute(ExpectAckno::new(None));
  test.execute(ExpectUnassembledBytes::new(0));
  test.execute(ExpectTotalAssembledBytes::new(0));
  test.execute(
    SegmentArrives::new()
      .with_syn()
      .with_seqno_u32(0)
      .with_result(SegmentResult::OK),
  );
  test.execute(ExpectAckno::new(Some(WrappingInt32::new(1))));
  test.execute(ExpectUnassembledBytes::new(0));
  test.execute(ExpectTotalAssembledBytes::new(0))
}

#[test]
fn connect_1() {
  let mut test = TCPReceiverTestHarness::with_capacity(5435);
  test.execute(ExpectAckno::new(None));
  test.execute(ExpectUnassembledBytes::new(0));
  test.execute(ExpectTotalAssembledBytes::new(0));
  test.execute(
    SegmentArrives::new()
      .with_syn()
      .with_seqno_u32(89347598)
      .with_result(SegmentResult::OK),
  );
  test.execute(ExpectAckno::new(Some(WrappingInt32::new(89347599))));
  test.execute(ExpectUnassembledBytes::new(0));
  test.execute(ExpectTotalAssembledBytes::new(0));
}

#[test]
fn connect_2() {
  let mut test = TCPReceiverTestHarness::with_capacity(5435);
  test.execute(ExpectAckno::new(None));
  test.execute(ExpectUnassembledBytes::new(0));
  test.execute(ExpectTotalAssembledBytes::new(0));
  test.execute(
    SegmentArrives::new()
      .with_seqno_u32(893475)
      .with_result(SegmentResult::NOT_SYN),
  );
  test.execute(ExpectAckno::new(None));
  test.execute(ExpectUnassembledBytes::new(0));
  test.execute(ExpectTotalAssembledBytes::new(0));
}
#[test]
fn connect_3() {
  let mut test = TCPReceiverTestHarness::with_capacity(5435);
  test.execute(ExpectAckno::new(None));
  test.execute(ExpectUnassembledBytes::new(0));
  test.execute(ExpectTotalAssembledBytes::new(0));
  test.execute(
    SegmentArrives::new()
      .with_ack_u32(0)
      .with_fin()
      .with_seqno_u32(893475)
      .with_result(SegmentResult::NOT_SYN),
  );
  test.execute(ExpectAckno::new(None));
  test.execute(ExpectUnassembledBytes::new(0));
  test.execute(ExpectTotalAssembledBytes::new(0));
}

#[test]
fn connect_4() {
  let mut test = TCPReceiverTestHarness::with_capacity(5435);
  test.execute(ExpectAckno::new(None));
  test.execute(ExpectUnassembledBytes::new(0));
  test.execute(ExpectTotalAssembledBytes::new(0));
  test.execute(
    SegmentArrives::new()
      .with_ack_u32(0)
      .with_fin()
      .with_seqno_u32(893475)
      .with_result(SegmentResult::NOT_SYN),
  );
  test.execute(ExpectAckno::new(None));
  test.execute(ExpectUnassembledBytes::new(0));
  test.execute(ExpectTotalAssembledBytes::new(0));
  test.execute(
    SegmentArrives::new()
      .with_syn()
      .with_seqno_u32(89347598)
      .with_result(SegmentResult::OK),
  );
  test.execute(ExpectAckno::new(Some(WrappingInt32::new(89347599))));
  test.execute(ExpectUnassembledBytes::new(0));
  test.execute(ExpectTotalAssembledBytes::new(0));
}

// #[test]
// fn connect_5() {
//   let mut test = TCPReceiverTestHarness::with_capacity(4000);
//   test.execute(
//     SegmentArrives::new()
//       .with_syn()
//       .with_seqno_u32(5)
//       .with_fin()
//       .with_result(SegmentResult::OK),
//   );
//   test.execute(ExpectState::new(TCPReceiverSummary::FIN_RECV));
//   test.execute(ExpectAckno::new(Some(WrappingInt32::new(7))));
//   test.execute(ExpectUnassembledBytes::new(0));
//   test.execute(ExpectTotalAssembledBytes::new(0));
// }
