#![allow(
    clippy::arbitrary_source_item_ordering,
    dead_code,
    clippy::field_scoped_visibility_modifiers,
    clippy::impl_trait_in_params,
    clippy::missing_const_for_fn,
    clippy::multiple_inherent_impl,
    clippy::needless_pass_by_value,
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
pub(crate) fn Table(children: leptos::prelude::Children) -> impl leptos::prelude::IntoView {
    leptos::view! { <singlestage::Table attr:data-name="Table" attr:class="w-full max-w-7xl text-sm caption-bottom">{children()}</singlestage::Table> }
}
