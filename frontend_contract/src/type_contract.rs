#[derive(proc_macro_getters::Getters)]
#[getters(bare)]
#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, PartialEq, Eq,
)]
pub struct TypeContract {
    #[getters(copy)]
    maximum: crate::numeric_bound::NumericBound,
    #[getters(copy)]
    minimum: crate::numeric_bound::NumericBound,
    #[getters(copy)]
    example: crate::value_example::ValueExample,
    #[getters(copy)]
    format: crate::value_format::ValueFormat,
    #[getters(copy)]
    input_kind: crate::input_kind::InputKind,
    #[getters(copy)]
    nullability: crate::nullability::Nullability,
    #[getters(copy)]
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
