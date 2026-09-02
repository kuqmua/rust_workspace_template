#[track_caller]
pub(crate) fn assert_ok_eq<T, E>(
    result: Result<T, E>,
    exp_id: impl Into<crate::test_exp_id::TestExpId>,
    t: &T,
) where
    T: PartialEq + std::fmt::Debug,
{
    assert_eq!(&crate::expect_ok::expect_ok(result, exp_id), t);
}
