#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    proc_macro_frontend_contract_derive_typed_route::TypedRoute,
)]
#[typed_route(
    authentication = frontend_contract::authentication_requirement::AuthenticationRequirement::Public,
    method = frontend_contract::route_method::RouteMethod::Post,
    mutation = frontend_contract::route_mutation::RouteMutation::Mutating,
    obligations = frontend_contract::route_coverage_obligation::PUBLIC_MUTATING_ROUTE_COVERAGE_OBLIGATIONS,
    error_policy = frontend_contract::route_error_policy::RouteErrorPolicy::Authentication,
    openapi_operation_id = "refresh",
    path = "/auth/refresh",
    request = crate::admin_no_body::AdminNoBody,
    response = crate::admin_sign_in_response::AdminSignInResponse,
    success_status = frontend_contract::success_status::SuccessStatus::Code200,
    transport = frontend_contract::public_transport::PublicTransport,
)]
pub struct AdminRefreshRoute;
