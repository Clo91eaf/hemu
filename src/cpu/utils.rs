fn bitmask(bits: u32) -> u32 {
  (1u32 << bits) - 1
}

// [lo, hi)
pub fn bits(x: u32, lo: u32, hi: u32) -> usize {
  assert!(hi >= lo);
  ((x >> lo) & bitmask(hi - lo)) as usize
}

pub fn sext(x: usize, len: u32) -> u64 {
  assert!(len <= 64);
  let extend_bits = 64 - len;
  (((x as i64) << extend_bits) >> extend_bits) as u64
}
