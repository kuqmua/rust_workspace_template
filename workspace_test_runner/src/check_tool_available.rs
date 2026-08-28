pub(crate) fn check_tool_available(
    path: crate::domain_types::ToolPath,
) -> crate::domain_types::ToolAvailable {
    crate::domain_types::ToolAvailable::from(std::path::Path::new(path.get()).exists())
}
