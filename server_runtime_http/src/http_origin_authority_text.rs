#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq)]
pub(super) struct HttpOriginAuthorityText(
    bounded_types::bounded_string::BoundedString<1usize, 512usize, false>,
);

impl HttpOriginAuthorityText {
    pub(crate) const fn get(&self) -> &str {
        self.0.as_str()
    }
}

impl TryFrom<String> for HttpOriginAuthorityText {
    type Error = crate::allowed_origin_error::AllowedOriginError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() || value.len() > 512usize || value.contains('@') {
            return Err(crate::allowed_origin_error::AllowedOriginError::Invalid);
        }
        let authority = match http::uri::Authority::try_from(value) {
            Ok(authority) => authority,
            Err(_error) => return Err(crate::allowed_origin_error::AllowedOriginError::Invalid),
        };
        let port = if authority.as_str().starts_with('[') {
            authority
                .as_str()
                .find(']')
                .and_then(|end| {
                    authority
                        .as_str()
                        .get(end.saturating_add(constants_usize::ONE)..)
                })
                .filter(|suffix| !suffix.is_empty())
                .and_then(|suffix| suffix.strip_prefix(':'))
        } else {
            authority
                .as_str()
                .rsplit_once(':')
                .map(|(_host, port)| port)
        };
        if port.is_some_and(|port_text| port_text.parse::<u16>().is_err()) {
            return Err(crate::allowed_origin_error::AllowedOriginError::Invalid);
        }
        bounded_types::bounded_string::BoundedString::try_from(authority.as_str().to_owned())
            .map(Self)
            .map_err(|source| match source {
                bounded_types::bounded_string_error::BoundedStringError::AboveMaximum {
                    ..
                }
                | bounded_types::bounded_string_error::BoundedStringError::BelowMinimum {
                    ..
                } => Self::Error::Invalid,
            })
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_origin_authority_validates_ports_for_hostnames_and_bracketed_ipv6() {
        let ipv6 = format!("[{}]", std::net::Ipv6Addr::LOCALHOST);
        assert!(
            [constants_str::LOCALHOST.to_owned(), ipv6]
                .into_iter()
                .all(|host| {
                    assert!(
                        crate::http_origin_authority_text::HttpOriginAuthorityText::try_from(
                            host.clone()
                        )
                        .is_ok_and(|authority| authority.get() == host)
                    );
                    [
                        (String::new(), false),
                        (0u32.to_string(), true),
                        (80u32.to_string(), true),
                        (443u32.to_string(), true),
                        (65_535u32.to_string(), true),
                        (65_536u32.to_string(), false),
                        (constants_str::X.to_owned(), false),
                    ]
                    .into_iter()
                    .all(|(port, accepted)| {
                        let text = format!("{host}:{port}");
                        let result =
                            crate::http_origin_authority_text::HttpOriginAuthorityText::try_from(
                                text.clone(),
                            );
                        if accepted {
                            result.is_ok_and(|authority| authority.get() == text)
                        } else {
                            result == Err(crate::allowed_origin_error::AllowedOriginError::Invalid)
                        }
                    })
                })
        );
    }

    #[test]
    fn test_origin_authority_enforces_exact_text_length_and_rejects_userinfo() {
        assert!(
            [1usize, 511usize, 512usize, 513usize]
                .into_iter()
                .all(|length| {
                    let text = constants_str::X.repeat(length);
                    let result =
                        crate::http_origin_authority_text::HttpOriginAuthorityText::try_from(
                            text.clone(),
                        );
                    if length <= 512usize {
                        result.is_ok_and(|authority| authority.get() == text)
                    } else {
                        result == Err(crate::allowed_origin_error::AllowedOriginError::Invalid)
                    }
                })
        );
        assert!(
            [
                String::new(),
                format!("{}@{}", constants_str::X, constants_str::LOCALHOST),
                ' '.to_string(),
                '['.to_string(),
            ]
            .into_iter()
            .all(|text| {
                crate::http_origin_authority_text::HttpOriginAuthorityText::try_from(text)
                    == Err(crate::allowed_origin_error::AllowedOriginError::Invalid)
            })
        );
    }
}
