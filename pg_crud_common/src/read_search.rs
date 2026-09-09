#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Default,
    proc_macro_newtype_bounded_string_wrapper::BoundedStringWrapper,
    proc_macro_newtype_as_ref_str::AsRefStr,
)]
#[bounded_string(max = 128usize, chars, serde, utoipa)]
pub struct ReadSearch(bounded_types::bounded_string::BoundedString<0usize, 128usize, true>);

#[cfg(test)]
mod tests {
    #[test]
    fn test_read_search_accepts_empty_and_maximum_length() {
        let empty =
            super::ReadSearch::try_from(String::new()).expect(constants_str::DIAGNOSTIC_49929BEB);
        assert!(empty.as_ref().is_empty());
        let mut search = constants_str::LOGIN.repeat(26usize);
        search.truncate(128usize);
        let maximum =
            super::ReadSearch::try_from(search).expect(constants_str::DIAGNOSTIC_0F19B1DE);
        assert_eq!(maximum.as_ref().len(), 128usize);
    }

    #[test]
    fn test_read_search_rejects_text_above_limit() {
        let _error = super::ReadSearch::try_from(constants_str::LOGIN.repeat(26usize))
            .expect_err(constants_str::DIAGNOSTIC_A6973077);
    }
}
