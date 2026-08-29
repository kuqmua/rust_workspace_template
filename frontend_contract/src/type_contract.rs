#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, PartialEq, Eq)]
pub struct TypeContract {
    maximum: crate::numeric_bound::NumericBound,
    minimum: crate::numeric_bound::NumericBound,
    example: crate::value_example::ValueExample,
    format: crate::value_format::ValueFormat,
    input_kind: crate::input_kind::InputKind,
    nullability: crate::nullability::Nullability,
    step: crate::input_step::InputStep,
}
impl TypeContract {
    #[must_use]
    pub const fn new(
        input_kind: crate::input_kind::InputKind,
        format: crate::value_format::ValueFormat,
        nullability: crate::nullability::Nullability,
    ) -> Self {
        Self {
            example: crate::value_example::ValueExample::None,
            format,
            input_kind,
            maximum: crate::numeric_bound::NumericBound::None,
            minimum: crate::numeric_bound::NumericBound::None,
            nullability,
            step: crate::input_step::InputStep::Any,
        }
    }
    #[must_use]
    pub const fn example(self) -> crate::value_example::ValueExample {
        self.example
    }
    #[must_use]
    pub const fn format(self) -> crate::value_format::ValueFormat {
        self.format
    }
    #[must_use]
    pub const fn input_kind(self) -> crate::input_kind::InputKind {
        self.input_kind
    }
    #[must_use]
    pub const fn maximum(self) -> crate::numeric_bound::NumericBound {
        self.maximum
    }
    #[must_use]
    pub const fn minimum(self) -> crate::numeric_bound::NumericBound {
        self.minimum
    }
    #[must_use]
    pub const fn nullability(self) -> crate::nullability::Nullability {
        self.nullability
    }
    #[must_use]
    pub const fn step(self) -> crate::input_step::InputStep {
        self.step
    }
    #[must_use]
    pub const fn supports_filtering(self) -> crate::capability_support::CapabilitySupport {
        if matches!(
            self.format,
            crate::value_format::ValueFormat::Bytes
                | crate::value_format::ValueFormat::Interval
                | crate::value_format::ValueFormat::Range
        ) {
            crate::capability_support::CapabilitySupport::Unsupported
        } else {
            crate::capability_support::CapabilitySupport::Supported
        }
    }
    #[must_use]
    pub const fn supports_sorting(self) -> crate::capability_support::CapabilitySupport {
        if matches!(
            self.format,
            crate::value_format::ValueFormat::Bytes | crate::value_format::ValueFormat::Range
        ) {
            crate::capability_support::CapabilitySupport::Unsupported
        } else {
            crate::capability_support::CapabilitySupport::Supported
        }
    }
    #[must_use]
    pub const fn with_example(mut self, value: crate::value_example::ValueExample) -> Self {
        self.example = value;
        self
    }
    #[must_use]
    pub const fn with_maximum(mut self, value: crate::numeric_bound::NumericBound) -> Self {
        self.maximum = value;
        self
    }
    #[must_use]
    pub const fn with_minimum(mut self, value: crate::numeric_bound::NumericBound) -> Self {
        self.minimum = value;
        self
    }
    #[must_use]
    pub const fn with_step(mut self, value: crate::input_step::InputStep) -> Self {
        self.step = value;
        self
    }
}
