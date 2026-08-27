use super::{NoRouteMessageCapacity, UriSuffixRef};

#[allow(clippy::single_call_fn)] // isolated for reuse in tests and payload builder when suffix is precomputed
pub(super) fn make_no_route_message_for_suffix(
    uri_suffix: UriSuffixRef<'_>,
) -> to_err_string::domain_types::ErrorText {
    let cap = NoRouteMessageCapacity::from(
        constants_str::COMMON_ROUTES_NO_ROUTE_MSG_PREFIX
            .len()
            .saturating_add(uri_suffix.0.len()),
    );
    let mut message = String::with_capacity(cap.0);
    message.push_str(constants_str::COMMON_ROUTES_NO_ROUTE_MSG_PREFIX);
    message.push_str(uri_suffix.0);
    to_err_string::domain_types::ErrorText::try_from(message)
        .unwrap_or_else(to_err_string::domain_types::ErrorText::from)
}
