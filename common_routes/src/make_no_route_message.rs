use super::{AxumHttpUriRef, make_no_route_message_for_suffix, uri_suffix};

#[allow(clippy::single_call_fn)] // single source for no-route text reused by payload builder and tests
pub(super) fn make_no_route_message(
    uri: AxumHttpUriRef<'_>,
) -> to_err_string::domain_types::ErrorText {
    make_no_route_message_for_suffix(uri_suffix(uri))
}
