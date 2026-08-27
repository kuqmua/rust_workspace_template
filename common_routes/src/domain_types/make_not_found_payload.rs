use super::{
    AxumHttpUriRef, NotFoundPayload, make_no_route_message, make_not_found_payload_with_message,
};

#[allow(clippy::single_call_fn)] // keeps fallback payload assembly in one place
pub(crate) fn make_not_found_payload(
    uri: AxumHttpUriRef<'_>,
    commit: git_info::domain_types::GitCommitLinkCow,
) -> NotFoundPayload {
    make_not_found_payload_with_message(make_no_route_message(uri), commit)
}
