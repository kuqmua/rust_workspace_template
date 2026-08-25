#![allow(clippy::single_call_fn)] // each server-rendered HTML handler is registered once in the Axum route inventory
#![allow(
    clippy::shadow_reuse,
    reason = "form adapters deliberately replace unvalidated extractor values with validated domain values"
)]

#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct SignInForm {
    login: server_admin_contract::domain_types::AdminLogin,
    password: server_admin_contract::domain_types::AdminPassword,
}

#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct ChangePasswordForm {
    current_password: server_admin_contract::domain_types::AdminPassword,
    new_password: server_admin_contract::domain_types::AdminNewPassword,
}

#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct RevokeSessionForm {
    session_id: server_admin_contract::domain_types::AdminSessionIdentifier,
    confirmation: server_admin_contract::domain_types::AdminBool,
}

#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct CreateUserForm {
    display_name: server_admin_contract::domain_types::AdminDisplayName,
    login: server_admin_contract::domain_types::AdminLogin,
    password: server_admin_contract::domain_types::AdminNewPassword,
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct UpdateUserForm {
    display_name: server_admin_contract::domain_types::AdminDisplayName,
    login: server_admin_contract::domain_types::AdminLogin,
    user_id: server_admin_contract::domain_types::AdminUserId,
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct UserPasswordForm {
    password: server_admin_contract::domain_types::AdminNewPassword,
    user_id: server_admin_contract::domain_types::AdminUserId,
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct UserBanForm {
    user_id: server_admin_contract::domain_types::AdminUserId,
    is_banned: server_admin_contract::domain_types::AdminBool,
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct UserIdForm {
    user_id: server_admin_contract::domain_types::AdminUserId,
    confirmation: server_admin_contract::domain_types::AdminBool,
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, serde::Deserialize)]
struct UserRolesForm {
    expected_role_ids: AdminHtmlFormText,
    #[serde(flatten)]
    selected: StdAdminHtmlSelected,
    user_id: server_admin_contract::domain_types::AdminUserId,
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct CreateRoleForm {
    name: server_admin_contract::domain_types::AdminRoleName,
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct UpdateRoleForm {
    name: server_admin_contract::domain_types::AdminRoleName,
    role_id: server_admin_contract::domain_types::AdminRoleId,
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct RoleIdForm {
    role_id: server_admin_contract::domain_types::AdminRoleId,
    confirmation: server_admin_contract::domain_types::AdminBool,
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, serde::Deserialize)]
struct RolePermissionsForm {
    expected_permission_ids: AdminHtmlFormText,
    #[serde(flatten)]
    selected: StdAdminHtmlSelected,
    role_id: server_admin_contract::domain_types::AdminRoleId,
}

const ADMIN_HTML_FORM_SELECTED_MAX_ITEMS: usize = 1_000usize;

#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
#[error("{message}", message = constants_str::ADMIN_HTML_FORM_TEXT_TOO_LONG)]
struct AdminHtmlFormTextError;
impl From<bounded_types::domain_types::BoundedValueError> for AdminHtmlFormTextError {
    fn from(_value: bounded_types::domain_types::BoundedValueError) -> Self {
        Self
    }
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
#[error("{message}", message = constants_str::ADMIN_HTML_FORM_KEY_TOO_LONG)]
struct AdminHtmlFormKeyError;
impl From<bounded_types::domain_types::BoundedValueError> for AdminHtmlFormKeyError {
    fn from(_value: bounded_types::domain_types::BoundedValueError) -> Self {
        Self
    }
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
#[error("administrator HTML form contains too many selected fields")]
struct StdAdminHtmlSelectedError;
impl From<bounded_types::domain_types::BoundedValueError> for StdAdminHtmlSelectedError {
    fn from(_value: bounded_types::domain_types::BoundedValueError) -> Self {
        Self
    }
}

#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, serde::Deserialize)]
#[serde(try_from = "String")]
struct AdminHtmlFormText(
    bounded_types::domain_types::text::BoundedString<0, { constants_usize::VALUE_8_192 }>,
);
impl TryFrom<String> for AdminHtmlFormText {
    type Error = AdminHtmlFormTextError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        bounded_types::domain_types::text::BoundedString::try_from(value)
            .map(Self)
            .map_err(AdminHtmlFormTextError::from)
    }
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Eq,
    Ord,
    PartialEq,
    PartialOrd,
    serde::Deserialize,
)]
#[serde(try_from = "String")]
struct AdminHtmlFormKey(
    bounded_types::domain_types::text::BoundedString<0, { constants_usize::VALUE_8_192 }>,
);
impl TryFrom<String> for AdminHtmlFormKey {
    type Error = AdminHtmlFormKeyError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        bounded_types::domain_types::text::BoundedString::try_from(value)
            .map(Self)
            .map_err(AdminHtmlFormKeyError::from)
    }
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Debug, newtype::FromInner, serde::Deserialize,
)]
#[serde(
    from = "bounded_types::domain_types::btree::BoundedBTreeMap<AdminHtmlFormKey, AdminHtmlFormText, ADMIN_HTML_FORM_SELECTED_MAX_ITEMS>"
)]
struct StdAdminHtmlSelected(
    bounded_types::domain_types::btree::BoundedBTreeMap<
        AdminHtmlFormKey,
        AdminHtmlFormText,
        ADMIN_HTML_FORM_SELECTED_MAX_ITEMS,
    >,
);
impl TryFrom<std::collections::BTreeMap<AdminHtmlFormKey, AdminHtmlFormText>>
    for StdAdminHtmlSelected
{
    type Error = StdAdminHtmlSelectedError;
    fn try_from(
        value: std::collections::BTreeMap<AdminHtmlFormKey, AdminHtmlFormText>,
    ) -> Result<Self, Self::Error> {
        bounded_types::domain_types::btree::BoundedBTreeMap::try_from(value)
            .map(Self)
            .map_err(StdAdminHtmlSelectedError::from)
    }
}

