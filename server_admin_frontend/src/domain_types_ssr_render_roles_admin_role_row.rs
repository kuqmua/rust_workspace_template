#![allow(
    clippy::single_call_fn,
    clippy::unused_trait_names,
    reason = "the SSR role-row view is composed once by the roles screen"
)]

pub(super) fn admin_role_row(
    item: &server_admin_contract::domain_types::AdminRoleSummary,
    page: &server_admin_contract::domain_types::AdminRolesPage,
) -> impl leptos::prelude::IntoView + use<> {
    let id = item.id().to_string();
    let name = item.name().to_string();
    let system = item.is_system().to_string();
    let permissions =
        crate::domain_types::shared::admin_table_cells::admin_role_permissions(item, page);
    leptos::view! {
        <crate::domain_types::with_owner::table::TableRow>
            <crate::domain_types::with_owner::table::TableCell data_label="id">{id}</crate::domain_types::with_owner::table::TableCell>
            <crate::domain_types::with_owner::table::TableCell data_label="name">{name}</crate::domain_types::with_owner::table::TableCell>
            <crate::domain_types::with_owner::table::TableCell data_label="system">{system}</crate::domain_types::with_owner::table::TableCell>
            {permissions}
        </crate::domain_types::with_owner::table::TableRow>
    }
}
