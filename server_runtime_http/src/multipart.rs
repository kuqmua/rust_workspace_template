#[cfg(test)]
mod tests {
    fn field_name() -> crate::multipart_field_name::MultipartFieldName {
        crate::multipart_field_name::MultipartFieldName::try_from(String::from(
            constants_str::FIELD,
        ))
        .expect("0f4b54a3 field_name invariant must hold")
    }
    fn text_part(value: &str) -> crate::multipart_text_part::MultipartTextPart {
        crate::multipart_text_part::MultipartTextPart::new(
            field_name(),
            crate::multipart_text_value::MultipartTextValue::try_from(value.to_owned())
                .expect("93b34391 text_part invariant must hold"),
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
        .expect("1d3de882 multipart_value_wrappers_enforce_each_boundary invariant must hold");
        assert_eq!(
            crate::multipart_field_name::MultipartFieldName::try_from("a".repeat(257usize)),
            Err(crate::multipart_value_error::MultipartValueError::TooLong {
                actual: crate::multipart_value_length::MultipartValueLength::from(257usize)
            })
        );
        assert_eq!(
            crate::multipart_field_name::MultipartFieldName::try_from(String::from("a\0b")),
            Err(crate::multipart_value_error::MultipartValueError::ControlCharacter)
        );

        assert_eq!(
            crate::multipart_file_name::MultipartFileName::try_from(String::new()),
            Err(crate::multipart_value_error::MultipartValueError::EmptyFileName)
        );
        let _file_name = crate::multipart_file_name::MultipartFileName::try_from(
            constants_str::A_ALT.repeat(1024usize),
        )
        .expect("7b3ca38e multipart_value_wrappers_enforce_each_boundary invariant must hold");
        assert_eq!(
            crate::multipart_file_name::MultipartFileName::try_from("a".repeat(1025usize)),
            Err(crate::multipart_value_error::MultipartValueError::TooLong {
                actual: crate::multipart_value_length::MultipartValueLength::from(1025usize)
            })
        );
        assert_eq!(
            crate::multipart_file_name::MultipartFileName::try_from(String::from("a\0b")),
            Err(crate::multipart_value_error::MultipartValueError::ControlCharacter)
        );
        assert_eq!(
            crate::multipart_field_name::MultipartFieldName::try_from(String::from(
                "field\r\ninjected"
            )),
            Err(crate::multipart_value_error::MultipartValueError::ControlCharacter)
        );
        assert_eq!(
            crate::multipart_file_name::MultipartFileName::try_from(String::from("..\\secret.txt")),
            Err(crate::multipart_value_error::MultipartValueError::PathComponent)
        );

        let _text = crate::multipart_text_value::MultipartTextValue::try_from(
            constants_str::A_ALT.repeat(65_536usize),
        )
        .expect("c2dd1657 multipart_value_wrappers_enforce_each_boundary invariant must hold");
        assert_eq!(
            crate::multipart_text_value::MultipartTextValue::try_from("a".repeat(65_537usize)),
            Err(crate::multipart_value_error::MultipartValueError::TooLong {
                actual: crate::multipart_value_length::MultipartValueLength::from(65_537usize)
            })
        );
        assert_eq!(
            crate::multipart_text_value::MultipartTextValue::try_from(String::from("\0")),
            Err(crate::multipart_value_error::MultipartValueError::Nul)
        );
    }
    #[test]
    fn test_multipart_parts_preserve_names_values_and_file_names() {
        let text = text_part(constants_str::VALUE_CD42404D);
        assert_eq!(text.name().as_ref(), "field");
        assert_eq!(text.value().as_ref(), "value");

        let file_name = crate::multipart_file_name::MultipartFileName::try_from(String::from(
            constants_str::VALUE_EAFB4AFF,
        ))
        .expect(
            "b76ab3ce multipart_parts_preserve_names_values_and_file_names invariant must hold",
        );
        let bytes = crate::multipart_bytes::MultipartBytes::try_from(vec![1u8, 2u8, 3u8]).expect(
            "e9e23985 multipart_parts_preserve_names_values_and_file_names invariant must hold",
        );
        let bytes_part = crate::multipart_bytes_part::MultipartBytesPart::new(field_name(), bytes)
            .with_file_name(file_name);
        assert_eq!(bytes_part.name().as_ref(), "field");
        assert_eq!(bytes_part.bytes().as_ref(), &[1u8, 2u8, 3u8]);
        assert_eq!(
            bytes_part.file_name().map(AsRef::as_ref),
            Some("report.txt")
        );
    }
    #[test]
    fn test_request_enforces_combined_payload_and_part_count() {
        let limited_request = crate::multipart_upload_request::MultipartUploadRequest::new()
            .with_text_part(
                text_part(constants_str::AB),
                crate::multipart_payload_maximum::MultipartPayloadMaximum::from(3usize),
            )
            .expect(
                "7797e0f1 request_enforces_combined_payload_and_part_count invariant must hold",
            );
        assert_eq!(
            limited_request.with_text_part(
                text_part("cd"),
                crate::multipart_payload_maximum::MultipartPayloadMaximum::from(3usize)
            ),
            Err(crate::multipart_request_error::MultipartRequestError::PayloadTooLarge)
        );

        let full_request = (constants_usize::ZERO..32usize)
            .try_fold(
                crate::multipart_upload_request::MultipartUploadRequest::new(),
                |accumulator, _idx| {
                    accumulator.with_text_part(
                        text_part(constants_str::PG_CRUD_EMPTY_SQL_SUFFIX),
                        crate::multipart_payload_maximum::MultipartPayloadMaximum::from(
                            constants_usize::ZERO,
                        ),
                    )
                },
            )
            .expect(
                "9cbea721 request_enforces_combined_payload_and_part_count invariant must hold",
            );
        assert_eq!(full_request.text_parts().len(), 32usize);
        assert_eq!(
            full_request.with_text_part(
                text_part(""),
                crate::multipart_payload_maximum::MultipartPayloadMaximum::from(
                    constants_usize::ZERO
                )
            ),
            Err(crate::multipart_request_error::MultipartRequestError::TooManyParts)
        );
    }
    #[test]
    fn test_storage_paths_validate_segments_and_preserve_file_extensions() {
        let _valid =
            crate::storage_path_segment::StoragePathSegment::try_from(String::from(constants_str::VALUE_A31BB256)).expect("20b6c6b2 storage_paths_validate_segments_and_preserve_file_extensions invariant must hold");
        assert_eq!(
            crate::storage_path_segment::StoragePathSegment::try_from(String::new()),
            Err(crate::storage_path_segment_error::StoragePathSegmentError::Invalid)
        );
        assert_eq!(
            crate::storage_path_segment::StoragePathSegment::try_from(String::from("../escape")),
            Err(crate::storage_path_segment_error::StoragePathSegmentError::Invalid)
        );
        assert_eq!(
            crate::storage_path_segment::StoragePathSegment::try_from("a".repeat(1025usize)),
            Err(crate::storage_path_segment_error::StoragePathSegmentError::Invalid)
        );

        let identifier =
            crate::storage_path_segment::StoragePathSegment::try_from(String::from(constants_str::VALUE_BCA3685F)).expect("ec2aa921 storage_paths_validate_segments_and_preserve_file_extensions invariant must hold");
        let unique = crate::storage_path_segment::StoragePathSegment::try_from(String::from(constants_str::VALUE_C2720445)).expect("51bb3e40 storage_paths_validate_segments_and_preserve_file_extensions invariant must hold");
        let file_name =
            crate::multipart_file_name::MultipartFileName::try_from(String::from(constants_str::VALUE_4A1282F3)).expect("3ea5274e storage_paths_validate_segments_and_preserve_file_extensions invariant must hold");
        assert_eq!(
            crate::identifier_file_storage_relative_path::identifier_file_storage_relative_path(
                &identifier,
                &unique,
                &file_name
            )
            .as_ref(),
            std::path::Path::new("entity/unique.gz")
        );
        let no_extension =
            crate::multipart_file_name::MultipartFileName::try_from(String::from(constants_str::VALUE_2B7814D3)).expect("b7a900a5 storage_paths_validate_segments_and_preserve_file_extensions invariant must hold");
        assert_eq!(
            crate::identifier_file_storage_relative_path::identifier_file_storage_relative_path(
                &identifier,
                &unique,
                &no_extension
            )
            .as_ref(),
            std::path::Path::new("entity/unique")
        );
        assert_eq!(
            crate::staging_directory_name::staging_directory_name(crate::file_staging_action::FileStagingAction::Delete)
                .expect("c5076b2f storage_paths_validate_segments_and_preserve_file_extensions invariant must hold")
                .as_ref(),
            constants_str::FILE_DELETE_STAGING_DIRECTORY
        );
        assert_eq!(
            crate::staging_directory_name::staging_directory_name(crate::file_staging_action::FileStagingAction::Upload)
                .expect("725e03de storage_paths_validate_segments_and_preserve_file_extensions invariant must hold")
                .as_ref(),
            constants_str::FILE_UPLOAD_STAGING_DIRECTORY
        );
    }
    #[test]
    fn test_request_rejects_payload_above_limit() {
        let name = crate::multipart_field_name::MultipartFieldName::try_from(String::from(
            constants_str::TEST_MULTIPART_FILE_FIELD,
        ))
        .expect("3696f97d request_rejects_payload_above_limit invariant must hold");
        let bytes =
            crate::multipart_bytes::MultipartBytes::try_from(vec![constants_u8::ZERO; 2usize])
                .expect("24f930b8 request_rejects_payload_above_limit invariant must hold");
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

// Root-owned module compatibility wrappers.
mod file_staging_action {}
mod file_staging_directory_name {}
mod identifier_file_storage_relative_path {}
mod multipart_bytes {}
mod multipart_bytes_part {}
mod multipart_bytes_parts {}
mod multipart_field_name {}
mod multipart_file_name {}
mod multipart_payload_maximum {}
mod multipart_request_error {}
mod multipart_text_part {}
mod multipart_text_parts {}
mod multipart_text_value {}
mod multipart_upload_request {}
mod multipart_value_error {}
mod multipart_value_length {}
mod staging_directory_name {}
mod storage_path_segment {}
mod storage_path_segment_error {}
mod runtime_storage_relative_path_buf {}
