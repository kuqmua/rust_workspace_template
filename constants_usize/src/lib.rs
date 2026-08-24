pub const ZERO: usize = 0usize;
pub const ONE: usize = 1usize;
pub const VALUE_8_192: usize = 8_192usize;
pub const VALUE_1_048_576: usize = 1_048_576usize;
pub const VALUE_16_777_216: usize = 16_777_216usize;

#[cfg(test)]
mod tests {
    #[test]
    fn values_match_primitives() {
        assert_eq!(super::ZERO, 0usize);
        assert_eq!(super::ONE, 1usize);
        assert_eq!(super::VALUE_8_192, 8_192usize);
        assert_eq!(super::VALUE_1_048_576, 1_048_576usize);
        assert_eq!(super::VALUE_16_777_216, 16_777_216usize);
    }
}
