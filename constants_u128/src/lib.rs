pub const ZERO: u128 = 0u128;

#[cfg(test)]
mod tests {
    #[test]
    fn u128_zero_matches_primitive() {
        assert_eq!(super::ZERO, 0u128);
    }
}
