#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, PartialEq, Eq)]
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
impl super::AdminError {
    #[track_caller]
    fn observed<Source>(
        source: Source,
        code: AdminObservedErrorCode,
    ) -> server_runtime_http::domain_types::ObservedError<Source>
    where
        Source: std::error::Error + 'static,
    {
        let value = match code {
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
        server_runtime_http::domain_types::ObservedError::capture(
            source,
            server_runtime_http::domain_types::ObservedErrorCode::from(value),
        )
    }

    const fn route_error_status(&self) -> frontend_contract::domain_types::RouteErrorStatus {
        match self {
            Self::Authentication | Self::AuthenticationSecretText(_) => {
                frontend_contract::domain_types::RouteErrorStatus::Authentication
            }
            Self::Authorization | Self::Csrf | Self::CsrfSecretText(_) => {
                frontend_contract::domain_types::RouteErrorStatus::Authorization
            }
            Self::Conflict => frontend_contract::domain_types::RouteErrorStatus::Conflict,
            Self::MethodNotAllowed => {
                frontend_contract::domain_types::RouteErrorStatus::MethodNotAllowed
            }
            Self::PayloadTooLarge => {
                frontend_contract::domain_types::RouteErrorStatus::PayloadTooLarge
            }
            Self::RateLimited => frontend_contract::domain_types::RouteErrorStatus::RateLimited,
            Self::Validation | Self::PasswordText(_) | Self::SecretText(_) => {
                frontend_contract::domain_types::RouteErrorStatus::Validation
            }
            Self::Pg(_) | Self::PasswordHash(_) | Self::Session(_) | Self::Header(_) => {
                frontend_contract::domain_types::RouteErrorStatus::Internal
            }
        }
    }

    #[track_caller]
    pub(super) fn authentication_secret_text(source: super::super::AdminSecretTextError) -> Self {
        Self::AuthenticationSecretText(Self::observed(
            source,
            AdminObservedErrorCode::AuthenticationSecretText,
        ))
    }

    #[track_caller]
    pub(super) fn csrf_secret_text(source: super::super::AdminSecretTextError) -> Self {
        Self::CsrfSecretText(Self::observed(
            source,
            AdminObservedErrorCode::CsrfSecretText,
        ))
    }

    #[track_caller]
    pub(super) fn header(source: super::HttpAdminHeaderValueError) -> Self {
        Self::Header(Self::observed(source, AdminObservedErrorCode::Header))
    }

    #[track_caller]
    pub(super) fn password_hash(source: super::super::AdminPasswordHashError) -> Self {
        Self::PasswordHash(Self::observed(source, AdminObservedErrorCode::PasswordHash))
    }

    #[track_caller]
    pub(super) fn password_text(source: super::super::AdminPasswordTryFromStringError) -> Self {
        Self::PasswordText(Self::observed(source, AdminObservedErrorCode::PasswordText))
    }

    #[track_caller]
    pub(super) fn postgresql(source: super::super::SqlxAdminError) -> Self {
        Self::Pg(Self::observed(source, AdminObservedErrorCode::Database))
    }

    #[track_caller]
    pub(super) fn session(source: super::AdminSessionError) -> Self {
        Self::Session(Self::observed(source, AdminObservedErrorCode::Session))
    }

    #[track_caller]
    pub(super) fn secret_text(source: super::super::AdminSecretTextError) -> Self {
        Self::SecretText(Self::observed(source, AdminObservedErrorCode::SecretText))
    }
}
impl From<sqlx::Error> for super::AdminError {
    fn from(value: sqlx::Error) -> Self {
        Self::postgresql(super::super::SqlxAdminError::from(value))
    }
}
impl From<super::super::SqlxAdminError> for super::AdminError {
    fn from(value: super::super::SqlxAdminError) -> Self {
        Self::postgresql(value)
    }
}
impl axum::response::IntoResponse for super::AdminError {
    fn into_response(self) -> axum::response::Response {
        let route_error_status = self.route_error_status();
        let error_type = server_runtime_http::domain_types::HttpErrorType::from(
            constants_str::ADMIN_API_ERROR_TYPE,
        );
        let optional_diagnostic = match &self {
            Self::Pg(source) => Some(
                server_runtime_http::domain_types::HttpErrorDiagnostic::from_observed(
                    error_type, source,
                ),
            ),
            Self::PasswordHash(source) => Some(
                server_runtime_http::domain_types::HttpErrorDiagnostic::from_observed(
                    error_type, source,
                ),
            ),
            Self::Session(source) => Some(
                server_runtime_http::domain_types::HttpErrorDiagnostic::from_observed(
                    error_type, source,
                ),
            ),
            Self::Header(source) => Some(
                server_runtime_http::domain_types::HttpErrorDiagnostic::from_observed(
                    error_type, source,
                ),
            ),
            Self::AuthenticationSecretText(source)
            | Self::CsrfSecretText(source)
            | Self::SecretText(source) => Some(
                server_runtime_http::domain_types::HttpErrorDiagnostic::from_observed(
                    error_type, source,
                ),
            ),
            Self::PasswordText(source) => Some(
                server_runtime_http::domain_types::HttpErrorDiagnostic::from_observed(
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
        super::admin_error_response_parts(route_error_status, optional_diagnostic)
    }
}
impl axum::response::IntoResponse for super::AxumAdminResponse {
    fn into_response(self) -> axum::response::Response {
        self.0
    }
}
