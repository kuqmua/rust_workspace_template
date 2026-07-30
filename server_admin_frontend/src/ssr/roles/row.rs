#![allow(
    clippy::single_call_fn,
    clippy::unused_trait_names,
    reason = "the SSR role-row view is composed once by the roles screen"
)]

mod actions;
mod permissions;

use leptos::prelude::{CustomAttribute, ElementChild};

pub(super) fn admin_role_row(
    item: &server_admin_contract::AdminRoleSummary,
    page: &server_admin_contract::AdminRolesPage,
    can_delete: server_admin_contract::AdminBool,
    can_update: server_admin_contract::AdminBool,
    can_update_permissions: server_admin_contract::AdminBool,
) -> impl leptos::prelude::IntoView + use<> {
    leptos::view! {
        <tr>
            <td data-label="id">{item.id().to_string()}</td>
            <td data-label="name">{item.name().to_string()}</td>
            <td data-label="system">{item.is_system().to_string()}</td>
            {permissions::admin_role_permissions(item, page, can_update_permissions)}
            {actions::admin_role_actions(item, can_delete, can_update)}
        </tr>
    }
}
