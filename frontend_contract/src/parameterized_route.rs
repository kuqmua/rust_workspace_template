pub trait ParameterizedRoute: crate::typed_route::TypedRoute {
    type Parameter;
    fn path(parameter: &Self::Parameter)
    -> crate::parameterized_route_path::ParameterizedRoutePath;
}
