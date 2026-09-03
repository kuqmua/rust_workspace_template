#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout)]
struct ParameterizedTestRoute;

impl frontend_contract::typed_route::TypedRoute for ParameterizedTestRoute {
    type Request = String;
    type Response = String;
    type Transport = frontend_contract::public_transport::PublicTransport;

    fn metadata() -> frontend_contract::route_metadata::RouteMetadata {
        frontend_contract::route_metadata::RouteMetadata::new(
            std::convert::identity(frontend_contract::route_method::RouteMethod::Get),
            frontend_contract::contract_str::ContractStr::from(constants_str::READ),
            frontend_contract::contract_str::ContractStr::from(constants_str::ROUTE),
        )
    }
}

impl frontend_contract::parameterized_route::ParameterizedRoute for ParameterizedTestRoute {
    type Parameter = u64;

    fn path(parameter: &Self::Parameter) -> frontend_contract::parameterized_route_path::ParameterizedRoutePath {
        let _value = parameter;
        frontend_contract::parameterized_route_path::ParameterizedRoutePath::default()
    }
}

fn main() {
    let wrong_parameter = String::new();
    let _path = frontend_contract::typed_parameterized_route_path::typed_parameterized_route_path::<ParameterizedTestRoute>(
        &wrong_parameter,
    );
}
