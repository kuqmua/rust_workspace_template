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
    register_openapi_schema::<crate::ApiProblem>(&mut schema_components);
    register_openapi_schema::<crate::ApiProblemDetail>(&mut schema_components);
    register_openapi_schema::<crate::ApiProblemField>(&mut schema_components);
    register_openapi_schema::<crate::ApiProblemKind>(&mut schema_components);
    register_openapi_schema::<crate::ApiProblemRequestId>(&mut schema_components);
    register_openapi_schema::<crate::ApiProblemStatus>(&mut schema_components);
    register_openapi_schema::<crate::ApiProblemViolation>(&mut schema_components);
    register_openapi_schema::<crate::FilterOperation>(&mut schema_components);
    register_openapi_schema::<crate::FilterValueShape>(&mut schema_components);
}
