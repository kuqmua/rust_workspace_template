pub(crate) fn admin_api_route_path(
    parameterized_route_path: frontend_contract::parameterized_route_path::ParameterizedRoutePath,
) -> Result<
    crate::admin_route_path::AdminRoutePath,
    crate::admin_route_path_error::AdminRoutePathError,
> {
    crate::admin_route_path::AdminRoutePath::try_from(String::from(parameterized_route_path))
}
