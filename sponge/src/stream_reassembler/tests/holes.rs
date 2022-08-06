use super::super::*;

#[test]
fn holes_0() {
  let mut sr = StreamReassembler::new(65000);

  sr.push_substring("b", 1, false);

  assert_eq!(sr.as_stream().bytes_written(), 0);
  assert_eq!(sr.as_stream().buffer_size(), 0);
  assert_eq!(sr.as_mut_stream().read(0), "");
  assert!(!sr.as_stream().eof());
}

#[test]
fn holes_1() {
  let mut sr = StreamReassembler::new(65000);

  sr.push_substring("b", 1, false);
  sr.push_substring("a", 0, false);

  assert_eq!(sr.as_stream().bytes_written(), 2);
  assert_eq!(sr.as_stream().buffer_size(), 2);
  assert_eq!(sr.as_mut_stream().read(2), "ab");
  assert!(!sr.as_stream().eof());
}

#[test]
fn holes_2() {
  let mut sr = StreamReassembler::new(65000);

  sr.push_substring("b", 1, true);

  assert_eq!(sr.as_stream().bytes_written(), 0);
  assert_eq!(sr.as_stream().buffer_size(), 0);
  assert_eq!(sr.as_mut_stream().read(0), "");
  assert!(!sr.as_stream().eof());

  sr.push_substring("a", 0, false);

  assert_eq!(sr.as_stream().bytes_written(), 2);
  assert_eq!(sr.as_stream().buffer_size(), 2);
  assert_eq!(sr.as_mut_stream().read(2), "ab");
  assert!(sr.as_stream().eof());
}

#[test]
fn holes_3() {
  let mut sr = StreamReassembler::new(65000);

  sr.push_substring("b", 1, false);
  sr.push_substring("ab", 0, false);

  assert_eq!(sr.as_stream().bytes_written(), 2);
  assert_eq!(sr.as_stream().buffer_size(), 2);
  assert_eq!(sr.as_mut_stream().read(2), "ab");
  assert!(!sr.as_stream().eof());
}

#[test]
fn holes_4() {
  let mut sr = StreamReassembler::new(65000);

  sr.push_substring("b", 1, false);
  assert_eq!(sr.as_stream().bytes_written(), 0);
  assert_eq!(sr.as_stream().buffer_size(), 0);
  assert_eq!(sr.as_mut_stream().read(0), "");
  assert!(!sr.as_stream().eof());

  sr.push_substring("d", 3, false);
  assert_eq!(sr.as_stream().bytes_written(), 0);
  assert_eq!(sr.as_stream().buffer_size(), 0);
  assert_eq!(sr.as_mut_stream().read(0), "");
  assert!(!sr.as_stream().eof());

  sr.push_substring("c", 2, false);
  assert_eq!(sr.as_stream().bytes_written(), 0);
  assert_eq!(sr.as_stream().buffer_size(), 0);
  assert_eq!(sr.as_mut_stream().read(0), "");
  assert!(!sr.as_stream().eof());

  sr.push_substring("a", 0, false);

  assert_eq!(sr.as_stream().bytes_written(), 4);
  assert_eq!(sr.as_stream().buffer_size(), 4);
  assert_eq!(sr.as_mut_stream().read(4), "abcd");
  assert!(!sr.as_stream().eof());
}

#[test]
fn holes_5() {
  let mut sr = StreamReassembler::new(65000);

  sr.push_substring("b", 1, false);
  assert_eq!(sr.as_stream().bytes_written(), 0);
  assert_eq!(sr.as_stream().buffer_size(), 0);
  assert_eq!(sr.as_mut_stream().read(0), "");
  assert!(!sr.as_stream().eof());

  sr.push_substring("d", 3, false);
  assert_eq!(sr.as_stream().bytes_written(), 0);
  assert_eq!(sr.as_stream().buffer_size(), 0);
  assert_eq!(sr.as_mut_stream().read(0), "");
  assert!(!sr.as_stream().eof());

  sr.push_substring("abc", 0, false);

  assert_eq!(sr.as_stream().bytes_written(), 4);
  assert_eq!(sr.as_stream().buffer_size(), 4);
  assert_eq!(sr.as_mut_stream().read(4), "abcd");
  assert!(!sr.as_stream().eof());
}

#[test]
fn holes_6() {
  let mut sr = StreamReassembler::new(65000);

  sr.push_substring("b", 1, false);
  assert_eq!(sr.as_stream().bytes_written(), 0);
  assert_eq!(sr.as_stream().buffer_size(), 0);
  assert_eq!(sr.as_mut_stream().read(0), "");
  assert!(!sr.as_stream().eof());

  sr.push_substring("d", 3, false);
  assert_eq!(sr.as_stream().bytes_written(), 0);
  assert_eq!(sr.as_stream().buffer_size(), 0);
  assert_eq!(sr.as_mut_stream().read(0), "");
  assert!(!sr.as_stream().eof());

  sr.push_substring("a", 0, false);
  assert_eq!(sr.as_stream().bytes_written(), 2);
  assert_eq!(sr.as_stream().buffer_size(), 2);
  assert_eq!(sr.as_mut_stream().read(2), "ab");
  assert!(!sr.as_stream().eof());

  sr.push_substring("c", 2, false);
  assert_eq!(sr.as_stream().bytes_written(), 4);
  assert_eq!(sr.as_stream().buffer_size(), 2);
  assert_eq!(sr.as_mut_stream().read(2), "cd");
  assert!(!sr.as_stream().eof());

  sr.push_substring("", 4, true);
  assert_eq!(sr.as_stream().bytes_written(), 4);
  assert_eq!(sr.as_stream().buffer_size(), 0);
  assert_eq!(sr.as_mut_stream().read(0), "");
  assert!(sr.as_stream().eof());
}
