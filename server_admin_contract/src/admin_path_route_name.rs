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
