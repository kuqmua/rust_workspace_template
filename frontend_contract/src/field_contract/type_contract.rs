use super::{
    CapabilitySupport, InputKind, InputStep, Nullability, NumericBound, ValueExample, ValueFormat,
};

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, PartialEq, Eq)]
pub struct TypeContract {
    maximum: NumericBound,
    minimum: NumericBound,
    example: ValueExample,
    format: ValueFormat,
    input_kind: InputKind,
    nullability: Nullability,
    step: InputStep,
}
impl TypeContract {
    #[must_use]
    pub const fn new(input_kind: InputKind, format: ValueFormat, nullability: Nullability) -> Self {
        Self {
            example: ValueExample::None,
            format,
            input_kind,
            maximum: NumericBound::None,
            minimum: NumericBound::None,
            nullability,
            step: InputStep::Any,
        }
    }
    #[must_use]
    pub const fn example(self) -> ValueExample {
        self.example
    }
    #[must_use]
    pub const fn format(self) -> ValueFormat {
        self.format
    }
    #[must_use]
    pub const fn input_kind(self) -> InputKind {
        self.input_kind
    }
    #[must_use]
    pub const fn maximum(self) -> NumericBound {
        self.maximum
    }
    #[must_use]
    pub const fn minimum(self) -> NumericBound {
        self.minimum
    }
    #[must_use]
    pub const fn nullability(self) -> Nullability {
        self.nullability
    }
    #[must_use]
    pub const fn step(self) -> InputStep {
        self.step
    }
    #[must_use]
    pub const fn supports_filtering(self) -> CapabilitySupport {
        if matches!(
            self.format,
            ValueFormat::Bytes | ValueFormat::Interval | ValueFormat::Range
        ) {
            CapabilitySupport::Unsupported
        } else {
            CapabilitySupport::Supported
        }
    }
    #[must_use]
    pub const fn supports_sorting(self) -> CapabilitySupport {
        if matches!(self.format, ValueFormat::Bytes | ValueFormat::Range) {
            CapabilitySupport::Unsupported
        } else {
            CapabilitySupport::Supported
        }
    }
    #[must_use]
    pub const fn with_example(mut self, value: ValueExample) -> Self {
        self.example = value;
        self
    }
    #[must_use]
    pub const fn with_maximum(mut self, value: NumericBound) -> Self {
        self.maximum = value;
        self
    }
    #[must_use]
    pub const fn with_minimum(mut self, value: NumericBound) -> Self {
        self.minimum = value;
        self
    }
    #[must_use]
    pub const fn with_step(mut self, value: InputStep) -> Self {
        self.step = value;
        self
    }
}
