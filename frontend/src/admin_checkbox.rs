#![allow(
    dead_code,
    clippy::field_scoped_visibility_modifiers,
    reason = "Leptos component expansion models props as generated fields and triggers false dead-field diagnostics even though the source function consumes every prop"
)]
#![allow(
    clippy::arbitrary_source_item_ordering,
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
pub(crate) fn AdminCheckbox(
    name: &'static str,
    value: &'static str,
    #[prop(optional)] bool: bool,
) -> impl leptos::prelude::IntoView {
    leptos::view! {
        <singlestage::Checkbox
            attr:data-name="Checkbox"
            class="peer size-4 shrink-0 rounded-[4px] border border-input shadow-xs outline-none transition-shadow dark:bg-input/30 checked:border-primary checked:bg-primary checked:text-primary-foreground focus-visible:border-ring focus-visible:ring-3 focus-visible:ring-ring/50 disabled:cursor-not-allowed disabled:opacity-50"
            name=name
            value=value
            required=bool
        />
    }
}
