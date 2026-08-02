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

use leptos::prelude::{
    AriaAttributes, Callable, ClassAttribute, CustomAttribute, ElementChild, GlobalAttributes,
};

#[leptos::component]
#[allow(
    unreachable_pub,
    reason = "Leptos component visibility is required for composition across frontend modules"
)]
pub(crate) fn AdminAlertDialog(
    id: String,
    title: &'static str,
    description: &'static str,
    trigger: &'static str,
    confirm: &'static str,
    #[prop(optional)] disabled: bool,
    on_confirm: leptos::prelude::Callback<()>,
) -> impl leptos::prelude::IntoView {
    let cancel_id = id.clone();
    let confirm_id = id.clone();
    leptos::view! {
        <crate::ui::button::AdminButton variant=crate::ui::button::AdminButtonVariant::Danger kind=crate::ui::button::AdminButtonKind::Button disabled=disabled command_for=id.clone() command="show-modal">{trigger}</crate::ui::button::AdminButton>
            <dialog data-name="AlertDialogContent" id=id class="w-full max-w-lg rounded-2xl border bg-background p-6 shadow-lg backdrop:bg-black/50" aria-label=title>
                <div data-name="AlertDialogBody" class="flex flex-col gap-4">
                    <div data-name="AlertDialogHeader" class="flex flex-col gap-2 text-center sm:text-left">
                        <h3 data-name="AlertDialogTitle" class="text-lg leading-none font-semibold">{title}</h3>
                        <p data-name="AlertDialogDescription" class="text-sm text-muted-foreground">{description}</p>
                    </div>
                    <footer data-name="AlertDialogFooter" class="flex flex-col-reverse gap-2 sm:flex-row sm:justify-end">
                        <crate::ui::button::AdminButton variant=crate::ui::button::AdminButtonVariant::Secondary kind=crate::ui::button::AdminButtonKind::Button command_for=cancel_id command="close">"Cancel"</crate::ui::button::AdminButton>
                        <crate::ui::button::AdminButton variant=crate::ui::button::AdminButtonVariant::Danger kind=crate::ui::button::AdminButtonKind::Button command_for=confirm_id command="close" on_click=leptos::prelude::Callback::new(move |_event| on_confirm.run(()))>{confirm}</crate::ui::button::AdminButton>
                    </footer>
                </div>
            </dialog>
    }
}
