pub fn square(s: u32) -> u64 {
    assert!(s > 0 && s < 65, "Square must be between 1 and 64");
  
    2_u64.pow(s - 1)
}

pub fn total() -> u64 {
    2_u64.wrapping_pow(64).wrapping_sub(1)
}
