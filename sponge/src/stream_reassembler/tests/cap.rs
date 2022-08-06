use super::super::*;

#[test]
fn cap_0() {
  let mut sr = StreamReassembler::new(2);

  sr.push_substring("ab", 0, false);
  assert_eq!(sr.as_stream().bytes_written(), 2);
  assert_eq!(sr.as_stream().buffer_size(), 2);
  assert_eq!(sr.as_mut_stream().read(2), "ab");

  sr.push_substring("cd", 2, false);
  assert_eq!(sr.as_stream().bytes_written(), 4);
  assert_eq!(sr.as_stream().buffer_size(), 2);
  assert_eq!(sr.as_mut_stream().read(2), "cd");

  sr.push_substring("ef", 4, false);
  assert_eq!(sr.as_stream().bytes_written(), 6);
  assert_eq!(sr.as_stream().buffer_size(), 2);
  assert_eq!(sr.as_mut_stream().read(2), "ef");
}

#[test]
fn cap_1() {
  let mut sr = StreamReassembler::new(2);

  sr.push_substring("ab", 0, false);
  assert_eq!(sr.as_stream().bytes_written(), 2);

  sr.push_substring("cd", 2, false);
  assert_eq!(sr.as_stream().bytes_written(), 2);

  assert_eq!(sr.as_stream().buffer_size(), 2);
  assert_eq!(sr.as_mut_stream().read(2), "ab");
  assert_eq!(sr.as_stream().bytes_written(), 2);

  sr.push_substring("cd", 2, false);
  assert_eq!(sr.as_stream().bytes_written(), 4);

  assert_eq!(sr.as_stream().buffer_size(), 2);
  assert_eq!(sr.as_mut_stream().read(2), "cd");
}

#[test]
fn cap_2() {
  let mut sr = StreamReassembler::new(2);

  sr.push_substring("bX", 1, false);
  assert_eq!(sr.as_stream().bytes_written(), 0);

  sr.push_substring("a", 0, false);
  assert_eq!(sr.as_stream().bytes_written(), 2);

  assert_eq!(sr.as_stream().buffer_size(), 2);
  assert_eq!(sr.as_mut_stream().read(2), "ab");
}

#[test]
fn cap_3() {
  let mut sr = StreamReassembler::new(1);

  sr.push_substring("ab", 0, false);
  assert_eq!(sr.as_stream().bytes_written(), 1);

  sr.push_substring("ab", 0, false);
  assert_eq!(sr.as_stream().bytes_written(), 1);

  assert_eq!(sr.as_stream().buffer_size(), 1);
  assert_eq!(sr.as_mut_stream().read(1), "a");
  assert_eq!(sr.as_stream().bytes_written(), 1);

  sr.push_substring("abc", 0, false);
  assert_eq!(sr.as_stream().bytes_written(), 2);

  assert_eq!(sr.as_stream().buffer_size(), 1);
  assert_eq!(sr.as_mut_stream().read(2), "b");
  assert_eq!(sr.as_stream().bytes_written(), 2);
}

#[test]
fn cap_4() {
  let mut sr = StreamReassembler::new(8);

  sr.push_substring("a", 0, false);
  assert_eq!(sr.as_stream().bytes_written(), 1);
  assert_eq!(sr.as_stream().buffer_size(), 1);
  assert_eq!(sr.as_mut_stream().read(1), "a");
  assert!(!sr.as_stream().eof());

  sr.push_substring("bc", 1, false);
  assert_eq!(sr.as_stream().bytes_written(), 3);
  assert!(!sr.as_stream().eof());

  sr.push_substring("ghi", 6, true);
  assert_eq!(sr.as_stream().bytes_written(), 3);
  assert!(!sr.as_stream().eof());

  sr.push_substring("cdefg", 2, false);
  assert_eq!(sr.as_stream().bytes_written(), 9);
  assert_eq!(sr.as_stream().buffer_size(), 8);
  assert_eq!(sr.as_mut_stream().read(8), "bcdefghi");
  assert!(sr.as_stream().eof());
}

#[test]
fn cap_5() {
  let mut sr = StreamReassembler::new(3);

  for i in (0..).map(|i| i * 3).take_while(|&i| i < 99997) {
    sr.push_substring("abcXXX", i, false);
    assert_eq!(sr.as_stream().bytes_written(), i + 3);
    assert_eq!(sr.as_stream().buffer_size(), 3);
    assert_eq!(sr.as_mut_stream().read(3), "abc");
  }
}
