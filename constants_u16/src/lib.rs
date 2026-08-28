pub const ZERO: u16 = 0u16;

#[cfg(test)]
mod tests {
    #[test]
    fn u16_zero_matches_primitive() {
        assert_eq!(super::ZERO, 0u16);
    }
}
