pub async fn check_body_size<BodyTy, LimitTy>(
    body: BodyTy,
    limit: LimitTy,
) -> Result<super::BytesBodyBytes, super::BodySizeError>
where
    BodyTy: Into<super::AxumBody>,
    LimitTy: Into<super::BodySizeLimitBytes>,
{
    let body_value = body.into();
    let limit_value = limit.into();
    let size_hint = axum::body::HttpBody::size_hint(&body_value.0);
    axum::body::to_bytes(body_value.0, limit_value.0)
        .await
        .map(super::BytesBodyBytes::from)
        .map_err(|error| super::BodySizeError::ReachedMaximumSizeOfBody {
            error: super::AxumBodySizeError::from(error),
            maximum_size_of_body_limit_in_bytes: limit_value,
            size_hint: super::HttpBodySizeHint::from(size_hint),
            location: location_macros::location!(),
        })
}
