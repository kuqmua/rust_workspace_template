pub(super) fn kebab_case(value: super::ProjectNameRef<'_>) -> super::ScaffoldText {
    super::ScaffoldText::try_from(value.0.replace('_', str_constants::HYPHEN))
        .unwrap_or_else(super::ScaffoldText::from)
}

#[allow(
    clippy::single_call_fn,
    reason = "service scaffold owns title case conversion"
)]
pub(super) fn title_case(value: super::ProjectNameRef<'_>) -> super::ScaffoldText {
    capitalized_parts(value, super::ScaffoldTextRef::from(str_constants::SPACE))
}

fn capitalized_parts(
    value: super::ProjectNameRef<'_>,
    separator: super::ScaffoldTextRef<'_>,
) -> super::ScaffoldText {
    let output = value
        .0
        .split('_')
        .filter(|part| !part.is_empty())
        .enumerate()
        .fold(
            String::with_capacity(value.0.len()),
            |mut output, (index, part)| {
                if index > usize_constants::ZERO {
                    output.push_str(separator.0);
                }
                let mut chars = part.chars();
                if let Some(first) = chars.next() {
                    output.extend(first.to_uppercase());
                    output.extend(chars);
                }
                output
            },
        );
    super::ScaffoldText::try_from(output).unwrap_or_else(super::ScaffoldText::from)
}

#[allow(
    clippy::single_call_fn,
    reason = "service scaffold owns identifier case conversion"
)]
pub(super) fn upper_camel_case(value: super::ProjectNameRef<'_>) -> super::ScaffoldText {
    capitalized_parts(value, super::ScaffoldTextRef::from(str_constants::EMPTY))
}

pub(super) fn validate_project_name(
    value: super::ProjectNameRef<'_>,
) -> Result<(), super::ScaffoldError> {
    let text = value.0;
    if text.is_empty()
        || text.starts_with('_')
        || text.ends_with('_')
        || text.contains(str_constants::WORKSPACE_SCAFFOLD_DOUBLE_UNDERSCORE)
        || !text
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'_')
    {
        return Err(super::ScaffoldError::ProjectName);
    }
    Ok(())
}

#[allow(
    clippy::single_call_fn,
    reason = "project command owns repository URL validation"
)]
pub(super) fn validate_repository_url(
    value: super::RepositoryUrlRef<'_>,
) -> Result<(), super::ScaffoldError> {
    if !value.0.starts_with(str_constants::HTTPS_SCHEME_PREFIX) || value.0.ends_with('/') {
        return Err(super::ScaffoldError::RepositoryUrl);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn project_name_conversions_are_consistent() {
        let value = super::super::ProjectNameRef::from("order_platform");
        assert_eq!(super::kebab_case(value).as_ref(), "order-platform");
        assert_eq!(super::title_case(value).as_ref(), "Order Platform");
        assert_eq!(super::upper_camel_case(value).as_ref(), "OrderPlatform");
    }
}
