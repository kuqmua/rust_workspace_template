#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug)]
// The owner module retains lint-sensitive semantics from the original implementation.
#[allow(clippy::arbitrary_source_item_ordering)]
pub struct OutboundUrlPolicy {
    schemes: &'static [super::OutboundUrlScheme],
    host_policy: super::OutboundHostPolicy,
}

impl OutboundUrlPolicy {
    #[must_use]
    pub const fn new(
        schemes: &'static [super::OutboundUrlScheme],
        host_policy: super::OutboundHostPolicy,
    ) -> Self {
        Self {
            schemes,
            host_policy,
        }
    }

    pub fn validate(
        self,
        value: super::OutboundUrlTextRef<'_>,
    ) -> Result<super::ReqwestOutboundUrl, super::OutboundUrlError> {
        if value.0.contains(['\0', '\r', '\n'])
            || value.0.as_bytes().windows(3usize).any(|window| {
                window.eq_ignore_ascii_case(constants_str::PERCENT_ENCODED_NUL)
                    || window.eq_ignore_ascii_case(constants_str::PERCENT_ENCODED_CR)
                    || window.eq_ignore_ascii_case(constants_str::PERCENT_ENCODED_LF)
            })
        {
            return Err(super::OutboundUrlError::ControlCharacter);
        }
        let url =
            reqwest::Url::parse(value.0).map_err(|_error| super::OutboundUrlError::Invalid)?;
        if !url.username().is_empty() || url.password().is_some() {
            return Err(super::OutboundUrlError::UserInfo);
        }
        if !self.schemes.iter().any(|scheme| match scheme {
            super::OutboundUrlScheme::Http => url.scheme() == constants_str::HTTP,
            super::OutboundUrlScheme::Https => url.scheme() == constants_str::HTTPS,
            super::OutboundUrlScheme::Rtsp => url.scheme() == constants_str::RTSP,
            super::OutboundUrlScheme::Rtsps => url.scheme() == constants_str::RTSPS,
        }) {
            return Err(super::OutboundUrlError::Scheme);
        }
        let host = url.host_str().ok_or(super::OutboundUrlError::MissingHost)?;
        if self.host_policy == super::OutboundHostPolicy::RejectPrivate
            && (host.eq_ignore_ascii_case(constants_str::LOCALHOST)
                || host
                    .to_ascii_lowercase()
                    .ends_with(constants_str::DOT_LOCALHOST))
        {
            return Err(super::OutboundUrlError::ForbiddenHost);
        }
        if self.host_policy == super::OutboundHostPolicy::RejectPrivate
            && host.parse::<std::net::IpAddr>().is_ok_and(|address| {
                super::resolve_outbound_address_disposition(super::OutboundIpAddr::from(address))
                    == super::OutboundAddressDisposition::Forbidden
            })
        {
            return Err(super::OutboundUrlError::ForbiddenHost);
        }
        Ok(super::ReqwestOutboundUrl::from(url))
    }

    pub fn validate_resolved_addresses(
        self,
        addresses: &[super::OutboundIpAddr],
    ) -> Result<(), super::OutboundUrlError> {
        if addresses.is_empty() {
            return Err(super::OutboundUrlError::MissingResolvedAddress);
        }
        if self.host_policy == super::OutboundHostPolicy::RejectPrivate
            && addresses.iter().any(|address| {
                super::resolve_outbound_address_disposition(*address)
                    == super::OutboundAddressDisposition::Forbidden
            })
        {
            Err(super::OutboundUrlError::ForbiddenHost)
        } else {
            Ok(())
        }
    }
}
