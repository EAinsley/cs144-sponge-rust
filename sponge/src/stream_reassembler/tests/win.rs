use super::super::*;
use rand::{
  distributions::Alphanumeric, prelude::*, seq::SliceRandom,
};

const NREPS: usize = 32;
const NSEGS: usize = 128;
const MAX_SEG_LEN: usize = 2048;

#[test]
fn win_0() {
  let mut rd = thread_rng();

  for _ in 0..NREPS {
    let mut sr = StreamReassembler::new(NSEGS * MAX_SEG_LEN);

    let mut seq_size = [(0, 0); NSEGS];
    let mut offset = 0;

    for i in 0..NSEGS {
      let size = rd.gen_range(1..MAX_SEG_LEN);
      let offs = offset.min(rd.gen_range(1..1024));
      seq_size[i] = (offset - offs, size + offs);
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

      let sr_size = sr.as_stream().buffer_size();
      let res = sr.as_mut_stream().read(sr_size);

      assert_eq!(sr_size, offset);
      assert_eq!(res, data);
    }
  }
}
