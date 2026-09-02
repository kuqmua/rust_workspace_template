#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    proc_macro_newtype::FromInner,
    proc_macro_newtype::GetInner,
)]
#[accessor(pub(crate))]
#[borrow]
pub(super) struct HttpNotificationHeaderMap(http::HeaderMap);
