use super::{TestExpId, expect_error};

#[track_caller]
pub(super) fn map_err<T, E, R>(
    v: Result<T, E>,
    exp_id: impl Into<TestExpId>,
    check: impl FnOnce(&E),
    map: impl FnOnce(E, &'static str) -> R,
) -> R {
    let exp_id = exp_id.into();
    let error = expect_error(v, exp_id.0);
    check(&error);
    map(error, exp_id.0)
}
