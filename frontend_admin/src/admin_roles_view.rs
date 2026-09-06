#![allow(
    clippy::field_scoped_visibility_modifiers,
    clippy::same_name_method,
    reason = "Leptos emits sibling props fields and builder methods with framework-defined visibility and names from the single component in this module"
)]

use leptos::prelude::{ClassAttribute, CustomAttribute, ElementChild};

#[leptos::component]
#[allow(
    unreachable_pub,
    reason = "Leptos component visibility is required for composition from the shell module"
)]
#[allow(
    clippy::needless_pass_by_value,
    reason = "Leptos props own page data so the generated component factory can move it across reactive render closures"
)]
pub(crate) fn AdminRolesView(
    authenticated_admin: server_admin_contract::authenticated_admin::AuthenticatedAdmin,
    admin_roles_page: server_admin_contract::admin_roles_page::AdminRolesPage,
    admin_csr_query: super::admin_csr_query::AdminCsrQuery,
) -> impl leptos::prelude::IntoView {
    let csr_admin_role_row =
        |admin_role_summary: &server_admin_contract::admin_role_summary::AdminRoleSummary| {
            let id = admin_role_summary.id().to_string();
            let name = admin_role_summary.name().to_string();
            let system = admin_role_summary.is_system().to_string();
            let permissions = crate::admin_role_permissions::admin_role_permissions(
                admin_role_summary,
                &admin_roles_page,
            );
            leptos::view! {
                <crate::table_row::TableRow>
                    <crate::table_cell::TableCell data_label="id">{id}</crate::table_cell::TableCell>
                    <crate::table_cell::TableCell data_label="name">{name}</crate::table_cell::TableCell>
                    <crate::table_cell::TableCell data_label="system">{system}</crate::table_cell::TableCell>
                    {permissions}
                </crate::table_row::TableRow>
            }
        };

    let can_create = bool::from(
        authenticated_admin
            .has_permission(server_admin_contract::admin_permission::AdminPermission::RolesCreate),
    );
    let can_manage = bool::from(
        authenticated_admin
            .has_permission(server_admin_contract::admin_permission::AdminPermission::RolesUpdate),
    ) || bool::from(
        authenticated_admin
            .has_permission(server_admin_contract::admin_permission::AdminPermission::RolesDelete),
    );
    let total = admin_roles_page.total();
    let rows = admin_roles_page
        .items()
        .iter()
        .map(csr_admin_role_row)
        .collect::<Vec<_>>();
    leptos::view! {
        <section class="table-admin_roles_page" data-renderer="csr">
            <div class="resource-actions">
                {can_create.then(|| leptos::view! { <crate::admin_button_link::AdminButtonLink str=server_admin_contract::admin_frontend_path::AdminFrontendPath::RolesCreate.get()>"Create role"</crate::admin_button_link::AdminButtonLink> })}
                {can_manage.then(|| leptos::view! { <crate::admin_button_link::AdminButtonLink str=server_admin_contract::admin_frontend_path::AdminFrontendPath::RolesManage.get() admin_button_variant=crate::admin_button_variant::AdminButtonVariant::Secondary>"Manage roles"</crate::admin_button_link::AdminButtonLink> })}
            </div>
            <crate::table_wrapper::TableWrapper><crate::table::Table><crate::table_header::TableHeader><crate::table_row::TableRow><crate::table_head::TableHead>"id"</crate::table_head::TableHead><crate::table_head::TableHead>"name"</crate::table_head::TableHead><crate::table_head::TableHead>"system"</crate::table_head::TableHead><crate::table_head::TableHead>"permissions"</crate::table_head::TableHead></crate::table_row::TableRow></crate::table_header::TableHeader>
            <crate::table_body::TableBody>{rows}</crate::table_body::TableBody></crate::table::Table></crate::table_wrapper::TableWrapper>
            <super::admin_pagination::AdminPagination admin_frontend_path=server_admin_contract::admin_frontend_path::AdminFrontendPath::Roles admin_csr_query=admin_csr_query admin_page_total=total />
        </section>
    }
}
