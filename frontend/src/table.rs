#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "Leptos component expansion models props as generated fields even though the source declares functions"
)]
#![allow(
    clippy::multiple_inherent_impl,
    clippy::same_name_method,
    clippy::shadow_reuse,
    clippy::unused_trait_names,
    unreachable_pub,
    reason = "Leptos component macro expansion generates builders, fields, and bindings with framework-defined shapes"
)]

#[allow(
    unused_import_braces,
    reason = "grouped Leptos prelude imports are required by workspace source policy"
)]
#[rustfmt::skip]
use leptos::prelude::{AddAnyAttr};

#[leptos::component]
#[allow(clippy::single_call_fn, reason = "lint suppression is required here")]
pub(crate) fn Table(children: leptos::prelude::Children) -> impl leptos::prelude::IntoView {
    leptos::view! { <singlestage::Table attr:data-name="Table" attr:class="w-full max-w-7xl text-sm caption-bottom">{children()}</singlestage::Table> }
}
