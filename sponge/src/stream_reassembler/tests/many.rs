use super::super::*;
use rand::{
  distributions::Alphanumeric, prelude::*, seq::SliceRandom,
};

const NREPS: usize = 32;
const NSEGS: usize = 128;
const MAX_SEG_LEN: usize = 2048;

#[test]
fn many_0() {
  let mut rd = thread_rng();

  for _ in 0..NREPS {
    let mut sr = StreamReassembler::new(65000);
    let mut seq_size = [(0, 0); NSEGS];
    let mut offset = 0;
    for i in 0..NSEGS {
      let size = rd.gen_range(1..=MAX_SEG_LEN);
      seq_size[i] = (offset, size);
      offset += size;
    }
    seq_size.shuffle(&mut rd);
    let data: String = (&mut rd)
      .sample_iter(Alphanumeric)
      .take(offset)
      .map(char::from)
      .collect();

    for (off, sz) in seq_size {
      sr.push_substring(
        &data[off..off + sz],
        off,
        off + sz == offset,
      );
    }
    let buf_size = sr.as_stream().buffer_size();
    let result = sr.as_mut_stream().read(buf_size);

    assert_eq!(sr.as_stream().bytes_written(), offset);
    assert_eq!(buf_size, offset);
    assert_eq!(result, data);
  }
}

#[test]
fn many_1() {
  let mut rd = thread_rng();
  for _ in 0..NREPS {
    let mut sr = StreamReassembler::new(65000);
    const SIZE: usize = 1024;
    let data: String = (&mut rd)
      .sample_iter(Alphanumeric)
      .take(SIZE)
      .map(char::from)
      .collect();

    sr.push_substring(&data, 0, false);
    sr.push_substring(&data[10..], SIZE + 10, false);

    let sr_size = sr.as_stream().buffer_size();
    let res = sr.as_mut_stream().read(sr_size);

    assert_eq!(sr.as_stream().bytes_written(), SIZE);
    assert_eq!(sr_size, SIZE);
    assert_eq!(res, data);

    sr.push_substring(&data[0..8], SIZE, true);

    let sr_size = sr.as_stream().buffer_size();
    let res = sr.as_mut_stream().read(sr_size);

    assert_eq!(sr.as_stream().bytes_written(), SIZE + 8);
    assert_eq!(sr_size, 8);
    assert_eq!(res, data[0..8]);
  }
}

#[test]
fn many_2() {
  let mut rd = thread_rng();

  for _ in 0..NREPS {
    let mut sr = StreamReassembler::new(65000);

    const SIZE: usize = 1024;
    let data: String = (&mut rd)
      .sample_iter(Alphanumeric)
      .take(SIZE)
      .map(char::from)
      .collect();

    sr.push_substring(&data, 0, false);
    sr.push_substring(&data[10..], SIZE + 10, false);

    let sr_size = sr.as_stream().buffer_size();
    let res = sr.as_mut_stream().read(sr_size);

    assert_eq!(sr.as_stream().bytes_written(), SIZE);
    assert_eq!(sr_size, SIZE);
    assert_eq!(res, data);

    sr.push_substring(&data[..15], SIZE, true);

    let sr_size = sr.as_mut_stream().buffer_size();
    let res = sr.as_mut_stream().read(sr_size);

    assert_eq!(sr.as_stream().bytes_written(), SIZE + 15);
    assert_eq!(sr_size, 15);
    assert_eq!(res, data[..15]);
  }
}
