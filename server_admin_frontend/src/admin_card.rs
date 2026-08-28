#![allow(
    clippy::arbitrary_source_item_ordering,
    reason = "component props and wire enum variants retain their semantic presentation order"
)]
#![allow(
    clippy::field_scoped_visibility_modifiers,
    clippy::impl_trait_in_params,
    clippy::missing_const_for_fn,
    clippy::multiple_inherent_impl,
    clippy::needless_pass_by_value,
    clippy::option_if_let_else,
    clippy::same_name_method,
    clippy::shadow_reuse,
    clippy::unused_trait_names,
    reason = "Leptos component macro expansion generates builders, fields, and bindings with framework-defined shapes"
)]

#[allow(unused_import_braces, reason = "grouped Leptos prelude imports are required by workspace source policy")]
#[rustfmt::skip]
use leptos::prelude::{AddAnyAttr};

#[leptos::component]
#[allow(
    unreachable_pub,
    reason = "Leptos component visibility is required for composition across frontend modules"
)]
pub(crate) fn AdminCard(
    #[prop(default = super::AdminCardVariant::default())] variant: super::AdminCardVariant,
    children: leptos::prelude::Children,
) -> impl leptos::prelude::IntoView {
    leptos::view! {
        <singlestage::Card attr:data-name="Card" attr:data-size="default" attr:class=variant.class()>
            <singlestage::CardContent attr:data-name="CardContent" attr:class="px-6">{children()}</singlestage::CardContent>
        </singlestage::Card>
    }
}
