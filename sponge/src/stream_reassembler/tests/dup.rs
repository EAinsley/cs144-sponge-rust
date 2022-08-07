use super::super::*;
use rand::prelude::*;

#[test]
fn dup_0() {
  let mut sr = StreamReassembler::new(65000);

  sr.push_substring("abcd", 0, false);
  assert_eq!(sr.as_stream().bytes_written(), 4);
  assert_eq!(sr.as_stream().buffer_size(), 4);
  assert_eq!(sr.as_mut_stream().read(4), "abcd");
  assert!(!sr.as_stream().eof());

  sr.push_substring("abcd", 0, false);
  assert_eq!(sr.as_stream().bytes_written(), 4);
  assert_eq!(sr.as_stream().buffer_size(), 0);
  assert_eq!(sr.as_mut_stream().read(0), "");
  assert!(!sr.as_stream().eof());
}

#[test]
fn dup_1() {
  let mut sr = StreamReassembler::new(65000);

  sr.push_substring("abcd", 0, false);
  assert_eq!(sr.as_stream().bytes_written(), 4);
  assert_eq!(sr.as_stream().buffer_size(), 4);
  assert_eq!(sr.as_mut_stream().read(4), "abcd");
  assert!(!sr.as_stream().eof());

  sr.push_substring("abcd", 4, false);
  assert_eq!(sr.as_stream().bytes_written(), 8);
  assert_eq!(sr.as_stream().buffer_size(), 4);
  assert_eq!(sr.as_mut_stream().read(4), "abcd");
  assert!(!sr.as_stream().eof());

  sr.push_substring("abcd", 0, false);
  assert_eq!(sr.as_stream().bytes_written(), 8);
  assert_eq!(sr.as_stream().buffer_size(), 0);
  assert_eq!(sr.as_mut_stream().read(0), "");
  assert!(!sr.as_stream().eof());

  sr.push_substring("abcd", 4, false);
  assert_eq!(sr.as_stream().bytes_written(), 8);
  assert_eq!(sr.as_stream().buffer_size(), 0);
  assert_eq!(sr.as_mut_stream().read(0), "");
  assert!(!sr.as_stream().eof());
}

#[test]
fn dup_2() {
  let mut rd = thread_rng();
  let mut sr = StreamReassembler::new(65000);

  sr.push_substring("abcdefgh", 0, false);
  assert_eq!(sr.as_stream().bytes_written(), 8);
  assert_eq!(sr.as_stream().buffer_size(), 8);
  assert_eq!(sr.as_mut_stream().read(8), "abcdefgh");
  assert!(!sr.as_stream().eof());

  let data = "abcdefgh";

  for _ in 0..1000 {
    let start_i = rd.gen_range(0..8);
    let end_i = rd.gen_range(start_i..8);
    sr.push_substring(&data[start_i..=end_i], start_i, false);

    assert_eq!(sr.as_stream().bytes_written(), 8);
    assert_eq!(sr.as_stream().buffer_size(), 0);
    assert_eq!(sr.as_mut_stream().read(0), "");
    assert!(!sr.as_stream().eof());
  }
}

#[test]
fn dup_3() {
  let mut sr = StreamReassembler::new(65000);

  sr.push_substring("abcd", 0, false);
  assert_eq!(sr.as_stream().bytes_written(), 4);
  assert_eq!(sr.as_stream().buffer_size(), 4);
  assert_eq!(sr.as_mut_stream().read(4), "abcd");
  assert!(!sr.as_stream().eof());

  sr.push_substring("abcdef", 0, false);
  assert_eq!(sr.as_stream().bytes_written(), 6);
  assert_eq!(sr.as_stream().buffer_size(), 2);
  assert_eq!(sr.as_mut_stream().read(2), "ef");
  assert!(!sr.as_stream().eof());
}
