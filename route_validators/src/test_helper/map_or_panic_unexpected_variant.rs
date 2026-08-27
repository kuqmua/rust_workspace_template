use super::{TestExpId, panic_unexpected_variant};

#[track_caller]
pub(super) fn map_or_panic_unexpected_variant<R>(
    map_res: Option<R>,
    exp_id: impl Into<TestExpId>,
) -> R {
    map_res.unwrap_or_else(|| panic_unexpected_variant(exp_id))
}
