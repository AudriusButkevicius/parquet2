use std::convert::TryInto;

pub mod bitpacking;
pub mod hybrid_rle;
pub mod uleb128;

pub use parquet_format::Encoding;

/// # Panics
/// This function panics iff `values.len() < 4`.
pub fn get_length(values: &[u8]) -> u32 {
    u32::from_le_bytes(values[0..4].try_into().unwrap())
}

/// Returns ceil(log2(x))
#[inline]
pub fn log2(mut x: u64) -> u32 {
    if x == 1 {
        return 0;
    }
    x -= 1;
    let mut result = 0;
    while x > 0 {
        x >>= 1;
        result += 1;
    }
    result
}

/// Returns the ceil of value/divisor
#[inline]
pub fn ceil8(value: usize) -> usize {
    value / 8 + ((value % 8 != 0) as usize)
}