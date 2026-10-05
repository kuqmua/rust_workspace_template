#[test]
fn test_frontend_open_api_wrappers_hide_nested_diagnostics_and_preserve_data() {
    let parameter = utoipa::openapi::path::ParameterBuilder::new()
        .name(constants_str::NEVER_PRINT_THIS_VALUE)
        .build();
    let path_parameter =
        crate::utoipa_open_api_path_parameter::UtoipaOpenApiPathParameter::from(parameter);
    assert_eq!(
        format!("{path_parameter:?}"),
        format!("{} {{ .. }}", constants_str::UTOIPAOPENAPIPATHPARAMETER)
    );
    let cloned_parameter = utoipa::openapi::path::Parameter::from(path_parameter.clone());
    let moved_parameter = utoipa::openapi::path::Parameter::from(path_parameter);
    assert_eq!(cloned_parameter.name, constants_str::NEVER_PRINT_THIS_VALUE);
    assert_eq!(moved_parameter.name, constants_str::NEVER_PRINT_THIS_VALUE);
    let schema = utoipa::openapi::RefOr::<utoipa::openapi::Schema>::Ref(
        utoipa::openapi::Ref::from_schema_name(constants_str::NEVER_PRINT_THIS_VALUE),
    );
    let expected_schema = serde_json::to_value(&schema);
    let route_schema = crate::utoipa_open_api_route_schema::UtoipaOpenApiRouteSchema::from(schema);
    assert_eq!(
        format!("{route_schema:?}"),
        format!("{} {{ .. }}", constants_str::OPEN_API_ROUTE_SCHEMA)
    );
    let cloned_schema =
        utoipa::openapi::RefOr::<utoipa::openapi::Schema>::from(route_schema.clone());
    let moved_schema = utoipa::openapi::RefOr::<utoipa::openapi::Schema>::from(route_schema);
    assert!(expected_schema.is_ok_and(|expected| {
        serde_json::to_value(&cloned_schema).is_ok_and(|actual| actual == expected)
            && serde_json::to_value(&moved_schema).is_ok_and(|actual| actual == expected)
    }));
    let mut components = utoipa::openapi::schema::Components::new();
    let _previous_schema = components.schemas.insert(
        constants_str::NEVER_PRINT_THIS_VALUE.to_owned(),
        moved_schema,
    );
    assert!({
        let mut components_ref_mut =
            crate::utoipa_open_api_components_ref_mut::UtoipaOpenApiComponentsRefMut::from(
                &mut components,
            );
        assert_eq!(
            format!("{components_ref_mut:?}"),
            format!("{} {{ .. }}", constants_str::UTOIPAOPENAPICOMPONENTSREFMUT)
        );
        assert!(
            components_ref_mut
                .schemas
                .contains_key(constants_str::NEVER_PRINT_THIS_VALUE)
        );
        components_ref_mut.schemas.clear();
        components_ref_mut.schemas.is_empty()
    });
    assert!(components.schemas.is_empty());
    let mut document = utoipa::openapi::OpenApi::new(
        utoipa::openapi::Info::new(
            constants_str::NEVER_PRINT_THIS_VALUE,
            constants_str::VALUE_1,
        ),
        utoipa::openapi::Paths::new(),
    );
    assert!({
        let mut document_ref_mut =
            crate::utoipa_open_api_ref_mut::UtoipaOpenApiRefMut::from(&mut document);
        assert_eq!(
            format!("{document_ref_mut:?}"),
            format!("{} {{ .. }}", constants_str::UTOIPAOPENAPIREFMUT)
        );
        document_ref_mut.as_mut().info.version = constants_str::VALUE_2.to_owned();
        document_ref_mut.as_mut().info.version == constants_str::VALUE_2
    });
    assert_eq!(document.info.title, constants_str::NEVER_PRINT_THIS_VALUE);
    assert_eq!(document.info.version, constants_str::VALUE_2);
}
