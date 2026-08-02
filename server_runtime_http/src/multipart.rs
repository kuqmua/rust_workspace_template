#![allow(
    clippy::arbitrary_source_item_ordering,
    reason = "multipart domain declarations stay adjacent to their validation implementations"
)]
#[derive(optml::Optml, Clone, Copy, Debug, Eq, PartialEq, newtype::FromInner)]
pub struct MultipartPayloadMaximum(usize);

#[derive(
    optml::Optml, Clone, Copy, Debug, Default, Eq, PartialEq, newtype::FromInner, newtype::Display,
)]
pub struct MultipartValueLength(usize);

#[derive(optml::Optml, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
pub enum MultipartValueError {
    #[error("multipart name must not contain control characters")]
    ControlCharacter,
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

#[derive(optml::Optml, Clone, Debug, Eq, PartialEq, newtype::AsRefStr)]
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
        if value.chars().any(char::is_control) {
            return Err(Self::Error::ControlCharacter);
        }
        Ok(Self(value))
    }
}
#[derive(optml::Optml, Clone, Debug, Eq, PartialEq, newtype::AsRefStr)]
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
        if value.chars().any(char::is_control) {
            return Err(Self::Error::ControlCharacter);
        }
        if value.contains(['/', '\\'])
            || std::path::Path::new(&value)
                .file_name()
                .and_then(|name| name.to_str())
                != Some(&value)
        {
            return Err(Self::Error::PathComponent);
        }
        Ok(Self(value))
    }
}
#[derive(optml::Optml, Clone, Debug, Eq, PartialEq, newtype::AsRefStr)]
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
#[derive(optml::Optml, Clone, Debug, Eq, PartialEq, newtype::AsRefTarget)]
pub struct MultipartBytes(bounded_types::BoundedVec<u8, 0, 16_777_216>);
impl TryFrom<Vec<u8>> for MultipartBytes {
    type Error = MultipartValueError;
    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        let actual = MultipartValueLength(value.len());
        match bounded_types::BoundedVec::try_from(value) {
            Ok(bounded) => Ok(Self(bounded)),
            Err(_error) => Err(Self::Error::TooLong { actual }),
        }
    }
}
#[derive(optml::Optml, Clone, Debug, Eq, PartialEq)]
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

#[derive(optml::Optml, Clone, Debug, Eq, PartialEq)]
pub struct MultipartBytesPart {
    bytes: MultipartBytes,
    file_name: Option<MultipartFileName>,
    name: MultipartFieldName,
}
#[derive(
    optml::Optml, Clone, Debug, Default, Eq, PartialEq, newtype::AsRefTarget, newtype::FromInner,
)]
struct MultipartBytesParts(Vec<MultipartBytesPart>);

#[derive(
    optml::Optml, Clone, Debug, Default, Eq, PartialEq, newtype::AsRefTarget, newtype::FromInner,
)]
struct MultipartTextParts(Vec<MultipartTextPart>);
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

#[derive(optml::Optml, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
pub enum MultipartRequestError {
    #[error("multipart request payload exceeds its maximum")]
    PayloadTooLarge,
    #[error("multipart request contains too many parts")]
    TooManyParts,
}

#[derive(optml::Optml, Clone, Debug, Default, Eq, PartialEq)]
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

#[derive(optml::Optml, Clone, Copy, Debug, Eq, PartialEq)]
pub enum FileStagingAction {
    Delete,
    Upload,
}

#[derive(optml::Optml, Clone, Debug, Eq, PartialEq, newtype::AsRefStr)]
pub struct FileStagingDirectoryName(String);
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

