pub(crate) fn path_exists(path: crate::InitPathRef<'_>) -> crate::InitPathExists {
    crate::InitPathExists::from(path.get().exists())
}
