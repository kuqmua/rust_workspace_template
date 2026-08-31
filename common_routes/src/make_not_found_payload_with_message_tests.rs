#[cfg(test)]
pub(super) fn make_not_found_payload_with_message(
    message: to_err_string::error_text::ErrorText,
    commit: git_info::git_commit_link_cow::GitCommitLinkCow,
) -> crate::not_found_payload::NotFoundPayload {
    crate::not_found_payload::NotFoundPayload::from_parts(
        commit,
        message,
        crate::open_api_specification_path::OpenApiSpecificationPath::from(
            constants_str::catalog::COMMON_ROUTES_SWAGGER_UI,
        ),
    )
}
