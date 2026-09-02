#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    proc_macro_frontend_contract::TypedRoute,
)]
#[typed_route(
    authentication = frontend_contract::authentication_requirement::AuthenticationRequirement::Authenticated,
    method = frontend_contract::route_method::RouteMethod::Get,
    mutation = frontend_contract::route_mutation::RouteMutation::ReadOnly,
    obligations = frontend_contract::route_coverage_obligation::AUTHENTICATED_READ_ROUTE_COVERAGE_OBLIGATIONS,
    error_policy = frontend_contract::route_error_policy::RouteErrorPolicy::Default,
    openapi_operation_id = "me",
    path = "/auth/me",
    request = crate::admin_no_body::AdminNoBody,
    response = crate::authenticated_admin::AuthenticatedAdmin,
    success_status = frontend_contract::success_status::SuccessStatus::Code200,
    transport = frontend_contract::authenticated_transport::AuthenticatedTransport,
)]
pub struct AdminMeRoute;
