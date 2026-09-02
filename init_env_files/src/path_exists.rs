pub(crate) fn path_exists(
    init_path_ref: crate::init_path_ref::InitPathRef<'_>,
) -> crate::init_path_exists::InitPathExists {
    crate::init_path_exists::InitPathExists::from(init_path_ref.get().exists())
}
