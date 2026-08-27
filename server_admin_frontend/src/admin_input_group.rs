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

#[leptos::component]
#[allow(
    unreachable_pub,
    reason = "Leptos component visibility is required for composition across frontend modules"
)]
pub(crate) fn AdminInputGroup(
    children: leptos::prelude::Children,
) -> impl leptos::prelude::IntoView {
    crate::domain_types::with_owner::with_owner(move || {
        leptos::view! {
            <singlestage::InputGroup class="table-page-size-controls">{children()}</singlestage::InputGroup>
        }
    })
}
