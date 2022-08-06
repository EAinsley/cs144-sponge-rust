use super::super::*;

#[test]
fn overlapping_0() {
  let mut sr = StreamReassembler::new(1000);

  sr.push_substring("a", 0, false);
  sr.push_substring("ab", 0, false);

  assert_eq!(sr.as_stream().bytes_written(), 2);
  assert_eq!(sr.as_stream().buffer_size(), 2);
  assert_eq!(sr.as_mut_stream().read(2), "ab");
}

#[test]
fn overlapping_1() {
  let mut sr = StreamReassembler::new(1000);

  sr.push_substring("a", 0, false);
  assert_eq!(sr.as_stream().buffer_size(), 1);
  assert_eq!(sr.as_mut_stream().read(1), "a");

  sr.push_substring("ab", 0, false);
  assert_eq!(sr.as_stream().buffer_size(), 1);
  assert_eq!(sr.as_mut_stream().read(1), "b");
  assert_eq!(sr.as_stream().bytes_written(), 2);
}

#[test]
fn overlapping_2() {
  let mut sr = StreamReassembler::new(1000);

  sr.push_substring("b", 1, false);
  assert_eq!(sr.as_stream().buffer_size(), 0);
  assert_eq!(sr.as_mut_stream().read(0), "");

  sr.push_substring("ab", 0, false);
  assert_eq!(sr.as_stream().buffer_size(), 2);
  assert_eq!(sr.as_mut_stream().read(2), "ab");
  assert_eq!(sr.unassembled_bytes(), 0);
  assert_eq!(sr.as_stream().bytes_written(), 2);
}

#[test]
fn overlapping_3() {
  let mut sr = StreamReassembler::new(1000);

  sr.push_substring("b", 1, false);
  assert_eq!(sr.as_stream().buffer_size(), 0);
  assert_eq!(sr.as_mut_stream().read(0), "");

  sr.push_substring("bc", 1, false);
  assert_eq!(sr.as_stream().buffer_size(), 0);
  assert_eq!(sr.as_mut_stream().read(0), "");
  assert_eq!(sr.unassembled_bytes(), 2);
  assert_eq!(sr.as_stream().bytes_written(), 0);
}

#[test]
fn overlapping_4() {
  let mut sr = StreamReassembler::new(1000);

  sr.push_substring("c", 2, false);
  assert_eq!(sr.as_stream().buffer_size(), 0);
  assert_eq!(sr.as_mut_stream().read(0), "");

  sr.push_substring("bcd", 1, false);
  assert_eq!(sr.as_stream().buffer_size(), 0);
  assert_eq!(sr.as_mut_stream().read(0), "");
  assert_eq!(sr.unassembled_bytes(), 3);
  assert_eq!(sr.as_stream().bytes_written(), 0);
}

#[test]
fn overlapping_5() {
  let mut sr = StreamReassembler::new(1000);

  sr.push_substring("b", 1, false);
  sr.push_substring("d", 3, false);
  assert_eq!(sr.as_stream().buffer_size(), 0);
  assert_eq!(sr.as_mut_stream().read(0), "");

  sr.push_substring("bcde", 1, false);
  assert_eq!(sr.as_stream().buffer_size(), 0);
  assert_eq!(sr.as_mut_stream().read(0), "");
  assert_eq!(sr.unassembled_bytes(), 4);
  assert_eq!(sr.as_stream().bytes_written(), 0);
}

#[test]
fn overlapping_6() {
  let mut sr = StreamReassembler::new(1000);

  sr.push_substring("c", 2, false);
  sr.push_substring("bcd", 1, false);

  assert_eq!(sr.as_stream().buffer_size(), 0);
  assert_eq!(sr.as_mut_stream().read(0), "");
  assert_eq!(sr.as_stream().bytes_written(), 0);
  assert_eq!(sr.unassembled_bytes(), 3);

  sr.push_substring("a", 0, false);
  assert_eq!(sr.as_stream().buffer_size(), 4);
  assert_eq!(sr.as_mut_stream().read(4), "abcd");
  assert_eq!(sr.unassembled_bytes(), 0);
  assert_eq!(sr.as_stream().bytes_written(), 4);
}

#[test]
fn overlapping_7() {
  let mut sr = StreamReassembler::new(1000);

  sr.push_substring("bcd", 1, false);
  sr.push_substring("c", 2, false);

  assert_eq!(sr.as_stream().buffer_size(), 0);
  assert_eq!(sr.as_mut_stream().read(0), "");
  assert_eq!(sr.as_stream().bytes_written(), 0);
  assert_eq!(sr.unassembled_bytes(), 3);

  sr.push_substring("a", 0, false);
  assert_eq!(sr.as_stream().buffer_size(), 4);
  assert_eq!(sr.as_mut_stream().read(4), "abcd");
  assert_eq!(sr.unassembled_bytes(), 0);
  assert_eq!(sr.as_stream().bytes_written(), 4);
}
