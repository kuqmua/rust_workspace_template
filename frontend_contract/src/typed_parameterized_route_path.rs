pub fn typed_parameterized_route_path<Route>(
    parameter: &Route::Parameter,
) -> Result<
    crate::parameterized_route_path::ParameterizedRoutePath,
    crate::parameterized_route_path_try_from_string_error::ParameterizedRoutePathTryFromStringError,
>
where
    Route: crate::parameterized_route::ParameterizedRoute,
{
    Route::path(parameter)
}
