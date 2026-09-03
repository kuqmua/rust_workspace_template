#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "Leptos component expansion models props as generated fields even though the source declares functions"
)]
#![allow(
    clippy::multiple_inherent_impl,
    clippy::same_name_method,
    clippy::shadow_reuse,
    reason = "Leptos component macro expansion generates builders, fields, and bindings with framework-defined shapes"
)]

#[leptos::component]
#[allow(
    unreachable_pub,
    reason = "Leptos component visibility is required for composition across frontend modules"
)]
#[allow(
    clippy::single_call_fn,
    reason = "admin input group remains a named owner because its boundary role is clearer and directly testable"
)]
pub(crate) fn AdminInputGroup(
    children: leptos::prelude::Children,
) -> impl leptos::prelude::IntoView {
    crate::with_owner::with_owner(move || {
        leptos::view! {
            <singlestage::InputGroup class="table-page-size-controls">{children()}</singlestage::InputGroup>
        }
    })
}
