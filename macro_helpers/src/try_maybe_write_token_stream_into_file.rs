pub fn try_maybe_write_token_stream_into_file<P>(
    should_write_token_stream_into_file: crate::should_write_token_stream_into_file::ShouldWriteTokenStreamIntoFile,
    file_name: P,
    ts: crate::proc_macro2_token_stream_ref::ProcMacro2TokenStreamRef<'_>,
    format_with_cargofmt: &crate::format_with_cargofmt::FormatWithCargofmt,
) -> std::io::Result<()>
where
    P: AsRef<std::path::Path>,
{
    if !matches!(
        should_write_token_stream_into_file,
        crate::should_write_token_stream_into_file::ShouldWriteTokenStreamIntoFile::True
    ) {
        return Ok(());
    }
    let string_cnt = ts.as_ref().to_string();
    let wr_outcome =
        crate::try_write_string_into_file_with_outcome::try_write_string_into_file_with_outcome(
            file_name,
            crate::string_file_content_ref::StringFileContentRef::from(string_cnt.as_str()),
        )?;
    if bool::from(wr_outcome.is_changed())
        && matches!(
            format_with_cargofmt,
            crate::format_with_cargofmt::FormatWithCargofmt::True
        )
    {
        let path = wr_outcome.path();
        let mut command = crate::tool_command::ToolCommand::new(
            crate::tool_program_ref::ToolProgramRef::from(constants_str::RUSTFMT),
        );
        let path_text = path.as_ref().to_string_lossy();
        let status = command
            .arg(crate::tool_arg_ref::ToolArgRef::from(path_text.as_ref()))
            .status()?;
        if !status.success() {
            return Err(std::io::Error::other(format!(
                "rustfmt failed for {}",
                path.as_ref().display()
            )));
        }
    }
    Ok(())
}
