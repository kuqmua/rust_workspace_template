#![allow(
    clippy::field_scoped_visibility_modifiers,
    clippy::same_name_method,
    reason = "Leptos component expansion generates framework-defined props fields and builder methods"
)]

#[leptos::component]
#[allow(
    clippy::single_call_fn,
    reason = "the role update action is shared by server-rendered and client-rendered table rows through Leptos view macro expansion"
)]
pub(crate) fn AdminRoleUpdateAction(
    admin_role_id: server_admin_contract::admin_role_id::AdminRoleId,
) -> impl leptos::prelude::IntoView {
    leptos::view! {
        <crate::admin_update_action::AdminUpdateAction admin_route_path=server_admin_contract::admin_route_path::AdminRoutePath::role_update_path(admin_role_id) />
    }
}
