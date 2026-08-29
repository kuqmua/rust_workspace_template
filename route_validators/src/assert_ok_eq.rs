#[track_caller]
pub(crate) fn assert_ok_eq<T, E>(
    v: Result<T, E>,
    exp_id: impl Into<crate::test_exp_id::TestExpId>,
    expected: &T,
) where
    T: PartialEq + std::fmt::Debug,
{
    assert_eq!(&crate::expect_ok::expect_ok(v, exp_id), expected);
}
