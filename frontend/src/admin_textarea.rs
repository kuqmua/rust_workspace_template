#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "Leptos component expansion models props as generated fields even though the source declares functions"
)]
#![allow(
    clippy::arbitrary_source_item_ordering,
    clippy::impl_trait_in_params,
    clippy::multiple_inherent_impl,
    clippy::needless_pass_by_value,
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
#[allow(clippy::single_call_fn, reason = "lint suppression is required here")]
pub(crate) fn AdminTextarea(
    #[prop(into)] admin_input_name: crate::admin_input_name::AdminInputName,
    #[prop(optional)] required: bool,
    #[prop(optional)] disabled: bool,
    #[prop(optional, into)] option: Option<
        crate::leptos_admin_input_signal::LeptosAdminInputSignal,
    >,
) -> impl leptos::prelude::IntoView {
    option.map_or_else(
        || leptos::prelude::IntoAny::into_any(leptos::view! {
            <singlestage::Textarea attr:data-name="Textarea" attr:class="ui-textarea field-sizing-content flex min-h-16 w-full rounded-md border border-input bg-transparent px-3 py-2 text-base shadow-xs outline-none transition-[color,box-shadow] placeholder:text-muted-foreground focus-visible:border-ring focus-visible:ring-2 focus-visible:ring-ring/50 aria-invalid:border-destructive aria-invalid:ring-destructive/20 disabled:cursor-not-allowed disabled:opacity-50 dark:bg-input/30 dark:aria-invalid:ring-destructive/40 md:text-sm" name=String::from(admin_input_name.as_ref()) required=required disabled=disabled />
        }),
        |value| leptos::prelude::IntoAny::into_any(leptos::view! {
            <singlestage::Textarea attr:data-name="Textarea" attr:class="ui-textarea field-sizing-content flex min-h-16 w-full rounded-md border border-input bg-transparent px-3 py-2 text-base shadow-xs outline-none transition-[color,box-shadow] placeholder:text-muted-foreground focus-visible:border-ring focus-visible:ring-2 focus-visible:ring-ring/50 aria-invalid:border-destructive aria-invalid:ring-destructive/20 disabled:cursor-not-allowed disabled:opacity-50 dark:bg-input/30 dark:aria-invalid:ring-destructive/40 md:text-sm" name=String::from(admin_input_name.as_ref()) required=required disabled=disabled value=value.signal() />
        }),
    )
}
