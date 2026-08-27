#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, newtype::FromInner)]
pub(crate) struct LeptosAdminInputSignal(leptos::prelude::RwSignal<String>);

impl LeptosAdminInputSignal {
    pub(crate) const fn signal(self) -> leptos::prelude::RwSignal<String> {
        self.0
    }
}
