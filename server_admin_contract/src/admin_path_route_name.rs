pub(crate) fn admin_path_route_name(
    admin_page_path_ref: crate::admin_page_path_ref::AdminPagePathRef<'static>,
) -> frontend_contract::contract_str::ContractStr {
    frontend_contract::contract_str::ContractStr::from(
        admin_page_path_ref
            .get()
            .rsplit_once('/')
            .map_or_else(|| admin_page_path_ref.get(), |(_prefix, name)| name),
    )
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_route_name_preserves_no_separator_and_empty_terminal_segments() {
        assert!(
            [
                (constants_str::LOGIN, constants_str::LOGIN),
                (constants_str::EMPTY, constants_str::EMPTY),
                (constants_str::SLASH, constants_str::EMPTY),
                (
                    crate::admin_frontend_path::AdminFrontendPath::UsersRead.get(),
                    constants_str::PG_CRUD_READ_RULE_ACTION
                ),
            ]
            .into_iter()
            .all(|(path, expected)| {
                super::admin_path_route_name(crate::admin_page_path_ref::AdminPagePathRef::from(
                    path,
                ))
                .as_ref()
                    == expected
            })
        );
    }
}
