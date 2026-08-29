impl<State> axum::extract::FromRequestParts<State> for crate::admin_peer_addr::AdminPeerAddr
where
    State: Send + Sync,
{
    type Rejection = crate::admin_error::AdminError;
    fn from_request_parts(
        parts: &mut http::request::Parts,
        _state: &State,
    ) -> impl Future<Output = Result<Self, Self::Rejection>> {
        std::future::ready(
            parts
                .extensions
                .get::<axum::extract::ConnectInfo<std::net::SocketAddr>>()
                .map(|value| {
                    Self::from(server_admin_core::admin_socket_addr::AdminSocketAddr::from(
                        value.0,
                    ))
                })
                .ok_or(crate::admin_error::AdminError::Authentication),
        )
    }
}
impl<S> axum::extract::FromRequestParts<S> for crate::http_admin_header_map::HttpAdminHeaderMap
where
    S: Send + Sync,
{
    type Rejection = std::convert::Infallible;
    fn from_request_parts(
        parts: &mut http::request::Parts,
        _state: &S,
    ) -> impl Future<Output = Result<Self, Self::Rejection>> {
        std::future::ready(Ok(Self::from(parts.headers.clone())))
    }
}
impl
    axum::extract::FromRequestParts<
        crate::shared_admin_auth_svc_state_arc::SharedAdminAuthSvcStateArc,
    > for crate::admin_auth_req::AdminAuthReq
{
    type Rejection = crate::admin_error::AdminError;
    fn from_request_parts(
        parts: &mut http::request::Parts,
        state: &crate::shared_admin_auth_svc_state_arc::SharedAdminAuthSvcStateArc,
    ) -> impl Future<Output = Result<Self, Self::Rejection>> {
        std::future::ready(
            parts
                .extensions
                .get::<axum::extract::ConnectInfo<std::net::SocketAddr>>()
                .map(|peer| Self {
                    headers: crate::http_admin_header_map::HttpAdminHeaderMap::from(
                        parts.headers.clone(),
                    ),
                    peer: crate::admin_peer_addr::AdminPeerAddr::from(
                        server_admin_core::admin_socket_addr::AdminSocketAddr::from(peer.0),
                    ),
                    state: state.clone(),
                })
                .ok_or(crate::admin_error::AdminError::Authentication),
        )
    }
}
impl<S> axum::extract::FromRequest<S> for crate::admin_sign_in_json::AdminSignInJson
where
    S: Send + Sync,
{
    type Rejection = crate::admin_error::AdminError;
    async fn from_request(req: axum::extract::Request, state: &S) -> Result<Self, Self::Rejection> {
        axum::Json::<server_admin_contract::admin_sign_in_req::AdminSignInReq>::from_request(
            req, state,
        )
        .await
        .map(|axum::Json(value)| Self::from(value))
        .map_err(|error| {
            crate::admin_error::AdminError::body_rejection(
                server_admin_core::std_admin_bool::StdAdminBool::from(
                    error.status() == http::StatusCode::PAYLOAD_TOO_LARGE,
                ),
            )
        })
    }
}
impl<S, Value> axum::extract::FromRequest<S> for crate::axum_admin_json::AxumAdminJson<Value>
where
    S: Send + Sync,
    Value: serde::de::DeserializeOwned + Send,
{
    type Rejection = crate::admin_error::AdminError;
    async fn from_request(req: axum::extract::Request, state: &S) -> Result<Self, Self::Rejection> {
        axum::Json::<Value>::from_request(req, state)
            .await
            .map(|axum::Json(value)| Self::from(value))
            .map_err(|error| {
                crate::admin_error::AdminError::body_rejection(
                    server_admin_core::std_admin_bool::StdAdminBool::from(
                        error.status() == http::StatusCode::PAYLOAD_TOO_LARGE,
                    ),
                )
            })
    }
}
impl<S, Value> axum::extract::FromRequest<S> for crate::axum_admin_form::AxumAdminForm<Value>
where
    S: Send + Sync,
    Value: serde::de::DeserializeOwned + Send,
{
    type Rejection = crate::admin_error::AdminError;
    async fn from_request(req: axum::extract::Request, state: &S) -> Result<Self, Self::Rejection> {
        axum::Form::<Value>::from_request(req, state)
            .await
            .map(|axum::Form(value)| Self::from(value))
            .map_err(|error| {
                crate::admin_error::AdminError::body_rejection(
                    server_admin_core::std_admin_bool::StdAdminBool::from(
                        error.status() == http::StatusCode::PAYLOAD_TOO_LARGE,
                    ),
                )
            })
    }
}
impl<S, Value> axum::extract::FromRequestParts<S> for crate::axum_admin_path::AxumAdminPath<Value>
where
    S: Send + Sync,
    Value: serde::de::DeserializeOwned + Send,
{
    type Rejection = crate::admin_error::AdminError;
    async fn from_request_parts(
        parts: &mut http::request::Parts,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        axum::extract::Path::<Value>::from_request_parts(parts, state)
            .await
            .map(|axum::extract::Path(value)| Self::from(value))
            .map_err(|_error| crate::admin_error::AdminError::Validation)
    }
}
impl<S, Value> axum::extract::FromRequestParts<S> for crate::axum_admin_query::AxumAdminQuery<Value>
where
    S: Send + Sync,
    Value: serde::de::DeserializeOwned + Send,
{
    type Rejection = crate::admin_error::AdminError;
    async fn from_request_parts(
        parts: &mut http::request::Parts,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        axum::extract::Query::<Value>::from_request_parts(parts, state)
            .await
            .map(|axum::extract::Query(value)| Self::from(value))
            .map_err(|_error| crate::admin_error::AdminError::Validation)
    }
}
impl
    axum::extract::FromRequestParts<
        crate::shared_admin_auth_svc_state_arc::SharedAdminAuthSvcStateArc,
    > for crate::admin_session_path::AdminSessionPath
{
    type Rejection = crate::admin_error::AdminError;
    async fn from_request_parts(
        parts: &mut http::request::Parts,
        state: &crate::shared_admin_auth_svc_state_arc::SharedAdminAuthSvcStateArc,
    ) -> Result<Self, Self::Rejection> {
        axum::extract::Path::<uuid::Uuid>::from_request_parts(parts, state)
            .await
            .map(|axum::extract::Path(value)| {
                Self::from(crate::admin_session_id::AdminSessionId::from(
                    server_admin_core::uuid_admin_value::UuidAdminValue::from(value),
                ))
            })
            .map_err(|_error| crate::admin_error::AdminError::Validation)
    }
}
