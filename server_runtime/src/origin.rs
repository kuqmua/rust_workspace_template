#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AllowedOrigin {
    authority: HttpOriginAuthorityText,
    scheme: HttpOriginSchemeText,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct HttpOriginAuthorityText(String);

impl TryFrom<String> for HttpOriginAuthorityText {
    type Error = AllowedOriginError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() || value.len() > 512usize {
            Err(AllowedOriginError)
        } else {
            Ok(Self(value))
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct HttpOriginSchemeText(String);

impl TryFrom<String> for HttpOriginSchemeText {
    type Error = AllowedOriginError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() || value.len() > 16usize {
            Err(AllowedOriginError)
        } else {
            Ok(Self(value))
        }
    }
}

impl TryFrom<String> for AllowedOrigin {
    type Error = AllowedOriginError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let (scheme, remainder) = value
            .split_once(str_constants::TEXT_ALT_10)
            .ok_or(AllowedOriginError)?;
        if (!scheme.eq_ignore_ascii_case(str_constants::HTTP)
            && !scheme.eq_ignore_ascii_case(str_constants::HTTPS))
            || remainder.is_empty()
            || remainder.contains(['/', '?', '#'])
        {
            return Err(AllowedOriginError);
        }
        Ok(Self {
            authority: HttpOriginAuthorityText::try_from(remainder.to_owned())?,
            scheme: HttpOriginSchemeText::try_from(scheme.to_owned())?,
        })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
#[error("{message}", message = str_constants::ALLOWED_HTTP_ORIGIN_IS_INVALID)]
pub struct AllowedOriginError;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AllowedOrigins(Vec<AllowedOrigin>);

impl TryFrom<Vec<String>> for AllowedOrigins {
    type Error = AllowedOriginsError;

    fn try_from(values: Vec<String>) -> Result<Self, Self::Error> {
        if values.len() > 128usize {
            return Err(AllowedOriginsError);
        }
        values
            .into_iter()
            .map(AllowedOrigin::try_from)
            .collect::<Result<Vec<AllowedOrigin>, AllowedOriginError>>()
            .map(Self)
            .map_err(|_error| AllowedOriginsError)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
#[error("{message}", message = str_constants::ALLOWED_HTTP_ORIGIN_LIST_IS_INVALID)]
pub struct AllowedOriginsError;

#[derive(Clone, Copy, Debug, newtype::FromInner)]
pub struct HttpOriginHeadersRef<'header>(&'header http::HeaderMap);

#[derive(Clone, Copy, Debug, newtype::FromInner)]
struct HttpOriginTextRef<'text>(&'text str);

#[derive(Clone, Copy, Debug, newtype::FromInner)]
struct AllowOriginSuffix(bool);

#[derive(Clone, Copy, Debug)]
struct ParsedHttpOriginRef<'text> {
    authority: HttpOriginTextRef<'text>,
    scheme: HttpOriginTextRef<'text>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, newtype::FromInner, newtype::IntoInnerFrom)]
pub struct RequestOriginAllowed(bool);

#[allow(clippy::single_call_fn)] // parsing is independently testable through origin resolution
fn parse_request_origin(
    value: HttpOriginTextRef<'_>,
    allow_suffix: AllowOriginSuffix,
) -> Option<ParsedHttpOriginRef<'_>> {
    let (scheme, remainder) = value.0.trim().split_once(str_constants::TEXT_ALT_10)?;
    if !scheme.eq_ignore_ascii_case(str_constants::HTTP)
        && !scheme.eq_ignore_ascii_case(str_constants::HTTPS)
    {
        return None;
    }
    let authority_end = remainder.find(['/', '?', '#']).unwrap_or(remainder.len());
    let authority = remainder.get(..authority_end)?;
    if authority.is_empty() || (!allow_suffix.0 && authority_end != remainder.len()) {
        None
    } else {
        Some(ParsedHttpOriginRef {
            authority: HttpOriginTextRef::from(authority),
            scheme: HttpOriginTextRef::from(scheme),
        })
    }
}

fn request_origin_value_is_allowed(
    value: HttpOriginTextRef<'_>,
    allow_suffix: AllowOriginSuffix,
    allowed_origins: &AllowedOrigins,
) -> RequestOriginAllowed {
    let Some(parsed) = parse_request_origin(value, allow_suffix) else {
        return RequestOriginAllowed::from(false);
    };
    RequestOriginAllowed::from(allowed_origins.0.iter().any(|allowed_origin| {
        allowed_origin
            .scheme
            .0
            .eq_ignore_ascii_case(parsed.scheme.0)
            && allowed_origin
                .authority
                .0
                .eq_ignore_ascii_case(parsed.authority.0)
    }))
}

#[allow(clippy::single_call_fn)] // parsing remains isolated from allow-list comparison
#[must_use]
pub fn request_origin_allowed(
    headers: HttpOriginHeadersRef<'_>,
    allowed_origins: &AllowedOrigins,
) -> RequestOriginAllowed {
    let allowed = headers.0.get(http::header::ORIGIN).map_or_else(
        || {
            headers
                .0
                .get(http::header::REFERER)
                .and_then(|value| value.to_str().ok())
                .is_some_and(|value| {
                    bool::from(request_origin_value_is_allowed(
                        HttpOriginTextRef::from(value),
                        AllowOriginSuffix::from(true),
                        allowed_origins,
                    ))
                })
        },
        |origin_header_value| {
            origin_header_value.to_str().is_ok_and(|origin_text| {
                bool::from(request_origin_value_is_allowed(
                    HttpOriginTextRef::from(origin_text),
                    AllowOriginSuffix::from(false),
                    allowed_origins,
                ))
            })
        },
    );
    RequestOriginAllowed::from(allowed)
}

#[cfg(test)]
mod tests {
    fn allowed_origins() -> super::AllowedOrigins {
        super::AllowedOrigins::try_from(vec![String::from(str_constants::HTTPS_ADMIN_EXAMPLE_COM)])
            .expect("782d2bed")
    }

    #[test]
    fn allowed_origins_reject_oversized_lists() {
        let values = vec![String::from(str_constants::HTTPS_ADMIN_EXAMPLE_COM); 129usize];
        assert_eq!(
            super::AllowedOrigins::try_from(values),
            Err(super::AllowedOriginsError)
        );
    }

    #[test]
    fn origin_requires_exact_authority_without_path() {
        let mut headers = http::HeaderMap::new();
        let _previous = headers.insert(
            http::header::ORIGIN,
            http::HeaderValue::from_static(str_constants::HTTPS_ADMIN_EXAMPLE_COM_PATH),
        );
        assert!(!bool::from(super::request_origin_allowed(
            super::HttpOriginHeadersRef::from(&headers),
            &allowed_origins(),
        )));
    }

    #[test]
    fn referer_accepts_path_and_compares_case_insensitively() {
        let mut headers = http::HeaderMap::new();
        let _previous = headers.insert(
            http::header::REFERER,
            http::HeaderValue::from_static(str_constants::HTTPS_ADMIN_EXAMPLE_COM_SETTINGS_UPPER),
        );
        assert!(bool::from(super::request_origin_allowed(
            super::HttpOriginHeadersRef::from(&headers),
            &allowed_origins(),
        )));
    }
}
