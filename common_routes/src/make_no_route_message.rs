use super::{AxumHttpUriRef, make_no_route_message_for_suffix, uri_suffix};

#[cfg(test)]
pub(super) fn make_no_route_message(
    uri: AxumHttpUriRef<'_>,
) -> to_err_string::domain_types::ErrorText {
    make_no_route_message_for_suffix(uri_suffix(uri))
}
