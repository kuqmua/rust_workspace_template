#![allow(
    clippy::field_scoped_visibility_modifiers,
    clippy::multiple_inherent_impl,
    clippy::same_name_method,
    clippy::shadow_reuse,
    clippy::unused_trait_names,
    reason = "Leptos component macro expansion generates builders, fields, and bindings with framework-defined shapes"
)]

use leptos::prelude::AddAnyAttr;

#[leptos::component]
#[allow(
    unreachable_pub,
    reason = "Leptos component visibility is required for card composition"
)]
#[allow(clippy::single_call_fn)] // named UI component or render stage has one composition owner
pub(crate) fn AdminCardHeader(
    children: leptos::prelude::Children,
) -> impl leptos::prelude::IntoView {
    leptos::view! { <singlestage::CardHeader attr:data-name="CardHeader">{children()}</singlestage::CardHeader> }
}
