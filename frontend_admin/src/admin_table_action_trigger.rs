#![allow(
    clippy::field_scoped_visibility_modifiers,
    clippy::same_name_method,
    reason = "Leptos component expansion generates framework-defined props fields and builder methods"
)]

use leptos::prelude::AddAnyAttr;

#[leptos::component]
#[allow(
    clippy::single_call_fn,
    reason = "the table action trigger is shared by read dialogs and update links through Leptos view macro expansion"
)]
pub(crate) fn AdminTableActionTrigger(
    label: &'static str,
    #[prop(optional)] dialog_id: String,
    #[prop(optional)] href: Option<server_admin_contract::admin_route_path::AdminRoutePath>,
    children: leptos::prelude::Children,
) -> impl leptos::prelude::IntoView {
    match href {
        Some(value) => leptos::prelude::IntoAny::into_any(leptos::view! {
            <singlestage::Link href=value.to_string() class=crate::admin_button_variant::AdminButtonVariant::Secondary.class() attr:aria-label=label attr:title=label>{children()}</singlestage::Link>
        }),
        None => leptos::prelude::IntoAny::into_any(leptos::view! {
            <crate::admin_button::AdminButton admin_button_variant=crate::admin_button_variant::AdminButtonVariant::Secondary admin_button_kind=crate::admin_button_kind::AdminButtonKind::Button command_for=dialog_id command="show-modal" aria_label=String::from(label) attr:title=label>{children()}</crate::admin_button::AdminButton>
        }),
    }
}
