pub(crate) fn assert_file_content(
    std_assert_file_path: crate::std_assert_file_path::StdAssertFilePath<'_>,
    expected_file_content: crate::expected_file_content::ExpectedFileContent<'_>,
) {
    let count = server_runtime_http::read_bounded_file::read_bounded_file(
        server_runtime_http::runtime_path_ref::RuntimePathRef::from(std_assert_file_path.as_ref()),
        server_runtime_http::bounded_read_maximum_bytes::BoundedReadMaximumBytes::from(
            expected_file_content.as_ref().len(),
        ),
    )
    .and_then(server_runtime_http::bounded_text::BoundedText::try_from)
    .expect(constants_str::DIAGNOSTIC_D5EC6712);
    assert_eq!(count.as_ref(), expected_file_content.as_ref());
}
