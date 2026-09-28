pub(crate) fn naming_kebab_case(
    project_name_ref: crate::project_name_ref::ProjectNameRef<'_>,
) -> Result<crate::scaffold_text::ScaffoldText, crate::scaffold_error::ScaffoldError> {
    let Ok(value) = crate::scaffold_text::ScaffoldText::try_from(
        project_name_ref.get().replace('_', constants_str::HYPHEN),
    ) else {
        return Err(crate::scaffold_error::ScaffoldError::ProjectName);
    };
    Ok(value)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_project_name_conversions_are_consistent() {
        let value = crate::project_name_ref::ProjectNameRef::from(constants_str::VALUE_F9EA74B8);
        assert!(matches!(
            crate::naming_kebab_case::naming_kebab_case(value),
            Ok(converted) if converted.as_ref() == constants_str::VALUE_77A8A329
        ));
        assert!(matches!(
            crate::naming_title_case::naming_title_case(value),
            Ok(converted) if converted.as_ref() == constants_str::VALUE_3EEF5CDE
        ));
        assert!(matches!(
            crate::naming_upper_camel_case::naming_upper_camel_case(value),
            Ok(converted) if converted.as_ref() == constants_str::VALUE_6B0B0F05
        ));
    }
}
