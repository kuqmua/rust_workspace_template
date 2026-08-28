pub async fn read_bounded_json_http_response(
    response: super::ReqwestResponse,
    maximum_bytes: super::BoundedReadMaximumBytes,
    concurrency: super::BoundedReadConcurrencyArcSemaphore,
) -> Result<super::BoundedJsonText, super::BoundedJsonReadError> {
    let bytes = super::read_bounded_http_response(response, maximum_bytes, concurrency)
        .await
        .map_err(super::BoundedJsonReadError::Read)?;
    super::parse_bounded_json_owned(bytes)
}
