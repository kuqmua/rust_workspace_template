#![allow(
    clippy::arbitrary_source_item_ordering,
    clippy::field_scoped_visibility_modifiers,
    clippy::impl_trait_in_params,
    clippy::missing_const_for_fn,
    clippy::multiple_inherent_impl,
    clippy::needless_pass_by_value,
    clippy::option_if_let_else,
    clippy::same_name_method,
    clippy::shadow_reuse,
    clippy::single_call_fn,
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
pub(crate) fn AdminSpinner() -> impl leptos::prelude::IntoView {
    leptos::view! {
        <div data-name="Spinner" class="ui-spinner loading-state" role="status" aria-live="polite">
            <singlestage::Spinner class="loading-spinner size-4 animate-spin rounded-full border-2 border-muted border-t-primary" />
            <span class="sr-only">"Loading\u{2026}"</span>
        </div>
    }
}
