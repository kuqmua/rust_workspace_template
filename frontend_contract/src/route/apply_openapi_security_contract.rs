use super::{OpenApiSecuritySchemeRef, TypedRoute};

pub fn apply_openapi_security_contract<Route>(
    operation: &mut utoipa::openapi::path::Operation,
    authenticated_scheme: OpenApiSecuritySchemeRef<'_>,
    csrf_scheme: OpenApiSecuritySchemeRef<'_>,
) where
    Route: TypedRoute,
{
    let metadata = Route::metadata();
    operation.security = match metadata.authentication() {
        crate::domain_types::AuthenticationRequirement::Public => None,
        crate::domain_types::AuthenticationRequirement::Authenticated
        | crate::domain_types::AuthenticationRequirement::Permission(_) => {
            let requirement = utoipa::openapi::security::SecurityRequirement::new(
                authenticated_scheme.0,
                std::iter::empty::<&str>(),
            );
            let complete_requirement =
                if metadata.mutation() == crate::domain_types::RouteMutation::Mutating {
                    requirement.add(csrf_scheme.0, std::iter::empty::<&str>())
                } else {
                    requirement
                };
            Some(vec![complete_requirement])
        }
    };
}
