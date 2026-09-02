#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq)]
pub(super) struct HttpOriginAuthorityText(String);

impl HttpOriginAuthorityText {
    pub(crate) const fn get(&self) -> &str {
        self.0.as_str()
    }
}

impl TryFrom<String> for HttpOriginAuthorityText {
    type Error = crate::allowed_origin_error::AllowedOriginError;

    fn try_from(string: String) -> Result<Self, Self::Error> {
        if string.is_empty() || string.len() > 512usize || string.contains('@') {
            return Err(crate::allowed_origin_error::AllowedOriginError::Invalid);
        }
        let authority = match http::uri::Authority::try_from(string) {
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
        Ok(Self(authority.as_str().to_owned()))
    }
}
