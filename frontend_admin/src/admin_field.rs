#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "Leptos component expansion models props as generated fields even though the source declares functions"
)]
#![allow(
    clippy::arbitrary_source_item_ordering,
    reason = "component props and wire enum variants retain their semantic presentation order"
)]
#![allow(
    clippy::impl_trait_in_params,
    clippy::multiple_inherent_impl,
    clippy::same_name_method,
    clippy::shadow_reuse,
    clippy::unused_trait_names,
    reason = "Leptos component macro expansion generates builders, fields, and bindings with framework-defined shapes"
)]

use leptos::prelude::{AddAnyAttr, ElementChild};

#[leptos::component]
#[allow(
    unreachable_pub,
    reason = "Leptos component visibility is required for composition across frontend modules"
)]
#[allow(
    clippy::single_call_fn,
    reason = "admin field remains a named owner because its boundary role is clearer and directly testable"
)]
pub(crate) fn AdminField(
    #[prop(into)] admin_field_label: crate::admin_field_label::AdminFieldLabel,
    children: leptos::prelude::Children,
) -> impl leptos::prelude::IntoView {
    let admin_field_label = String::from(admin_field_label.into_inner());
    crate::with_owner::with_owner(move || {
        leptos::view! {
            <singlestage::Field attr:data-name="Field" class="ui-field group/field flex w-full flex-col gap-3 data-[invalid=true]:text-destructive [&>*]:w-full [&>.hidden]:w-auto">
                <singlestage::FieldLabel attr:data-name="Label" class="group/field-label peer/field-label flex w-fit flex-col gap-2 text-sm font-medium leading-snug group-data-[disabled=true]/field:opacity-50"><span>{admin_field_label}</span>{children()}</singlestage::FieldLabel>
            </singlestage::Field>
        }
    })
}
