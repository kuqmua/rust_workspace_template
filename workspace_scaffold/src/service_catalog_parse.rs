#[allow(
    clippy::single_call_fn,
    reason = "deployment synchronization owns catalog parsing"
)]
pub(super) fn parse(
    source: super::ScaffoldTextRef<'_>,
) -> Result<super::ServiceCatalogEntries, super::ScaffoldError> {
    let mut entries = Vec::new();
    let mut current = None;
    source.0.lines().try_for_each(|raw_line| {
        let trimmed_line = raw_line.trim();
        if trimmed_line == constants_str::VALUE_484ADD83 {
            if let Some(draft) = current.take() {
                entries.push(super::ServiceCatalogDraft::finish(draft)?);
            }
            current = Some(super::ServiceCatalogDraft::default());
            return Ok(());
        }
        let Some(draft) = current.as_mut() else {
            return Ok(());
        };
        if let Some(value) = super::service_catalog_string_value::string_value(
            super::ScaffoldTextRef::from(trimmed_line),
            super::ScaffoldTextRef::from(constants_str::CRATE),
        )? {
            draft.crate_name = Some(
                super::ServiceCrate::try_from(value.as_ref().to_owned())
                    .map_err(|_error| super::ScaffoldError::Catalog)?,
            );
            return Ok(());
        }
        if let Some(value) = super::service_catalog_string_value::string_value(
            super::ScaffoldTextRef::from(trimmed_line),
            super::ScaffoldTextRef::from(constants_str::VALUE_DB669AF6),
        )? {
            draft.compose_name = Some(
                super::ServiceComposeName::try_from(value.as_ref().to_owned())
                    .map_err(|_error| super::ScaffoldError::Catalog)?,
            );
            return Ok(());
        }
        if let Some(value) = super::service_catalog_string_value::string_value(
            super::ScaffoldTextRef::from(trimmed_line),
            super::ScaffoldTextRef::from(constants_str::VALUE_739ED940),
        )? {
            draft.compose_file = Some(
                super::ServiceComposeFile::try_from(value.as_ref().to_owned())
                    .map_err(|_error| super::ScaffoldError::Catalog)?,
            );
            return Ok(());
        }
        if let Some(value) = super::service_catalog_string_value::string_value(
            super::ScaffoldTextRef::from(trimmed_line),
            super::ScaffoldTextRef::from(constants_str::VALUE_254DB0FB),
        )? {
            draft.dockerfile = Some(
                super::ServiceDockerfile::try_from(value.as_ref().to_owned())
                    .map_err(|_error| super::ScaffoldError::Catalog)?,
            );
            return Ok(());
        }
        if let Some(value) = super::service_catalog_string_value::string_value(
            super::ScaffoldTextRef::from(trimmed_line),
            super::ScaffoldTextRef::from(constants_str::VALUE_6105D6CC),
        )? {
            draft.image = Some(
                super::ServiceImage::try_from(value.as_ref().to_owned())
                    .map_err(|_error| super::ScaffoldError::Catalog)?,
            );
            return Ok(());
        }
        if let Some(value) = super::service_catalog_string_value::string_value(
            super::ScaffoldTextRef::from(trimmed_line),
            super::ScaffoldTextRef::from(constants_str::VALUE_94ABCB2D),
        )? {
            draft.kubernetes_manifest = Some(
                super::ServiceKubernetesManifest::try_from(value.as_ref().to_owned())
                    .map_err(|_error| super::ScaffoldError::Catalog)?,
            );
            return Ok(());
        }
        if let Some(port) = trimmed_line
            .strip_prefix(constants_str::VALUE_F8D397A3)
            .and_then(|port_text| port_text.trim().strip_prefix('='))
            .map(str::trim)
            .and_then(|port_text| port_text.parse::<u16>().ok())
        {
            draft.port = Some(super::ServicePort::from(port));
            return Ok(());
        }
        if let Some(value) = super::service_catalog_string_value::string_value(
            super::ScaffoldTextRef::from(trimmed_line),
            super::ScaffoldTextRef::from(constants_str::VALUE_20E49707),
        )? {
            draft.socket_env = Some(
                super::ServiceSocketEnv::try_from(value.as_ref().to_owned())
                    .map_err(|_error| super::ScaffoldError::Catalog)?,
            );
            return Ok(());
        }
        if let Some(release) =
            trimmed_line
                .strip_prefix(constants_str::RELEASE)
                .and_then(|release_text| {
                    release_text
                        .trim()
                        .strip_prefix('=')
                        .map(str::trim)
                        .and_then(|parsed_text| parsed_text.parse::<bool>().ok())
                })
        {
            draft.release = Some(super::ShouldRelease::from(release));
        }
        Ok::<(), super::ScaffoldError>(())
    })?;
    if let Some(draft) = current {
        entries.push(super::ServiceCatalogDraft::finish(draft)?);
    }
    if entries.is_empty() {
        return Err(super::ScaffoldError::Catalog);
    }
    Ok(super::ServiceCatalogEntries::from(
        bounded_types::domain_types::vector::BoundedVec::from_max_iter(entries),
    ))
}

#[cfg(test)]
mod tests {
    #[test]
    fn empty_catalog_is_rejected() {
        let _error = super::parse(super::super::ScaffoldTextRef::from(
            constants_str::PG_CRUD_EMPTY_SQL_SUFFIX,
        ))
        .expect_err(constants_str::VALUE_5621BCEA);
    }
}
