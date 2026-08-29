#[allow(clippy::single_call_fn)] // named command or composition stage has one orchestration owner
pub(crate) fn check_tool_available(
    path: crate::tool_path::ToolPath,
) -> crate::tool_available::ToolAvailable {
    crate::tool_available::ToolAvailable::from(std::path::Path::new(path.get()).exists())
}
