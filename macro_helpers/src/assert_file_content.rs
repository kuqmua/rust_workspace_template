pub(crate) fn assert_file_content(
    path: super::StdAssertFilePath<'_>,
    exp: super::ExpectedFileContent<'_>,
) {
    let cnt = server_runtime_http::domain_types::read_bounded_file(
        server_runtime_http::domain_types::PathRef::from(path.0),
        server_runtime_http::domain_types::BoundedReadMaximumBytes::from(exp.0.len()),
    )
    .and_then(server_runtime_http::domain_types::BoundedText::try_from)
    .expect("d5ec6712 assert_file_content invariant must hold");
    assert_eq!(cnt.as_ref(), exp.0);
}
