#![allow(
    clippy::single_call_fn,
    clippy::unused_trait_names,
    reason = "CSR and SSR targets each compile one call site and Leptos cell rendering requires attribute traits"
)]

pub(crate) fn admin_role_permissions(
    item: &server_admin_contract::domain_types::AdminRoleSummary,
    page: &server_admin_contract::domain_types::AdminRolesPage,
) -> impl leptos::prelude::IntoView + use<> {
    let names = String::from(super::text::join_text(
        page.permissions()
            .iter()
            .filter(|permission| item.permission_ids().contains(&permission.id()))
            .map(server_admin_contract::domain_types::AdminPermissionSummary::name)
            .map(|name| name.as_ref().as_str()),
    ));
    leptos::view! { <crate::domain_types::with_owner::table::table_cell::TableCell data_label="permissions">{names}</crate::domain_types::with_owner::table::table_cell::TableCell> }
}
