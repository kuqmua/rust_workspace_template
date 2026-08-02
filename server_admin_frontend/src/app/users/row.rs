use leptos::prelude::{ClassAttribute, CustomAttribute, ElementChild};

pub(super) fn admin_user_row(
    item: &server_admin_contract::AdminUserSummary,
    page: &server_admin_contract::AdminUsersPage,
) -> impl leptos::prelude::IntoView + use<> {
    leptos::view! {
        <tr data-name="TableRow" class="border-b transition-colors data-[state=selected]:bg-muted hover:bg-muted/50">
            <td data-name="TableCell" class="p-4 align-middle [&:has([role=checkbox])]:pr-0 [&:has([role=checkbox])]:pl-3" data-label="id">{item.id().to_string()}</td>
            <td data-name="TableCell" class="p-4 align-middle [&:has([role=checkbox])]:pr-0 [&:has([role=checkbox])]:pl-3" data-label="login">{item.login().to_string()}</td>
            <td data-name="TableCell" class="p-4 align-middle [&:has([role=checkbox])]:pr-0 [&:has([role=checkbox])]:pl-3" data-label="display_name">{item.display_name().to_string()}</td>
            <td data-name="TableCell" class="p-4 align-middle [&:has([role=checkbox])]:pr-0 [&:has([role=checkbox])]:pl-3" data-label="banned">{item.is_banned().to_string()}</td>
            {crate::shared::admin_table_cells::admin_user_roles(item, page)}
        </tr>
    }
}
