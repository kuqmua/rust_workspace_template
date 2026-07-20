#![allow(clippy::single_call_fn)] // each server-rendered HTML handler is registered once in the Axum route inventory
#![allow(
    clippy::shadow_reuse,
    reason = "form adapters deliberately replace unvalidated extractor values with validated domain values"
)]

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct SignInForm {
    login: server_admin_contract::AdminLogin,
    password: server_admin_contract::AdminPassword,
}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct ChangePasswordForm {
    current_password: server_admin_contract::AdminPassword,
    new_password: server_admin_contract::AdminNewPassword,
    #[serde(default)]
    revoke_other_sessions: server_admin_contract::AdminBool,
}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct RevokeSessionForm {
    confirmation: server_admin_contract::AdminBool,
    session_id: server_admin_contract::AdminSessionIdentifier,
}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct CreateUserForm {
    display_name: server_admin_contract::AdminDisplayName,
    login: server_admin_contract::AdminLogin,
    password: server_admin_contract::AdminNewPassword,
}
#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct UpdateUserForm {
    display_name: server_admin_contract::AdminDisplayName,
    login: server_admin_contract::AdminLogin,
    user_id: server_admin_contract::AdminUserId,
}
#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct UserPasswordForm {
    password: server_admin_contract::AdminNewPassword,
    user_id: server_admin_contract::AdminUserId,
}
#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct UserBanForm {
    is_banned: server_admin_contract::AdminBool,
    user_id: server_admin_contract::AdminUserId,
}
#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct UserIdForm {
    confirmation: server_admin_contract::AdminBool,
    user_id: server_admin_contract::AdminUserId,
}
#[derive(Debug, serde::Deserialize)]
struct UserRolesForm {
    expected_role_ids: AdminHtmlFormText,
    #[serde(flatten)]
    selected: StdAdminHtmlSelected,
    user_id: server_admin_contract::AdminUserId,
}
#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct CreateRoleForm {
    name: server_admin_contract::AdminRoleName,
}
#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct UpdateRoleForm {
    name: server_admin_contract::AdminRoleName,
    role_id: server_admin_contract::AdminRoleId,
}
#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct RoleIdForm {
    confirmation: server_admin_contract::AdminBool,
    role_id: server_admin_contract::AdminRoleId,
}
#[derive(Debug, serde::Deserialize)]
struct RolePermissionsForm {
    expected_permission_ids: AdminHtmlFormText,
    #[serde(flatten)]
    selected: StdAdminHtmlSelected,
    role_id: server_admin_contract::AdminRoleId,
}

const ADMIN_HTML_FORM_TEXT_MAX_BYTES: usize = 8_192usize;

#[derive(Debug, thiserror::Error)]
#[error("{message}", message = str_constants::ADMIN_HTML_FORM_TEXT_TOO_LONG)]
struct AdminHtmlFormTextError;
#[derive(Debug, thiserror::Error)]
#[error("{message}", message = str_constants::ADMIN_HTML_FORM_KEY_TOO_LONG)]
struct AdminHtmlFormKeyError;

#[derive(Debug, serde::Deserialize)]
#[serde(try_from = "String")]
struct AdminHtmlFormText(String);
impl TryFrom<String> for AdminHtmlFormText {
    type Error = AdminHtmlFormTextError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        (value.len() <= ADMIN_HTML_FORM_TEXT_MAX_BYTES)
            .then_some(Self(value))
            .ok_or(AdminHtmlFormTextError)
    }
}
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, serde::Deserialize)]
#[serde(try_from = "String")]
struct AdminHtmlFormKey(String);
impl TryFrom<String> for AdminHtmlFormKey {
    type Error = AdminHtmlFormKeyError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        (value.len() <= ADMIN_HTML_FORM_TEXT_MAX_BYTES)
            .then_some(Self(value))
            .ok_or(AdminHtmlFormKeyError)
    }
}
#[derive(Debug, serde::Deserialize)]
struct StdAdminHtmlSelected(std::collections::BTreeMap<AdminHtmlFormKey, AdminHtmlFormText>);

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct SettingsForm {
    default_admin_route: server_admin_contract::AdminDefaultRoute,
    main_logo: AdminHtmlFormText,
    organization_contacts: AdminHtmlFormText,
    organization_name: AdminHtmlFormText,
    primary_color: AdminHtmlFormText,
    site_name: server_admin_contract::AdminSiteName,
    support_url: AdminHtmlFormText,
    tab_title: AdminHtmlFormText,
}

