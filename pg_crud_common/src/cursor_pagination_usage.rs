#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq,
)]
pub enum CursorPaginationUsage {
    CursorOnly,
    NoOffsetNoCursor,
    OffsetAndCursor,
    OffsetOnly,
}

impl CursorPaginationUsage {
    #[must_use]
    pub const fn from_presence(
        offset_pagination_presence: crate::offset_pagination_presence::OffsetPaginationPresence,
        signed_cursor_presence: crate::signed_cursor_presence::SignedCursorPresence,
    ) -> Self {
        match (offset_pagination_presence, signed_cursor_presence) {
            (
                crate::offset_pagination_presence::OffsetPaginationPresence::Absent,
                crate::signed_cursor_presence::SignedCursorPresence::Absent,
            ) => Self::NoOffsetNoCursor,
            (
                crate::offset_pagination_presence::OffsetPaginationPresence::Absent,
                crate::signed_cursor_presence::SignedCursorPresence::Present,
            ) => Self::CursorOnly,
            (
                crate::offset_pagination_presence::OffsetPaginationPresence::Present,
                crate::signed_cursor_presence::SignedCursorPresence::Absent,
            ) => Self::OffsetOnly,
            (
                crate::offset_pagination_presence::OffsetPaginationPresence::Present,
                crate::signed_cursor_presence::SignedCursorPresence::Present,
            ) => Self::OffsetAndCursor,
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_pagination_usage_distinguishes_cursor_and_offset() {
        assert_eq!(
            crate::cursor_pagination_usage::CursorPaginationUsage::from_presence(
                crate::offset_pagination_presence::OffsetPaginationPresence::Present,
                crate::signed_cursor_presence::SignedCursorPresence::Present,
            ),
            crate::cursor_pagination_usage::CursorPaginationUsage::OffsetAndCursor
        );
        assert_eq!(
            crate::cursor_pagination_usage::CursorPaginationUsage::from_presence(
                crate::offset_pagination_presence::OffsetPaginationPresence::Absent,
                crate::signed_cursor_presence::SignedCursorPresence::Present,
            ),
            crate::cursor_pagination_usage::CursorPaginationUsage::CursorOnly
        );
    }

    #[test]
    fn test_all_cursor_and_offset_presence_combinations_have_distinct_usage() {
        assert!(
            [
                (
                    crate::offset_pagination_presence::OffsetPaginationPresence::Absent,
                    crate::signed_cursor_presence::SignedCursorPresence::Absent,
                    crate::cursor_pagination_usage::CursorPaginationUsage::NoOffsetNoCursor
                ),
                (
                    crate::offset_pagination_presence::OffsetPaginationPresence::Absent,
                    crate::signed_cursor_presence::SignedCursorPresence::Present,
                    crate::cursor_pagination_usage::CursorPaginationUsage::CursorOnly
                ),
                (
                    crate::offset_pagination_presence::OffsetPaginationPresence::Present,
                    crate::signed_cursor_presence::SignedCursorPresence::Absent,
                    crate::cursor_pagination_usage::CursorPaginationUsage::OffsetOnly
                ),
                (
                    crate::offset_pagination_presence::OffsetPaginationPresence::Present,
                    crate::signed_cursor_presence::SignedCursorPresence::Present,
                    crate::cursor_pagination_usage::CursorPaginationUsage::OffsetAndCursor
                ),
            ]
            .into_iter()
            .all(
                |(offset_pagination_presence, signed_cursor_presence, expected)| {
                    crate::cursor_pagination_usage::CursorPaginationUsage::from_presence(
                        offset_pagination_presence,
                        signed_cursor_presence,
                    ) == expected
                }
            )
        );
    }
}
