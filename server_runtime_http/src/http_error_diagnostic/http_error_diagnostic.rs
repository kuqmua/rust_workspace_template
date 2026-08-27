#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug)]
pub struct HttpErrorDiagnostic {
    backtrace: super::StdHttpErrorBacktrace,
    error_chain: super::StdHttpErrorChain,
    location: super::super::StdPanicLocation,
    span_trace: super::TracingHttpSpanTrace,
    telemetry: super::HttpErrorTelemetry,
}

impl HttpErrorDiagnostic {
    pub(in crate::domain_types) const fn backtrace(&self) -> &super::StdHttpErrorBacktrace {
        &self.backtrace
    }

    #[track_caller]
    #[must_use]
    pub fn capture(
        telemetry: super::HttpErrorTelemetry,
        error: &(dyn std::error::Error + 'static),
    ) -> Self {
        let current_span = tracing::Span::current();
        let span_trace = current_span.metadata().map_or_else(
            || constants_str::HTTP_SPAN_UNAVAILABLE.to_owned(),
            |metadata| format!("{current_span:?} [{}]", metadata.name()),
        );
        Self {
            backtrace: super::StdHttpErrorBacktrace::from(
                std::backtrace::Backtrace::force_capture()
                    .to_string()
                    .into_boxed_str(),
            ),
            error_chain: Self::error_chain(error),
            location: super::super::StdPanicLocation::from(std::panic::Location::caller()),
            span_trace: super::TracingHttpSpanTrace::from(span_trace.into_boxed_str()),
            telemetry,
        }
    }

    fn error_chain(error: &(dyn std::error::Error + 'static)) -> super::StdHttpErrorChain {
        let mut error_chain = error.to_string();
        let mut optional_source = error.source();
        while let Some(source) = optional_source {
            error_chain.push_str(constants_str::HTTP_ERROR_CHAIN_SEPARATOR);
            error_chain.push_str(source.to_string().as_str());
            optional_source = source.source();
        }
        super::StdHttpErrorChain::from(error_chain.into_boxed_str())
    }

    pub(in crate::domain_types) const fn error_chain_text(&self) -> &super::StdHttpErrorChain {
        &self.error_chain
    }

    #[must_use]
    pub fn from_observed<Source>(
        error_type: super::HttpErrorType,
        error: &super::super::ObservedError<Source>,
    ) -> Self
    where
        Source: std::error::Error + 'static,
    {
        Self {
            backtrace: super::StdHttpErrorBacktrace::from(
                error.backtrace().to_string().into_boxed_str(),
            ),
            error_chain: Self::error_chain(error),
            location: error.location(),
            span_trace: super::TracingHttpSpanTrace::from(
                error.span_trace().to_string().into_boxed_str(),
            ),
            telemetry: super::HttpErrorTelemetry::new(
                error_type,
                super::HttpErrorCode::from(error.error_code().get()),
            ),
        }
    }

    pub(in crate::domain_types) const fn location(&self) -> &super::super::StdPanicLocation {
        &self.location
    }

    pub(in crate::domain_types) const fn span_trace(&self) -> &super::TracingHttpSpanTrace {
        &self.span_trace
    }

    #[allow(clippy::single_call_fn)]
    pub(in crate::domain_types) const fn telemetry(&self) -> super::HttpErrorTelemetry {
        self.telemetry
    }
}