fn html_response(html: server_admin_frontend::ssr::AdminSsrHtml) -> axum::response::Response {
    axum::response::IntoResponse::into_response(axum::response::Html(String::from(html)))
}

fn html_page_error(error: super::AdminApiError) -> axum::response::Response {
    if matches!(error, super::AdminApiError::Authentication) {
        axum::response::IntoResponse::into_response(axum::response::Redirect::to(
            server_admin_contract::AdminFrontendPath::SignIn.get(),
        ))
    } else {
        axum::response::IntoResponse::into_response(error)
    }
}

async fn page_context(
    auth: &super::AdminAuthReq,
) -> Result<
    (
        server_admin_contract::AuthenticatedAdmin,
        server_admin_contract::AdminBrandingView,
    ),
    super::AdminApiError,
> {
    let admin = super::handlers::me_view(auth.clone()).await?;
    let branding = super::handlers::branding_view(auth.clone()).await?;
    Ok((admin, branding))
}

fn form_auth(mut auth: super::AdminAuthReq) -> Result<super::AdminAuthReq, super::AdminApiError> {
    let token = super::super::find_admin_cookie(
        super::super::HttpAdminHeaderMapRef::from(auth.headers.as_ref()),
        super::super::AdminCookieKind::Csrf,
    )
    .ok_or(super::AdminApiError::Csrf)?;
    let value = http::HeaderValue::from_str(token.as_ref()).map_err(|error| {
        super::AdminApiError::Header(super::HttpAdminHeaderValueError::from(error))
    })?;
    let _previous = auth.headers.0.insert(
        http::HeaderName::from_static(str_constants::X_CSRF_TOKEN_ALT),
        value,
    );
    Ok(auth)
}

fn redirect_with_headers(
    path: server_admin_contract::AdminFrontendPath,
    source: &super::AxumAdminResponse,
) -> axum::response::Response {
    let mut target =
        axum::response::IntoResponse::into_response(axum::response::Redirect::to(path.get()));
    source
        .0
        .headers()
        .get_all(http::header::SET_COOKIE)
        .iter()
        .cloned()
        .for_each(|value| {
            let _appended = target.headers_mut().append(http::header::SET_COOKIE, value);
        });
    target
}

fn success_redirect(path: server_admin_contract::AdminFrontendPath) -> axum::response::Response {
    axum::response::IntoResponse::into_response(axum::response::Redirect::to(
        format!("{}{}", path.get(), str_constants::ADMIN_HTML_SAVED_FRAGMENT).as_str(),
    ))
}

fn user_path(value: server_admin_contract::AdminUserId) -> super::super::AdminUserId {
    super::super::AdminUserId::from(i64::from(value))
}

fn role_path(value: server_admin_contract::AdminRoleId) -> super::super::AdminRoleId {
    super::super::AdminRoleId::from(i64::from(value))
}

fn action_result(
    result: Result<super::AxumAdminResponse, super::AdminApiError>,
    path: server_admin_contract::AdminFrontendPath,
) -> axum::response::Response {
    match result {
        Ok(_response) => success_redirect(path),
        Err(error) => axum::response::IntoResponse::into_response(error),
    }
}

fn optional_setting<Value, Error>(
    value: AdminHtmlFormText,
) -> Result<Option<Value>, super::AdminApiError>
where
    Value: TryFrom<String, Error = Error>,
{
    if value.0.trim().is_empty() {
        Ok(None)
    } else {
        Value::try_from(value.0)
            .map(Some)
            .map_err(|_error| super::AdminApiError::Validation)
    }
}

fn role_ids(
    value: &AdminHtmlFormText,
) -> Result<Vec<server_admin_contract::AdminRoleId>, super::AdminApiError> {
    value
        .0
        .split(',')
        .filter(|item| !item.is_empty())
        .map(|item| {
            item.parse::<i64>()
                .map(server_admin_contract::AdminRoleId::from)
                .map_err(|_error| super::AdminApiError::Validation)
        })
        .collect()
}

