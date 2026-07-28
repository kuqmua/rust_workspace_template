#![allow(
    clippy::arbitrary_source_item_ordering,
    reason = "HTTP header policy types stay grouped with their builders"
)]
const CONTENT_DISPOSITION_FILE_NAME_MAXIMUM_BYTES: usize = 4096usize;

#[derive(Clone, Copy, Debug, newtype::FromInner)]
pub struct HttpAttachmentFileNameRef<'value_lt>(&'value_lt str);

#[derive(Clone, Debug, newtype::FromInner, newtype::IntoInnerFrom)]
pub struct HttpContentDisposition(http::HeaderValue);

#[derive(Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
pub enum HttpContentDispositionError {
    #[error("attachment file name must not be empty")]
    Empty,
    #[error("generated Content-Disposition header value is invalid")]
    InvalidHeaderValue,
    #[error("attachment file name is too long")]
    TooLong,
}

pub fn build_attachment_content_disposition(
    file_name: HttpAttachmentFileNameRef<'_>,
) -> Result<HttpContentDisposition, HttpContentDispositionError> {
    if file_name.0.is_empty() {
        return Err(HttpContentDispositionError::Empty);
    }
    if file_name.0.len() > CONTENT_DISPOSITION_FILE_NAME_MAXIMUM_BYTES {
        return Err(HttpContentDispositionError::TooLong);
    }
    let escaped = file_name
        .0
        .chars()
        .map(|character| {
            if character == '"' || character == '/' || character == '\\' || character.is_control() {
                '_'
            } else {
                character
            }
        })
        .collect::<String>();
    let fallback = escaped.chars().fold(
        String::with_capacity(escaped.len()),
        |mut output, character| {
            output.push(if character.is_ascii() { character } else { '_' });
            output
        },
    );
    let encoded = escaped.as_bytes().iter().fold(
        String::with_capacity(escaped.len().saturating_mul(3usize)),
        |mut output, byte| {
            if byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'.' | b'_' | b'~') {
                output.push(char::from(*byte));
            } else {
                output.push('%');
                output.push(char::from(
                    str_constants::ASCII_UPPER_HEX_DIGITS
                        .get(usize::from(byte >> 4u8))
                        .copied()
                        .unwrap_or(b'0'),
                ));
                output.push(char::from(
                    str_constants::ASCII_UPPER_HEX_DIGITS
                        .get(usize::from(byte & 0x0fu8))
                        .copied()
                        .unwrap_or(b'0'),
                ));
            }
            output
        },
    );
    let mut header = String::with_capacity(
        str_constants::CONTENT_DISPOSITION_ATTACHMENT_PREFIX
            .len()
            .saturating_add(fallback.len())
            .saturating_add(str_constants::CONTENT_DISPOSITION_UTF8_DELIMITER.len())
            .saturating_add(encoded.len()),
    );
    header.push_str(str_constants::CONTENT_DISPOSITION_ATTACHMENT_PREFIX);
    header.push_str(fallback.as_str());
    header.push_str(str_constants::CONTENT_DISPOSITION_UTF8_DELIMITER);
    header.push_str(encoded.as_str());
    http::HeaderValue::try_from(header)
        .map(HttpContentDisposition)
        .map_err(|_error| HttpContentDispositionError::InvalidHeaderValue)
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
pub enum HttpContentLengthError {
    #[error("Content-Length must not be empty")]
    Empty,
    #[error("Content-Length must contain only ASCII digits")]
    InvalidSymbol,
    #[error("Content-Length exceeds u64")]
    OutOfRange,
    #[error("Content-Length contains too many digits")]
    TooLong,
}
#[derive(Clone, Debug, Eq, PartialEq, newtype::AsRefStr)]
pub struct HttpContentLength(String);
impl TryFrom<String> for HttpContentLength {
    type Error = HttpContentLengthError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() > 20usize {
            return Err(Self::Error::TooLong);
        }
        if value.is_empty() {
            return Err(Self::Error::Empty);
        }
        if !value.bytes().all(|byte| byte.is_ascii_digit()) {
            return Err(Self::Error::InvalidSymbol);
        }
        let _parsed = value
            .parse::<u64>()
            .map_err(|_error| Self::Error::OutOfRange)?;
        Ok(Self(value))
    }
}
impl TryFrom<HttpContentLength> for u64 {
    type Error = HttpContentLengthError;
    fn try_from(value: HttpContentLength) -> Result<Self, Self::Error> {
        value
            .0
            .parse::<Self>()
            .map_err(|_error| Self::Error::OutOfRange)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn content_disposition_sanitizes_and_encodes_file_name() {
        let value =
            super::build_attachment_content_disposition(super::HttpAttachmentFileNameRef::from(
                str_constants::TEST_UNSAFE_UNICODE_ATTACHMENT_FILE_NAME,
            ))
            .expect("ec78ce2e");
        let header = http::HeaderValue::from(value);
        assert_eq!(
            header,
            http::HeaderValue::from_static(
                str_constants::TEST_SAFE_UNICODE_ATTACHMENT_CONTENT_DISPOSITION
            )
        );
    }

    #[test]
    fn content_length_accepts_u64_maximum() {
        let value =
            super::HttpContentLength::try_from(str_constants::TEST_U64_MAXIMUM_TEXT.to_owned())
                .expect("f87ab266");
        assert_eq!(u64::try_from(value), Ok(u64::MAX));
    }
}
