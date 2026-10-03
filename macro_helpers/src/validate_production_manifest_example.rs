pub fn validate_production_manifest_example(
    string_file_content_ref: crate::string_file_content_ref::StringFileContentRef<'_>,
) -> Result<(), crate::production_manifest_error::ProductionManifestError> {
    if crate::validate_production_manifest::validate_production_manifest(string_file_content_ref)
        .is_ok()
    {
        return Err(crate::production_manifest_error::ProductionManifestError::ExampleAccepted);
    }
    let candidate = string_file_content_ref
        .as_ref()
        .replace(
            constants_str::MIGRATION_A02B427B,
            constants_str::MIGRATION_712F2F0D,
        )
        .replace(
            constants_str::MIGRATION_26404E17,
            constants_str::MIGRATION_1234E574,
        )
        .replace(
            constants_str::MIGRATION_05027AB9,
            constants_str::MIGRATION_60E05BD1,
        )
        .replace(
            constants_str::MIGRATION_3937F190,
            constants_str::MIGRATION_1976556C,
        )
        .replace(
            constants_str::PRODUCTION_MANIFEST_PROXY,
            constants_str::MIGRATION_D1940771,
        );
    crate::validate_production_manifest::validate_production_manifest(
        crate::string_file_content_ref::StringFileContentRef::from(candidate.as_str()),
    )
}
