#[test]
fn test_production_manifest_preserves_shell_validation_rules() {
    let validate = crate::validate_production_manifest::validate_production_manifest;
    assert_eq!(
        validate(crate::string_file_content_ref::StringFileContentRef::from(
            constants_str::PRODUCTION_MANIFEST_VALID_TEST
        )),
        Ok(())
    );
    [
        (
            constants_str::PRODUCTION_MANIFEST_MODE,
            crate::production_manifest_error::ProductionManifestError::ProductionMode,
        ),
        (
            constants_str::PRODUCTION_MANIFEST_COOKIE,
            crate::production_manifest_error::ProductionManifestError::SecureCookie,
        ),
        (
            constants_str::PRODUCTION_MANIFEST_DEPLOYMENT,
            crate::production_manifest_error::ProductionManifestError::Deployment,
        ),
        (
            constants_str::PRODUCTION_MANIFEST_NETWORK,
            crate::production_manifest_error::ProductionManifestError::NetworkPolicy,
        ),
        (
            constants_str::PRODUCTION_MANIFEST_BUDGET,
            crate::production_manifest_error::ProductionManifestError::DisruptionBudget,
        ),
    ]
    .into_iter()
    .fold((), |(), (required, expected)| {
        let missing =
            constants_str::PRODUCTION_MANIFEST_VALID_TEST.replace(required, constants_str::EMPTY);
        assert_eq!(
            validate(crate::string_file_content_ref::StringFileContentRef::from(
                missing.as_str()
            )),
            Err(expected)
        );
    });
    [
        (
            constants_str::PRODUCTION_MANIFEST_EXAMPLE,
            crate::production_manifest_error::ProductionManifestError::Placeholder,
        ),
        (
            constants_str::MIGRATION_05027AB9,
            crate::production_manifest_error::ProductionManifestError::Placeholder,
        ),
        (
            constants_str::MIGRATION_82A96816,
            crate::production_manifest_error::ProductionManifestError::Placeholder,
        ),
        (
            constants_str::PRODUCTION_MANIFEST_PROXY,
            crate::production_manifest_error::ProductionManifestError::TrustedProxy,
        ),
    ]
    .into_iter()
    .fold((), |(), (extra, expected)| {
        let invalid = format!("{}{}", constants_str::PRODUCTION_MANIFEST_VALID_TEST, extra);
        assert_eq!(
            validate(crate::string_file_content_ref::StringFileContentRef::from(
                invalid.as_str()
            )),
            Err(expected)
        );
    });
    [
        constants_str::MIGRATION_C7DC2D25,
        constants_str::MIGRATION_E531EF0F,
        constants_str::MIGRATION_D53EDA7A,
        constants_str::MIGRATION_F704D566,
        constants_str::MIGRATION_B73585ED,
    ]
    .into_iter()
    .fold((), |(), digest| {
        let invalid = constants_str::PRODUCTION_MANIFEST_VALID_TEST
            .replace(constants_str::MIGRATION_60E05BD1, digest);
        assert_eq!(
            validate(crate::string_file_content_ref::StringFileContentRef::from(
                invalid.as_str()
            )),
            Err(crate::production_manifest_error::ProductionManifestError::Images)
        );
    });
    [
        constants_str::PRODUCTION_MANIFEST_REPLACE,
        constants_str::MIGRATION_C2EAE5A6,
    ]
    .into_iter()
    .fold((), |(), extra| {
        let accepted = format!("{}{}", constants_str::PRODUCTION_MANIFEST_VALID_TEST, extra);
        assert_eq!(
            validate(crate::string_file_content_ref::StringFileContentRef::from(
                accepted.as_str()
            )),
            Ok(())
        );
    });
}

#[test]
fn test_production_manifest_example_rejects_example_and_accepts_exact_ci_substitutions() {
    assert_eq!(
        crate::validate_production_manifest_example::validate_production_manifest_example(
            crate::string_file_content_ref::StringFileContentRef::from(
                constants_str::PRODUCTION_MANIFEST_EXAMPLE_TEST
            ),
        ),
        Ok(())
    );
    assert_eq!(
        crate::validate_production_manifest_example::validate_production_manifest_example(
            crate::string_file_content_ref::StringFileContentRef::from(
                constants_str::PRODUCTION_MANIFEST_VALID_TEST
            ),
        ),
        Err(crate::production_manifest_error::ProductionManifestError::ExampleAccepted)
    );
    assert_eq!(
        crate::validate_production_manifest_example::validate_production_manifest_example(
            crate::string_file_content_ref::StringFileContentRef::from(constants_str::EMPTY),
        ),
        Err(crate::production_manifest_error::ProductionManifestError::Images)
    );
}

#[test]
fn test_production_manifest_preserves_error_precedence_with_multiple_failures() {
    [
        (
            constants_str::EMPTY.to_owned(),
            crate::production_manifest_error::ProductionManifestError::Images,
        ),
        (
            constants_str::PRODUCTION_MANIFEST_EXAMPLE.to_owned(),
            crate::production_manifest_error::ProductionManifestError::Placeholder,
        ),
        (
            constants_str::PRODUCTION_MANIFEST_PROXY.to_owned(),
            crate::production_manifest_error::ProductionManifestError::TrustedProxy,
        ),
        (
            [
                constants_str::PRODUCTION_MANIFEST_PROXY,
                constants_str::PRODUCTION_MANIFEST_EXAMPLE,
            ]
            .concat(),
            crate::production_manifest_error::ProductionManifestError::Placeholder,
        ),
    ]
    .into_iter()
    .fold((), |(), (manifest, expected)| {
        assert_eq!(
            crate::validate_production_manifest::validate_production_manifest(
                crate::string_file_content_ref::StringFileContentRef::from(manifest.as_str()),
            ),
            Err(expected)
        );
    });
    let requirements = [
        (
            constants_str::PRODUCTION_MANIFEST_MODE,
            crate::production_manifest_error::ProductionManifestError::ProductionMode,
        ),
        (
            constants_str::PRODUCTION_MANIFEST_COOKIE,
            crate::production_manifest_error::ProductionManifestError::SecureCookie,
        ),
        (
            constants_str::PRODUCTION_MANIFEST_DEPLOYMENT,
            crate::production_manifest_error::ProductionManifestError::Deployment,
        ),
        (
            constants_str::PRODUCTION_MANIFEST_NETWORK,
            crate::production_manifest_error::ProductionManifestError::NetworkPolicy,
        ),
        (
            constants_str::PRODUCTION_MANIFEST_BUDGET,
            crate::production_manifest_error::ProductionManifestError::DisruptionBudget,
        ),
    ];
    requirements
        .iter()
        .enumerate()
        .fold((), |(), (index, (_, expected))| {
            let manifest = requirements.iter().skip(index).fold(
                constants_str::PRODUCTION_MANIFEST_VALID_TEST.to_owned(),
                |remaining, (required, _)| remaining.replace(required, constants_str::EMPTY),
            );
            assert_eq!(
                crate::validate_production_manifest::validate_production_manifest(
                    crate::string_file_content_ref::StringFileContentRef::from(manifest.as_str()),
                ),
                Err(*expected)
            );
        });
}
