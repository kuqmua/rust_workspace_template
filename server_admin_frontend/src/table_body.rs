#![allow(
    clippy::field_scoped_visibility_modifiers,
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
#[allow(clippy::single_call_fn)] // named UI component or render stage has one composition owner
pub(crate) fn TableBody(children: leptos::prelude::Children) -> impl leptos::prelude::IntoView {
    leptos::view! { <singlestage::TableBody attr:data-name="TableBody" attr:class="[&_tr:last-child]:border-0">{children()}</singlestage::TableBody> }
}
