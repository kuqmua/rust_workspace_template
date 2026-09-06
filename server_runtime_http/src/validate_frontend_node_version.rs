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
