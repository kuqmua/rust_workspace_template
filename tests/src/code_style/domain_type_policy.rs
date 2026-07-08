#[test]
fn string_wrappers_do_not_use_from_string() {
    super::assert_rs_ast_ers_empty_with_ctx(
        "e2a6b9c4",
        "string wrappers must validate length; use TryFrom<String> with a length check instead of From<String>:",
        |path, ast, ers| {
            let string_wrapper_names = super::string_wrapper_names(ast);
            let visitor = super::visit_syn_file(
                ast,
                super::StringWrapperFromVisitor {
                    ers: Vec::new(),
                    string_wrapper_names: &string_wrapper_names,
                    try_from_string_names: std::collections::BTreeSet::new(),
                    try_from_string_len_checked_names: std::collections::BTreeSet::new(),
                },
            );
            ers.extend(string_wrapper_names.iter().filter_map(|name| {
                        if visitor.try_from_string_names.contains(name) {
                            None
                        } else {
                            Some(format!(
                                "{}: string wrapper `{name}` must implement `TryFrom<String>` with a length check",
                                path.display()
                            ))
                        }
                    }));
            ers.extend(string_wrapper_names.iter().filter_map(|name| {
                        if visitor.try_from_string_len_checked_names.contains(name) {
                            None
                        } else {
                            Some(format!(
                                "{}: string wrapper `{name}` implements `TryFrom<String>` without a `.len()` check",
                                path.display()
                            ))
                        }
                    }));
            ers.extend(
                visitor
                    .ers
                    .into_iter()
                    .map(|er| format!("{}: {er}", path.display())),
            );
        },
    );
}
#[test]
fn public_tuple_wrappers_do_not_expose_inner_field() {
    super::assert_rs_ast_ers_empty_with_ctx(
        "b7c84e2a",
        "public tuple wrappers must not expose inner fields; initialize them through From/TryFrom:",
        |path, ast, ers| {
            let visitor = super::visit_syn_file(
                ast,
                super::PublicTupleWrapperFieldVisitor { ers: Vec::new() },
            );
            ers.extend(
                visitor
                    .ers
                    .into_iter()
                    .map(|er| format!("{}: {er}", path.display())),
            );
        },
    );
}
#[test]
fn domain_boundaries_use_repository_declared_types() {
    let repo_crates = super::workspace_crate_names();
    let repo_types = super::declared_domain_type_names();
    super::assert_rs_ast_ers_empty_with_ctx(
        "a7f9c3e1",
        "raw external or primitive types found in domain boundaries; use repository domain wrapper types initialized with From/TryFrom:",
        |path, ast, ers| {
            if !super::domain_type_policy_should_check_path(path) {
                return;
            }
            let visitor = super::visit_syn_file(
                ast,
                super::DomainTypePolicyVisitor {
                    ers: Vec::new(),
                    generic_scopes: Vec::new(),
                    repo_crates: &repo_crates,
                    repo_types: &repo_types,
                },
            );
            ers.extend(
                visitor
                    .ers
                    .into_iter()
                    .map(|er| format!("{}: {er}", path.display())),
            );
        },
    );
}
#[test]
fn external_leaf_tuple_wrappers_include_crate_name() {
    let repo_crates = super::workspace_crate_names();
    super::assert_rs_ast_ers_empty_with_ctx(
        "b93d2a8c",
        "tuple wrappers over external types must include the external crate name:",
        |path, ast, ers| {
            if !super::domain_type_policy_should_check_path(path) {
                return;
            }
            let visitor = super::visit_syn_file(
                ast,
                super::ExternalLeafWrapperNameVisitor {
                    ers: Vec::new(),
                    repo_crates: &repo_crates,
                },
            );
            ers.extend(
                visitor
                    .ers
                    .into_iter()
                    .map(|er| format!("{}: {er}", path.display())),
            );
        },
    );
}
