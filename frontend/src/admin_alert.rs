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
#[allow(clippy::single_call_fn, reason = "lint suppression is required here")]
pub(crate) fn AdminAlert(
    #[prop(default = crate::admin_alert_variant::AdminAlertVariant::default())]
    admin_alert_variant: crate::admin_alert_variant::AdminAlertVariant,
    #[prop(optional)] option: Option<&'static str>,
    children: leptos::prelude::Children,
) -> impl leptos::prelude::IntoView {
    leptos::view! {
        <singlestage::Alert attr:data-name="Alert" id=option.map(String::from) attr:class=admin_alert_variant.class() role=admin_alert_variant.role()>{children()}</singlestage::Alert>
    }
}
