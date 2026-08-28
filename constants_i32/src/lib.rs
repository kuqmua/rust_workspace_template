pub const ZERO: i32 = 0i32;

#[cfg(test)]
mod tests {
    #[test]
    fn i32_zero_matches_primitive() {
        assert_eq!(super::ZERO, 0i32);
    }
}
