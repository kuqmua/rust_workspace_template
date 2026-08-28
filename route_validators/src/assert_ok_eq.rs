use super::{TestExpId, expect_ok};

#[track_caller]
pub(crate) fn assert_ok_eq<T, E>(v: Result<T, E>, exp_id: impl Into<TestExpId>, expected: &T)
where
    T: PartialEq + std::fmt::Debug,
{
    assert_eq!(&expect_ok(v, exp_id), expected);
}
