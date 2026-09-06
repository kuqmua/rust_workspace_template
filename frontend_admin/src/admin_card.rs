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

#[allow(unused_import_braces, reason = "grouped Leptos prelude imports are required by workspace source policy")]
#[rustfmt::skip]
use leptos::prelude::{AddAnyAttr};

#[leptos::component]
#[allow(
    unreachable_pub,
    reason = "Leptos component visibility is required for composition across frontend modules"
)]
#[allow(
    clippy::single_call_fn,
    reason = "admin card requires this localized allowance for generated or framework-constrained code verified by focused tests"
)]
pub(crate) fn AdminCard(
    #[prop(default = crate::admin_card_variant::AdminCardVariant::default())]
    admin_card_variant: crate::admin_card_variant::AdminCardVariant,
    children: leptos::prelude::Children,
) -> impl leptos::prelude::IntoView {
    leptos::view! {
        <singlestage::Card attr:data-name="Card" attr:data-size="default" attr:class=admin_card_variant.class()>
            <singlestage::CardContent attr:data-name="CardContent" attr:class="px-6">{children()}</singlestage::CardContent>
        </singlestage::Card>
    }
}
