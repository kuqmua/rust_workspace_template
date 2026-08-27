#[cfg(feature = "test-utils")]
#[must_use]
pub const fn f64_test_cases_vec() -> [f64; 18] {
    [
        f64::EPSILON,
        f64::MAX,
        f64::MIN,
        f64::MIN_POSITIVE,
        -1e300,
        -1e-300,
        -9_007_199_254_740_990.0,
        -100.0,
        -10.0,
        -1.0,
        -0.0,
        0.0,
        1.0,
        10.0,
        100.0,
        9_007_199_254_740_990.0,
        1e-300,
        1e300,
    ]
}
