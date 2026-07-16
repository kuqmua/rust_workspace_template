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
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Connect => str_constants::CONNECT,
            Self::Delete => str_constants::DELETE,
            Self::Get => str_constants::GET,
            Self::Head => str_constants::HEAD,
            Self::Options => str_constants::OPTIONS,
            Self::Patch => str_constants::PATCH,
            Self::Post => str_constants::POST,
            Self::Put => str_constants::PUT,
            Self::Trace => str_constants::TRACE,
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RouteMetadata {
    method: RouteMethod,
    openapi_operation_id: crate::ContractStr,
    path: crate::ContractStr,
}
impl RouteMetadata {
    #[must_use]
    pub const fn new(
        method: RouteMethod,
        openapi_operation_id: crate::ContractStr,
        path: crate::ContractStr,
    ) -> Self {
        Self {
            method,
            openapi_operation_id,
            path,
        }
    }
    #[must_use]
    pub fn method(self) -> crate::ContractStr {
        crate::ContractStr::from(self.method.as_str())
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
}
pub trait TypedRoute: Sized {
    type Request: serde::Serialize + serde::de::DeserializeOwned;
    type Response: serde::Serialize + serde::de::DeserializeOwned;
    type Transport: RouteTransport;
    fn metadata() -> RouteMetadata;
}
pub trait CoveredRoute: TypedRoute {
    fn coverage_descriptor() -> crate::RouteCoverageDescriptor;
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RouteBodyLimit(usize);
impl RouteBodyLimit {
    #[must_use]
    pub const fn get(self) -> usize {
        self.0
    }
}
impl From<usize> for RouteBodyLimit {
    fn from(value: usize) -> Self {
        Self(value)
    }
}
pub trait RouteFamily {
    #[must_use]
    fn body_limit() -> Option<RouteBodyLimit> {
        None
    }
    fn coverage_descriptors() -> Vec<crate::RouteCoverageDescriptor>;
    fn route_metadata() -> Vec<RouteMetadata> {
        Self::coverage_descriptors()
            .into_iter()
            .map(crate::RouteCoverageDescriptor::metadata)
            .collect()
    }
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
#[must_use]
pub fn typed_route_path<Route>() -> crate::ContractStr
where
    Route: TypedRoute,
{
    Route::metadata().path()
}
#[cfg(test)]
mod tests {
    #[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
    struct Request(u64);
    #[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
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
        let request = super::client_request::<Route>(Request(1u64));
        let response = super::server_response::<Route>(Response(2u64));
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