fn permission_ids(
    value: &AdminHtmlFormText,
) -> Result<Vec<server_admin_contract::AdminPermissionId>, super::AdminApiError> {
    value
        .0
        .split(',')
        .filter(|item| !item.is_empty())
        .map(|item| {
            item.parse::<i64>()
                .map(server_admin_contract::AdminPermissionId::from)
                .map_err(|_error| super::AdminApiError::Validation)
        })
        .collect()
}

async fn sign_in_page(auth: super::AdminAuthReq) -> axum::response::Response {
    match super::handlers::branding_view(auth).await {
        Ok(branding) => html_response(server_admin_frontend::ssr::render_sign_in(
            None,
            Some(&branding),
        )),
        Err(error) => html_page_error(error),
    }
}

async fn dashboard(auth: super::AdminAuthReq) -> axum::response::Response {
    let context_result = page_context(&auth).await;
    let view_result = super::handlers::dashboard_view(auth).await;
    match (context_result, view_result) {
        (Ok((admin, branding)), Ok(view)) => html_response(
            server_admin_frontend::ssr::render_dashboard(&view, &admin, &branding),
        ),
        (Err(error), _) | (_, Err(error)) => html_page_error(error),
    }
}

async fn users(
    auth: super::AdminAuthReq,
    super::AxumAdminQuery(query): super::AxumAdminQuery<server_admin_contract::AdminTableQuery>,
) -> axum::response::Response {
    let context_result = page_context(&auth).await;
    let page_result = super::handlers::users_page(auth, super::AxumAdminQuery(query.clone())).await;
    match (context_result, page_result) {
        (Ok((admin, branding)), Ok(page)) => html_response(
            server_admin_frontend::ssr::render_users(&page, &query, &admin, &branding),
        ),
        (Err(error), _) | (_, Err(error)) => html_page_error(error),
    }
}

async fn roles(
    auth: super::AdminAuthReq,
    super::AxumAdminQuery(query): super::AxumAdminQuery<server_admin_contract::AdminTableQuery>,
) -> axum::response::Response {
    let context_result = page_context(&auth).await;
    let page_result = super::handlers::roles_page(auth, super::AxumAdminQuery(query.clone())).await;
    match (context_result, page_result) {
        (Ok((admin, branding)), Ok(page)) => html_response(
            server_admin_frontend::ssr::render_roles(&page, &query, &admin, &branding),
        ),
        (Err(error), _) | (_, Err(error)) => html_page_error(error),
    }
}

async fn permissions(
    auth: super::AdminAuthReq,
    super::AxumAdminQuery(query): super::AxumAdminQuery<server_admin_contract::AdminTableQuery>,
) -> axum::response::Response {
    let context_result = page_context(&auth).await;
    let page_result =
        super::handlers::permissions_page(auth, super::AxumAdminQuery(query.clone())).await;
    match (context_result, page_result) {
        (Ok((admin, branding)), Ok(page)) => html_response(
            server_admin_frontend::ssr::render_permissions(&page, &query, &admin, &branding),
        ),
        (Err(error), _) | (_, Err(error)) => html_page_error(error),
    }
}

async fn sessions(auth: super::AdminAuthReq) -> axum::response::Response {
    let context_result = page_context(&auth).await;
    let items_result = super::handlers::sessions_view(auth).await;
    match (context_result, items_result) {
        (Ok((admin, branding)), Ok(items)) => html_response(
            server_admin_frontend::ssr::render_sessions(&items, &admin, &branding),
        ),
        (Err(error), _) | (_, Err(error)) => html_page_error(error),
    }
}

async fn profile(auth: super::AdminAuthReq) -> axum::response::Response {
    match page_context(&auth).await {
        Ok((admin, branding)) => html_response(server_admin_frontend::ssr::render_profile(
            &admin, &branding,
        )),
        Err(error) => html_page_error(error),
    }
}

async fn settings(auth: super::AdminAuthReq) -> axum::response::Response {
    let context_result = page_context(&auth).await;
    let view_result = super::handlers::settings_view(auth).await;
    match (context_result, view_result) {
        (Ok((admin, branding)), Ok(view)) => html_response(
            server_admin_frontend::ssr::render_settings(&view, &admin, &branding),
        ),
        (Err(error), _) | (_, Err(error)) => html_page_error(error),
    }
}

