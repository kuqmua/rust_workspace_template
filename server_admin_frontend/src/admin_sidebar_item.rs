#![allow(
    clippy::field_scoped_visibility_modifiers,
    clippy::multiple_inherent_impl,
    clippy::same_name_method,
    clippy::shadow_reuse,
    clippy::unused_trait_names,
    unused_imports,
    reason = "Leptos component macro expansion generates framework-defined shapes"
)]

#[allow(unused_import_braces, reason = "grouped Leptos prelude imports are required by workspace source policy")]
#[rustfmt::skip]
use leptos::prelude::{AddAnyAttr, ClassAttribute, CustomAttribute, ElementChild};

#[leptos::component]
#[allow(
    unreachable_pub,
    reason = "Leptos component visibility is required for navigation composition"
)]
#[allow(clippy::single_call_fn)] // named UI component or render stage has one composition owner
pub(crate) fn AdminSidebarItem(
    children: leptos::prelude::Children,
) -> impl leptos::prelude::IntoView {
    leptos::view! { <li>{children()}</li> }
}
