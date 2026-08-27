#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq)]
pub struct CursorMaximumLength(
    pub(super) super::cursor_maximum_length_non_zero_usize::CursorMaximumLengthNonZeroUsize,
);

impl TryFrom<usize> for CursorMaximumLength {
    type Error = crate::domain_types::CursorCodecBuildError;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        std::num::NonZeroUsize::new(value)
            .map(Self::from)
            .ok_or(crate::domain_types::CursorCodecBuildError::ZeroMaximumLength)
    }
}

impl From<std::num::NonZeroUsize> for CursorMaximumLength {
    fn from(value: std::num::NonZeroUsize) -> Self {
        Self(
            super::cursor_maximum_length_non_zero_usize::CursorMaximumLengthNonZeroUsize::from(
                value,
            ),
        )
    }
}
