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
#[allow(clippy::single_call_fn, reason = "lint suppression is required here")]
pub(crate) fn AdminSpinner() -> impl leptos::prelude::IntoView {
    leptos::view! {
        <div data-name="Spinner" class="ui-spinner loading-state" role="status" aria-live="polite">
            <singlestage::Spinner class="loading-spinner size-4 animate-spin rounded-full border-2 border-muted border-t-primary" />
            <span class="sr-only">"Loading\u{2026}"</span>
        </div>
    }
}
