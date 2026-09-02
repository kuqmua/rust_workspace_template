#[derive(
    proc_macro_getters::Getters,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    proc_macro_newtype::FromGetter,
)]
#[from_getter(source = crate::pagination_offset::PaginationOffset, getter = get)]
pub struct ListOffset(i64);

#[cfg(test)]
mod tests {
    #[test]
    fn test_pagination_offset_converts_through_getter() {
        let offset = crate::pagination_offset::PaginationOffset::from(11i32);
        assert_eq!(super::ListOffset::from(offset), super::ListOffset(11i64));
    }
}
