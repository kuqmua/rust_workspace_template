#[derive(Debug, Clone, Copy, proc_macro_optimal_memory_layout::OptimalMemoryLayout)]
pub enum ToolConsoleStream {
    StandardError,
    StandardOutput,
}

impl ToolConsoleStream {
    pub fn write(
        self,
        std_fmt_arguments: crate::std_fmt_arguments::StdFmtArguments<'_>,
    ) -> Result<(), crate::tool_console_write_error::ToolConsoleWriteError> {
        match self {
            Self::StandardError => self.write_to(
                crate::std_io_write_ref::StdIoWriteRef::from(&mut std::io::stderr()),
                std_fmt_arguments,
            ),
            Self::StandardOutput => self.write_to(
                crate::std_io_write_ref::StdIoWriteRef::from(&mut std::io::stdout()),
                std_fmt_arguments,
            ),
        }
    }

    pub fn write_or_exit(self, std_fmt_arguments: crate::std_fmt_arguments::StdFmtArguments<'_>) {
        self.write(std_fmt_arguments)
            .unwrap_or_else(|tool_console_write_error| {
                Self::StandardError
                    .write(crate::std_fmt_arguments::StdFmtArguments::from(
                        format_args!("{tool_console_write_error}{}", constants_str::NEWLINE),
                    ))
                    .unwrap_or_else(|_diagnostic_write_error| std::process::exit(1));
                std::process::exit(1);
            });
    }

    fn write_to<Writer>(
        self,
        mut std_io_write_ref: crate::std_io_write_ref::StdIoWriteRef<'_, Writer>,
        std_fmt_arguments: crate::std_fmt_arguments::StdFmtArguments<'_>,
    ) -> Result<(), crate::tool_console_write_error::ToolConsoleWriteError>
    where
        Writer: std::io::Write,
    {
        std::io::Write::write_fmt(&mut **std_io_write_ref, *std_fmt_arguments).map_err(|error| {
            let execution_io_error = crate::std_tool_io_error::StdToolIoError::from(error);
            match self {
                Self::StandardError => {
                    crate::tool_console_write_error::ToolConsoleWriteError::StandardError(
                        execution_io_error,
                    )
                }
                Self::StandardOutput => {
                    crate::tool_console_write_error::ToolConsoleWriteError::StandardOutput(
                        execution_io_error,
                    )
                }
            }
        })
    }
}

#[cfg(test)]
mod tests {
    #[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout)]
    struct TestConsoleWriteFailure;

    impl std::io::Write for TestConsoleWriteFailure {
        fn write(&mut self, _buffer: &[u8]) -> std::io::Result<usize> {
            Err(std::io::Error::from(std::io::ErrorKind::BrokenPipe))
        }

        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    #[test]
    fn test_console_writer_preserves_text_and_explicit_newline() {
        let verify = |tool_console_stream: super::ToolConsoleStream| {
            let mut bytes = Vec::new();
            tool_console_stream
                .write_to(
                    crate::std_io_write_ref::StdIoWriteRef::from(&mut bytes),
                    crate::std_fmt_arguments::StdFmtArguments::from(format_args!(
                        "{}{}",
                        constants_str::ERROR,
                        constants_str::NEWLINE
                    )),
                )
                .expect(constants_str::DIAGNOSTIC_C30DA4F4);
            assert_eq!(
                bytes,
                format!("{}{}", constants_str::ERROR, constants_str::NEWLINE).as_bytes()
            );
        };
        verify(super::ToolConsoleStream::StandardError);
        verify(super::ToolConsoleStream::StandardOutput);
    }

    #[test]
    fn test_console_writer_retains_stream_and_broken_pipe_source() {
        let verify = |tool_console_stream: super::ToolConsoleStream| {
            let error = tool_console_stream
                .write_to(
                    crate::std_io_write_ref::StdIoWriteRef::from(&mut TestConsoleWriteFailure),
                    crate::std_fmt_arguments::StdFmtArguments::from(format_args!(
                        "{}",
                        constants_str::ERROR
                    )),
                )
                .expect_err(constants_str::DIAGNOSTIC_EB03C3D5);
            match (tool_console_stream, error) {
                (
                    super::ToolConsoleStream::StandardError,
                    crate::tool_console_write_error::ToolConsoleWriteError::StandardError(source),
                )
                | (
                    super::ToolConsoleStream::StandardOutput,
                    crate::tool_console_write_error::ToolConsoleWriteError::StandardOutput(source),
                ) => assert_eq!(source.kind(), std::io::ErrorKind::BrokenPipe),
                _ => assert_eq!(
                    constants_usize::ZERO,
                    constants_usize::ONE,
                    "{}",
                    constants_str::DIAGNOSTIC_427BB867
                ),
            }
        };
        verify(super::ToolConsoleStream::StandardError);
        verify(super::ToolConsoleStream::StandardOutput);
    }
}
