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
pub(crate) enum AdminBadgeVariant {
    #[default]
    Neutral,
    Success,
}

impl AdminBadgeVariant {
    fn class(self) -> &'static str {
        match self {
            Self::Neutral => {
                "ui-badge ui-badge-neutral inline-flex w-fit items-center rounded-md border border-transparent bg-muted px-2.5 py-0.5 text-xs font-semibold text-muted-foreground transition-colors hover:bg-muted/80 focus:outline-hidden focus:ring-2 focus:ring-ring focus:ring-offset-2"
            }
            Self::Success => {
                "ui-badge ui-badge-success inline-flex w-fit items-center rounded-md border border-transparent bg-success-light px-2.5 py-0.5 text-xs font-semibold text-success-dark transition-colors hover:bg-success-light/80 focus:outline-hidden focus:ring-2 focus:ring-ring focus:ring-offset-2"
            }
        }
    }
}

#[leptos::component]
#[allow(
    unreachable_pub,
    reason = "Leptos component visibility is required for composition across frontend modules"
)]
pub(crate) fn AdminBadge(
    #[prop(default = AdminBadgeVariant::default())] variant: AdminBadgeVariant,
    children: leptos::prelude::Children,
) -> impl leptos::prelude::IntoView {
    leptos::view! {
        <singlestage::Badge attr:data-name="Badge" attr:class=variant.class()>{children()}</singlestage::Badge>
    }
}
