#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "Leptos component expansion models props as generated fields even though the source declares functions"
)]
#![allow(
    clippy::arbitrary_source_item_ordering,
    clippy::impl_trait_in_params,
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
pub(crate) fn TableCell(
    #[prop(optional, into)] data_label: Option<std::borrow::Cow<'static, str>>,
    #[prop(optional, into)] data_field: Option<std::borrow::Cow<'static, str>>,
    #[prop(optional)] class: Option<&'static str>,
    children: leptos::prelude::Children,
) -> impl leptos::prelude::IntoView {
    let class = class.map_or_else(
        || {
            std::borrow::Cow::Borrowed(
                constants_str::VALUE_19AB4EBD,
            )
        },
        |class| {
            std::borrow::Cow::Owned(format!(
                "p-4 align-middle [&:has([role=checkbox])]:pr-0 [&:has([role=checkbox])]:pl-3 {class}"
            ))
        },
    );
    leptos::view! { <singlestage::TableCell attr:data-name="TableCell" attr:data-label=data_label attr:data-field=data_field attr:class=class>{children()}</singlestage::TableCell> }
}
