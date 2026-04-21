#[cfg(test)]
mod integration_tests {
    use shared_logic as _;
    use test_helpers::{run_server_command_with_report_format, stdout_as_utf8};

    const SERVER_BINARY_PATH: &str = env!("CARGO_BIN_EXE_server");

    #[test]
    fn json_output_snapshot_contract_is_stable() {
        let output = run_server_command_with_report_format(
            SERVER_BINARY_PATH,
            &["--wire-format", "13|*|7"],
            "json".as_ref(),
        )
        .expect("8e1c7a2d");
        let standard_output = stdout_as_utf8(&output).expect("3d7a1e9c");

        assert!(output.status.success());
        assert_eq!(
            standard_output.trim_end(),
            "{\"operation\":\"*\",\"left\":13,\"right\":7,\"result\":91}"
        );
    }
}
