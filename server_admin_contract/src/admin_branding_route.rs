#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    frontend_contract_macros::TypedRoute,
)]
#[typed_route(error_policy = frontend_contract::route_error_policy::RouteErrorPolicy::Default, authentication = frontend_contract::authentication_requirement::AuthenticationRequirement::Public, method = frontend_contract::route_method::RouteMethod::Get, mutation = frontend_contract::route_mutation::RouteMutation::ReadOnly, obligations = frontend_contract::route_coverage_obligation::PUBLIC_READ_ROUTE_COVERAGE_OBLIGATIONS, openapi_operation_id = "branding", path = "/branding", request = crate::admin_no_body::AdminNoBody, response = crate::admin_branding_view::AdminBrandingView, success_status = frontend_contract::success_status::SuccessStatus::Code200, transport = frontend_contract::public_transport::PublicTransport)]
pub struct AdminBrandingRoute;
