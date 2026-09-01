#[derive(generate_accessor::Getters, optimal_memory_layout::OptimalMemoryLayout, Default)]
struct SecretBoxStringVisitor {
    argument_identifiers: crate::types::SourceTextBTreeSet,
    found_count: crate::types::AnalyzerCount,
}

#[derive(generate_accessor::Getters, optimal_memory_layout::OptimalMemoryLayout, Default)]
struct BoundedStringIdentifierVisitor {
    identifiers: crate::types::SourceTextBTreeSet,
}

impl<'ast> syn::visit::Visit<'ast> for BoundedStringIdentifierVisitor {
    fn visit_item_struct(&mut self, i: &'ast syn::ItemStruct) {
        let derives_bounded_string = i.attrs.iter().any(|attribute| {
            crate::code_style::derive_attr_has_terminal(
                crate::types::SynAttributeRef::from(attribute),
                crate::types::SourceTextRef::from(stringify!(BoundedString)),
            )
            .get()
        });
        let stores_bounded_string = match &i.fields {
            syn::Fields::Unnamed(fields) if fields.unnamed.len() == 1 => {
                fields.unnamed.first().is_some_and(|field| {
                    crate::code_style::type_path_ends_with_identifier(
                        crate::types::SynTypeRef::from(&field.ty),
                        crate::types::SourceTextRef::from(constants_str::BOUNDEDSTRING),
                    )
                    .get()
                })
            }
            syn::Fields::Named(_) | syn::Fields::Unnamed(_) | syn::Fields::Unit => false,
        };
        if derives_bounded_string || stores_bounded_string {
            let _: bool = self.identifiers.insert(i.ident.to_string());
        }
        syn::visit::visit_item_struct(self, i);
    }
}
impl<'ast> syn::visit::Visit<'ast> for SecretBoxStringVisitor {
    fn visit_macro(&mut self, i: &'ast syn::Macro) {
        let tokens = i.tokens.to_string();
        if tokens.contains(constants_str::VALUE_27E33079)
            || tokens.contains(constants_str::VALUE_B7FC9172)
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
fn test_secret_boxes_do_not_use_raw_string_anywhere_in_repository() {
    crate::code_style::assert_rs_ast_ers_empty_with_ctx(
        crate::types::StaticStr::from(constants_str::VALUE_3B1BC5FE),
        crate::types::SourceTextRef::from(constants_str::VALUE_820D50A4),
        |path, ast, errors| {
            let visitor = crate::code_style::visit_syn_file(
                crate::types::SynFileRef::from(ast),
                SecretBoxStringVisitor::default(),
            );
            crate::code_style::push_repeated_file_error(
                crate::types::DiagnosticMsgsMutRef::from(errors),
                crate::types::PathRef::from(path),
                crate::types::SourceTextRef::from(constants_str::VALUE_05D8F7AC),
                *visitor.get_found_count(),
            );
        },
    );
}
#[test]
fn test_repository_secret_box_policy_rejects_raw_string_generic_argument() {
    let raw = syn::parse_str::<syn::Type>(constants_str::VALUE_02D2E24C)
        .expect(constants_str::DIAGNOSTIC_35A98AEA);
    let qualified = syn::parse_str::<syn::Type>(constants_str::VALUE_171D86A4)
        .expect(constants_str::DIAGNOSTIC_28CD22E6);
    let bounded = syn::parse_str::<syn::Type>(constants_str::VALUE_5BF4FAD8)
        .expect(constants_str::DIAGNOSTIC_F7FC1398);
    assert!(type_is_secret_box_string(&raw));
    assert!(type_is_secret_box_string(&qualified));
    assert!(!type_is_secret_box_string(&bounded));
}
#[test]
fn test_repository_secret_box_policy_checks_generated_tokens() {
    let ast =
        syn::parse_file(constants_str::VALUE_53E9A56F).expect(constants_str::DIAGNOSTIC_47BF1CF6);
    let visitor = crate::code_style::visit_syn_file(
        crate::types::SynFileRef::from(&ast),
        SecretBoxStringVisitor::default(),
    );
    assert_eq!(visitor.get_found_count().get(), constants_usize::ONE);
}
#[test]
fn test_repository_secret_boxes_use_bounded_string_types() {
    super::test_code_style_snapshot::with_codebase_snapshot(|snapshot| {
        let bounded_identifiers = snapshot
            .rs_files()
            .iter()
            .flat_map(|source_file| {
                crate::code_style::visit_syn_file(
                    crate::types::SynFileRef::from(source_file.ast().as_ref()),
                    BoundedStringIdentifierVisitor::default(),
                )
                .identifiers
                .into_iter()
            })
            .collect::<std::collections::BTreeSet<_>>();
        let unbounded_arguments = snapshot
            .rs_files()
            .iter()
            .flat_map(|source_file| {
                crate::code_style::visit_syn_file(
                    crate::types::SynFileRef::from(source_file.ast().as_ref()),
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
