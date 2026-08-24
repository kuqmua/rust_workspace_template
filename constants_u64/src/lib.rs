pub const ZERO: u64 = 0u64;

#[cfg(test)]
mod tests {
    #[test]
    fn zero_matches_primitive() {
        assert_eq!(super::ZERO, 0u64);
    }
}
