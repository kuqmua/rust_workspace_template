#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq)]
pub struct VersionedUrlSafeWireTokenText {
    encoded_payload: crate::domain_types::UrlSafeTokenPartText,
    encoded_signature: crate::domain_types::UrlSafeTokenPartText,
    version: crate::domain_types::UrlSafeTokenPartText,
}

impl VersionedUrlSafeWireTokenText {
    #[must_use]
    pub const fn encoded_payload(&self) -> &crate::domain_types::UrlSafeTokenPartText {
        &self.encoded_payload
    }

    #[must_use]
    pub const fn encoded_signature(&self) -> &crate::domain_types::UrlSafeTokenPartText {
        &self.encoded_signature
    }

    #[must_use]
    pub const fn version(&self) -> &crate::domain_types::UrlSafeTokenPartText {
        &self.version
    }
}

impl TryFrom<String> for VersionedUrlSafeWireTokenText {
    type Error = super::VersionedUrlSafeWireTokenTextError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() > constants_usize::VALUE_8_192 {
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
            crate::domain_types::UrlSafeTokenPartText::try_from(part.to_owned())
                .map_err(|_error| super::VersionedUrlSafeWireTokenTextError::InvalidPart)
        };
        Ok(Self {
            encoded_payload: parse_part(encoded_payload)?,
            encoded_signature: parse_part(encoded_signature)?,
            version: parse_part(version)?,
        })
    }
}
