use super::{TestExpId, map_err};

#[track_caller]
pub(crate) fn expect_error_mapped<T, E, R>(
    v: Result<T, E>,
    exp_id: impl Into<TestExpId>,
    map: impl FnOnce(E, &'static str) -> R,
) -> R {
    map_err(v, exp_id, |_| (), map)
}
