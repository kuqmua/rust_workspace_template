use super::{
    AxumHttpUriRef, NotFoundPayload, make_no_route_message, make_not_found_payload_with_message,
};

#[cfg(test)]
pub(crate) fn make_not_found_payload(
    uri: AxumHttpUriRef<'_>,
    commit: git_info::GitCommitLinkCow,
) -> NotFoundPayload {
    make_not_found_payload_with_message(make_no_route_message(uri), commit)
}
