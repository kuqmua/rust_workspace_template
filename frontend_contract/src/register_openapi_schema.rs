#[allow(clippy::needless_for_each)] // iterator form follows the workspace no-for-loop policy
pub fn register_openapi_schema<Schema>(
    components: &mut crate::utoipa_open_api_components_ref_mut::UtoipaOpenApiComponentsRefMut<'_>,
) where
    Schema: utoipa::ToSchema,
{
    let name = Schema::name();
    let schema = <Schema as utoipa::PartialSchema>::schema();
    let qualified_name = std::any::type_name::<Schema>().replace(
        constants_str::test_fixtures::DOUBLE_COLON,
        constants_str::catalog::DOT,
    );
    let _previous_qualified_schema = components.0.schemas.insert(qualified_name, schema.clone());
    if let Some(crate_name) = std::any::type_name::<Schema>()
        .split(constants_str::test_fixtures::DOUBLE_COLON)
        .next()
    {
        let _previous_crate_schema = components
            .0
            .schemas
            .insert(format!("{crate_name}.{name}"), schema.clone());
    }
    let _previous_named_schema = components.0.schemas.insert(name.into_owned(), schema);
    let mut referenced_schemas = Vec::new();
    Schema::schemas(&mut referenced_schemas);
    referenced_schemas
        .into_iter()
        .for_each(|(referenced_name, referenced_schema)| {
            let _previous_schema = components
                .0
                .schemas
                .insert(referenced_name, referenced_schema);
        });
}
