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
        offset: crate::offset_pagination_presence::OffsetPaginationPresence,
        cursor: crate::signed_cursor_presence::SignedCursorPresence,
    ) -> Self {
        match (offset, cursor) {
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
}
