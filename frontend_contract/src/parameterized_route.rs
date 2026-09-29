pub trait ParameterizedRoute: crate::typed_route::TypedRoute {
    type Parameter;
    fn path(
        parameter: &Self::Parameter,
    ) -> Result<
        crate::parameterized_route_path::ParameterizedRoutePath,
        crate::parameterized_route_path_try_from_string_error::ParameterizedRoutePathTryFromStringError,
    >;
}
