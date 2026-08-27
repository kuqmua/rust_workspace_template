use super::AdminPagePathRef;

pub(in crate::domain_types) fn admin_path_route_name(
    path: AdminPagePathRef<'static>,
) -> frontend_contract::domain_types::ContractStr {
    frontend_contract::domain_types::ContractStr::from(
        path.get()
            .rsplit_once('/')
            .map_or(path.get(), |(_prefix, name)| name),
    )
}
