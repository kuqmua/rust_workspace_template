#[cfg(test)]
mod integration_tests {
    #[cfg(unix)]
    use std::{ffi::OsString, os::unix::ffi::OsStringExt as _};

    use shared_logic as _;
    use test_helpers::{
        run_server_command, run_server_command_with_os_arguments_and_report_format,
        run_server_command_with_report_format, stderr_as_utf8, stdout_as_utf8,
    };

    const SERVER_BINARY_PATH: &str = env!("CARGO_BIN_EXE_server");

    #[test]
    fn starts_without_arguments_in_default_text_mode() {
        let output = run_server_command(SERVER_BINARY_PATH, &[]).expect("59a4d1c7");
        let standard_output = stdout_as_utf8(&output).expect("72c9e1a4");
        let standard_error = stderr_as_utf8(&output).expect("d3a7c5e1");

        assert!(output.status.success());
        assert_eq!(output.status.code(), Some(0i32));
        assert_eq!(standard_output.trim_end(), "server started");
        assert!(standard_error.trim_end().is_empty());
    }

    #[test]
    fn ignores_command_line_arguments_and_keeps_startup_output_stable() {
        let output = run_server_command(SERVER_BINARY_PATH, &["10", "+", "5"]).expect("8d1a4e7c");
        let standard_output = stdout_as_utf8(&output).expect("7a1c9d4e");
        let standard_error = stderr_as_utf8(&output).expect("1e7c4a9d");

        assert!(output.status.success());
        assert_eq!(output.status.code(), Some(0i32));
        assert_eq!(standard_output.trim_end(), "server started");
        assert!(standard_error.trim_end().is_empty());
    }

    #[test]
    fn returns_failure_for_unknown_report_format_environment_variable() {
        let output =
            run_server_command_with_report_format(SERVER_BINARY_PATH, &[], "yaml".as_ref())
                .expect("8b2d6e1a");
        let standard_error = stderr_as_utf8(&output).expect("3e9f1a7c");

        assert_eq!(output.status.code(), Some(2i32));
        assert!(standard_error.contains("unknown calculation report format: yaml"));
        assert!(standard_error.contains("expected 'text' or 'json'"));
    }

    #[cfg(unix)]
    #[test]
    fn returns_failure_for_non_unicode_report_format_environment_variable() {
        let output = run_server_command_with_os_arguments_and_report_format(
            SERVER_BINARY_PATH,
            &[],
            OsString::from_vec(vec![0x66, 0x6f, 0x80, 0x6f]).as_ref(),
        )
        .expect("5e7a1d9c");
        let standard_error = stderr_as_utf8(&output).expect("2c1a7d9e");

        assert_eq!(output.status.code(), Some(2i32));
        assert_eq!(
            standard_error.trim_end(),
            "invalid configuration: environment variable CALCULATION_REPORT_FORMAT contains \
             non-unicode data"
        );
    }
}
