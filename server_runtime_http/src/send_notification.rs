// The owner module retains lint-sensitive semantics from the original implementation.

#[allow(
    clippy::single_call_fn,
    reason = "Axum registers this generic route handler indirectly"
)]
pub(super) async fn send_notification<Sender>(
    state: crate::axum_notification_state::AxumNotificationState<Sender>,
    request: crate::axum_notification_json::AxumNotificationJson,
) -> http::StatusCode
where
    Sender: crate::notification_sender::NotificationSender,
{
    let authorization = state
        .headers()
        .get()
        .get(http::header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok());
    let authorized = match crate::resolve_bearer_authorization::resolve_bearer_authorization(
        crate::http_authorization_header_text_ref::HttpAuthorizationHeaderTextRef::from(
            authorization,
        ),
    ) {
        crate::bearer_authorization_resolution::BearerAuthorizationResolution::Resolved(token) => {
            bool::from(state.state().token().authorizes(
                crate::notification_api_token_ref::NotificationApiTokenRef::from(token.as_ref()),
            ))
        }
        crate::bearer_authorization_resolution::BearerAuthorizationResolution::Invalid
        | crate::bearer_authorization_resolution::BearerAuthorizationResolution::Missing => false,
    };
    if !authorized {
        return http::StatusCode::UNAUTHORIZED;
    }
    let Some(_permit) = state.state().permits().try_acquire() else {
        return http::StatusCode::TOO_MANY_REQUESTS;
    };
    match state
        .state()
        .sender()
        .send(
            crate::runtime_notification_message::RuntimeNotificationMessage::from(
                request.into_inner(),
            ),
        )
        .await
    {
        Ok(()) => http::StatusCode::NO_CONTENT,
        Err(_error) => http::StatusCode::BAD_GATEWAY,
    }
}
