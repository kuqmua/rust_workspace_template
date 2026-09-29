#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Eq,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub struct OutboundAllowedHost(
    bounded_types::bounded_string::BoundedString<1usize, 253usize, false>,
);

impl OutboundAllowedHost {
    pub(crate) const fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl TryFrom<String> for OutboundAllowedHost {
    type Error = crate::outbound_host_allowlist_error::OutboundHostAllowlistError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let ipv6_host = value
            .strip_prefix('[')
            .and_then(|host| host.strip_suffix(']'))
            .and_then(|host| host.parse::<std::net::Ipv6Addr>().ok());
        if value.is_empty()
            || value.len() > 253usize
            || value.bytes().any(|byte| {
                !(byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'-' | b':' | b'[' | b']'))
            })
            || ((value.contains(':') || value.contains(['[', ']'])) && ipv6_host.is_none())
        {
            return Err(
                crate::outbound_host_allowlist_error::OutboundHostAllowlistError::InvalidHost,
            );
        }
        let normalized = ipv6_host.map_or_else(
            || value.to_ascii_lowercase(),
            |host| {
                let mut text = host.to_string();
                text.insert(constants_usize::ZERO, '[');
                text.push(']');
                text
            },
        );
        if ipv6_host.is_none() {
            let mut candidate_url = String::from(constants_str::HTTP_SCHEME_PREFIX);
            candidate_url.push_str(normalized.as_str());
            let parsed_url = match reqwest::Url::parse(candidate_url.as_str()) {
                Ok(url) => url,
                Err(_error) => {
                    return Err(
                        crate::outbound_host_allowlist_error::OutboundHostAllowlistError::InvalidHost,
                    );
                }
            };
            if parsed_url.host_str() != Some(normalized.as_str()) {
                return Err(
                    crate::outbound_host_allowlist_error::OutboundHostAllowlistError::InvalidHost,
                );
            }
        }
        bounded_types::bounded_string::BoundedString::try_from(normalized)
            .map(Self)
            .map_err(|source| match source {
                bounded_types::bounded_string_error::BoundedStringError::AboveMaximum {
                    ..
                }
                | bounded_types::bounded_string_error::BoundedStringError::BelowMinimum {
                    ..
                } => Self::Error::InvalidHost,
            })
    }
}