#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct SettingsForm {
    default_admin_route: server_admin_contract::domain_types::AdminDefaultRoute,
    main_logo: AdminHtmlFormText,
    organization_contacts: AdminHtmlFormText,
    organization_name: AdminHtmlFormText,
    primary_color: AdminHtmlFormText,
    site_name: server_admin_contract::domain_types::AdminSiteName,
    support_url: AdminHtmlFormText,
    tab_title: AdminHtmlFormText,
}

fn html_response(
    html: server_admin_frontend::domain_types::ssr::AdminSsrHtml,
) -> axum::response::Response {
    axum::response::IntoResponse::into_response(axum::response::Html(String::from(html)))
}

fn html_page_error(error: super::AdminError) -> axum::response::Response {
    if matches!(error, super::AdminError::Authentication) {
        axum::response::IntoResponse::into_response(axum::response::Redirect::to(
            server_admin_contract::domain_types::AdminFrontendPath::SignIn.get(),
        ))
    } else {
        axum::response::IntoResponse::into_response(error)
    }
}

async fn page_context(
    auth: &super::AdminAuthReq,
) -> Result<
    (
        server_admin_contract::domain_types::AuthenticatedAdmin,
        server_admin_contract::domain_types::AdminBrandingView,
        super::super::AdminPasswordChangeRequired,
    ),
    super::AdminError,
> {
    let (admin, password_change_required) = super::account::me_context_view_ref(auth).await?;
    let branding = super::settings::branding_view_ref(auth).await?;
    Ok((admin, branding, password_change_required))
}

fn form_auth(mut auth: super::AdminAuthReq) -> Result<super::AdminAuthReq, super::AdminError> {
    if !super::origin_is_present_and_allowed(
        auth.state.as_ref(),
        super::super::HttpAdminHeaderMapRef::from(auth.headers.as_ref()),
    )
    .get()
    {
        return Err(super::AdminError::Csrf);
    }
    let token = super::super::find_admin_cookie(
        super::super::HttpAdminHeaderMapRef::from(auth.headers.as_ref()),
        super::super::AdminCookieKind::Csrf,
    )
    .ok_or(super::AdminError::Csrf)?;
    let value = http::HeaderValue::from_str(token.as_ref()).map_err(|error| {
        super::AdminError::header(super::HttpAdminHeaderValueError::from(error))
    })?;
    let _previous = auth.headers.0.insert(
        http::HeaderName::from_static(constants_str::X_CSRF_TOKEN_ALT),
        value,
    );
    Ok(auth)
}

fn success_redirect(
    path: server_admin_contract::domain_types::AdminFrontendPath,
) -> axum::response::Response {
    axum::response::IntoResponse::into_response(axum::response::Redirect::to(
        format!("{}{}", path.get(), constants_str::ADMIN_HTML_SAVED_FRAGMENT).as_str(),
    ))
}

fn user_path(value: server_admin_contract::domain_types::AdminUserId) -> super::super::AdminUserId {
    super::super::AdminUserId::from(value.value())
}

fn role_path(value: server_admin_contract::domain_types::AdminRoleId) -> super::super::AdminRoleId {
    super::super::AdminRoleId::from(value.value())
}

fn action_result(
    result: Result<super::AxumAdminResponse, super::AdminError>,
    path: server_admin_contract::domain_types::AdminFrontendPath,
) -> axum::response::Response {
    match result {
        Ok(_response) => success_redirect(path),
        Err(error) => axum::response::IntoResponse::into_response(error),
    }
}

async fn authenticated_action<Action, ActionFuture>(
    auth: super::AdminAuthReq,
    path: server_admin_contract::domain_types::AdminFrontendPath,
    action: Action,
) -> axum::response::Response
where
    Action: FnOnce(super::AdminAuthReq) -> ActionFuture,
    ActionFuture: Future<Output = Result<super::AxumAdminResponse, super::AdminError>>,
{
    let Ok(auth) = form_auth(auth) else {
        return axum::response::IntoResponse::into_response(super::AdminError::Csrf);
    };
    action_result(action(auth).await, path)
}

