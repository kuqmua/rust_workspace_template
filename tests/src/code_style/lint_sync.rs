#[derive(optml::Optml, Debug, Eq, PartialEq)]
enum LintProbeDisposition {
    Supported,
    Unknown,
    Unstable,
}

fn probe_lint(tool: &str, lint: &str) -> LintProbeDisposition {
    let lint_arg = if tool == str_constants::CLIPPY_DRIVER {
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
        str_constants::CODE_STYLE_LINT_PROBE_CRATE_NAME_ARG,
        str_constants::CODE_STYLE_LINT_PROBE_CRATE_NAME,
        str_constants::CODE_STYLE_LINT_PROBE_CRATE_TYPE_ARG,
        str_constants::LIB,
        str_constants::CODE_STYLE_LINT_PROBE_EDITION_ARG,
        str_constants::CODE_STYLE_LINT_PROBE_EDITION,
        str_constants::CODE_STYLE_LINT_PROBE_EMIT_METADATA_ARG,
        str_constants::CODE_STYLE_LINT_PROBE_OUTPUT_ARG,
        output_path_text.as_ref(),
        str_constants::CODE_STYLE_LINT_PROBE_DENY_ARG,
        str_constants::CODE_STYLE_LINT_PROBE_UNKNOWN_LINTS,
        str_constants::CODE_STYLE_LINT_PROBE_DENY_ARG,
        lint_arg.as_str(),
        str_constants::CODE_STYLE_LINT_PROBE_INPUT_PATH,
    ];
    let output = macros_helpers::tool_command::ToolCommand::new(
        macros_helpers::tool_command::ToolProgramRef::from(tool),
    )
    .args(macros_helpers::tool_command::ToolArgsRef::from(
        args.as_slice(),
    ))
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
    if stderr.contains(str_constants::CODE_STYLE_LINT_PROBE_UNSTABLE_DIAGNOSTIC) {
        LintProbeDisposition::Unstable
    } else {
        LintProbeDisposition::Unknown
    }
}

#[test]
fn check_if_workspace_cargo_toml_workspace_lints_clippy_contains_all_clippy_lints() {
    assert!(
        str_constants::CODE_STYLE_CLIPPY_LINT_EXCEPTIONS.is_empty(),
        "42f9b1d6"
    );
    super::assert_workspace_lints_match(
        super::RustOrClippy::Clippy,
        super::types::StaticStr::from(str_constants::CLIPPY_DRIVER),
        super::types::AnalyzerBool::from(true),
        super::types::StaticStr::from(str_constants::VALUE_8895CA50),
        super::types::StaticStrSliceRef::from(
            str_constants::CODE_STYLE_CLIPPY_LINT_EXCEPTIONS.as_slice(),
        ),
    );
}
#[test]
fn check_if_workspace_cargo_toml_workspace_lints_rust_contains_all_rust_lints() {
    let reviewed_exceptions = [
        (
            str_constants::IMPLICIT_PROVENANCE_CASTS,
            "rust-lang/rust#130351",
        ),
        (
            str_constants::MULTIPLE_SUPERTRAIT_UPCASTABLE,
            "rust-lang/rust#150833",
        ),
        (str_constants::MUST_NOT_SUSPEND, "rust-lang/rust#83310"),
        (
            str_constants::NON_EXHAUSTIVE_OMITTED_PATTERNS,
            "rust-lang/rust#89554",
        ),
        (
            str_constants::DEFAULT_OVERRIDES_DEFAULT_FIELDS,
            "rust-lang/rust#132162",
        ),
        (
            str_constants::TEST_UNSTABLE_LINT,
            "nightly compiler test-only lint",
        ),
        (
            str_constants::RESOLVING_TO_ITEMS_SHADOWING_SUPERTRAIT_ITEMS,
            "rust-lang/rust#89151",
        ),
        (
            str_constants::SHADOWING_SUPERTRAIT_ITEMS,
            "rust-lang/rust#89151",
        ),
        (
            str_constants::UNQUALIFIED_LOCAL_IMPORTS,
            "rust-lang/rust#138299",
        ),
        (
            str_constants::DEPRECATED_LLVM_INTRINSIC,
            "rust-lang/rust#29602",
        ),
        (
            str_constants::TAIL_CALL_TRACK_CALLER,
            "rust-lang/rust#112788",
        ),
    ];
    let _validated_exceptions = reviewed_exceptions
        .iter()
        .map(|(lint, reason)| {
            assert!(!reason.is_empty(), "829d6e1f");
            assert_eq!(
                probe_lint(str_constants::RUSTC, lint),
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
        super::types::StaticStr::from(str_constants::RUSTC),
        super::types::AnalyzerBool::default(),
        super::types::StaticStr::from(str_constants::VALUE_3C20B457),
        //todo on commit momment seems like this lints still not added to rustc, but in the list of rustc -W help
        super::types::StaticStrSliceRef::from(exceptions.as_slice()),
    );
}
#[test]
fn clippy_lint_exceptions_are_unique() {
    assert_eq!(
        str_constants::CODE_STYLE_CLIPPY_LINT_EXCEPTIONS
            .iter()
            .collect::<std::collections::BTreeSet<_>>()
            .len(),
        str_constants::CODE_STYLE_CLIPPY_LINT_EXCEPTIONS.len(),
        "98ca7133"
    );
}

#[test]
fn lint_probe_distinguishes_supported_unstable_and_unknown_lints() {
    assert_eq!(
        probe_lint(str_constants::CLIPPY_DRIVER, "disallowed_fields"),
        LintProbeDisposition::Supported,
        "6bc218de"
    );
    assert_eq!(
        probe_lint(
            str_constants::RUSTC,
            str_constants::IMPLICIT_PROVENANCE_CASTS
        ),
        LintProbeDisposition::Unstable,
        "e1437af9"
    );
    assert_eq!(
        probe_lint(str_constants::RUSTC, "code_style_nonexistent_lint"),
        LintProbeDisposition::Unknown,
        "907ca5d3"
    );
}
