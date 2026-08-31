pub const ZERO: i32 = 0i32;

#[cfg(test)]
mod tests {
    #[test]
    fn test_i32_zero_matches_primitive() {
        assert_eq!(super::ZERO, 0i32);
    }
}
