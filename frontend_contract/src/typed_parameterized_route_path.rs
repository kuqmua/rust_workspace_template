#[must_use]
pub fn typed_parameterized_route_path<Route>(
    parameter: &Route::Parameter,
) -> crate::parameterized_route_path::ParameterizedRoutePath
where
    Route: crate::parameterized_route::ParameterizedRoute,
{
    Route::path(parameter)
}
