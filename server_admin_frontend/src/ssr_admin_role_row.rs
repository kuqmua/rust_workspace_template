#![allow(
    clippy::single_call_fn,
    clippy::unused_trait_names,
    reason = "the SSR role-row view is composed once by the roles screen"
)]

pub(super) fn ssr_admin_role_row(
    item: &server_admin_contract::domain_types::AdminRoleSummary,
    page: &server_admin_contract::domain_types::AdminRolesPage,
) -> impl leptos::prelude::IntoView + use<> {
    let id = item.id().to_string();
    let name = item.name().to_string();
    let system = item.is_system().to_string();
    let permissions =
        crate::domain_types::shared::admin_role_permissions::admin_role_permissions(item, page);
    leptos::view! {
        <crate::domain_types::with_owner::tables::table_row::TableRow>
            <crate::domain_types::with_owner::tables::table_cell::TableCell data_label="id">{id}</crate::domain_types::with_owner::tables::table_cell::TableCell>
            <crate::domain_types::with_owner::tables::table_cell::TableCell data_label="name">{name}</crate::domain_types::with_owner::tables::table_cell::TableCell>
            <crate::domain_types::with_owner::tables::table_cell::TableCell data_label="system">{system}</crate::domain_types::with_owner::tables::table_cell::TableCell>
            {permissions}
        </crate::domain_types::with_owner::tables::table_row::TableRow>
    }
}
