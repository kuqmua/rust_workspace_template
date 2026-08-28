#![allow(
    clippy::arbitrary_source_item_ordering,
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
pub(crate) fn TableHead(
    #[prop(optional)] data_field: Option<String>,
    #[prop(optional)] data_filter_count: Option<String>,
    children: leptos::prelude::Children,
) -> impl leptos::prelude::IntoView {
    leptos::view! { <singlestage::TableHead attr:data-name="TableHead" attr:data-field=data_field attr:data-filter-count=data_filter_count attr:class="h-10 px-2 text-left align-middle font-medium text-muted-foreground [&:has([role=checkbox])]:pr-0 [&>[role=checkbox]]:translate-y-[2px]">{children()}</singlestage::TableHead> }
}
