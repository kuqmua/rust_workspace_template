pub(crate) const fn optimistic_concurrency_capable<
    Capability,
    HttpMethod,
    Operation,
    OperationKind,
    PermissionAction,
    StatusCode,
>(
    dsc: &crate::operation_dsc::OperationDsc<
        Capability,
        HttpMethod,
        Operation,
        OperationKind,
        PermissionAction,
        StatusCode,
    >,
) -> Capability
where
    Capability: Copy,
{
    *dsc.get_optimistic_concurrency_capable()
}

#[cfg(test)]
mod tests {
    #[test]
    fn returns_optimistic_concurrency_capability() {
        let dsc = crate::operation_dsc::OperationDsc::new((), false, (), (), true, (), ());
        assert!(crate::optimistic_concurrency_capable::optimistic_concurrency_capable(&dsc));
    }
}
