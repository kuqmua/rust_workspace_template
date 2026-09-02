#[cfg(test)]
pub(super) fn make_not_found_payload_with_message(
    error_text: to_err_string::error_text::ErrorText,
    git_commit_link_cow: git_info::git_commit_link_cow::GitCommitLinkCow,
) -> crate::not_found_payload::NotFoundPayload {
    crate::not_found_payload::NotFoundPayload::from_parts(
        git_commit_link_cow,
        error_text,
        crate::open_api_specification_path::OpenApiSpecificationPath::from(
            constants_str::COMMON_ROUTES_SWAGGER_UI,
        ),
    )
}
