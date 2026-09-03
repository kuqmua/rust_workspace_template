#[cfg(test)]
mod tests {
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