#[derive(optml::Optml, Clone, Debug, Eq, PartialEq, newtype::AsRefStr)]
pub struct StoragePathSegment(String);
#[derive(optml::Optml, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
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
#[derive(optml::Optml, Clone, Debug, Eq, PartialEq, newtype::AsRefTarget, newtype::FromInner)]
pub struct StdStorageRelativePath(std::path::PathBuf);
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
    fn field_name() -> super::MultipartFieldName {
        super::MultipartFieldName::try_from(String::from("field")).expect("0f4b54a3")
    }
    fn text_part(value: &str) -> super::MultipartTextPart {
        super::MultipartTextPart::new(
            field_name(),
            super::MultipartTextValue::try_from(value.to_owned()).expect("93b34391"),
        )
    }
    #[test]
    fn multipart_value_wrappers_enforce_each_boundary() {
        assert_eq!(
            super::MultipartFieldName::try_from(String::new()),
            Err(super::MultipartValueError::EmptyFieldName)
        );
        let _field_name =
            super::MultipartFieldName::try_from("a".repeat(256usize)).expect("1d3de882");
        assert_eq!(
            super::MultipartFieldName::try_from("a".repeat(257usize)),
            Err(super::MultipartValueError::TooLong {
                actual: super::MultipartValueLength::from(257usize)
            })
        );
        assert_eq!(
            super::MultipartFieldName::try_from(String::from("a\0b")),
            Err(super::MultipartValueError::ControlCharacter)
        );

        assert_eq!(
            super::MultipartFileName::try_from(String::new()),
            Err(super::MultipartValueError::EmptyFileName)
        );
        let _file_name =
            super::MultipartFileName::try_from("a".repeat(1024usize)).expect("7b3ca38e");
        assert_eq!(
            super::MultipartFileName::try_from("a".repeat(1025usize)),
            Err(super::MultipartValueError::TooLong {
                actual: super::MultipartValueLength::from(1025usize)
            })
        );
        assert_eq!(
            super::MultipartFileName::try_from(String::from("a\0b")),
            Err(super::MultipartValueError::ControlCharacter)
        );
        assert_eq!(
            super::MultipartFieldName::try_from(String::from("field\r\ninjected")),
            Err(super::MultipartValueError::ControlCharacter)
        );
        assert_eq!(
            super::MultipartFileName::try_from(String::from("..\\secret.txt")),
            Err(super::MultipartValueError::PathComponent)
        );

        let _text = super::MultipartTextValue::try_from("a".repeat(65_536usize)).expect("c2dd1657");
        assert_eq!(
            super::MultipartTextValue::try_from("a".repeat(65_537usize)),
            Err(super::MultipartValueError::TooLong {
                actual: super::MultipartValueLength::from(65_537usize)
            })
        );
        assert_eq!(
            super::MultipartTextValue::try_from(String::from("\0")),
            Err(super::MultipartValueError::Nul)
        );
    }
    #[test]
    fn multipart_parts_preserve_names_values_and_file_names() {
        let text = text_part("value");
        assert_eq!(text.name().as_ref(), "field");
        assert_eq!(text.value().as_ref(), "value");

        let file_name =
            super::MultipartFileName::try_from(String::from("report.txt")).expect("b76ab3ce");
        let bytes = super::MultipartBytes::try_from(vec![1u8, 2u8, 3u8]).expect("e9e23985");
        let bytes_part =
            super::MultipartBytesPart::new(field_name(), bytes).with_file_name(file_name);
        assert_eq!(bytes_part.name().as_ref(), "field");
        assert_eq!(bytes_part.bytes().as_ref(), &[1u8, 2u8, 3u8]);
        assert_eq!(
            bytes_part.file_name().map(AsRef::as_ref),
            Some("report.txt")
        );
    }
    #[test]
    fn request_enforces_combined_payload_and_part_count() {
        let limited_request = super::MultipartUploadRequest::new()
            .with_text_part(
                text_part("ab"),
                super::MultipartPayloadMaximum::from(3usize),
            )
            .expect("7797e0f1");
        assert_eq!(
            limited_request.with_text_part(
                text_part("cd"),
                super::MultipartPayloadMaximum::from(3usize)
            ),
            Err(super::MultipartRequestError::PayloadTooLarge)
        );

        let full_request = (0usize..32usize)
            .try_fold(super::MultipartUploadRequest::new(), |accumulator, _idx| {
                accumulator
                    .with_text_part(text_part(""), super::MultipartPayloadMaximum::from(0usize))
            })
            .expect("9cbea721");
        assert_eq!(full_request.text_parts().len(), 32usize);
        assert_eq!(
            full_request
                .with_text_part(text_part(""), super::MultipartPayloadMaximum::from(0usize)),
            Err(super::MultipartRequestError::TooManyParts)
        );
    }
    #[test]
    fn storage_paths_validate_segments_and_preserve_file_extensions() {
        let _valid =
            super::StoragePathSegment::try_from(String::from("abc-_123")).expect("20b6c6b2");
        assert_eq!(
            super::StoragePathSegment::try_from(String::new()),
            Err(super::StoragePathSegmentError)
        );
        assert_eq!(
            super::StoragePathSegment::try_from(String::from("../escape")),
            Err(super::StoragePathSegmentError)
        );
        assert_eq!(
            super::StoragePathSegment::try_from("a".repeat(1025usize)),
            Err(super::StoragePathSegmentError)
        );

        let identifier =
            super::StoragePathSegment::try_from(String::from("entity")).expect("ec2aa921");
        let unique = super::StoragePathSegment::try_from(String::from("unique")).expect("51bb3e40");
        let file_name =
            super::MultipartFileName::try_from(String::from("report.tar.gz")).expect("3ea5274e");
        assert_eq!(
            super::identifier_file_storage_relative_path(&identifier, &unique, &file_name).as_ref(),
            std::path::Path::new("entity/unique.gz")
        );
        let no_extension =
            super::MultipartFileName::try_from(String::from("README")).expect("b7a900a5");
        assert_eq!(
            super::identifier_file_storage_relative_path(&identifier, &unique, &no_extension)
                .as_ref(),
            std::path::Path::new("entity/unique")
        );
        assert_eq!(
            super::staging_directory_name(super::FileStagingAction::Delete)
                .expect("c5076b2f")
                .as_ref(),
            str_constants::FILE_DELETE_STAGING_DIRECTORY
        );
        assert_eq!(
            super::staging_directory_name(super::FileStagingAction::Upload)
                .expect("725e03de")
                .as_ref(),
            str_constants::FILE_UPLOAD_STAGING_DIRECTORY
        );
    }
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
