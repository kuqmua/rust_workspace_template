#[derive(generate_accessor::Getters)]
#[getters(bare)]
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq)]
pub struct MultipartBytesPart {
    bytes: crate::multipart_bytes::MultipartBytes,
    #[getters(skip)]
    file_name: Option<crate::multipart_file_name::MultipartFileName>,
    name: crate::multipart_field_name::MultipartFieldName,
}

impl MultipartBytesPart {
    #[must_use]
    pub const fn file_name(&self) -> Option<&crate::multipart_file_name::MultipartFileName> {
        self.file_name.as_ref()
    }

    #[must_use]
    pub const fn new(
        name: crate::multipart_field_name::MultipartFieldName,
        bytes: crate::multipart_bytes::MultipartBytes,
    ) -> Self {
        Self {
            bytes,
            file_name: None,
            name,
        }
    }

    #[must_use]
    pub fn with_file_name(
        mut self,
        file_name: crate::multipart_file_name::MultipartFileName,
    ) -> Self {
        self.file_name = Some(file_name);
        self
    }
}
