use super::super::*;

#[test]
fn single_0() {
  let mut sr = StreamReassembler::new(65000);

  assert_eq!(sr.as_stream().bytes_written(), 0);
  assert!(sr.as_stream().buffer_empty());
  assert_eq!(sr.as_mut_stream().read(0), "");
  assert!(!sr.as_stream().eof());
}

#[test]
fn single_1() {
  let mut sr = StreamReassembler::new(65000);

  sr.push_substring("a", 0, false);

  assert_eq!(sr.as_stream().bytes_written(), 1);
  assert_eq!(sr.as_stream().buffer_size(), 1);
  assert_eq!(sr.as_mut_stream().read(1), "a");
  assert!(!sr.as_stream().eof());
}

#[test]
fn single_2() {
  let mut sr = StreamReassembler::new(65000);

  sr.push_substring("a", 0, true);

  assert_eq!(sr.as_stream().bytes_written(), 1);
  assert_eq!(sr.as_stream().buffer_size(), 1);
  assert_eq!(sr.as_mut_stream().read(1), "a");
  assert!(sr.as_stream().eof());
}

#[test]
fn single_3() {
  let mut sr = StreamReassembler::new(65000);

  sr.push_substring("", 0, true);

  assert_eq!(sr.as_stream().bytes_written(), 0);
  assert_eq!(sr.as_stream().buffer_size(), 0);
  assert_eq!(sr.as_mut_stream().read(0), "");
  assert!(sr.as_stream().eof());
}

#[test]
fn single_4() {
  let mut sr = StreamReassembler::new(65000);

  sr.push_substring("b", 0, true);

  assert_eq!(sr.as_stream().bytes_written(), 1);
  assert_eq!(sr.as_stream().buffer_size(), 1);
  assert_eq!(sr.as_mut_stream().read(1), "b");
  assert!(sr.as_stream().eof());
}

#[test]
fn signle_5() {
  let mut sr = StreamReassembler::new(65000);

  sr.push_substring("", 0, false);

  assert_eq!(sr.as_stream().bytes_written(), 0);
  assert_eq!(sr.as_stream().buffer_size(), 0);
  assert_eq!(sr.as_mut_stream().read(0), "");
  assert!(!sr.as_stream().eof());
}

#[test]
fn signle_6() {
  let mut sr = StreamReassembler::new(8);

  sr.push_substring("abcdefgh", 0, false);

  assert_eq!(sr.as_stream().bytes_written(), 8);
  assert_eq!(sr.as_stream().buffer_size(), 8);
  assert_eq!(sr.as_mut_stream().read(8), "abcdefgh");
  assert!(!sr.as_stream().eof());
}

#[test]
fn signle_7() {
  let mut sr = StreamReassembler::new(8);

  sr.push_substring("abcdefgh", 0, true);

  assert_eq!(sr.as_stream().bytes_written(), 8);
  assert_eq!(sr.as_stream().buffer_size(), 8);
  assert_eq!(sr.as_mut_stream().read(8), "abcdefgh");
  assert!(sr.as_stream().eof());
}

#[test]
fn signle_8() {
  let mut sr = StreamReassembler::new(8);

  sr.push_substring("abc", 0, false);
  assert_eq!(sr.as_stream().bytes_written(), 3);

  sr.push_substring("bcdefgh", 1, true);

  assert_eq!(sr.as_stream().bytes_written(), 8);
  assert_eq!(sr.as_stream().buffer_size(), 8);
  assert_eq!(sr.as_mut_stream().read(8), "abcdefgh");
  assert!(sr.as_stream().eof());
}

#[test]
fn signle_9() {
  let mut sr = StreamReassembler::new(8);

  sr.push_substring("abc", 0, false);
  assert_eq!(sr.as_stream().bytes_written(), 3);
  assert!(!sr.as_stream().eof());

  sr.push_substring("ghX", 6, true);
  assert_eq!(sr.as_stream().bytes_written(), 3);
  assert!(!sr.as_stream().eof());

  sr.push_substring("cdefg", 2, false);
  assert_eq!(sr.as_stream().bytes_written(), 8);
  assert_eq!(sr.as_stream().buffer_size(), 8);
  assert_eq!(sr.as_mut_stream().read(8), "abcdefgh");
  assert!(!sr.as_stream().eof());
}

#[test]
fn signle_10() {
  let mut sr = StreamReassembler::new(8);

  sr.push_substring("abc", 0, false);
  assert_eq!(sr.as_stream().bytes_written(), 3);
  assert!(!sr.as_stream().eof());

  sr.push_substring("", 6, false);
  assert_eq!(sr.as_stream().bytes_written(), 3);
  assert!(!sr.as_stream().eof());

  sr.push_substring("de", 3, true);
  assert_eq!(sr.as_stream().bytes_written(), 5);
  assert_eq!(sr.as_stream().buffer_size(), 5);
  assert_eq!(sr.as_mut_stream().read(5), "abcde");
  assert!(sr.as_stream().eof());
}
