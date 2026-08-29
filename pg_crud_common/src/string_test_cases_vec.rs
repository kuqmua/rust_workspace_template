#[cfg(feature = "test-utils")]
#[must_use]
pub fn string_test_cases_vec() -> [String; 12] {
    [
        String::new(),
        constants_str::catalog::A_ALT.to_owned(),
        constants_str::catalog::HELLO_WORLD.to_owned(),
        constants_str::catalog::THREE_SPACES.to_owned(),
        constants_str::catalog::NEWLINE_CARRIAGE_RETURN_TAB.to_owned(),
        constants_str::catalog::VALUE_1234567890.to_owned(),
        constants_str::integration_fixtures::U_1F600.to_owned(),
        constants_str::integration_fixtures::U_3053_U_3093_U_306B_U_3061_U_306F.to_owned(),
        constants_str::integration_fixtures::U_1F30D_U_1F680_U_2728_RUST_U_1F496_U_1F980.to_owned(),
        constants_str::catalog::A_ALT.repeat(1024),
        constants_str::catalog::LINE1_NEWLINE_LINE2_NEWLINE_LINE3.to_owned(),
        constants_str::integration_fixtures::U_1F496.to_owned(),
    ]
}
