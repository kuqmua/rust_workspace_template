#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, PartialEq, Eq,
)]
enum AdminObservedErrorCode {
    AuthenticationSecretText,
    CsrfSecretText,
    Collection,
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
            AdminObservedErrorCode::Collection => constants_str::ADMIN_OBSERVED_ERROR_COLLECTION,
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
            Self::Validation
            | Self::ValidationCollection(_)
            | Self::PasswordText(_)
            | Self::SecretText(_) => {
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
    pub(crate) fn validation_collection(
        admin_collection_error: server_admin_contract::admin_collection_error::AdminCollectionError,
    ) -> Self {
        Self::ValidationCollection(Self::observed(
            admin_collection_error,
            AdminObservedErrorCode::Collection,
        ))
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
            Self::ValidationCollection(source) => Some(
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

#[cfg(test)]
mod tests {
    #[test]
    fn test_admin_observed_error_codes_preserve_source_kind() {
        let cases = [
            (
                super::AdminObservedErrorCode::AuthenticationSecretText,
                constants_str::ADMIN_OBSERVED_ERROR_AUTH_SECRET_TEXT,
            ),
            (
                super::AdminObservedErrorCode::CsrfSecretText,
                constants_str::ADMIN_OBSERVED_ERROR_CSRF_SECRET_TEXT,
            ),
            (
                super::AdminObservedErrorCode::Collection,
                constants_str::ADMIN_OBSERVED_ERROR_COLLECTION,
            ),
            (
                super::AdminObservedErrorCode::Database,
                constants_str::ADMIN_OBSERVED_ERROR_DATABASE,
            ),
            (
                super::AdminObservedErrorCode::Header,
                constants_str::ADMIN_OBSERVED_ERROR_RESPONSE_HEADER,
            ),
            (
                super::AdminObservedErrorCode::PasswordHash,
                constants_str::ADMIN_OBSERVED_ERROR_PASSWORD_HASH,
            ),
            (
                super::AdminObservedErrorCode::PasswordText,
                constants_str::ADMIN_OBSERVED_ERROR_PASSWORD_TEXT,
            ),
            (
                super::AdminObservedErrorCode::SecretText,
                constants_str::ADMIN_OBSERVED_ERROR_SECRET_TEXT,
            ),
            (
                super::AdminObservedErrorCode::Session,
                constants_str::ADMIN_OBSERVED_ERROR_SESSION,
            ),
        ];
        assert!(cases.into_iter().all(|(code, expected)| {
            let source = std::io::Error::from(std::io::ErrorKind::InvalidData);
            let observed = crate::admin_error::AdminError::observed(source, code);
            observed.error_code()
                == server_observability::observed_error_code::ObservedErrorCode::from(expected)
                && observed.source_ref().kind() == std::io::ErrorKind::InvalidData
        }));
    }
    #[test]
    fn test_admin_plain_errors_map_to_route_statuses() {
        let cases = [
            (
                crate::admin_error::AdminError::Authentication,
                frontend_contract::route_error_status::RouteErrorStatus::Authentication,
            ),
            (
                crate::admin_error::AdminError::Authorization,
                frontend_contract::route_error_status::RouteErrorStatus::Authorization,
            ),
            (
                crate::admin_error::AdminError::Csrf,
                frontend_contract::route_error_status::RouteErrorStatus::Authorization,
            ),
            (
                crate::admin_error::AdminError::Conflict,
                frontend_contract::route_error_status::RouteErrorStatus::Conflict,
            ),
            (
                crate::admin_error::AdminError::MethodNotAllowed,
                frontend_contract::route_error_status::RouteErrorStatus::MethodNotAllowed,
            ),
            (
                crate::admin_error::AdminError::PayloadTooLarge,
                frontend_contract::route_error_status::RouteErrorStatus::PayloadTooLarge,
            ),
            (
                crate::admin_error::AdminError::RateLimited,
                frontend_contract::route_error_status::RouteErrorStatus::RateLimited,
            ),
            (
                crate::admin_error::AdminError::Validation,
                frontend_contract::route_error_status::RouteErrorStatus::Validation,
            ),
        ];
        assert!(
            cases
                .into_iter()
                .all(|(error, expected)| error.route_error_status() == expected)
        );
    }

    #[test]
    fn test_admin_secret_error_status_depends_on_operation() {
        let source = crate::admin_secret_text_error::AdminSecretTextError::ContainsNul;
        let cases = [
            (
                crate::admin_error::AdminError::authentication_secret_text(source),
                frontend_contract::route_error_status::RouteErrorStatus::Authentication,
                constants_str::ADMIN_OBSERVED_ERROR_AUTH_SECRET_TEXT,
            ),
            (
                crate::admin_error::AdminError::csrf_secret_text(source),
                frontend_contract::route_error_status::RouteErrorStatus::Authorization,
                constants_str::ADMIN_OBSERVED_ERROR_CSRF_SECRET_TEXT,
            ),
            (
                crate::admin_error::AdminError::secret_text(source),
                frontend_contract::route_error_status::RouteErrorStatus::Validation,
                constants_str::ADMIN_OBSERVED_ERROR_SECRET_TEXT,
            ),
        ];
        assert!(cases.into_iter().all(|(error, status, code)| {
            if error.route_error_status() != status {
                return false;
            }
            match error {
                crate::admin_error::AdminError::AuthenticationSecretText(observed)
                | crate::admin_error::AdminError::CsrfSecretText(observed)
                | crate::admin_error::AdminError::SecretText(observed) => {
                    observed.error_code()
                        == server_observability::observed_error_code::ObservedErrorCode::from(code)
                        && *observed.source_ref() == source
                }
                crate::admin_error::AdminError::Authentication
                | crate::admin_error::AdminError::Authorization
                | crate::admin_error::AdminError::Conflict
                | crate::admin_error::AdminError::Csrf
                | crate::admin_error::AdminError::RateLimited
                | crate::admin_error::AdminError::Validation
                | crate::admin_error::AdminError::ValidationCollection(_)
                | crate::admin_error::AdminError::Pg(_)
                | crate::admin_error::AdminError::PasswordHash(_)
                | crate::admin_error::AdminError::PasswordText(_)
                | crate::admin_error::AdminError::PayloadTooLarge
                | crate::admin_error::AdminError::MethodNotAllowed
                | crate::admin_error::AdminError::Session(_)
                | crate::admin_error::AdminError::Header(_) => false,
            }
        }));
    }

    #[test]
    fn test_admin_body_rejection_distinguishes_payload_limit() {
        assert!(matches!(
            crate::admin_error::AdminError::body_rejection(true.into()),
            crate::admin_error::AdminError::PayloadTooLarge
        ));
        assert!(matches!(
            crate::admin_error::AdminError::body_rejection(false.into()),
            crate::admin_error::AdminError::Validation
        ));
    }
    #[test]
    fn test_admin_infrastructure_errors_map_to_internal_status() {
        let errors = [
            crate::admin_error::AdminError::from(sqlx::Error::RowNotFound),
            crate::admin_error::AdminError::session(
                crate::admin_session_error::AdminSessionError::SystemClock,
            ),
            crate::admin_error::AdminError::password_hash(
                crate::admin_password_hash_error::AdminPasswordHashError::BoundedText(
                    bounded_types::bounded_string_error::BoundedStringError::AboveMaximum {
                        actual_length: 2usize.into(),
                        maximum_length: 1usize.into(),
                    },
                ),
            ),
        ];
        assert!(errors.into_iter().all(|error| error.route_error_status()
            == frontend_contract::route_error_status::RouteErrorStatus::Internal));
        assert!(
            http::HeaderValue::from_bytes(constants_str::NEWLINE.as_bytes()).is_err_and(|source| {
                crate::admin_error::AdminError::header(
                    crate::http_admin_header_value_error::HttpAdminHeaderValueError::from(source),
                )
                .route_error_status()
                    == frontend_contract::route_error_status::RouteErrorStatus::Internal
            })
        );
    }
    #[test]
    fn test_admin_typed_validation_errors_map_to_validation_status() {
        let errors = [
            crate::admin_error::AdminError::password_text(crate::admin_password_try_from_string_error::AdminPasswordTryFromStringError::InvalidLength),
            crate::admin_error::AdminError::validation_collection(server_admin_contract::admin_collection_error::AdminCollectionError::TooLong(bounded_types::bounded_value_error::BoundedValueError::AboveMax { actual: 2usize.into(), max: 1usize.into() })),
        ];
        assert!(errors.into_iter().all(|error| error.route_error_status()
            == frontend_contract::route_error_status::RouteErrorStatus::Validation));
    }
}
