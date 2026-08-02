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

use leptos::prelude::{ClassAttribute, CustomAttribute, ElementChild};

#[derive(optml::Optml, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) enum AdminCardVariant {
    #[default]
    Default,
    #[cfg(not(target_arch = "wasm32"))]
    Auth,
    #[cfg(not(target_arch = "wasm32"))]
    Code,
    Profile,
    Security,
    Settings,
}

impl AdminCardVariant {
    fn class(self) -> &'static str {
        match self {
            Self::Default => {
                "ui-card flex flex-col gap-4 rounded-xl border bg-card py-6 text-card-foreground shadow-sm"
            }
            #[cfg(not(target_arch = "wasm32"))]
            Self::Auth => {
                "ui-card auth-card flex flex-col gap-4 rounded-xl border bg-card py-6 text-card-foreground shadow-sm"
            }
            #[cfg(not(target_arch = "wasm32"))]
            Self::Code => {
                "ui-card code-card flex flex-col gap-4 rounded-xl border bg-card py-6 text-card-foreground shadow-sm"
            }
            Self::Profile => {
                "ui-card profile-card flex flex-col gap-4 rounded-xl border bg-card py-6 text-card-foreground shadow-sm"
            }
            Self::Security => {
                "ui-card security-card flex flex-col gap-4 rounded-xl border bg-card py-6 text-card-foreground shadow-sm"
            }
            Self::Settings => {
                "ui-card settings-card flex flex-col gap-4 rounded-xl border bg-card py-6 text-card-foreground shadow-sm"
            }
        }
    }
}

#[leptos::component]
#[allow(
    unreachable_pub,
    reason = "Leptos component visibility is required for composition across frontend modules"
)]
pub(crate) fn AdminCard(
    #[prop(default = AdminCardVariant::default())] variant: AdminCardVariant,
    children: leptos::prelude::Children,
) -> impl leptos::prelude::IntoView {
    leptos::view! {
        <div data-name="Card" data-size="default" class=variant.class()>
            <div data-name="CardContent" class="px-6">{children()}</div>
        </div>
    }
}
