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
            Self::Neutral => constants_str::VALUE_5386B853,
            Self::Success => constants_str::VALUE_01AFB233,
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
