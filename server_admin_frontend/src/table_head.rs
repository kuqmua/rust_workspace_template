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
pub(crate) fn TableHead(
    #[prop(optional)] data_field: Option<String>,
    #[prop(optional)] data_filter_count: Option<String>,
    children: leptos::prelude::Children,
) -> impl leptos::prelude::IntoView {
    leptos::view! { <singlestage::TableHead attr:data-name="TableHead" attr:data-field=data_field attr:data-filter-count=data_filter_count attr:class="h-10 px-2 text-left align-middle font-medium text-muted-foreground [&:has([role=checkbox])]:pr-0 [&>[role=checkbox]]:translate-y-[2px]">{children()}</singlestage::TableHead> }
}
