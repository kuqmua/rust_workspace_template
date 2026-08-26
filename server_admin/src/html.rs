#![allow(clippy::single_call_fn)] // each server-rendered HTML endpoint is registered once in the Axum route inventory
#![allow(
    clippy::shadow_reuse,
    reason = "form adapters deliberately replace unvalidated extractor values with validated domain values"
)]

#[path = "application__html__actions.rs"]
mod actions;
#[path = "application__html__forms.rs"]
mod forms;
#[path = "application__html__pages.rs"]
mod pages;

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
    if !super::authorization::origin_is_present_and_allowed(
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
    value: forms::AdminHtmlFormText,
) -> Result<Option<Value>, super::AdminError>
where
    Value: TryFrom<String, Error = Error>,
{
    if value.trim().is_empty() {
        Ok(None)
    } else {
        Value::try_from(
            bounded_types::domain_types::bounded_string::BoundedString::<
                0,
                { constants_usize::VALUE_8_192 },
            >::from(value)
            .into_inner(),
        )
        .map(Some)
        .map_err(|_error| super::AdminError::Validation)
    }
}

fn role_ids(
    value: &forms::AdminHtmlFormText,
) -> Result<server_admin_contract::domain_types::AdminRoleIds, super::AdminError> {
    assignment_ids::<
        server_admin_contract::domain_types::AdminRoleId,
        _,
        server_admin_contract::domain_types::AdminRoleIds,
        _,
    >(value)
}

fn permission_ids(
    value: &forms::AdminHtmlFormText,
) -> Result<server_admin_contract::domain_types::AdminPermissionIds, super::AdminError> {
    assignment_ids::<
        server_admin_contract::domain_types::AdminPermissionId,
        _,
        server_admin_contract::domain_types::AdminPermissionIds,
        _,
    >(value)
}

fn assignment_ids<Id, IdError, Ids, IdsError>(
    value: &forms::AdminHtmlFormText,
) -> Result<Ids, super::AdminError>
where
    Id: TryFrom<i64, Error = IdError>,
    Ids: TryFrom<Vec<Id>, Error = IdsError>,
{
    if value.is_empty() {
        return Ids::try_from(Vec::new()).map_err(|_error| super::AdminError::Validation);
    }
    let values = value
        .split(',')
        .map(|item| {
            let parsed = item
                .parse::<i64>()
                .map_err(|_error| super::AdminError::Validation)?;
            Id::try_from(parsed).map_err(|_error| super::AdminError::Validation)
        })
        .collect::<Result<Vec<_>, _>>()?;
    Ids::try_from(values).map_err(|_error| super::AdminError::Validation)
}

fn authenticated_selected_form<Ids, Parse>(
    auth: super::AdminAuthReq,
    expected: &forms::AdminHtmlFormText,
    selected: forms::StdAdminHtmlSelected,
    parse: Parse,
) -> Result<(super::AdminAuthReq, Ids, Ids), super::AdminError>
where
    Parse: Fn(&forms::AdminHtmlFormText) -> Result<Ids, super::AdminError>,
{
    let auth = form_auth(auth)?;
    let expected = parse(expected)?;
    let separator = constants_str::COMMA_SPACE.trim();
    let selected = bounded_types::domain_types::btree::BoundedBTreeMap::<
        forms::AdminHtmlFormKey,
        forms::AdminHtmlFormText,
        { forms::ADMIN_HTML_FORM_SELECTED_MAX_ITEMS },
    >::from(selected);
    let capacity = selected
        .iter()
        .map(|(_key, value)| value.len().get())
        .sum::<usize>()
        .saturating_add(
            selected
                .len()
                .get()
                .saturating_sub(constants_usize::ONE)
                .saturating_mul(separator.len()),
        );
    let text = selected.into_values().enumerate().fold(
        String::with_capacity(capacity),
        |mut text, (index, value)| {
            if index > constants_usize::ZERO {
                text.push_str(separator);
            }
            text.push_str(value.as_ref());
            text
        },
    );
    let selected_ids = forms::AdminHtmlFormText::try_from(text)
        .map_err(|_error| super::AdminError::Validation)
        .and_then(|value| parse(&value))?;
    Ok((auth, expected, selected_ids))
}

pub(super) fn routes(
    state: super::SharedAdminAuthSvcStateArc,
    swagger_enabled: super::AdminHtmlSwaggerEnabled,
) -> super::AxumAdminAuthRouter {
    let router = axum::Router::from(pages::router()).merge(axum::Router::from(actions::router()));
    let router = if swagger_enabled.0 {
        router.merge(axum::Router::from(pages::swagger_router()))
    } else {
        router
    };
    super::AxumAdminAuthRouter(router.with_state(state))
}
#[cfg(test)]
#[path = "application__html__tests.rs"]
mod tests;
