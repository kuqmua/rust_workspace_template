pub use crate::axum_router::AxumRouter;
pub use crate::batched_cleanup::{
    CleanupBatchCount, CleanupBatchSize, CleanupBatchSizeError, CleanupCompletion,
    CleanupContinuation, CleanupReport, CleanupRows, run_batched_cleanup,
};
pub use crate::bounded_read::{
    BoundedBytes, BoundedJsonReadError, BoundedJsonText, BoundedReadConcurrencyArcSemaphore,
    BoundedReadConcurrencyMaximumNonZeroUsize, BoundedReadError, BoundedReadFromUtf8Error,
    BoundedReadIoError, BoundedReadMaximumBytes, BoundedText, IoErrorPresenceDisposition, PathRef,
    ReqwestError, ReqwestResponse, SerdeJsonError, classify_not_found_io_error, parse_bounded_json,
    read_bounded_file, read_bounded_file_async, read_bounded_http_response,
    read_bounded_json_file_async, read_bounded_json_http_response,
};
pub use crate::child_process::{
    ChildDiagnostic, ChildDiagnosticMaximumNonZeroUsize, ChildExitStatus, ChildProcessCompletion,
    ChildProcessError, ChildProcessId, ChildProcessIoError, ChildProcessReport,
    ChildProcessReports, ChildProcessSet, ChildProcessSetError, ChildProcessSetMaximumNonZeroUsize,
    ChildProcessSucceeded, ChildProcessSupervisor, TokioChildProcess, TokioChildProcessJoinError,
};
pub use crate::client_ip::{
    ClientAddrParseError, ClientSocketAddr, HttpHeaderMapRef, ParseIntError, ResolvedClientIpAddr,
    TrustedProxyRange, TrustedProxyRangeParseError, TrustedProxyRanges, TrustedProxyRangesError,
    TrustedProxyRangesParseError, TrustedProxyRangesTextRef, parse_trusted_proxy_ranges,
    resolve_client_ip, resolve_header_text,
};
pub use crate::cors::{
    HttpCorsAllowOriginHeaderValues, HttpCorsAllowOriginHeaderValuesError,
    HttpCorsAllowOriginTextRef, parse_cors_allow_origin,
};
pub use crate::csp::{
    HttpCspBuilder, HttpCspDirectiveName, HttpCspDirectiveValue, HttpCspMaximumBytesError,
    HttpCspTokenError,
};
pub use crate::domain_types_request_id::{
    HttpHeaderToStrError, RequestId, RequestIdTryFromHttpHeaderValueError,
    RequestIdTryFromStringError,
};
pub use crate::domain_types_service_runtime::{
    ServiceRuntimeIoError, TokioServiceRuntime, build_service_runtime,
    wait_for_service_shutdown_signal,
};
pub use crate::fallback::{
    FallbackResponseMode, HttpAcceptHeaderMaximumBytes, HttpFallbackApiPrefixRef,
    HttpFallbackMetricsPathRef, HttpFallbackRequestPathRef, HttpOptionalAcceptHeaderRef,
    resolve_fallback_response_mode,
};
pub use crate::geojson::{GeoJsonDocumentText, GeoJsonValidationError, SerdeJsonGeoJsonError};
pub use crate::header_text::{
    HttpHeaderName, HttpHeaderTextBytes, HttpHeaderTextMaximumBytes,
    HttpHeaderTextMaximumBytesError, HttpHeaderTextRef, HttpHeaderTextResolution,
};
pub use crate::health::{
    HealthComponentStatus, HealthProbeSucceeded, HealthProbeTimeoutDuration, HealthReadiness,
    HealthSnapshot, ServiceLivenessSnapshot, add_health_routes, run_health_probe,
};
pub use crate::http_client::{
    ReqwestClient, ReqwestClientBuildError, ReqwestClientPolicy, ReqwestConnectTimeoutDuration,
    ReqwestRequestTimeoutDuration, StdReqwestTimeoutError,
};
pub use crate::http_error_diagnostic::{
    HttpErrorCode, HttpErrorDiagnostic, HttpErrorTelemetry, HttpErrorType,
};
pub use crate::http_header_policy::{
    HttpAttachmentFileNameRef, HttpContentDisposition, HttpContentDispositionError,
    HttpContentLength, HttpContentLengthError, build_attachment_content_disposition,
};
pub use crate::http_policy::{
    BearerAuthorizationResolution, CookieResolution, HttpAuthorizationHeaderTextRef,
    HttpBearerTokenRef, HttpContentTypeTextRef, HttpCookieHeadersRef, HttpCookieNameRef,
    HttpCookieValueRef, OptionalJsonBodyPresence, OptionalJsonContentType,
    OptionalJsonContentTypeDecision, classify_optional_json_content_type,
    resolve_bearer_authorization, resolve_optional_json_content_type_decision,
    resolve_unique_cookie,
};
pub use crate::http_request_span_config::HttpRequestSpanConfig;
pub use crate::http_status_error::{HttpErrorClass, HttpErrorStatus, classify_http_error_status};
pub use crate::lifecycle::{
    BackgroundTask, BackgroundTaskOutcome, BackgroundTaskShutdownError, RequestTimeoutDuration,
    RunIntervalDuration, StdRequestTimeoutTryFromDurationError, StdRunIntervalTryFromDurationError,
    TokioAbortTask, TokioTaskJoinError, abort_and_wait_task, spawn_interval_task,
};
pub use crate::limits::{
    AcquirePermitError, ArcTokioSemaphore, PermitWaitTimeoutDuration, RetryAfterSecs,
    RetryAfterSecsTryFromU64Error, SemaphorePermitCountNonZeroUsize, TokioAcquireError,
    TokioOwnedSemaphorePermit, acquire_permit,
};
pub use crate::metrics_layer::{
    HttpMetricsLayer, HttpMetricsPathCacheMaximum, HttpMetricsPathCacheMaximumTryFromUsizeError,
    MetricsResponseBody, MetricsResponseBodyError,
};
pub use crate::multipart::{
    FileStagingAction, FileStagingDirectoryName, MultipartBytes, MultipartBytesPart,
    MultipartFieldName, MultipartFileName, MultipartPayloadMaximum, MultipartRequestError,
    MultipartTextPart, MultipartTextValue, MultipartUploadRequest, MultipartValueError,
    MultipartValueLength, StoragePathSegment, StoragePathSegmentError, StorageRelativePathBuf,
    identifier_file_storage_relative_path, staging_directory_name,
};
pub use crate::notification::{
    AxumNotificationRouter, NotificationApiToken, NotificationApiTokenAuthorized,
    NotificationApiTokenError, NotificationApiTokenRef, NotificationMessage,
    NotificationMessageError, NotificationRequest, NotificationSender, NotificationServiceState,
    notification_router,
};
pub use crate::origin::{
    AllowedOrigin, AllowedOriginError, AllowedOrigins, AllowedOriginsError, HttpOriginHeadersRef,
    RequestOriginAllowed, resolve_request_origin_allowed,
};
pub use crate::outbound_url::{
    OutboundAllowedHost, OutboundHostAllowlist, OutboundHostAllowlistError, OutboundHostPolicy,
    OutboundIpAddr, OutboundUrlError, OutboundUrlPolicy, OutboundUrlScheme, OutboundUrlTextRef,
    ReqwestOutboundUrl,
};
pub use crate::path_policy::{
    HttpAllowedPathPrefixRef, HttpNormalizedPath, HttpNormalizedPathError, HttpProxyPath,
    HttpProxyPathError, HttpProxyPathPrefixMatch, HttpProxyPathRef, HttpRequestPathRef,
    normalize_identifier_path, proxy_path_matches_prefix,
};
pub use crate::pg_rate_limit::{
    PgRateLimitDecision, PgRateLimitError, PgRateLimitMaximum, PgRateLimitQueryRef,
    PgRateLimitScopeRef, PgRateLimitSubjectRef, PgRateLimitValidationError,
    PgRateLimitWindowSeconds, SqlxPgRateLimitError, SqlxPgRateLimitPoolRef, enforce_pg_rate_limit,
};
pub use crate::redacted_url::{
    RedactedUrl, RedactedUrlTextRef, redact_rtsp_url_userinfo, redact_url_userinfo,
};
pub use crate::request_id_layer::RequestIdLayer;
pub use crate::request_timeout::RequestTimeoutLayer;
pub use crate::secure_cookie::{
    HttpCookieAccess, HttpCookieName, HttpCookieSecure, HttpCookieValue, HttpSecureCookieError,
    HttpSetCookieHeaderValue, StdCookieMaxAgeSeconds, build_secure_strict_cookie,
};
pub use crate::service::{
    ServeIoError, ServeWithGracefulShutdownError, ServiceRuntime, TokioTcpListener,
    add_status_route, serve_with_graceful_shutdown,
};
pub use crate::trace_context::{
    HttpHostRef, HttpMethodRef, HttpOpentelemetryHeaderMapMut, HttpOpentelemetryHeaderMapRef,
    HttpTraceParent, HttpTraceParentError, HttpTraceState, HttpTraceStateError,
    OpentelemetryContext, OutboundTraceContext, ReqwestRequest, ReqwestRequestBuilder,
    extract_remote_trace_context, inject_trace_context,
};
pub use crate::wire_token::{VersionedUrlSafeWireTokenText, VersionedUrlSafeWireTokenTextError};
pub use security_headers::{
    ForwardedProtoTrust, HttpContentSecurityPolicy, HttpContentSecurityPolicyError,
    SecurityHeadersLayer,
};
pub use server_observability::service_tracing_format::ServiceTracingFormat;
pub use server_observability::service_tracing_format::initialization::*;
pub use server_observability::*;
pub use server_runtime_core::*;
pub use text_policy::domain_types::{
    BoundedTextPolicyError, FixedLengthAsciiHexText, FixedLengthAsciiHexTextError,
    NonEmptyTrimmedText, RequiredNulFreeBoundedText, UrlSafeTokenPartText,
    UrlSafeTokenPartTextError,
};
pub use text_policy::domain_types::{
    PasswordLength, PasswordLengthRange, PasswordLengthRangeError, PasswordPolicyViolation,
    PasswordTextRef, validate_password_policy,
};

