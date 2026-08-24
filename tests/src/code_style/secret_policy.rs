#[derive(optml::Optml, Default)]
struct SecretBoxStringVisitor {
    argument_identifiers: super::types::StdSourceTextSet,
    found_count: super::types::AnalyzerCount,
}
impl<'ast> syn::visit::Visit<'ast> for SecretBoxStringVisitor {
    fn visit_macro(&mut self, i: &'ast syn::Macro) {
        let tokens = i.tokens.to_string();
        if tokens.contains("SecretBox < String >")
            || tokens.contains("SecretBox < std :: string :: String >")
        {
            self.found_count.saturating_inc();
        }
        syn::visit::visit_macro(self, i);
    }

    fn visit_type(&mut self, i: &'ast syn::Type) {
        if let Some(identifier) = type_secret_box_argument_identifier(i) {
            let _was_inserted = self.argument_identifiers.insert(identifier.to_string());
        }
        if type_is_secret_box_string(i) {
            self.found_count.saturating_inc();
        }
        syn::visit::visit_type(self, i);
    }
}
fn type_is_secret_box_string(ty: &syn::Type) -> bool {
    type_secret_box_argument_identifier(ty)
        .is_some_and(|identifier| identifier == stringify!(String))
}
fn type_secret_box_argument_identifier(ty: &syn::Type) -> Option<&syn::Ident> {
    let syn::Type::Path(type_path) = ty else {
        return None;
    };
    let segment = type_path.path.segments.last()?;
    if segment.ident != stringify!(SecretBox) {
        return None;
    }
    let syn::PathArguments::AngleBracketed(arguments) = &segment.arguments else {
        return None;
    };
    arguments.args.iter().find_map(|argument| {
        let syn::GenericArgument::Type(syn::Type::Path(argument_path)) = argument else {
            return None;
        };
        argument_path.path.segments.last().map(|value| &value.ident)
    })
}
#[test]
fn secret_boxes_do_not_use_raw_string_anywhere_in_repository() {
    super::assert_rs_ast_ers_empty_with_ctx(
        super::types::StaticStr::from("6c5a524e"),
        super::types::SourceTextRef::from(
            "SecretBox generic parameters must use a bounded string wrapper",
        ),
        |path, ast, errors| {
            let visitor = super::visit_syn_file(
                super::types::SynFileRef::from(ast),
                SecretBoxStringVisitor::default(),
            );
            super::push_repeated_file_error(
                super::types::DiagnosticMsgsMutRef::from(errors),
                super::types::StdPathRef::from(path),
                super::types::SourceTextRef::from("SecretBox<String>"),
                visitor.found_count,
            );
        },
    );
}
#[test]
fn repository_secret_box_policy_rejects_raw_string_generic_argument() {
    let raw = syn::parse_str::<syn::Type>("secrecy::SecretBox<String>").expect("35a98aea repository_secret_box_policy_rejects_raw_string_generic_argument invariant must hold");
    let qualified =
        syn::parse_str::<syn::Type>("secrecy::SecretBox<std::string::String>").expect("28cd22e6 repository_secret_box_policy_rejects_raw_string_generic_argument invariant must hold");
    let bounded =
        syn::parse_str::<syn::Type>("secrecy::SecretBox<StdAdminString>").expect("f7fc1398 repository_secret_box_policy_rejects_raw_string_generic_argument invariant must hold");
    assert!(type_is_secret_box_string(&raw));
    assert!(type_is_secret_box_string(&qualified));
    assert!(!type_is_secret_box_string(&bounded));
}
#[test]
fn repository_secret_box_policy_checks_generated_tokens() {
    let ast = syn::parse_file(
        "fn generated() { quote::quote! { struct Secret(secrecy::SecretBox<String>); }; }",
    )
    .expect("47bf1cf6 generated invariant must hold");
    let visitor = super::visit_syn_file(
        super::types::SynFileRef::from(&ast),
        SecretBoxStringVisitor::default(),
    );
    assert_eq!(visitor.found_count.get(), 1usize);
}
#[test]
fn repository_secret_boxes_use_bounded_string_types() {
    super::snapshot::with_codebase_snapshot(|snapshot| {
        let bounded_identifiers = snapshot
            .rs_files()
            .iter()
            .flat_map(|source_file| source_file.ast().as_ref().items.iter())
            .filter_map(|item| {
                let syn::Item::Struct(item_struct) = item else {
                    return None;
                };
                item_struct
                    .attrs
                    .iter()
                    .any(|attribute| {
                        super::derive_attr_has_terminal(
                            super::types::SynAttributeRef::from(attribute),
                            super::types::SourceTextRef::from(stringify!(BoundedString)),
                        )
                        .get()
                    })
                    .then(|| item_struct.ident.to_string())
            })
            .collect::<std::collections::BTreeSet<_>>();
        let unbounded_arguments = snapshot
            .rs_files()
            .iter()
            .flat_map(|source_file| {
                super::visit_syn_file(
                    super::types::SynFileRef::from(source_file.ast().as_ref()),
                    SecretBoxStringVisitor::default(),
                )
                .argument_identifiers
                .into_iter()
            })
            .filter(|identifier| !bounded_identifiers.contains(identifier))
            .collect::<Vec<_>>();
        assert!(
            unbounded_arguments.is_empty(),
            "29510826 SecretBox generic parameters are not BoundedString types:\n{}",
            unbounded_arguments.join("\n")
        );
    });
}
