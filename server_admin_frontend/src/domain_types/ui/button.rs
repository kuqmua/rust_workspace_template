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

use leptos::prelude::{Callable, ClassAttribute, ElementChild, OnAttribute};

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) enum AdminButtonVariant {
    #[default]
    Primary,
    Secondary,
    Danger,
}

impl AdminButtonVariant {
    fn class(self) -> &'static str {
        match self {
            Self::Primary => constants_str::VALUE_82FEF3B0,
            Self::Secondary => constants_str::VALUE_D720672A,
            Self::Danger => constants_str::VALUE_7BE8BA9D,
        }
    }
}

#[leptos::component]
#[allow(
    unreachable_pub,
    reason = "Leptos component visibility is required for composition across frontend modules"
)]
pub(crate) fn AdminButtonLink(
    href: &'static str,
    #[prop(default = AdminButtonVariant::default())] variant: AdminButtonVariant,
    children: leptos::prelude::Children,
) -> impl leptos::prelude::IntoView {
    leptos::view! {
        <singlestage::Link class=variant.class() href=href>{children()}</singlestage::Link>
    }
}

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) enum AdminButtonKind {
    Button,
    #[default]
    Submit,
}

impl AdminButtonKind {
    fn value(self) -> &'static str {
        match self {
            Self::Button => constants_str::VALUE_C3E2D78F,
            Self::Submit => constants_str::VALUE_75490BD7,
        }
    }
}

#[leptos::component]
#[allow(
    unreachable_pub,
    reason = "Leptos component visibility is required for composition across frontend modules"
)]
pub(crate) fn AdminButton(
    #[prop(default = AdminButtonVariant::default())] variant: AdminButtonVariant,
    #[prop(default = AdminButtonKind::default())] kind: AdminButtonKind,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] command_for: Option<String>,
    #[prop(optional)] command: Option<&'static str>,
    #[prop(optional)] popover_target: Option<String>,
    #[prop(optional)] popover_target_action: Option<&'static str>,
    #[prop(optional)] aria_label: Option<String>,
    #[prop(optional)] style: Option<String>,
    #[prop(optional)] form: Option<String>,
    #[prop(optional)] on_click: Option<leptos::prelude::Callback<leptos::ev::MouseEvent>>,
    children: leptos::prelude::Children,
) -> impl leptos::prelude::IntoView {
    match on_click {
        Some(callback) => leptos::prelude::IntoAny::into_any(leptos::view! {
            <span class="contents" on:click=move |event| callback.run(event)><singlestage::Button class=variant.class() button_type=kind.value() disabled=disabled commandfor=command_for command=command.map(String::from) popovertarget=popover_target popovertargetaction=popover_target_action.map(String::from) aria_label=aria_label style=style form=form>{children()}</singlestage::Button></span>
        }),
        None => leptos::prelude::IntoAny::into_any(leptos::view! {
            <singlestage::Button class=variant.class() button_type=kind.value() disabled=disabled commandfor=command_for command=command.map(String::from) popovertarget=popover_target popovertargetaction=popover_target_action.map(String::from) aria_label=aria_label style=style form=form>{children()}</singlestage::Button>
        }),
    }
}
