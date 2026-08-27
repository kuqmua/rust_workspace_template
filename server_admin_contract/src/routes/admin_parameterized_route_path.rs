use super::{AdminRoutePath, admin_api_route_path};

#[must_use]
pub fn admin_parameterized_route_path<Route>(parameter: &Route::Parameter) -> AdminRoutePath
where
    Route: frontend_contract::domain_types::ParameterizedRoute,
{
    admin_api_route_path(
        frontend_contract::domain_types::typed_parameterized_route_path::<Route>(parameter),
    )
}
