use super::super::*;

#[test]
fn seq_1() {
  let mut sr = StreamReassembler::new(65000);

  sr.push_substring("abcd", 0, false);
  assert_eq!(sr.as_stream().bytes_written(), 4);
  assert_eq!(sr.as_stream().buffer_size(), 4);
  assert_eq!(sr.as_mut_stream().read(4), "abcd");
  assert!(!sr.as_stream().eof());

  sr.push_substring("efgh", 4, false);
  assert_eq!(sr.as_stream().bytes_written(), 8);
  assert_eq!(sr.as_stream().buffer_size(), 4);
  assert_eq!(sr.as_mut_stream().read(4), "efgh");
  assert!(!sr.as_stream().eof());
}

#[test]
fn seq_2() {
  let mut sr = StreamReassembler::new(65000);

  for i in 0..100 {
    assert_eq!(sr.as_stream().bytes_written(), i * 4);
    sr.push_substring("abcd", i * 4, false);
    assert!(!sr.as_stream().eof());
  }

  let ss = "abcd".repeat(100);
  assert_eq!(sr.as_stream().buffer_size(), ss.len());
  assert_eq!(sr.as_mut_stream().read(ss.len()), ss);
  assert!(!sr.as_stream().eof());
}

#[test]
fn seq_3() {
  let mut sr = StreamReassembler::new(65000);
  for i in 0..100 {
    assert_eq!(sr.as_stream().bytes_written(), i * 4);
    sr.push_substring("abcd", i * 4, false);
    assert!(!sr.as_stream().eof());

    assert_eq!(sr.as_stream().buffer_size(), 4);
    assert_eq!(sr.as_mut_stream().read(4), "abcd");
  }
}
