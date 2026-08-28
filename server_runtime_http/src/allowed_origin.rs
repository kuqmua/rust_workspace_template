#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq)]
pub struct AllowedOrigin {
    pub(super) authority: super::HttpOriginAuthorityText,
    pub(super) scheme: super::HttpOriginSchemeText,
}

impl TryFrom<String> for AllowedOrigin {
    type Error = super::AllowedOriginError;

    fn try_from(mut value: String) -> Result<Self, Self::Error> {
        let (scheme, authority_start) = {
            let (scheme, remainder) = value
                .split_once(constants_str::TEXT_ALT_10)
                .ok_or(super::AllowedOriginError)?;
            if (!scheme.eq_ignore_ascii_case(constants_str::HTTP)
                && !scheme.eq_ignore_ascii_case(constants_str::HTTPS))
                || remainder.is_empty()
                || remainder.contains(['/', '?', '#'])
            {
                return Err(super::AllowedOriginError);
            }
            (
                scheme.to_owned(),
                scheme
                    .len()
                    .saturating_add(constants_str::TEXT_ALT_10.len()),
            )
        };
        drop(value.drain(..authority_start));
        Ok(Self {
            authority: super::HttpOriginAuthorityText::try_from(value)?,
            scheme: super::HttpOriginSchemeText::try_from(scheme)?,
        })
    }
}
