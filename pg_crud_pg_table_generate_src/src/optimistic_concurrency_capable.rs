pub(crate) const fn optimistic_concurrency_capable<
    Capability,
    HttpMethod,
    Operation,
    OperationKind,
    PermissionAction,
    StatusCode,
>(
    operation_descriptor: &crate::operation_descriptor::OperationDescriptor<
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
    *operation_descriptor.get_optimistic_concurrency_capable()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_returns_optimistic_concurrency_capability() {
        let descriptor =
            crate::operation_descriptor::OperationDescriptor::new((), false, (), (), true, (), ());
        assert!(crate::optimistic_concurrency_capable::optimistic_concurrency_capable(&descriptor));
    }
}
