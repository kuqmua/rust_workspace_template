#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, PartialEq, Eq,
)]
enum AdminObservedErrorCode {
    AuthenticationSecretText,
    CsrfSecretText,
    Database,
    Header,
    PasswordHash,
    PasswordText,
    SecretText,
    Session,
}
impl crate::admin_error::AdminError {
    #[track_caller]
    fn observed<Source>(
        source: Source,
        admin_observed_error_code: AdminObservedErrorCode,
    ) -> server_observability::observed_error::ObservedError<Source>
    where
        Source: std::error::Error + 'static,
    {
        let value = match admin_observed_error_code {
            AdminObservedErrorCode::AuthenticationSecretText => {
                constants_str::ADMIN_OBSERVED_ERROR_AUTH_SECRET_TEXT
            }
            AdminObservedErrorCode::CsrfSecretText => {
                constants_str::ADMIN_OBSERVED_ERROR_CSRF_SECRET_TEXT
            }
            AdminObservedErrorCode::Database => constants_str::ADMIN_OBSERVED_ERROR_DATABASE,
            AdminObservedErrorCode::Header => constants_str::ADMIN_OBSERVED_ERROR_RESPONSE_HEADER,
            AdminObservedErrorCode::PasswordHash => {
                constants_str::ADMIN_OBSERVED_ERROR_PASSWORD_HASH
            }
            AdminObservedErrorCode::PasswordText => {
                constants_str::ADMIN_OBSERVED_ERROR_PASSWORD_TEXT
            }
            AdminObservedErrorCode::SecretText => constants_str::ADMIN_OBSERVED_ERROR_SECRET_TEXT,
            AdminObservedErrorCode::Session => constants_str::ADMIN_OBSERVED_ERROR_SESSION,
        };
        server_observability::observed_error::ObservedError::capture(
            source,
            server_observability::observed_error_code::ObservedErrorCode::from(value),
        )
    }

    const fn route_error_status(&self) -> frontend_contract::route_error_status::RouteErrorStatus {
        match self {
            Self::Authentication | Self::AuthenticationSecretText(_) => {
                frontend_contract::route_error_status::RouteErrorStatus::Authentication
            }
            Self::Authorization | Self::Csrf | Self::CsrfSecretText(_) => {
                frontend_contract::route_error_status::RouteErrorStatus::Authorization
            }
            Self::Conflict => frontend_contract::route_error_status::RouteErrorStatus::Conflict,
            Self::MethodNotAllowed => {
                frontend_contract::route_error_status::RouteErrorStatus::MethodNotAllowed
            }
            Self::PayloadTooLarge => {
                frontend_contract::route_error_status::RouteErrorStatus::PayloadTooLarge
            }
            Self::RateLimited => {
                frontend_contract::route_error_status::RouteErrorStatus::RateLimited
            }
            Self::Validation | Self::PasswordText(_) | Self::SecretText(_) => {
                frontend_contract::route_error_status::RouteErrorStatus::Validation
            }
            Self::Pg(_) | Self::PasswordHash(_) | Self::Session(_) | Self::Header(_) => {
                frontend_contract::route_error_status::RouteErrorStatus::Internal
            }
        }
    }

    #[track_caller]
    pub(crate) fn authentication_secret_text(
        admin_secret_text_error: crate::admin_secret_text_error::AdminSecretTextError,
    ) -> Self {
        Self::AuthenticationSecretText(Self::observed(
            admin_secret_text_error,
            AdminObservedErrorCode::AuthenticationSecretText,
        ))
    }

    pub(crate) const fn body_rejection(
        std_admin_bool: server_admin_core::std_admin_bool::StdAdminBool,
    ) -> Self {
        if std_admin_bool.get() {
            Self::PayloadTooLarge
        } else {
            Self::Validation
        }
    }

    #[track_caller]
    pub(crate) fn csrf_secret_text(
        admin_secret_text_error: crate::admin_secret_text_error::AdminSecretTextError,
    ) -> Self {
        Self::CsrfSecretText(Self::observed(
            admin_secret_text_error,
            AdminObservedErrorCode::CsrfSecretText,
        ))
    }

