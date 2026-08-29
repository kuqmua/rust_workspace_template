#[must_use]
pub fn admin_parameterized_route_path<Route>(
    parameter: &Route::Parameter,
) -> crate::admin_route_path::AdminRoutePath
where
    Route: frontend_contract::parameterized_route::ParameterizedRoute,
{
    crate::admin_api_route_path::admin_api_route_path(
        frontend_contract::typed_parameterized_route_path::typed_parameterized_route_path::<Route>(
            parameter,
        ),
    )
}
