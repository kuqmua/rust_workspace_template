#![allow(
    clippy::multiple_inherent_impl,
    reason = "signal ownership and settings-specific behavior are implemented in their owning modules"
)]
#[derive(
    Debug,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    proc_macro_newtype_from_inner::FromInner,
)]
pub struct LeptosAdminInputSignal(leptos::prelude::RwSignal<String>);

impl LeptosAdminInputSignal {
    pub(crate) const fn signal(self) -> leptos::prelude::RwSignal<String> {
        self.0
    }
}
