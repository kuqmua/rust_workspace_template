#[derive(proc_macro_getters::Getters)]
#[getters(bare)]
#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq)]
pub struct AllowedOrigin {
    authority: crate::http_origin_authority_text::HttpOriginAuthorityText,
    scheme: crate::http_origin_scheme_text::HttpOriginSchemeText,
}

impl TryFrom<String> for AllowedOrigin {
    type Error = crate::allowed_origin_error::AllowedOriginError;

    fn try_from(mut value: String) -> Result<Self, Self::Error> {
        let (scheme, authority_start) = {
            let (scheme, remainder) = value
                .split_once(constants_str::TEXT_ALT_10)
                .ok_or(crate::allowed_origin_error::AllowedOriginError::Invalid)?;
            if (!scheme.eq_ignore_ascii_case(constants_str::HTTP)
                && !scheme.eq_ignore_ascii_case(constants_str::HTTPS))
                || remainder.is_empty()
                || remainder.contains(['/', '?', '#'])
            {
                return Err(crate::allowed_origin_error::AllowedOriginError::Invalid);
            }
            (
                scheme.to_owned(),
                scheme
                    .len()
                    .saturating_add(constants_str::TEXT_ALT_10.len()),
            )
        };
        drop(value.drain(..authority_start));
        let validated_authority =
            crate::http_origin_authority_text::HttpOriginAuthorityText::try_from(value)?;
        let default_port = if scheme.eq_ignore_ascii_case(constants_str::HTTP) {
            80u16
        } else {
            443u16
        };
        let authority = match validated_authority.get().rsplit_once(':') {
            Some((host, port)) if port.parse::<u16>() == Ok(default_port) => {
                crate::http_origin_authority_text::HttpOriginAuthorityText::try_from(
                    host.to_owned(),
                )?
            }
            _ => validated_authority,
        };
        Ok(Self {
            authority,
            scheme: crate::http_origin_scheme_text::HttpOriginSchemeText::try_from(scheme)?,
        })
    }
}
