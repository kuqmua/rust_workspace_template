#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug)]
pub struct AdminGeneratedAuthLayer {
    state: crate::domain_types::auth::SharedAdminAuthSvcStateArc,
}
impl From<crate::domain_types::auth::SharedAdminAuthSvcStateArc> for AdminGeneratedAuthLayer {
    fn from(value: crate::domain_types::auth::SharedAdminAuthSvcStateArc) -> Self {
        Self { state: value }
    }
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug)]
pub struct AdminGeneratedAuthService<Service> {
    inner: Service,
    state: crate::domain_types::auth::SharedAdminAuthSvcStateArc,
}
impl<Service> tower::Layer<Service> for AdminGeneratedAuthLayer {
    type Service = AdminGeneratedAuthService<Service>;
    fn layer(&self, inner: Service) -> Self::Service {
        AdminGeneratedAuthService {
            inner,
            state: self.state.clone(),
        }
    }
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
            let contract = crate::domain_types::generated_tables::AdminGeneratedTable::ALL
                .iter()
                .copied()
                .find_map(|table| {
                    table.route_contract(crate::domain_types::StdAdminStrRef::from(path))
                })
                .map(|contract| (contract.permission(), contract.mutates(), contract.method()))
                .or_else(|| {
                    path.ends_with(
                        server_admin_contract::domain_types::AdminFrontendPath::OpenApiDocument
                            .get(),
                    )
                    .then_some((
                        Some(crate::domain_types::StdAdminStrRef::from(
                            server_admin_contract::domain_types::AdminPermission::OpenApiRead
                                .as_str()
                                .get(),
                        )),
                        crate::domain_types::StdAdminBool::from(false),
                        frontend_contract::domain_types::HttpMethod::Get,
                    ))
                })
                .or_else(|| {
                    path.ends_with(
                        server_admin_contract::domain_types::AdminFrontendPath::Metrics.get(),
                    )
                    .then_some((
                        Some(crate::domain_types::StdAdminStrRef::from(
                            server_admin_contract::domain_types::AdminPermission::MetricsRead
                                .as_str()
                                .get(),
                        )),
                        crate::domain_types::StdAdminBool::from(false),
                        frontend_contract::domain_types::HttpMethod::Get,
                    ))
                });
            let Some((Some(permission), mutates, method)) = contract else {
                return Ok(axum::response::IntoResponse::into_response(
                    crate::domain_types::auth::AdminError::Authorization,
                ));
            };
            if !matches!(
                (req.method(), method),
                (
                    &http::Method::DELETE,
                    frontend_contract::domain_types::HttpMethod::Delete
                ) | (
                    &http::Method::GET,
                    frontend_contract::domain_types::HttpMethod::Get
                ) | (
                    &http::Method::PATCH,
                    frontend_contract::domain_types::HttpMethod::Patch
                ) | (
                    &http::Method::POST,
                    frontend_contract::domain_types::HttpMethod::Post
                ) | (
                    &http::Method::PUT,
                    frontend_contract::domain_types::HttpMethod::Put
                )
            ) {
                return Ok(axum::response::IntoResponse::into_response(
                    crate::domain_types::auth::AdminError::MethodNotAllowed,
                ));
            }
            let Some(peer) = req
                .extensions()
                .get::<axum::extract::ConnectInfo<std::net::SocketAddr>>()
                .map(|peer| {
                    crate::domain_types::auth::AdminPeerAddr::from(
                        crate::domain_types::AdminSocketAddr::from(peer.0),
                    )
                })
            else {
                return Ok(axum::response::IntoResponse::into_response(
                    crate::domain_types::auth::AdminError::Authentication,
                ));
            };
            let authenticated = match crate::domain_types::auth::authorize_generated_request(
                state.as_ref(),
                crate::domain_types::HttpAdminHeaderMapRef::from(req.headers()),
                peer,
                server_admin_contract::domain_types::AdminPermissionStrRef::from(permission.get()),
                mutates,
            )
            .await
            {
                Ok(value) => value,
                Err(error) => {
                    return Ok(axum::response::IntoResponse::into_response(error));
                }
            };
            let actor = match pg_table::domain_types::PgTableIdempotencyActor::try_from(
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
