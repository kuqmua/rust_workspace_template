#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "Leptos component expansion models props as generated fields even though the source declares functions"
)]
#![allow(
    clippy::same_name_method,
    clippy::unused_trait_names,
    reason = "Leptos component macro expansion generates builders, fields, and bindings with framework-defined shapes"
)]

use leptos::prelude::{
    AriaAttributes, ClassAttribute, CustomAttribute, ElementChild, GlobalAttributes,
};

#[leptos::component]
#[allow(
    unreachable_pub,
    reason = "Leptos component visibility is required for composition across frontend modules"
)]
#[allow(
    clippy::single_call_fn,
    reason = "admin spinner remains a named owner because its boundary role is clearer and directly testable"
)]
pub(crate) fn AdminSpinner() -> impl leptos::prelude::IntoView {
    leptos::view! {
        <div data-name="Spinner" class="ui-spinner loading-state" role="status" aria-live="polite" aria-label=constants_str::ADMIN_UI_LOADING>
            <span aria-hidden=constants_str::TRUE class="loading-spinner size-4 animate-spin rounded-full border-2 border-muted border-t-primary"></span>
            <span class="sr-only">{constants_str::ADMIN_UI_LOADING}</span>
        </div>
    }
}
