#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, frontend_contract::TypedRoute,
)]
#[typed_route(
    authentication = frontend_contract::AuthenticationRequirement::Public,
    method = frontend_contract::RouteMethod::Post,
    mutation = frontend_contract::RouteMutation::Mutating,
    obligations = frontend_contract::PUBLIC_MUTATING_ROUTE_COVERAGE_OBLIGATIONS,
    error_policy = frontend_contract::RouteErrorPolicy::Authentication,
    openapi_operation_id = "sign_in",
    path = "/auth/sign_in",
    request = crate::domain_types::AdminSignInReq,
    request_body = frontend_contract::RouteRequestBody::Json,
    response = crate::domain_types::AdminSignInRes,
    success_status = frontend_contract::SuccessStatus::Code200,
    transport = frontend_contract::PublicTransport,
)]
pub struct AdminSignInRoute;
