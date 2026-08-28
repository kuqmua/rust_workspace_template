use super::{AxumHttpUriRef, make_no_route_message_for_suffix, uri_suffix};

#[allow(
    clippy::single_call_fn,
    reason = "URI message composition remains directly unit tested"
)]
pub(super) fn make_no_route_message(
    uri: AxumHttpUriRef<'_>,
) -> to_err_string::domain_types::ErrorText {
    make_no_route_message_for_suffix(uri_suffix(uri))
}
