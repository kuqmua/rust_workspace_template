#[allow(clippy::single_call_fn)] // reusable validator keeps check_commit focused on feature-toggle behavior
pub(crate) fn validate_commit_header(
    headers: crate::domain_types::header_value::AxumHeadersRef<'_>,
) -> Result<(), super::CommitError> {
    super::validate_commit_header_value(super::read_commit_header_str(headers)?)
}
