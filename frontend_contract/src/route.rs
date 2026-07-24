pub trait RouteTransport {}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PublicTransport;
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AuthenticatedTransport;
impl RouteTransport for PublicTransport {}
impl RouteTransport for AuthenticatedTransport {}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RouteMethod {
    Connect,
    Delete,
    Get,
    Head,
    Options,
    Patch,
    Post,
    Put,
    Trace,
}
impl RouteMethod {
    #[must_use]
    pub fn as_str(self) -> crate::ContractStr {
        crate::ContractStr::from(match self {
            Self::Connect => str_constants::CONNECT,
            Self::Delete => str_constants::DELETE,
            Self::Get => str_constants::GET,
            Self::Head => str_constants::HEAD,
            Self::Options => str_constants::OPTIONS,
            Self::Patch => str_constants::PATCH,
            Self::Post => str_constants::POST,
            Self::Put => str_constants::PUT,
            Self::Trace => str_constants::TRACE,
        })
    }
}
#[derive(Clone, Copy, Debug, newtype::FromInner, newtype::IntoInnerFrom)]
pub struct AxumMethodFilter(axum::routing::MethodFilter);
#[must_use]
pub fn axum_method_filter(method: crate::HttpMethod) -> AxumMethodFilter {
    AxumMethodFilter::from(match method {
        crate::HttpMethod::Connect => axum::routing::MethodFilter::CONNECT,
        crate::HttpMethod::Delete => axum::routing::MethodFilter::DELETE,
        crate::HttpMethod::Get => axum::routing::MethodFilter::GET,
        crate::HttpMethod::Head => axum::routing::MethodFilter::HEAD,
        crate::HttpMethod::Options => axum::routing::MethodFilter::OPTIONS,
        crate::HttpMethod::Patch => axum::routing::MethodFilter::PATCH,
        crate::HttpMethod::Post => axum::routing::MethodFilter::POST,
        crate::HttpMethod::Put => axum::routing::MethodFilter::PUT,
        crate::HttpMethod::Trace => axum::routing::MethodFilter::TRACE,
    })
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RouteMetadata {
    authentication: crate::AuthenticationRequirement,
    error_statuses: &'static [crate::RouteErrorStatus],
    method: RouteMethod,
    mutation: crate::RouteMutation,
    openapi_operation_id: crate::ContractStr,
    path: crate::ContractStr,
    success_status: crate::SuccessStatus,
}
impl RouteMetadata {
    #[must_use]
    pub const fn new(
        method: RouteMethod,
        openapi_operation_id: crate::ContractStr,
        path: crate::ContractStr,
    ) -> Self {
        Self::new_with_policy(
            crate::AuthenticationRequirement::Public,
            &[],
            method,
            crate::RouteMutation::ReadOnly,
            openapi_operation_id,
            path,
            crate::SuccessStatus::Code200,
        )
    }
    #[must_use]
    pub const fn new_with_policy(
        authentication: crate::AuthenticationRequirement,
        error_statuses: &'static [crate::RouteErrorStatus],
        method: RouteMethod,
        mutation: crate::RouteMutation,
        openapi_operation_id: crate::ContractStr,
        path: crate::ContractStr,
        success_status: crate::SuccessStatus,
    ) -> Self {
        Self {
            authentication,
            error_statuses,
            method,
            mutation,
            openapi_operation_id,
            path,
            success_status,
        }
    }
    #[must_use]
    pub const fn authentication(self) -> crate::AuthenticationRequirement {
        self.authentication
    }
    #[must_use]
    pub const fn error_statuses(self) -> &'static [crate::RouteErrorStatus] {
        self.error_statuses
    }
    #[must_use]
    pub const fn access(self) -> crate::RouteAccess {
        match self.authentication {
            crate::AuthenticationRequirement::Public => crate::RouteAccess::Public,
            crate::AuthenticationRequirement::Authenticated
            | crate::AuthenticationRequirement::Permission(_) => crate::RouteAccess::Authenticated,
        }
    }
    #[must_use]
    pub fn method(self) -> crate::ContractStr {
        self.method.as_str()
    }
    #[must_use]
    pub const fn route_method(self) -> RouteMethod {
        self.method
    }
    #[must_use]
    pub const fn openapi_operation_id(self) -> crate::ContractStr {
        self.openapi_operation_id
    }
    #[must_use]
    pub const fn path(self) -> crate::ContractStr {
        self.path
    }
    #[must_use]
    pub const fn mutation(self) -> crate::RouteMutation {
        self.mutation
    }
    #[must_use]
    pub const fn success_status(self) -> crate::SuccessStatus {
        self.success_status
    }
    #[must_use]
    pub const fn contract(self) -> crate::RouteContract {
        crate::RouteContract::new(
            self.authentication,
            match self.method {
                RouteMethod::Connect => crate::HttpMethod::Connect,
                RouteMethod::Delete => crate::HttpMethod::Delete,
                RouteMethod::Get => crate::HttpMethod::Get,
                RouteMethod::Head => crate::HttpMethod::Head,
                RouteMethod::Options => crate::HttpMethod::Options,
                RouteMethod::Patch => crate::HttpMethod::Patch,
                RouteMethod::Post => crate::HttpMethod::Post,
                RouteMethod::Put => crate::HttpMethod::Put,
                RouteMethod::Trace => crate::HttpMethod::Trace,
            },
            match self.mutation {
                crate::RouteMutation::ReadOnly => crate::MutationKind::ReadOnly,
                crate::RouteMutation::Mutating => crate::MutationKind::Mutating,
            },
            self.path,
            self.success_status,
        )
    }
}
#[derive(newtype::FromInner)]
pub struct UtoipaOpenApiComponentsRefMut<'value_lt>(
    &'value_lt mut utoipa::openapi::schema::Components,
);
impl std::fmt::Debug for UtoipaOpenApiComponentsRefMut<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(str_constants::UTOIPAOPENAPICOMPONENTSREFMUT)
            .finish_non_exhaustive()
    }
}
#[derive(newtype::FromInner)]
pub struct UtoipaOpenApiRefMut<'value_lt>(&'value_lt mut utoipa::openapi::OpenApi);
impl std::fmt::Debug for UtoipaOpenApiRefMut<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(str_constants::UTOIPAOPENAPIREFMUT)
            .finish_non_exhaustive()
    }
}
pub trait TypedRoute: Sized {
    type Request: serde::Serialize + serde::de::DeserializeOwned;
    type Response: serde::Serialize + serde::de::DeserializeOwned;
    type Transport: RouteTransport;
    fn metadata() -> RouteMetadata;
    #[must_use]
    fn openapi_request_schema() -> Option<UtoipaOpenApiRouteSchema> {
        None
    }
    #[must_use]
    fn openapi_request_body_schema() -> Option<UtoipaOpenApiRouteSchema> {
        None
    }
    #[must_use]
    fn openapi_response_schema() -> Option<UtoipaOpenApiRouteSchema> {
        None
    }
    #[must_use]
    fn openapi_error_response_schema(
        _status: crate::RouteErrorStatus,
    ) -> Option<UtoipaOpenApiRouteSchema> {
        Some(UtoipaOpenApiRouteSchema::from(
            <crate::ApiProblem as utoipa::ToSchema>::schema().1,
        ))
    }
    #[must_use]
    fn openapi_path_parameter() -> Option<UtoipaOpenApiPathParameter> {
        None
    }
    #[must_use]
    fn request_body() -> RouteRequestBody {
        RouteRequestBody::Absent
    }
    fn register_openapi_schemas(_components: &mut UtoipaOpenApiComponentsRefMut<'_>) {}
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RouteRequestBody {
    Absent,
    Json,
}
#[derive(Clone, Debug)]
pub struct RouteSchemaContract {
    metadata: RouteMetadata,
    request_schema: Option<UtoipaOpenApiRouteSchema>,
    response_schema: Option<UtoipaOpenApiRouteSchema>,
}
impl RouteSchemaContract {
    #[must_use]
    pub fn from_typed_route<Route>() -> Self
    where
        Route: TypedRoute,
    {
        Self {
            metadata: Route::metadata(),
            request_schema: Route::openapi_request_schema(),
            response_schema: Route::openapi_response_schema(),
        }
    }
    #[must_use]
    pub const fn metadata(&self) -> RouteMetadata {
        self.metadata
    }
    #[must_use]
    pub const fn request_schema(&self) -> Option<&UtoipaOpenApiRouteSchema> {
        self.request_schema.as_ref()
    }
    #[must_use]
    pub const fn response_schema(&self) -> Option<&UtoipaOpenApiRouteSchema> {
        self.response_schema.as_ref()
    }
}
#[derive(Clone, newtype::FromInner, newtype::IntoInnerFrom)]
pub struct UtoipaOpenApiRouteSchema(utoipa::openapi::RefOr<utoipa::openapi::Schema>);
impl std::fmt::Debug for UtoipaOpenApiRouteSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(str_constants::OPEN_API_ROUTE_SCHEMA)
            .finish_non_exhaustive()
    }
}
#[derive(Clone, newtype::FromInner, newtype::IntoInnerFrom)]
pub struct UtoipaOpenApiPathParameter(utoipa::openapi::path::Parameter);
impl std::fmt::Debug for UtoipaOpenApiPathParameter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(str_constants::UTOIPAOPENAPIPATHPARAMETER)
            .finish_non_exhaustive()
    }
}
#[derive(Clone, Debug, Default, Eq, PartialEq, newtype::IntoInnerFrom)]
pub struct ParameterizedRoutePath(String);
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ParameterizedRoutePathTryFromStringError;
impl TryFrom<String> for ParameterizedRoutePath {
    type Error = ParameterizedRoutePathTryFromStringError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() > 8192usize {
            Err(ParameterizedRoutePathTryFromStringError)
        } else {
            Ok(Self(value))
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, newtype::FromInner)]
pub struct OpenApiSecuritySchemeRef<'value_lt>(&'value_lt str);
pub trait CoveredRoute: TypedRoute {
    fn coverage_descriptor() -> crate::RouteCoverageDescriptor;
}
pub trait ParameterizedRoute: TypedRoute {
    type Parameter;
    fn path(parameter: &Self::Parameter) -> ParameterizedRoutePath;
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, newtype::FromInner)]
pub struct RouteBodyLimit(usize);
#[derive(
    Clone,
    Debug,
    Default,
    Eq,
    PartialEq,
    newtype::AsRefTarget,
    newtype::FromInner,
    newtype::IntoInnerFrom,
)]
pub struct RouteCoverageDescriptors(Vec<crate::RouteCoverageDescriptor>);
#[derive(
    Clone, Debug, Default, newtype::AsRefTarget, newtype::FromInner, newtype::IntoInnerFrom,
)]
pub struct RouteSchemaContracts(Vec<RouteSchemaContract>);
#[derive(
    Clone,
    Debug,
    Default,
    Eq,
    PartialEq,
    newtype::AsRefTarget,
    newtype::FromInner,
    newtype::IntoInnerFrom,
)]
pub struct RouteMetadataList(Vec<RouteMetadata>);
impl RouteBodyLimit {
    #[must_use]
    pub const fn get(self) -> usize {
        self.0
    }
}
pub trait RouteFamily {
    const ROUTE_COUNT: usize = 0usize;
    #[must_use]
    fn body_limit() -> Option<RouteBodyLimit> {
        None
    }
    fn coverage_descriptors() -> RouteCoverageDescriptors;
    #[must_use]
    fn schema_contracts() -> RouteSchemaContracts {
        RouteSchemaContracts::default()
    }
    fn route_metadata() -> RouteMetadataList {
        Vec::from(Self::coverage_descriptors())
            .into_iter()
            .map(crate::RouteCoverageDescriptor::metadata)
            .collect::<Vec<_>>()
            .into()
    }
}
pub trait RouteInFamily<Family>
where
    Family: RouteFamily,
{
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RouteRequest<Route>
where
    Route: TypedRoute,
{
    body: Route::Request,
}
impl<Route> RouteRequest<Route>
where
    Route: TypedRoute,
{
    #[must_use]
    pub const fn new(body: Route::Request) -> Self {
        Self { body }
    }
    #[must_use]
    pub const fn body(&self) -> &Route::Request {
        &self.body
    }
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RouteResponse<Route>
where
    Route: TypedRoute,
{
    body: Route::Response,
}
impl<Route> RouteResponse<Route>
where
    Route: TypedRoute,
{
    #[must_use]
    pub const fn body(&self) -> &Route::Response {
        &self.body
    }
}
#[must_use]
pub const fn client_request<Route>(body: Route::Request) -> RouteRequest<Route>
where
    Route: TypedRoute,
{
    RouteRequest::new(body)
}
#[must_use]
pub const fn server_response<Route>(body: Route::Response) -> RouteResponse<Route>
where
    Route: TypedRoute,
{
    RouteResponse { body }
}
#[must_use]
pub fn client_route_metadata<Route>() -> RouteMetadata
where
    Route: TypedRoute,
{
    Route::metadata()
}
#[must_use]
pub fn server_route_metadata<Route>() -> RouteMetadata
where
    Route: TypedRoute,
{
    Route::metadata()
}
#[must_use]
pub fn openapi_route_metadata<Route>() -> RouteMetadata
where
    Route: TypedRoute,
{
    Route::metadata()
}
pub fn apply_openapi_success_contract<Route>(operation: &mut utoipa::openapi::path::Operation)
where
    Route: TypedRoute,
{
    let metadata = Route::metadata();
    operation
        .responses
        .responses
        .retain(|status, _response| !status.starts_with('2'));
    let status = metadata.success_status().transport_status().to_string();
    let mut response = utoipa::openapi::response::Response::new(status.clone());
    if metadata.success_status() != crate::SuccessStatus::Code204
        && let Some(schema) = Route::openapi_response_schema()
    {
        let _previous_content = response.content.insert(
            str_constants::APPLICATION_JSON.to_owned(),
            utoipa::openapi::Content::new::<utoipa::openapi::RefOr<utoipa::openapi::Schema>>(
                schema.into(),
            ),
        );
    }
    let _previous_response = operation
        .responses
        .responses
        .insert(status, utoipa::openapi::RefOr::T(response));
}
pub fn apply_openapi_request_contract<Route>(operation: &mut utoipa::openapi::path::Operation)
where
    Route: TypedRoute,
{
    operation.request_body = match Route::request_body() {
        RouteRequestBody::Absent => None,
        RouteRequestBody::Json => Route::openapi_request_body_schema().map(|schema| {
            utoipa::openapi::request_body::RequestBodyBuilder::new()
                .required(Some(utoipa::openapi::Required::True))
                .content(
                    str_constants::APPLICATION_JSON,
                    utoipa::openapi::Content::new::<
                        utoipa::openapi::RefOr<utoipa::openapi::Schema>,
                    >(schema.into()),
                )
                .build()
        }),
    };
}
#[allow(clippy::needless_for_each)] // iterator form follows the workspace no-for-loop policy
pub fn register_openapi_schema<'schema_lt, Schema>(
    components: &mut UtoipaOpenApiComponentsRefMut<'_>,
) where
    Schema: utoipa::ToSchema<'schema_lt>,
{
    let aliases = Schema::aliases();
    if aliases.is_empty() {
        let (name, schema) = Schema::schema();
        let qualified_name = std::any::type_name::<Schema>()
            .replace(str_constants::DOUBLE_COLON, str_constants::DOT);
        let _previous_qualified_schema =
            components.0.schemas.insert(qualified_name, schema.clone());
        if let Some(crate_name) = std::any::type_name::<Schema>()
            .split(str_constants::DOUBLE_COLON)
            .next()
        {
            let _previous_crate_schema = components
                .0
                .schemas
                .insert(format!("{crate_name}.{name}"), schema.clone());
        }
        let _previous_named_schema = components.0.schemas.insert(name.to_owned(), schema);
    } else {
        aliases.into_iter().for_each(|(name, schema)| {
            let _previous_schema = components
                .0
                .schemas
                .insert(name.to_owned(), utoipa::openapi::RefOr::T(schema));
        });
    }
}
pub fn register_openapi_route_schemas<Route>(document: &mut UtoipaOpenApiRefMut<'_>)
where
    Route: TypedRoute,
{
    let raw_components = document
        .0
        .components
        .get_or_insert_with(utoipa::openapi::schema::Components::new);
    let mut schema_components = UtoipaOpenApiComponentsRefMut::from(raw_components);
    Route::register_openapi_schemas(&mut schema_components);
    register_openapi_schema::<crate::ApiProblem>(&mut schema_components);
    register_openapi_schema::<crate::ApiProblemDetail>(&mut schema_components);
    register_openapi_schema::<crate::ApiProblemField>(&mut schema_components);
    register_openapi_schema::<crate::ApiProblemKind>(&mut schema_components);
    register_openapi_schema::<crate::ApiProblemRequestId>(&mut schema_components);
    register_openapi_schema::<crate::ApiProblemStatus>(&mut schema_components);
    register_openapi_schema::<crate::ApiProblemViolation>(&mut schema_components);
    register_openapi_schema::<crate::FilterOperation>(&mut schema_components);
    register_openapi_schema::<crate::FilterValueShape>(&mut schema_components);
}
pub fn apply_openapi_path_parameter_contract<Route>(
    operation: &mut utoipa::openapi::path::Operation,
) where
    Route: TypedRoute,
{
    if let Some(parameter) = Route::openapi_path_parameter() {
        operation
            .parameters
            .get_or_insert_default()
            .push(parameter.into());
    }
}
pub fn apply_openapi_security_contract<Route>(
    operation: &mut utoipa::openapi::path::Operation,
    authenticated_scheme: OpenApiSecuritySchemeRef<'_>,
    csrf_scheme: OpenApiSecuritySchemeRef<'_>,
) where
    Route: TypedRoute,
{
    let metadata = Route::metadata();
    operation.security = match metadata.authentication() {
        crate::AuthenticationRequirement::Public => None,
        crate::AuthenticationRequirement::Authenticated
        | crate::AuthenticationRequirement::Permission(_) => {
            let requirement = utoipa::openapi::security::SecurityRequirement::new(
                authenticated_scheme.0,
                std::iter::empty::<&str>(),
            );
            let complete_requirement = if metadata.mutation() == crate::RouteMutation::Mutating {
                requirement.add(csrf_scheme.0, std::iter::empty::<&str>())
            } else {
                requirement
            };
            Some(vec![complete_requirement])
        }
    };
}
pub fn apply_openapi_error_contract<Route>(operation: &mut utoipa::openapi::path::Operation)
where
    Route: TypedRoute,
{
    operation
        .responses
        .responses
        .retain(|status, _response| !status.starts_with('4') && !status.starts_with('5'));
    Route::metadata()
        .error_statuses()
        .iter()
        .copied()
        .for_each(|error_status| {
            let status = error_status.transport_status().to_string();
            let mut response = utoipa::openapi::response::Response::new(status.clone());
            if let Some(schema) = Route::openapi_error_response_schema(error_status) {
                let _previous_content =
                    response.content.insert(
                        str_constants::APPLICATION_JSON.to_owned(),
                        utoipa::openapi::Content::new::<
                            utoipa::openapi::RefOr<utoipa::openapi::Schema>,
                        >(schema.into()),
                    );
            }
            if error_status == crate::RouteErrorStatus::RateLimited {
                let _previous_header = response.headers.insert(
                    str_constants::RETRY_AFTER.to_owned(),
                    utoipa::openapi::header::Header::default(),
                );
            }
            let _previous_response = operation
                .responses
                .responses
                .insert(status, utoipa::openapi::RefOr::T(response));
        });
}
#[must_use]
pub fn typed_route_path<Route>() -> crate::ContractStr
where
    Route: TypedRoute,
{
    Route::metadata().path()
}
#[must_use]
pub fn typed_parameterized_route_path<Route>(parameter: &Route::Parameter) -> ParameterizedRoutePath
where
    Route: ParameterizedRoute,
{
    Route::path(parameter)
}
#[cfg(test)]
mod tests {
    #[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
    #[serde(from = "u64")]
    #[derive(newtype::FromInner)]
    struct Request(u64);

    #[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
    #[serde(from = "u64")]
    #[derive(newtype::FromInner)]
    struct Response(u64);

    struct Route;
    impl super::TypedRoute for Route {
        type Request = Request;
        type Response = Response;
        type Transport = super::PublicTransport;
        fn metadata() -> super::RouteMetadata {
            super::RouteMetadata::new(
                super::RouteMethod::Get,
                crate::ContractStr::from(str_constants::ROUTE_READ),
                crate::ContractStr::from(str_constants::ROUTE),
            )
        }
    }
    #[test]
    fn matching_request_response_and_metadata_share_one_route_contract() {
        let request = super::client_request::<Route>(Request::from(1u64));
        let response = super::server_response::<Route>(Response::from(2u64));
        assert_eq!(request.body(), &Request(1u64));
        assert_eq!(response.body(), &Response(2u64));
        assert_eq!(
            <Route as super::TypedRoute>::metadata().path().as_ref(),
            "/route"
        );
        assert_eq!(
            super::client_route_metadata::<Route>(),
            super::server_route_metadata::<Route>()
        );
        assert_eq!(
            super::server_route_metadata::<Route>(),
            super::openapi_route_metadata::<Route>()
        );
    }
}
