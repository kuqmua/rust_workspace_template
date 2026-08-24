#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    newtype::Display,
    newtype::FromInner,
)]
pub struct ObservedErrorCode(&'static str);

impl ObservedErrorCode {
    #[must_use]
    pub const fn get(self) -> &'static str {
        self.0
    }
}

#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Debug, newtype::Display, newtype::FromInner,
)]
pub struct StdObservedErrorBacktrace(std::backtrace::Backtrace);

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    newtype::Display,
    newtype::FromInner,
)]
pub struct StdPanicLocation(&'static std::panic::Location<'static>);

#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Debug, newtype::Display, newtype::FromInner,
)]
pub struct TracingObservedErrorSpanTrace(Box<str>);

#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
#[error("{source}")]
pub struct ObservedError<Source>
where
    Source: std::error::Error + 'static,
{
    backtrace: StdObservedErrorBacktrace,
    error_code: ObservedErrorCode,
    location: StdPanicLocation,
    source: Source,
    span_trace: TracingObservedErrorSpanTrace,
}

impl<Source> ObservedError<Source>
where
    Source: std::error::Error + 'static,
{
    #[must_use]
    pub const fn backtrace(&self) -> &StdObservedErrorBacktrace {
        &self.backtrace
    }

    #[track_caller]
    #[must_use]
    pub fn capture(source: Source, error_code: ObservedErrorCode) -> Self {
        let current_span = tracing::Span::current();
        let span_trace = current_span.metadata().map_or_else(
            || constants_str::HTTP_SPAN_UNAVAILABLE.to_owned(),
            |metadata| format!("{current_span:?} [{}]", metadata.name()),
        );
        Self {
            backtrace: StdObservedErrorBacktrace::from(std::backtrace::Backtrace::force_capture()),
            error_code,
            location: StdPanicLocation::from(std::panic::Location::caller()),
            source,
            span_trace: TracingObservedErrorSpanTrace::from(span_trace.into_boxed_str()),
        }
    }

    #[must_use]
    pub const fn error_code(&self) -> ObservedErrorCode {
        self.error_code
    }

    #[must_use]
    pub const fn location(&self) -> StdPanicLocation {
        self.location
    }

    #[must_use]
    pub const fn source_ref(&self) -> &Source {
        &self.source
    }

    #[must_use]
    pub const fn span_trace(&self) -> &TracingObservedErrorSpanTrace {
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
            super::ObservedErrorCode::from("infrastructure_failed"),
        );
        assert_eq!(
            observed.error_code(),
            super::ObservedErrorCode::from("infrastructure_failed")
        );
        assert_eq!(observed.source_ref().to_string(), "infrastructure failed");
        assert_eq!(observed.location().0.line(), expected_line);
        assert!(!observed.backtrace().to_string().is_empty());
        assert!(!observed.span_trace().to_string().is_empty());
    }
}
