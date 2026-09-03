#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    proc_macro_frontend_contract_derive_typed_route::TypedRoute,
)]
#[typed_route(
    authentication = frontend_contract::authentication_requirement::AuthenticationRequirement::Public,
    error_statuses = &[],
    method = frontend_contract::route_method::RouteMethod::Get,
    mutation = frontend_contract::route_mutation::RouteMutation::ReadOnly,
    obligations = frontend_contract::route_coverage_obligation::PUBLIC_READ_ROUTE_COVERAGE_OBLIGATIONS,
    openapi_operation_id = "git_info",
    path = "/git_info",
    request = crate::common_no_body::CommonNoBody,
    response = crate::git_info::GitInfo,
    success_status = frontend_contract::success_status::SuccessStatus::Code200,
    transport = frontend_contract::public_transport::PublicTransport
)]
pub struct GitInfoRoute;
