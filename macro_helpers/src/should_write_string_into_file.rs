// The owner module retains lint-sensitive semantics from the original implementation.

#[allow(clippy::single_call_fn)] // metadata comparison remains isolated from the mutation that consumes its decision
pub(super) fn should_write_string_into_file(
    path: super::WrittenFilePathRef<'_>,
    string_cnt: super::StringFileContentRef<'_>,
) -> std::io::Result<super::ShouldWriteString> {
    let path_ref = path.as_ref();
    let string_cnt_ref = string_cnt.as_ref();
    match std::fs::metadata(path_ref) {
        Ok(v) => {
            let new_len_u64 = u64::try_from(string_cnt_ref.len()).map_err(|_error| {
                std::io::Error::other(constants_str::VALUE_2F4D7A8C_FAILED_CONVERTING_STRING_LENGTH)
            })?;
            if v.len() != new_len_u64 {
                return Ok(super::ShouldWriteString::from(true));
            }
            let mut old_file = std::fs::File::open(path_ref)?;
            let mut offset = constants_usize::ZERO;
            let mut old_chunk = [constants_u8::ZERO; 8192];
            loop {
                let read_len = std::io::Read::read(&mut old_file, &mut old_chunk)?;
                if read_len == constants_usize::ZERO {
                    if offset == string_cnt_ref.len() {
                        return Ok(super::ShouldWriteString::from(false));
                    }
                    super::validate_existing_file_text(
                        path,
                        super::GeneratedFileMaximumBytes::from(string_cnt_ref.len()),
                    )?;
                    return Ok(super::ShouldWriteString::from(true));
                }
                let end = offset.checked_add(read_len).ok_or_else(|| {
                    std::io::Error::other(
                        constants_str::VALUE_5F28D14C_GENERATED_FILE_COMPARISON_OFFSET_OVERFLOW,
                    )
                })?;
                let Some(new_chunk) = string_cnt_ref.as_bytes().get(offset..end) else {
                    super::validate_existing_file_text(
                        path,
                        super::GeneratedFileMaximumBytes::from(string_cnt_ref.len()),
                    )?;
                    return Ok(super::ShouldWriteString::from(true));
                };
                let Some(old_chunk_read) = old_chunk.get(..read_len) else {
                    return Err(std::io::Error::other(constants_str::F83D470A_GENERATED_FILE_COMPARISON_READ_LENGTH_EXCEEDS_BUFFER));
                };
                if old_chunk_read != new_chunk {
                    super::validate_existing_file_text(
                        path,
                        super::GeneratedFileMaximumBytes::from(string_cnt_ref.len()),
                    )?;
                    return Ok(super::ShouldWriteString::from(true));
                }
                offset = end;
            }
        }
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            Ok(super::ShouldWriteString::from(true))
        }
        Err(error) => Err(error),
    }
}