fn optional_setting<Value, Error>(
    value: AdminHtmlFormText,
) -> Result<Option<Value>, super::AdminError>
where
    Value: TryFrom<String, Error = Error>,
{
    if value.0.trim().is_empty() {
        Ok(None)
    } else {
        Value::try_from(value.0.into_inner())
            .map(Some)
            .map_err(|_error| super::AdminError::Validation)
    }
}

fn role_ids(
    value: &AdminHtmlFormText,
) -> Result<server_admin_contract::domain_types::AdminRoleIds, super::AdminError> {
    if value.0.is_empty() {
        return server_admin_contract::domain_types::AdminRoleIds::try_from(Vec::new()).map_err(
            |server_admin_contract::domain_types::AdminCollectionError::TooLong| {
                super::AdminError::Validation
            },
        );
    }
    let values = value
        .0
        .split(',')
        .map(|item| {
            let parsed = item
                .parse::<i64>()
                .map_err(|_error| super::AdminError::Validation)?;
            server_admin_contract::domain_types::AdminRoleId::try_from(parsed)
                .map_err(|_error| super::AdminError::Validation)
        })
        .collect::<Result<Vec<_>, _>>()?;
    server_admin_contract::domain_types::AdminRoleIds::try_from(values)
        .map_err(|_error| super::AdminError::Validation)
}

fn permission_ids(
    value: &AdminHtmlFormText,
) -> Result<server_admin_contract::domain_types::AdminPermissionIds, super::AdminError> {
    if value.0.is_empty() {
        return server_admin_contract::domain_types::AdminPermissionIds::try_from(Vec::new())
            .map_err(
                |server_admin_contract::domain_types::AdminCollectionError::TooLong| {
                    super::AdminError::Validation
                },
            );
    }
    let values = value
        .0
        .split(',')
        .map(|item| {
            let parsed = item
                .parse::<i64>()
                .map_err(|_error| super::AdminError::Validation)?;
            server_admin_contract::domain_types::AdminPermissionId::try_from(parsed)
                .map_err(|_error| super::AdminError::Validation)
        })
        .collect::<Result<Vec<_>, _>>()?;
    server_admin_contract::domain_types::AdminPermissionIds::try_from(values)
        .map_err(|_error| super::AdminError::Validation)
}

fn authenticated_selected_form<Ids, Parse>(
    auth: super::AdminAuthReq,
    expected: &AdminHtmlFormText,
    selected: StdAdminHtmlSelected,
    parse: Parse,
) -> Result<(super::AdminAuthReq, Ids, Ids), super::AdminError>
where
    Parse: Fn(&AdminHtmlFormText) -> Result<Ids, super::AdminError>,
{
    let auth = form_auth(auth)?;
    let expected = parse(expected)?;
    let separator = constants_str::COMMA_SPACE.trim();
    let capacity = selected
        .0
        .iter()
        .map(|(_key, value)| value.0.len().get())
        .sum::<usize>()
        .saturating_add(
            selected
                .0
                .len()
                .get()
                .saturating_sub(constants_usize::ONE)
                .saturating_mul(separator.len()),
        );
    let text = selected.0.into_values().enumerate().fold(
        String::with_capacity(capacity),
        |mut text, (index, value)| {
            if index > constants_usize::ZERO {
                text.push_str(separator);
            }
            text.push_str(value.0.as_ref());
            text
        },
    );
    let selected_ids = AdminHtmlFormText::try_from(text)
        .map_err(|_error| super::AdminError::Validation)
        .and_then(|value| parse(&value))?;
    Ok((auth, expected, selected_ids))
}

#[frontend_contract::domain_types::route_error(AdminSignInPageError)]
async fn sign_in_page(auth: super::AdminAuthReq) -> axum::response::Response {
    match super::settings::branding_view(auth).await {
        Ok(branding) => html_response(server_admin_frontend::domain_types::ssr::render_sign_in(
            None,
            Some(&branding),
        )),
        Err(error) => html_page_error(error),
    }
}

async fn csr_page(
    auth: super::AdminAuthReq,
    page: server_admin_contract::domain_types::AdminPage,
    active_table: Option<server_admin_contract::domain_types::AdminDataTable>,
) -> axum::response::Response {
    match page_context(&auth).await {
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
            html_response(server_admin_frontend::domain_types::ssr::render_admin_csr(
                page,
                active_table,
                &admin,
                &branding,
            ))
        }
        Ok(_context) => html_page_error(super::AdminError::Authorization),
        Err(error) => html_page_error(error),
    }
}

