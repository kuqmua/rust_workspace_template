#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    newtype::FromInner,
    newtype::IntoInnerFrom,
)]
pub(super) struct LeptosAdminFilterOperationSignal(leptos::prelude::RwSignal<String>);
