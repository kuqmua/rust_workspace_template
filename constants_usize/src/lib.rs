pub const ZERO: usize = 0usize;
pub const ONE: usize = 1usize;
pub const TWO: usize = 2usize;
pub const THREE: usize = 3usize;
pub const FOUR: usize = 4usize;
pub const SIX: usize = 6usize;
pub const SEVEN: usize = 7usize;
pub const EIGHT: usize = 8usize;
pub const ELEVEN: usize = 11usize;
pub const TWENTY: usize = 20usize;
pub const VALUE_64: usize = 64usize;
pub const VALUE_128: usize = 128usize;
pub const VALUE_1_024: usize = 1_024usize;
pub const VALUE_4_096: usize = 4_096usize;
pub const VALUE_8_192: usize = 8_192usize;
pub const VALUE_1_048_576: usize = 1_048_576usize;
pub const VALUE_16_777_216: usize = 16_777_216usize;

#[cfg(test)]
mod tests {
    #[test]
    fn values_match_primitives() {
        assert_eq!(super::ZERO, 0usize);
        assert_eq!(super::ONE, 1usize);
        assert_eq!(super::TWO, 2usize);
        assert_eq!(super::THREE, 3usize);
        assert_eq!(super::FOUR, 4usize);
        assert_eq!(super::SIX, 6usize);
        assert_eq!(super::SEVEN, 7usize);
        assert_eq!(super::EIGHT, 8usize);
        assert_eq!(super::ELEVEN, 11usize);
        assert_eq!(super::TWENTY, 20usize);
        assert_eq!(super::VALUE_64, 64usize);
        assert_eq!(super::VALUE_128, 128usize);
        assert_eq!(super::VALUE_1_024, 1_024usize);
        assert_eq!(super::VALUE_4_096, 4_096usize);
        assert_eq!(super::VALUE_8_192, 8_192usize);
        assert_eq!(super::VALUE_1_048_576, 1_048_576usize);
        assert_eq!(super::VALUE_16_777_216, 16_777_216usize);
    }
}
