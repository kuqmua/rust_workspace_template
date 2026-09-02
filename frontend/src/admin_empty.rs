#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "Leptos component expansion models props as generated fields even though the source declares functions"
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
#[allow(clippy::single_call_fn)] // named UI component or render stage has one composition owner
pub(crate) fn AdminEmpty(children: leptos::prelude::Children) -> impl leptos::prelude::IntoView {
    leptos::view! {
        <singlestage::Empty attr:data-name="Empty" attr:class="ui-empty empty-state flex flex-col items-center justify-center gap-4 rounded-lg border border-dashed p-8 text-center">
            <singlestage::EmptyHeader attr:data-name="EmptyHeader">
                <singlestage::EmptyTitle attr:data-name="EmptyTitle">{children()}</singlestage::EmptyTitle>
            </singlestage::EmptyHeader>
        </singlestage::Empty>
    }
}
