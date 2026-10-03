#[test]
fn test_open_api_wrappers_keep_debug_labels_and_document_metadata() {
    let document = utoipa::openapi::OpenApi::new(
        utoipa::openapi::Info::new(
            constants_str::NEVER_PRINT_THIS_VALUE,
            constants_str::VALUE_1,
        ),
        utoipa::openapi::Paths::new(),
    );
    let authentication =
        crate::utoipa_admin_auth_open_api::UtoipaAdminAuthOpenApi::from(document.clone());
    assert_eq!(
        format!("{authentication:?}"),
        constants_str::UTOIPAADMINAUTHOPENAPI
    );
    assert_eq!(
        authentication.get_inner().info.title,
        constants_str::NEVER_PRINT_THIS_VALUE
    );
    let authentication_document = utoipa::openapi::OpenApi::from(authentication.clone());
    assert_eq!(
        authentication.get_inner().info.title,
        constants_str::NEVER_PRINT_THIS_VALUE
    );
    assert_eq!(
        authentication_document.info.title,
        constants_str::NEVER_PRINT_THIS_VALUE
    );
    assert_eq!(authentication_document.info.version, constants_str::VALUE_1);
    assert!(authentication_document.paths.paths.is_empty());
    let administrator = crate::utoipa_admin_open_api::UtoipaAdminOpenApi::from(document);
    assert_eq!(
        format!("{administrator:?}"),
        constants_str::UTOIPAADMINOPENAPI
    );
    assert_eq!(
        administrator.get_inner().info.title,
        constants_str::NEVER_PRINT_THIS_VALUE
    );
    let administrator_document = utoipa::openapi::OpenApi::from(administrator.clone());
    assert_eq!(
        administrator.get_inner().info.title,
        constants_str::NEVER_PRINT_THIS_VALUE
    );
    assert_eq!(
        administrator_document.info.title,
        constants_str::NEVER_PRINT_THIS_VALUE
    );
    assert_eq!(administrator_document.info.version, constants_str::VALUE_1);
    assert!(administrator_document.paths.paths.is_empty());
}
