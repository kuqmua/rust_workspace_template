#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    proc_macro_frontend_contract::TypedRoute,
)]
#[typed_route(
    authentication = frontend_contract::authentication_requirement::AuthenticationRequirement::Public,
    method = frontend_contract::route_method::RouteMethod::Post,
    mutation = frontend_contract::route_mutation::RouteMutation::Mutating,
    obligations = frontend_contract::route_coverage_obligation::PUBLIC_MUTATING_ROUTE_COVERAGE_OBLIGATIONS,
    error_policy = frontend_contract::route_error_policy::RouteErrorPolicy::Authentication,
    openapi_operation_id = "sign_in",
    path = "/auth/sign_in",
    request = crate::admin_sign_in_req::AdminSignInReq,
    request_body = frontend_contract::route_request_body::RouteRequestBody::Json,
    response = crate::admin_sign_in_res::AdminSignInRes,
    success_status = frontend_contract::success_status::SuccessStatus::Code200,
    transport = frontend_contract::public_transport::PublicTransport,
)]
pub struct AdminSignInRoute;
