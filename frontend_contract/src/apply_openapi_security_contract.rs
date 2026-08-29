pub fn apply_openapi_security_contract<Route>(
    operation: &mut utoipa::openapi::path::Operation,
    authenticated_scheme: crate::open_api_security_scheme_ref::OpenApiSecuritySchemeRef<'_>,
    csrf_scheme: crate::open_api_security_scheme_ref::OpenApiSecuritySchemeRef<'_>,
) where
    Route: crate::typed_route::TypedRoute,
{
    let metadata = Route::metadata();
    operation.security = match metadata.authentication() {
        crate::authentication_requirement::AuthenticationRequirement::Public => None,
        crate::authentication_requirement::AuthenticationRequirement::Authenticated
        | crate::authentication_requirement::AuthenticationRequirement::Permission(_) => {
            let requirement = utoipa::openapi::security::SecurityRequirement::new(
                authenticated_scheme.0,
                std::iter::empty::<&str>(),
            );
            let complete_requirement =
                if metadata.mutation() == crate::route_mutation::RouteMutation::Mutating {
                    requirement.add(csrf_scheme.0, std::iter::empty::<&str>())
                } else {
                    requirement
                };
            Some(vec![complete_requirement])
        }
    };
}
