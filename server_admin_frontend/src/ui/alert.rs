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
    clippy::single_call_fn,
    clippy::unused_trait_names,
    reason = "Leptos component macro expansion generates builders, fields, and bindings with framework-defined shapes"
)]

#[allow(unused_import_braces, reason = "grouped Leptos prelude imports are required by workspace source policy")]
#[rustfmt::skip]
use leptos::prelude::{AddAnyAttr};

#[derive(optml::Optml, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) enum AdminAlertVariant {
    #[default]
    Error,
    #[cfg(not(target_arch = "wasm32"))]
    Success,
}

impl AdminAlertVariant {
    fn class(self) -> &'static str {
        match self {
            Self::Error => {
                "ui-alert field-error relative w-full rounded-lg border px-4 py-3 text-sm [&>svg+div]:-translate-y-[3px] [&>svg]:absolute [&>svg]:left-4 [&>svg]:top-4 [&>svg]:text-foreground [&>svg~*]:pl-7"
            }
            #[cfg(not(target_arch = "wasm32"))]
            Self::Success => {
                "ui-alert flash-success relative w-full rounded-lg border px-4 py-3 text-sm [&>svg+div]:-translate-y-[3px] [&>svg]:absolute [&>svg]:left-4 [&>svg]:top-4 [&>svg]:text-foreground [&>svg~*]:pl-7"
            }
        }
    }

    fn role(self) -> &'static str {
        match self {
            Self::Error => str_constants::HTML_ALERT_ROLE,
            #[cfg(not(target_arch = "wasm32"))]
            Self::Success => str_constants::HTML_STATUS_ROLE,
        }
    }
}

#[leptos::component]
#[allow(
    unreachable_pub,
    reason = "Leptos component visibility is required for composition across frontend modules"
)]
pub(crate) fn AdminAlert(
    #[prop(default = AdminAlertVariant::default())] variant: AdminAlertVariant,
    #[prop(optional)] id: Option<&'static str>,
    children: leptos::prelude::Children,
) -> impl leptos::prelude::IntoView {
    leptos::view! {
        <singlestage::Alert attr:data-name="Alert" id=id.map(String::from) attr:class=variant.class() role=variant.role()>{children()}</singlestage::Alert>
    }
}
