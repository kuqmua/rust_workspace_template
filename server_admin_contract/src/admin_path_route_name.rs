use super::AdminPagePathRef;

pub(crate) fn admin_path_route_name(
    path: AdminPagePathRef<'static>,
) -> frontend_contract::ContractStr {
    frontend_contract::ContractStr::from(
        path.get()
            .rsplit_once('/')
            .map_or_else(|| path.get(), |(_prefix, name)| name),
    )
}
