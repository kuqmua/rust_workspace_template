pub fn validate_pagination_invariants<Identifier>(
    first_page: &[Identifier],
    first_total: crate::pagination_total::PaginationTotal,
    second_page: &[Identifier],
    second_total: crate::pagination_total::PaginationTotal,
) -> Result<(), crate::data_invariant_violation::DataInvariantViolation>
where
    Identifier: PartialEq,
{
    if first_total != second_total {
        return Err(
            crate::data_invariant_violation::DataInvariantViolation::PaginationTotalChanged,
        );
    }
    if first_page
        .iter()
        .any(|identifier| second_page.contains(identifier))
    {
        Err(crate::data_invariant_violation::DataInvariantViolation::PaginationItemsOverlap)
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_pagination_rejects_overlap_and_total_changes() {
        assert_eq!(
            crate::validate_pagination_invariants::validate_pagination_invariants(
                &[1u8, 2u8],
                4usize.into(),
                &[2u8, 3u8],
                4usize.into(),
            ),
            Err(crate::data_invariant_violation::DataInvariantViolation::PaginationItemsOverlap)
        );
        assert_eq!(
            crate::validate_pagination_invariants::validate_pagination_invariants(
                &[1u8],
                2usize.into(),
                &[2u8],
                3usize.into(),
            ),
            Err(crate::data_invariant_violation::DataInvariantViolation::PaginationTotalChanged)
        );
    }
    #[test]
    fn test_pagination_accepts_disjoint_and_empty_pages() {
        assert_eq!(
            crate::validate_pagination_invariants::validate_pagination_invariants(
                &[1u8, 2u8],
                4usize.into(),
                &[3u8, 4u8],
                4usize.into()
            ),
            Ok(())
        );
        assert_eq!(
            crate::validate_pagination_invariants::validate_pagination_invariants(
                &[0u8; 0],
                0usize.into(),
                &[],
                0usize.into()
            ),
            Ok(())
        );
        assert_eq!(
            crate::validate_pagination_invariants::validate_pagination_invariants(
                &[1u8],
                1usize.into(),
                &[],
                1usize.into()
            ),
            Ok(())
        );
        assert_eq!(
            crate::validate_pagination_invariants::validate_pagination_invariants(
                &[],
                1usize.into(),
                &[1u8],
                1usize.into()
            ),
            Ok(())
        );
    }

    #[test]
    fn test_changed_pagination_total_precedes_overlapping_items() {
        assert_eq!(
            crate::validate_pagination_invariants::validate_pagination_invariants(
                &[1u8],
                1usize.into(),
                &[1u8],
                2usize.into()
            ),
            Err(crate::data_invariant_violation::DataInvariantViolation::PaginationTotalChanged)
        );
    }
}
