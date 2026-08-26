pub(crate) fn naming_kebab_case(value: super::ProjectNameRef<'_>) -> super::ScaffoldText {
    super::ScaffoldText::try_from(value.0.replace('_', constants_str::HYPHEN))
        .unwrap_or_else(super::ScaffoldText::from)
}

#[cfg(test)]
mod tests {
    #[test]
    fn project_name_conversions_are_consistent() {
        let value = super::super::ProjectNameRef::from(constants_str::VALUE_F9EA74B8);
        assert_eq!(super::naming_kebab_case(value).as_ref(), "order-platform");
        assert_eq!(
            super::super::naming_title_case::naming_title_case(value).as_ref(),
            "Order Platform"
        );
        assert_eq!(
            super::super::naming_upper_camel_case::naming_upper_camel_case(value).as_ref(),
            "OrderPlatform"
        );
    }
}
