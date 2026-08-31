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

use leptos::prelude::{AddAnyAttr, Callable, ClassAttribute, CustomAttribute, ElementChild};

#[leptos::component]
#[allow(
    unreachable_pub,
    reason = "Leptos component visibility is required for composition across frontend modules"
)]
#[allow(clippy::single_call_fn)] // named UI component or render stage has one composition owner
pub(crate) fn AdminAlertDialog(
    id: String,
    title: &'static str,
    description: &'static str,
    trigger: &'static str,
    confirm: &'static str,
    #[prop(optional)] disabled: bool,
    on_confirm: leptos::prelude::Callback<()>,
) -> impl leptos::prelude::IntoView {
    crate::with_owner::with_owner(move || {
        leptos::view! {
            <singlestage::Dialog alert=true id=id class="w-full max-w-lg rounded-2xl border bg-background p-6 shadow-lg" dialog_trigger=singlestage::DialogTrigger::builder().children(leptos::prelude::ToChildren::to_children(move || leptos::view! {
                <crate::admin_button::AdminButton variant=crate::admin_button_variant::AdminButtonVariant::Danger kind=crate::admin_button_kind::AdminButtonKind::Button disabled=disabled>{trigger}</crate::admin_button::AdminButton>
            })).build()>
            <singlestage::DialogContent attr:data-name="AlertDialogContent" class="flex flex-col gap-4">
                <div data-name="AlertDialogBody" class="contents">
                    <singlestage::DialogHeader attr:data-name="AlertDialogHeader" class="flex flex-col gap-2 text-center sm:text-left">
                        <singlestage::DialogTitle attr:data-name="AlertDialogTitle" class="text-lg leading-none font-semibold">{title}</singlestage::DialogTitle>
                        <singlestage::DialogDescription attr:data-name="AlertDialogDescription" class="text-sm text-muted-foreground">{description}</singlestage::DialogDescription>
                    </singlestage::DialogHeader>
                    <singlestage::DialogFooter attr:data-name="AlertDialogFooter" class="flex flex-col-reverse gap-2 sm:flex-row sm:justify-end">
                        <crate::admin_button::AdminButton variant=crate::admin_button_variant::AdminButtonVariant::Secondary>"Cancel"</crate::admin_button::AdminButton>
                        <crate::admin_button::AdminButton variant=crate::admin_button_variant::AdminButtonVariant::Danger on_click=leptos::prelude::Callback::new(move |_event| on_confirm.run(()))>{confirm}</crate::admin_button::AdminButton>
                    </singlestage::DialogFooter>
                </div>
            </singlestage::DialogContent>
            </singlestage::Dialog>
        }
    })
}
