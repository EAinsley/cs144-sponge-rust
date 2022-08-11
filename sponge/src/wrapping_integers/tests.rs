use super::*;
use rand::prelude::*;

#[test]
fn wrapping_integers_cmp() {
  assert_eq!(WrappingInt32::new(3) != WrappingInt32::new(1), true);
  assert_eq!(WrappingInt32::new(3) == WrappingInt32::new(1), false);

  const N_REPS: u32 = 4096;

  for _ in 0..N_REPS {
    let n: u32 = thread_rng().gen();
    let diff: u8 = thread_rng().gen();
    let m = n + diff as u32;
    assert_eq!(
      WrappingInt32::new(n) == WrappingInt32::new(m),
      n == m
    );
    assert_eq!(
      WrappingInt32::new(n) != WrappingInt32::new(m),
      n != m
    );
  }
}
#[test]
fn wrapping_integers_unwrap() {
  // Unwrap the first byte after ISN
  assert_eq!(
    WrappingInt32::unwrap(
      WrappingInt32::new(1),
      WrappingInt32::new(0),
      0
    ),
    1
  );
  // Unwrap the first byte after the first wrap
  assert_eq!(
    WrappingInt32::unwrap(
      WrappingInt32::new(1),
      WrappingInt32::new(0),
      u32::MAX as u64
    ),
    (1 << 32) + 1
  );
  // Unwrap the last byte before the third wrap
  assert_eq!(
    WrappingInt32::unwrap(
      WrappingInt32::new(u32::MAX - 1),
      WrappingInt32::new(0),
      3 * (1 << 32)
    ),
    3 * (1 << 32) - 2
  );
  // Unwrap the 10th from last byte before the third wrap
  assert_eq!(
    WrappingInt32::unwrap(
      WrappingInt32::new(u32::MAX - 10),
      WrappingInt32::new(0),
      3 * (1 << 32)
    ),
    3 * (1 << 32) - 11
  );
  // Non-zero ISN
  assert_eq!(
    WrappingInt32::unwrap(
      WrappingInt32::new(u32::MAX),
      WrappingInt32::new(10),
      3 * (1 << 32)
    ),
    3 * (1 << 32) - 11
  );
  // Big unwrap
  assert_eq!(
    WrappingInt32::unwrap(
      WrappingInt32::new(u32::MAX),
      WrappingInt32::new(0),
      0
    ),
    u32::MAX as u64
  );
  // Unwrap an non-zero ISN
  assert_eq!(
    WrappingInt32::unwrap(
      WrappingInt32::new(16),
      WrappingInt32::new(16),
      0
    ),
    0
  );
  // Big unwrap with non-zero ISN
  assert_eq!(
    WrappingInt32::unwrap(
      WrappingInt32::new(15),
      WrappingInt32::new(16),
      0
    ),
    u32::MAX as u64
  );
  // Big unwrap with non-zero ISN
  assert_eq!(
    WrappingInt32::unwrap(
      WrappingInt32::new(0),
      WrappingInt32::new(i32::MAX as u32),
      0
    ),
    i32::MAX as u64 + 2
  );
  // Barely big unwrap with non-zero ISN
  assert_eq!(
    WrappingInt32::unwrap(
      WrappingInt32::new(u32::MAX),
      WrappingInt32::new(i32::MAX as u32),
      0
    ),
    1 << 31
  );
  // Nearly big unwrap with non-zero ISN
  assert_eq!(
    WrappingInt32::unwrap(
      WrappingInt32::new(u32::MAX),
      WrappingInt32::new(1 << 31),
      0
    ),
    u32::MAX as u64 >> 1
  );
}
#[test]
fn wrapping_integers_wrap() {
  assert_eq!(
    WrappingInt32::wrap(3 * (1 << 32), WrappingInt32::new(0)),
    WrappingInt32::new(0)
  );
  assert_eq!(
    WrappingInt32::wrap(3 * (1 << 32) + 17, WrappingInt32::new(15)),
    WrappingInt32::new(32)
  );
  assert_eq!(
    WrappingInt32::wrap(7 * (1 << 32) - 2, WrappingInt32::new(15)),
    WrappingInt32::new(13)
  );
}
#[test]
fn wrapping_integers_roundtrip() {
  let mut rd = thread_rng();
  const BIG_OFFSET: u64 = (1u64 << 31) - 1;
  for _ in 1..1_000_000 {
    let isn = WrappingInt32::new(rd.gen());
    let val: u64 = rd.gen_range(0..(1 << 63));
    let offset: u64 = rd.gen_range(0..((1 << 31) - 1));
    check_roundtrip(isn, val, val);
    check_roundtrip(isn, val + 1, val);
    check_roundtrip(isn, val - 1, val);
    check_roundtrip(isn, val + offset, val);
    check_roundtrip(isn, val - offset, val);
    check_roundtrip(isn, val + BIG_OFFSET, val);
    check_roundtrip(isn, val - BIG_OFFSET, val);
  }
}

fn check_roundtrip(isn: WrappingInt32, value: u64, checkpoint: u64) {
  assert_eq!(
    WrappingInt32::unwrap(
      WrappingInt32::wrap(value, isn),
      isn,
      checkpoint
    ),
    value
  );
}
