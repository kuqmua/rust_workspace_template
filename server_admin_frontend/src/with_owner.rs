pub(crate) fn with_owner<View>(build: impl FnOnce() -> View) -> impl leptos::prelude::IntoView
where
    View: leptos::prelude::IntoView,
{
    let owner = leptos::prelude::Owner::new();
    let view = owner.with(build);
    leptos::tachys::reactive_graph::OwnedView::new_with_owner(view, owner)
}

// Root-owned module compatibility wrappers.
#[cfg(any(target_arch = "wasm32", test))]
pub(crate) mod admin_alert_dialog {
    pub use super::super::admin_alert_dialog::*;
}
#[cfg(not(target_arch = "wasm32"))]
pub(crate) mod admin_checkbox {
    pub use super::super::admin_checkbox::*;
}
#[cfg(any(target_arch = "wasm32", test))]
pub(crate) mod admin_empty {
    pub use super::super::admin_empty::*;
}
pub(crate) mod admin_spinner {
    pub use super::super::admin_spinner::*;
}
pub(crate) mod admin_textarea {
    pub use super::super::admin_textarea::*;
}
pub(crate) mod alert {
    pub use super::super::domain_types_with_owner_alert::*;
}
pub(crate) mod badge {
    pub use super::super::domain_types_with_owner_badge::*;
}
pub(crate) mod button {
    pub use super::super::domain_types_with_owner_button::*;
}
pub(crate) mod card {
    pub use super::super::domain_types_with_owner_card::*;
}
pub(crate) mod field {
    pub use super::super::domain_types_with_owner_field::*;
}
pub(crate) mod input {
    pub use super::super::domain_types_with_owner_input::*;
}
pub(crate) mod navigation {
    pub use super::super::domain_types_with_owner_navigation::*;
}
pub(crate) mod tables {
    pub use super::super::domain_types_with_owner_table::*;
}
