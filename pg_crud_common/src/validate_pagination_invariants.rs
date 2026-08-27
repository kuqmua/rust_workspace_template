pub fn validate_pagination_invariants<Identifier>(
    first_page: &[Identifier],
    first_total: crate::domain_types::PaginationTotal,
    second_page: &[Identifier],
    second_total: crate::domain_types::PaginationTotal,
) -> Result<(), crate::domain_types::DataInvariantViolation>
where
    Identifier: PartialEq,
{
    if first_total != second_total {
        return Err(crate::domain_types::DataInvariantViolation::PaginationTotalChanged);
    }
    if first_page
        .iter()
        .any(|identifier| second_page.contains(identifier))
    {
        Err(crate::domain_types::DataInvariantViolation::PaginationItemsOverlap)
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn pagination_rejects_overlap_and_total_changes() {
        assert_eq!(
            super::validate_pagination_invariants(
                &[1u8, 2u8],
                4usize.into(),
                &[2u8, 3u8],
                4usize.into(),
            ),
            Err(crate::domain_types::DataInvariantViolation::PaginationItemsOverlap)
        );
        assert_eq!(
            super::validate_pagination_invariants(&[1u8], 2usize.into(), &[2u8], 3usize.into(),),
            Err(crate::domain_types::DataInvariantViolation::PaginationTotalChanged)
        );
    }
}
