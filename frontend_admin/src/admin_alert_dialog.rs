#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "Leptos component expansion models props as generated fields even though the source declares functions"
)]
#![allow(
    clippy::arbitrary_source_item_ordering,
    clippy::multiple_inherent_impl,
    clippy::same_name_method,
    clippy::shadow_reuse,
    clippy::unused_trait_names,
    reason = "Leptos component macro expansion generates builders, fields, and bindings with framework-defined shapes"
)]

use leptos::prelude::{
    AddAnyAttr, AriaAttributes, Callable, ClassAttribute, ElementChild, GlobalAttributes,
};

#[leptos::component]
#[allow(
    unreachable_pub,
    reason = "Leptos component visibility is required for composition across frontend modules"
)]
#[allow(
    clippy::single_call_fn,
    reason = "admin alert dialog requires this localized allowance for generated or framework-constrained code verified by focused tests"
)]
pub(crate) fn AdminAlertDialog(
    string: String,
    title: &'static str,
    description: &'static str,
    trigger: &'static str,
    confirm: &'static str,
    #[prop(optional)] bool: bool,
    callback: leptos::prelude::Callback<()>,
) -> impl leptos::prelude::IntoView {
    crate::with_owner::with_owner(move || {
        if bool {
            return leptos::prelude::IntoAny::into_any(leptos::view! {
                <crate::admin_button::AdminButton admin_button_variant=crate::admin_button_variant::AdminButtonVariant::Danger admin_button_kind=crate::admin_button_kind::AdminButtonKind::Button bool=true>{trigger}</crate::admin_button::AdminButton>
            });
        }
        leptos::prelude::IntoAny::into_any(leptos::view! {
            <crate::admin_button::AdminButton admin_button_variant=crate::admin_button_variant::AdminButtonVariant::Danger admin_button_kind=crate::admin_button_kind::AdminButtonKind::Button attr:commandfor=string.clone() attr:command="show-modal">{trigger}</crate::admin_button::AdminButton>
            <dialog id=string class="singlestage-dialog" aria-label=title aria-description=description>
                <form method="dialog">
                    <crate::admin_button::AdminButton admin_button_variant=crate::admin_button_variant::AdminButtonVariant::Danger on_click=leptos::prelude::Callback::new(move |_event| callback.run(()))>{confirm}</crate::admin_button::AdminButton>
                    <crate::admin_button::AdminButton admin_button_variant=crate::admin_button_variant::AdminButtonVariant::Secondary>{constants_str::ADMIN_BUTTON_CANCEL}</crate::admin_button::AdminButton>
                </form>
            </dialog>
        })
    })
}
