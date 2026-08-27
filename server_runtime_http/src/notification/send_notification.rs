#[allow(clippy::single_call_fn)]
pub(super) async fn send_notification<Sender>(
    state: super::AxumNotificationState<Sender>,
    request: super::AxumNotificationJson,
) -> http::StatusCode
where
    Sender: super::NotificationSender,
{
    let authorization = state
        .headers
        .0
        .get(http::header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok());
    let authorized = match crate::domain_types::resolve_bearer_authorization(
        crate::domain_types::HttpAuthorizationHeaderTextRef::from(authorization),
    ) {
        crate::domain_types::BearerAuthorizationResolution::Resolved(token) => bool::from(
            state
                .state
                .token
                .authorizes(super::NotificationApiTokenRef::from(token.as_ref())),
        ),
        crate::domain_types::BearerAuthorizationResolution::Invalid
        | crate::domain_types::BearerAuthorizationResolution::Missing => false,
    };
    if !authorized {
        return http::StatusCode::UNAUTHORIZED;
    }
    let Some(_permit) = state.state.permits.try_acquire() else {
        return http::StatusCode::TOO_MANY_REQUESTS;
    };
    match state.state.sender.send(request.0.message).await {
        Ok(()) => http::StatusCode::NO_CONTENT,
        Err(_error) => http::StatusCode::BAD_GATEWAY,
    }
}
