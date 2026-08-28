pub const ZERO: u64 = 0u64;
pub const ONE: u64 = 1u64;

#[cfg(test)]
mod tests {
    #[test]
    fn u64_values_match_primitives() {
        assert_eq!(super::ZERO, 0u64);
        assert_eq!(super::ONE, 1u64);
    }
}
