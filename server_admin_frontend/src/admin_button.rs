#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "Leptos component expansion models props as generated fields even though the source declares functions"
)]
#![allow(
    clippy::arbitrary_source_item_ordering,
    reason = "component props and wire enum variants retain their semantic presentation order"
)]
#![allow(
    clippy::multiple_inherent_impl,
    clippy::same_name_method,
    clippy::shadow_reuse,
    clippy::unused_trait_names,
    reason = "Leptos component macro expansion generates builders, fields, and bindings with framework-defined shapes"
)]

use leptos::prelude::{Callable, ClassAttribute, ElementChild, OnAttribute};

#[leptos::component]
#[allow(
    unreachable_pub,
    reason = "Leptos component visibility is required for composition across frontend modules"
)]
#[allow(clippy::single_call_fn)] // named UI component or render stage has one composition owner
pub(crate) fn AdminButton(
    #[prop(default = crate::admin_button_variant::AdminButtonVariant::default())]
    admin_button_variant: crate::admin_button_variant::AdminButtonVariant,
    #[prop(default = crate::admin_button_kind::AdminButtonKind::default())]
    admin_button_kind: crate::admin_button_kind::AdminButtonKind,
    #[prop(optional)] bool: bool,
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
            <span class="contents" on:click=move |event| callback.run(event)><singlestage::Button class=admin_button_variant.class() button_type=admin_button_kind.value() disabled=bool commandfor=command_for command=command.map(String::from) popovertarget=popover_target popovertargetaction=popover_target_action.map(String::from) aria_label=aria_label style=style form=form>{children()}</singlestage::Button></span>
        }),
        None => leptos::prelude::IntoAny::into_any(leptos::view! {
            <singlestage::Button class=admin_button_variant.class() button_type=admin_button_kind.value() disabled=bool commandfor=command_for command=command.map(String::from) popovertarget=popover_target popovertargetaction=popover_target_action.map(String::from) aria_label=aria_label style=style form=form>{children()}</singlestage::Button>
        }),
    }
}
