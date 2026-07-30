#![allow(
    clippy::single_call_fn,
    clippy::unused_trait_names,
    reason = "the SSR user-row view is composed once by the users screen"
)]

mod actions;
mod roles;

use leptos::prelude::{CustomAttribute, ElementChild};

pub(super) fn admin_user_row(
    item: &server_admin_contract::AdminUserSummary,
    page: &server_admin_contract::AdminUsersPage,
    can_delete: server_admin_contract::AdminBool,
    can_update: server_admin_contract::AdminBool,
    can_update_roles: server_admin_contract::AdminBool,
) -> impl leptos::prelude::IntoView + use<> {
    leptos::view! {
        <tr>
            <td data-label="id">{item.id().to_string()}</td>
            <td data-label="login">{item.login().to_string()}</td>
            <td data-label="display_name">{item.display_name().to_string()}</td>
            <td data-label="banned">{item.is_banned().to_string()}</td>
            {roles::admin_user_roles(item, page, can_update_roles)}
            {actions::admin_user_actions(item, can_delete, can_update)}
        </tr>
    }
}
