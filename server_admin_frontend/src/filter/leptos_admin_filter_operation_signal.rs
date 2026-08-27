#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, newtype::FromInner)]
pub(super) struct LeptosAdminFilterOperationSignal(pub(super) leptos::prelude::RwSignal<String>);
