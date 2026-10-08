#[derive(
    proc_macro_getters::Getters,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    proc_macro_newtype_from_inner::FromInner,
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

#[cfg(test)]
mod tests {
    #[test]
    fn test_cursor_maximum_length_rejects_zero_and_preserves_positive_limits() {
        assert_eq!(
            crate::cursor_maximum_length::CursorMaximumLength::try_from(0usize),
            Err(crate::cursor_codec_build_error::CursorCodecBuildError::ZeroMaximumLength)
        );
        assert!(
            [1usize, 1_024usize, 100_000usize]
                .into_iter()
                .all(|length| {
                    crate::cursor_maximum_length::CursorMaximumLength::try_from(length).is_ok_and(
                        |cursor_maximum_length| cursor_maximum_length.get_inner().get() == length,
                    )
                })
        );
    }
}
