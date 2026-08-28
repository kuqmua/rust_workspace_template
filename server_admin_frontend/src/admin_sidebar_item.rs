#![allow(
    clippy::arbitrary_source_item_ordering,
    clippy::field_scoped_visibility_modifiers,
    clippy::impl_trait_in_params,
    clippy::missing_const_for_fn,
    clippy::multiple_inherent_impl,
    clippy::needless_pass_by_value,
    clippy::option_if_let_else,
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
pub(crate) fn AdminSidebarItem(
    children: leptos::prelude::Children,
) -> impl leptos::prelude::IntoView {
    leptos::view! { <li>{children()}</li> }
}
