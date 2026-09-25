#![allow(
    clippy::field_scoped_visibility_modifiers,
    clippy::same_name_method,
    reason = "Leptos component expansion generates framework-defined props fields and builder methods"
)]

#[leptos::component]
#[allow(
    clippy::single_call_fn,
    reason = "the user update action is shared by server-rendered and client-rendered table rows through Leptos view macro expansion"
)]
pub(crate) fn AdminUserUpdateAction() -> impl leptos::prelude::IntoView {
    leptos::view! {
        <crate::admin_button_link::AdminButtonLink str=server_admin_contract::admin_frontend_path::AdminFrontendPath::UsersUpdate.get()>{constants_str::PG_CRUD_UPDATE_RULE_ACTION}</crate::admin_button_link::AdminButtonLink>
    }
}
