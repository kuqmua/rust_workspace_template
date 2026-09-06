#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "Leptos component expansion models props as generated fields even though the source declares functions"
)]
#![allow(
    clippy::arbitrary_source_item_ordering,
    clippy::multiple_inherent_impl,
    clippy::same_name_method,
    clippy::shadow_reuse,
    clippy::unused_trait_names,
    unused_imports,
    reason = "Leptos component macro expansion generates builders, fields, and bindings with framework-defined shapes"
)]

#[allow(unused_import_braces, reason = "grouped Leptos prelude imports are required by workspace source policy")]
#[rustfmt::skip]
use leptos::prelude::{AddAnyAttr, ClassAttribute, CustomAttribute, ElementChild};

#[leptos::component]
#[allow(
    unreachable_pub,
    reason = "Leptos component visibility is required for composition across frontend modules"
)]
#[allow(
    clippy::single_call_fn,
    reason = "admin navigation link remains a named owner because its boundary role is clearer and directly testable"
)]
pub(crate) fn AdminNavigationLink(
    string: String,
    bool: bool,
    children: leptos::prelude::Children,
) -> impl leptos::prelude::IntoView {
    leptos::view! {
        <singlestage::Link
            attr:data-name="NavigationMenuLink"
            class=if bool { "active inline-flex items-center rounded-sm text-sm font-medium text-foreground transition-colors focus:outline-none" } else { "inline-flex items-center rounded-sm text-sm font-medium text-foreground/70 transition-colors hover:text-foreground focus:outline-none" }
            attr:aria-current=bool.then_some("page")
            href=string
        >
            {children()}
        </singlestage::Link>
    }
}
