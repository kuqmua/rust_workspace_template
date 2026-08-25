#[frontend_contract::domain_types::route_error(AdminSignInPageError)]
pub(super) async fn sign_in_page(auth: super::super::AdminAuthReq) -> axum::response::Response {
    match super::super::settings::branding_view(auth).await {
        Ok(branding) => super::html_response(
            server_admin_frontend::domain_types::ssr::render_sign_in(None, Some(&branding)),
        ),
        Err(error) => super::html_page_error(error),
    }
}

async fn csr_page(
    auth: super::super::AdminAuthReq,
    page: server_admin_contract::domain_types::AdminPage,
    active_table: Option<server_admin_contract::domain_types::AdminDataTable>,
) -> axum::response::Response {
    match super::page_context(&auth).await {
        Ok((_admin, _branding, password_change_required))
            if *password_change_required
                && page != server_admin_contract::domain_types::AdminPage::Profile =>
        {
            axum::response::IntoResponse::into_response(axum::response::Redirect::to(
                server_admin_contract::domain_types::AdminFrontendPath::Profile.get(),
            ))
        }
        Ok((admin, branding, _password_change_required))
            if bool::from(admin.can_access(page))
                && active_table
                    .is_none_or(|table| bool::from(admin.has_permission(table.permission()))) =>
        {
            super::html_response(server_admin_frontend::domain_types::ssr::render_admin_csr(
                page,
                active_table,
                &admin,
                &branding,
            ))
        }
        Ok(_context) => super::html_page_error(super::super::AdminError::Authorization),
        Err(error) => super::html_page_error(error),
    }
}

async fn crud_page<View, Load, LoadFuture, Render>(
    auth: super::super::AdminAuthReq,
    permissions: &[server_admin_contract::domain_types::AdminPermission],
    load: Load,
    render: Render,
) -> axum::response::Response
where
    Load: FnOnce(super::super::AdminAuthReq) -> LoadFuture,
    LoadFuture: Future<Output = Result<View, super::super::AdminError>>,
    Render: FnOnce(
        &View,
        &server_admin_contract::domain_types::AuthenticatedAdmin,
        &server_admin_contract::domain_types::AdminBrandingView,
    ) -> server_admin_frontend::domain_types::ssr::AdminSsrHtml,
{
    match super::page_context(&auth).await {
        Ok((_admin, _branding, password_change_required)) if *password_change_required => {
            axum::response::IntoResponse::into_response(axum::response::Redirect::to(
                server_admin_contract::domain_types::AdminFrontendPath::Profile.get(),
            ))
        }
        Ok((admin, branding, _password_change_required))
            if permissions
                .iter()
                .any(|permission| bool::from(admin.has_permission(*permission))) =>
        {
            match load(auth).await {
                Ok(view) => super::html_response(render(&view, &admin, &branding)),
                Err(error) => super::html_page_error(error),
            }
        }
        Ok(_context) => super::html_page_error(super::super::AdminError::Authorization),
        Err(error) => super::html_page_error(error),
    }
}

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy)]
enum AdminCrudPage {
    RoleCreate,
    RoleManage,
    UserCreate,
    UserManage,
}

