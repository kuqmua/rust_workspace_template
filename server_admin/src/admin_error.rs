#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
pub(crate) enum AdminError {
    #[error("{message}", message = constants_str::ADMIN_DIAGNOSTIC_ADMINISTRATOR_AUTHENTICATION_FAILED)]
    Authentication,
    #[error("{message}", message = constants_str::ADMIN_DIAGNOSTIC_ADMINISTRATOR_AUTHENTICATION_SECRET_TEXT_IS_INVALID)]
    AuthenticationSecretText(
        #[source]
        server_observability::observed_error::ObservedError<
            crate::admin_secret_text_error::AdminSecretTextError,
        >,
    ),
    #[error("{message}", message = constants_str::ADMIN_DIAGNOSTIC_ADMINISTRATOR_AUTHORIZATION_FAILED)]
    Authorization,
    #[error("{message}", message = constants_str::ADMIN_DIAGNOSTIC_ADMINISTRATOR_OPERATION_CONFLICTS_WITH_CURRENT_STATE)]
    Conflict,
    #[error("{message}", message = constants_str::ADMIN_DIAGNOSTIC_ADMINISTRATOR_REQUEST_FAILED_CSRF_VALIDATION)]
    Csrf,
    #[error("{message}", message = constants_str::ADMIN_DIAGNOSTIC_ADMINISTRATOR_CSRF_SECRET_TEXT_IS_INVALID)]
    CsrfSecretText(
        #[source]
        server_observability::observed_error::ObservedError<
            crate::admin_secret_text_error::AdminSecretTextError,
        >,
    ),
    #[error("{message}", message = constants_str::ADMIN_DIAGNOSTIC_ADMINISTRATOR_AUTHENTICATION_IS_TEMPORARILY_RATE_LIMITED)]
    RateLimited,
    #[error("{message}", message = constants_str::ADMIN_DIAGNOSTIC_ADMINISTRATOR_REQUEST_VALIDATION_FAILED)]
    Validation,
    #[error("{message}: {0:?}", message = constants_str::ADMIN_DIAGNOSTIC_ADMINISTRATOR_API_DATABASE_OPERATION_FAILED)]
    Pg(
        #[source]
        server_observability::observed_error::ObservedError<
            crate::sqlx_admin_error::SqlxAdminError,
        >,
    ),
    #[error("{message}: {0}", message = constants_str::ADMIN_DIAGNOSTIC_ADMINISTRATOR_PASSWORD_HASHING_FAILED)]
    PasswordHash(
        #[source]
        server_observability::observed_error::ObservedError<
            crate::admin_password_hash_error::AdminPasswordHashError,
        >,
    ),
    #[error("{message}", message = constants_str::ADMIN_DIAGNOSTIC_ADMINISTRATOR_PASSWORD_TEXT_IS_INVALID)]
    PasswordText(
        #[source]
        server_observability::observed_error::ObservedError<
            crate::admin_password_try_from_string_error::AdminPasswordTryFromStringError,
        >,
    ),
    #[error("{message}", message = constants_str::ADMIN_DIAGNOSTIC_ADMINISTRATOR_REQUEST_BODY_IS_TOO_LARGE)]
    PayloadTooLarge,
    #[error("{message}", message = constants_str::ADMIN_DIAGNOSTIC_ADMINISTRATOR_SECRET_TEXT_IS_INVALID)]
    SecretText(
        #[source]
        server_observability::observed_error::ObservedError<
            crate::admin_secret_text_error::AdminSecretTextError,
        >,
    ),
    #[error("{message}", message = constants_str::ADMIN_DIAGNOSTIC_ADMINISTRATOR_ROUTE_DOES_NOT_SUPPORT_THIS_HTTP_METHOD)]
    MethodNotAllowed,
    #[error("{message}: {0}", message = constants_str::ADMIN_DIAGNOSTIC_ADMINISTRATOR_SESSION_OPERATION_FAILED)]
    Session(
        #[source]
        server_observability::observed_error::ObservedError<
            crate::admin_session_error::AdminSessionError,
        >,
    ),
    #[error("{message}: {0:?}", message = constants_str::ADMIN_DIAGNOSTIC_ADMINISTRATOR_RESPONSE_HEADER_IS_INVALID)]
    Header(
        #[source]
        server_observability::observed_error::ObservedError<
            crate::http_admin_header_value_error::HttpAdminHeaderValueError,
        >,
    ),
}
