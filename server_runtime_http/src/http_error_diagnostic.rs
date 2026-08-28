#![allow(
    clippy::arbitrary_source_item_ordering,
    clippy::module_inception,
    reason = "the flat source facade keeps its owner adjacent to implementation while declaring sibling modules"
)]
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug)]
pub struct HttpErrorDiagnostic {
    backtrace: StdHttpErrorBacktrace,
    error_chain: StdHttpErrorChain,
    location: super::StdPanicLocation,
    span_trace: TracingHttpSpanTrace,
    telemetry: HttpErrorTelemetry,
}

impl HttpErrorDiagnostic {
    pub(crate) const fn backtrace(&self) -> &StdHttpErrorBacktrace {
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

    pub(crate) const fn error_chain_text(&self) -> &StdHttpErrorChain {
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

    pub(crate) const fn location(&self) -> &super::StdPanicLocation {
        &self.location
    }

    pub(crate) const fn span_trace(&self) -> &TracingHttpSpanTrace {
        &self.span_trace
    }

    // The owner module retains lint-sensitive semantics from the original implementation.

    #[allow(
        clippy::single_call_fn,
        reason = "typed telemetry accessor keeps private representation out of middleware"
    )]
    pub(crate) const fn telemetry(&self) -> HttpErrorTelemetry {
        self.telemetry
    }
}

pub(super) use crate::capture_without_context::capture_without_context;
pub use crate::http_error_code::HttpErrorCode;
pub use crate::http_error_telemetry::HttpErrorTelemetry;
pub use crate::http_error_type::HttpErrorType;
use crate::http_error_without_diagnostic_context::HttpErrorWithoutDiagnosticContext;
pub(super) use crate::std_http_error_backtrace::StdHttpErrorBacktrace;
pub(super) use crate::std_http_error_chain::StdHttpErrorChain;
pub(super) use crate::tracing_http_span_trace::TracingHttpSpanTrace;

#[cfg(test)]
mod tests {
    #[test]
    fn fallback_diagnostic_keeps_telemetry() {
        let telemetry = super::HttpErrorTelemetry::new(
            super::HttpErrorType::from(constants_str::VALUE_AF7C24A2),
            super::HttpErrorCode::from(constants_str::VALUE_CF4DCEBB),
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

// Root-owned module compatibility wrappers.
mod capture_without_context {
    pub use crate::capture_without_context::*;
}
mod http_error_code {
    pub use crate::http_error_code::*;
}
mod http_error_telemetry {
    pub use crate::http_error_telemetry::*;
}
mod http_error_type {
    pub use crate::http_error_type::*;
}
mod http_error_without_diagnostic_context {
    pub use crate::http_error_without_diagnostic_context::*;
}
mod std_http_error_backtrace {
    pub use crate::std_http_error_backtrace::*;
}
mod std_http_error_chain {
    pub use crate::std_http_error_chain::*;
}
mod tracing_http_span_trace {
    pub use crate::tracing_http_span_trace::*;
}
