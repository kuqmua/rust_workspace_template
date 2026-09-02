#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "Leptos component expansion models props as generated fields even though the source declares functions"
)]
#![allow(
    clippy::arbitrary_source_item_ordering,
    reason = "component props and wire enum variants retain their semantic presentation order"
)]
#![allow(
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
pub(crate) fn AdminCardTitle(
    #[prop(optional)] option: Option<&'static str>,
    children: leptos::prelude::Children,
) -> impl leptos::prelude::IntoView {
    leptos::view! { <singlestage::CardTitle attr:data-name="CardTitle" class=option.map(String::from)>{children()}</singlestage::CardTitle> }
}
