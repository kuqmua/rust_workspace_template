#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    proc_macro_newtype_deref_mut_target::DerefMutTarget,
    proc_macro_newtype_deref_target::DerefTarget,
    proc_macro_newtype_from_inner::FromInner,
)]
pub(super) struct FunctionBodyLocationsBTreeMapMutRef<'map_lt>(
    &'map_lt mut std::collections::BTreeMap<
        crate::function_body_hash::FunctionBodyHash,
        crate::source_text_list::SourceTextList,
    >,
);
impl<'map_lt>
    From<&'map_lt mut crate::function_body_locations_b_tree_map::FunctionBodyLocationsBTreeMap>
    for FunctionBodyLocationsBTreeMapMutRef<'map_lt>
{
    fn from(
        value: &'map_lt mut crate::function_body_locations_b_tree_map::FunctionBodyLocationsBTreeMap,
    ) -> Self {
        Self::from(&mut **value)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_function_location_adapter_updates_its_source_map() {
        let mut function_body_locations_b_tree_map =
            crate::function_body_locations_b_tree_map::FunctionBodyLocationsBTreeMap::default();
        let function_body_hash = crate::function_body_hash::FunctionBodyHash::from(1u64);
        super::FunctionBodyLocationsBTreeMapMutRef::from(&mut function_body_locations_b_tree_map)
            .entry(function_body_hash)
            .or_default()
            .push(String::from(constants_str::ERROR));
        assert_eq!(
            function_body_locations_b_tree_map
                .get(&function_body_hash)
                .map(|source_text_list| source_text_list.as_slice()),
            Some([String::from(constants_str::ERROR)].as_slice())
        );
    }
}
