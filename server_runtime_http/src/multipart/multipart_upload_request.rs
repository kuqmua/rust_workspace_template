#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Default, Eq, PartialEq)]
pub struct MultipartUploadRequest {
    bytes_parts: super::MultipartBytesParts,
    payload_bytes: super::MultipartValueLength,
    text_parts: super::MultipartTextParts,
}

impl MultipartUploadRequest {
    #[must_use]
    pub const fn bytes_parts(&self) -> &[super::MultipartBytesPart] {
        self.bytes_parts.0.as_slice()
    }

    fn ensure_additional_part(
        &mut self,
        part_bytes: super::MultipartValueLength,
        maximum: super::MultipartPayloadMaximum,
    ) -> Result<(), super::MultipartRequestError> {
        if self
            .bytes_parts
            .as_ref()
            .len()
            .saturating_add(self.text_parts.as_ref().len())
            >= 32usize
        {
            return Err(super::MultipartRequestError::TooManyParts);
        }
        let payload_bytes = self
            .payload_bytes
            .0
            .checked_add(part_bytes.0)
            .ok_or(super::MultipartRequestError::PayloadTooLarge)?;
        if payload_bytes > maximum.0 {
            return Err(super::MultipartRequestError::PayloadTooLarge);
        }
        self.payload_bytes = super::MultipartValueLength::from(payload_bytes);
        Ok(())
    }

    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub const fn text_parts(&self) -> &[super::MultipartTextPart] {
        self.text_parts.0.as_slice()
    }

    pub fn with_bytes_part(
        mut self,
        part: super::MultipartBytesPart,
        maximum: super::MultipartPayloadMaximum,
    ) -> Result<Self, super::MultipartRequestError> {
        self.ensure_additional_part(
            super::MultipartValueLength::from(part.bytes().as_ref().len()),
            maximum,
        )?;
        self.bytes_parts.0.push(part);
        Ok(self)
    }

    pub fn with_text_part(
        mut self,
        part: super::MultipartTextPart,
        maximum: super::MultipartPayloadMaximum,
    ) -> Result<Self, super::MultipartRequestError> {
        self.ensure_additional_part(
            super::MultipartValueLength::from(part.value().as_ref().len()),
            maximum,
        )?;
        self.text_parts.0.push(part);
        Ok(self)
    }
}
