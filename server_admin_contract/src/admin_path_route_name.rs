pub(crate) fn admin_path_route_name(
    path: crate::admin_page_path_ref::AdminPagePathRef<'static>,
) -> frontend_contract::contract_str::ContractStr {
    frontend_contract::contract_str::ContractStr::from(
        path.get()
            .rsplit_once('/')
            .map_or_else(|| path.get(), |(_prefix, name)| name),
    )
}
