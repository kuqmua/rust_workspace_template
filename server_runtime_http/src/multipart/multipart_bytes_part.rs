#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq)]
pub struct MultipartBytesPart {
    bytes: super::MultipartBytes,
    file_name: Option<super::MultipartFileName>,
    name: super::MultipartFieldName,
}

impl MultipartBytesPart {
    #[must_use]
    pub const fn bytes(&self) -> &super::MultipartBytes {
        &self.bytes
    }

    #[must_use]
    pub const fn file_name(&self) -> Option<&super::MultipartFileName> {
        self.file_name.as_ref()
    }

    #[must_use]
    pub const fn name(&self) -> &super::MultipartFieldName {
        &self.name
    }

    #[must_use]
    pub const fn new(name: super::MultipartFieldName, bytes: super::MultipartBytes) -> Self {
        Self {
            bytes,
            file_name: None,
            name,
        }
    }

    #[must_use]
    pub fn with_file_name(mut self, file_name: super::MultipartFileName) -> Self {
        self.file_name = Some(file_name);
        self
    }
}
