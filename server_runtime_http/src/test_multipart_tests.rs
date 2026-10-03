#[cfg(test)]
mod tests {
    #[test]
    fn test_multipart_name_length_errors_precede_content_errors() {
        assert!(['\0', '\n', '/', '\\'].into_iter().all(|character| {
            let field = character.to_string().repeat(257usize);
            let filename = character.to_string().repeat(1025usize);
            crate::multipart_field_name::MultipartFieldName::try_from(field)
                == Err(crate::multipart_value_error::MultipartValueError::TooLong {
                    actual: crate::multipart_value_length::MultipartValueLength::from(257usize),
                })
                && crate::multipart_file_name::MultipartFileName::try_from(filename)
                    == Err(crate::multipart_value_error::MultipartValueError::TooLong {
                        actual: crate::multipart_value_length::MultipartValueLength::from(
                            1025usize,
                        ),
                    })
        }));
    }

    #[test]
    fn test_multipart_name_ascii_validation_preserves_allowed_characters() {
        assert!((0u8..=127u8).all(|byte| {
            let character = char::from(byte);
            let text = format!("{}{character}", constants_str::X);
            let field = crate::multipart_field_name::MultipartFieldName::try_from(text.clone());
            let filename = crate::multipart_file_name::MultipartFileName::try_from(text.clone());
            if character.is_control() {
                return field
                    == Err(crate::multipart_value_error::MultipartValueError::ControlCharacter)
                    && filename
                        == Err(
                            crate::multipart_value_error::MultipartValueError::ControlCharacter,
                        );
            }
            field.is_ok_and(|value| value.as_ref() == text)
                && if matches!(character, '/' | '\\') {
                    filename
                        == Err(crate::multipart_value_error::MultipartValueError::PathComponent)
                } else {
                    filename.is_ok_and(|value| value.as_ref() == text)
                }
        }));
    }

    #[test]
    fn test_multipart_names_preserve_utf8_and_reject_exact_byte_overflow() {
        let filename = '\u{e9}'.to_string().repeat(512usize);
        assert!(
            crate::multipart_file_name::MultipartFileName::try_from(filename.clone())
                .is_ok_and(|value| value.as_ref() == filename)
        );
        let mut oversized_filename = filename;
        oversized_filename.push('x');
        assert_eq!(
            crate::multipart_file_name::MultipartFileName::try_from(oversized_filename),
            Err(crate::multipart_value_error::MultipartValueError::TooLong {
                actual: crate::multipart_value_length::MultipartValueLength::from(1025usize)
            })
        );
        let field = '\u{e9}'.to_string().repeat(128usize);
        assert!(
            crate::multipart_field_name::MultipartFieldName::try_from(field.clone())
                .is_ok_and(|value| value.as_ref() == field)
        );
        let mut oversized_field = field;
        oversized_field.push('x');
        assert_eq!(
            crate::multipart_field_name::MultipartFieldName::try_from(oversized_field),
            Err(crate::multipart_value_error::MultipartValueError::TooLong {
                actual: crate::multipart_value_length::MultipartValueLength::from(257usize)
            })
        );
    }

    #[test]
    fn test_multipart_filename_rejects_path_components_and_control_characters() {
        assert!(
            [
                constants_str::CURRENT_PATH_SEGMENT.to_owned(),
                constants_str::PARENT_PATH_SEGMENT.to_owned(),
                '/'.to_string(),
                '\\'.to_string(),
                format!("{}/{}", constants_str::X, constants_str::X),
                format!("{}\\{}", constants_str::X, constants_str::X),
            ]
            .into_iter()
            .all(|text| {
                crate::multipart_file_name::MultipartFileName::try_from(text)
                    == Err(crate::multipart_value_error::MultipartValueError::PathComponent)
            })
        );
        assert!(
            ['\0', '\n', '\r', '\t', '\u{7f}', '\u{85}']
                .into_iter()
                .all(|character| {
                    let mut text = constants_str::X.to_owned();
                    text.push(character);
                    crate::multipart_file_name::MultipartFileName::try_from(text.clone())
                        == Err(crate::multipart_value_error::MultipartValueError::ControlCharacter)
                        && crate::multipart_field_name::MultipartFieldName::try_from(text)
                            == Err(
                                crate::multipart_value_error::MultipartValueError::ControlCharacter,
                            )
                })
        );
    }

