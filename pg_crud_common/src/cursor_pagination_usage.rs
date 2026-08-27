#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq)]
pub enum CursorPaginationUsage {
    CursorOnly,
    NoOffsetNoCursor,
    OffsetAndCursor,
    OffsetOnly,
}

impl CursorPaginationUsage {
    #[must_use]
    pub const fn from_presence(
        offset: crate::domain_types::OffsetPaginationPresence,
        cursor: crate::domain_types::SignedCursorPresence,
    ) -> Self {
        match (offset, cursor) {
            (
                crate::domain_types::OffsetPaginationPresence::Absent,
                crate::domain_types::SignedCursorPresence::Absent,
            ) => Self::NoOffsetNoCursor,
            (
                crate::domain_types::OffsetPaginationPresence::Absent,
                crate::domain_types::SignedCursorPresence::Present,
            ) => Self::CursorOnly,
            (
                crate::domain_types::OffsetPaginationPresence::Present,
                crate::domain_types::SignedCursorPresence::Absent,
            ) => Self::OffsetOnly,
            (
                crate::domain_types::OffsetPaginationPresence::Present,
                crate::domain_types::SignedCursorPresence::Present,
            ) => Self::OffsetAndCursor,
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn pagination_usage_distinguishes_cursor_and_offset() {
        assert_eq!(
            super::CursorPaginationUsage::from_presence(
                crate::domain_types::OffsetPaginationPresence::Present,
                crate::domain_types::SignedCursorPresence::Present,
            ),
            super::CursorPaginationUsage::OffsetAndCursor
        );
        assert_eq!(
            super::CursorPaginationUsage::from_presence(
                crate::domain_types::OffsetPaginationPresence::Absent,
                crate::domain_types::SignedCursorPresence::Present,
            ),
            super::CursorPaginationUsage::CursorOnly
        );
    }
}
