pub mod address;
pub mod gsm7;
pub mod time;

pub(crate) fn div_round_up(dividend: usize, divisor: usize) -> usize {
    (dividend + divisor - 1) / divisor
}

pub(crate) fn align(len: usize, align_pow2: usize) -> usize {
    (len + align_pow2 - 1) & !(align_pow2 - 1)
}
