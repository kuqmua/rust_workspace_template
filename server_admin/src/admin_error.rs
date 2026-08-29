#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
pub(crate) enum AdminError {
    #[error("administrator authentication failed")]
    Authentication,
    #[error("administrator authentication secret text is invalid")]
    AuthenticationSecretText(
        #[source]
        server_observability::observed_error::ObservedError<
            crate::admin_secret_text_error::AdminSecretTextError,
        >,
    ),
    #[error("administrator authorization failed")]
    Authorization,
    #[error("administrator operation conflicts with current state")]
    Conflict,
    #[error("administrator request failed CSRF validation")]
    Csrf,
    #[error("administrator CSRF secret text is invalid")]
    CsrfSecretText(
        #[source]
        server_observability::observed_error::ObservedError<
            crate::admin_secret_text_error::AdminSecretTextError,
        >,
    ),
    #[error("administrator authentication is temporarily rate limited")]
    RateLimited,
    #[error("administrator request validation failed")]
    Validation,
    #[error("administrator API database operation failed: {0:?}")]
    Pg(
        #[source]
        server_observability::observed_error::ObservedError<
            crate::sqlx_admin_error::SqlxAdminError,
        >,
    ),
    #[error("administrator password hashing failed: {0}")]
    PasswordHash(
        #[source]
        server_observability::observed_error::ObservedError<
            crate::admin_password_hash_error::AdminPasswordHashError,
        >,
    ),
    #[error("administrator password text is invalid")]
    PasswordText(
        #[source]
        server_observability::observed_error::ObservedError<
            crate::admin_password_try_from_string_error::AdminPasswordTryFromStringError,
        >,
    ),
    #[error("administrator request body is too large")]
    PayloadTooLarge,
    #[error("administrator secret text is invalid")]
    SecretText(
        #[source]
        server_observability::observed_error::ObservedError<
            crate::admin_secret_text_error::AdminSecretTextError,
        >,
    ),
    #[error("administrator route does not support this HTTP method")]
    MethodNotAllowed,
    #[error("administrator session operation failed: {0}")]
    Session(
        #[source]
        server_observability::observed_error::ObservedError<
            crate::admin_session_error::AdminSessionError,
        >,
    ),
    #[error("administrator response header is invalid: {0:?}")]
    Header(
        #[source]
        server_observability::observed_error::ObservedError<
            crate::http_admin_header_value_error::HttpAdminHeaderValueError,
        >,
    ),
}
