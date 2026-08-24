fn string_value(
    line: super::ScaffoldTextRef<'_>,
    key: super::ScaffoldTextRef<'_>,
) -> Result<Option<super::ScaffoldText>, super::ScaffoldError> {
    line.0
        .strip_prefix(key.0)
        .and_then(|value| value.trim().strip_prefix('='))
        .map(str::trim)
        .and_then(|value| value.strip_prefix('"'))
        .and_then(|value| value.strip_suffix('"'))
        .map(str::to_owned)
        .map(super::ScaffoldText::try_from)
        .transpose()
        .map_err(|_error| super::ScaffoldError::Catalog)
}

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
        if trimmed_line == "[[service]]" {
            if let Some(draft) = current.take() {
                entries.push(super::ServiceCatalogDraft::finish(draft)?);
            }
            current = Some(super::ServiceCatalogDraft::default());
            return Ok(());
        }
        let Some(draft) = current.as_mut() else {
            return Ok(());
        };
        if let Some(value) = string_value(
            super::ScaffoldTextRef::from(trimmed_line),
            super::ScaffoldTextRef::from("crate"),
        )? {
            draft.crate_name = Some(
                super::ServiceCrate::try_from(value.as_ref().to_owned())
                    .map_err(|_error| super::ScaffoldError::Catalog)?,
            );
            return Ok(());
        }
        if let Some(value) = string_value(
            super::ScaffoldTextRef::from(trimmed_line),
            super::ScaffoldTextRef::from("compose"),
        )? {
            draft.compose_name = Some(
                super::ServiceComposeName::try_from(value.as_ref().to_owned())
                    .map_err(|_error| super::ScaffoldError::Catalog)?,
            );
            return Ok(());
        }
        if let Some(value) = string_value(
            super::ScaffoldTextRef::from(trimmed_line),
            super::ScaffoldTextRef::from("compose_file"),
        )? {
            draft.compose_file = Some(
                super::ServiceComposeFile::try_from(value.as_ref().to_owned())
                    .map_err(|_error| super::ScaffoldError::Catalog)?,
            );
            return Ok(());
        }
        if let Some(value) = string_value(
            super::ScaffoldTextRef::from(trimmed_line),
            super::ScaffoldTextRef::from("dockerfile"),
        )? {
            draft.dockerfile = Some(
                super::ServiceDockerfile::try_from(value.as_ref().to_owned())
                    .map_err(|_error| super::ScaffoldError::Catalog)?,
            );
            return Ok(());
        }
        if let Some(value) = string_value(
            super::ScaffoldTextRef::from(trimmed_line),
            super::ScaffoldTextRef::from("image"),
        )? {
            draft.image = Some(
                super::ServiceImage::try_from(value.as_ref().to_owned())
                    .map_err(|_error| super::ScaffoldError::Catalog)?,
            );
            return Ok(());
        }
        if let Some(value) = string_value(
            super::ScaffoldTextRef::from(trimmed_line),
            super::ScaffoldTextRef::from("kubernetes"),
        )? {
            draft.kubernetes_manifest = Some(
                super::ServiceKubernetesManifest::try_from(value.as_ref().to_owned())
                    .map_err(|_error| super::ScaffoldError::Catalog)?,
            );
            return Ok(());
        }
        if let Some(port) = trimmed_line
            .strip_prefix("port")
            .and_then(|port_text| port_text.trim().strip_prefix('='))
            .map(str::trim)
            .and_then(|port_text| port_text.parse::<u16>().ok())
        {
            draft.port = Some(super::ServicePort::from(port));
            return Ok(());
        }
        if let Some(value) = string_value(
            super::ScaffoldTextRef::from(trimmed_line),
            super::ScaffoldTextRef::from("socket_env"),
        )? {
            draft.socket_env = Some(
                super::ServiceSocketEnv::try_from(value.as_ref().to_owned())
                    .map_err(|_error| super::ScaffoldError::Catalog)?,
            );
            return Ok(());
        }
        if let Some(release) = trimmed_line
            .strip_prefix("release")
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
        bounded_types::BoundedVec::from_max_iter(entries),
    ))
}

fn render_release_entries(entries: super::ServiceCatalogEntriesRef<'_>) -> super::ScaffoldText {
    let output_capacity = entries
        .0
        .iter()
        .filter(|entry| bool::from(entry.release))
        .map(|entry| {
            constants_str::WORKSPACE_SCAFFOLD_MATRIX_NAME_INDENT
                .len()
                .saturating_add(entry.image.as_ref().len())
                .saturating_add(constants_str::WORKSPACE_SCAFFOLD_MATRIX_DOCKERFILE_INDENT.len())
                .saturating_add(entry.dockerfile.as_ref().len())
                .saturating_add(constants_usize::ONE)
        })
        .sum::<usize>();
    super::ScaffoldText::try_from(
        entries
            .0
            .iter()
            .filter(|entry| bool::from(entry.release))
            .fold(
                String::with_capacity(output_capacity),
                |mut output, entry| {
                    output.push_str(constants_str::WORKSPACE_SCAFFOLD_MATRIX_NAME_INDENT);
                    output.push_str(entry.image.as_ref());
                    output.push_str(constants_str::WORKSPACE_SCAFFOLD_MATRIX_DOCKERFILE_INDENT);
                    output.push_str(entry.dockerfile.as_ref());
                    output.push('\n');
                    output
                },
            ),
    )
    .unwrap_or_else(super::ScaffoldText::from)
}

#[allow(
    clippy::single_call_fn,
    reason = "deployment synchronization owns the CI projection"
)]
pub(super) fn render_ci_matrix(
    entries: super::ServiceCatalogEntriesRef<'_>,
) -> super::ScaffoldText {
    render_release_entries(entries)
}

#[allow(
    clippy::single_call_fn,
    reason = "deployment synchronization owns the release projection"
)]
pub(super) fn render_release_matrix(
    entries: super::ServiceCatalogEntriesRef<'_>,
) -> super::ScaffoldText {
    render_release_entries(entries)
}

#[cfg(test)]
mod tests {
    #[test]
    fn empty_catalog_is_rejected() {
        let _error = super::parse(super::super::ScaffoldTextRef::from("")).expect_err("f8d37a21");
    }
}
