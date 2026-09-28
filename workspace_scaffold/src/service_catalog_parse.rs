#[allow(
    clippy::single_call_fn,
    reason = "service catalog parse remains named because catalog validation has focused unit tests"
)]
pub(super) fn service_catalog_parse(
    scaffold_text_ref: crate::scaffold_text_ref::ScaffoldTextRef<'_>,
) -> Result<
    crate::service_catalog_entries::ServiceCatalogEntries,
    crate::scaffold_error::ScaffoldError,
> {
    let mut entries = Vec::new();
    let mut current = None;
    scaffold_text_ref.get().lines().try_for_each(|raw_line| {
        let trimmed_line = raw_line.trim();
        if trimmed_line.is_empty() || trimmed_line.starts_with('#') {
            return Ok(());
        }
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
            return Err(crate::scaffold_error::ScaffoldError::Catalog);
        };
        if let Some(value) = crate::service_catalog_string_value::service_catalog_string_value(
            crate::scaffold_text_ref::ScaffoldTextRef::from(trimmed_line),
            crate::scaffold_text_ref::ScaffoldTextRef::from(constants_str::CRATE),
        )? {
            if draft.get_crate_name_mut().is_some() {
                return Err(crate::scaffold_error::ScaffoldError::Catalog);
            }
            *draft.get_crate_name_mut() = Some(
                crate::service_crate::ServiceCrate::try_from(value.as_ref().to_owned())
                    .map_err(|_error| crate::scaffold_error::ScaffoldError::Catalog)?,
            );
            return Ok(());
        }
        if let Some(value) = crate::service_catalog_string_value::service_catalog_string_value(
            crate::scaffold_text_ref::ScaffoldTextRef::from(trimmed_line),
            crate::scaffold_text_ref::ScaffoldTextRef::from(constants_str::VALUE_DB669AF6),
        )? {
            if draft.get_compose_name_mut().is_some() {
                return Err(crate::scaffold_error::ScaffoldError::Catalog);
            }
            *draft.get_compose_name_mut() = Some(
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
            if draft.get_compose_file_mut().is_some() {
                return Err(crate::scaffold_error::ScaffoldError::Catalog);
            }
            *draft.get_compose_file_mut() = Some(
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
            if draft.get_dockerfile_mut().is_some() {
                return Err(crate::scaffold_error::ScaffoldError::Catalog);
            }
            *draft.get_dockerfile_mut() = Some(
                crate::service_dockerfile::ServiceDockerfile::try_from(value.as_ref().to_owned())
                    .map_err(|_error| crate::scaffold_error::ScaffoldError::Catalog)?,
            );
            return Ok(());
        }
        if let Some(value) = crate::service_catalog_string_value::service_catalog_string_value(
            crate::scaffold_text_ref::ScaffoldTextRef::from(trimmed_line),
            crate::scaffold_text_ref::ScaffoldTextRef::from(constants_str::VALUE_6105D6CC),
        )? {
            if draft.get_image_mut().is_some() {
                return Err(crate::scaffold_error::ScaffoldError::Catalog);
            }
            *draft.get_image_mut() = Some(
                crate::service_image::ServiceImage::try_from(value.as_ref().to_owned())
                    .map_err(|_error| crate::scaffold_error::ScaffoldError::Catalog)?,
            );
            return Ok(());
        }
        if let Some(value) = crate::service_catalog_string_value::service_catalog_string_value(
            crate::scaffold_text_ref::ScaffoldTextRef::from(trimmed_line),
            crate::scaffold_text_ref::ScaffoldTextRef::from(constants_str::VALUE_94ABCB2D),
        )? {
            if draft.get_kubernetes_manifest_mut().is_some() {
                return Err(crate::scaffold_error::ScaffoldError::Catalog);
            }
            *draft.get_kubernetes_manifest_mut() = Some(
                crate::service_kubernetes_manifest::ServiceKubernetesManifest::try_from(
                    value.as_ref().to_owned(),
                )
                .map_err(|_error| crate::scaffold_error::ScaffoldError::Catalog)?,
            );
            return Ok(());
        }
        if let Some(port_text) = trimmed_line
            .strip_prefix(constants_str::VALUE_F8D397A3)
            .and_then(|port_text| port_text.trim().strip_prefix('='))
            .map(str::trim)
        {
            let Ok(port) = port_text.parse::<u16>() else {
                return Err(crate::scaffold_error::ScaffoldError::Catalog);
            };
            if port == constants_u16::ZERO || draft.get_port_mut().is_some() {
                return Err(crate::scaffold_error::ScaffoldError::Catalog);
            }
            *draft.get_port_mut() = Some(crate::service_port::ServicePort::from(port));
            return Ok(());
        }
        if let Some(value) = crate::service_catalog_string_value::service_catalog_string_value(
            crate::scaffold_text_ref::ScaffoldTextRef::from(trimmed_line),
            crate::scaffold_text_ref::ScaffoldTextRef::from(constants_str::VALUE_20E49707),
        )? {
            if draft.get_socket_env_mut().is_some() {
                return Err(crate::scaffold_error::ScaffoldError::Catalog);
            }
            *draft.get_socket_env_mut() = Some(
                crate::service_socket_env::ServiceSocketEnv::try_from(value.as_ref().to_owned())
                    .map_err(|_error| crate::scaffold_error::ScaffoldError::Catalog)?,
            );
            return Ok(());
        }
        if let Some(release_text) = trimmed_line
            .strip_prefix(constants_str::RELEASE)
            .and_then(|release_text| release_text.trim().strip_prefix('='))
            .map(str::trim)
        {
            let Ok(release) = release_text.parse::<bool>() else {
                return Err(crate::scaffold_error::ScaffoldError::Catalog);
            };
            if draft.get_release_mut().is_some() {
                return Err(crate::scaffold_error::ScaffoldError::Catalog);
            }
            *draft.get_release_mut() = Some(crate::should_release::ShouldRelease::from(release));
            return Ok(());
        }
        Err(crate::scaffold_error::ScaffoldError::Catalog)
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
    fn test_catalog_decodes_escaped_service_name() {
        let maybe_original_line = constants_str::VALUE_D4291B4A
            .lines()
            .find(|line| line.starts_with(constants_str::CRATE));
        assert!(maybe_original_line.is_some_and(|original_line| {
            let catalog = constants_str::VALUE_D4291B4A.replacen(
                original_line,
                constants_str::WORKSPACE_SCAFFOLD_ESCAPED_CRATE_LINE.trim_end(),
                constants_usize::ONE,
            );
            crate::service_catalog_parse::service_catalog_parse(
                crate::scaffold_text_ref::ScaffoldTextRef::from(catalog.as_str()),
            )
            .is_ok_and(|entries| {
                entries.get_inner().as_slice().first().is_some_and(|entry| {
                    entry.get_crate_name().as_ref() == constants_str::VALUE_B3EACD33
                })
            })
        }));
    }

    #[test]
    fn test_zero_service_port_is_rejected() {
        let catalog = constants_str::VALUE_D4291B4A.replacen(
            constants_str::WORKSPACE_SCAFFOLD_PORT_8080_LINE,
            constants_str::WORKSPACE_SCAFFOLD_ZERO_PORT_LINE,
            constants_usize::ONE,
        );
        assert!(matches!(
            crate::service_catalog_parse::service_catalog_parse(
                crate::scaffold_text_ref::ScaffoldTextRef::from(catalog.as_str())
            ),
            Err(crate::scaffold_error::ScaffoldError::Catalog)
        ));
    }

    #[test]
    fn test_unknown_catalog_lines_are_rejected() {
        let fixture = constants_str::VALUE_D4291B4A;
        assert!(
            [
                format!(
                    "{}{}",
                    constants_str::WORKSPACE_SCAFFOLD_UNKNOWN_FIELD_LINE,
                    fixture
                ),
                format!(
                    "{fixture}{}",
                    constants_str::WORKSPACE_SCAFFOLD_UNKNOWN_FIELD_LINE
                ),
            ]
            .into_iter()
            .all(|catalog| matches!(
                crate::service_catalog_parse::service_catalog_parse(
                    crate::scaffold_text_ref::ScaffoldTextRef::from(catalog.as_str())
                ),
                Err(crate::scaffold_error::ScaffoldError::Catalog)
            ))
        );
    }

    #[test]
    fn test_duplicate_catalog_fields_are_rejected() {
        let catalog = constants_str::VALUE_D4291B4A;
        assert!(
            [
                constants_str::CRATE,
                constants_str::VALUE_DB669AF6,
                constants_str::VALUE_739ED940,
                constants_str::VALUE_254DB0FB,
                constants_str::VALUE_6105D6CC,
                constants_str::VALUE_94ABCB2D,
                constants_str::VALUE_F8D397A3,
                constants_str::VALUE_20E49707,
                constants_str::RELEASE,
            ]
            .into_iter()
            .all(|key| {
                catalog
                    .lines()
                    .find(|line| line.starts_with(key))
                    .is_some_and(|duplicate| {
                        let duplicated_catalog =
                            format!("{catalog}{duplicate}{}", constants_str::NEWLINE);
                        matches!(
                            crate::service_catalog_parse::service_catalog_parse(
                                crate::scaffold_text_ref::ScaffoldTextRef::from(
                                    duplicated_catalog.as_str()
                                )
                            ),
                            Err(crate::scaffold_error::ScaffoldError::Catalog)
                        )
                    })
            })
        );
    }

    #[test]
    fn test_invalid_string_field_is_rejected() {
        let catalog = format!(
            "{}{}",
            constants_str::VALUE_D4291B4A,
            constants_str::WORKSPACE_SCAFFOLD_INVALID_CRATE_SUFFIX
        );
        assert!(matches!(
            crate::service_catalog_parse::service_catalog_parse(
                crate::scaffold_text_ref::ScaffoldTextRef::from(catalog.as_str())
            ),
            Err(crate::scaffold_error::ScaffoldError::Catalog)
        ));
    }

    #[test]
    fn test_invalid_port_and_release_fields_are_rejected() {
        assert!(
            [
                constants_str::WORKSPACE_SCAFFOLD_INVALID_PORT_SUFFIX,
                constants_str::WORKSPACE_SCAFFOLD_INVALID_RELEASE_SUFFIX,
            ]
            .into_iter()
            .all(|suffix| {
                let catalog = format!("{}{suffix}", constants_str::VALUE_D4291B4A);
                matches!(
                    crate::service_catalog_parse::service_catalog_parse(
                        crate::scaffold_text_ref::ScaffoldTextRef::from(catalog.as_str())
                    ),
                    Err(crate::scaffold_error::ScaffoldError::Catalog)
                )
            })
        );
    }

    #[test]
    fn test_empty_catalog_is_rejected() {
        let _error = crate::service_catalog_parse::service_catalog_parse(
            crate::scaffold_text_ref::ScaffoldTextRef::from(
                constants_str::PG_CRUD_EMPTY_SQL_SUFFIX,
            ),
        )
        .expect_err(constants_str::VALUE_5621BCEA);
    }
}
