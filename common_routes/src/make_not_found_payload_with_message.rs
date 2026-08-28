use super::{NotFoundPayload, OpenApiSpecificationPath};

#[allow(
    clippy::single_call_fn,
    reason = "typed payload construction remains directly unit tested"
)]
pub(super) fn make_not_found_payload_with_message(
    message: to_err_string::domain_types::ErrorText,
    commit: git_info::domain_types::GitCommitLinkCow,
) -> NotFoundPayload {
    NotFoundPayload {
        commit,
        message,
        open_api_specification: OpenApiSpecificationPath::from(
            constants_str::COMMON_ROUTES_SWAGGER_UI,
        ),
    }
}
