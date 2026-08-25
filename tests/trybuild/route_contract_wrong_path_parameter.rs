#[derive(optimal_memory_layout::OptimalMemoryLayout)]
struct ParameterizedTestRoute;

impl frontend_contract::domain_types::TypedRoute for ParameterizedTestRoute {
    type Request = String;
    type Response = String;
    type Transport = frontend_contract::domain_types::PublicTransport;

    fn metadata() -> frontend_contract::domain_types::RouteMetadata {
        frontend_contract::domain_types::RouteMetadata::new(
            frontend_contract::domain_types::RouteMethod::Get,
            frontend_contract::domain_types::ContractStr::from(constants_str::READ),
            frontend_contract::domain_types::ContractStr::from(constants_str::ROUTE),
        )
    }
}

impl frontend_contract::domain_types::ParameterizedRoute for ParameterizedTestRoute {
    type Parameter = u64;

    fn path(parameter: &Self::Parameter) -> frontend_contract::domain_types::ParameterizedRoutePath {
        let _value = parameter;
        frontend_contract::domain_types::ParameterizedRoutePath::default()
    }
}

fn main() {
    let wrong_parameter = String::new();
    let _path = frontend_contract::domain_types::typed_parameterized_route_path::<ParameterizedTestRoute>(
        &wrong_parameter,
    );
}