    #[track_caller]
    pub(crate) fn header(
        http_admin_header_value_error: crate::http_admin_header_value_error::HttpAdminHeaderValueError,
    ) -> Self {
        Self::Header(Self::observed(
            http_admin_header_value_error,
            AdminObservedErrorCode::Header,
        ))
    }

    #[track_caller]
    pub(crate) fn password_hash(
        admin_password_hash_error: crate::admin_password_hash_error::AdminPasswordHashError,
    ) -> Self {
        Self::PasswordHash(Self::observed(
            admin_password_hash_error,
            AdminObservedErrorCode::PasswordHash,
        ))
    }

    #[track_caller]
    pub(crate) fn password_text(
        admin_password_try_from_string_error: crate::admin_password_try_from_string_error::AdminPasswordTryFromStringError,
    ) -> Self {
        Self::PasswordText(Self::observed(
            admin_password_try_from_string_error,
            AdminObservedErrorCode::PasswordText,
        ))
    }

    #[track_caller]
    pub(crate) fn postgresql(sqlx_admin_error: crate::sqlx_admin_error::SqlxAdminError) -> Self {
        Self::Pg(Self::observed(
            sqlx_admin_error,
            AdminObservedErrorCode::Database,
        ))
    }

    #[track_caller]
    pub(crate) fn session(
        admin_session_error: crate::admin_session_error::AdminSessionError,
    ) -> Self {
        Self::Session(Self::observed(
            admin_session_error,
            AdminObservedErrorCode::Session,
        ))
    }

    #[track_caller]
    pub(crate) fn secret_text(
        admin_secret_text_error: crate::admin_secret_text_error::AdminSecretTextError,
    ) -> Self {
        Self::SecretText(Self::observed(
            admin_secret_text_error,
            AdminObservedErrorCode::SecretText,
        ))
    }
}
impl From<sqlx::Error> for crate::admin_error::AdminError {
    fn from(value: sqlx::Error) -> Self {
        Self::postgresql(crate::sqlx_admin_error::SqlxAdminError::from(value))
    }
}
impl From<crate::sqlx_admin_error::SqlxAdminError> for crate::admin_error::AdminError {
    fn from(value: crate::sqlx_admin_error::SqlxAdminError) -> Self {
        Self::postgresql(value)
    }
}
impl axum::response::IntoResponse for crate::admin_error::AdminError {
    fn into_response(self) -> axum::response::Response {
        let route_error_status = self.route_error_status();
        let error_type = server_runtime_http::http_error_type::HttpErrorType::from(
            constants_str::ADMIN_API_ERROR_TYPE,
        );
        let optional_diagnostic = match &self {
            Self::Pg(source) => Some(
                server_runtime_http::http_error_diagnostic::HttpErrorDiagnostic::from_observed(
                    error_type, source,
                ),
            ),
            Self::PasswordHash(source) => Some(
                server_runtime_http::http_error_diagnostic::HttpErrorDiagnostic::from_observed(
                    error_type, source,
                ),
            ),
            Self::Session(source) => Some(
                server_runtime_http::http_error_diagnostic::HttpErrorDiagnostic::from_observed(
                    error_type, source,
                ),
            ),
            Self::Header(source) => Some(
                server_runtime_http::http_error_diagnostic::HttpErrorDiagnostic::from_observed(
                    error_type, source,
                ),
            ),
            Self::AuthenticationSecretText(source)
            | Self::CsrfSecretText(source)
            | Self::SecretText(source) => Some(
                server_runtime_http::http_error_diagnostic::HttpErrorDiagnostic::from_observed(
                    error_type, source,
                ),
            ),
            Self::PasswordText(source) => Some(
                server_runtime_http::http_error_diagnostic::HttpErrorDiagnostic::from_observed(
                    error_type, source,
                ),
            ),
            Self::Authentication
            | Self::Authorization
            | Self::Conflict
            | Self::Csrf
            | Self::MethodNotAllowed
            | Self::PayloadTooLarge
            | Self::RateLimited
            | Self::Validation => None,
        };
        crate::admin_error_response_parts::admin_error_response_parts(
            route_error_status,
            optional_diagnostic,
        )
    }
}
impl axum::response::IntoResponse for crate::axum_admin_response::AxumAdminResponse {
    fn into_response(self) -> axum::response::Response {
        axum::response::Response::from(self)
    }
}
