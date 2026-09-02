#[derive(proc_macro_getters::Getters)]
#[getters(bare)]
#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq)]
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
        multipart_field_name: crate::multipart_field_name::MultipartFieldName,
        multipart_bytes: crate::multipart_bytes::MultipartBytes,
    ) -> Self {
        Self {
            bytes: multipart_bytes,
            file_name: None,
            name: multipart_field_name,
        }
    }

    #[must_use]
    pub fn with_file_name(
        mut self,
        multipart_file_name: crate::multipart_file_name::MultipartFileName,
    ) -> Self {
        self.file_name = Some(multipart_file_name);
        self
    }
}
