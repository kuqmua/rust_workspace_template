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

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Default, PartialEq, Eq)]
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
        <singlestage::Card attr:data-name="Card" attr:data-size="default" attr:class=variant.class()>
            <singlestage::CardContent attr:data-name="CardContent" attr:class="px-6">{children()}</singlestage::CardContent>
        </singlestage::Card>
    }
}

#[leptos::component]
#[allow(
    unreachable_pub,
    reason = "Leptos component visibility is required for card composition"
)]
pub(crate) fn AdminCardHeader(
    children: leptos::prelude::Children,
) -> impl leptos::prelude::IntoView {
    leptos::view! { <singlestage::CardHeader attr:data-name="CardHeader">{children()}</singlestage::CardHeader> }
}

#[leptos::component]
#[allow(
    unreachable_pub,
    reason = "Leptos component visibility is required for card composition"
)]
pub(crate) fn AdminCardTitle(
    #[prop(optional)] class: Option<&'static str>,
    children: leptos::prelude::Children,
) -> impl leptos::prelude::IntoView {
    leptos::view! { <singlestage::CardTitle attr:data-name="CardTitle" class=class.map(String::from)>{children()}</singlestage::CardTitle> }
}

#[cfg(not(target_arch = "wasm32"))]
#[leptos::component]
#[allow(
    unreachable_pub,
    reason = "Leptos component visibility is required for card composition"
)]
pub(crate) fn AdminCardDescription(
    children: leptos::prelude::Children,
) -> impl leptos::prelude::IntoView {
    leptos::view! { <singlestage::CardDescription attr:data-name="CardDescription">{children()}</singlestage::CardDescription> }
}

#[leptos::component]
#[allow(
    unreachable_pub,
    reason = "Leptos component visibility is required for card composition"
)]
pub(crate) fn AdminCardFooter(
    children: leptos::prelude::Children,
) -> impl leptos::prelude::IntoView {
    leptos::view! { <singlestage::CardFooter attr:data-name="CardFooter">{children()}</singlestage::CardFooter> }
}