    fn field_name() -> crate::multipart_field_name::MultipartFieldName {
        crate::multipart_field_name::MultipartFieldName::try_from(String::from(
            constants_str::FIELD,
        ))
        .expect(constants_str::DIAGNOSTIC_0F4B54A3)
    }
    fn text_part(str: &str) -> crate::multipart_text_part::MultipartTextPart {
        crate::multipart_text_part::MultipartTextPart::new(
            field_name(),
            crate::multipart_text_value::MultipartTextValue::try_from(str.to_owned())
                .expect(constants_str::DIAGNOSTIC_93B34391),
        )
    }
    #[test]
    fn test_multipart_value_wrappers_enforce_each_boundary() {
        assert_eq!(
            crate::multipart_field_name::MultipartFieldName::try_from(String::new()),
            Err(crate::multipart_value_error::MultipartValueError::EmptyFieldName)
        );
        let _field_name = crate::multipart_field_name::MultipartFieldName::try_from(
            constants_str::A_ALT.repeat(256usize),
        )
        .expect(constants_str::DIAGNOSTIC_1D3DE882);
        assert_eq!(
            crate::multipart_field_name::MultipartFieldName::try_from(
                constants_str::A_ALT.repeat(257usize)
            ),
            Err(crate::multipart_value_error::MultipartValueError::TooLong {
                actual: crate::multipart_value_length::MultipartValueLength::from(257usize)
            })
        );
        assert_eq!(
            crate::multipart_field_name::MultipartFieldName::try_from(String::from(
                constants_str::VALUE_59B271AE
            )),
            Err(crate::multipart_value_error::MultipartValueError::ControlCharacter)
        );

        assert_eq!(
            crate::multipart_file_name::MultipartFileName::try_from(String::new()),
            Err(crate::multipart_value_error::MultipartValueError::EmptyFileName)
        );
        let _file_name = crate::multipart_file_name::MultipartFileName::try_from(
            constants_str::A_ALT.repeat(1024usize),
        )
        .expect(constants_str::DIAGNOSTIC_7B3CA38E);
        assert_eq!(
            crate::multipart_file_name::MultipartFileName::try_from(
                constants_str::A_ALT.repeat(1025usize)
            ),
            Err(crate::multipart_value_error::MultipartValueError::TooLong {
                actual: crate::multipart_value_length::MultipartValueLength::from(1025usize)
            })
        );
        assert_eq!(
            crate::multipart_file_name::MultipartFileName::try_from(String::from(
                constants_str::VALUE_59B271AE
            )),
            Err(crate::multipart_value_error::MultipartValueError::ControlCharacter)
        );
        assert_eq!(
            crate::multipart_field_name::MultipartFieldName::try_from(String::from(
                constants_str::VALUE_0C6873A1
            )),
            Err(crate::multipart_value_error::MultipartValueError::ControlCharacter)
        );
        assert_eq!(
            crate::multipart_file_name::MultipartFileName::try_from(String::from(
                constants_str::VALUE_0B8B255E
            )),
            Err(crate::multipart_value_error::MultipartValueError::PathComponent)
        );

        let _text = crate::multipart_text_value::MultipartTextValue::try_from(
            constants_str::A_ALT.repeat(65_536usize),
        )
        .expect(constants_str::DIAGNOSTIC_C2DD1657);
        assert_eq!(
            crate::multipart_text_value::MultipartTextValue::try_from(
                constants_str::A_ALT.repeat(65_537usize)
            ),
            Err(crate::multipart_value_error::MultipartValueError::TooLong {
                actual: crate::multipart_value_length::MultipartValueLength::from(65_537usize)
            })
        );
        assert_eq!(
            crate::multipart_text_value::MultipartTextValue::try_from(String::from(
                constants_str::VALUE_6E340B9C
            )),
            Err(crate::multipart_value_error::MultipartValueError::Nul)
        );
    }
    #[test]
    fn test_multipart_parts_preserve_names_values_and_file_names() {
        let text = text_part(constants_str::VALUE_CD42404D);
        assert_eq!(text.name().as_ref(), constants_str::FIELD);
        assert_eq!(
            text.value().as_ref(),
            constants_str::CODE_STYLE_VALUE_IDENTIFIER
        );

        let file_name = crate::multipart_file_name::MultipartFileName::try_from(String::from(
            constants_str::VALUE_EAFB4AFF,
        ))
        .expect(constants_str::DIAGNOSTIC_B76AB3CE);
        let bytes = crate::multipart_bytes::MultipartBytes::try_from(vec![1u8, 2u8, 3u8])
            .expect(constants_str::DIAGNOSTIC_E9E23985);
        let bytes_part = crate::multipart_bytes_part::MultipartBytesPart::new(field_name(), bytes)
            .with_file_name(file_name);
        assert_eq!(bytes_part.name().as_ref(), constants_str::FIELD);
        assert_eq!(bytes_part.bytes().as_ref(), &[1u8, 2u8, 3u8]);
        assert_eq!(
            bytes_part.file_name().map(AsRef::as_ref),
            Some(constants_str::VALUE_EAFB4AFF)
        );
        let empty_bytes_result = crate::multipart_bytes::MultipartBytes::try_from(Vec::new());
        assert!(empty_bytes_result.is_ok());
        let Ok(empty_bytes) = empty_bytes_result else {
            return;
        };
        let unnamed_file =
            crate::multipart_bytes_part::MultipartBytesPart::new(field_name(), empty_bytes);
        let maximum = crate::multipart_payload_maximum::MultipartPayloadMaximum::from(6usize);
        let request = crate::multipart_upload_request::MultipartUploadRequest::new()
            .with_bytes_part(bytes_part, maximum)
            .and_then(|multipart_upload_request| {
                multipart_upload_request.with_text_part(text_part(constants_str::AB), maximum)
            })
            .and_then(|multipart_upload_request| {
                multipart_upload_request.with_bytes_part(unnamed_file, maximum)
            })
            .and_then(|multipart_upload_request| {
                multipart_upload_request.with_text_part(text_part(constants_str::X), maximum)
            });
        assert!(request.is_ok_and(|multipart_upload_request| {
            matches!(multipart_upload_request.bytes_parts(), [first, second]
                if first.name().as_ref() == constants_str::FIELD
                    && first.bytes().as_ref() == [1u8, 2u8, 3u8]
                    && first.file_name().is_some_and(|multipart_file_name| multipart_file_name.as_ref() == constants_str::VALUE_EAFB4AFF)
                    && second.name().as_ref() == constants_str::FIELD
                    && second.bytes().as_ref().is_empty()
                    && second.file_name().is_none())
                && matches!(multipart_upload_request.text_parts(), [first, second]
                    if first.value().as_ref() == constants_str::AB && second.value().as_ref() == constants_str::X)
        }));
    }
    #[test]
    fn test_request_enforces_combined_payload_and_part_count() {
        let limited_request = crate::multipart_upload_request::MultipartUploadRequest::new()
            .with_text_part(
                text_part(constants_str::AB),
                crate::multipart_payload_maximum::MultipartPayloadMaximum::from(3usize),
            )
            .expect(constants_str::DIAGNOSTIC_7797E0F1);
        assert_eq!(
            limited_request.with_text_part(
                text_part(constants_str::VALUE_21E721C3),
                crate::multipart_payload_maximum::MultipartPayloadMaximum::from(3usize)
            ),
            Err(crate::multipart_request_error::MultipartRequestError::PayloadTooLarge)
        );

        let full_request = (constants_usize::ZERO..32usize)
            .try_fold(
                crate::multipart_upload_request::MultipartUploadRequest::new(),
                |accumulator, _index| {
                    accumulator.with_text_part(
                        text_part(constants_str::PG_CRUD_EMPTY_SQL_SUFFIX),
                        crate::multipart_payload_maximum::MultipartPayloadMaximum::from(
                            constants_usize::ZERO,
                        ),
                    )
                },
            )
            .expect(constants_str::DIAGNOSTIC_9CBEA721);
        assert_eq!(full_request.text_parts().len(), 32usize);
        assert_eq!(
            full_request.with_text_part(
                text_part(constants_str::EMPTY),
                crate::multipart_payload_maximum::MultipartPayloadMaximum::from(
                    constants_usize::ZERO
                )
            ),
            Err(crate::multipart_request_error::MultipartRequestError::TooManyParts)
        );
    }
    #[test]
    fn test_storage_paths_validate_segments_and_preserve_file_extensions() {
        let _valid = crate::storage_path_segment::StoragePathSegment::try_from(String::from(
            constants_str::VALUE_A31BB256,
        ))
        .expect(constants_str::DIAGNOSTIC_20B6C6B2);
        assert_eq!(
            crate::storage_path_segment::StoragePathSegment::try_from(String::new()),
            Err(crate::storage_path_segment_error::StoragePathSegmentError::Invalid)
        );
        assert_eq!(
            crate::storage_path_segment::StoragePathSegment::try_from(String::from(
                constants_str::VALUE_1BA7343C
            )),
            Err(crate::storage_path_segment_error::StoragePathSegmentError::Invalid)
        );
        assert_eq!(
            crate::storage_path_segment::StoragePathSegment::try_from(
                constants_str::A_ALT.repeat(1025usize)
            ),
            Err(crate::storage_path_segment_error::StoragePathSegmentError::Invalid)
        );

        let identifier = crate::storage_path_segment::StoragePathSegment::try_from(String::from(
            constants_str::VALUE_BCA3685F,
        ))
        .expect(constants_str::DIAGNOSTIC_EC2AA921);
        let unique = crate::storage_path_segment::StoragePathSegment::try_from(String::from(
            constants_str::VALUE_C2720445,
        ))
        .expect(constants_str::DIAGNOSTIC_51BB3E40);
        let file_name = crate::multipart_file_name::MultipartFileName::try_from(String::from(
            constants_str::VALUE_4A1282F3,
        ))
        .expect(constants_str::DIAGNOSTIC_3EA5274E);
        assert_eq!(
            crate::identifier_file_storage_relative_path::identifier_file_storage_relative_path(
                &identifier,
                &unique,
                &file_name
            )
            .as_ref(),
            std::path::Path::new(constants_str::VALUE_E5F0A5A4)
        );
        let no_extension = crate::multipart_file_name::MultipartFileName::try_from(String::from(
            constants_str::VALUE_2B7814D3,
        ))
        .expect(constants_str::DIAGNOSTIC_B7A900A5);
        assert_eq!(
            crate::identifier_file_storage_relative_path::identifier_file_storage_relative_path(
                &identifier,
                &unique,
                &no_extension
            )
            .as_ref(),
            std::path::Path::new(constants_str::VALUE_9779C6F7)
        );
        assert_eq!(
            crate::staging_directory_name::staging_directory_name(
                crate::file_staging_action::FileStagingAction::Delete
            )
            .expect(constants_str::DIAGNOSTIC_C5076B2F)
            .as_ref(),
            constants_str::FILE_DELETE_STAGING_DIRECTORY
        );
        assert_eq!(
            crate::staging_directory_name::staging_directory_name(
                crate::file_staging_action::FileStagingAction::Upload
            )
            .expect(constants_str::DIAGNOSTIC_725E03DE)
            .as_ref(),
            constants_str::FILE_UPLOAD_STAGING_DIRECTORY
        );
    }
    #[test]
    fn test_request_rejects_payload_above_limit() {
        let name = crate::multipart_field_name::MultipartFieldName::try_from(String::from(
            constants_str::TEST_MULTIPART_FILE_FIELD,
        ))
        .expect(constants_str::DIAGNOSTIC_3696F97D);
        let bytes =
            crate::multipart_bytes::MultipartBytes::try_from(vec![constants_u8::ZERO; 2usize])
                .expect(constants_str::DIAGNOSTIC_24F930B8);
        let result = crate::multipart_upload_request::MultipartUploadRequest::new()
            .with_bytes_part(
                crate::multipart_bytes_part::MultipartBytesPart::new(name, bytes),
                crate::multipart_payload_maximum::MultipartPayloadMaximum::from(
                    constants_usize::ONE,
                ),
            );
        assert_eq!(
            result,
            Err(crate::multipart_request_error::MultipartRequestError::PayloadTooLarge)
        );
    }
    #[test]
    fn test_file_name_rejects_path_traversal() {
        assert_eq!(
            crate::multipart_file_name::MultipartFileName::try_from(String::from(
                constants_str::TEST_PATH_TRAVERSAL,
            )),
            Err(crate::multipart_value_error::MultipartValueError::PathComponent)
        );
    }
}
