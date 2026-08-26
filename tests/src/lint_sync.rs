#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, Eq, PartialEq)]
enum LintProbeDisposition {
    Supported,
    Unknown,
    Unstable,
}

fn probe_lint(tool: &str, lint: &str) -> LintProbeDisposition {
    let lint_arg = if tool == constants_str::CLIPPY_DRIVER {
        format!("clippy::{lint}")
    } else {
        lint.to_owned()
    };
    let output_path = std::env::temp_dir().join(format!(
        "code-style-lint-probe-{}-{}.rmeta",
        std::process::id(),
        lint
    ));
    let output_path_text = output_path.to_string_lossy();
    let args = [
        constants_str::CODE_STYLE_LINT_PROBE_CRATE_NAME_ARG,
        constants_str::CODE_STYLE_LINT_PROBE_CRATE_NAME,
        constants_str::CODE_STYLE_LINT_PROBE_CRATE_TYPE_ARG,
        constants_str::LIB,
        constants_str::CODE_STYLE_LINT_PROBE_EDITION_ARG,
        constants_str::CODE_STYLE_LINT_PROBE_EDITION,
        constants_str::CODE_STYLE_LINT_PROBE_EMIT_METADATA_ARG,
        constants_str::CODE_STYLE_LINT_PROBE_OUTPUT_ARG,
        output_path_text.as_ref(),
        constants_str::CODE_STYLE_LINT_PROBE_DENY_ARG,
        constants_str::CODE_STYLE_LINT_PROBE_UNKNOWN_LINTS,
        constants_str::CODE_STYLE_LINT_PROBE_DENY_ARG,
        lint_arg.as_str(),
        constants_str::CODE_STYLE_LINT_PROBE_INPUT_PATH,
    ];
    let output = macro_helpers::domain_types::tool_command::ToolCommand::new(
        macro_helpers::domain_types::tool_command::ToolProgramRef::from(tool),
    )
    .args(macro_helpers::domain_types::tool_command::ToolArgsRef::from(args.as_slice()))
    .output()
    .expect("3a17d9c2 probe_lint invariant must hold");
    if let Err(error) = std::fs::remove_file(output_path.as_path())
        && error.kind() != std::io::ErrorKind::NotFound
    {
        panic!("51e8c6b4: {error}");
    }
    if output.as_ref().status.success() {
        return LintProbeDisposition::Supported;
    }
    let stderr = String::from_utf8_lossy(&output.as_ref().stderr);
    if stderr.contains(constants_str::CODE_STYLE_LINT_PROBE_UNSTABLE_DIAGNOSTIC) {
        LintProbeDisposition::Unstable
    } else {
        LintProbeDisposition::Unknown
    }
}

#[test]
fn check_if_workspace_cargo_toml_workspace_lints_clippy_contains_all_clippy_lints() {
    assert!(
        constants_str::CODE_STYLE_CLIPPY_LINT_EXCEPTIONS.is_empty(),
        "42f9b1d6"
    );
    super::assert_workspace_lints_match(
        super::RustOrClippy::Clippy,
        super::types::StaticStr::from(constants_str::CLIPPY_DRIVER),
        super::types::AnalyzerBool::from(true),
        super::types::StaticStr::from(constants_str::VALUE_8895CA50),
        super::types::StaticStrSliceRef::from(
            constants_str::CODE_STYLE_CLIPPY_LINT_EXCEPTIONS.as_slice(),
        ),
    );
}
#[test]
fn check_if_workspace_cargo_toml_workspace_lints_rust_contains_all_rust_lints() {
    let reviewed_exceptions = [
        (
            constants_str::IMPLICIT_PROVENANCE_CASTS,
            constants_str::VALUE_2642B498,
        ),
        (
            constants_str::MULTIPLE_SUPERTRAIT_UPCASTABLE,
            constants_str::VALUE_5CA1A822,
        ),
        (
            constants_str::MUST_NOT_SUSPEND,
            constants_str::VALUE_8B5456A9,
        ),
        (
            constants_str::NON_EXHAUSTIVE_OMITTED_PATTERNS,
            constants_str::VALUE_C14A18CA,
        ),
        (
            constants_str::DEFAULT_OVERRIDES_DEFAULT_FIELDS,
            constants_str::VALUE_8C3E05BE,
        ),
        (
            constants_str::TEST_UNSTABLE_LINT,
            constants_str::VALUE_25E2DA35,
        ),
        (
            constants_str::RESOLVING_TO_ITEMS_SHADOWING_SUPERTRAIT_ITEMS,
            constants_str::VALUE_9201B73E,
        ),
        (
            constants_str::SHADOWING_SUPERTRAIT_ITEMS,
            constants_str::VALUE_9201B73E,
        ),
        (
            constants_str::UNQUALIFIED_LOCAL_IMPORTS,
            constants_str::VALUE_8EA48DC5,
        ),
        (
            constants_str::DEPRECATED_LLVM_INTRINSIC,
            constants_str::VALUE_837349B6,
        ),
        (
            constants_str::TAIL_CALL_TRACK_CALLER,
            constants_str::VALUE_94F20D79,
        ),
    ];
    let _validated_exceptions = reviewed_exceptions
        .iter()
        .map(|(lint, reason)| {
            assert!(!reason.is_empty(), "829d6e1f");
            assert_eq!(
                probe_lint(constants_str::RUSTC, lint),
                LintProbeDisposition::Unstable,
                "f70c3b82: `{lint}` is no longer an unsupported unstable lint; remove the exception or update the reviewed reason"
            );
        })
        .collect::<Vec<()>>();
    let exceptions = reviewed_exceptions
        .iter()
        .map(|(lint, _reason)| *lint)
        .collect::<Vec<&str>>();
    assert_eq!(
        exceptions
            .iter()
            .collect::<std::collections::BTreeSet<_>>()
            .len(),
        exceptions.len(),
        "bd2fa22f"
    );
    super::assert_workspace_lints_match(
        super::RustOrClippy::Rust,
        super::types::StaticStr::from(constants_str::RUSTC),
        super::types::AnalyzerBool::default(),
        super::types::StaticStr::from(constants_str::VALUE_3C20B457),
        //todo on commit momment seems like this lints still not added to rustc, but in the list of rustc -W help
        super::types::StaticStrSliceRef::from(exceptions.as_slice()),
    );
}
#[test]
fn clippy_lint_exceptions_are_unique() {
    assert_eq!(
        constants_str::CODE_STYLE_CLIPPY_LINT_EXCEPTIONS
            .iter()
            .collect::<std::collections::BTreeSet<_>>()
            .len(),
        constants_str::CODE_STYLE_CLIPPY_LINT_EXCEPTIONS.len(),
        "98ca7133"
    );
}

#[test]
fn lint_probe_distinguishes_supported_unstable_and_unknown_lints() {
    assert_eq!(
        probe_lint(constants_str::CLIPPY_DRIVER, "disallowed_fields"),
        LintProbeDisposition::Supported,
        "6bc218de"
    );
    assert_eq!(
        probe_lint(
            constants_str::RUSTC,
            constants_str::IMPLICIT_PROVENANCE_CASTS
        ),
        LintProbeDisposition::Unstable,
        "e1437af9"
    );
    assert_eq!(
        probe_lint(constants_str::RUSTC, "code_style_nonexistent_lint"),
        LintProbeDisposition::Unknown,
        "907ca5d3"
    );
}