async fn audit(
    auth: super::AdminAuthReq,
    super::AxumAdminQuery(query): super::AxumAdminQuery<super::AdminAuditQuery>,
) -> axum::response::Response {
    let context_result = page_context(&auth).await;
    let page_result = super::audit::query_page(auth, super::AxumAdminQuery(query)).await;
    match (context_result, page_result) {
        (Ok((admin, branding)), Ok(page)) => html_response(
            server_admin_frontend::ssr::render_audit(&page, &admin, &branding),
        ),
        (Err(error), _) | (_, Err(error)) => html_page_error(error),
    }
}

async fn version(auth: super::AdminAuthReq) -> axum::response::Response {
    match page_context(&auth).await {
        Ok((admin, branding)) => match (
            server_admin_frontend::ssr::AdminSsrText::try_from(
                str_constants::VERSION_ALT.to_owned(),
            ),
            server_admin_frontend::ssr::AdminSsrText::try_from(
                git_info::PROJECT_GIT_INFO.commit.to_string(),
            ),
        ) {
            (Ok(title), Ok(text)) => {
                html_response(server_admin_frontend::ssr::render_text_page_with_access(
                    server_admin_contract::AdminPage::Version,
                    title,
                    text,
                    &admin,
                    &branding,
                ))
            }
            (Err(_error), _) | (_, Err(_error)) => {
                axum::response::IntoResponse::into_response(http::StatusCode::INTERNAL_SERVER_ERROR)
            }
        },
        Err(error) => html_page_error(error),
    }
}

