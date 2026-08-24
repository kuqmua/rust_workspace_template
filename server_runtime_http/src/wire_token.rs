const VERSIONED_WIRE_TOKEN_MAXIMUM_BYTES: usize = 8192usize;

#[derive(optml::Optml, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
pub enum VersionedUrlSafeWireTokenTextError {
    #[error("wire token contains an invalid URL-safe part")]
    InvalidPart,
    #[error("wire token has an invalid structure")]
    InvalidStructure,
    #[error("wire token is too long")]
    TooLong,
}

#[derive(optml::Optml, Clone, Debug, Eq, PartialEq)]
pub struct VersionedUrlSafeWireTokenText {
    encoded_payload: crate::UrlSafeTokenPartText,
    encoded_signature: crate::UrlSafeTokenPartText,
    version: crate::UrlSafeTokenPartText,
}
impl VersionedUrlSafeWireTokenText {
    #[must_use]
    pub const fn encoded_payload(&self) -> &crate::UrlSafeTokenPartText {
        &self.encoded_payload
    }

    #[must_use]
    pub const fn encoded_signature(&self) -> &crate::UrlSafeTokenPartText {
        &self.encoded_signature
    }

    #[must_use]
    pub const fn version(&self) -> &crate::UrlSafeTokenPartText {
        &self.version
    }
}
impl TryFrom<String> for VersionedUrlSafeWireTokenText {
    type Error = VersionedUrlSafeWireTokenTextError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() > VERSIONED_WIRE_TOKEN_MAXIMUM_BYTES {
            return Err(Self::Error::TooLong);
        }
        let mut parts = value.split('.');
        let (Some(version), Some(encoded_payload), Some(encoded_signature)) =
            (parts.next(), parts.next(), parts.next())
        else {
            return Err(Self::Error::InvalidStructure);
        };
        if parts.next().is_some() {
            return Err(Self::Error::InvalidStructure);
        }
        let parse_part = |part: &str| {
            crate::UrlSafeTokenPartText::try_from(part.to_owned())
                .map_err(|_error| VersionedUrlSafeWireTokenTextError::InvalidPart)
        };
        Ok(Self {
            encoded_payload: parse_part(encoded_payload)?,
            encoded_signature: parse_part(encoded_signature)?,
            version: parse_part(version)?,
        })
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn versioned_wire_token_splits_valid_parts() {
        let value = super::VersionedUrlSafeWireTokenText::try_from(
            str_constants::TEST_VERSIONED_URL_SAFE_WIRE_TOKEN.to_owned(),
        )
        .expect("8c3d9457 versioned_wire_token_splits_valid_parts invariant must hold");
        assert_eq!(value.version().as_ref(), str_constants::TEST_TOKEN_VERSION);
        assert_eq!(
            value.encoded_payload().as_ref(),
            str_constants::TEST_TOKEN_PAYLOAD
        );
        assert_eq!(
            value.encoded_signature().as_ref(),
            str_constants::TEST_TOKEN_SIGNATURE
        );
    }
}
