#[cfg(feature = "test-utils")]
#[must_use]
pub const fn u64_test_cases_vec() -> [u64; 3] {
    [u64::MIN, 0, u64::MAX]
}
