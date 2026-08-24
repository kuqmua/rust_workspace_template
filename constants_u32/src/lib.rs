pub const ZERO: u32 = 0u32;

#[cfg(test)]
mod tests {
    #[test]
    fn zero_matches_primitive() {
        assert_eq!(super::ZERO, 0u32);
    }
}
