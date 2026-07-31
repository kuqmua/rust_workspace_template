#![allow(
    clippy::single_call_fn,
    clippy::unused_trait_names,
    reason = "CSR and SSR targets each compile one call site and Leptos cell rendering requires attribute traits in lexical scope"
)]

use leptos::prelude::{CustomAttribute, ElementChild};

pub(crate) fn admin_user_roles(
    item: &server_admin_contract::AdminUserSummary,
    page: &server_admin_contract::AdminUsersPage,
) -> impl leptos::prelude::IntoView + use<> {
    let names = page
        .roles()
        .iter()
        .filter(|role| item.role_ids().contains(&role.id()))
        .map(|role| role.name().to_string())
        .collect::<Vec<_>>()
        .join(str_constants::COMMA_SPACE);
    leptos::view! { <td data-label="roles">{names}</td> }
}

pub(crate) fn admin_role_permissions(
    item: &server_admin_contract::AdminRoleSummary,
    page: &server_admin_contract::AdminRolesPage,
) -> impl leptos::prelude::IntoView + use<> {
    let names = page
        .permissions()
        .iter()
        .filter(|permission| item.permission_ids().contains(&permission.id()))
        .map(|permission| permission.name().to_string())
        .collect::<Vec<_>>()
        .join(str_constants::COMMA_SPACE);
    leptos::view! { <td data-label="permissions">{names}</td> }
}
