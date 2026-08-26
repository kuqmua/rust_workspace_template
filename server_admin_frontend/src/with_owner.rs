#[cfg(any(target_arch = "wasm32", test))]
#[path = "domain_types_with_owner_admin_alert_dialog.rs"]
pub(crate) mod admin_alert_dialog;
#[cfg(not(target_arch = "wasm32"))]
#[path = "domain_types_with_owner_admin_checkbox.rs"]
pub(crate) mod admin_checkbox;
#[cfg(any(target_arch = "wasm32", test))]
#[path = "domain_types_with_owner_admin_empty.rs"]
pub(crate) mod admin_empty;
#[path = "domain_types_with_owner_admin_spinner.rs"]
pub(crate) mod admin_spinner;
#[path = "domain_types_with_owner_admin_textarea.rs"]
pub(crate) mod admin_textarea;
#[path = "domain_types_with_owner_alert.rs"]
pub(crate) mod alert;
#[path = "domain_types_with_owner_badge.rs"]
pub(crate) mod badge;
#[path = "domain_types_with_owner_button.rs"]
pub(crate) mod button;
#[path = "domain_types_with_owner_card.rs"]
pub(crate) mod card;
#[path = "domain_types_with_owner_field.rs"]
pub(crate) mod field;
#[path = "domain_types_with_owner_input.rs"]
pub(crate) mod input;
#[path = "domain_types_with_owner_navigation.rs"]
pub(crate) mod navigation;
#[path = "domain_types_with_owner_table.rs"]
pub(crate) mod tables;

pub(crate) fn with_owner<View>(build: impl FnOnce() -> View) -> impl leptos::prelude::IntoView
where
    View: leptos::prelude::IntoView,
{
    let owner = leptos::prelude::Owner::new();
    let view = owner.with(build);
    leptos::tachys::reactive_graph::OwnedView::new_with_owner(view, owner)
}

#[cfg(test)]
#[path = "domain_types_with_owner_tests.rs"]
mod tests;
