pub fn admin_parameterized_route_path<Route>(
    parameter: &Route::Parameter,
) -> Result<
    crate::admin_route_path::AdminRoutePath,
    crate::admin_route_path_error::AdminRoutePathError,
>
where
    Route: frontend_contract::parameterized_route::ParameterizedRoute,
{
    frontend_contract::typed_parameterized_route_path::typed_parameterized_route_path::<Route>(
        parameter,
    )
    .map_err(crate::admin_route_path_error::AdminRoutePathError::from)
    .and_then(crate::admin_api_route_path::admin_api_route_path)
}
