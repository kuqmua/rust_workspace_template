#[cfg(test)]
pub(super) fn make_no_route_message_for_suffix(
    uri_suffix_ref: crate::uri_suffix_ref::UriSuffixRef<'_>,
) -> to_err_string::error_text::ErrorText {
    let cap = crate::no_route_message_capacity::NoRouteMessageCapacity::from(
        constants_str::COMMON_ROUTES_NO_ROUTE_MSG_PREFIX
            .len()
            .saturating_add(uri_suffix_ref.len()),
    );
    let mut message = String::with_capacity(*cap);
    message.push_str(constants_str::COMMON_ROUTES_NO_ROUTE_MSG_PREFIX);
    message.push_str(&uri_suffix_ref);
    to_err_string::error_text::ErrorText::try_from(message)
        .unwrap_or_else(to_err_string::error_text::ErrorText::from)
}
