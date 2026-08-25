impl<State> axum::extract::FromRequestParts<State> for super::AdminPeerAddr
where
    State: Send + Sync,
{
    type Rejection = super::AdminError;
    fn from_request_parts(
        parts: &mut http::request::Parts,
        _state: &State,
    ) -> impl Future<Output = Result<Self, Self::Rejection>> {
        std::future::ready(
            parts
                .extensions
                .get::<axum::extract::ConnectInfo<std::net::SocketAddr>>()
                .map(|value| Self::from(super::super::AdminSocketAddr::from(value.0)))
                .ok_or(super::AdminError::Authentication),
        )
    }
}
impl<S> axum::extract::FromRequestParts<S> for super::HttpAdminHeaderMap
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
impl axum::extract::FromRequestParts<super::SharedAdminAuthSvcStateArc> for super::AdminAuthReq {
    type Rejection = super::AdminError;
    fn from_request_parts(
        parts: &mut http::request::Parts,
        state: &super::SharedAdminAuthSvcStateArc,
    ) -> impl Future<Output = Result<Self, Self::Rejection>> {
        std::future::ready(
            parts
                .extensions
                .get::<axum::extract::ConnectInfo<std::net::SocketAddr>>()
                .map(|peer| Self {
                    headers: super::HttpAdminHeaderMap::from(parts.headers.clone()),
                    peer: super::AdminPeerAddr::from(super::super::AdminSocketAddr::from(peer.0)),
                    state: state.clone(),
                })
                .ok_or(super::AdminError::Authentication),
        )
    }
}
impl<S> axum::extract::FromRequest<S> for super::AdminSignInJson
where
    S: Send + Sync,
{
    type Rejection = super::AdminError;
    async fn from_request(req: axum::extract::Request, state: &S) -> Result<Self, Self::Rejection> {
        axum::Json::<server_admin_contract::domain_types::AdminSignInReq>::from_request(req, state)
            .await
            .map(|axum::Json(value)| Self::from(value))
            .map_err(|error| {
                if error.status() == http::StatusCode::PAYLOAD_TOO_LARGE {
                    super::AdminError::PayloadTooLarge
                } else {
                    super::AdminError::Validation
                }
            })
    }
}
impl<S, Value> axum::extract::FromRequest<S> for super::AxumAdminJson<Value>
where
    S: Send + Sync,
    Value: serde::de::DeserializeOwned + Send,
{
    type Rejection = super::AdminError;
    async fn from_request(req: axum::extract::Request, state: &S) -> Result<Self, Self::Rejection> {
        axum::Json::<Value>::from_request(req, state)
            .await
            .map(|axum::Json(value)| Self::from(value))
            .map_err(|error| {
                if error.status() == http::StatusCode::PAYLOAD_TOO_LARGE {
                    super::AdminError::PayloadTooLarge
                } else {
                    super::AdminError::Validation
                }
            })
    }
}
impl<S, Value> axum::extract::FromRequest<S> for super::AxumAdminForm<Value>
where
    S: Send + Sync,
    Value: serde::de::DeserializeOwned + Send,
{
    type Rejection = super::AdminError;
    async fn from_request(req: axum::extract::Request, state: &S) -> Result<Self, Self::Rejection> {
        axum::Form::<Value>::from_request(req, state)
            .await
            .map(|axum::Form(value)| Self::from(value))
            .map_err(|error| {
                if error.status() == http::StatusCode::PAYLOAD_TOO_LARGE {
                    super::AdminError::PayloadTooLarge
                } else {
                    super::AdminError::Validation
                }
            })
    }
}
impl<S, Value> axum::extract::FromRequestParts<S> for super::AxumAdminPath<Value>
where
    S: Send + Sync,
    Value: serde::de::DeserializeOwned + Send,
{
    type Rejection = super::AdminError;
    async fn from_request_parts(
        parts: &mut http::request::Parts,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        axum::extract::Path::<Value>::from_request_parts(parts, state)
            .await
            .map(|axum::extract::Path(value)| Self::from(value))
            .map_err(|_error| super::AdminError::Validation)
    }
}
impl<S, Value> axum::extract::FromRequestParts<S> for super::AxumAdminQuery<Value>
where
    S: Send + Sync,
    Value: serde::de::DeserializeOwned + Send,
{
    type Rejection = super::AdminError;
    async fn from_request_parts(
        parts: &mut http::request::Parts,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        axum::extract::Query::<Value>::from_request_parts(parts, state)
            .await
            .map(|axum::extract::Query(value)| Self::from(value))
            .map_err(|_error| super::AdminError::Validation)
    }
}
impl axum::extract::FromRequestParts<super::SharedAdminAuthSvcStateArc>
    for super::AdminSessionPath
{
    type Rejection = super::AdminError;
    async fn from_request_parts(
        parts: &mut http::request::Parts,
        state: &super::SharedAdminAuthSvcStateArc,
    ) -> Result<Self, Self::Rejection> {
        axum::extract::Path::<uuid::Uuid>::from_request_parts(parts, state)
            .await
            .map(|axum::extract::Path(value)| {
                Self::from(super::super::AdminSessionId::from(
                    super::super::UuidAdminValue::from(value),
                ))
            })
            .map_err(|_error| super::AdminError::Validation)
    }
}
