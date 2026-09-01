#[must_use]
pub fn admin_api_open_api() -> crate::utoipa_admin_auth_open_api::UtoipaAdminAuthOpenApi {
    let mut document = crate::admin_auth_route_registry::open_api();
    let body_limit_description =
        <server_admin_contract::admin_route::AdminAuthenticationRouteFamily as frontend_contract::route_family::RouteFamily>::body_limit()
            .map(|limit| {
                format!(
                    "{}{}",
                    constants_str::OPENAPI_REQUEST_BODY_MAXIMUM_BYTES_PREFIX,
                    limit.get()
                )
            });
    document
        .paths
        .paths
        .values_mut()
        .flat_map(|path| {
            [
                path.get.as_mut(),
                path.put.as_mut(),
                path.post.as_mut(),
                path.delete.as_mut(),
                path.options.as_mut(),
                path.head.as_mut(),
                path.patch.as_mut(),
                path.trace.as_mut(),
            ]
            .into_iter()
            .flatten()
        })
        .for_each(|operation| {
            if let (Some(request_body), Some(description)) = (
                operation.request_body.as_mut(),
                body_limit_description.as_ref(),
            ) {
                request_body.description = Some(description.clone());
            }
        });
    if let Some(components) = document.components.as_mut() {
        components.add_security_scheme(
            constants_str::ADMIN_COOKIE,
            utoipa::openapi::security::SecurityScheme::ApiKey(
                utoipa::openapi::security::ApiKey::Cookie(
                    utoipa::openapi::security::ApiKeyValue::with_description(
                        constants_str::SERVER_ADMIN_ACCESS_COOKIE_NAME,
                        constants_str::HTTPONLY_ADMINISTRATOR_ACCESS_TOKEN_COOKIE,
                    ),
                ),
            ),
        );
        components.add_security_scheme(
            constants_str::ADMIN_CSRF,
            utoipa::openapi::security::SecurityScheme::ApiKey(
                utoipa::openapi::security::ApiKey::Header(
                    utoipa::openapi::security::ApiKeyValue::with_description(
                        constants_str::X_CSRF_TOKEN,
                        constants_str::CSRF_TOKEN_BOUND_TO_THE_ADMINISTRATOR_ACCESS_SESSION,
                    ),
                ),
            ),
        );
    }
    crate::utoipa_admin_auth_open_api::UtoipaAdminAuthOpenApi::from(document)
}
