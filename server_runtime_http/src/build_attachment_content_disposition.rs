pub fn build_attachment_content_disposition(
    file_name: crate::http_attachment_file_name_ref::HttpAttachmentFileNameRef<'_>,
) -> Result<
    crate::http_content_disposition::HttpContentDisposition,
    crate::http_content_disposition_error::HttpContentDispositionError,
> {
    let file_name_text = file_name.get();
    if file_name_text.is_empty() {
        return Err(crate::http_content_disposition_error::HttpContentDispositionError::Empty);
    }
    if file_name_text.len() > constants_usize::VALUE_4_096 {
        return Err(crate::http_content_disposition_error::HttpContentDispositionError::TooLong);
    }
    let escaped = file_name_text
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
    let encoded = percent_encoding::utf8_percent_encode(
        escaped.as_str(),
        crate::content_disposition_percent_encode_set::CONTENT_DISPOSITION_PERCENT_ENCODE_SET,
    )
    .to_string();
    let mut header = String::with_capacity(
        constants_str::CONTENT_DISPOSITION_ATTACHMENT_PREFIX
            .len()
            .saturating_add(fallback.len())
            .saturating_add(constants_str::CONTENT_DISPOSITION_UTF8_DELIMITER.len())
            .saturating_add(encoded.len()),
    );
    header.push_str(constants_str::CONTENT_DISPOSITION_ATTACHMENT_PREFIX);
    header.push_str(fallback.as_str());
    header.push_str(constants_str::CONTENT_DISPOSITION_UTF8_DELIMITER);
    header.push_str(encoded.as_str());
    http::HeaderValue::try_from(header)
        .map(crate::http_content_disposition::HttpContentDisposition::from)
        .map_err(|_error| {
            crate::http_content_disposition_error::HttpContentDispositionError::InvalidHeaderValue
        })
}
