use super::{AdminCrudPage, crud_page};

pub(crate) async fn crud_resource_page(
    auth: crate::AdminAuthReq,
    page: AdminCrudPage,
) -> axum::response::Response {
    match page {
        AdminCrudPage::UserCreate => {
            crud_page(
                auth,
                &[server_admin_contract::domain_types::AdminPermission::UsersCreate],
                async |_auth| Ok(()),
                |_view, admin, branding| {
                    server_admin_frontend::domain_types::ssr::render_user_create(admin, branding)
                },
            )
            .await
        }
        AdminCrudPage::UserManage => {
            crud_page(
                auth,
                &[
                    server_admin_contract::domain_types::AdminPermission::UsersUpdate,
                    server_admin_contract::domain_types::AdminPermission::UsersDelete,
                ],
                |auth| {
                    crate::queries_users_page::queries_users_page(
                        auth,
                        crate::AxumAdminQuery(
                            server_admin_contract::domain_types::AdminTableQuery::default(),
                        ),
                    )
                },
                server_admin_frontend::domain_types::ssr::render_user_manage,
            )
            .await
        }
        AdminCrudPage::RoleCreate => {
            crud_page(
                auth,
                &[server_admin_contract::domain_types::AdminPermission::RolesCreate],
                async |_auth| Ok(()),
                |_view, admin, branding| {
                    server_admin_frontend::domain_types::ssr::render_role_create(admin, branding)
                },
            )
            .await
        }
        AdminCrudPage::RoleManage => {
            crud_page(
                auth,
                &[
                    server_admin_contract::domain_types::AdminPermission::RolesUpdate,
                    server_admin_contract::domain_types::AdminPermission::RolesDelete,
                ],
                |auth| {
                    crate::queries_roles_page::queries_roles_page(
                        auth,
                        crate::AxumAdminQuery(
                            server_admin_contract::domain_types::AdminTableQuery::default(),
                        ),
                    )
                },
                server_admin_frontend::domain_types::ssr::render_role_manage,
            )
            .await
        }
    }
}
