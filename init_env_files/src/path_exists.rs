pub(crate) fn path_exists(
    path: crate::init_path_ref::InitPathRef<'_>,
) -> crate::init_path_exists::InitPathExists {
    crate::init_path_exists::InitPathExists::from(path.get().exists())
}
