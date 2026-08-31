pub const ZERO: i64 = 0i64;
pub const ONE: i64 = 1i64;

#[cfg(test)]
mod tests {
    #[test]
    fn test_i64_values_match_primitives() {
        assert_eq!(super::ZERO, 0i64);
        assert_eq!(super::ONE, 1i64);
    }
}
