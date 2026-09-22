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

use leptos::prelude::{AddAnyAttr, ClassAttribute, CustomAttribute, ElementChild};

#[leptos::component]
#[allow(
    clippy::single_call_fn,
    reason = "the shared table action component is invoked through Leptos view macro expansion in CSR and SSR session rows"
)]
pub(crate) fn AdminTableActions(
    read_path: Option<String>,
    command_for: String,
    children: leptos::prelude::Children,
) -> impl leptos::prelude::IntoView {
    leptos::view! {
        <div class="table-actions">
            {read_path.map(|read_path| leptos::view! {
                <singlestage::Link class=crate::admin_button_variant::AdminButtonVariant::Secondary.class() href=read_path attr:aria-label=constants_str::PG_CRUD_READ_PERMISSION_ACTION attr:title=constants_str::PG_CRUD_READ_PERMISSION_ACTION>
                    <svg viewBox="0 0 24 24" aria-hidden=constants_str::TRUE fill="none" stroke="currentColor" stroke-width="2">
                        <path d="M2 12s3.5-7 10-7 10 7 10 7-3.5 7-10 7S2 12 2 12Z"></path>
                        <circle cx="12" cy="12" r="3"></circle>
                    </svg>
                </singlestage::Link>
            })}
            <crate::admin_button::AdminButton admin_button_variant=crate::admin_button_variant::AdminButtonVariant::Secondary admin_button_kind=crate::admin_button_kind::AdminButtonKind::Button command_for=command_for command="show-modal" aria_label=String::from(constants_str::PG_CRUD_DELETE_PERMISSION_ACTION) attr:title=constants_str::PG_CRUD_DELETE_PERMISSION_ACTION>
                <svg viewBox="0 0 24 24" aria-hidden=constants_str::TRUE fill="none" stroke="currentColor" stroke-width="2">
                    <path d="M3 6h18"></path>
                    <path d="M8 6V4h8v2"></path>
                    <path d="M19 6l-1 14H6L5 6"></path>
                    <path d="M10 11v5"></path>
                    <path d="M14 11v5"></path>
                </svg>
            </crate::admin_button::AdminButton>
            {children()}
        </div>
    }
}
