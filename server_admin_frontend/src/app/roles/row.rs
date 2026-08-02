use leptos::prelude::{ClassAttribute, CustomAttribute, ElementChild};

pub(super) fn admin_role_row(
    item: &server_admin_contract::AdminRoleSummary,
    page: &server_admin_contract::AdminRolesPage,
) -> impl leptos::prelude::IntoView + use<> {
    leptos::view! {
        <tr data-name="TableRow" class="border-b transition-colors data-[state=selected]:bg-muted hover:bg-muted/50">
            <td data-name="TableCell" class="p-4 align-middle [&:has([role=checkbox])]:pr-0 [&:has([role=checkbox])]:pl-3" data-label="id">{item.id().to_string()}</td>
            <td data-name="TableCell" class="p-4 align-middle [&:has([role=checkbox])]:pr-0 [&:has([role=checkbox])]:pl-3" data-label="name">{item.name().to_string()}</td>
            <td data-name="TableCell" class="p-4 align-middle [&:has([role=checkbox])]:pr-0 [&:has([role=checkbox])]:pl-3" data-label="system">{item.is_system().to_string()}</td>
            {crate::shared::admin_table_cells::admin_role_permissions(item, page)}
        </tr>
    }
}
