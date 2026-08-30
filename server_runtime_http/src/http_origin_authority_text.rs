#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq)]
pub(super) struct HttpOriginAuthorityText(pub(super) String);

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
        Ok(Self(authority.as_str().to_owned()))
    }
}
