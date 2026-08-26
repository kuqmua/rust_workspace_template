#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
#[error("{source}")]
pub struct ObservedError<Source>
where
    Source: std::error::Error + 'static,
{
    backtrace: super::observed_error_backtrace::ObservedErrorBacktrace,
    error_code: super::observed_error_code::ObservedErrorCode,
    location: super::std_panic_location::StdPanicLocation,
    source: Source,
    span_trace: super::tracing_observed_error_span_trace::TracingObservedErrorSpanTrace,
}

impl<Source> ObservedError<Source>
where
    Source: std::error::Error + 'static,
{
    #[must_use]
    pub const fn backtrace(&self) -> &super::observed_error_backtrace::ObservedErrorBacktrace {
        &self.backtrace
    }

    #[track_caller]
    #[must_use]
    pub fn capture(
        source: Source,
        error_code: super::observed_error_code::ObservedErrorCode,
    ) -> Self {
        let current_span = tracing::Span::current();
        let span_trace = current_span.metadata().map_or_else(
            || constants_str::HTTP_SPAN_UNAVAILABLE.to_owned(),
            |metadata| format!("{current_span:?} [{}]", metadata.name()),
        );
        Self {
            backtrace: super::observed_error_backtrace::ObservedErrorBacktrace::from(
                std::backtrace::Backtrace::force_capture(),
            ),
            error_code,
            location: super::std_panic_location::StdPanicLocation::from(
                std::panic::Location::caller(),
            ),
            source,
            span_trace:
                super::tracing_observed_error_span_trace::TracingObservedErrorSpanTrace::from(
                    span_trace.into_boxed_str(),
                ),
        }
    }

    #[must_use]
    pub const fn error_code(&self) -> super::observed_error_code::ObservedErrorCode {
        self.error_code
    }

    #[must_use]
    pub const fn location(&self) -> super::std_panic_location::StdPanicLocation {
        self.location
    }

    #[must_use]
    pub const fn source_ref(&self) -> &Source {
        &self.source
    }

    #[must_use]
    pub const fn span_trace(
        &self,
    ) -> &super::tracing_observed_error_span_trace::TracingObservedErrorSpanTrace {
        &self.span_trace
    }
}

#[cfg(test)]
mod tests {
    #[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
    #[error("infrastructure failed")]
    struct InfrastructureTestError;

    #[test]
    fn capture_preserves_source_code_and_diagnostics_at_call_site() {
        let expected_line = line!() + 1u32;
        let observed = super::ObservedError::capture(
            InfrastructureTestError,
            super::super::observed_error_code::ObservedErrorCode::from(
                constants_str::VALUE_D99F528C,
            ),
        );
        assert_eq!(
            observed.error_code(),
            super::super::observed_error_code::ObservedErrorCode::from("infrastructure_failed")
        );
        assert_eq!(observed.source_ref().to_string(), "infrastructure failed");
        assert_eq!(observed.location().line(), expected_line);
        assert!(!observed.backtrace().to_string().is_empty());
        assert!(!observed.span_trace().to_string().is_empty());
    }
}
