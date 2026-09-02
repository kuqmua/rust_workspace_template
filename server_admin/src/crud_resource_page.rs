pub(crate) async fn crud_resource_page(
    admin_auth_request: crate::admin_auth_request::AdminAuthRequest,
    admin_crud_page: crate::admin_crud_page::AdminCrudPage,
) -> axum::response::Response {
    match admin_crud_page {
        crate::admin_crud_page::AdminCrudPage::UserCreate => {
            crate::crud_page::crud_page(
                admin_auth_request,
                &[server_admin_contract::admin_permission::AdminPermission::UsersCreate],
                async |_auth| Ok(()),
                |_view, admin, branding| {
                    frontend::render_user_create::render_user_create(admin, branding)
                },
            )
            .await
        }
        crate::admin_crud_page::AdminCrudPage::UserManage => {
            crate::crud_page::crud_page(
                admin_auth_request,
                &[
                    server_admin_contract::admin_permission::AdminPermission::UsersUpdate,
                    server_admin_contract::admin_permission::AdminPermission::UsersDelete,
                ],
                |auth| {
                    crate::queries_users_page::queries_users_page(
                        auth,
                        crate::axum_admin_query::AxumAdminQuery::from(
                            server_admin_contract::admin_table_query::AdminTableQuery::default(),
                        ),
                    )
                },
                frontend::render_user_manage::render_user_manage,
            )
            .await
        }
        crate::admin_crud_page::AdminCrudPage::RoleCreate => {
            crate::crud_page::crud_page(
                admin_auth_request,
                &[server_admin_contract::admin_permission::AdminPermission::RolesCreate],
                async |_auth| Ok(()),
                |_view, admin, branding| {
                    frontend::render_role_create::render_role_create(admin, branding)
                },
            )
            .await
        }
        crate::admin_crud_page::AdminCrudPage::RoleManage => {
            crate::crud_page::crud_page(
                admin_auth_request,
                &[
                    server_admin_contract::admin_permission::AdminPermission::RolesUpdate,
                    server_admin_contract::admin_permission::AdminPermission::RolesDelete,
                ],
                |auth| {
                    crate::queries_roles_page::queries_roles_page(
                        auth,
                        crate::axum_admin_query::AxumAdminQuery::from(
                            server_admin_contract::admin_table_query::AdminTableQuery::default(),
                        ),
                    )
                },
                frontend::render_role_manage::render_role_manage,
            )
            .await
        }
    }
}