async fn crud_resource_page(
    auth: super::super::AdminAuthReq,
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
                    super::super::users::queries::users_page(
                        auth,
                        super::super::AxumAdminQuery(
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
                    super::super::roles::queries::roles_page(
                        auth,
                        super::super::AxumAdminQuery(
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

#[frontend_contract::domain_types::route_error(AdminDataTablesPageError)]
pub(super) async fn data_tables(
    auth: super::super::AdminAuthReq,
    super::super::AxumAdminPath(table): super::super::AxumAdminPath<
        server_admin_contract::domain_types::AdminDataTable,
    >,
) -> axum::response::Response {
    csr_page(
        auth,
        server_admin_contract::domain_types::AdminPage::Tables,
        Some(table),
    )
    .await
}

#[frontend_contract::domain_types::route_error(AdminUsersPageError)]
pub(super) async fn users(auth: super::super::AdminAuthReq) -> axum::response::Response {
    csr_page(
        auth,
        server_admin_contract::domain_types::AdminPage::Tables,
        Some(server_admin_contract::domain_types::AdminDataTable::Users),
    )
    .await
}

#[frontend_contract::domain_types::route_error(AdminUsersCreatePageError)]
pub(super) async fn users_create_page(
    auth: super::super::AdminAuthReq,
) -> axum::response::Response {
    crud_resource_page(auth, AdminCrudPage::UserCreate).await
}

#[frontend_contract::domain_types::route_error(AdminUsersManagePageError)]
pub(super) async fn users_manage_page(
    auth: super::super::AdminAuthReq,
) -> axum::response::Response {
    crud_resource_page(auth, AdminCrudPage::UserManage).await
}

#[frontend_contract::domain_types::route_error(AdminRolesPageError)]
pub(super) async fn roles(auth: super::super::AdminAuthReq) -> axum::response::Response {
    csr_page(
        auth,
        server_admin_contract::domain_types::AdminPage::Tables,
        Some(server_admin_contract::domain_types::AdminDataTable::Roles),
    )
    .await
}

#[frontend_contract::domain_types::route_error(AdminRolesCreatePageError)]
pub(super) async fn roles_create_page(
    auth: super::super::AdminAuthReq,
) -> axum::response::Response {
    crud_resource_page(auth, AdminCrudPage::RoleCreate).await
}

#[frontend_contract::domain_types::route_error(AdminRolesManagePageError)]
pub(super) async fn roles_manage_page(
    auth: super::super::AdminAuthReq,
) -> axum::response::Response {
    crud_resource_page(auth, AdminCrudPage::RoleManage).await
}

#[frontend_contract::domain_types::route_error(AdminPermissionsPageError)]
pub(super) async fn permissions(auth: super::super::AdminAuthReq) -> axum::response::Response {
    csr_page(
        auth,
        server_admin_contract::domain_types::AdminPage::Tables,
        Some(server_admin_contract::domain_types::AdminDataTable::Permissions),
    )
    .await
}

#[frontend_contract::domain_types::route_error(AdminSessionsPageError)]
pub(super) async fn sessions(auth: super::super::AdminAuthReq) -> axum::response::Response {
    csr_page(
        auth,
        server_admin_contract::domain_types::AdminPage::Sessions,
        None,
    )
    .await
}

#[frontend_contract::domain_types::route_error(AdminProfilePageError)]
pub(super) async fn profile(auth: super::super::AdminAuthReq) -> axum::response::Response {
    csr_page(
        auth,
        server_admin_contract::domain_types::AdminPage::Profile,
        None,
    )
    .await
}

#[frontend_contract::domain_types::route_error(AdminSettingsPageError)]
pub(super) async fn settings(auth: super::super::AdminAuthReq) -> axum::response::Response {
    csr_page(
        auth,
        server_admin_contract::domain_types::AdminPage::Settings,
        None,
    )
    .await
}

#[frontend_contract::domain_types::route_error(AdminVersionPageError)]
pub(super) async fn version(auth: super::super::AdminAuthReq) -> axum::response::Response {
    match super::page_context(&auth).await {
        Ok((_admin, _branding, password_change_required)) if *password_change_required => {
            axum::response::IntoResponse::into_response(axum::response::Redirect::to(
                server_admin_contract::domain_types::AdminFrontendPath::Profile.get(),
            ))
        }
        Ok((admin, branding, _password_change_required)) => match (
            server_admin_frontend::domain_types::ssr::AdminSsrText::try_from(
                constants_str::VERSION_ALT.to_owned(),
            ),
            server_admin_frontend::domain_types::ssr::AdminSsrText::try_from(
                git_info::domain_types::project_git_info()
                    .commit()
                    .to_string(),
            ),
        ) {
            (Ok(title), Ok(text)) => super::html_response(
                server_admin_frontend::domain_types::ssr::render_text_page_with_access(
                    server_admin_contract::domain_types::AdminPage::Version,
                    title,
                    text,
                    &admin,
                    &branding,
                ),
            ),
            (Err(_error), _) | (_, Err(_error)) => {
                axum::response::IntoResponse::into_response(http::StatusCode::INTERNAL_SERVER_ERROR)
            }
        },
        Err(error) => super::html_page_error(error),
    }
}

#[frontend_contract::domain_types::route_error(AdminOpenApiPageError)]
pub(super) async fn open_api(auth: super::super::AdminAuthReq) -> axum::response::Response {
    let branding_result = super::super::settings::branding_view_ref(&auth).await;
    let authorized = super::super::authorization::authorize_generated_request(
        auth.state.as_ref(),
        super::super::super::HttpAdminHeaderMapRef::from(auth.headers.as_ref()),
        auth.peer,
        super::super::super::AdminPermission::OpenApiRead.as_str(),
        super::super::super::StdAdminBool::from(false),
    )
    .await;
    match (authorized, branding_result) {
        (Ok(admin), Ok(branding)) => {
            let admin = match super::super::authenticated_admin_contract(&admin) {
                Ok(value) => value,
                Err(error) => return super::html_page_error(error),
            };
            let document = utoipa::openapi::OpenApi::from(
                super::super::super::generated_tables::generated_open_api(),
            );
            match serde_json::to_string_pretty(&document) {
                Ok(text) => match (
                    server_admin_frontend::domain_types::ssr::AdminSsrText::try_from(
                        constants_str::OPENAPI_DOCUMENT.to_owned(),
                    ),
                    server_admin_frontend::domain_types::ssr::AdminSsrText::try_from(text),
                ) {
                    (Ok(title), Ok(text)) => super::html_response(
                        server_admin_frontend::domain_types::ssr::render_text_page_with_access(
                            server_admin_contract::domain_types::AdminPage::OpenApi,
                            title,
                            text,
                            &admin,
                            &branding,
                        ),
                    ),
                    (Err(_error), _) | (_, Err(_error)) => {
                        axum::response::IntoResponse::into_response(
                            http::StatusCode::INTERNAL_SERVER_ERROR,
                        )
                    }
                },
                Err(_error) => axum::response::IntoResponse::into_response(
                    http::StatusCode::INTERNAL_SERVER_ERROR,
                ),
            }
        }
        (Err(error), _) | (_, Err(error)) => super::html_page_error(error),
    }
}

#[derive(optimal_memory_layout::OptimalMemoryLayout)]
#[frontend_contract::domain_types::endpoint_registry(
    state = super::super::SharedAdminAuthSvcStateArc;
    (server_admin_contract::domain_types::AdminFrontendPath::SignIn, sign_in_page),
    (server_admin_contract::domain_types::AdminFrontendPath::Tables, data_tables),
    (server_admin_contract::domain_types::AdminFrontendPath::Users, users),
    (server_admin_contract::domain_types::AdminFrontendPath::UsersCreate, users_create_page),
    (server_admin_contract::domain_types::AdminFrontendPath::UsersManage, users_manage_page),
    (server_admin_contract::domain_types::AdminFrontendPath::Roles, roles),
    (server_admin_contract::domain_types::AdminFrontendPath::RolesCreate, roles_create_page),
    (server_admin_contract::domain_types::AdminFrontendPath::RolesManage, roles_manage_page),
    (server_admin_contract::domain_types::AdminFrontendPath::Permissions, permissions),
    (server_admin_contract::domain_types::AdminFrontendPath::Sessions, sessions),
    (server_admin_contract::domain_types::AdminFrontendPath::Profile, profile),
    (server_admin_contract::domain_types::AdminFrontendPath::Settings, settings),
    (server_admin_contract::domain_types::AdminFrontendPath::Version, version),
)]
struct AdminHtmlPageRouteRegistry;

#[derive(optimal_memory_layout::OptimalMemoryLayout)]
#[frontend_contract::domain_types::endpoint_registry(
    state = super::super::SharedAdminAuthSvcStateArc;
    (server_admin_contract::domain_types::AdminFrontendPath::OpenApi, open_api),
)]
struct AdminHtmlSwaggerRouteRegistry;

pub(super) fn router() -> super::super::AxumAdminStateRouter {
    super::super::AxumAdminStateRouter::from(AdminHtmlPageRouteRegistry::router())
}

pub(super) fn swagger_router() -> super::super::AxumAdminStateRouter {
    super::super::AxumAdminStateRouter::from(AdminHtmlSwaggerRouteRegistry::router())
}
