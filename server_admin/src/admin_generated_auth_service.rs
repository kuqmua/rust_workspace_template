#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the generated-auth layer constructs this service while fields remain private outside the facade"
)]
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug)]
pub struct AdminGeneratedAuthService<Service> {
    pub(crate) inner: Service,
    pub(crate) state: crate::shared_admin_auth_svc_state_arc::SharedAdminAuthSvcStateArc,
}
impl<Service> tower::Service<axum::extract::Request> for AdminGeneratedAuthService<Service>
where
    Service: tower::Service<axum::extract::Request, Response = axum::response::Response>
        + Clone
        + Send
        + 'static,
    Service::Future: Send + 'static,
    Service::Error: Send + 'static,
{
    type Error = Service::Error;
    type Future = std::pin::Pin<
        Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send + 'static>,
    >;
    type Response = axum::response::Response;
    fn call(&mut self, mut req: axum::extract::Request) -> Self::Future {
        let mut inner = self.inner.clone();
        std::mem::swap(&mut inner, &mut self.inner);
        let state = self.state.clone();
        Box::pin(async move {
            let path = req.uri().path();
            let contract = crate::admin_generated_table::AdminGeneratedTable::ALL
                .iter()
                .copied()
                .find_map(|table| {
                    table.route_contract(server_admin_core::std_admin_str_ref::StdAdminStrRef::from(path))
                })
                .map(|contract| (contract.permission(), contract.mutates(), contract.method()))
                .or_else(|| {
                    path.ends_with(
                        server_admin_contract::admin_frontend_path::AdminFrontendPath::OpenApiDocument
                            .get(),
                    )
                    .then_some((
                        Some(server_admin_core::std_admin_str_ref::StdAdminStrRef::from(
                            server_admin_contract::admin_permission::AdminPermission::OpenApiRead
                                .as_str()
                                .get(),
                        )),
                        server_admin_core::std_admin_bool::StdAdminBool::from(false),
                        frontend_contract::route_method::RouteMethod::Get,
                    ))
                })
                .or_else(|| {
                    path.ends_with(
                        server_admin_contract::admin_frontend_path::AdminFrontendPath::Metrics.get(),
                    )
                    .then_some((
                        Some(server_admin_core::std_admin_str_ref::StdAdminStrRef::from(
                            server_admin_contract::admin_permission::AdminPermission::MetricsRead
                                .as_str()
                                .get(),
                        )),
                        server_admin_core::std_admin_bool::StdAdminBool::from(false),
                        frontend_contract::route_method::RouteMethod::Get,
                    ))
                });
            let Some((Some(permission), mutates, method)) = contract else {
                return Ok(axum::response::IntoResponse::into_response(
                    crate::admin_error::AdminError::Authorization,
                ));
            };
            if !matches!(
                (req.method(), method),
                (
                    &http::Method::DELETE,
                    frontend_contract::route_method::RouteMethod::Delete
                ) | (
                    &http::Method::GET,
                    frontend_contract::route_method::RouteMethod::Get
                ) | (
                    &http::Method::PATCH,
                    frontend_contract::route_method::RouteMethod::Patch
                ) | (
                    &http::Method::POST,
                    frontend_contract::route_method::RouteMethod::Post
                ) | (
                    &http::Method::PUT,
                    frontend_contract::route_method::RouteMethod::Put
                )
            ) {
                return Ok(axum::response::IntoResponse::into_response(
                    crate::admin_error::AdminError::MethodNotAllowed,
                ));
            }
            let Some(peer) = req
                .extensions()
                .get::<axum::extract::ConnectInfo<std::net::SocketAddr>>()
                .map(|peer| {
                    crate::admin_peer_addr::AdminPeerAddr::from(
                        server_admin_core::admin_socket_addr::AdminSocketAddr::from(peer.0),
                    )
                })
            else {
                return Ok(axum::response::IntoResponse::into_response(
                    crate::admin_error::AdminError::Authentication,
                ));
            };
            let authenticated =
                match crate::authorization_authorize_generated_request::authorization_authorize_generated_request(
                    state.as_ref(),
                    crate::http_admin_header_map_ref::HttpAdminHeaderMapRef::from(req.headers()),
                    peer,
                    server_admin_contract::admin_permission_str_ref::AdminPermissionStrRef::from(
                        permission.get(),
                    ),
                    mutates,
                )
                .await
                {
                    Ok(value) => value,
                    Err(error) => {
                        return Ok(axum::response::IntoResponse::into_response(error));
                    }
                };
            let actor =
                match pg_table::pg_table_idempotency_actor::PgTableIdempotencyActor::try_from(
                    authenticated.id().to_string(),
                ) {
                    Ok(value) => value,
                    Err(_error) => {
                        return Ok(axum::response::IntoResponse::into_response(
                            http::StatusCode::INTERNAL_SERVER_ERROR,
                        ));
                    }
                };
            let _previous_actor = req.extensions_mut().insert(actor);
            tower::Service::call(&mut inner, req).await
        })
    }
    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        tower::Service::poll_ready(&mut self.inner, cx)
    }
}
