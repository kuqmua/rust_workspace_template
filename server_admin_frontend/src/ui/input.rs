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

use leptos::prelude::{BindAttribute, ClassAttribute, CustomAttribute};

#[derive(optml::Optml, Clone, Debug, PartialEq, Eq)]
pub(crate) struct AdminInputName(Box<str>);

impl From<&'static str> for AdminInputName {
    fn from(value: &'static str) -> Self {
        Self(Box::<str>::from(value))
    }
}

impl From<server_admin_contract::AdminSettingName> for AdminInputName {
    fn from(value: server_admin_contract::AdminSettingName) -> Self {
        Self(value.as_ref().to_owned().into_boxed_str())
    }
}

impl AsRef<str> for AdminInputName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[derive(optml::Optml, Clone, Copy, Debug, newtype::FromInner)]
pub(crate) struct LeptosAdminInputSignal(leptos::prelude::RwSignal<String>);

impl LeptosAdminInputSignal {
    pub(crate) const fn signal(self) -> leptos::prelude::RwSignal<String> {
        self.0
    }
}

#[derive(optml::Optml, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) enum AdminInputKind {
    #[default]
    Text,
    Password,
    Number,
    Url,
}

impl AdminInputKind {
    fn value(self) -> &'static str {
        match self {
            Self::Text => "text",
            Self::Password => "password",
            Self::Number => "number",
            Self::Url => "url",
        }
    }
}

#[leptos::component]
#[allow(
    unreachable_pub,
    reason = "Leptos component visibility is required for composition across frontend modules"
)]
pub(crate) fn AdminInput(
    #[prop(into)] name: AdminInputName,
    #[prop(default = AdminInputKind::default())] kind: AdminInputKind,
    #[prop(optional)] autocomplete: Option<&'static str>,
    #[prop(optional)] required: bool,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] minlength: Option<usize>,
    #[prop(optional)] maxlength: Option<usize>,
    #[prop(optional)] min: Option<u16>,
    #[prop(optional)] max: Option<u16>,
    #[prop(optional)] initial_value: Option<String>,
    #[prop(optional, into)] bind_value: Option<LeptosAdminInputSignal>,
) -> impl leptos::prelude::IntoView {
    let input_type = kind.value();
    match bind_value {
        Some(value) => leptos::prelude::IntoAny::into_any(leptos::view! {
            <input data-name="Input" class="ui-input flex h-9 w-full min-w-0 rounded-md border border-input bg-transparent px-3 py-1 text-base text-foreground shadow-xs outline-none transition-[color,box-shadow] file:inline-flex file:h-7 file:border-0 file:bg-transparent file:text-sm file:font-medium file:text-foreground placeholder:text-muted-foreground selection:bg-primary selection:text-primary-foreground read-only:bg-muted focus-visible:border-ring focus-visible:ring-2 focus-visible:ring-ring/50 aria-invalid:border-destructive aria-invalid:ring-destructive/20 disabled:pointer-events-none disabled:cursor-not-allowed disabled:opacity-50 dark:bg-input/30 dark:aria-invalid:ring-destructive/40 md:text-sm" name=name.as_ref() type=input_type autocomplete=autocomplete required=required disabled=disabled minlength=minlength maxlength=maxlength min=min max=max value=initial_value bind:value=value.signal() />
        }),
        None => leptos::prelude::IntoAny::into_any(leptos::view! {
            <input data-name="Input" class="ui-input flex h-9 w-full min-w-0 rounded-md border border-input bg-transparent px-3 py-1 text-base text-foreground shadow-xs outline-none transition-[color,box-shadow] file:inline-flex file:h-7 file:border-0 file:bg-transparent file:text-sm file:font-medium file:text-foreground placeholder:text-muted-foreground selection:bg-primary selection:text-primary-foreground read-only:bg-muted focus-visible:border-ring focus-visible:ring-2 focus-visible:ring-ring/50 aria-invalid:border-destructive aria-invalid:ring-destructive/20 disabled:pointer-events-none disabled:cursor-not-allowed disabled:opacity-50 dark:bg-input/30 dark:aria-invalid:ring-destructive/40 md:text-sm" name=name.as_ref() type=input_type autocomplete=autocomplete required=required disabled=disabled minlength=minlength maxlength=maxlength min=min max=max value=initial_value />
        }),
    }
}
