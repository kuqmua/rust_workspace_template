#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Default, Eq, PartialEq)]
pub struct MultipartUploadRequest {
    bytes_parts: crate::multipart_bytes_parts::MultipartBytesParts,
    payload_bytes: crate::multipart_value_length::MultipartValueLength,
    text_parts: crate::multipart_text_parts::MultipartTextParts,
}

impl MultipartUploadRequest {
    #[must_use]
    pub const fn bytes_parts(&self) -> &[crate::multipart_bytes_part::MultipartBytesPart] {
        self.bytes_parts.as_slice()
    }

    fn ensure_additional_part(
        &mut self,
        part_bytes: crate::multipart_value_length::MultipartValueLength,
        maximum: crate::multipart_payload_maximum::MultipartPayloadMaximum,
    ) -> Result<(), crate::multipart_request_error::MultipartRequestError> {
        if self
            .bytes_parts
            .as_ref()
            .len()
            .saturating_add(self.text_parts.as_ref().len())
            >= 32usize
        {
            return Err(crate::multipart_request_error::MultipartRequestError::TooManyParts);
        }
        let payload_bytes = self
            .payload_bytes
            .get()
            .checked_add(part_bytes.get())
            .ok_or(crate::multipart_request_error::MultipartRequestError::PayloadTooLarge)?;
        if payload_bytes > maximum.get() {
            return Err(crate::multipart_request_error::MultipartRequestError::PayloadTooLarge);
        }
        self.payload_bytes =
            crate::multipart_value_length::MultipartValueLength::from(payload_bytes);
        Ok(())
    }

    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub const fn text_parts(&self) -> &[crate::multipart_text_part::MultipartTextPart] {
        self.text_parts.as_slice()
    }

    pub fn with_bytes_part(
        mut self,
        part: crate::multipart_bytes_part::MultipartBytesPart,
        maximum: crate::multipart_payload_maximum::MultipartPayloadMaximum,
    ) -> Result<Self, crate::multipart_request_error::MultipartRequestError> {
        self.ensure_additional_part(
            crate::multipart_value_length::MultipartValueLength::from(part.bytes().as_ref().len()),
            maximum,
        )?;
        Vec::push(&mut self.bytes_parts, part);
        Ok(self)
    }

    pub fn with_text_part(
        mut self,
        part: crate::multipart_text_part::MultipartTextPart,
        maximum: crate::multipart_payload_maximum::MultipartPayloadMaximum,
    ) -> Result<Self, crate::multipart_request_error::MultipartRequestError> {
        self.ensure_additional_part(
            crate::multipart_value_length::MultipartValueLength::from(part.value().as_ref().len()),
            maximum,
        )?;
        self.text_parts.push(part);
        Ok(self)
    }
}
