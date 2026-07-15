#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CursorMaximumLength(usize);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CursorPaginationUsage {
    CursorOnly,
    NoOffsetNoCursor,
    OffsetAndCursor,
    OffsetOnly,
}

impl CursorPaginationUsage {
    #[must_use]
    pub const fn from_presence(
        offset: OffsetPaginationPresence,
        cursor: SignedCursorPresence,
    ) -> Self {
        match (offset, cursor) {
            (OffsetPaginationPresence::Absent, SignedCursorPresence::Absent) => {
                Self::NoOffsetNoCursor
            }
            (OffsetPaginationPresence::Absent, SignedCursorPresence::Present) => Self::CursorOnly,
            (OffsetPaginationPresence::Present, SignedCursorPresence::Absent) => Self::OffsetOnly,
            (OffsetPaginationPresence::Present, SignedCursorPresence::Present) => {
                Self::OffsetAndCursor
            }
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OffsetPaginationPresence {
    Absent,
    Present,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SignedCursorPresence {
    Absent,
    Present,
}

impl TryFrom<usize> for CursorMaximumLength {
    type Error = CursorCodecBuildError;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        std::num::NonZeroUsize::new(value)
            .map(|non_zero_value| Self(non_zero_value.get()))
            .ok_or(CursorCodecBuildError::ZeroMaximumLength)
    }
}

#[derive(Clone)]
pub struct CursorSigningKey(Vec<u8>);

impl std::fmt::Debug for CursorSigningKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(std::any::type_name::<Self>())
            .finish_non_exhaustive()
    }
}

impl TryFrom<Vec<u8>> for CursorSigningKey {
    type Error = CursorSigningKeyError;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        if value.is_empty() {
            Err(CursorSigningKeyError)
        } else {
            Ok(Self(value))
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
#[error("{message}", message = str_constants::CURSOR_SIGNING_KEY_MUST_NOT_BE_EMPTY)]
pub struct CursorSigningKeyError;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CursorPayload(String);

impl CursorPayload {
    const MAXIMUM_LENGTH: usize = 65_536usize;
}

impl AsRef<str> for CursorPayload {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

impl TryFrom<String> for CursorPayload {
    type Error = CursorPayloadError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() || value.len() > Self::MAXIMUM_LENGTH {
            Err(CursorPayloadError)
        } else {
            Ok(Self(value))
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
#[error("{message}", message = str_constants::CURSOR_PAYLOAD_MUST_NOT_BE_EMPTY)]
pub struct CursorPayloadError;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SignedCursor(String);

impl SignedCursor {
    const MAXIMUM_LENGTH: usize = 65_536usize;
}

impl AsRef<str> for SignedCursor {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

impl TryFrom<String> for SignedCursor {
    type Error = SignedCursorError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() || value.len() > Self::MAXIMUM_LENGTH {
            Err(SignedCursorError)
        } else {
            Ok(Self(value))
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
#[error("{message}", message = str_constants::SIGNED_CURSOR_MUST_NOT_BE_EMPTY)]
pub struct SignedCursorError;

#[derive(Clone, Debug)]
pub struct CursorCodec {
    key: CursorSigningKey,
    maximum_length: CursorMaximumLength,
}

#[allow(clippy::arbitrary_source_item_ordering)] // constructor remains before operational methods
impl CursorCodec {
    #[must_use]
    pub const fn new(key: CursorSigningKey, maximum_length: CursorMaximumLength) -> Self {
        Self {
            key,
            maximum_length,
        }
    }

    pub fn encode(&self, payload: &CursorPayload) -> Result<SignedCursor, CursorEncodeError> {
        let encoded_payload = base64::Engine::encode(
            &base64::engine::general_purpose::URL_SAFE_NO_PAD,
            payload.as_ref().as_bytes(),
        );
        let signed_text = format!("{}.{encoded_payload}", str_constants::CURSOR_VERSION_V1);
        let mut mac =
            <hmac::Hmac<sha2::Sha256> as hmac::Mac>::new_from_slice(self.key.0.as_slice())
                .map_err(|_error| CursorEncodeError::InvalidSigningKey)?;
        hmac::Mac::update(&mut mac, signed_text.as_bytes());
        let encoded_signature = base64::Engine::encode(
            &base64::engine::general_purpose::URL_SAFE_NO_PAD,
            hmac::Mac::finalize(mac).into_bytes(),
        );
        let cursor_text = format!("{signed_text}.{encoded_signature}");
        if cursor_text.len() > self.maximum_length.0 {
            return Err(CursorEncodeError::MaximumLengthExceeded);
        }
        Ok(SignedCursor(cursor_text))
    }

    pub fn decode(&self, cursor: &SignedCursor) -> Result<CursorPayload, CursorDecodeError> {
        if cursor.as_ref().len() > self.maximum_length.0 {
            return Err(CursorDecodeError::MaximumLengthExceeded);
        }
        let mut parts = cursor.as_ref().split('.');
        let version = parts.next().ok_or(CursorDecodeError::InvalidFormat)?;
        let encoded_payload = parts.next().ok_or(CursorDecodeError::InvalidFormat)?;
        let encoded_signature = parts.next().ok_or(CursorDecodeError::InvalidFormat)?;
        if parts.next().is_some() || version != str_constants::CURSOR_VERSION_V1 {
            return Err(CursorDecodeError::InvalidFormat);
        }
        let signature = base64::Engine::decode(
            &base64::engine::general_purpose::URL_SAFE_NO_PAD,
            encoded_signature,
        )
        .map_err(|_error| CursorDecodeError::InvalidSignature)?;
        let mut mac =
            <hmac::Hmac<sha2::Sha256> as hmac::Mac>::new_from_slice(self.key.0.as_slice())
                .map_err(|_error| CursorDecodeError::InvalidSigningKey)?;
        hmac::Mac::update(&mut mac, format!("{version}.{encoded_payload}").as_bytes());
        hmac::Mac::verify_slice(mac, signature.as_slice())
            .map_err(|_error| CursorDecodeError::InvalidSignature)?;
        let payload_bytes = base64::Engine::decode(
            &base64::engine::general_purpose::URL_SAFE_NO_PAD,
            encoded_payload,
        )
        .map_err(|_error| CursorDecodeError::InvalidPayload)?;
        let payload_text =
            String::from_utf8(payload_bytes).map_err(|_error| CursorDecodeError::InvalidPayload)?;
        CursorPayload::try_from(payload_text).map_err(|_error| CursorDecodeError::InvalidPayload)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
pub enum CursorCodecBuildError {
    #[error("{message}", message = str_constants::CURSOR_MAXIMUM_LENGTH_MUST_BE_GREATER_THAN_ZERO)]
    ZeroMaximumLength,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
pub enum CursorEncodeError {
    #[error("{message}", message = str_constants::CURSOR_SIGNING_KEY_IS_INVALID)]
    InvalidSigningKey,
    #[error("{message}", message = str_constants::CURSOR_EXCEEDS_MAXIMUM_LENGTH)]
    MaximumLengthExceeded,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
pub enum CursorDecodeError {
    #[error("{message}", message = str_constants::CURSOR_FORMAT_IS_INVALID)]
    InvalidFormat,
    #[error("{message}", message = str_constants::CURSOR_PAYLOAD_IS_INVALID)]
    InvalidPayload,
    #[error("{message}", message = str_constants::CURSOR_SIGNATURE_IS_INVALID)]
    InvalidSignature,
    #[error("{message}", message = str_constants::CURSOR_SIGNING_KEY_IS_INVALID)]
    InvalidSigningKey,
    #[error("{message}", message = str_constants::CURSOR_EXCEEDS_MAXIMUM_LENGTH)]
    MaximumLengthExceeded,
}

#[cfg(test)]
mod tests {
    fn codec() -> super::CursorCodec {
        super::CursorCodec::new(
            super::CursorSigningKey::try_from(vec![7u8; 32usize]).expect("556f25ae"),
            super::CursorMaximumLength::try_from(1_024usize).expect("30c8f351"),
        )
    }

    #[test]
    fn signed_cursor_round_trip_preserves_payload() {
        let payload =
            super::CursorPayload::try_from(String::from(str_constants::CURSOR_TEST_JSON_PAYLOAD))
                .expect("ead70a9e");
        let cursor = codec().encode(&payload).expect("47ad934b");
        assert_eq!(codec().decode(&cursor).expect("cc4bf589"), payload);
    }

    #[test]
    fn modified_cursor_is_rejected() {
        let payload =
            super::CursorPayload::try_from(String::from(str_constants::CURSOR_TEST_PAYLOAD))
                .expect("256860a7");
        let cursor = codec().encode(&payload).expect("22fc1ce9");
        let modified =
            super::SignedCursor::try_from(format!("{}x", cursor.as_ref())).expect("64b5f541");
        assert_eq!(
            codec().decode(&modified),
            Err(super::CursorDecodeError::InvalidSignature)
        );
    }

    #[test]
    fn pagination_usage_distinguishes_cursor_and_offset() {
        assert_eq!(
            super::CursorPaginationUsage::from_presence(
                super::OffsetPaginationPresence::Present,
                super::SignedCursorPresence::Present,
            ),
            super::CursorPaginationUsage::OffsetAndCursor
        );
        assert_eq!(
            super::CursorPaginationUsage::from_presence(
                super::OffsetPaginationPresence::Absent,
                super::SignedCursorPresence::Present,
            ),
            super::CursorPaginationUsage::CursorOnly
        );
    }
}
