#![allow(
    clippy::arbitrary_source_item_ordering,
    reason = "the flat source facade keeps its owner adjacent to implementation while declaring sibling modules"
)]
#[derive(proc_macro_getters::Getters)]
#[getters(bare)]
#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Clone, Debug)]
pub struct HttpErrorDiagnostic {
    backtrace: crate::std_http_error_backtrace::StdHttpErrorBacktrace,
    #[getters(skip)]
    error_chain: crate::std_http_error_chain::StdHttpErrorChain,
    location: server_observability::std_panic_location::StdPanicLocation,
    span_trace: crate::tracing_http_span_trace::TracingHttpSpanTrace,
    telemetry: crate::http_error_telemetry::HttpErrorTelemetry,
}

impl HttpErrorDiagnostic {
    #[track_caller]
    #[must_use]
    pub fn capture(
        telemetry: crate::http_error_telemetry::HttpErrorTelemetry,
        error: &(dyn std::error::Error + 'static),
    ) -> Self {
        let current_span = tracing::Span::current();
        let span_trace = current_span.metadata().map_or_else(
            || constants_str::HTTP_SPAN_UNAVAILABLE.to_owned(),
            |metadata| format!("{current_span:?} [{}]", metadata.name()),
        );
        Self {
            backtrace: crate::std_http_error_backtrace::StdHttpErrorBacktrace::from(
                std::backtrace::Backtrace::force_capture()
                    .to_string()
                    .into_boxed_str(),
            ),
            error_chain: Self::error_chain(error),
            location: server_observability::std_panic_location::StdPanicLocation::from(
                std::panic::Location::caller(),
            ),
            span_trace: crate::tracing_http_span_trace::TracingHttpSpanTrace::from(
                span_trace.into_boxed_str(),
            ),
            telemetry,
        }
    }

    fn error_chain(
        error: &(dyn std::error::Error + 'static),
    ) -> crate::std_http_error_chain::StdHttpErrorChain {
        #[derive(
            proc_macro_optimal_memory_layout::OptimalMemoryLayout, proc_macro_newtype::FromInner,
        )]
        struct ErrorChain<'error_lt>(&'error_lt (dyn std::error::Error + 'static));
        impl std::fmt::Display for ErrorChain<'_> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let mut is_first = true;
                let mut current = Some(self.0);
                while let Some(error) = current {
                    if !is_first {
                        f.write_str(constants_str::HTTP_ERROR_CHAIN_SEPARATOR)?;
                    }
                    std::fmt::Display::fmt(error, f)?;
                    is_first = false;
                    current = error.source();
                }
                Ok(())
            }
        }
        crate::std_http_error_chain::StdHttpErrorChain::from(
            ErrorChain::from(error).to_string().into_boxed_str(),
        )
    }

    pub(crate) const fn error_chain_text(&self) -> &crate::std_http_error_chain::StdHttpErrorChain {
        &self.error_chain
    }

    #[must_use]
    pub fn from_observed<Source>(
        error_type: crate::http_error_type::HttpErrorType,
        error: &server_observability::observed_error::ObservedError<Source>,
    ) -> Self
    where
        Source: std::error::Error + 'static,
    {
        Self {
            backtrace: crate::std_http_error_backtrace::StdHttpErrorBacktrace::from(
                error.backtrace().to_string().into_boxed_str(),
            ),
            error_chain: Self::error_chain(error),
            location: error.location(),
            span_trace: crate::tracing_http_span_trace::TracingHttpSpanTrace::from(
                error.span_trace().to_string().into_boxed_str(),
            ),
            telemetry: crate::http_error_telemetry::HttpErrorTelemetry::new(
                error_type,
                crate::http_error_code::HttpErrorCode::from(error.error_code().get()),
            ),
        }
    }
}

impl From<&HttpErrorDiagnostic> for crate::http_error_telemetry::HttpErrorTelemetry {
    fn from(value: &HttpErrorDiagnostic) -> Self {
        value.telemetry
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn test_fallback_diagnostic_keeps_telemetry() {
        let telemetry = crate::http_error_telemetry::HttpErrorTelemetry::new(
            crate::http_error_type::HttpErrorType::from(constants_str::VALUE_AF7C24A2),
            crate::http_error_code::HttpErrorCode::from(constants_str::VALUE_CF4DCEBB),
        );
        let diagnostic = super::HttpErrorDiagnostic::capture(
            telemetry,
            &crate::http_error_without_diagnostic_context::HttpErrorWithoutDiagnosticContext::Missing,
        );
        assert_eq!(
            diagnostic.telemetry.error_code().to_string(),
            "test_failure"
        );
        assert_eq!(diagnostic.telemetry.error_type().to_string(), "test.error");
    }
}
