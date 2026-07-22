#![allow(
    clippy::arbitrary_source_item_ordering,
    reason = "multipart domain declarations stay adjacent to their validation implementations"
)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, newtype::FromInner)]
pub struct MultipartPayloadMaximum(usize);

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, newtype::FromInner)]
pub struct MultipartValueLength(usize);

impl std::fmt::Display for MultipartValueLength {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
pub enum MultipartValueError {
    #[error("multipart field name must not be empty")]
    EmptyFieldName,
    #[error("multipart file name must not be empty")]
    EmptyFileName,
    #[error("multipart value must not contain NUL")]
    Nul,
    #[error("multipart file name must not contain path components")]
    PathComponent,
    #[error("multipart value length {actual} exceeds its maximum")]
    TooLong { actual: MultipartValueLength },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MultipartFieldName(String);
impl TryFrom<String> for MultipartFieldName {
    type Error = MultipartValueError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err(Self::Error::EmptyFieldName);
        }
        if value.len() > 256usize {
            return Err(Self::Error::TooLong {
                actual: MultipartValueLength(value.len()),
            });
        }
        if value.contains('\0') {
            return Err(Self::Error::Nul);
        }
        Ok(Self(value))
    }
}
impl AsRef<str> for MultipartFieldName {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MultipartFileName(String);
impl TryFrom<String> for MultipartFileName {
    type Error = MultipartValueError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err(Self::Error::EmptyFileName);
        }
        if value.len() > 1024usize {
            return Err(Self::Error::TooLong {
                actual: MultipartValueLength(value.len()),
            });
        }
        if value.contains('\0') {
            return Err(Self::Error::Nul);
        }
        if std::path::Path::new(&value)
            .file_name()
            .and_then(|name| name.to_str())
            != Some(&value)
        {
            return Err(Self::Error::PathComponent);
        }
        Ok(Self(value))
    }
}
impl AsRef<str> for MultipartFileName {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MultipartTextValue(String);
impl TryFrom<String> for MultipartTextValue {
    type Error = MultipartValueError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() > 65_536usize {
            return Err(Self::Error::TooLong {
                actual: MultipartValueLength(value.len()),
            });
        }
        if value.contains('\0') {
            return Err(Self::Error::Nul);
        }
        Ok(Self(value))
    }
}
impl AsRef<str> for MultipartTextValue {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MultipartBytes(Vec<u8>);
impl TryFrom<Vec<u8>> for MultipartBytes {
    type Error = MultipartValueError;
    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        if value.len() > 16_777_216usize {
            Err(Self::Error::TooLong {
                actual: MultipartValueLength(value.len()),
            })
        } else {
            Ok(Self(value))
        }
    }
}
impl AsRef<[u8]> for MultipartBytes {
    fn as_ref(&self) -> &[u8] {
        self.0.as_slice()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MultipartTextPart {
    name: MultipartFieldName,
    value: MultipartTextValue,
}
impl MultipartTextPart {
    #[must_use]
    pub const fn name(&self) -> &MultipartFieldName {
        &self.name
    }
    #[must_use]
    pub const fn new(name: MultipartFieldName, value: MultipartTextValue) -> Self {
        Self { name, value }
    }
    #[must_use]
    pub const fn value(&self) -> &MultipartTextValue {
        &self.value
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MultipartBytesPart {
    bytes: MultipartBytes,
    file_name: Option<MultipartFileName>,
    name: MultipartFieldName,
}
#[derive(Clone, Debug, Default, Eq, PartialEq, newtype::FromInner)]
struct MultipartBytesParts(Vec<MultipartBytesPart>);

#[derive(Clone, Debug, Default, Eq, PartialEq, newtype::FromInner)]
struct MultipartTextParts(Vec<MultipartTextPart>);

impl AsRef<[MultipartBytesPart]> for MultipartBytesParts {
    fn as_ref(&self) -> &[MultipartBytesPart] {
        self.0.as_slice()
    }
}
impl AsRef<[MultipartTextPart]> for MultipartTextParts {
    fn as_ref(&self) -> &[MultipartTextPart] {
        self.0.as_slice()
    }
}
impl MultipartBytesPart {
    #[must_use]
    pub const fn bytes(&self) -> &MultipartBytes {
        &self.bytes
    }
    #[must_use]
    pub const fn file_name(&self) -> Option<&MultipartFileName> {
        self.file_name.as_ref()
    }
    #[must_use]
    pub const fn name(&self) -> &MultipartFieldName {
        &self.name
    }
    #[must_use]
    pub const fn new(name: MultipartFieldName, bytes: MultipartBytes) -> Self {
        Self {
            bytes,
            file_name: None,
            name,
        }
    }
    #[must_use]
    pub fn with_file_name(mut self, file_name: MultipartFileName) -> Self {
        self.file_name = Some(file_name);
        self
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
pub enum MultipartRequestError {
    #[error("multipart request payload exceeds its maximum")]
    PayloadTooLarge,
    #[error("multipart request contains too many parts")]
    TooManyParts,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct MultipartUploadRequest {
    bytes_parts: MultipartBytesParts,
    payload_bytes: MultipartValueLength,
    text_parts: MultipartTextParts,
}
impl MultipartUploadRequest {
    #[must_use]
    pub const fn bytes_parts(&self) -> &[MultipartBytesPart] {
        self.bytes_parts.0.as_slice()
    }
    fn ensure_additional_part(
        &mut self,
        part_bytes: MultipartValueLength,
        maximum: MultipartPayloadMaximum,
    ) -> Result<(), MultipartRequestError> {
        if self
            .bytes_parts
            .as_ref()
            .len()
            .saturating_add(self.text_parts.as_ref().len())
            >= 32usize
        {
            return Err(MultipartRequestError::TooManyParts);
        }
        let payload_bytes = self
            .payload_bytes
            .0
            .checked_add(part_bytes.0)
            .ok_or(MultipartRequestError::PayloadTooLarge)?;
        if payload_bytes > maximum.0 {
            return Err(MultipartRequestError::PayloadTooLarge);
        }
        self.payload_bytes = MultipartValueLength::from(payload_bytes);
        Ok(())
    }
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    #[must_use]
    pub const fn text_parts(&self) -> &[MultipartTextPart] {
        self.text_parts.0.as_slice()
    }
    pub fn with_bytes_part(
        mut self,
        part: MultipartBytesPart,
        maximum: MultipartPayloadMaximum,
    ) -> Result<Self, MultipartRequestError> {
        self.ensure_additional_part(
            MultipartValueLength::from(part.bytes().as_ref().len()),
            maximum,
        )?;
        self.bytes_parts.0.push(part);
        Ok(self)
    }
    pub fn with_text_part(
        mut self,
        part: MultipartTextPart,
        maximum: MultipartPayloadMaximum,
    ) -> Result<Self, MultipartRequestError> {
        self.ensure_additional_part(
            MultipartValueLength::from(part.value().as_ref().len()),
            maximum,
        )?;
        self.text_parts.0.push(part);
        Ok(self)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FileStagingAction {
    Delete,
    Upload,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FileStagingDirectoryName(String);
impl AsRef<str> for FileStagingDirectoryName {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}
impl TryFrom<String> for FileStagingDirectoryName {
    type Error = MultipartValueError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() > 256usize {
            Err(MultipartValueError::TooLong {
                actual: MultipartValueLength(value.len()),
            })
        } else {
            Ok(Self(value))
        }
    }
}
pub fn staging_directory_name(
    action: FileStagingAction,
) -> Result<FileStagingDirectoryName, MultipartValueError> {
    FileStagingDirectoryName::try_from(String::from(match action {
        FileStagingAction::Delete => str_constants::FILE_DELETE_STAGING_DIRECTORY,
        FileStagingAction::Upload => str_constants::FILE_UPLOAD_STAGING_DIRECTORY,
    }))
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StoragePathSegment(String);
#[derive(Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
#[error("invalid storage path segment")]
pub struct StoragePathSegmentError;
impl TryFrom<String> for StoragePathSegment {
    type Error = StoragePathSegmentError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() > 1024usize {
            return Err(StoragePathSegmentError);
        }
        text_policy::validate_url_safe_token_part(
            text_policy::UrlSafeTokenPartRef::from(value.as_str()),
            text_policy::UrlSafeTokenPartMaximumBytes::from(1024usize),
        )
        .map_err(|_error| StoragePathSegmentError)?;
        Ok(Self(value))
    }
}
impl AsRef<str> for StoragePathSegment {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

#[derive(Clone, Debug, Eq, PartialEq, newtype::FromInner)]
pub struct StdStorageRelativePath(std::path::PathBuf);

impl AsRef<std::path::Path> for StdStorageRelativePath {
    fn as_ref(&self) -> &std::path::Path {
        self.0.as_path()
    }
}
#[must_use]
pub fn identifier_file_storage_relative_path(
    identifier: &StoragePathSegment,
    unique_file_identifier: &StoragePathSegment,
    file_name: &MultipartFileName,
) -> StdStorageRelativePath {
    let extension = std::path::Path::new(file_name.as_ref())
        .extension()
        .and_then(|value| value.to_str());
    let stored_file_name = extension.map_or_else(
        || unique_file_identifier.as_ref().to_owned(),
        |value| format!("{}.{value}", unique_file_identifier.as_ref()),
    );
    StdStorageRelativePath::from(std::path::Path::new(identifier.as_ref()).join(stored_file_name))
}

#[cfg(test)]
mod tests {
    #[test]
    fn request_rejects_payload_above_limit() {
        let name = super::MultipartFieldName::try_from(String::from(
            str_constants::TEST_MULTIPART_FILE_FIELD,
        ))
        .expect("3696f97d");
        let bytes = super::MultipartBytes::try_from(vec![0u8; 2usize]).expect("24f930b8");
        let result = super::MultipartUploadRequest::new().with_bytes_part(
            super::MultipartBytesPart::new(name, bytes),
            super::MultipartPayloadMaximum::from(1usize),
        );
        assert_eq!(result, Err(super::MultipartRequestError::PayloadTooLarge));
    }
    #[test]
    fn file_name_rejects_path_traversal() {
        assert_eq!(
            super::MultipartFileName::try_from(String::from(str_constants::TEST_PATH_TRAVERSAL,)),
            Err(super::MultipartValueError::PathComponent)
        );
    }
}
