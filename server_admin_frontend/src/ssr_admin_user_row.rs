#![allow(
    clippy::single_call_fn,
    clippy::unused_trait_names,
    reason = "the SSR user-row view is composed once by the users screen"
)]

pub(super) fn ssr_admin_user_row(
    item: &server_admin_contract::domain_types::AdminUserSummary,
    page: &server_admin_contract::domain_types::AdminUsersPage,
) -> impl leptos::prelude::IntoView + use<> {
    let id = item.id().to_string();
    let login = item.login().to_string();
    let display_name = item.display_name().to_string();
    let banned = item.is_banned().to_string();
    let roles = crate::domain_types::shared::admin_user_roles::admin_user_roles(item, page);
    leptos::view! {
        <crate::domain_types::with_owner::tables::table_row::TableRow>
            <crate::domain_types::with_owner::tables::table_cell::TableCell data_label="id">{id}</crate::domain_types::with_owner::tables::table_cell::TableCell>
            <crate::domain_types::with_owner::tables::table_cell::TableCell data_label="login">{login}</crate::domain_types::with_owner::tables::table_cell::TableCell>
            <crate::domain_types::with_owner::tables::table_cell::TableCell data_label="display_name">{display_name}</crate::domain_types::with_owner::tables::table_cell::TableCell>
            <crate::domain_types::with_owner::tables::table_cell::TableCell data_label="banned">{banned}</crate::domain_types::with_owner::tables::table_cell::TableCell>
            {roles}
        </crate::domain_types::with_owner::tables::table_row::TableRow>
    }
}
