pub(super) fn service_catalog_parse(
    source: crate::scaffold_text_ref::ScaffoldTextRef<'_>,
) -> Result<
    crate::service_catalog_entries::ServiceCatalogEntries,
    crate::scaffold_error::ScaffoldError,
> {
    let mut entries = Vec::new();
    let mut current = None;
    source.0.lines().try_for_each(|raw_line| {
        let trimmed_line = raw_line.trim();
        if trimmed_line == constants_str::VALUE_484ADD83 {
            if let Some(draft) = current.take() {
                entries.push(crate::service_catalog_draft::ServiceCatalogDraft::finish(
                    draft,
                )?);
            }
            current = Some(crate::service_catalog_draft::ServiceCatalogDraft::default());
            return Ok(());
        }
        let Some(draft) = current.as_mut() else {
            return Ok(());
        };
        if let Some(value) = crate::service_catalog_string_value::service_catalog_string_value(
            crate::scaffold_text_ref::ScaffoldTextRef::from(trimmed_line),
            crate::scaffold_text_ref::ScaffoldTextRef::from(constants_str::CRATE),
        )? {
            draft.crate_name = Some(
                crate::service_crate::ServiceCrate::try_from(value.as_ref().to_owned())
                    .map_err(|_error| crate::scaffold_error::ScaffoldError::Catalog)?,
            );
            return Ok(());
        }
        if let Some(value) = crate::service_catalog_string_value::service_catalog_string_value(
            crate::scaffold_text_ref::ScaffoldTextRef::from(trimmed_line),
            crate::scaffold_text_ref::ScaffoldTextRef::from(constants_str::VALUE_DB669AF6),
        )? {
            draft.compose_name = Some(
                crate::service_compose_name::ServiceComposeName::try_from(
                    value.as_ref().to_owned(),
                )
                .map_err(|_error| crate::scaffold_error::ScaffoldError::Catalog)?,
            );
            return Ok(());
        }
        if let Some(value) = crate::service_catalog_string_value::service_catalog_string_value(
            crate::scaffold_text_ref::ScaffoldTextRef::from(trimmed_line),
            crate::scaffold_text_ref::ScaffoldTextRef::from(constants_str::VALUE_739ED940),
        )? {
            draft.compose_file = Some(
                crate::service_compose_file::ServiceComposeFile::try_from(
                    value.as_ref().to_owned(),
                )
                .map_err(|_error| crate::scaffold_error::ScaffoldError::Catalog)?,
            );
            return Ok(());
        }
        if let Some(value) = crate::service_catalog_string_value::service_catalog_string_value(
            crate::scaffold_text_ref::ScaffoldTextRef::from(trimmed_line),
            crate::scaffold_text_ref::ScaffoldTextRef::from(constants_str::VALUE_254DB0FB),
        )? {
            draft.dockerfile = Some(
                crate::service_dockerfile::ServiceDockerfile::try_from(value.as_ref().to_owned())
                    .map_err(|_error| crate::scaffold_error::ScaffoldError::Catalog)?,
            );
            return Ok(());
        }
        if let Some(value) = crate::service_catalog_string_value::service_catalog_string_value(
            crate::scaffold_text_ref::ScaffoldTextRef::from(trimmed_line),
            crate::scaffold_text_ref::ScaffoldTextRef::from(constants_str::VALUE_6105D6CC),
        )? {
            draft.image = Some(
                crate::service_image::ServiceImage::try_from(value.as_ref().to_owned())
                    .map_err(|_error| crate::scaffold_error::ScaffoldError::Catalog)?,
            );
            return Ok(());
        }
        if let Some(value) = crate::service_catalog_string_value::service_catalog_string_value(
            crate::scaffold_text_ref::ScaffoldTextRef::from(trimmed_line),
            crate::scaffold_text_ref::ScaffoldTextRef::from(constants_str::VALUE_94ABCB2D),
        )? {
            draft.kubernetes_manifest = Some(
                crate::service_kubernetes_manifest::ServiceKubernetesManifest::try_from(
                    value.as_ref().to_owned(),
                )
                .map_err(|_error| crate::scaffold_error::ScaffoldError::Catalog)?,
            );
            return Ok(());
        }
        if let Some(port) = trimmed_line
            .strip_prefix(constants_str::VALUE_F8D397A3)
            .and_then(|port_text| port_text.trim().strip_prefix('='))
            .map(str::trim)
            .and_then(|port_text| port_text.parse::<u16>().ok())
        {
            draft.port = Some(crate::service_port::ServicePort::from(port));
            return Ok(());
        }
        if let Some(value) = crate::service_catalog_string_value::service_catalog_string_value(
            crate::scaffold_text_ref::ScaffoldTextRef::from(trimmed_line),
            crate::scaffold_text_ref::ScaffoldTextRef::from(constants_str::VALUE_20E49707),
        )? {
            draft.socket_env = Some(
                crate::service_socket_env::ServiceSocketEnv::try_from(value.as_ref().to_owned())
                    .map_err(|_error| crate::scaffold_error::ScaffoldError::Catalog)?,
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
            draft.release = Some(crate::should_release::ShouldRelease::from(release));
        }
        Ok::<(), crate::scaffold_error::ScaffoldError>(())
    })?;
    if let Some(draft) = current {
        entries.push(crate::service_catalog_draft::ServiceCatalogDraft::finish(
            draft,
        )?);
    }
    if entries.is_empty() {
        return Err(crate::scaffold_error::ScaffoldError::Catalog);
    }
    Ok(crate::service_catalog_entries::ServiceCatalogEntries::from(
        bounded_types::bounded_vec::BoundedVec::from_max_iter(entries),
    ))
}

#[cfg(test)]
mod tests {
    #[test]
    fn empty_catalog_is_rejected() {
        let _error = crate::service_catalog_parse::service_catalog_parse(
            crate::scaffold_text_ref::ScaffoldTextRef::from(
                constants_str::PG_CRUD_EMPTY_SQL_SUFFIX,
            ),
        )
        .expect_err(constants_str::VALUE_5621BCEA);
    }
}
