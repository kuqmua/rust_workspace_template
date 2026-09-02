pub(crate) fn naming_kebab_case(
    project_name_ref: crate::project_name_ref::ProjectNameRef<'_>,
) -> crate::scaffold_text::ScaffoldText {
    crate::scaffold_text::ScaffoldText::try_from(
        project_name_ref.get().replace('_', constants_str::HYPHEN),
    )
    .unwrap_or_else(crate::scaffold_text::ScaffoldText::from)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_project_name_conversions_are_consistent() {
        let value = crate::project_name_ref::ProjectNameRef::from(constants_str::VALUE_F9EA74B8);
        assert_eq!(
            crate::naming_kebab_case::naming_kebab_case(value).as_ref(),
            constants_str::VALUE_77A8A329
        );
        assert_eq!(
            crate::naming_title_case::naming_title_case(value).as_ref(),
            constants_str::VALUE_3EEF5CDE
        );
        assert_eq!(
            crate::naming_upper_camel_case::naming_upper_camel_case(value).as_ref(),
            constants_str::VALUE_6B0B0F05
        );
    }
}
