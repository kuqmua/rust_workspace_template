const PRODUCTION_MODULE_MAX_LINES: usize = 2_500usize;
const INLINE_TEST_SEPARATION_MIN_LINES: usize = 1_024usize;
fn large_module_exceptions() -> [&'static str; 3] {
    [
        "pg_crud_pg_table_generate_src/src/source.rs",
        "pg_crud_pg_types_generate_src/src/source.rs",
        "constants_str/src/lib.rs",
    ]
}

fn is_test_source(path: &std::path::Path) -> bool {
    super::is_test_source_path(super::types::StdPathRef::from(path)).get()
        || path
            .components()
            .any(|component| component.as_os_str() == "test_fixtures")
}

#[allow(
    clippy::single_call_fn,
    reason = "the named AST predicate keeps module ownership detection separate from assertion flow"
)]
fn has_inline_test_module(ast: &syn::File) -> bool {
    ast.items.iter().any(|item| {
        let syn::Item::Mod(module) = item else {
            return false;
        };
        module.ident == "tests"
            && module.content.is_some()
            && module.attrs.iter().any(|attribute| {
                super::attr_is_test_only_cfg(super::types::SynAttributeRef::from(attribute)).get()
            })
    })
}

#[allow(
    clippy::single_call_fn,
    reason = "the named predicate centralizes the exact reviewed exception match"
)]
fn is_large_module_exception(path: &std::path::Path) -> bool {
    large_module_exceptions()
        .iter()
        .any(|exception| path.ends_with(exception))
}

#[test]
fn production_modules_have_bounded_responsibility() {
    super::snapshot::with_codebase_snapshot(|snapshot| {
        let violations = snapshot
            .rs_files()
            .iter()
            .filter(|file| !is_test_source(file.path().as_ref()))
            .filter_map(|file| {
                let line_count = file.content().as_ref().lines().count();
                (line_count > PRODUCTION_MODULE_MAX_LINES
                    && !is_large_module_exception(file.path().as_ref()))
                .then(|| format!("{}: {line_count} lines", file.path().as_ref().display()))
            })
            .collect::<Vec<_>>();
        assert!(
            violations.is_empty(),
            "production modules above {PRODUCTION_MODULE_MAX_LINES} lines must be split by responsibility:\n{}",
            violations.join("\n")
        );
    });
}

#[test]
fn large_production_modules_keep_tests_in_separate_files() {
    super::snapshot::with_codebase_snapshot(|snapshot| {
        let violations = snapshot
            .rs_files()
            .iter()
            .filter(|file| !is_test_source(file.path().as_ref()))
            .filter(|file| {
                file.content().as_ref().lines().count() > INLINE_TEST_SEPARATION_MIN_LINES
            })
            .filter(|file| has_inline_test_module(file.ast().as_ref()))
            .map(|file| file.path().as_ref().display().to_string())
            .collect::<Vec<_>>();
        assert!(
            violations.is_empty(),
            "large production modules must keep tests in separate files:\n{}",
            violations.join("\n")
        );
    });
}

#[test]
#[allow(
    clippy::needless_for_each,
    reason = "the iterator form follows the workspace no-for-loop policy"
)]
fn large_module_exceptions_are_exact_and_still_needed() {
    super::snapshot::with_codebase_snapshot(|snapshot| {
        large_module_exceptions().iter().for_each(|exception| {
            let matching_file = snapshot
                .rs_files()
                .iter()
                .find(|file| file.path().as_ref().ends_with(exception));
            let Some(file) = matching_file else {
                panic!("2d8f4a61 missing large-module exception target: {exception}");
            };
            assert!(
                file.content().as_ref().lines().count() > PRODUCTION_MODULE_MAX_LINES,
                "9a71c5e3 stale large-module exception: {exception}"
            );
        });
    });
}
