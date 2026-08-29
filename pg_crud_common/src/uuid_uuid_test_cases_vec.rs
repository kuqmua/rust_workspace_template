#[must_use]
pub fn uuid_uuid_test_cases_vec() -> crate::uuid_uuid_test_cases::UuidUuidTestCases {
    crate::uuid_uuid_test_cases::UuidUuidTestCases::from([uuid::Uuid::from_u128(
        0x123e_4567_e89b_42d3_a456_4266_1417_4000u128,
    )])
}
