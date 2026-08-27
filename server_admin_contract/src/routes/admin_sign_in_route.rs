#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    frontend_contract::domain_types::TypedRoute,
)]
#[typed_route(
    authentication = frontend_contract::domain_types::AuthenticationRequirement::Public,
    method = frontend_contract::domain_types::RouteMethod::Post,
    mutation = frontend_contract::domain_types::RouteMutation::Mutating,
    obligations = frontend_contract::domain_types::PUBLIC_MUTATING_ROUTE_COVERAGE_OBLIGATIONS,
    error_policy = frontend_contract::domain_types::RouteErrorPolicy::Authentication,
    openapi_operation_id = "sign_in",
    path = "/auth/sign_in",
    request = crate::domain_types::AdminSignInReq,
    request_body = frontend_contract::domain_types::RouteRequestBody::Json,
    response = crate::domain_types::AdminSignInRes,
    success_status = frontend_contract::domain_types::SuccessStatus::Code200,
    transport = frontend_contract::domain_types::PublicTransport,
)]
pub struct AdminSignInRoute;