async fn open_api(auth: super::AdminAuthReq) -> axum::response::Response {
    let branding_result = super::handlers::branding_view(auth.clone()).await;
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
                    server_admin_frontend::ssr::AdminSsrText::try_from(
                        str_constants::OPENAPI_DOCUMENT.to_owned(),
                    ),
                    server_admin_frontend::ssr::AdminSsrText::try_from(text),
                ) {
                    (Ok(title), Ok(text)) => {
                        html_response(server_admin_frontend::ssr::render_text_page_with_access(
                            server_admin_contract::AdminPage::OpenApi,
                            title,
                            text,
                            &admin,
                            &branding,
                        ))
                    }
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

async fn root() -> axum::response::Response {
    axum::response::IntoResponse::into_response(axum::response::Redirect::to(
        server_admin_contract::AdminFrontendPath::Dashboard.get(),
    ))
}

async fn sign_out(auth: super::AdminAuthReq) -> axum::response::Response {
    match form_auth(auth) {
        Ok(auth) => match super::handlers::sign_out(auth).await {
            Ok(response) => {
                redirect_with_headers(server_admin_contract::AdminFrontendPath::SignIn, &response)
            }
            Err(error) => axum::response::IntoResponse::into_response(error),
        },
        Err(error) => axum::response::IntoResponse::into_response(error),
    }
}

async fn change_password(
    auth: super::AdminAuthReq,
    super::AxumAdminForm(form): super::AxumAdminForm<ChangePasswordForm>,
) -> axum::response::Response {
    match form_auth(auth) {
        Ok(auth) => {
            let request = server_admin_contract::AdminChangeOwnPasswordReq::new(
                form.current_password,
                form.new_password,
                form.revoke_other_sessions,
            );
            match super::handlers::change_own_password(auth, super::AxumAdminJson(request)).await {
                Ok(_response) => {
                    success_redirect(server_admin_contract::AdminFrontendPath::Profile)
                }
                Err(error) => axum::response::IntoResponse::into_response(error),
            }
        }
        Err(error) => axum::response::IntoResponse::into_response(error),
    }
}

async fn revoke_session(
    auth: super::AdminAuthReq,
    super::AxumAdminForm(form): super::AxumAdminForm<RevokeSessionForm>,
) -> axum::response::Response {
    if !bool::from(form.confirmation) {
        return axum::response::IntoResponse::into_response(super::AdminApiError::Validation);
    }
    let session_id = form
        .session_id
        .to_string()
        .parse::<uuid::Uuid>()
        .map(super::super::UuidAdminValue::from)
        .map(super::super::AdminSessionId::from);
    let Ok(session_id) = session_id else {
        return axum::response::IntoResponse::into_response(super::AdminApiError::Validation);
    };
    match form_auth(auth) {
        Ok(auth) => {
            match super::handlers::revoke_session(auth, super::AdminSessionPath(session_id)).await {
                Ok(_response) => {
                    success_redirect(server_admin_contract::AdminFrontendPath::Sessions)
                }
                Err(error) => axum::response::IntoResponse::into_response(error),
            }
        }
        Err(error) => axum::response::IntoResponse::into_response(error),
    }
}

async fn create_user(
    auth: super::AdminAuthReq,
    super::AxumAdminForm(form): super::AxumAdminForm<CreateUserForm>,
) -> axum::response::Response {
    let Ok(auth) = form_auth(auth) else {
        return axum::response::IntoResponse::into_response(super::AdminApiError::Csrf);
    };
    let request = server_admin_contract::AdminCreateUserReq::new(
        form.display_name,
        form.login,
        form.password,
    );
    action_result(
        super::handlers::create_user(auth, super::AxumAdminJson(request)).await,
        server_admin_contract::AdminFrontendPath::Users,
    )
}

async fn update_user(
    auth: super::AdminAuthReq,
    super::AxumAdminForm(form): super::AxumAdminForm<UpdateUserForm>,
) -> axum::response::Response {
    let Ok(auth) = form_auth(auth) else {
        return axum::response::IntoResponse::into_response(super::AdminApiError::Csrf);
    };
    let request =
        server_admin_contract::AdminUpdateUserReq::new(Some(form.display_name), Some(form.login));
    action_result(
        super::handlers::update_user(
            auth,
            super::AxumAdminPath(user_path(form.user_id)),
            super::AxumAdminJson(request),
        )
        .await,
        server_admin_contract::AdminFrontendPath::Users,
    )
}

async fn user_password(
    auth: super::AdminAuthReq,
    super::AxumAdminForm(form): super::AxumAdminForm<UserPasswordForm>,
) -> axum::response::Response {
    let Ok(auth) = form_auth(auth) else {
        return axum::response::IntoResponse::into_response(super::AdminApiError::Csrf);
    };
    action_result(
        super::handlers::set_user_password(
            auth,
            super::AxumAdminPath(user_path(form.user_id)),
            super::AxumAdminJson(server_admin_contract::AdminSetUserPasswordReq::new(
                form.password,
            )),
        )
        .await,
        server_admin_contract::AdminFrontendPath::Users,
    )
}

async fn user_ban(
    auth: super::AdminAuthReq,
    super::AxumAdminForm(form): super::AxumAdminForm<UserBanForm>,
) -> axum::response::Response {
    let Ok(auth) = form_auth(auth) else {
        return axum::response::IntoResponse::into_response(super::AdminApiError::Csrf);
    };
    action_result(
        super::handlers::set_user_ban(
            auth,
            super::AxumAdminPath(user_path(form.user_id)),
            super::AxumAdminJson(server_admin_contract::AdminSetUserBanReq::new(
                form.is_banned,
            )),
        )
        .await,
        server_admin_contract::AdminFrontendPath::Users,
    )
}

async fn delete_user(
    auth: super::AdminAuthReq,
    super::AxumAdminForm(form): super::AxumAdminForm<UserIdForm>,
) -> axum::response::Response {
    if !bool::from(form.confirmation) {
        return axum::response::IntoResponse::into_response(super::AdminApiError::Validation);
    }
    let Ok(auth) = form_auth(auth) else {
        return axum::response::IntoResponse::into_response(super::AdminApiError::Csrf);
    };
    action_result(
        super::handlers::delete_user(auth, super::AxumAdminPath(user_path(form.user_id))).await,
        server_admin_contract::AdminFrontendPath::Users,
    )
}

async fn user_roles(
    auth: super::AdminAuthReq,
    super::AxumAdminForm(form): super::AxumAdminForm<UserRolesForm>,
) -> axum::response::Response {
    let Ok(auth) = form_auth(auth) else {
        return axum::response::IntoResponse::into_response(super::AdminApiError::Csrf);
    };
    let expected = role_ids(&form.expected_role_ids);
    let selected_text = AdminHtmlFormText::try_from(
        form.selected
            .0
            .into_values()
            .map(|value| value.0)
            .collect::<Vec<_>>()
            .join(str_constants::COMMA_SPACE.trim()),
    );
    let selected = selected_text
        .map_err(|_error| super::AdminApiError::Validation)
        .and_then(|value| role_ids(&value));
    let (Ok(expected), Ok(selected)) = (expected, selected) else {
        return axum::response::IntoResponse::into_response(super::AdminApiError::Validation);
    };
    let request = server_admin_contract::AdminSetUserRolesReq::new(expected, selected);
    action_result(
        super::handlers::set_user_roles(
            auth,
            super::AxumAdminPath(user_path(form.user_id)),
            super::AxumAdminJson(request),
        )
        .await,
        server_admin_contract::AdminFrontendPath::Users,
    )
}

async fn create_role(
    auth: super::AdminAuthReq,
    super::AxumAdminForm(form): super::AxumAdminForm<CreateRoleForm>,
) -> axum::response::Response {
    let Ok(auth) = form_auth(auth) else {
        return axum::response::IntoResponse::into_response(super::AdminApiError::Csrf);
    };
    action_result(
        super::handlers::create_role(
            auth,
            super::AxumAdminJson(server_admin_contract::AdminCreateRoleReq::new(form.name)),
        )
        .await,
        server_admin_contract::AdminFrontendPath::Roles,
    )
}

async fn update_role(
    auth: super::AdminAuthReq,
    super::AxumAdminForm(form): super::AxumAdminForm<UpdateRoleForm>,
) -> axum::response::Response {
    let Ok(auth) = form_auth(auth) else {
        return axum::response::IntoResponse::into_response(super::AdminApiError::Csrf);
    };
    action_result(
        super::handlers::update_role(
            auth,
            super::AxumAdminPath(role_path(form.role_id)),
            super::AxumAdminJson(server_admin_contract::AdminUpdateRoleReq::new(form.name)),
        )
        .await,
        server_admin_contract::AdminFrontendPath::Roles,
    )
}

async fn delete_role(
    auth: super::AdminAuthReq,
    super::AxumAdminForm(form): super::AxumAdminForm<RoleIdForm>,
) -> axum::response::Response {
    if !bool::from(form.confirmation) {
        return axum::response::IntoResponse::into_response(super::AdminApiError::Validation);
    }
    let Ok(auth) = form_auth(auth) else {
        return axum::response::IntoResponse::into_response(super::AdminApiError::Csrf);
    };
    action_result(
        super::handlers::delete_role(auth, super::AxumAdminPath(role_path(form.role_id))).await,
        server_admin_contract::AdminFrontendPath::Roles,
    )
}

async fn role_permissions(
    auth: super::AdminAuthReq,
    super::AxumAdminForm(form): super::AxumAdminForm<RolePermissionsForm>,
) -> axum::response::Response {
    let Ok(auth) = form_auth(auth) else {
        return axum::response::IntoResponse::into_response(super::AdminApiError::Csrf);
    };
    let expected = permission_ids(&form.expected_permission_ids);
    let selected_text = AdminHtmlFormText::try_from(
        form.selected
            .0
            .into_values()
            .map(|value| value.0)
            .collect::<Vec<_>>()
            .join(str_constants::COMMA_SPACE.trim()),
    );
    let selected = selected_text
        .map_err(|_error| super::AdminApiError::Validation)
        .and_then(|value| permission_ids(&value));
    let (Ok(expected), Ok(selected)) = (expected, selected) else {
        return axum::response::IntoResponse::into_response(super::AdminApiError::Validation);
    };
    let request = server_admin_contract::AdminSetRolePermissionsReq::new(expected, selected);
    action_result(
        super::handlers::set_role_permissions(
            auth,
            super::AxumAdminPath(role_path(form.role_id)),
            super::AxumAdminJson(request),
        )
        .await,
        server_admin_contract::AdminFrontendPath::Roles,
    )
}

async fn update_settings(
    auth: super::AdminAuthReq,
    super::AxumAdminForm(form): super::AxumAdminForm<SettingsForm>,
) -> axum::response::Response {
    let Ok(auth) = form_auth(auth) else {
        return axum::response::IntoResponse::into_response(super::AdminApiError::Csrf);
    };
    let parsed = (
        optional_setting::<server_admin_contract::AdminMainLogo, _>(form.main_logo),
        optional_setting::<server_admin_contract::AdminOrganizationContacts, _>(
            form.organization_contacts,
        ),
        optional_setting::<server_admin_contract::AdminOrganizationName, _>(form.organization_name),
        optional_setting::<server_admin_contract::AdminPrimaryColor, _>(form.primary_color),
        optional_setting::<server_admin_contract::AdminSupportUrl, _>(form.support_url),
        optional_setting::<server_admin_contract::AdminTabTitle, _>(form.tab_title),
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
        return axum::response::IntoResponse::into_response(super::AdminApiError::Validation);
    };
    let mut clear = Vec::new();
    [
        (
            main_logo.is_none(),
            server_admin_contract::AdminOptionalSetting::MainLogo,
        ),
        (
            organization_contacts.is_none(),
            server_admin_contract::AdminOptionalSetting::OrganizationContacts,
        ),
        (
            organization_name.is_none(),
            server_admin_contract::AdminOptionalSetting::OrganizationName,
        ),
        (
            primary_color.is_none(),
            server_admin_contract::AdminOptionalSetting::PrimaryColor,
        ),
        (
            support_url.is_none(),
            server_admin_contract::AdminOptionalSetting::SupportUrl,
        ),
        (
            tab_title.is_none(),
            server_admin_contract::AdminOptionalSetting::TabTitle,
        ),
    ]
    .into_iter()
    .filter_map(|(is_empty, setting)| is_empty.then_some(setting))
    .for_each(|setting| clear.push(setting));
    let request = server_admin_contract::AdminUpdateSettingsReq::new(
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
        super::handlers::update_settings(auth, super::AxumAdminJson(request)).await,
        server_admin_contract::AdminFrontendPath::Settings,
    )
}

async fn finish_sign_in(
    auth: super::AdminAuthReq,
    peer: super::AdminPeerAddr,
    request: server_admin_contract::AdminSignInReq,
) -> axum::response::Response {
    let branding = super::handlers::branding_view(auth.clone()).await.ok();
    match super::handlers::sign_in(auth, peer, super::AdminSignInJson(request)).await {
        Ok(response) => {
            let source = response.0;
            let mut target =
                axum::response::IntoResponse::into_response(axum::response::Redirect::to(
                    server_admin_contract::AdminFrontendPath::Dashboard.get(),
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
            let message_result = server_admin_frontend::ssr::AdminSsrErrorMessage::try_from(
                String::from(str_constants::SIGN_IN_FAILED),
            );
            match message_result {
                Ok(error_message) => axum::response::IntoResponse::into_response((
                    http::StatusCode::UNAUTHORIZED,
                    axum::response::Html(String::from(server_admin_frontend::ssr::render_sign_in(
                        Some(error_message),
                        branding.as_ref(),
                    ))),
                )),
                Err(_message_error) => axum::response::IntoResponse::into_response(
                    http::StatusCode::INTERNAL_SERVER_ERROR,
                ),
            }
        }
    }
}

async fn sign_in(
    auth: super::AdminAuthReq,
    peer: super::AdminPeerAddr,
    super::AxumAdminForm(form): super::AxumAdminForm<SignInForm>,
) -> axum::response::Response {
    finish_sign_in(
        auth,
        peer,
        server_admin_contract::AdminSignInReq::new(form.login, form.password),
    )
    .await
}

pub(super) fn routes(
    state: super::StdSharedAdminAuthSvcState,
    swagger_enabled: super::AdminHtmlSwaggerEnabled,
) -> super::AxumAdminAuthRouter {
    let router = axum::Router::new();
    let router = if swagger_enabled.0 {
        router.route(
            server_admin_contract::AdminFrontendPath::OpenApi.get(),
            axum::routing::get(open_api),
        )
    } else {
        router
    };
    super::AxumAdminAuthRouter(
        router
            .route(
                server_admin_contract::AdminFrontendPath::Root.get(),
                axum::routing::get(root),
            )
            .route(
                server_admin_contract::AdminFrontendPath::SignIn.get(),
                axum::routing::get(sign_in_page),
            )
            .route(
                server_admin_contract::AdminFrontendPath::Dashboard.get(),
                axum::routing::get(dashboard),
            )
            .route(
                server_admin_contract::AdminFrontendPath::Users.get(),
                axum::routing::get(users),
            )
            .route(
                server_admin_contract::AdminFrontendPath::Roles.get(),
                axum::routing::get(roles),
            )
            .route(
                server_admin_contract::AdminFrontendPath::Permissions.get(),
                axum::routing::get(permissions),
            )
            .route(
                server_admin_contract::AdminFrontendPath::Sessions.get(),
                axum::routing::get(sessions),
            )
            .route(
                server_admin_contract::AdminFrontendPath::Profile.get(),
                axum::routing::get(profile),
            )
            .route(
                server_admin_contract::AdminFrontendPath::Settings.get(),
                axum::routing::get(settings),
            )
            .route(
                server_admin_contract::AdminFrontendPath::Audit.get(),
                axum::routing::get(audit),
            )
            .route(
                server_admin_contract::AdminFrontendPath::Version.get(),
                axum::routing::get(version),
            )
            .route(
                server_admin_contract::AdminHtmlAction::SignIn.get(),
                axum::routing::post(sign_in),
            )
            .route(
                server_admin_contract::AdminHtmlAction::SignOut.get(),
                axum::routing::post(sign_out),
            )
            .route(
                server_admin_contract::AdminHtmlAction::ProfilePassword.get(),
                axum::routing::post(change_password),
            )
            .route(
                server_admin_contract::AdminHtmlAction::SessionRevoke.get(),
                axum::routing::post(revoke_session),
            )
            .route(
                server_admin_contract::AdminHtmlAction::UserCreate.get(),
                axum::routing::post(create_user),
            )
            .route(
                server_admin_contract::AdminHtmlAction::UserUpdate.get(),
                axum::routing::post(update_user),
            )
            .route(
                server_admin_contract::AdminHtmlAction::UserPassword.get(),
                axum::routing::post(user_password),
            )
            .route(
                server_admin_contract::AdminHtmlAction::UserBan.get(),
                axum::routing::post(user_ban),
            )
            .route(
                server_admin_contract::AdminHtmlAction::UserDelete.get(),
                axum::routing::post(delete_user),
            )
            .route(
                server_admin_contract::AdminHtmlAction::UserRoles.get(),
                axum::routing::post(user_roles),
            )
            .route(
                server_admin_contract::AdminHtmlAction::RoleCreate.get(),
                axum::routing::post(create_role),
            )
            .route(
                server_admin_contract::AdminHtmlAction::RoleUpdate.get(),
                axum::routing::post(update_role),
            )
            .route(
                server_admin_contract::AdminHtmlAction::RoleDelete.get(),
                axum::routing::post(delete_role),
            )
            .route(
                server_admin_contract::AdminHtmlAction::RolePermissions.get(),
                axum::routing::post(role_permissions),
            )
            .route(
                server_admin_contract::AdminHtmlAction::SettingsUpdate.get(),
                axum::routing::post(update_settings),
            )
            .with_state(state),
    )
}

#[cfg(test)]
mod tests {
    #[test]
    fn successful_mutation_redirects_to_visible_server_feedback() {
        let response = super::success_redirect(server_admin_contract::AdminFrontendPath::Users);
        assert_eq!(response.status(), http::StatusCode::SEE_OTHER);
        assert_eq!(
            response.headers().get(http::header::LOCATION),
            Some(&http::HeaderValue::from_static("/admin/users#saved"))
        );
    }

    #[tokio::test]
    async fn role_assignment_form_accepts_dynamic_checkbox_fields() {
        let request = http::Request::builder()
            .method(http::Method::POST)
            .header(
                http::header::CONTENT_TYPE,
                str_constants::APPLICATION_X_WWW_FORM_URLENCODED,
            )
            .body(axum::body::Body::from(
                "expected_role_ids=1%2C2&user_id=7&role_1=1&role_2=2",
            ));
        let Ok(request) = request else {
            panic!("6f44bd85");
        };
        let result = <super::super::AxumAdminForm<super::UserRolesForm> as axum::extract::FromRequest<
            (),
        >>::from_request(request, &())
        .await;
        let Ok(super::super::AxumAdminForm(form)) = result else {
            panic!("f639d7d1");
        };

        assert_eq!(i64::from(form.user_id), 7i64);
        assert_eq!(form.expected_role_ids.0, "1,2");
        assert_eq!(form.selected.0.len(), 2usize);
    }
}
