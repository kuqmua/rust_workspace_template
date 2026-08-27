use super::{TestExpId, map_or_panic_unexpected_variant};

#[track_caller]
pub(crate) fn expect_variant<T, R>(
    v: T,
    map: impl FnOnce(T) -> Option<R>,
    exp_id: impl Into<TestExpId>,
) -> R {
    map_or_panic_unexpected_variant(map(v), exp_id)
}
