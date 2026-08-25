#[cfg(any(target_arch = "wasm32", test))]
pub(crate) mod admin_alert_dialog;
#[cfg(not(target_arch = "wasm32"))]
pub(crate) mod admin_checkbox;
#[cfg(any(target_arch = "wasm32", test))]
pub(crate) mod admin_empty;
pub(crate) mod admin_spinner;
pub(crate) mod admin_textarea;
pub(crate) mod alert;
pub(crate) mod badge;
pub(crate) mod button;
pub(crate) mod card;
pub(crate) mod field;
pub(crate) mod input;
pub(crate) mod navigation;
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
mod tests;
