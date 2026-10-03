#[allow(
    clippy::single_call_fn,
    reason = "the startup version gate remains item-scoped for direct deterministic tests without launching external tools"
)]
pub(crate) fn validate_frontend_node_version(
    bounded_text: &crate::bounded_text::BoundedText,
) -> Result<(), crate::frontend_preparation_error::FrontendPreparationError> {
    let version = bounded_text
        .as_ref()
        .trim()
        .strip_prefix('v')
        .and_then(|version| version.split_once('.'))
        .ok_or(crate::frontend_preparation_error::FrontendPreparationError::NodeVersion)?
        .0
        .parse::<u32>()
        .map_err(|source| {
            crate::frontend_preparation_error::FrontendPreparationError::NodeVersionParse(
                crate::service_runtime_io_error::ServiceRuntimeIoError::from(
                    std::io::Error::other(source),
                ),
            )
        })?;
    if version < 22u32 {
        return Err(crate::frontend_preparation_error::FrontendPreparationError::NodeUnsupported);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_frontend_node_major_version_parse_errors() {
        assert!([
            String::default(),
            constants_str::X.to_owned(),
            (u64::from(u32::MAX) + 1u64).to_string(),
        ]
        .into_iter()
        .all(|major| {
            let text = format!("v{major}.0");
            crate::bounded_text::BoundedText::try_from(text).is_ok_and(
                |bounded_text| matches!(
                    crate::validate_frontend_node_version::validate_frontend_node_version(
                        &bounded_text
                    ),
                    Err(crate::frontend_preparation_error::FrontendPreparationError::NodeVersionParse(source))
                        if Some(source.to_string()) == major.parse::<u32>().err().map(|error| error.to_string())
                )
            )
        }));
    }

    #[test]
    fn test_frontend_node_major_version_threshold_and_whitespace() {
        assert!([(21u32, false), (22u32, true), (u32::MAX, true)]
            .into_iter()
            .all(|(major, supported)| {
                let text = format!(" \tv{major}.0\n");
                crate::bounded_text::BoundedText::try_from(text).is_ok_and(
                    |bounded_text| {
                        let result = crate::validate_frontend_node_version::validate_frontend_node_version(&bounded_text);
                        if supported {
                            result.is_ok()
                        } else {
                            matches!(result, Err(crate::frontend_preparation_error::FrontendPreparationError::NodeUnsupported))
                        }
                    }
                )
            }));
    }

    #[test]
    #[allow(
        clippy::panic_in_result_fn,
        reason = "the test harness propagates bounded fixture setup errors while assertions verify the version gate"
    )]
    fn test_frontend_preparation_accepts_supported_node_versions()
    -> Result<(), crate::bounded_read_error::BoundedReadError> {
        [
            constants_str::FRONTEND_NODE_VERSION_SUPPORTED,
            constants_str::FRONTEND_NODE_VERSION_NEW,
        ]
        .into_iter()
        .try_for_each(|version| {
            let bounded_text = crate::bounded_text::BoundedText::try_from(version.to_owned())?;
            assert!(matches!(
                crate::validate_frontend_node_version::validate_frontend_node_version(
                    &bounded_text
                ),
                Ok(())
            ));
            Ok(())
        })
    }

    #[test]
    #[allow(
        clippy::panic_in_result_fn,
        reason = "the test harness propagates bounded fixture setup errors while assertions verify the version gate"
    )]
    fn test_frontend_preparation_rejects_old_and_invalid_node_versions()
    -> Result<(), crate::bounded_read_error::BoundedReadError> {
        let old = crate::bounded_text::BoundedText::try_from(
            constants_str::FRONTEND_NODE_VERSION_OLD.to_owned(),
        )?;
        assert!(matches!(
            crate::validate_frontend_node_version::validate_frontend_node_version(&old),
            Err(crate::frontend_preparation_error::FrontendPreparationError::NodeUnsupported)
        ));
        let invalid =
            crate::bounded_text::BoundedText::try_from(constants_str::VALUE_F1234D75.to_owned())?;
        assert!(matches!(
            crate::validate_frontend_node_version::validate_frontend_node_version(&invalid),
            Err(crate::frontend_preparation_error::FrontendPreparationError::NodeVersion)
        ));
        Ok(())
    }
}
