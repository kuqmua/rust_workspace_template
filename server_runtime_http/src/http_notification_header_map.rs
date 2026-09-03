#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    proc_macro_newtype_from_inner::FromInner,
    proc_macro_newtype_get_inner::GetInner,
)]
#[accessor(pub(crate))]
#[borrow]
pub(super) struct HttpNotificationHeaderMap(http::HeaderMap);
