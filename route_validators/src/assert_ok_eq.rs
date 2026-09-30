#[track_caller]
pub(crate) fn assert_ok_eq<T, E>(
    result: Result<T, E>,
    expectation_id: impl Into<crate::test_expectation_id::TestExpectationId>,
    t: &T,
) where
    T: PartialEq + std::fmt::Debug,
{
    assert_eq!(&crate::expect_ok::expect_ok(result, expectation_id), t);
}
