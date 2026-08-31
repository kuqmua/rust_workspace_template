#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug)]
// The owner module retains lint-sensitive semantics from the original implementation.
#[allow(clippy::arbitrary_source_item_ordering)]
pub struct OutboundUrlPolicy {
    schemes: &'static [crate::outbound_url_scheme::OutboundUrlScheme],
    host_policy: crate::outbound_host_policy::OutboundHostPolicy,
}

impl OutboundUrlPolicy {
    #[must_use]
    pub const fn new(
        schemes: &'static [crate::outbound_url_scheme::OutboundUrlScheme],
        host_policy: crate::outbound_host_policy::OutboundHostPolicy,
    ) -> Self {
        Self {
            schemes,
            host_policy,
        }
    }

    pub fn validate(
        self,
        value: crate::outbound_url_text_ref::OutboundUrlTextRef<'_>,
    ) -> Result<
        crate::reqwest_outbound_url::ReqwestOutboundUrl,
        crate::outbound_url_error::OutboundUrlError,
    > {
        let value_text = value.get();
        if value_text.contains(['\0', '\r', '\n'])
            || value_text.as_bytes().windows(3usize).any(|window| {
                window.eq_ignore_ascii_case(constants_str::PERCENT_ENCODED_NUL)
                    || window.eq_ignore_ascii_case(constants_str::PERCENT_ENCODED_CR)
                    || window.eq_ignore_ascii_case(constants_str::PERCENT_ENCODED_LF)
            })
        {
            return Err(crate::outbound_url_error::OutboundUrlError::ControlCharacter);
        }
        let url = reqwest::Url::parse(value_text)
            .map_err(|_error| crate::outbound_url_error::OutboundUrlError::Invalid)?;
        if !url.username().is_empty() || url.password().is_some() {
            return Err(crate::outbound_url_error::OutboundUrlError::UserInfo);
        }
        if !self.schemes.iter().any(|scheme| match scheme {
            crate::outbound_url_scheme::OutboundUrlScheme::Http => {
                url.scheme() == constants_str::HTTP
            }
            crate::outbound_url_scheme::OutboundUrlScheme::Https => {
                url.scheme() == constants_str::HTTPS
            }
            crate::outbound_url_scheme::OutboundUrlScheme::Rtsp => {
                url.scheme() == constants_str::RTSP
            }
            crate::outbound_url_scheme::OutboundUrlScheme::Rtsps => {
                url.scheme() == constants_str::RTSPS
            }
        }) {
            return Err(crate::outbound_url_error::OutboundUrlError::Scheme);
        }
        let host = url
            .host_str()
            .ok_or(crate::outbound_url_error::OutboundUrlError::MissingHost)?;
        if self.host_policy == crate::outbound_host_policy::OutboundHostPolicy::RejectPrivate
            && (host.eq_ignore_ascii_case(constants_str::LOCALHOST)
                || host
                    .to_ascii_lowercase()
                    .ends_with(constants_str::DOT_LOCALHOST))
        {
            return Err(crate::outbound_url_error::OutboundUrlError::ForbiddenHost);
        }
        if self.host_policy == crate::outbound_host_policy::OutboundHostPolicy::RejectPrivate
            && host.parse::<std::net::IpAddr>().is_ok_and(|address| {
                crate::resolve_outbound_address_disposition::resolve_outbound_address_disposition(
                    crate::outbound_ip_addr::OutboundIpAddr::from(address),
                ) == crate::outbound_address_disposition::OutboundAddressDisposition::Forbidden
            })
        {
            return Err(crate::outbound_url_error::OutboundUrlError::ForbiddenHost);
        }
        Ok(crate::reqwest_outbound_url::ReqwestOutboundUrl::from(url))
    }

    pub fn validate_resolved_addresses(
        self,
        addresses: &[crate::outbound_ip_addr::OutboundIpAddr],
    ) -> Result<(), crate::outbound_url_error::OutboundUrlError> {
        if addresses.is_empty() {
            return Err(crate::outbound_url_error::OutboundUrlError::MissingResolvedAddress);
        }
        if self.host_policy == crate::outbound_host_policy::OutboundHostPolicy::RejectPrivate
            && addresses.iter().any(|address| {
                crate::resolve_outbound_address_disposition::resolve_outbound_address_disposition(
                    *address,
                ) == crate::outbound_address_disposition::OutboundAddressDisposition::Forbidden
            })
        {
            Err(crate::outbound_url_error::OutboundUrlError::ForbiddenHost)
        } else {
            Ok(())
        }
    }
}
