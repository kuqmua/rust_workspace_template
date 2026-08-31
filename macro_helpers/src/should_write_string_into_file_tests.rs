// The owner module retains lint-sensitive semantics from the original implementation.

#[cfg(test)]
pub(super) fn should_write_string_into_file(
    path: crate::written_file_path_ref::WrittenFilePathRef<'_>,
    string_cnt: crate::string_file_content_ref::StringFileContentRef<'_>,
) -> std::io::Result<crate::should_write_string::ShouldWriteString> {
    let path_ref = path.as_ref();
    let string_cnt_ref = string_cnt.as_ref();
    match std::fs::metadata(path_ref) {
        Ok(v) => {
            let new_len_u64 = u64::try_from(string_cnt_ref.len()).map_err(|_error| {
                std::io::Error::other(constants_str::VALUE_2F4D7A8C_FAILED_CONVERTING_STRING_LENGTH)
            })?;
            if v.len() != new_len_u64 {
                return Ok(crate::should_write_string::ShouldWriteString::from(true));
            }
            let mut old_file = std::fs::File::open(path_ref)?;
            let mut offset = constants_usize::ZERO;
            let mut old_chunk = [constants_u8::ZERO; 8192];
            loop {
                let read_len = std::io::Read::read(&mut old_file, &mut old_chunk)?;
                if read_len == constants_usize::ZERO {
                    if offset == string_cnt_ref.len() {
                        return Ok(crate::should_write_string::ShouldWriteString::from(false));
                    }
                    crate::validate_existing_file_text::validate_existing_file_text(
                        path,
                        crate::generated_file_maximum_bytes::GeneratedFileMaximumBytes::from(
                            string_cnt_ref.len(),
                        ),
                    )?;
                    return Ok(crate::should_write_string::ShouldWriteString::from(true));
                }
                let end = offset.checked_add(read_len).ok_or_else(|| {
                    std::io::Error::other(
                        constants_str::VALUE_5F28D14C_GENERATED_FILE_COMPARISON_OFFSET_OVERFLOW,
                    )
                })?;
                let Some(new_chunk) = string_cnt_ref.as_bytes().get(offset..end) else {
                    crate::validate_existing_file_text::validate_existing_file_text(
                        path,
                        crate::generated_file_maximum_bytes::GeneratedFileMaximumBytes::from(
                            string_cnt_ref.len(),
                        ),
                    )?;
                    return Ok(crate::should_write_string::ShouldWriteString::from(true));
                };
                let Some(old_chunk_read) = old_chunk.get(..read_len) else {
                    return Err(std::io::Error::other(constants_str::F83D470A_GENERATED_FILE_COMPARISON_READ_LENGTH_EXCEEDS_BUFFER));
                };
                if old_chunk_read != new_chunk {
                    crate::validate_existing_file_text::validate_existing_file_text(
                        path,
                        crate::generated_file_maximum_bytes::GeneratedFileMaximumBytes::from(
                            string_cnt_ref.len(),
                        ),
                    )?;
                    return Ok(crate::should_write_string::ShouldWriteString::from(true));
                }
                offset = end;
            }
        }
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            Ok(crate::should_write_string::ShouldWriteString::from(true))
        }
        Err(error) => Err(error),
    }
}
