struct ParameterizedTestRoute;

impl frontend_contract::TypedRoute for ParameterizedTestRoute {
    type Request = String;
    type Response = String;
    type Transport = frontend_contract::PublicTransport;

    fn metadata() -> frontend_contract::RouteMetadata {
        frontend_contract::RouteMetadata::new(
            frontend_contract::RouteMethod::Get,
            frontend_contract::ContractStr::from(str_constants::READ),
            frontend_contract::ContractStr::from(str_constants::ROUTE),
        )
    }
}

impl frontend_contract::ParameterizedRoute for ParameterizedTestRoute {
    type Parameter = u64;

    fn path(parameter: &Self::Parameter) -> String {
        parameter.to_string()
    }
}

fn main() {
    let wrong_parameter = String::new();
    let _path = frontend_contract::typed_parameterized_route_path::<ParameterizedTestRoute>(
        &wrong_parameter,
    );
}
