#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
pub enum ObservedError<Source>
where
    Source: std::error::Error + 'static,
{
    #[error("{source}")]
    Captured {
        backtrace: crate::observed_error_backtrace::ObservedErrorBacktrace,
        error_code: crate::observed_error_code::ObservedErrorCode,
        location: crate::std_panic_location::StdPanicLocation,
        source: Source,
        span_trace: crate::tracing_observed_error_span_trace::TracingObservedErrorSpanTrace,
    },
}

impl<Source> ObservedError<Source>
where
    Source: std::error::Error + 'static,
{
    #[must_use]
    pub const fn backtrace(&self) -> &crate::observed_error_backtrace::ObservedErrorBacktrace {
        match self {
            Self::Captured { backtrace, .. } => backtrace,
        }
    }

    #[track_caller]
    #[must_use]
    pub fn capture(
        source: Source,
        error_code: crate::observed_error_code::ObservedErrorCode,
    ) -> Self {
        let current_span = tracing::Span::current();
        let span_trace = current_span.metadata().map_or_else(
            || constants_str::HTTP_SPAN_UNAVAILABLE.to_owned(),
            |metadata| format!("{current_span:?} [{}]", metadata.name()),
        );
        Self::Captured {
            backtrace: crate::observed_error_backtrace::ObservedErrorBacktrace::from(
                std::backtrace::Backtrace::force_capture(),
            ),
            error_code,
            location: crate::std_panic_location::StdPanicLocation::from(
                std::panic::Location::caller(),
            ),
            source,
            span_trace:
                crate::tracing_observed_error_span_trace::TracingObservedErrorSpanTrace::from(
                    span_trace.into_boxed_str(),
                ),
        }
    }

    #[must_use]
    pub const fn error_code(&self) -> crate::observed_error_code::ObservedErrorCode {
        match self {
            Self::Captured { error_code, .. } => *error_code,
        }
    }

    #[must_use]
    pub const fn location(&self) -> crate::std_panic_location::StdPanicLocation {
        match self {
            Self::Captured { location, .. } => *location,
        }
    }

    #[must_use]
    pub const fn source_ref(&self) -> &Source {
        match self {
            Self::Captured { source, .. } => source,
        }
    }

    #[must_use]
    pub const fn span_trace(
        &self,
    ) -> &crate::tracing_observed_error_span_trace::TracingObservedErrorSpanTrace {
        match self {
            Self::Captured { span_trace, .. } => span_trace,
        }
    }
}

#[cfg(test)]
mod tests {
    #[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
    enum InfrastructureTestError {
        #[error("infrastructure failed")]
        Failed,
    }

    #[test]
    fn test_capture_preserves_source_code_and_diagnostics_at_call_site() {
        let expected_line = line!() + 1u32;
        let observed = super::ObservedError::capture(
            InfrastructureTestError::Failed,
            crate::observed_error_code::ObservedErrorCode::from(constants_str::VALUE_D99F528C),
        );
        assert_eq!(
            observed.error_code(),
            crate::observed_error_code::ObservedErrorCode::from(constants_str::VALUE_D99F528C)
        );
        assert_eq!(
            observed.source_ref().to_string(),
            constants_str::VALUE_31572E02
        );
        assert_eq!(observed.location().line(), expected_line);
        assert!(!observed.backtrace().to_string().is_empty());
        assert!(!observed.span_trace().to_string().is_empty());
    }
}
