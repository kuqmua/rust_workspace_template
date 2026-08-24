#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    newtype::Display,
    newtype::FromInner,
)]
pub struct HttpErrorCode(&'static str);

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    newtype::Display,
    newtype::FromInner,
)]
pub struct HttpErrorType(&'static str);

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug)]
pub struct HttpErrorTelemetry {
    error_code: HttpErrorCode,
    error_type: HttpErrorType,
}

#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, newtype::Display, newtype::FromInner,
)]
pub(super) struct StdHttpErrorBacktrace(Box<str>);

#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, newtype::Display, newtype::FromInner,
)]
pub(super) struct StdHttpErrorChain(Box<str>);

#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, newtype::Display, newtype::FromInner,
)]
pub(super) struct TracingHttpSpanTrace(Box<str>);

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug)]
pub struct HttpErrorDiagnostic {
    backtrace: StdHttpErrorBacktrace,
    error_chain: StdHttpErrorChain,
    location: super::StdPanicLocation,
    span_trace: TracingHttpSpanTrace,
    telemetry: HttpErrorTelemetry,
}

impl HttpErrorDiagnostic {
    pub(super) const fn backtrace(&self) -> &StdHttpErrorBacktrace {
        &self.backtrace
    }

    #[track_caller]
    #[must_use]
    pub fn capture(
        telemetry: HttpErrorTelemetry,
        error: &(dyn std::error::Error + 'static),
    ) -> Self {
        let current_span = tracing::Span::current();
        let span_trace = current_span.metadata().map_or_else(
            || constants_str::HTTP_SPAN_UNAVAILABLE.to_owned(),
            |metadata| format!("{current_span:?} [{}]", metadata.name()),
        );
        Self {
            backtrace: StdHttpErrorBacktrace::from(
                std::backtrace::Backtrace::force_capture()
                    .to_string()
                    .into_boxed_str(),
            ),
            error_chain: Self::error_chain(error),
            location: super::StdPanicLocation::from(std::panic::Location::caller()),
            span_trace: TracingHttpSpanTrace::from(span_trace.into_boxed_str()),
            telemetry,
        }
    }

    fn error_chain(error: &(dyn std::error::Error + 'static)) -> StdHttpErrorChain {
        let mut error_chain = error.to_string();
        let mut optional_source = error.source();
        while let Some(source) = optional_source {
            error_chain.push_str(constants_str::HTTP_ERROR_CHAIN_SEPARATOR);
            error_chain.push_str(source.to_string().as_str());
            optional_source = source.source();
        }
        StdHttpErrorChain::from(error_chain.into_boxed_str())
    }

    pub(super) const fn error_chain_text(&self) -> &StdHttpErrorChain {
        &self.error_chain
    }

    #[must_use]
    pub fn from_observed<Source>(
        error_type: HttpErrorType,
        error: &super::ObservedError<Source>,
    ) -> Self
    where
        Source: std::error::Error + 'static,
    {
        Self {
            backtrace: StdHttpErrorBacktrace::from(error.backtrace().to_string().into_boxed_str()),
            error_chain: Self::error_chain(error),
            location: error.location(),
            span_trace: TracingHttpSpanTrace::from(error.span_trace().to_string().into_boxed_str()),
            telemetry: HttpErrorTelemetry::new(
                error_type,
                HttpErrorCode::from(error.error_code().get()),
            ),
        }
    }

    pub(super) const fn location(&self) -> &super::StdPanicLocation {
        &self.location
    }

    pub(super) const fn span_trace(&self) -> &TracingHttpSpanTrace {
        &self.span_trace
    }

    #[allow(
        clippy::single_call_fn,
        reason = "keeps diagnostic representation private while request telemetry consumes it"
    )]
    pub(super) const fn telemetry(&self) -> HttpErrorTelemetry {
        self.telemetry
    }
}

#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
#[error("{}", constants_str::HTTP_ERROR_WITHOUT_DIAGNOSTIC_CONTEXT)]
struct HttpErrorWithoutDiagnosticContext;

impl HttpErrorTelemetry {
    pub(super) const fn error_code(self) -> HttpErrorCode {
        self.error_code
    }

    pub(super) const fn error_type(self) -> HttpErrorType {
        self.error_type
    }

    #[must_use]
    pub const fn new(error_type: HttpErrorType, error_code: HttpErrorCode) -> Self {
        Self {
            error_code,
            error_type,
        }
    }
}

#[track_caller]
#[allow(
    clippy::single_call_fn,
    reason = "keeps fallback diagnostic construction inside the diagnostic owner module"
)]
pub(super) fn capture_without_context(telemetry: HttpErrorTelemetry) -> HttpErrorDiagnostic {
    HttpErrorDiagnostic::capture(telemetry, &HttpErrorWithoutDiagnosticContext)
}

#[cfg(test)]
mod tests {
    #[test]
    fn fallback_diagnostic_keeps_telemetry() {
        let telemetry = super::HttpErrorTelemetry::new(
            super::HttpErrorType::from("test.error"),
            super::HttpErrorCode::from("test_failure"),
        );
        let diagnostic = super::capture_without_context(telemetry);
        assert_eq!(
            diagnostic.telemetry().error_code().to_string(),
            "test_failure"
        );
        assert_eq!(
            diagnostic.telemetry().error_type().to_string(),
            "test.error"
        );
    }
}
