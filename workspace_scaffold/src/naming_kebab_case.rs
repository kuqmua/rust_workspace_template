pub(crate) fn naming_kebab_case(
    value: crate::project_name_ref::ProjectNameRef<'_>,
) -> crate::scaffold_text::ScaffoldText {
    crate::scaffold_text::ScaffoldText::try_from(value.get().replace('_', constants_str::HYPHEN))
        .unwrap_or_else(crate::scaffold_text::ScaffoldText::from)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_project_name_conversions_are_consistent() {
        let value = crate::project_name_ref::ProjectNameRef::from(constants_str::VALUE_F9EA74B8);
        assert_eq!(
            crate::naming_kebab_case::naming_kebab_case(value).as_ref(),
            "order-platform"
        );
        assert_eq!(
            crate::naming_title_case::naming_title_case(value).as_ref(),
            "Order Platform"
        );
        assert_eq!(
            crate::naming_upper_camel_case::naming_upper_camel_case(value).as_ref(),
            "OrderPlatform"
        );
    }
}
