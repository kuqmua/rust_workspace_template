#[track_caller]
pub(crate) fn expect_variant<T, R>(
    t: T,
    map: impl FnOnce(T) -> Option<R>,
    expectation_id: impl Into<crate::test_expectation_id::TestExpectationId>,
) -> R {
    crate::map_or_panic_unexpected_variant::map_or_panic_unexpected_variant(map(t), expectation_id)
}
