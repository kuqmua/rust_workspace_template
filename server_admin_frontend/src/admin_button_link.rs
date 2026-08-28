#![allow(
    clippy::arbitrary_source_item_ordering,
    reason = "component props and wire enum variants retain their semantic presentation order"
)]
#![allow(
    clippy::field_scoped_visibility_modifiers,
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
#[allow(clippy::single_call_fn)] // named UI component or render stage has one composition owner
pub(crate) fn AdminButtonLink(
    href: &'static str,
    #[prop(default = super::AdminButtonVariant::default())] variant: super::AdminButtonVariant,
    children: leptos::prelude::Children,
) -> impl leptos::prelude::IntoView {
    leptos::view! {
        <singlestage::Link class=variant.class() href=href>{children()}</singlestage::Link>
    }
}
