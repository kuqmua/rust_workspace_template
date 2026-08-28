use super::{
    TypedRoute, UtoipaOpenApiComponentsRefMut, UtoipaOpenApiRefMut, register_openapi_schema,
};

pub fn register_openapi_route_schemas<Route>(document: &mut UtoipaOpenApiRefMut<'_>)
where
    Route: TypedRoute,
{
    let raw_components = document
        .0
        .components
        .get_or_insert_with(utoipa::openapi::schema::Components::new);
    let mut schema_components = UtoipaOpenApiComponentsRefMut::from(raw_components);
    Route::register_openapi_schemas(&mut schema_components);
    register_openapi_schema::<crate::domain_types::ApiProblem>(&mut schema_components);
    register_openapi_schema::<crate::domain_types::ApiProblemDetail>(&mut schema_components);
    register_openapi_schema::<crate::domain_types::ApiProblemField>(&mut schema_components);
    register_openapi_schema::<crate::domain_types::ApiProblemKind>(&mut schema_components);
    register_openapi_schema::<crate::domain_types::ApiProblemRequestId>(&mut schema_components);
    register_openapi_schema::<crate::domain_types::ApiProblemStatus>(&mut schema_components);
    register_openapi_schema::<crate::domain_types::ApiProblemViolation>(&mut schema_components);
    register_openapi_schema::<crate::domain_types::FilterOperation>(&mut schema_components);
    register_openapi_schema::<crate::domain_types::FilterValueShape>(&mut schema_components);
}
