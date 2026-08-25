pub(super) fn admin_user_row(
    item: &server_admin_contract::domain_types::AdminUserSummary,
    page: &server_admin_contract::domain_types::AdminUsersPage,
) -> impl leptos::prelude::IntoView + use<> {
    let id = item.id().to_string();
    let login = item.login().to_string();
    let display_name = item.display_name().to_string();
    let banned = item.is_banned().to_string();
    let roles = crate::domain_types::shared::admin_table_cells::admin_user_roles(item, page);
    leptos::view! {
        <crate::domain_types::ui::table::TableRow>
            <crate::domain_types::ui::table::TableCell data_label="id">{id}</crate::domain_types::ui::table::TableCell>
            <crate::domain_types::ui::table::TableCell data_label="login">{login}</crate::domain_types::ui::table::TableCell>
            <crate::domain_types::ui::table::TableCell data_label="display_name">{display_name}</crate::domain_types::ui::table::TableCell>
            <crate::domain_types::ui::table::TableCell data_label="banned">{banned}</crate::domain_types::ui::table::TableCell>
            {roles}
        </crate::domain_types::ui::table::TableRow>
    }
}
