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

use leptos::prelude::{ClassAttribute, CustomAttribute, ElementChild};

#[derive(optml::Optml, Clone, Debug, PartialEq, Eq)]
pub(crate) struct AdminFieldLabel(Box<str>);

impl From<&'static str> for AdminFieldLabel {
    fn from(value: &'static str) -> Self {
        Self(Box::<str>::from(value))
    }
}

impl From<String> for AdminFieldLabel {
    fn from(value: String) -> Self {
        Self(value.into_boxed_str())
    }
}

impl AsRef<str> for AdminFieldLabel {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[leptos::component]
#[allow(
    unreachable_pub,
    reason = "Leptos component visibility is required for composition across frontend modules"
)]
pub(crate) fn AdminField(
    #[prop(into)] label: AdminFieldLabel,
    children: leptos::prelude::Children,
) -> impl leptos::prelude::IntoView {
    let label = String::from(label.0);
    leptos::view! {
        <div data-name="Field" class="ui-field group/field flex w-full flex-col gap-3 data-[invalid=true]:text-destructive [&>*]:w-full [&>.hidden]:w-auto">
            <label data-name="Label" class="group/field-label peer/field-label flex w-fit flex-col gap-2 text-sm font-medium leading-snug group-data-[disabled=true]/field:opacity-50"><span>{label}</span>{children()}</label>
        </div>
    }
}
