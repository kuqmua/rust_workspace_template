#[derive(
    generate_accessor::Getters,
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    newtype::FromInner,
)]
pub struct CursorMaximumLength(std::num::NonZeroUsize);

impl TryFrom<usize> for CursorMaximumLength {
    type Error = crate::cursor_codec_build_error::CursorCodecBuildError;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        std::num::NonZeroUsize::new(value)
            .map(Self::from)
            .ok_or(crate::cursor_codec_build_error::CursorCodecBuildError::ZeroMaximumLength)
    }
}
