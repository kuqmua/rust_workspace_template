pub async fn read_bounded_json_http_response(
    response: crate::reqwest_response::ReqwestResponse,
    maximum_bytes: crate::bounded_read_maximum_bytes::BoundedReadMaximumBytes,
    concurrency: crate::bounded_read_concurrency_arc_semaphore::BoundedReadConcurrencyArcSemaphore,
) -> Result<
    crate::bounded_json_text::BoundedJsonText,
    crate::bounded_json_read_error::BoundedJsonReadError,
> {
    let bytes = crate::read_bounded_http_response::read_bounded_http_response(
        response,
        maximum_bytes,
        concurrency,
    )
    .await
    .map_err(crate::bounded_json_read_error::BoundedJsonReadError::Read)?;
    crate::parse_bounded_json_owned::parse_bounded_json_owned(bytes)
}
