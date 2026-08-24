pub(crate) mod alert;
#[cfg(any(target_arch = "wasm32", test))]
pub(crate) mod alert_dialog;
pub(crate) mod badge;
pub(crate) mod button;
pub(crate) mod card;
#[cfg(not(target_arch = "wasm32"))]
pub(crate) mod checkbox;
#[cfg(any(target_arch = "wasm32", test))]
pub(crate) mod empty;
pub(crate) mod field;
pub(crate) mod input;
pub(crate) mod navigation;
pub(crate) mod spinner;
pub(crate) mod table;
pub(crate) mod textarea;

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
