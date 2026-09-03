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
#[allow(
    clippy::single_call_fn,
    reason = "admin input remains a named owner because its boundary role is clearer and directly testable"
)]
pub(crate) fn AdminInput(
    #[prop(into)] admin_input_name: crate::admin_input_name::AdminInputName,
    #[prop(default = crate::admin_input_kind::AdminInputKind::default())]
    admin_input_kind: crate::admin_input_kind::AdminInputKind,
    #[prop(optional)] autocomplete: Option<&'static str>,
    #[prop(optional)] required: bool,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] minlength: Option<usize>,
    #[prop(optional)] maxlength: Option<usize>,
    #[prop(optional)] min: Option<u16>,
    #[prop(optional)] max: Option<u16>,
    #[prop(optional)] initial_value: Option<String>,
    #[prop(optional, into)] bind_value: Option<
        crate::leptos_admin_input_signal::LeptosAdminInputSignal,
    >,
) -> impl leptos::prelude::IntoView {
    let input_type = admin_input_kind.value();
    match bind_value {
        Some(value) => leptos::prelude::IntoAny::into_any(leptos::view! {
            <singlestage::Input attr:data-name="Input" attr:class="ui-input flex h-9 w-full min-w-0 rounded-md border border-input bg-transparent px-3 py-1 text-base text-foreground shadow-xs outline-none transition-[color,box-shadow] file:inline-flex file:h-7 file:border-0 file:bg-transparent file:text-sm file:font-medium file:text-foreground placeholder:text-muted-foreground selection:bg-primary selection:text-primary-foreground read-only:bg-muted focus-visible:border-ring focus-visible:ring-2 focus-visible:ring-ring/50 aria-invalid:border-destructive aria-invalid:ring-destructive/20 disabled:pointer-events-none disabled:cursor-not-allowed disabled:opacity-50 dark:bg-input/30 dark:aria-invalid:ring-destructive/40 md:text-sm" name=String::from(admin_input_name.as_ref()) input_type=input_type autocomplete=autocomplete.map(String::from) required=required disabled=disabled minlength=minlength maxlength=maxlength min=min.map(|min_value| min_value.to_string()) max=max.map(|max_value| max_value.to_string()) default=initial_value value=value.signal() />
        }),
        None => leptos::prelude::IntoAny::into_any(leptos::view! {
            <singlestage::Input attr:data-name="Input" attr:class="ui-input flex h-9 w-full min-w-0 rounded-md border border-input bg-transparent px-3 py-1 text-base text-foreground shadow-xs outline-none transition-[color,box-shadow] file:inline-flex file:h-7 file:border-0 file:bg-transparent file:text-sm file:font-medium file:text-foreground placeholder:text-muted-foreground selection:bg-primary selection:text-primary-foreground read-only:bg-muted focus-visible:border-ring focus-visible:ring-2 focus-visible:ring-ring/50 aria-invalid:border-destructive aria-invalid:ring-destructive/20 disabled:pointer-events-none disabled:cursor-not-allowed disabled:opacity-50 dark:bg-input/30 dark:aria-invalid:ring-destructive/40 md:text-sm" name=String::from(admin_input_name.as_ref()) input_type=input_type autocomplete=autocomplete.map(String::from) required=required disabled=disabled minlength=minlength maxlength=maxlength min=min.map(|value| value.to_string()) max=max.map(|value| value.to_string()) value=initial_value.unwrap_or_default() />
        }),
    }
}
