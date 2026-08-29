pub(crate) async fn crud_resource_page(
    auth: crate::admin_auth_req::AdminAuthReq,
    page: crate::admin_crud_page::AdminCrudPage,
) -> axum::response::Response {
    match page {
        crate::admin_crud_page::AdminCrudPage::UserCreate => {
            crate::crud_page::crud_page(
                auth,
                &[server_admin_contract::admin_permission::AdminPermission::UsersCreate],
                async |_auth| Ok(()),
                |_view, admin, branding| {
                    server_admin_frontend::render_user_create::render_user_create(admin, branding)
                },
            )
            .await
        }
        crate::admin_crud_page::AdminCrudPage::UserManage => {
            crate::crud_page::crud_page(
                auth,
                &[
                    server_admin_contract::admin_permission::AdminPermission::UsersUpdate,
                    server_admin_contract::admin_permission::AdminPermission::UsersDelete,
                ],
                |auth| {
                    crate::queries_users_page::queries_users_page(
                        auth,
                        crate::axum_admin_query::AxumAdminQuery(
                            server_admin_contract::admin_table_query::AdminTableQuery::default(),
                        ),
                    )
                },
                server_admin_frontend::render_user_manage::render_user_manage,
            )
            .await
        }
        crate::admin_crud_page::AdminCrudPage::RoleCreate => {
            crate::crud_page::crud_page(
                auth,
                &[server_admin_contract::admin_permission::AdminPermission::RolesCreate],
                async |_auth| Ok(()),
                |_view, admin, branding| {
                    server_admin_frontend::render_role_create::render_role_create(admin, branding)
                },
            )
            .await
        }
        crate::admin_crud_page::AdminCrudPage::RoleManage => {
            crate::crud_page::crud_page(
                auth,
                &[
                    server_admin_contract::admin_permission::AdminPermission::RolesUpdate,
                    server_admin_contract::admin_permission::AdminPermission::RolesDelete,
                ],
                |auth| {
                    crate::queries_roles_page::queries_roles_page(
                        auth,
                        crate::axum_admin_query::AxumAdminQuery(
                            server_admin_contract::admin_table_query::AdminTableQuery::default(),
                        ),
                    )
                },
                server_admin_frontend::render_role_manage::render_role_manage,
            )
            .await
        }
    }
}
