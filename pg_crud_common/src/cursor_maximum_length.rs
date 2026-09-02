#[derive(
    proc_macro_getters::Getters,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    proc_macro_newtype::FromInner,
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
