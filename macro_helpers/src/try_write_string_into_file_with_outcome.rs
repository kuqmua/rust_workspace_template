pub fn try_write_string_into_file_with_outcome<P>(
    file_name: P,
    string_cnt: super::StringFileContentRef<'_>,
) -> std::io::Result<super::WritePathOutcome>
where
    P: AsRef<std::path::Path>,
{
    let path = crate::RsFilePathBuf::from(file_name.as_ref().with_extension(constants_str::RS));
    let path_ref = path.as_ref();
    let string_cnt_ref = string_cnt.as_ref();
    let should_write = match std::fs::metadata(path_ref) {
        Ok(metadata) => {
            let new_len_u64 = u64::try_from(string_cnt_ref.len()).map_err(|_error| {
                std::io::Error::other(constants_str::VALUE_2F4D7A8C_FAILED_CONVERTING_STRING_LENGTH)
            })?;
            if metadata.len() == new_len_u64 {
                let mut old_file = std::fs::File::open(path_ref)?;
                let mut offset = constants_usize::ZERO;
                let mut old_chunk = [constants_u8::ZERO; 8192];
                loop {
                    let read_len = std::io::Read::read(&mut old_file, &mut old_chunk)?;
                    if read_len == constants_usize::ZERO {
                        if offset == string_cnt_ref.len() {
                            break super::ShouldWriteString::from(false);
                        }
                        super::validate_existing_file_text(
                            super::WrittenFilePathRef::from(path_ref),
                            super::GeneratedFileMaximumBytes::from(string_cnt_ref.len()),
                        )?;
                        break super::ShouldWriteString::from(true);
                    }
                    let end = offset.checked_add(read_len).ok_or_else(|| {
                        std::io::Error::other(
                            constants_str::VALUE_5F28D14C_GENERATED_FILE_COMPARISON_OFFSET_OVERFLOW,
                        )
                    })?;
                    let Some(new_chunk) = string_cnt_ref.as_bytes().get(offset..end) else {
                        super::validate_existing_file_text(
                            super::WrittenFilePathRef::from(path_ref),
                            super::GeneratedFileMaximumBytes::from(string_cnt_ref.len()),
                        )?;
                        break super::ShouldWriteString::from(true);
                    };
                    let Some(old_chunk_read) = old_chunk.get(..read_len) else {
                        return Err(std::io::Error::other(constants_str::F83D470A_GENERATED_FILE_COMPARISON_READ_LENGTH_EXCEEDS_BUFFER));
                    };
                    if old_chunk_read != new_chunk {
                        super::validate_existing_file_text(
                            super::WrittenFilePathRef::from(path_ref),
                            super::GeneratedFileMaximumBytes::from(string_cnt_ref.len()),
                        )?;
                        break super::ShouldWriteString::from(true);
                    }
                    offset = end;
                }
            } else {
                super::ShouldWriteString::from(true)
            }
        }
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            super::ShouldWriteString::from(true)
        }
        Err(error) => return Err(error),
    };
    if bool::from(should_write) {
        let mut file = atomic_write_file::AtomicWriteFile::open(path_ref)?;
        std::io::Write::write_all(&mut file, string_cnt_ref.as_bytes())?;
        file.commit()?;
    }
    let path_buf = super::WrittenFilePathBuf::from(path_ref.to_path_buf());
    Ok(if bool::from(should_write) {
        super::WritePathOutcome::Changed(path_buf)
    } else {
        super::WritePathOutcome::Unchanged(path_buf)
    })
}