// Root-owned module compatibility wrappers.
mod axum_router {
    pub use crate::axum_router::*;
}
mod batched_cleanup {
    pub use crate::batched_cleanup::*;
}
mod bounded_read {
    pub use crate::bounded_read::*;
}
mod child_process {
    pub use crate::child_process::*;
}
mod client_ip {
    pub use crate::client_ip::*;
}
mod cors {
    pub use crate::cors::*;
}
mod csp {
    pub use crate::csp::*;
}
mod fallback {
    pub use crate::fallback::*;
}
mod geojson {
    pub use crate::geojson::*;
}
mod header_text {
    pub use crate::header_text::*;
}
mod health {
    pub use crate::health::*;
}
mod http_client {
    pub use crate::http_client::*;
}
mod http_error_diagnostic {
    pub use crate::http_error_diagnostic::*;
}
mod http_header_policy {
    pub use crate::http_header_policy::*;
}
mod http_policy {
    pub use crate::http_policy::*;
}
mod http_request_span_config {
    pub use crate::http_request_span_config::*;
}
mod http_status_error {
    pub use crate::http_status_error::*;
}
mod lifecycle {
    pub use crate::lifecycle::*;
}
mod limits {
    pub use crate::limits::*;
}
mod metrics_layer {
    pub use crate::metrics_layer::*;
}
mod multipart {
    pub use crate::multipart::*;
}
mod notification {
    pub use crate::notification::*;
}
mod origin {
    pub use crate::origin::*;
}
mod outbound_url {
    pub use crate::outbound_url::*;
}
mod path_policy {
    pub use crate::path_policy::*;
}
mod pg_rate_limit {
    pub use crate::pg_rate_limit::*;
}
mod redacted_url {
    pub use crate::redacted_url::*;
}
mod request_id {
    pub use crate::domain_types_request_id::*;
}
mod request_id_layer {
    pub use crate::request_id_layer::*;
}
mod request_id_service {
    pub use crate::request_id_service::*;
}
mod request_id_tower_layer {
    pub use crate::request_id_tower_layer::*;
}
mod request_timeout {
    pub use crate::request_timeout::*;
}
mod secure_cookie {
    pub use crate::secure_cookie::*;
}
mod security_headers {
    pub use crate::domain_types_security_headers::*;
}
mod service {
    pub use crate::service::*;
}
mod service_runtime {
    pub use crate::domain_types_service_runtime::*;
}
mod trace_context {
    pub use crate::trace_context::*;
}
mod wire_token {
    pub use crate::wire_token::*;
}
