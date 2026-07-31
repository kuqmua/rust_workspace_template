#![allow(
    clippy::single_call_fn,
    clippy::unused_trait_names,
    reason = "the SSR role-row view is composed once by the roles screen"
)]

use leptos::prelude::{CustomAttribute, ElementChild};

pub(super) fn admin_role_row(
    item: &server_admin_contract::AdminRoleSummary,
    page: &server_admin_contract::AdminRolesPage,
) -> impl leptos::prelude::IntoView + use<> {
    leptos::view! {
        <tr>
            <td data-label="id">{item.id().to_string()}</td>
            <td data-label="name">{item.name().to_string()}</td>
            <td data-label="system">{item.is_system().to_string()}</td>
            {crate::shared::admin_table_cells::admin_role_permissions(item, page)}
        </tr>
    }
}
