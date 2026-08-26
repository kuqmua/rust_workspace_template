#[cfg(any(target_arch = "wasm32", test))]
#[path = "domain_types__with_owner__admin_alert_dialog.rs"]
pub(crate) mod admin_alert_dialog;
#[cfg(not(target_arch = "wasm32"))]
#[path = "domain_types__with_owner__admin_checkbox.rs"]
pub(crate) mod admin_checkbox;
#[cfg(any(target_arch = "wasm32", test))]
#[path = "domain_types__with_owner__admin_empty.rs"]
pub(crate) mod admin_empty;
#[path = "domain_types__with_owner__admin_spinner.rs"]
pub(crate) mod admin_spinner;
#[path = "domain_types__with_owner__admin_textarea.rs"]
pub(crate) mod admin_textarea;
#[path = "domain_types__with_owner__alert.rs"]
pub(crate) mod alert;
#[path = "domain_types__with_owner__badge.rs"]
pub(crate) mod badge;
#[path = "domain_types__with_owner__button.rs"]
pub(crate) mod button;
#[path = "domain_types__with_owner__card.rs"]
pub(crate) mod card;
#[path = "domain_types__with_owner__field.rs"]
pub(crate) mod field;
#[path = "domain_types__with_owner__input.rs"]
pub(crate) mod input;
#[path = "domain_types__with_owner__navigation.rs"]
pub(crate) mod navigation;
#[path = "domain_types__with_owner__table.rs"]
pub(crate) mod table;

pub(crate) fn with_owner<View>(build: impl FnOnce() -> View) -> impl leptos::prelude::IntoView
where
    View: leptos::prelude::IntoView,
{
    let owner = leptos::prelude::Owner::new();
    let view = owner.with(build);
    leptos::tachys::reactive_graph::OwnedView::new_with_owner(view, owner)
}

#[cfg(test)]
#[path = "domain_types__with_owner__tests.rs"]
mod tests;
