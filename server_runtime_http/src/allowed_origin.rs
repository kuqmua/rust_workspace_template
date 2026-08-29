#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq)]
pub struct AllowedOrigin {
    pub(super) authority: crate::http_origin_authority_text::HttpOriginAuthorityText,
    pub(super) scheme: crate::http_origin_scheme_text::HttpOriginSchemeText,
}

impl TryFrom<String> for AllowedOrigin {
    type Error = crate::allowed_origin_error::AllowedOriginError;

    fn try_from(mut value: String) -> Result<Self, Self::Error> {
        let (scheme, authority_start) = {
            let (scheme, remainder) = value
                .split_once(constants_str::catalog::TEXT_ALT_10)
                .ok_or(crate::allowed_origin_error::AllowedOriginError)?;
            if (!scheme.eq_ignore_ascii_case(constants_str::catalog::HTTP)
                && !scheme.eq_ignore_ascii_case(constants_str::catalog::HTTPS))
                || remainder.is_empty()
                || remainder.contains(['/', '?', '#'])
            {
                return Err(crate::allowed_origin_error::AllowedOriginError);
            }
            (
                scheme.to_owned(),
                scheme
                    .len()
                    .saturating_add(constants_str::catalog::TEXT_ALT_10.len()),
            )
        };
        drop(value.drain(..authority_start));
        Ok(Self {
            authority: crate::http_origin_authority_text::HttpOriginAuthorityText::try_from(value)?,
            scheme: crate::http_origin_scheme_text::HttpOriginSchemeText::try_from(scheme)?,
        })
    }
}
