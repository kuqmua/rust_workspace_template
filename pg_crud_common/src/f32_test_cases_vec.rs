#[cfg(feature = "test-utils")]
#[must_use]
pub const fn f32_test_cases_vec() -> [f32; 18] {
    [
        f32::EPSILON,
        f32::MAX,
        f32::MIN,
        f32::MIN_POSITIVE,
        -1e30,
        -1e-30,
        -16_777_214.0,
        -100.0,
        -10.0,
        -1.0,
        -0.0,
        0.0,
        1.0,
        10.0,
        100.0,
        16_777_214.0,
        1e-30,
        1e30,
    ]
}
