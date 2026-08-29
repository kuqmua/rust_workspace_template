#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq)]
pub struct CursorMaximumLength(pub(super) std::num::NonZeroUsize);

impl TryFrom<usize> for CursorMaximumLength {
    type Error = crate::cursor_codec_build_error::CursorCodecBuildError;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        std::num::NonZeroUsize::new(value)
            .map(Self::from)
            .ok_or(crate::cursor_codec_build_error::CursorCodecBuildError::ZeroMaximumLength)
    }
}

impl From<std::num::NonZeroUsize> for CursorMaximumLength {
    fn from(value: std::num::NonZeroUsize) -> Self {
        Self(value)
    }
}
