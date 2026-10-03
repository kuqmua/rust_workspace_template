pub fn validate_production_manifest(
    string_file_content_ref: crate::string_file_content_ref::StringFileContentRef<'_>,
) -> Result<(), crate::production_manifest_error::ProductionManifestError> {
    let (placeholder, proxy, images, mode, cookie, deployment, network, budget) =
        string_file_content_ref.as_ref().split('\n').fold(
            (false, false, 0usize, false, false, false, false, false),
            |(placeholder, proxy, images, mode, cookie, deployment, network, budget), line| {
                let replacement = line
                    .match_indices(constants_str::PRODUCTION_MANIFEST_REPLACE)
                    .any(|(index, matched)| {
                        line.get(index.saturating_add(matched.len())..)
                            .and_then(|suffix| suffix.bytes().next())
                            .is_some_and(|byte| byte.is_ascii_lowercase() || byte == b'-')
                    });
                let image = line
                    .trim_start_matches(|character| {
                        matches!(character, ' ' | '\t' | '\r' | '\u{000b}' | '\u{000c}')
                    })
                    .strip_prefix(constants_str::PRODUCTION_MANIFEST_IMAGE)
                    .and_then(|image| image.rsplit_once(constants_str::PRODUCTION_MANIFEST_DIGEST))
                    .is_some_and(|(name, digest)| {
                        !name.is_empty()
                            && digest.len() == 64usize
                            && digest
                                .bytes()
                                .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
                    });
                (
                    placeholder
                        || line.contains(constants_str::PRODUCTION_MANIFEST_EXAMPLE)
                        || replacement,
                    proxy || line.contains(constants_str::PRODUCTION_MANIFEST_PROXY),
                    images.saturating_add(usize::from(image)),
                    mode || line.contains(constants_str::PRODUCTION_MANIFEST_MODE),
                    cookie || line.contains(constants_str::PRODUCTION_MANIFEST_COOKIE),
                    deployment || line.contains(constants_str::PRODUCTION_MANIFEST_DEPLOYMENT),
                    network || line.contains(constants_str::PRODUCTION_MANIFEST_NETWORK),
                    budget || line.contains(constants_str::PRODUCTION_MANIFEST_BUDGET),
                )
            },
        );
    if placeholder {
        return Err(crate::production_manifest_error::ProductionManifestError::Placeholder);
    }
    if proxy {
        return Err(crate::production_manifest_error::ProductionManifestError::TrustedProxy);
    }
    if images < 2usize {
        return Err(crate::production_manifest_error::ProductionManifestError::Images);
    }
    if !mode {
        return Err(crate::production_manifest_error::ProductionManifestError::ProductionMode);
    }
    if !cookie {
        return Err(crate::production_manifest_error::ProductionManifestError::SecureCookie);
    }
    if !deployment {
        return Err(crate::production_manifest_error::ProductionManifestError::Deployment);
    }
    if !network {
        return Err(crate::production_manifest_error::ProductionManifestError::NetworkPolicy);
    }
    if !budget {
        return Err(crate::production_manifest_error::ProductionManifestError::DisruptionBudget);
    }
    Ok(())
}
