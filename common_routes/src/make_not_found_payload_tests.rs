#[cfg(test)]
pub(crate) fn make_not_found_payload(
    uri: crate::axum_http_uri_ref::AxumHttpUriRef<'_>,
    commit: git_info::git_commit_link_cow::GitCommitLinkCow,
) -> crate::not_found_payload::NotFoundPayload {
    crate::make_not_found_payload_with_message_tests::make_not_found_payload_with_message(
        crate::make_no_route_message_tests::make_no_route_message(uri),
        commit,
    )
}
