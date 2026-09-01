pub(crate) fn assert_file_content(
    path: crate::std_assert_file_path::StdAssertFilePath<'_>,
    exp: crate::expected_file_content::ExpectedFileContent<'_>,
) {
    let cnt = server_runtime_http::read_bounded_file::read_bounded_file(
        server_runtime_http::runtime_path_ref::RuntimePathRef::from(path.as_ref()),
        server_runtime_http::bounded_read_maximum_bytes::BoundedReadMaximumBytes::from(
            exp.as_ref().len(),
        ),
    )
    .and_then(server_runtime_http::bounded_text::BoundedText::try_from)
    .expect(constants_str::DIAGNOSTIC_D5EC6712);
    assert_eq!(cnt.as_ref(), exp.as_ref());
}
