#![allow(
    clippy::single_call_fn,
    clippy::unused_trait_names,
    reason = "the SSR role-row view is composed once by the roles screen"
)]

pub(super) fn admin_role_row(
    item: &server_admin_contract::AdminRoleSummary,
    page: &server_admin_contract::AdminRolesPage,
) -> impl leptos::prelude::IntoView + use<> {
    let id = item.id().to_string();
    let name = item.name().to_string();
    let system = item.is_system().to_string();
    let permissions = crate::shared::admin_table_cells::admin_role_permissions(item, page);
    leptos::view! {
        <crate::ui::table::TableRow>
            <crate::ui::table::TableCell data_label="id">{id}</crate::ui::table::TableCell>
            <crate::ui::table::TableCell data_label="name">{name}</crate::ui::table::TableCell>
            <crate::ui::table::TableCell data_label="system">{system}</crate::ui::table::TableCell>
            {permissions}
        </crate::ui::table::TableRow>
    }
}
