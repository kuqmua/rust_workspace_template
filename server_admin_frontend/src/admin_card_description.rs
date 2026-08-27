#![allow(
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

use leptos::prelude::AddAnyAttr;

#[cfg(not(target_arch = "wasm32"))]
#[leptos::component]
#[allow(
    unreachable_pub,
    reason = "Leptos component visibility is required for card composition"
)]
pub(crate) fn AdminCardDescription(
    children: leptos::prelude::Children,
) -> impl leptos::prelude::IntoView {
    leptos::view! { <singlestage::CardDescription attr:data-name="CardDescription">{children()}</singlestage::CardDescription> }
}