async fn crud_page<View, Load, LoadFuture, Render>(
    auth: super::AdminAuthReq,
    permissions: &[server_admin_contract::domain_types::AdminPermission],
    load: Load,
    render: Render,
) -> axum::response::Response
where
    Load: FnOnce(super::AdminAuthReq) -> LoadFuture,
    LoadFuture: Future<Output = Result<View, super::AdminError>>,
    Render: FnOnce(
        &View,
        &server_admin_contract::domain_types::AuthenticatedAdmin,
        &server_admin_contract::domain_types::AdminBrandingView,
    ) -> server_admin_frontend::domain_types::ssr::AdminSsrHtml,
{
    match page_context(&auth).await {
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
                Ok(view) => html_response(render(&view, &admin, &branding)),
                Err(error) => html_page_error(error),
            }
        }
        Ok(_context) => html_page_error(super::AdminError::Authorization),
        Err(error) => html_page_error(error),
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
    auth: super::AdminAuthReq,
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
                    super::users::users_page(
                        auth,
                        super::AxumAdminQuery(
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
                    super::roles::roles_page(
                        auth,
                        super::AxumAdminQuery(
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
async fn data_tables(
    auth: super::AdminAuthReq,
    super::AxumAdminPath(table): super::AxumAdminPath<
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
async fn users(auth: super::AdminAuthReq) -> axum::response::Response {
    csr_page(
        auth,
        server_admin_contract::domain_types::AdminPage::Tables,
        Some(server_admin_contract::domain_types::AdminDataTable::Users),
    )
    .await
}

#[frontend_contract::domain_types::route_error(AdminUsersCreatePageError)]
async fn users_create_page(auth: super::AdminAuthReq) -> axum::response::Response {
    crud_resource_page(auth, AdminCrudPage::UserCreate).await
}

#[frontend_contract::domain_types::route_error(AdminUsersManagePageError)]
async fn users_manage_page(auth: super::AdminAuthReq) -> axum::response::Response {
    crud_resource_page(auth, AdminCrudPage::UserManage).await
}

#[frontend_contract::domain_types::route_error(AdminRolesPageError)]
async fn roles(auth: super::AdminAuthReq) -> axum::response::Response {
    csr_page(
        auth,
        server_admin_contract::domain_types::AdminPage::Tables,
        Some(server_admin_contract::domain_types::AdminDataTable::Roles),
    )
    .await
}

#[frontend_contract::domain_types::route_error(AdminRolesCreatePageError)]
async fn roles_create_page(auth: super::AdminAuthReq) -> axum::response::Response {
    crud_resource_page(auth, AdminCrudPage::RoleCreate).await
}

#[frontend_contract::domain_types::route_error(AdminRolesManagePageError)]
async fn roles_manage_page(auth: super::AdminAuthReq) -> axum::response::Response {
    crud_resource_page(auth, AdminCrudPage::RoleManage).await
}

#[frontend_contract::domain_types::route_error(AdminPermissionsPageError)]
async fn permissions(auth: super::AdminAuthReq) -> axum::response::Response {
    csr_page(
        auth,
        server_admin_contract::domain_types::AdminPage::Tables,
        Some(server_admin_contract::domain_types::AdminDataTable::Permissions),
    )
    .await
}

#[frontend_contract::domain_types::route_error(AdminSessionsPageError)]
async fn sessions(auth: super::AdminAuthReq) -> axum::response::Response {
    csr_page(
        auth,
        server_admin_contract::domain_types::AdminPage::Sessions,
        None,
    )
    .await
}

#[frontend_contract::domain_types::route_error(AdminProfilePageError)]
async fn profile(auth: super::AdminAuthReq) -> axum::response::Response {
    csr_page(
        auth,
        server_admin_contract::domain_types::AdminPage::Profile,
        None,
    )
    .await
}

#[frontend_contract::domain_types::route_error(AdminSettingsPageError)]
async fn settings(auth: super::AdminAuthReq) -> axum::response::Response {
    csr_page(
        auth,
        server_admin_contract::domain_types::AdminPage::Settings,
        None,
    )
    .await
}

#[frontend_contract::domain_types::route_error(AdminVersionPageError)]
async fn version(auth: super::AdminAuthReq) -> axum::response::Response {
    match page_context(&auth).await {
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
            (Ok(title), Ok(text)) => html_response(
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
        Err(error) => html_page_error(error),
    }
}

#[frontend_contract::domain_types::route_error(AdminOpenApiPageError)]
async fn open_api(auth: super::AdminAuthReq) -> axum::response::Response {
    let branding_result = super::settings::branding_view_ref(&auth).await;
    let authorized = super::authorize_generated_request(
        auth.state.as_ref(),
        super::super::HttpAdminHeaderMapRef::from(auth.headers.as_ref()),
        auth.peer,
        super::super::AdminPermission::OpenApiRead.as_str(),
        super::super::StdAdminBool::from(false),
    )
    .await;
    match (authorized, branding_result) {
        (Ok(admin), Ok(branding)) => {
            let admin = match super::authenticated_admin_contract(&admin) {
                Ok(value) => value,
                Err(error) => return html_page_error(error),
            };
            let document = utoipa::openapi::OpenApi::from(
                super::super::generated_tables::generated_open_api(),
            );
            match serde_json::to_string_pretty(&document) {
                Ok(text) => match (
                    server_admin_frontend::domain_types::ssr::AdminSsrText::try_from(
                        constants_str::OPENAPI_DOCUMENT.to_owned(),
                    ),
                    server_admin_frontend::domain_types::ssr::AdminSsrText::try_from(text),
                ) {
                    (Ok(title), Ok(text)) => html_response(
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
        (Err(error), _) | (_, Err(error)) => html_page_error(error),
    }
}

#[frontend_contract::domain_types::route_operation]
async fn root() -> axum::response::Response {
    axum::response::IntoResponse::into_response(axum::response::Redirect::to(
        server_admin_contract::domain_types::AdminFrontendPath::Users.get(),
    ))
}

#[frontend_contract::domain_types::route_error(AdminHtmlSignOutError)]
async fn sign_out(auth: super::AdminAuthReq) -> axum::response::Response {
    match form_auth(auth) {
        Ok(auth) => match super::authn::sign_out(auth).await {
            Ok(response) => {
                let mut target =
                    axum::response::IntoResponse::into_response(axum::response::Redirect::to(
                        server_admin_contract::domain_types::AdminFrontendPath::SignIn.get(),
                    ));
                response
                    .0
                    .headers()
                    .get_all(http::header::SET_COOKIE)
                    .iter()
                    .cloned()
                    .for_each(|value| {
                        let _appended =
                            target.headers_mut().append(http::header::SET_COOKIE, value);
                    });
                target
            }
            Err(error) => axum::response::IntoResponse::into_response(error),
        },
        Err(error) => axum::response::IntoResponse::into_response(error),
    }
}

#[frontend_contract::domain_types::route_error(AdminHtmlChangePasswordError)]
async fn change_password(
    auth: super::AdminAuthReq,
    super::AxumAdminForm(form): super::AxumAdminForm<ChangePasswordForm>,
) -> axum::response::Response {
    match form_auth(auth) {
        Ok(auth) => {
            let request = server_admin_contract::domain_types::AdminChangeOwnPasswordReq::new(
                form.current_password,
                form.new_password,
            );
            match super::account::change_own_password(auth, super::AxumAdminJson(request)).await {
                Ok(_response) => success_redirect(
                    server_admin_contract::domain_types::AdminFrontendPath::Profile,
                ),
                Err(error) => axum::response::IntoResponse::into_response(error),
            }
        }
        Err(error) => axum::response::IntoResponse::into_response(error),
    }
}

#[frontend_contract::domain_types::route_error(AdminHtmlRevokeSessionError)]
async fn revoke_session(
    auth: super::AdminAuthReq,
    super::AxumAdminForm(form): super::AxumAdminForm<RevokeSessionForm>,
) -> axum::response::Response {
    if !bool::from(form.confirmation) {
        return axum::response::IntoResponse::into_response(super::AdminError::Validation);
    }
    let session_id = form
        .session_id
        .to_string()
        .parse::<uuid::Uuid>()
        .map(super::super::UuidAdminValue::from)
        .map(super::super::AdminSessionId::from);
    let Ok(session_id) = session_id else {
        return axum::response::IntoResponse::into_response(super::AdminError::Validation);
    };
    match form_auth(auth) {
        Ok(auth) => {
            match super::sessions::revoke_session(auth, super::AdminSessionPath(session_id)).await {
                Ok(_response) => success_redirect(
                    server_admin_contract::domain_types::AdminFrontendPath::Sessions,
                ),
                Err(error) => axum::response::IntoResponse::into_response(error),
            }
        }
        Err(error) => axum::response::IntoResponse::into_response(error),
    }
}

#[frontend_contract::domain_types::route_error(AdminHtmlCreateUserError)]
async fn create_user(
    auth: super::AdminAuthReq,
    super::AxumAdminForm(form): super::AxumAdminForm<CreateUserForm>,
) -> axum::response::Response {
    let Ok(auth) = form_auth(auth) else {
        return axum::response::IntoResponse::into_response(super::AdminError::Csrf);
    };
    let request = server_admin_contract::domain_types::AdminCreateUserReq::new(
        form.display_name,
        form.login,
        form.password,
    );
    action_result(
        super::users::create(auth, super::AxumAdminJson(request)).await,
        server_admin_contract::domain_types::AdminFrontendPath::Users,
    )
}

#[frontend_contract::domain_types::route_error(AdminHtmlUpdateUserError)]
async fn update_user(
    auth: super::AdminAuthReq,
    super::AxumAdminForm(form): super::AxumAdminForm<UpdateUserForm>,
) -> axum::response::Response {
    let Ok(auth) = form_auth(auth) else {
        return axum::response::IntoResponse::into_response(super::AdminError::Csrf);
    };
    let request = server_admin_contract::domain_types::AdminUpdateUserReq::new(
        Some(form.display_name),
        Some(form.login),
    );
    action_result(
        super::users::update(
            auth,
            super::AxumAdminPath(user_path(form.user_id)),
            super::AxumAdminJson(request),
        )
        .await,
        server_admin_contract::domain_types::AdminFrontendPath::Users,
    )
}

#[frontend_contract::domain_types::route_error(AdminHtmlUserPasswordError)]
async fn user_password(
    auth: super::AdminAuthReq,
    super::AxumAdminForm(form): super::AxumAdminForm<UserPasswordForm>,
) -> axum::response::Response {
    authenticated_action(
        auth,
        server_admin_contract::domain_types::AdminFrontendPath::Users,
        |auth| {
            super::users::set_password(
                auth,
                super::AxumAdminPath(user_path(form.user_id)),
                super::AxumAdminJson(
                    server_admin_contract::domain_types::AdminSetUserPasswordReq::new(
                        form.password,
                    ),
                ),
            )
        },
    )
    .await
}

#[frontend_contract::domain_types::route_error(AdminHtmlUserBanError)]
async fn user_ban(
    auth: super::AdminAuthReq,
    super::AxumAdminForm(form): super::AxumAdminForm<UserBanForm>,
) -> axum::response::Response {
    authenticated_action(
        auth,
        server_admin_contract::domain_types::AdminFrontendPath::Users,
        |auth| {
            super::users::set_ban(
                auth,
                super::AxumAdminPath(user_path(form.user_id)),
                super::AxumAdminJson(
                    server_admin_contract::domain_types::AdminSetUserBanReq::new(form.is_banned),
                ),
            )
        },
    )
    .await
}

#[frontend_contract::domain_types::route_error(AdminHtmlDeleteUserError)]
async fn delete_user(
    auth: super::AdminAuthReq,
    super::AxumAdminForm(form): super::AxumAdminForm<UserIdForm>,
) -> axum::response::Response {
    if !bool::from(form.confirmation) {
        return axum::response::IntoResponse::into_response(super::AdminError::Validation);
    }
    authenticated_action(
        auth,
        server_admin_contract::domain_types::AdminFrontendPath::Users,
        |auth| super::users::delete(auth, super::AxumAdminPath(user_path(form.user_id))),
    )
    .await
}

#[frontend_contract::domain_types::route_error(AdminHtmlUserRolesError)]
async fn user_roles(
    auth: super::AdminAuthReq,
    super::AxumAdminForm(form): super::AxumAdminForm<UserRolesForm>,
) -> axum::response::Response {
    let (auth, expected, selected) =
        match authenticated_selected_form(auth, &form.expected_role_ids, form.selected, role_ids) {
            Ok(values) => values,
            Err(error) => return axum::response::IntoResponse::into_response(error),
        };
    let request =
        server_admin_contract::domain_types::AdminSetUserRolesReq::new(expected, selected);
    action_result(
        super::users::set_roles(
            auth,
            super::AxumAdminPath(user_path(form.user_id)),
            super::AxumAdminJson(request),
        )
        .await,
        server_admin_contract::domain_types::AdminFrontendPath::Users,
    )
}

#[frontend_contract::domain_types::route_error(AdminHtmlCreateRoleError)]
async fn create_role(
    auth: super::AdminAuthReq,
    super::AxumAdminForm(form): super::AxumAdminForm<CreateRoleForm>,
) -> axum::response::Response {
    let Ok(auth) = form_auth(auth) else {
        return axum::response::IntoResponse::into_response(super::AdminError::Csrf);
    };
    action_result(
        super::roles::create(
            auth,
            super::AxumAdminJson(
                server_admin_contract::domain_types::AdminCreateRoleReq::new(form.name),
            ),
        )
        .await,
        server_admin_contract::domain_types::AdminFrontendPath::Roles,
    )
}

#[frontend_contract::domain_types::route_error(AdminHtmlUpdateRoleError)]
async fn update_role(
    auth: super::AdminAuthReq,
    super::AxumAdminForm(form): super::AxumAdminForm<UpdateRoleForm>,
) -> axum::response::Response {
    let Ok(auth) = form_auth(auth) else {
        return axum::response::IntoResponse::into_response(super::AdminError::Csrf);
    };
    action_result(
        super::roles::update(
            auth,
            super::AxumAdminPath(role_path(form.role_id)),
            super::AxumAdminJson(
                server_admin_contract::domain_types::AdminUpdateRoleReq::new(form.name),
            ),
        )
        .await,
        server_admin_contract::domain_types::AdminFrontendPath::Roles,
    )
}

#[frontend_contract::domain_types::route_error(AdminHtmlDeleteRoleError)]
async fn delete_role(
    auth: super::AdminAuthReq,
    super::AxumAdminForm(form): super::AxumAdminForm<RoleIdForm>,
) -> axum::response::Response {
    if !bool::from(form.confirmation) {
        return axum::response::IntoResponse::into_response(super::AdminError::Validation);
    }
    authenticated_action(
        auth,
        server_admin_contract::domain_types::AdminFrontendPath::Roles,
        |auth| super::roles::delete(auth, super::AxumAdminPath(role_path(form.role_id))),
    )
    .await
}

#[frontend_contract::domain_types::route_error(AdminHtmlRolePermissionsError)]
async fn role_permissions(
    auth: super::AdminAuthReq,
    super::AxumAdminForm(form): super::AxumAdminForm<RolePermissionsForm>,
) -> axum::response::Response {
    let (auth, expected, selected) = match authenticated_selected_form(
        auth,
        &form.expected_permission_ids,
        form.selected,
        permission_ids,
    ) {
        Ok(values) => values,
        Err(error) => return axum::response::IntoResponse::into_response(error),
    };
    let request =
        server_admin_contract::domain_types::AdminSetRolePermissionsReq::new(expected, selected);
    action_result(
        super::roles::set_permissions(
            auth,
            super::AxumAdminPath(role_path(form.role_id)),
            super::AxumAdminJson(request),
        )
        .await,
        server_admin_contract::domain_types::AdminFrontendPath::Roles,
    )
}

#[frontend_contract::domain_types::route_error(AdminHtmlUpdateSettingsError)]
async fn update_settings(
    auth: super::AdminAuthReq,
    super::AxumAdminForm(form): super::AxumAdminForm<SettingsForm>,
) -> axum::response::Response {
    let Ok(auth) = form_auth(auth) else {
        return axum::response::IntoResponse::into_response(super::AdminError::Csrf);
    };
    let parsed = (
        optional_setting::<server_admin_contract::domain_types::AdminMainLogo, _>(form.main_logo),
        optional_setting::<server_admin_contract::domain_types::AdminOrganizationContacts, _>(
            form.organization_contacts,
        ),
        optional_setting::<server_admin_contract::domain_types::AdminOrganizationName, _>(
            form.organization_name,
        ),
        optional_setting::<server_admin_contract::domain_types::AdminPrimaryColor, _>(
            form.primary_color,
        ),
        optional_setting::<server_admin_contract::domain_types::AdminSupportUrl, _>(
            form.support_url,
        ),
        optional_setting::<server_admin_contract::domain_types::AdminTabTitle, _>(form.tab_title),
    );
    let (
        Ok(main_logo),
        Ok(organization_contacts),
        Ok(organization_name),
        Ok(primary_color),
        Ok(support_url),
        Ok(tab_title),
    ) = parsed
    else {
        return axum::response::IntoResponse::into_response(super::AdminError::Validation);
    };
    let mut clear = Vec::new();
    [
        (
            main_logo.is_none(),
            server_admin_contract::domain_types::AdminOptionalSetting::MainLogo,
        ),
        (
            organization_contacts.is_none(),
            server_admin_contract::domain_types::AdminOptionalSetting::OrganizationContacts,
        ),
        (
            organization_name.is_none(),
            server_admin_contract::domain_types::AdminOptionalSetting::OrganizationName,
        ),
        (
            primary_color.is_none(),
            server_admin_contract::domain_types::AdminOptionalSetting::PrimaryColor,
        ),
        (
            support_url.is_none(),
            server_admin_contract::domain_types::AdminOptionalSetting::SupportUrl,
        ),
        (
            tab_title.is_none(),
            server_admin_contract::domain_types::AdminOptionalSetting::TabTitle,
        ),
    ]
    .into_iter()
    .filter_map(|(is_empty, setting)| is_empty.then_some(setting))
    .for_each(|setting| clear.push(setting));
    let Ok(clear) = server_admin_contract::domain_types::AdminOptionalSettings::try_from(clear)
    else {
        return axum::response::IntoResponse::into_response(super::AdminError::Validation);
    };
    let request = server_admin_contract::domain_types::AdminUpdateSettingsReq::new(
        Some(form.default_admin_route),
        main_logo,
        organization_contacts,
        organization_name,
        primary_color,
        Some(form.site_name),
        support_url,
        tab_title,
        clear,
    );
    action_result(
        super::settings::update(auth, super::AxumAdminJson(request)).await,
        server_admin_contract::domain_types::AdminFrontendPath::Settings,
    )
}

#[frontend_contract::domain_types::route_error(AdminHtmlSignInError)]
async fn sign_in(
    auth: super::AdminAuthReq,
    peer: super::AdminPeerAddr,
    super::AxumAdminForm(form): super::AxumAdminForm<SignInForm>,
) -> axum::response::Response {
    let branding = super::settings::branding_view_ref(&auth).await.ok();
    match super::authn::sign_in(
        auth,
        peer,
        super::AdminSignInJson(server_admin_contract::domain_types::AdminSignInReq::new(
            form.login,
            form.password,
        )),
    )
    .await
    {
        Ok(response) => {
            let source = response.0;
            let mut target =
                axum::response::IntoResponse::into_response(axum::response::Redirect::to(
                    server_admin_contract::domain_types::AdminFrontendPath::Users.get(),
                ));
            source
                .headers()
                .get_all(http::header::SET_COOKIE)
                .iter()
                .cloned()
                .for_each(|value| {
                    let _appended = target.headers_mut().append(http::header::SET_COOKIE, value);
                });
            target
        }
        Err(_error) => {
            let message_result =
                server_admin_frontend::domain_types::ssr::AdminSsrErrorMessage::try_from(
                    String::from(constants_str::SIGN_IN_FAILED),
                );
            match message_result {
                Ok(error_message) => axum::response::IntoResponse::into_response((
                    http::StatusCode::UNAUTHORIZED,
                    axum::response::Html(String::from(
                        server_admin_frontend::domain_types::ssr::render_sign_in(
                            Some(error_message),
                            branding.as_ref(),
                        ),
                    )),
                )),
                Err(_message_error) => axum::response::IntoResponse::into_response(
                    http::StatusCode::INTERNAL_SERVER_ERROR,
                ),
            }
        }
    }
}

#[derive(optimal_memory_layout::OptimalMemoryLayout)]
#[frontend_contract::domain_types::handler_registry(
    state = super::SharedAdminAuthSvcStateArc;
    (
        server_admin_contract::domain_types::AdminFrontendPath::Root,
        root
    ),
    (
        server_admin_contract::domain_types::AdminFrontendPath::SignIn,
        sign_in_page
    ),
    (
        server_admin_contract::domain_types::AdminFrontendPath::Tables,
        data_tables
    ),
    (
        server_admin_contract::domain_types::AdminFrontendPath::Users,
        users
    ),
    (
        server_admin_contract::domain_types::AdminFrontendPath::UsersCreate,
        users_create_page
    ),
    (
        server_admin_contract::domain_types::AdminFrontendPath::UsersManage,
        users_manage_page
    ),
    (
        server_admin_contract::domain_types::AdminFrontendPath::Roles,
        roles
    ),
    (
        server_admin_contract::domain_types::AdminFrontendPath::RolesCreate,
        roles_create_page
    ),
    (
        server_admin_contract::domain_types::AdminFrontendPath::RolesManage,
        roles_manage_page
    ),
    (
        server_admin_contract::domain_types::AdminFrontendPath::Permissions,
        permissions
    ),
    (
        server_admin_contract::domain_types::AdminFrontendPath::Sessions,
        sessions
    ),
    (
        server_admin_contract::domain_types::AdminFrontendPath::Profile,
        profile
    ),
    (
        server_admin_contract::domain_types::AdminFrontendPath::Settings,
        settings
    ),
    (
        server_admin_contract::domain_types::AdminFrontendPath::Version,
        version
    ),
    (
        server_admin_contract::domain_types::AdminHtmlAction::SignIn,
        sign_in
    ),
    (
        server_admin_contract::domain_types::AdminHtmlAction::SignOut,
        sign_out
    ),
    (
        server_admin_contract::domain_types::AdminHtmlAction::ProfilePassword,
        change_password
    ),
    (
        server_admin_contract::domain_types::AdminHtmlAction::SessionRevoke,
        revoke_session
    ),
    (
        server_admin_contract::domain_types::AdminHtmlAction::UserCreate,
        create_user
    ),
    (
        server_admin_contract::domain_types::AdminHtmlAction::UserUpdate,
        update_user
    ),
    (
        server_admin_contract::domain_types::AdminHtmlAction::UserPassword,
        user_password
    ),
    (
        server_admin_contract::domain_types::AdminHtmlAction::UserBan,
        user_ban
    ),
    (
        server_admin_contract::domain_types::AdminHtmlAction::UserDelete,
        delete_user
    ),
    (
        server_admin_contract::domain_types::AdminHtmlAction::UserRoles,
        user_roles
    ),
    (
        server_admin_contract::domain_types::AdminHtmlAction::RoleCreate,
        create_role
    ),
    (
        server_admin_contract::domain_types::AdminHtmlAction::RoleUpdate,
        update_role
    ),
    (
        server_admin_contract::domain_types::AdminHtmlAction::RoleDelete,
        delete_role
    ),
    (
        server_admin_contract::domain_types::AdminHtmlAction::RolePermissions,
        role_permissions
    ),
    (
        server_admin_contract::domain_types::AdminHtmlAction::SettingsUpdate,
        update_settings
    ),
)]
struct AdminHtmlRouteRegistry;

#[derive(optimal_memory_layout::OptimalMemoryLayout)]
#[frontend_contract::domain_types::handler_registry(
    state = super::SharedAdminAuthSvcStateArc;
    (
        server_admin_contract::domain_types::AdminFrontendPath::OpenApi,
        open_api
    ),
)]
struct AdminHtmlSwaggerRouteRegistry;

pub(super) fn routes(
    state: super::SharedAdminAuthSvcStateArc,
    swagger_enabled: super::AdminHtmlSwaggerEnabled,
) -> super::AxumAdminAuthRouter {
    let router = AdminHtmlRouteRegistry::router();
    let router = if swagger_enabled.0 {
        router.merge(AdminHtmlSwaggerRouteRegistry::router())
    } else {
        router
    };
    super::AxumAdminAuthRouter(router.with_state(state))
}

#[cfg(test)]
mod tests;
