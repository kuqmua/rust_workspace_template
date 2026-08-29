#![allow(
    clippy::multiple_inherent_impl,
    reason = "signal ownership and settings-specific behavior are implemented in their owning modules"
)]
#[derive(Debug, optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, newtype::FromInner)]
pub struct LeptosAdminInputSignal(leptos::prelude::RwSignal<String>);

impl LeptosAdminInputSignal {
    pub(crate) const fn signal(self) -> leptos::prelude::RwSignal<String> {
        self.0
    }
}
