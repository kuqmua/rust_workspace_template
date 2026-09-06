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
pub(crate) fn AdminUsersView(
    authenticated_admin: server_admin_contract::authenticated_admin::AuthenticatedAdmin,
    admin_users_page: server_admin_contract::admin_users_page::AdminUsersPage,
    admin_csr_query: super::admin_csr_query::AdminCsrQuery,
) -> impl leptos::prelude::IntoView {
    let csr_admin_user_row =
        |admin_user_summary: &server_admin_contract::admin_user_summary::AdminUserSummary| {
            let id = admin_user_summary.id().to_string();
            let login = admin_user_summary.login().to_string();
            let display_name = admin_user_summary.display_name().to_string();
            let banned = admin_user_summary.is_banned().to_string();
            let roles =
                crate::admin_user_roles::admin_user_roles(admin_user_summary, &admin_users_page);
            leptos::view! {
                <crate::table_row::TableRow>
                    <crate::table_cell::TableCell data_label="id">{id}</crate::table_cell::TableCell>
                    <crate::table_cell::TableCell data_label="login">{login}</crate::table_cell::TableCell>
                    <crate::table_cell::TableCell data_label="display_name">{display_name}</crate::table_cell::TableCell>
                    <crate::table_cell::TableCell data_label="banned">{banned}</crate::table_cell::TableCell>
                    {roles}
                </crate::table_row::TableRow>
            }
        };

    let can_create = bool::from(
        authenticated_admin
            .has_permission(server_admin_contract::admin_permission::AdminPermission::UsersCreate),
    );
    let can_manage = bool::from(
        authenticated_admin
            .has_permission(server_admin_contract::admin_permission::AdminPermission::UsersUpdate),
    ) || bool::from(
        authenticated_admin
            .has_permission(server_admin_contract::admin_permission::AdminPermission::UsersDelete),
    );
    let total = admin_users_page.total();
    let rows = admin_users_page
        .items()
        .iter()
        .map(csr_admin_user_row)
        .collect::<Vec<_>>();
    leptos::view! {
        <section class="table-admin_users_page" data-renderer="csr">
            <div class="resource-actions">
                {can_create.then(|| leptos::view! { <crate::admin_button_link::AdminButtonLink str=server_admin_contract::admin_frontend_path::AdminFrontendPath::UsersCreate.get()>{constants_str::ADMIN_BUTTON_CREATE_USER}</crate::admin_button_link::AdminButtonLink> })}
                {can_manage.then(|| leptos::view! { <crate::admin_button_link::AdminButtonLink str=server_admin_contract::admin_frontend_path::AdminFrontendPath::UsersManage.get() admin_button_variant=crate::admin_button_variant::AdminButtonVariant::Secondary>{constants_str::ADMIN_BUTTON_MANAGE_USERS}</crate::admin_button_link::AdminButtonLink> })}
            </div>
            <crate::table_wrapper::TableWrapper><crate::table::Table><crate::table_header::TableHeader><crate::table_row::TableRow><crate::table_head::TableHead>"id"</crate::table_head::TableHead><crate::table_head::TableHead>"login"</crate::table_head::TableHead><crate::table_head::TableHead>"display_name"</crate::table_head::TableHead><crate::table_head::TableHead>"banned"</crate::table_head::TableHead><crate::table_head::TableHead>"roles"</crate::table_head::TableHead></crate::table_row::TableRow></crate::table_header::TableHeader>
            <crate::table_body::TableBody>{rows}</crate::table_body::TableBody></crate::table::Table></crate::table_wrapper::TableWrapper>
            <super::admin_pagination::AdminPagination admin_frontend_path=server_admin_contract::admin_frontend_path::AdminFrontendPath::Users admin_csr_query=admin_csr_query admin_page_total=total />
        </section>
    }
}
