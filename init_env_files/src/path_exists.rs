pub(crate) fn path_exists(
    path: crate::domain_types::InitPathRef<'_>,
) -> crate::domain_types::InitPathExists {
    crate::domain_types::InitPathExists::from(path.get().exists())
}
