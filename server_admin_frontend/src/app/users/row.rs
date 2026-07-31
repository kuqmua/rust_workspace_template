use leptos::prelude::{CustomAttribute, ElementChild};

pub(super) fn admin_user_row(
    item: &server_admin_contract::AdminUserSummary,
    page: &server_admin_contract::AdminUsersPage,
) -> impl leptos::prelude::IntoView + use<> {
    leptos::view! {
        <tr>
            <td data-label="id">{item.id().to_string()}</td>
            <td data-label="login">{item.login().to_string()}</td>
            <td data-label="display_name">{item.display_name().to_string()}</td>
            <td data-label="banned">{item.is_banned().to_string()}</td>
            {crate::shared::admin_table_cells::admin_user_roles(item, page)}
        </tr>
    }
}
