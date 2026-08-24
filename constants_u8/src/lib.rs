pub const ZERO: u8 = 0u8;

#[cfg(test)]
mod tests {
    #[test]
    fn zero_matches_primitive() {
        assert_eq!(super::ZERO, 0u8);
    }
}
