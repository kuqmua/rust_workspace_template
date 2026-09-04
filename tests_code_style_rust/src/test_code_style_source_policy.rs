#[derive(
    proc_macro_getters::Getters, Default, proc_macro_optimal_memory_layout::OptimalMemoryLayout,
)]
struct HandwrittenFieldGetterVisitor {
    violations: crate::types::SourceTextList,
}

#[derive(
    proc_macro_getters::Getters, Default, proc_macro_optimal_memory_layout::OptimalMemoryLayout,
)]
struct ModuleWideSingleCallAllowVisitor {
    violations: crate::types::SourceTextList,
}

#[derive(
    proc_macro_getters::Getters, Default, proc_macro_optimal_memory_layout::OptimalMemoryLayout,
)]
struct DuplicateConfigurationTestVisitor {
    violations: crate::types::DiagnosticMessages,
}

#[derive(
    proc_macro_getters::Getters, Default, proc_macro_optimal_memory_layout::OptimalMemoryLayout,
)]
struct EmptyModuleVisitor {
    violations: crate::types::DiagnosticMessages,
}

#[derive(
    proc_macro_getters::Getters, Default, proc_macro_optimal_memory_layout::OptimalMemoryLayout,
)]
struct ConversionInputNameVisitor {
    violations: crate::types::DiagnosticMessages,
}

#[derive(
    proc_macro_getters::Getters, Default, proc_macro_optimal_memory_layout::OptimalMemoryLayout,
)]
struct EmptyFunctionBodyVisitor {
    violations: crate::types::DiagnosticMessages,
}

impl EmptyFunctionBodyVisitor {
    fn record(&mut self, identifier: &syn::Ident, block: &syn::Block) {
        if block.stmts.is_empty() {
            self.violations.push(format!(
                "line {}: empty function `{identifier}`",
                syn::spanned::Spanned::span(identifier).start().line
            ));
        }
    }
}

impl<'ast_lt> syn::visit::Visit<'ast_lt> for EmptyFunctionBodyVisitor {
    fn visit_impl_item_fn(&mut self, impl_item_fn: &'ast_lt syn::ImplItemFn) {
        let syn::ImplItemFn { sig, block, .. } = impl_item_fn;
        self.record(&sig.ident, block);
        syn::visit::visit_impl_item_fn(self, impl_item_fn);
    }

    fn visit_item_fn(&mut self, item_fn: &'ast_lt syn::ItemFn) {
        if item_fn.block.stmts.is_empty() {
            self.violations.push(format!(
                "line {}: empty function `{}`",
                syn::spanned::Spanned::span(&item_fn.sig.ident).start().line,
                item_fn.sig.ident
            ));
        }
        syn::visit::visit_item_fn(self, item_fn);
    }

    fn visit_trait_item_fn(&mut self, trait_item_fn: &'ast_lt syn::TraitItemFn) {
        if let Some(block) = &trait_item_fn.default {
            self.record(&trait_item_fn.sig.ident, block);
        }
        syn::visit::visit_trait_item_fn(self, trait_item_fn);
    }
}

#[derive(
    proc_macro_getters::Getters, Default, proc_macro_optimal_memory_layout::OptimalMemoryLayout,
)]
struct GeneratedPublicUseVisitor {
    violations: crate::types::DiagnosticMessages,
}

impl<'ast_lt> syn::visit::Visit<'ast_lt> for GeneratedPublicUseVisitor {
    fn visit_lit_str(&mut self, lit_str: &'ast_lt syn::LitStr) {
        if lit_str.value().lines().any(|line| {
            line.trim_start()
                .starts_with(constants_str::PUBLIC_USE_PREFIX)
        }) {
            self.violations.push(format!(
                "line {}: generated Rust source contains a public re-export",
                syn::spanned::Spanned::span(lit_str).start().line
            ));
        }
        syn::visit::visit_lit_str(self, lit_str);
    }
}

impl<'ast_lt> syn::visit::Visit<'ast_lt> for DuplicateConfigurationTestVisitor {
    fn visit_item(&mut self, item: &'ast_lt syn::Item) {
        if crate::code_style::cfg_test_attr_count(crate::types::SynItemRef::from(item))
            > constants_usize::ONE
        {
            self.violations.push(format!(
                "line {}: duplicate #[cfg(test)] attributes",
                syn::spanned::Spanned::span(item).start().line
            ));
        }
        syn::visit::visit_item(self, item);
    }
}

impl<'ast_lt> syn::visit::Visit<'ast_lt> for EmptyModuleVisitor {
    fn visit_file(&mut self, file: &'ast_lt syn::File) {
        if file.items.is_empty() {
            self.violations
                .push(String::from(constants_str::EMPTY_MODULE_SOURCE_FILE));
        }
        syn::visit::visit_file(self, file);
    }

    fn visit_item_mod(&mut self, item_mod: &'ast_lt syn::ItemMod) {
        if item_mod
            .content
            .as_ref()
            .is_some_and(|(_brace, items)| items.is_empty())
        {
            self.violations.push(format!(
                "line {}: empty module `{}`",
                syn::spanned::Spanned::span(item_mod).start().line,
                item_mod.ident
            ));
        }
        syn::visit::visit_item_mod(self, item_mod);
    }
}

impl<'ast_lt> syn::visit::Visit<'ast_lt> for ConversionInputNameVisitor {
    fn visit_item_impl(&mut self, item_impl: &'ast_lt syn::ItemImpl) {
        let Some(trait_identifier) = item_impl
            .trait_
            .as_ref()
            .and_then(|(path, _)| path.segments.last())
            .map(|segment| &segment.ident)
        else {
            syn::visit::visit_item_impl(self, item_impl);
            return;
        };
        if trait_identifier != constants_str::FROM_ALT_3
            && trait_identifier != constants_str::TRYFROM
        {
            syn::visit::visit_item_impl(self, item_impl);
            return;
        }
        item_impl.items.iter().for_each(|item| {
            let syn::ImplItem::Fn(function) = item else {
                return;
            };
            if function.sig.ident != constants_str::CODE_STYLE_FROM_FN_IDENTIFIER
                && function.sig.ident != constants_str::NEWTYPE_TRY_FROM
            {
                return;
            }
            let Some(syn::FnArg::Typed(argument)) = function.sig.inputs.first() else {
                return;
            };
            if !matches!(argument.pat.as_ref(), syn::Pat::Ident(identifier) if identifier.ident == constants_str::CODE_STYLE_VALUE_IDENTIFIER)
            {
                self.violations.push(format!(
                    "line {}: {} input parameter must be named `value`",
                    syn::spanned::Spanned::span(argument).start().line,
                    trait_identifier
                ));
            }
        });
        syn::visit::visit_item_impl(self, item_impl);
    }
}

impl<'ast_lt> syn::visit::Visit<'ast_lt> for ModuleWideSingleCallAllowVisitor {
    fn visit_attribute(&mut self, attribute: &'ast_lt syn::Attribute) {
        if matches!(&attribute.style, syn::AttrStyle::Inner(_))
            && attribute.path().is_ident(constants_str::VALUE_41008373)
            && matches!(&attribute.meta, syn::Meta::List(list) if list
                .tokens
                .to_string()
                .split_whitespace()
                .collect::<String>()
                .contains(constants_str::SHARED_VALUES_CLIPPY_SINGLE_CALL_FN))
        {
            self.violations.push(format!(
                "line {}: clippy::single_call_fn must be allowed only on the exact item",
                syn::spanned::Spanned::span(attribute).start().line
            ));
        }
        syn::visit::visit_attribute(self, attribute);
    }
}

impl<'ast_lt> syn::visit::Visit<'ast_lt> for HandwrittenFieldGetterVisitor {
    fn visit_item_impl(&mut self, item_impl: &'ast_lt syn::ItemImpl) {
        if item_impl.trait_.is_none() {
            item_impl.items.iter().for_each(|item| {
                let syn::ImplItem::Fn(method) = item else {
                    return;
                };
                if method.sig.inputs.len() != constants_usize::ONE {
                    return;
                }
                let has_getter_prefix = method
                    .sig
                    .ident
                    .to_string()
                    .starts_with(constants_str::GETTER_PREFIX);
                let returned_field = method.block.stmts.first().and_then(|statement| {
                    if method.block.stmts.len() != constants_usize::ONE {
                        return None;
                    }
                    let syn::Stmt::Expr(expression, None) = statement else {
                        return None;
                    };
                    let returned_expression = if let syn::Expr::Reference(reference) = expression {
                        reference.expr.as_ref()
                    } else {
                        expression
                    };
                    let syn::Expr::Field(field) = returned_expression else {
                        return None;
                    };
                    let syn::Expr::Path(base) = field.base.as_ref() else {
                        return None;
                    };
                    (base.path.is_ident(constants_str::SELF_ALT)).then_some(&field.member)
                });
                let directly_returns_named_field = returned_field.is_some_and(|member| {
                    matches!(member, syn::Member::Named(field) if field == &method.sig.ident)
                });
                let copies_generated_inner_getter = method.sig.ident == constants_str::GET_ALT
                    && method.block.stmts.len() == constants_usize::ONE
                    && method.block.stmts.first().is_some_and(|statement| {
                        let syn::Stmt::Expr(syn::Expr::Unary(unary), None) = statement else {
                            return false;
                        };
                        let syn::UnOp::Deref(_) = unary.op else {
                            return false;
                        };
                        let syn::Expr::MethodCall(method_call) = unary.expr.as_ref() else {
                            return false;
                        };
                        method_call.method == constants_str::GET_INNER
                            && method_call.args.is_empty()
                            && matches!(method_call.receiver.as_ref(), syn::Expr::Path(receiver) if receiver.path.is_ident(constants_str::SELF_ALT))
                    });
                if has_getter_prefix
                    || directly_returns_named_field
                    || copies_generated_inner_getter
                {
                    self.violations.push(method.sig.ident.to_string());
                }
            });
        }
        syn::visit::visit_item_impl(self, item_impl);
    }
}

#[test]
fn test_cfg_test_attribute_is_not_duplicated() {
    crate::code_style::assert_rs_ast_errors_empty_with_context(
        crate::types::StaticStr::from(constants_str::F4ECA965),
        crate::types::SourceTextRef::from(constants_str::DUPLICATE_CFG_TEST_ATTRIBUTES),
        |path, ast, errors| {
            let visitor = crate::code_style::visit_syn_file(
                crate::types::SynFileRef::from(ast),
                DuplicateConfigurationTestVisitor::default(),
            );
            errors.extend(
                visitor
                    .get_violations()
                    .clone()
                    .into_iter()
                    .map(|violation| format!("{}: {violation}", path.display())),
            );
        },
    );
}

#[test]
fn test_cfg_test_attribute_policy_rejects_a_duplicate() {
    let ast: syn::File = syn::parse_quote! {
        #[cfg(test)]
        #[cfg(test)]
        mod test_example {}
    };
    assert_eq!(
        crate::code_style::cfg_test_attr_count(crate::types::SynItemRef::from(
            ast.items.first().expect(constants_str::DIAGNOSTIC_ED9CAF91),
        )),
        constants_usize::TWO
    );
    let visitor = crate::code_style::visit_syn_file(
        crate::types::SynFileRef::from(&ast),
        DuplicateConfigurationTestVisitor::default(),
    );
    assert_eq!(visitor.get_violations().len(), constants_usize::ONE);
}

#[test]
fn test_empty_modules_are_forbidden() {
    crate::code_style::assert_rs_ast_errors_empty_with_context(
        crate::types::StaticStr::from(constants_str::A7C19E42),
        crate::types::SourceTextRef::from(constants_str::EMPTY_MODULES_ARE_FORBIDDEN),
        |path, ast, errors| {
            let visitor = crate::code_style::visit_syn_file(
                crate::types::SynFileRef::from(ast),
                EmptyModuleVisitor::default(),
            );
            errors.extend(
                visitor
                    .get_violations()
                    .clone()
                    .into_iter()
                    .map(|violation| format!("{}: {violation}", path.display())),
            );
        },
    );
}

#[test]
fn test_empty_module_policy_rejects_an_inline_module_without_items() {
    let ast: syn::File = syn::parse_quote! {
        mod test_example {}
    };
    let visitor = crate::code_style::visit_syn_file(
        crate::types::SynFileRef::from(&ast),
        EmptyModuleVisitor::default(),
    );
    assert_eq!(visitor.get_violations().len(), constants_usize::ONE);
}

#[test]
fn test_empty_module_policy_rejects_a_source_file_without_items() {
    let ast = syn::parse_file(constants_str::EMPTY).expect(constants_str::DIAGNOSTIC_AE2E1C74);
    let visitor = crate::code_style::visit_syn_file(
        crate::types::SynFileRef::from(&ast),
        EmptyModuleVisitor::default(),
    );
    assert_eq!(visitor.get_violations().len(), constants_usize::ONE);
}

#[test]
fn test_from_and_try_from_input_parameters_are_named_value() {
    super::test_code_style_snapshot::with_codebase_snapshot(|snapshot| {
        let violations = snapshot
            .rs_files()
            .iter()
            .flat_map(|source_file| {
                let mut visitor = ConversionInputNameVisitor::default();
                syn::visit::Visit::visit_file(&mut visitor, source_file.ast().as_ref());
                visitor
                    .get_violations()
                    .clone()
                    .into_iter()
                    .map(|violation| {
                        format!("{}:{violation}", source_file.path().as_ref().display())
                    })
            })
            .collect::<Vec<String>>();
        crate::code_style::assert_joined_errors_empty(
            crate::types::SourceTextListRef::from(violations.as_slice()),
            crate::types::StaticStr::from(
                constants_str::FROM_AND_TRY_FROM_INPUT_PARAMETERS_MUST_BE_NAMED_VALUE,
            ),
        );
    });
}

#[test]
fn test_from_and_try_from_input_parameter_policy_rejects_nonstandard_names() {
    let ast: syn::File = syn::parse_quote! {
        impl From<u8> for Example {
            fn from(input: u8) -> Self {
                Self(input)
            }
        }
        impl TryFrom<u16> for Example {
            type Error = Error;
            fn try_from(raw: u16) -> Result<Self, Self::Error> {
                Ok(Self(raw))
            }
        }
    };
    let visitor = crate::code_style::visit_syn_file(
        crate::types::SynFileRef::from(&ast),
        ConversionInputNameVisitor::default(),
    );
    assert_eq!(visitor.get_violations().len(), constants_usize::TWO);
}

#[test]
fn test_single_call_fn_is_never_allowed_for_a_whole_module() {
    super::test_code_style_snapshot::with_codebase_snapshot(|snapshot| {
        let violations = snapshot
            .rs_files()
            .iter()
            .flat_map(|source_file| {
                let mut visitor = ModuleWideSingleCallAllowVisitor::default();
                syn::visit::Visit::visit_file(&mut visitor, source_file.ast().as_ref());
                visitor
                    .get_violations()
                    .clone()
                    .into_iter()
                    .map(|violation| {
                        format!("{}:{violation}", source_file.path().as_ref().display())
                    })
            })
            .collect::<Vec<String>>();
        crate::code_style::assert_joined_errors_empty(
            crate::types::SourceTextListRef::from(violations.as_slice()),
            crate::types::StaticStr::from(
                constants_str::CODE_STYLE_SINGLE_CALL_FN_ITEM_SCOPE_REASON,
            ),
        );
    });
}

#[test]
fn test_field_getters_are_generated() {
    super::test_code_style_snapshot::with_codebase_snapshot(|snapshot| {
        let violations = snapshot
            .rs_files()
            .iter()
            .flat_map(|source_file| {
                let mut visitor = HandwrittenFieldGetterVisitor::default();
                syn::visit::Visit::visit_file(&mut visitor, source_file.ast().as_ref());
                visitor.get_violations().clone().into_iter().map(|method| {
                    format!(
                        "{} contains handwritten field getter `{method}`; derive proc_macro_getters::Getters",
                        source_file.path().as_ref().display()
                    )
                })
            })
            .collect::<Vec<String>>();
        assert!(violations.is_empty(), "{violations:#?}");
    });
}

#[test]
fn test_field_getter_policy_rejects_copying_a_generated_inner_getter() {
    let ast: syn::File = syn::parse_quote! {
        struct Example(u64);
        impl Example {
            const fn get(self) -> u64 {
                *self.get_inner()
            }
        }
    };
    let visitor = crate::code_style::visit_syn_file(
        crate::types::SynFileRef::from(&ast),
        HandwrittenFieldGetterVisitor::default(),
    );
    assert_eq!(visitor.get_violations().len(), constants_usize::ONE);
}

#[test]
fn test_struct_fields_are_private_without_exceptions() {
    super::test_code_style_snapshot::with_codebase_snapshot(|snapshot| {
        let violations = snapshot
            .rs_files()
            .iter()
            .flat_map(|source_file| {
                let mut visitor = super::source_analysis::PublicStructFieldVisitor::default();
                syn::visit::Visit::visit_file(&mut visitor, source_file.ast().as_ref());
                visitor.get_violations().clone().into_iter().map(|item| {
                    format!(
                        "{} exposes a non-private struct field in {item}; keep every field private and expose access through a generated getter",
                        source_file.path().as_ref().display()
                    )
                })
            })
            .collect::<Vec<String>>();
        assert!(violations.is_empty(), "{violations:#?}");
    });
}

#[test]
fn test_generated_struct_fields_are_private_without_exceptions() {
    super::test_code_style_snapshot::with_codebase_snapshot(|snapshot| {
        let violations = snapshot
            .rs_files()
            .iter()
            .flat_map(|source_file| {
                let mut visitor =
                    super::source_analysis::GeneratedPublicStructFieldVisitor::default();
                syn::visit::Visit::visit_file(&mut visitor, source_file.ast().as_ref());
                visitor.get_violations().clone().into_iter().map(|item| {
                    format!(
                        "{} generates a non-private struct field in `{item}`",
                        source_file.path().as_ref().display()
                    )
                })
            })
            .collect::<Vec<String>>();
        assert!(violations.is_empty(), "{violations:#?}");
    });
}

#[test]
fn test_provider_traits_do_not_use_get_prefix() {
    let pattern =
        regex::Regex::new(constants_str::VALUE_B2BAA955).expect(constants_str::DIAGNOSTIC_CBE7BF15);
    let mut errors = Vec::new();
    crate::code_style::for_each_rs_file(|file| {
        pattern
            .captures_iter(file.content().as_ref())
            .for_each(|captures| {
                let Some(name) = captures.get(1usize).map(|value| value.as_str()) else {
                    return;
                };
                errors.push(format!(
                    "{}: provider trait `{name}` must use the `Provider` suffix",
                    file.path().as_ref().display()
                ));
            });
    });
    errors.sort();
    crate::code_style::assert_joined_errors_empty(
        crate::types::SourceTextListRef::from(errors.as_slice()),
        crate::types::StaticStr::from(constants_str::VALUE_669E43DB),
    );
}

#[test]
fn test_all_files_are_english_only() {
    let mut errors = super::test_code_style_snapshot::with_codebase_snapshot(|snapshot| {
        rayon::iter::ParallelIterator::reduce(
            rayon::iter::ParallelIterator::map(
                rayon::iter::IntoParallelRefIterator::par_iter(snapshot.project_source_files()),
                |source_file| {
                    source_file
                        .content()
                        .as_ref()
                        .lines()
                        .enumerate()
                        .flat_map(|(line_index, line)| {
                            let line_number = line_index.saturating_add(1usize);
                            line.chars()
                                .filter(|ch| {
                                    !matches!(ch, '\n' | '\r' | '\t' | '\u{2014}' | '\u{2194}')
                                        && !ch.is_ascii()
                                })
                                .map(move |ch| {
                                    format!(
                                        "{}:{} non-english symbol `{}` (U+{:04X})",
                                        source_file.path().as_ref().display(),
                                        line_number,
                                        ch,
                                        u32::from(ch)
                                    )
                                })
                        })
                        .collect::<Vec<String>>()
                },
            ),
            Vec::new,
            |mut accumulator, mut item| {
                accumulator.append(&mut item);
                accumulator
            },
        )
    });
    errors.sort();
    crate::code_style::assert_joined_errors_empty_with_context(
        crate::types::SourceTextListRef::from(errors.as_slice()),
        crate::types::StaticStr::from(constants_str::VALUE_8DB37A2F),
        crate::types::SourceTextRef::from(constants_str::NON_ENGLISH_SYMBOLS),
    );
}
#[test]
fn test_expect_and_panic_messages_start_with_unique_diagnostic_ids() {
    let mut all_ids = Vec::new();
    let mut all_errors = Vec::new();
    crate::code_style::for_each_rs_file(|file| {
        let (path, ast) = (file.path().as_ref(), file.ast().as_ref());
        let visitor = crate::code_style::visit_syn_file(
            crate::types::SynFileRef::from(ast),
            super::source_analysis::DiagnosticIdVisitor::new(
                crate::types::DiagnosticMessages::default(),
                crate::types::SourceTextList::default(),
            ),
        );
        all_ids.extend(visitor.get_ids().iter().cloned());
        all_errors.extend(
            visitor
                .get_errors()
                .iter()
                .map(|error| format!("{path:?}: {error}")),
        );
    });
    let mut seen = std::collections::HashSet::new();
    let duplicates = all_ids
        .iter()
        .filter(|identifier| !seen.insert(identifier.as_str()))
        .cloned()
        .collect::<Vec<String>>();
    if !duplicates.is_empty() {
        all_errors.push(format!("duplicate UUIDs found: {duplicates:?}"));
    }
    assert!(all_errors.is_empty(), "6062a9e9 {all_errors:#?}");
}
#[test]
fn test_diagnostic_id_visitor_checks_expect_methods_and_panic_macros() {
    let ast =
        syn::parse_file(constants_str::VALUE_D1E0CA47).expect(constants_str::DIAGNOSTIC_95D174AC);
    let visitor = crate::code_style::visit_syn_file(
        crate::types::SynFileRef::from(&ast),
        super::source_analysis::DiagnosticIdVisitor::new(
            crate::types::DiagnosticMessages::default(),
            crate::types::SourceTextList::default(),
        ),
    );
    assert!(visitor.get_errors().is_empty());
    assert_eq!(
        visitor.get_ids().as_slice(),
        [
            String::from(constants_str::VALUE_0A05D7B2),
            String::from(constants_str::VALUE_371EE848)
        ]
    );

    let invalid_ast =
        syn::parse_file(constants_str::VALUE_BFBFB833).expect(constants_str::DIAGNOSTIC_6C3A48F1);
    let invalid_visitor = crate::code_style::visit_syn_file(
        crate::types::SynFileRef::from(&invalid_ast),
        super::source_analysis::DiagnosticIdVisitor::new(
            crate::types::DiagnosticMessages::default(),
            crate::types::SourceTextList::default(),
        ),
    );
    assert_eq!(invalid_visitor.get_errors().len(), 3usize);
}
#[test]
fn test_diagnostic_id_visitor_checks_generated_expect_and_panic_tokens() {
    let ast =
        syn::parse_file(constants_str::VALUE_38F6372C).expect(constants_str::DIAGNOSTIC_227C291C);
    let visitor = crate::code_style::visit_syn_file(
        crate::types::SynFileRef::from(&ast),
        super::source_analysis::DiagnosticIdVisitor::new(
            crate::types::DiagnosticMessages::default(),
            crate::types::SourceTextList::default(),
        ),
    );
    assert_eq!(visitor.get_ids().len(), 2usize);
    assert_eq!(visitor.get_errors().len(), 3usize);
}
#[test]
fn test_check_rs_files_contains_only_unique_uuid_v4() {
    let regex = regex::Regex::new(constants_str::B_0_9A_FA_F_8_0_9A_FA_F_4_4)
        .expect(constants_str::DIAGNOSTIC_E098A1FF);
    let mut seen = std::collections::HashSet::new();
    crate::code_style::for_each_rs_file(|file| {
        let v = file.content().as_ref();
        regex.find_iter(v).for_each(|element_714b3d9c| {
            let uuid = uuid::Uuid::parse_str(element_714b3d9c.as_str())
                .expect(constants_str::DIAGNOSTIC_C9711EFD);
            assert!(uuid.get_version_num() == 4, "49b49b21");
            assert!(seen.insert(uuid), "4cf9d239");
        });
    });
}
#[test]
fn test_no_dbg_macro_in_source_code() {
    crate::code_style::assert_rs_ast_errors_empty_with_context(
        crate::types::StaticStr::from(constants_str::F1C7A4E3),
        crate::types::SourceTextRef::from(constants_str::DBG_FOUND),
        |path, ast, errors| {
            let visitor = crate::code_style::visit_syn_file(
                crate::types::SynFileRef::from(ast),
                super::source_analysis::DbgVisitor::new(crate::types::AnalyzerBool::default()),
            );
            if visitor.get_found().get() {
                errors.push(format!("{}: contains dbg!()", path.display()));
            }
        },
    );
}
#[test]
fn test_no_for_loops_in_source_code() {
    crate::code_style::assert_rs_ast_errors_empty_with_context(
        crate::types::StaticStr::from(constants_str::F4C2A9E1),
        crate::types::SourceTextRef::from(
            constants_str::FOR_LOOPS_FOUND_USE_ITERATOR_METHODS_SUCH_AS_MAP_FILTER_FOLD_TRY,
        ),
        |path, ast, errors| {
            let visitor = crate::code_style::visit_syn_file(
                crate::types::SynFileRef::from(ast),
                super::source_analysis::ForLoopVisitor::new(crate::types::AnalyzerCount::default()),
            );
            crate::code_style::push_repeated_file_error(
                crate::types::DiagnosticMessagesMutRef::from(&mut *errors),
                crate::types::PathRef::from(path),
                crate::types::SourceTextRef::from(
                    constants_str::CONTAINS_FOR_LOOP_USE_ITERATOR_METHODS_INSTEAD,
                ),
                *visitor.get_found_count(),
            );
        },
    );
}

#[test]
fn test_map_err_does_not_discard_source_with_wildcard() {
    super::test_code_style_snapshot::with_codebase_snapshot(|snapshot| {
        let violations = snapshot
            .rs_files()
            .iter()
            .filter_map(|source_file| {
                let mut visitor = super::source_analysis::SourceDroppingMapErrVisitor::default();
                syn::visit::Visit::visit_file(&mut visitor, source_file.ast().as_ref());
                (visitor.get_found_count().get() != constants_usize::ZERO).then(|| {
                    format!(
                        "{} discards a map_err source with a wildcard",
                        source_file.path().as_ref().display()
                    )
                })
            })
            .collect::<Vec<String>>();
        assert!(violations.is_empty(), "{violations:#?}");
    });
}

#[test]
fn test_workspace_crate_sources_do_not_use_include_macro() {
    #[derive(Default, proc_macro_optimal_memory_layout::OptimalMemoryLayout)]
    struct IncludeMacroVisitor {
        lines: Vec<usize>,
    }
    impl<'ast> syn::visit::Visit<'ast> for IncludeMacroVisitor {
        fn visit_macro(&mut self, i: &'ast syn::Macro) {
            if i.path.is_ident(stringify!(include)) {
                self.lines
                    .push(syn::spanned::Spanned::span(&i.path).start().line);
            }
            syn::visit::visit_macro(self, i);
        }
    }
    super::test_code_style_snapshot::with_codebase_snapshot(|snapshot| {
        let violations = snapshot
            .rs_files()
            .iter()
            .filter_map(|file| {
                let mut visitor = IncludeMacroVisitor::default();
                syn::visit::Visit::visit_file(&mut visitor, file.ast().as_ref());
                (!visitor.lines.is_empty())
                    .then(|| format!("{}: {:?}", file.path().as_ref().display(), visitor.lines))
            })
            .collect::<Vec<_>>();
        assert!(
            violations.is_empty(),
            "646fbd75 include macros are forbidden in workspace crate sources:\n{}",
            violations.join("\n")
        );
    });
}

#[test]
fn test_numeric_conversions_do_not_use_as_casts() {
    super::test_code_style_snapshot::with_codebase_snapshot(|snapshot| {
        let violations = snapshot
            .rs_files()
            .iter()
            .filter_map(|source_file| {
                let mut visitor = super::source_analysis::NumericAsCastVisitor::default();
                syn::visit::Visit::visit_file(&mut visitor, source_file.ast().as_ref());
                (visitor.get_found_count().get() != constants_usize::ZERO).then(|| {
                    format!(
                        "{} contains {} numeric as cast(s)",
                        source_file.path().as_ref().display(),
                        visitor.get_found_count().get()
                    )
                })
            })
            .collect::<Vec<String>>();
        assert!(violations.is_empty(), "{violations:#?}");
    });
}

#[test]
fn test_runtime_struct_fields_do_not_expose_untyped_json_values() {
    super::test_code_style_snapshot::with_codebase_snapshot(|snapshot| {
        let violations = snapshot
            .rs_files()
            .iter()
            .flat_map(|source_file| {
                let mut visitor = super::source_analysis::SerdeJsonValueFieldVisitor::default();
                syn::visit::Visit::visit_file(&mut visitor, source_file.ast().as_ref());
                visitor.get_violations().clone().into_iter().map(|item| {
                    format!(
                        "{} exposes serde_json::Value in {item}",
                        source_file.path().as_ref().display()
                    )
                })
            })
            .collect::<Vec<String>>();
        assert!(violations.is_empty(), "{violations:#?}");
    });
}

#[test]
fn test_struct_field_visibility_policy_rejects_restricted_visibility() {
    let ast = syn::parse_file(constants_str::CODE_STYLE_STRUCT_FIELD_VISIBILITY_FIXTURE)
        .expect(constants_str::DIAGNOSTIC_8C99DE4E);
    let mut visitor = super::source_analysis::PublicStructFieldVisitor::default();
    syn::visit::Visit::visit_file(&mut visitor, &ast);
    assert_eq!(
        visitor.get_violations().as_slice(),
        [
            constants_str::VALUE_0BC21577,
            constants_str::VALUE_1FEF7771,
            constants_str::VALUE_0732DC65,
            constants_str::VALUE_0AA2EAA9,
            constants_str::VALUE_1CAC5DE9,
            constants_str::VALUE_354F1DC6,
        ],
        "e69e2e99"
    );
}

#[test]
fn test_generated_struct_field_visibility_policy_rejects_every_public_form() {
    let ast = syn::parse_file(constants_str::CODE_STYLE_GENERATED_STRUCT_FIELD_VISIBILITY_FIXTURE)
        .expect(constants_str::DIAGNOSTIC_77DE048C);
    let mut visitor = super::source_analysis::GeneratedPublicStructFieldVisitor::default();
    syn::visit::Visit::visit_file(&mut visitor, &ast);
    assert_eq!(visitor.get_violations().len(), 5usize, "85b3fba7");
}
#[test]
fn test_spawned_tasks_must_retain_an_owner() {
    crate::code_style::assert_rs_ast_errors_empty_with_context(
        crate::types::StaticStr::from(constants_str::VALUE_5D0D5BF0),
        crate::types::SourceTextRef::from(constants_str::SPAWNED_TASKS_ARE_DISCARDED),
        |path, ast, errors| {
            let visitor = crate::code_style::visit_syn_file(
                crate::types::SynFileRef::from(ast),
                super::source_analysis::LostSpawnVisitor::new(
                    crate::types::DiagnosticMessages::default(),
                ),
            );
            errors.extend(
                visitor
                    .get_errors()
                    .clone()
                    .into_iter()
                    .map(|error| format!("{}: {error}", path.display())),
            );
        },
    );
}
#[test]
fn test_spawned_task_policy_rejects_bare_wildcard_and_ignored_bindings() {
    let ast =
        syn::parse_file(constants_str::VALUE_EBB24851).expect(constants_str::DIAGNOSTIC_94B344D7);
    let visitor = crate::code_style::visit_syn_file(
        crate::types::SynFileRef::from(&ast),
        super::source_analysis::LostSpawnVisitor::new(crate::types::DiagnosticMessages::default()),
    );
    assert_eq!(visitor.get_errors().len(), 4usize);
}
#[test]
fn test_direct_environment_and_filesystem_access_stays_at_owned_boundaries() {
    crate::code_style::assert_rs_ast_errors_empty_with_context(
        crate::types::StaticStr::from(constants_str::VALUE_321360D4),
        crate::types::SourceTextRef::from(constants_str::DIRECT_ENVIRONMENT_OR_FILESYSTEM_ACCESS_EXISTS_OUTSIDE_APPROVED_CONFIGURATION_TOOLING_TEST_AND),
        |path, ast, errors| {
            if crate::code_style::is_test_crate_source_path(crate::types::PathRef::from(path)).get()
                || crate::code_style::is_cfg_test_declared_child(path)
                || crate::code_style::is_direct_fs_owner_source_path(crate::types::PathRef::from(path)).get()
                || constants_str::CODE_STYLE_BOUNDED_READ_OWNER_SUFFIXES
                    .iter()
                    .any(|suffix| path.ends_with(suffix))
            {
                return;
            }
            let visitor = crate::code_style::visit_syn_file(
                crate::types::SynFileRef::from(ast),
                super::source_analysis::DirectPathCallVisitor::new(crate::types::DiagnosticMessages::default()),
            );
            errors.extend(visitor.get_calls().clone().into_iter().filter_map(|call| {
                (call.starts_with(constants_str::STD_PATH_ENV_PATH)
                    || call.starts_with(constants_str::STD_PATH_FS_PATH)
                    || call.starts_with(constants_str::TOKIO_PATH_FS_PATH))
                .then(|| format!("{}: direct `{call}`", path.display()))
            }));
        },
    );
}
#[test]
fn test_direct_filesystem_owner_inventory_is_exact_justified_and_current() {
    assert_eq!(
        constants_str::CODE_STYLE_DIRECT_FS_OWNER_SUFFIXES.len(),
        constants_str::CODE_STYLE_DIRECT_FS_OWNER_REASONS.len(),
        "6e1a9c30"
    );
    let workspace_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect(constants_str::DIAGNOSTIC_4D82F1B7);
    let missing_or_unjustified = constants_str::CODE_STYLE_DIRECT_FS_OWNER_SUFFIXES
        .iter()
        .zip(constants_str::CODE_STYLE_DIRECT_FS_OWNER_REASONS)
        .filter(|(suffix, reason)| {
            let reviewed_path = workspace_root.join(suffix.trim_start_matches('/'));
            let split_owner_exists = reviewed_path
                .file_stem()
                .and_then(std::ffi::OsStr::to_str)
                .zip(reviewed_path.parent())
                .is_some_and(|(stem, parent)| parent.join(stem).is_dir());
            reason.trim().is_empty() || (!reviewed_path.is_file() && !split_owner_exists)
        })
        .collect::<Vec<(&&str, &str)>>();
    assert!(
        missing_or_unjustified.is_empty(),
        "c70b25ea {missing_or_unjustified:?}"
    );
    super::test_code_style_snapshot::with_codebase_snapshot(|snapshot| {
        let mut matched = std::collections::BTreeSet::new();
        snapshot
            .rs_files()
            .iter()
            .filter(|source_file| {
                !crate::code_style::is_test_source_path(crate::types::PathRef::from(
                    std::borrow::Borrow::<std::path::Path>::borrow(source_file.path()),
                ))
                .get()
            })
            .for_each(|source_file| {
                let visitor = crate::code_style::visit_syn_file(
                    crate::types::SynFileRef::from(source_file.ast().as_ref()),
                    super::source_analysis::DirectPathCallVisitor::new(
                        crate::types::DiagnosticMessages::default(),
                    ),
                );
                let has_direct_access = visitor.get_calls().iter().any(|call| {
                    call.starts_with(constants_str::STD_PATH_ENV_PATH)
                        || call.starts_with(constants_str::STD_PATH_FS_PATH)
                        || call.starts_with(constants_str::TOKIO_PATH_FS_PATH)
                });
                if !has_direct_access {
                    return;
                }
                let path = source_file.path().as_ref().to_string_lossy();
                constants_str::CODE_STYLE_DIRECT_FS_OWNER_SUFFIXES
                    .iter()
                    .filter(|suffix| {
                        path.ends_with(**suffix)
                            || crate::code_style::declared_child_matches(path.as_ref(), suffix)
                    })
                    .for_each(|suffix| {
                        let _inserted = matched.insert(*suffix);
                    });
            });
        let stale = constants_str::CODE_STYLE_DIRECT_FS_OWNER_SUFFIXES
            .iter()
            .filter(|suffix| !matched.contains(**suffix))
            .collect::<Vec<&&str>>();
        assert!(
            stale.is_empty(),
            "3c9e41b7 stale direct filesystem owners: {stale:?}"
        );
    });
    assert!(
        crate::code_style::is_direct_fs_owner_source_path(crate::types::PathRef::from(
            std::path::Path::new(constants_str::VALUE_FC45F701)
        ))
        .get(),
        "5b71e44a"
    );
    assert!(
        !crate::code_style::is_direct_fs_owner_source_path(crate::types::PathRef::from(
            std::path::Path::new(constants_str::VALUE_2179FBD7)
        ))
        .get(),
        "f1428b6c"
    );
}
#[test]
fn test_runtime_data_reads_are_bounded() {
    crate::code_style::assert_rs_ast_errors_empty_with_context(
        crate::types::StaticStr::from(constants_str::VALUE_37B593CE),
        crate::types::SourceTextRef::from(
            constants_str::RUNTIME_CODE_PERFORMS_AN_UNBOUNDED_FILE_OR_HTTP_RESPONSE_READ,
        ),
        |path, ast, errors| {
            let path_text = path.to_string_lossy();
            if crate::code_style::is_test_crate_source_path(crate::types::PathRef::from(path)).get()
                || constants_str::CODE_STYLE_BOUNDED_READ_OWNER_SUFFIXES
                    .iter()
                    .any(|suffix| path_text.ends_with(suffix))
            {
                return;
            }
            let visitor = crate::code_style::visit_syn_file(
                crate::types::SynFileRef::from(ast),
                super::source_analysis::UnboundedReadVisitor::new(
                    crate::types::DiagnosticMessages::default(),
                ),
            );
            errors.extend(
                visitor
                    .get_calls()
                    .clone()
                    .into_iter()
                    .map(|call| format!("{}: unbounded `{call}`", path.display())),
            );
        },
    );
}
#[test]
fn test_bounded_read_policy_rejects_sync_and_async_whole_file_reads() {
    let syntax: syn::File = syn::parse_quote! {
        pub(super) async fn reads() {
            let _first = std::fs::read(stringify!(first));
            let _second = std::fs::read_to_string(stringify!(second));
            let _third = tokio::fs::read(stringify!(third)).await;
            let _fourth = tokio::fs::read_to_string(stringify!(fourth)).await;
        }
    };
    let visitor = crate::code_style::visit_syn_file(
        crate::types::SynFileRef::from(&syntax),
        super::source_analysis::UnboundedReadVisitor::new(
            crate::types::DiagnosticMessages::default(),
        ),
    );
    assert_eq!(visitor.get_calls().len(), 4usize, "46638c47");
}
#[test]
fn test_raw_runtime_sql_identifier_inventory_matches_reviewed_baseline() {
    let mut observed = std::collections::BTreeMap::<String, usize>::new();
    crate::code_style::for_each_rs_file(|file| {
        let (path, content) = (file.path().as_ref(), file.content().as_ref());
        let path_text = path.to_string_lossy();
        if crate::code_style::is_test_crate_source_path(crate::types::PathRef::from(path)).get()
            || path_text.ends_with(constants_str::CODE_STYLE_WORKSPACE_SCAFFOLD_FS_OWNER_SUFFIX)
            || path_text.ends_with(constants_str::PG_CRUD_PG_CRUD_COMMON_SRC_SQL_IDENTIFIER_RS)
            || crate::code_style::is_str_constants_source_path(crate::types::PathRef::from(path))
                .get()
        {
            return;
        }
        let count = [
            constants_str::FROM,
            constants_str::INTO,
            constants_str::UPDATE,
        ]
        .into_iter()
        .map(|pattern| content.matches(pattern).count())
        .sum::<usize>();
        if count != constants_usize::ZERO {
            let workspace_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                .parent()
                .expect(constants_str::DIAGNOSTIC_19512C63);
            let relative = path
                .strip_prefix(workspace_root)
                .unwrap_or(path)
                .to_string_lossy()
                .to_string();
            let _previous = observed.insert(relative, count);
        }
    });
    let expected = std::collections::BTreeMap::new();
    assert_eq!(observed, expected, "raw SQL identifier baseline changed");
}
#[test]
fn test_production_pg_error_classification_is_centralized() {
    super::test_code_style_snapshot::with_codebase_snapshot(|snapshot| {
        let violations = snapshot
            .rs_files()
            .iter()
            .filter(|source_file| {
                let path = source_file.path().as_ref().to_string_lossy();
                !crate::code_style::is_test_crate_source_path(crate::types::PathRef::from(
                    source_file.path().as_ref(),
                ))
                .get()
                    && !path.ends_with(constants_str::PG_CRUD_COMMON_SRC_PG_ERROR_RS)
                    && !crate::code_style::is_str_constants_source_path(
                        crate::types::PathRef::from(source_file.path().as_ref()),
                    )
                    .get()
                    && (source_file
                        .content()
                        .as_ref()
                        .contains(constants_str::IS_UNIQUE_VIOLATION_CALL)
                        || source_file
                            .content()
                            .as_ref()
                            .contains(constants_str::PG_SQLSTATE_PREFIX))
            })
            .map(|source_file| {
                format!(
                    "{} classifies PostgreSQL errors outside pg_crud_common",
                    source_file.path().as_ref().display()
                )
            })
            .collect::<Vec<String>>();
        assert!(violations.is_empty(), "{violations:#?}");
    });
}
#[test]
fn test_direct_process_command_creation_stays_in_shared_tooling() {
    crate::code_style::assert_rs_ast_errors_empty_with_context(
        crate::types::StaticStr::from(constants_str::F170AA14),
        crate::types::SourceTextRef::from(constants_str::DIRECT_COMMAND_PATH_NEW_USAGE_EXISTS_OUTSIDE_MACRO_HELPERS_PATH_TOOL_COMMAND),
        |path, ast, errors| {
            if path.ends_with(constants_str::MACRO_HELPERS_SRC_TOOL_COMMAND_RS) {
                return;
            }
            let visitor = crate::code_style::visit_syn_file(
                crate::types::SynFileRef::from(ast),
                super::source_analysis::DirectPathCallVisitor::new(crate::types::DiagnosticMessages::default()),
            );
            errors.extend(
                visitor
                    .get_calls().clone().into_iter()
                    .filter(|call| call == constants_str::STD_PATH_PROCESS_PATH_COMMAND_PATH_NEW)
                    .map(|call| format!("{}: direct `{call}`", path.display())),
            );
        },
    );
}
#[test]
fn test_abort_and_transmute_calls_match_reviewed_baseline() {
    let mut observed_abort_paths = Vec::new();
    let mut errors = Vec::new();
    crate::code_style::for_each_rs_file(|file| {
        let (path, ast) = (file.path().as_ref(), file.ast().as_ref());
        let visitor = crate::code_style::visit_syn_file(
            crate::types::SynFileRef::from(ast),
            super::source_analysis::DirectPathCallVisitor::new(
                crate::types::DiagnosticMessages::default(),
            ),
        );
        visitor.get_calls().clone().into_iter().for_each(|call| {
            if call == constants_str::STD_PATH_PROCESS_PATH_ABORT {
                observed_abort_paths.push(path.to_string_lossy().to_string());
            }
            if call.ends_with(constants_str::PATH_TRANSMUTE) {
                errors.push(format!("{}: forbidden `{call}`", path.display()));
            }
        });
    });
    observed_abort_paths.sort();
    if !observed_abort_paths.is_empty() {
        errors.push(format!(
            "process abort calls are forbidden; observed={observed_abort_paths:?}"
        ));
    }
    crate::code_style::assert_joined_errors_empty_with_context(
        crate::types::SourceTextListRef::from(errors.as_slice()),
        crate::types::StaticStr::from(constants_str::F87F82B6),
        crate::types::SourceTextRef::from(constants_str::ABORT_TRANSMUTE_POLICY_VIOLATIONS),
    );
}
#[test]
fn test_every_workspace_struct_and_enum_derives_optimal_memory_layout() {
    crate::code_style::assert_rs_ast_errors_empty_with_context(
        crate::types::StaticStr::from(constants_str::VALUE_71790FED),
        crate::types::SourceTextRef::from(constants_str::VALUE_6264CCC9),
        |path, ast, errors| {
            if path.ends_with(constants_str::VALUE_30B1AC8C) {
                return;
            }
            let visitor = crate::code_style::visit_syn_file(
                crate::types::SynFileRef::from(ast),
                super::source_analysis::OptimalMemoryLayoutVisitor::default(),
            );
            errors.extend(
                visitor
                    .get_errors()
                    .clone()
                    .into_iter()
                    .map(|error| format!("{}: {error}", path.display())),
            );
        },
    );
}
#[test]
fn test_optimal_memory_layout_derive_visitor_checks_structs_and_enums() {
    let ast =
        syn::parse_file(constants_str::VALUE_936BA38B).expect(constants_str::DIAGNOSTIC_34FB5A61);
    let visitor = crate::code_style::visit_syn_file(
        crate::types::SynFileRef::from(&ast),
        super::source_analysis::OptimalMemoryLayoutVisitor::default(),
    );
    assert_eq!(
        visitor.get_errors().as_slice(),
        [constants_str::VALUE_2B8772F7, constants_str::VALUE_717A1158,],
        "42dc6e3b"
    );
}
#[test]
fn test_unit_tests_use_deterministic_time_and_randomness_patterns() {
    crate::code_style::assert_rs_ast_errors_empty_with_context(
        crate::types::StaticStr::from(constants_str::VALUE_821D4A76),
        crate::types::SourceTextRef::from(constants_str::UNIT_TESTS_USE_NONDETERMINISTIC_TIME_SLEEP_OR_RANDOMNESS_WITHOUT_A_REVIEWED_OWNER),
        |path, ast, errors| {
            let scan_entire_file = crate::code_style::is_test_source_path(crate::types::PathRef::from(path))
                .get()
                && !path.ends_with(constants_str::VALUE_4A3D63F7)
                && !path
                    .components()
                    .any(|component| component.as_os_str() == constants_str::CODE_STYLE);
            let visitor = crate::code_style::visit_syn_file(
                crate::types::SynFileRef::from(ast),
                super::source_analysis::TestNondeterminismVisitor::new(crate::types::DiagnosticMessages::default(), crate::types::AnalyzerCount::from(usize::from(scan_entire_file))),
            );
            errors.extend(
                visitor
                    .get_calls()
                    .iter()
                    .map(|call| format!("{}: nondeterministic `{call}`", path.display())),
            );
        },
    );
}
#[test]
fn test_unit_test_nondeterminism_visitor_rejects_sync_async_time_and_randomness() {
    let ast =
        syn::parse_file(constants_str::VALUE_402DAFF0).expect(constants_str::DIAGNOSTIC_9354F086);
    let visitor = crate::code_style::visit_syn_file(
        crate::types::SynFileRef::from(&ast),
        super::source_analysis::TestNondeterminismVisitor::new(
            crate::types::DiagnosticMessages::default(),
            crate::types::AnalyzerCount::default(),
        ),
    );
    assert_eq!(
        visitor.get_calls().as_slice(),
        [
            constants_str::TOKIO_PATH_TIME_PATH_SLEEP,
            constants_str::UUID_PATH_UUID_PATH_NEW_V4,
            constants_str::STD_PATH_TIME_PATH_SYSTEMTIME_PATH_NOW,
            constants_str::STD_PATH_TIME_PATH_INSTANT_PATH_NOW,
            constants_str::RAND_PATH_RNG,
            constants_str::GETRANDOM_PATH_FILL,
            constants_str::RAND_PATH_RNGS_PATH_OS_RNG,
        ],
        "fa8d2bb1"
    );
    let integration_visitor = crate::code_style::visit_syn_file(
        crate::types::SynFileRef::from(&ast),
        super::source_analysis::TestNondeterminismVisitor::new(
            crate::types::DiagnosticMessages::default(),
            crate::types::AnalyzerCount::from(constants_usize::ONE),
        ),
    );
    assert_eq!(integration_visitor.get_calls().len(), 8usize, "78fde80e");
}
#[test]
fn test_generated_source_templates_do_not_embed_random_test_values() {
    crate::code_style::assert_rs_ast_errors_empty_with_context(
        crate::types::StaticStr::from(constants_str::VALUE_1491FF0E),
        crate::types::SourceTextRef::from(constants_str::VALUE_920FAF03),
        |path, ast, errors| {
            let visitor = crate::code_style::visit_syn_file(
                crate::types::SynFileRef::from(ast),
                super::source_analysis::GeneratedRandomnessVisitor::new(
                    crate::types::DiagnosticMessages::default(),
                ),
            );
            errors.extend(
                visitor
                    .get_calls()
                    .clone()
                    .into_iter()
                    .map(|call| format!("{}: generated `{call}`", path.display())),
            );
        },
    );
}
#[test]
fn test_generated_randomness_policy_inspects_quote_token_streams() {
    let source = [
        constants_str::VALUE_B04CA9E8,
        constants_str::VALUE_C7C4300B,
        constants_str::VALUE_2328A0D2,
        constants_str::VALUE_D10B36AA,
    ]
    .join(constants_str::NEWLINE);
    let ast = syn::parse_file(source.as_str()).expect(constants_str::DIAGNOSTIC_04E98F91);
    let visitor = crate::code_style::visit_syn_file(
        crate::types::SynFileRef::from(&ast),
        super::source_analysis::GeneratedRandomnessVisitor::new(
            crate::types::DiagnosticMessages::default(),
        ),
    );
    assert_eq!(visitor.get_calls().len(), 2usize);
}
#[test]
fn test_process_static_state_matches_reviewed_inventory() {
    #[derive(
        proc_macro_getters::Getters, proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    )]
    struct StaticStateException {
        identifier: &'static str,
        path_suffix: &'static str,
        reason: &'static str,
    }
    let exceptions = [
        StaticStateException {
            identifier: constants_str::VALUE_4E9D8B24,
            path_suffix: constants_str::VALUE_9DDB2371,
            reason: constants_str::VALUE_9B0E1F72,
        },
        StaticStateException {
            identifier: constants_str::VALUE_F783DB26,
            path_suffix: constants_str::VALUE_865824F9,
            reason: constants_str::VALUE_946801A9,
        },
        StaticStateException {
            identifier: constants_str::VALUE_F783DB26,
            path_suffix: constants_str::VALUE_F67EAA19,
            reason: constants_str::VALUE_946801A9,
        },
        StaticStateException {
            identifier: constants_str::VALUE_1CA9CD1C,
            path_suffix: constants_str::VALUE_959AEDDC,
            reason: constants_str::VALUE_C677F169,
        },
        StaticStateException {
            identifier: constants_str::VALUE_5A6DD0A3,
            path_suffix: constants_str::VALUE_959AEDDC,
            reason: constants_str::VALUE_CAD59B9B,
        },
        StaticStateException {
            identifier: constants_str::VALUE_00B29514,
            path_suffix: constants_str::VALUE_96554632,
            reason: constants_str::VALUE_FB4C1B30,
        },
        StaticStateException {
            identifier: constants_str::VALUE_3623F7E2,
            path_suffix: constants_str::VALUE_392D41BA,
            reason: constants_str::VALUE_B2FEB0FD,
        },
        StaticStateException {
            identifier: constants_str::DECLARED_CHILDREN,
            path_suffix: constants_str::VALUE_4A3D63F7,
            reason:
                constants_str::MODULE_DECLARATION_GRAPH_IS_CACHED_FOR_REPOSITORY_POLICY_MATCHING,
        },
    ];
    crate::code_style::assert_rs_ast_errors_empty_with_context(
        crate::types::StaticStr::from(constants_str::VALUE_118C4174),
        crate::types::SourceTextRef::from(constants_str::VALUE_9EC9C4B2),
        |path, ast, errors| {
            let visitor = crate::code_style::visit_syn_file(
                crate::types::SynFileRef::from(ast),
                super::source_analysis::StaticStateVisitor::new(
                    crate::types::SourceTextList::default(),
                ),
            );
            visitor
                .get_identifiers()
                .clone()
                .into_iter()
                .for_each(|identifier| {
                    let reviewed = exceptions.iter().any(|exception| {
                        (path.ends_with(exception.path_suffix)
                            || crate::code_style::declared_child_matches(
                                path.to_string_lossy().as_ref(),
                                exception.path_suffix,
                            ))
                            && exception.identifier == identifier
                            && !exception.reason.is_empty()
                    });
                    if !reviewed {
                        errors.push(format!(
                            "{}: unreviewed static `{identifier}`",
                            path.display()
                        ));
                    }
                });
        },
    );
}
#[test]
fn test_library_sources_do_not_use_print_macros() {
    crate::code_style::assert_rs_ast_errors_empty_with_context(
        crate::types::StaticStr::from(constants_str::VALUE_776EEBB3),
        crate::types::SourceTextRef::from(constants_str::VALUE_9908E138),
        |path, ast, errors| {
            let is_library_source = path
                .ancestors()
                .find(|ancestor| {
                    ancestor
                        .file_name()
                        .is_some_and(|name| name == constants_str::SRC_ALT)
                })
                .is_some_and(|source_directory| {
                    source_directory
                        .join(constants_str::VALUE_0544FC95)
                        .exists()
                });
            if !is_library_source {
                return;
            }
            let visitor = crate::code_style::visit_syn_file(
                crate::types::SynFileRef::from(ast),
                super::source_analysis::PrintMacroVisitor::new(
                    crate::types::DiagnosticMessages::default(),
                ),
            );
            visitor.get_calls().clone().into_iter().for_each(|call| {
                errors.push(format!("{}: library `{call}!`", path.display()));
            });
        },
    );
}
#[test]
fn test_production_code_does_not_use_line_print_macros() {
    crate::code_style::assert_rs_ast_errors_empty_with_context(
        crate::types::StaticStr::from(constants_str::VALUE_018B0C9F),
        crate::types::SourceTextRef::from(constants_str::VALUE_70D9A674),
        |path, ast, errors| {
            if crate::code_style::is_test_crate_source_path(crate::types::PathRef::from(path)).get()
            {
                return;
            }
            let visitor = crate::code_style::visit_syn_file(
                crate::types::SynFileRef::from(ast),
                super::source_analysis::ProductionLinePrintMacroVisitor::new(
                    crate::types::DiagnosticMessages::default(),
                ),
            );
            visitor.get_calls().clone().into_iter().for_each(|call| {
                errors.push(format!(
                    "{}: `{call}!`: {}",
                    path.display(),
                    constants_str::VALUE_70D9A674
                ));
            });
        },
    );
}
#[test]
fn test_module_and_function_names_use_single_underscores() {
    crate::code_style::assert_rs_ast_errors_empty_with_context(
        crate::types::StaticStr::from(constants_str::VALUE_AE652DDA),
        crate::types::SourceTextRef::from(constants_str::VALUE_63194000),
        |path, ast, errors| {
            if path
                .file_name()
                .and_then(std::ffi::OsStr::to_str)
                .is_some_and(|name| {
                    name.contains(constants_str::WORKSPACE_SCAFFOLD_DOUBLE_UNDERSCORE)
                })
            {
                errors.push(format!(
                    "{}: module filename contains a double underscore",
                    path.display()
                ));
            }
            let visitor = crate::code_style::visit_syn_file(
                crate::types::SynFileRef::from(ast),
                super::source_analysis::DoubleUnderscoreNamingVisitor::new(
                    crate::types::DiagnosticMessages::default(),
                ),
            );
            visitor
                .get_identifiers()
                .clone()
                .into_iter()
                .for_each(|identifier| {
                    errors.push(format!(
                        "{}: `{identifier}` contains a double underscore",
                        path.display()
                    ));
                });
        },
    );
}
#[test]
fn test_module_and_function_names_do_not_use_unclear_short_forms() {
    crate::code_style::assert_rs_ast_errors_empty_with_context(
        crate::types::StaticStr::from(constants_str::VALUE_AE652DDA),
        crate::types::SourceTextRef::from(constants_str::VALUE_63194000),
        |path, ast, errors| {
            if path
                .file_stem()
                .and_then(std::ffi::OsStr::to_str)
                .is_some_and(|name| {
                    name.split('_')
                        .any(|part| part == constants_str::WORKSPACE_SHORT_HELPER_TOKEN)
                })
            {
                errors.push(format!(
                    "{}: module filename abbreviates helper as hlp",
                    path.display()
                ));
            }
            let visitor = crate::code_style::visit_syn_file(
                crate::types::SynFileRef::from(ast),
                super::source_analysis::ShortFunctionNamingVisitor::new(
                    crate::types::DiagnosticMessages::default(),
                ),
            );
            visitor
                .get_identifiers()
                .clone()
                .into_iter()
                .for_each(|identifier| {
                    errors.push(format!(
                        "{}: `{identifier}` abbreviates make as mk",
                        path.display()
                    ));
                });
        },
    );
}
#[test]
fn test_struct_fields_do_not_use_opaque_short_names() {
    crate::code_style::assert_rs_ast_errors_empty_with_context(
        crate::types::StaticStr::from(constants_str::VALUE_AE652DDA),
        crate::types::SourceTextRef::from(constants_str::VALUE_63194000),
        |path, ast, errors| {
            let visitor = crate::code_style::visit_syn_file(
                crate::types::SynFileRef::from(ast),
                super::source_analysis::OpaqueShortFieldNamingVisitor::new(
                    crate::types::DiagnosticMessages::default(),
                ),
            );
            visitor
                .get_identifiers()
                .clone()
                .into_iter()
                .for_each(|identifier| {
                    errors.push(format!(
                        "{}: struct field `{identifier}` uses an opaque short name",
                        path.display()
                    ));
                });
        },
    );
}
#[test]
fn test_serde_renames_do_not_introduce_opaque_short_names() {
    crate::code_style::assert_rs_ast_errors_empty_with_context(
        crate::types::StaticStr::from(constants_str::VALUE_AE652DDA),
        crate::types::SourceTextRef::from(constants_str::VALUE_63194000),
        |path, ast, errors| {
            let visitor = crate::code_style::visit_syn_file(
                crate::types::SynFileRef::from(ast),
                super::source_analysis::OpaqueSerdeRenameVisitor::new(
                    crate::types::DiagnosticMessages::default(),
                ),
            );
            visitor
                .get_identifiers()
                .clone()
                .into_iter()
                .for_each(|identifier| {
                    errors.push(format!(
                        "{}: serde rename `{identifier}` uses an opaque short name",
                        path.display()
                    ));
                });
        },
    );
}
#[test]
fn test_production_line_print_macro_policy_allows_test_code_and_rejects_production_code() {
    let ast =
        syn::parse_file(constants_str::VALUE_606F2B07).expect(constants_str::DIAGNOSTIC_A508C55D);
    let visitor = crate::code_style::visit_syn_file(
        crate::types::SynFileRef::from(&ast),
        super::source_analysis::ProductionLinePrintMacroVisitor::new(
            crate::types::DiagnosticMessages::default(),
        ),
    );
    assert_eq!(
        visitor.get_calls().as_slice(),
        [
            constants_str::SHARED_VALUES_PRINTLN.to_owned(),
            constants_str::SHARED_VALUES_EPRINTLN.to_owned()
        ]
    );
    assert_eq!(
        constants_str::VALUE_70D9A674,
        std::str::from_utf8(b"instead of using println! and eprintln!, use tracing/telemetry")
            .expect(constants_str::VALUE_2DE961C6)
    );
}
#[test]
fn test_sensitive_text_wrappers_do_not_derive_unredacted_debug_or_display() {
    crate::code_style::assert_rs_ast_errors_empty_with_context(
        crate::types::StaticStr::from(constants_str::VALUE_2E395A49),
        crate::types::SourceTextRef::from(constants_str::VALUE_4C5A6F95),
        |path, ast, errors| {
            let visitor = crate::code_style::visit_syn_file(
                crate::types::SynFileRef::from(ast),
                super::source_analysis::SensitiveTextDebugDeriveVisitor::new(
                    crate::types::DiagnosticMessages::default(),
                ),
            );
            errors.extend(
                visitor
                    .get_errors()
                    .clone()
                    .into_iter()
                    .map(|error| format!("{}: {error}", path.display())),
            );
        },
    );
}
#[test]
fn test_sensitive_text_debug_policy_distinguishes_redacted_derives() {
    let ast =
        syn::parse_file(constants_str::VALUE_BC13B693).expect(constants_str::DIAGNOSTIC_3D72B9E0);
    let visitor = crate::code_style::visit_syn_file(
        crate::types::SynFileRef::from(&ast),
        super::source_analysis::SensitiveTextDebugDeriveVisitor::new(
            crate::types::DiagnosticMessages::default(),
        ),
    );
    assert_eq!(visitor.get_errors().len(), 4usize);
    assert!(
        visitor
            .get_errors()
            .iter()
            .any(|error| error.contains(constants_str::VALUE_607E1E95))
    );
    assert!(
        visitor
            .get_errors()
            .iter()
            .any(|error| error.contains(constants_str::VALUE_F9719A33))
    );
    assert!(
        visitor
            .get_errors()
            .iter()
            .any(|error| error.contains(constants_str::VALUE_3E487F75))
    );
}
#[test]
fn test_error_formatters_do_not_expose_sensitive_fields() {
    crate::code_style::assert_rs_ast_errors_empty_with_context(
        crate::types::StaticStr::from(constants_str::VALUE_10E432C3),
        crate::types::SourceTextRef::from(constants_str::VALUE_ED81BDD6),
        |path, ast, errors| {
            let visitor = crate::code_style::visit_syn_file(
                crate::types::SynFileRef::from(ast),
                super::source_analysis::SensitiveErrorFormatVisitor::new(
                    crate::types::DiagnosticMessages::default(),
                ),
            );
            errors.extend(
                visitor
                    .get_errors()
                    .clone()
                    .into_iter()
                    .map(|error| format!("{}: {error}", path.display())),
            );
        },
    );
}
#[test]
fn test_sensitive_error_format_policy_rejects_named_and_tuple_placeholders() {
    let ast =
        syn::parse_file(constants_str::VALUE_2CC8E3AF).expect(constants_str::DIAGNOSTIC_D8CC09CA);
    let visitor = crate::code_style::visit_syn_file(
        crate::types::SynFileRef::from(&ast),
        super::source_analysis::SensitiveErrorFormatVisitor::new(
            crate::types::DiagnosticMessages::default(),
        ),
    );
    assert_eq!(visitor.get_errors().len(), 2usize);
}
#[test]
fn test_no_todo_or_unimplemented_macro_in_source_code() {
    crate::code_style::assert_rs_ast_errors_empty_with_context(
        crate::types::StaticStr::from(constants_str::C4E9A2D7),
        crate::types::SourceTextRef::from(constants_str::TODO_UNIMPLEMENTED_FOUND),
        |path, ast, errors| {
            let visitor = crate::code_style::visit_syn_file(
                crate::types::SynFileRef::from(ast),
                super::source_analysis::TodoUnimplVisitor::new(
                    crate::types::AnalyzerCount::default(),
                    crate::types::AnalyzerCount::default(),
                ),
            );
            crate::code_style::push_repeated_file_error(
                crate::types::DiagnosticMessagesMutRef::from(&mut *errors),
                crate::types::PathRef::from(path),
                crate::types::SourceTextRef::from(constants_str::CONTAINS_TODO),
                *visitor.get_todo_found(),
            );
            crate::code_style::push_repeated_file_error(
                crate::types::DiagnosticMessagesMutRef::from(&mut *errors),
                crate::types::PathRef::from(path),
                crate::types::SourceTextRef::from(constants_str::CONTAINS_UNIMPLEMENTED),
                *visitor.get_unimplemented_found(),
            );
        },
    );
}
#[test]
fn test_source_lint_suppressions_have_explicit_reasons() {
    crate::code_style::assert_rs_ast_errors_empty_with_context(
        crate::types::StaticStr::from(constants_str::VALUE_7410D6B1),
        crate::types::SourceTextRef::from(constants_str::VALUE_2DAB1928),
        |path, ast, errors| {
            let source = std::fs::read_to_string(path).expect(constants_str::DIAGNOSTIC_8D3BCA08);
            let visitor = crate::code_style::visit_syn_file(
                crate::types::SynFileRef::from(ast),
                super::source_analysis::AllowReasonVisitor::new(
                    crate::types::DiagnosticMessages::default(),
                    crate::types::SourceTextList::from(
                        source.lines().map(str::to_owned).collect::<Vec<String>>(),
                    ),
                ),
            );
            errors.extend(
                visitor
                    .get_errors()
                    .clone()
                    .into_iter()
                    .map(|error| format!("{}: {error}", path.display())),
            );
        },
    );
}
#[test]
fn test_lint_suppression_reasons_are_specific() {
    super::test_code_style_snapshot::with_codebase_snapshot(|snapshot| {
        let violations = snapshot
            .project_source_files()
            .iter()
            .filter(|source_file| {
                source_file
                    .content()
                    .as_ref()
                    .contains(constants_str::GENERIC_LINT_SUPPRESSION_REASON)
            })
            .map(|source_file| source_file.path().as_ref().display().to_string())
            .collect::<Vec<String>>();
        assert!(
            violations.is_empty(),
            "{}: {violations:#?}",
            constants_str::GENERIC_LINT_SUPPRESSION_REASONS_ARE_FORBIDDEN
        );
    });
}
#[test]
fn test_source_lint_reason_policy_accepts_argument_and_comment_reasons() {
    let source = constants_str::VALUE_1D86D8F2;
    let ast = syn::parse_file(source).expect(constants_str::DIAGNOSTIC_EC218827);
    let visitor = crate::code_style::visit_syn_file(
        crate::types::SynFileRef::from(&ast),
        super::source_analysis::AllowReasonVisitor::new(
            crate::types::DiagnosticMessages::default(),
            crate::types::SourceTextList::from(
                source.lines().map(str::to_owned).collect::<Vec<String>>(),
            ),
        ),
    );
    assert_eq!(visitor.get_errors().len(), constants_usize::ONE);
}
#[test]
fn test_route_operation_error_policy_rejects_shared_types() {
    let ast = syn::parse_file(constants_str::CODE_STYLE_ROUTE_ENDPOINT_OPERATION_ERROR_FIXTURE)
        .expect(constants_str::DIAGNOSTIC_752FBB70);
    let visitor = crate::code_style::visit_syn_file(
        crate::types::SynFileRef::from(&ast),
        super::source_analysis::RouteOperationErrorVisitor::default(),
    );
    assert_eq!(visitor.get_errors().len(), 2usize);
}
#[test]
#[allow(
    clippy::needless_for_each,
    reason = "iterator form is required by the workspace no-for-loop policy"
)]
fn test_admin_route_errors_do_not_wrap_a_shared_operation_error() {
    super::test_code_style_snapshot::with_codebase_snapshot(|snapshot| {
        let auth = snapshot
            .rs_files()
            .iter()
            .find(|file| {
                file.path()
                    .as_ref()
                    .ends_with(constants_str::VALUE_0690A45F)
            })
            .expect(constants_str::DIAGNOSTIC_9585D60C)
            .content()
            .as_ref();
        let macros = snapshot
            .rs_files()
            .iter()
            .find(|file| {
                file.path()
                    .as_ref()
                    .ends_with(constants_str::VALUE_00ABFB22)
            })
            .expect(constants_str::DIAGNOSTIC_890B3180)
            .content()
            .as_ref();
        assert!(!auth.contains(constants_str::VALUE_68D941E2), "7c9f1bb0");
        assert!(auth.contains(constants_str::VALUE_C6F679F0), "166dc25a");
        assert!(macros.contains(constants_str::VALUE_DE36A108), "259e7ebd");
        [
            constants_str::VALUE_D7A45F10,
            constants_str::VALUE_1467E095,
            constants_str::VALUE_C2E67087,
            constants_str::VALUE_71657339,
            constants_str::VALUE_DB71AF6A,
            constants_str::VALUE_B9AFDC8D,
            constants_str::VALUE_00E5A912,
            constants_str::VALUE_EAB76571,
            constants_str::VALUE_91F980AF,
            constants_str::VALUE_0D833D68,
            constants_str::VALUE_682B824C,
            constants_str::VALUE_075F10C0,
        ]
        .iter()
        .for_each(|variant| {
            assert!(
                macros.contains(variant),
                "927e5901: admin route error macro is missing `{variant}`"
            );
        });
    });
}
#[test]
fn test_source_does_not_retain_commented_debug_statements() {
    crate::code_style::assert_rs_ast_errors_empty_with_context(
        crate::types::StaticStr::from(constants_str::VALUE_16B3BD74),
        crate::types::SourceTextRef::from(constants_str::VALUE_353E0299),
        |path, _, errors| {
            let source = std::fs::read_to_string(path).expect(constants_str::DIAGNOSTIC_2B06297B);
            errors.extend(
                crate::code_style::commented_debug_statements(crate::types::SourceTextRef::from(
                    source.as_str(),
                ))
                .into_iter()
                .map(|error| format!("{}: {error}", path.display())),
            );
        },
    );
}
#[test]
fn test_commented_debug_statement_policy_rejects_debug_macros_only() {
    let source = [
        concat!("/", "/", " println!(\"debug\");"),
        concat!("/", "/", " dbg!(value);"),
        concat!("/", "/", " explanation of println usage"),
        constants_str::VALUE_F7D8E121,
    ]
    .join(constants_str::NEWLINE);
    let violations = crate::code_style::commented_debug_statements(
        crate::types::SourceTextRef::from(source.as_str()),
    );
    assert_eq!(violations.len(), 2usize);
}
#[test]
fn test_project_text_files_have_stable_line_endings_and_no_trailing_whitespace() {
    let repository_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect(constants_str::DIAGNOSTIC_FD1294E4);
    let mut command = macro_helpers::tool_command::ToolCommand::new(
        macro_helpers::tool_program_ref::ToolProgramRef::from(constants_str::GIT_PROGRAM),
    );
    let _command = command
        .current_dir(macro_helpers::macro_path_ref::MacroPathRef::from(
            repository_root,
        ))
        .args(macro_helpers::tool_args_ref::ToolArgsRef::from(
            constants_str::GIT_LS_FILES_ARGS.as_slice(),
        ));
    let output = command.output().expect(constants_str::DIAGNOSTIC_E6C52471);
    assert!(output.status.success(), "9041df16");
    let tracked_paths =
        std::str::from_utf8(&output.stdout).expect(constants_str::DIAGNOSTIC_B9976FE8);
    let mut violations = Vec::<String>::new();
    tracked_paths
        .split_terminator('\0')
        .for_each(|relative_path| {
            let path = repository_root.join(relative_path);
            let bytes = match std::fs::read(path.as_path()) {
                Ok(bytes) => bytes,
                Err(error) if error.kind() == std::io::ErrorKind::NotFound => return,
                Err(error) => std::panic::panic_any(constants_str::PANIC_D808E460.replacen(
                    constants_str::PANIC_PLACEHOLDER_81240055,
                    error.to_string().as_str(),
                    1usize,
                )),
            };
            let Ok(source) = std::str::from_utf8(bytes.as_slice()) else {
                return;
            };
            violations.extend(
                crate::code_style::text_content_hygiene_errors(crate::types::SourceTextRef::from(
                    source,
                ))
                .into_iter()
                .map(|error| format!("{}: {error}", path.display())),
            );
        });
    assert!(violations.is_empty(), "8c22bed1 {violations:#?}");
}
#[test]
fn test_text_content_hygiene_policy_rejects_all_line_ending_violations() {
    let mut source = String::from(constants_str::FIRST_ALT);
    source.push(' ');
    source.push('\r');
    source.push('\n');
    source.push_str(constants_str::VALUE_3547CB11);
    let violations = crate::code_style::text_content_hygiene_errors(
        crate::types::SourceTextRef::from(source.as_str()),
    );
    assert_eq!(violations.len(), 3usize);
}
#[test]
fn test_no_macro_rules_in_source_code() {
    let mut errors = Vec::new();
    crate::code_style::for_each_rs_file(|file| {
        errors.extend(crate::code_style::macro_rules_errors(
            crate::types::PathRef::from(file.path().as_ref()),
            crate::types::SourceTextRef::from(file.content().as_ref()),
        ));
    });
    crate::code_style::assert_joined_errors_empty_with_context(
        crate::types::SourceTextListRef::from(errors.as_slice()),
        crate::types::StaticStr::from(constants_str::B6E2A9F4),
        crate::types::SourceTextRef::from(
            constants_str::MACRO_RULES_FOUND_USE_WORKSPACE_PROC_MACRO_CRATES_INSTEAD,
        ),
    );
}
#[test]
fn test_macro_rules_policy_recommends_a_proc_macro_crate() {
    let source = format!(
        "{}! generated {{ () => {{}}; }}",
        constants_str::MACRO_RULES
    );
    let errors = crate::code_style::macro_rules_errors(
        crate::types::PathRef::from(std::path::Path::new(constants_str::TESTS_SRC_LIB_RS)),
        crate::types::SourceTextRef::from(source.as_str()),
    );
    assert_eq!(errors.len(), constants_usize::ONE);
    assert!(
        errors.first().is_some_and(|error| error.contains(
            constants_str::MACRO_RULES_FOUND_USE_WORKSPACE_PROC_MACRO_CRATES_INSTEAD
                .trim_end_matches(':')
        )),
        "1f2d4c8a {errors:#?}"
    );
}
#[test]
fn test_no_include_asset_macros_outside_allowlist() {
    crate::code_style::assert_rs_ast_errors_empty_with_context(
        crate::types::StaticStr::from(constants_str::A6D4F2C9),
        crate::types::SourceTextRef::from(constants_str::INCLUDE_STR_OR_INCLUDE_BYTES_FOUND_OUTSIDE_EXPLICIT_GENERATED_TEST_FIXTURE_ALLOWLIST),
        |path, ast, errors| {
            let visitor = crate::code_style::visit_syn_file(
                crate::types::SynFileRef::from(ast),
                super::source_analysis::IncludeAssetMacroVisitor::new(crate::types::DiagnosticMessages::default()),
            );
            errors.extend(
                visitor
                    .get_errors().clone().into_iter()
                    .map(|error| format!("{}: {error}", path.display())),
            );
        },
    );
}
fn append_non_public_use_import_er(
    path: &std::path::Path,
    analyzer_bool: crate::types::AnalyzerBool,
    vec: &mut Vec<String>,
) {
    if analyzer_bool.get() {
        vec.push(format!(
            "{}: found non-public use import; use the explicit path at the usage site",
            path.display()
        ));
    }
}

fn append_public_use_import_errors(
    path: &std::path::Path,
    source_text_list: &crate::types::SourceTextList,
    vec: &mut Vec<String>,
) {
    vec.extend(source_text_list.iter().map(|public_use_root| {
        format!(
            "{}: found public use import rooted at `{public_use_root}`; use the explicit path at the usage site",
            path.display()
        )
    }));
}
#[test]
#[allow(
    clippy::wildcard_enum_match_arm,
    reason = "test code style source policy requires this localized allowance for generated or framework-constrained code verified by focused tests"
)]
fn test_public_reexports_are_forbidden_and_private_imports_are_restricted() {
    crate::code_style::assert_rs_ast_errors_empty_with_context(
        crate::types::StaticStr::from(constants_str::B4E7C2A9),
        crate::types::SourceTextRef::from(
            constants_str::FORBIDDEN_PUBLIC_REEXPORTS_OR_PRIVATE_IMPORTS_FOUND_PREFER_EXPLICIT_PATHS,
        ),
        |path, ast, errors| {
            let visitor = crate::code_style::visit_syn_file(
                crate::types::SynFileRef::from(ast),
                super::source_analysis::UseImportVisitor::new(crate::types::SourceTextList::default(), crate::types::AnalyzerBool::default(), crate::types::AnalyzerBool::default()),
            );
            append_non_public_use_import_er(
                path,
                *visitor.get_found_non_public_use_import(),
                errors,
            );
            append_public_use_import_errors(path, visitor.get_public_use_roots(), errors);
            let generated_visitor = crate::code_style::visit_syn_file(
                crate::types::SynFileRef::from(ast),
                GeneratedPublicUseVisitor::default(),
            );
            errors.extend(
                generated_visitor
                    .get_violations()
                    .clone()
                    .into_iter()
                    .map(|violation| format!("{}: {violation}", path.display())),
            );
            if visitor.get_found_use_rename().get() {
                errors.push(format!(
                        "{}: found use rename with `as`; use the original item name or rename the item at its definition",
                        path.display()
                    ));
            }
        },
    );
}
#[test]
fn test_declared_child_does_not_bypass_non_public_use_import_policy() {
    let ast = syn::parse_file(constants_str::CODE_STYLE_DECLARED_CHILD_USE_FIXTURE)
        .expect(constants_str::DIAGNOSTIC_B67D5CF1);
    let visitor = crate::code_style::visit_syn_file(
        crate::types::SynFileRef::from(&ast),
        super::source_analysis::UseImportVisitor::new(
            crate::types::SourceTextList::default(),
            crate::types::AnalyzerBool::default(),
            crate::types::AnalyzerBool::default(),
        ),
    );
    let mut errors = Vec::<String>::new();
    append_non_public_use_import_er(
        std::path::Path::new(constants_str::CODE_STYLE_DECLARED_CHILD_FIXTURE_PATH),
        *visitor.get_found_non_public_use_import(),
        &mut errors,
    );
    append_non_public_use_import_er(
        std::path::Path::new(constants_str::CODE_STYLE_NESTED_OWNER_USE_FIXTURE_PATH),
        *visitor.get_found_non_public_use_import(),
        &mut errors,
    );
    assert_eq!(errors.len(), constants_usize::TWO, "e23d18a4");
}
#[test]
fn test_use_import_policy_detects_private_imports_and_public_reexports() {
    let ast =
        syn::parse_file(constants_str::VALUE_B2B1AD10).expect(constants_str::DIAGNOSTIC_7B9E6F31);
    let visitor = crate::code_style::visit_syn_file(
        crate::types::SynFileRef::from(&ast),
        super::source_analysis::UseImportVisitor::new(
            crate::types::SourceTextList::default(),
            crate::types::AnalyzerBool::default(),
            crate::types::AnalyzerBool::default(),
        ),
    );
    assert!(visitor.get_found_non_public_use_import().get(), "ac09626a");
    assert!(!visitor.get_found_use_rename().get(), "c2bff14e");
    assert_eq!(
        visitor.get_public_use_roots().len(),
        constants_usize::ONE,
        "3f4798c8"
    );

    let private_import_is_rejected = |item_use: syn::ItemUse| {
        let import_ast = syn::File {
            shebang: None,
            frontmatter: None,
            attrs: Vec::new(),
            items: vec![syn::Item::Use(item_use)],
        };
        crate::code_style::visit_syn_file(
            crate::types::SynFileRef::from(&import_ast),
            super::source_analysis::UseImportVisitor::new(
                crate::types::SourceTextList::default(),
                crate::types::AnalyzerBool::default(),
                crate::types::AnalyzerBool::default(),
            ),
        )
        .get_found_non_public_use_import()
        .get()
    };
    assert!(
        private_import_is_rejected(syn::parse_quote!(
            use bounded_types::bounded_vec::BoundedVec;
        )),
        "8ea3c40f"
    );
    assert!(
        private_import_is_rejected(syn::parse_quote!(
            use bounded_types::bounded_string::BoundedString;
        )),
        "18f67a2d"
    );

    let leptos_ast =
        syn::parse_file(constants_str::VALUE_444213A9).expect(constants_str::DIAGNOSTIC_56F86B52);
    let leptos_visitor = crate::code_style::visit_syn_file(
        crate::types::SynFileRef::from(&leptos_ast),
        super::source_analysis::UseImportVisitor::new(
            crate::types::SourceTextList::default(),
            crate::types::AnalyzerBool::default(),
            crate::types::AnalyzerBool::default(),
        ),
    );
    assert!(
        !leptos_visitor.get_found_non_public_use_import().get(),
        "5969a9a3"
    );
}
#[test]
fn test_cfg_test_modules_do_not_hide_forbidden_public_reexports() {
    let ast = syn::parse_file(constants_str::CODE_STYLE_REEXPORT_WITH_LOGIC_FIXTURE)
        .expect(constants_str::DIAGNOSTIC_12D3EA75);
    let visitor = crate::code_style::visit_syn_file(
        crate::types::SynFileRef::from(&ast),
        super::source_analysis::UseImportVisitor::new(
            crate::types::SourceTextList::default(),
            crate::types::AnalyzerBool::default(),
            crate::types::AnalyzerBool::default(),
        ),
    );
    let mut errors = Vec::<String>::new();
    append_public_use_import_errors(
        std::path::Path::new(constants_str::CODE_STYLE_DECLARED_CHILD_FIXTURE_PATH),
        visitor.get_public_use_roots(),
        &mut errors,
    );
    assert_eq!(errors.len(), 2usize, "654501aa");
}
#[test]
fn test_no_type_aliases_in_rust_sources() {
    crate::code_style::assert_rs_ast_errors_empty_with_context(
        crate::types::StaticStr::from(constants_str::C6E4F7A1),
        crate::types::SourceTextRef::from(
            constants_str::TYPE_ALIASES_FOUND_USE_EXPLICIT_TYPES_AT_USAGE_SITES,
        ),
        |path, ast, errors| {
            let visitor = crate::code_style::visit_syn_file(
                crate::types::SynFileRef::from(ast),
                super::source_analysis::TypeAliasVisitor::new(
                    crate::types::DiagnosticMessages::default(),
                ),
            );
            errors.extend(
                visitor
                    .get_errors()
                    .clone()
                    .into_iter()
                    .map(|error| format!("{}: {error}", path.display())),
            );
        },
    );
}
#[test]
fn test_no_empty_enums_in_rust_sources() {
    crate::code_style::assert_rs_ast_errors_empty_with_context(
        crate::types::StaticStr::from(constants_str::VALUE_19A18AE4),
        crate::types::SourceTextRef::from(constants_str::VALUE_721EDC25),
        |path, ast, errors| {
            let visitor = crate::code_style::visit_syn_file(
                crate::types::SynFileRef::from(ast),
                super::source_analysis::EmptyEnumVisitor::new(
                    crate::types::DiagnosticMessages::default(),
                ),
            );
            errors.extend(
                visitor
                    .get_errors()
                    .clone()
                    .into_iter()
                    .map(|error| format!("{}: {error}", path.display())),
            );
        },
    );
}
#[test]
fn test_empty_enum_policy_checks_items_and_attribute_payloads() {
    let source = [
        constants_str::VALUE_BFF335C4,
        constants_str::VALUE_D10B36AA,
        constants_str::VALUE_68E5AB24,
        constants_str::VALUE_1A46177C,
        constants_str::VALUE_9DC6533C,
        constants_str::VALUE_1A9A6650,
    ]
    .join(constants_str::NEWLINE);
    let ast = syn::parse_file(&source).expect(constants_str::DIAGNOSTIC_E52F247C);
    let visitor = crate::code_style::visit_syn_file(
        crate::types::SynFileRef::from(&ast),
        super::source_analysis::EmptyEnumVisitor::new(crate::types::DiagnosticMessages::default()),
    );
    assert_eq!(visitor.get_errors().len(), 2usize);
    assert!(
        visitor
            .get_errors()
            .iter()
            .any(|error| error.contains(constants_str::VALUE_2F5B45FB))
    );
    assert!(
        visitor
            .get_errors()
            .iter()
            .any(|error| error.contains(constants_str::VALUE_3E422B73))
    );
}
#[test]
fn test_infallible_functions_return_concrete_types() {
    crate::code_style::assert_rs_ast_errors_empty_with_context(
        crate::types::StaticStr::from(constants_str::VALUE_6E60D726),
        crate::types::SourceTextRef::from(constants_str::VALUE_4BAB9A8D),
        |path, ast, errors| {
            let visitor = crate::code_style::visit_syn_file(
                crate::types::SynFileRef::from(ast),
                super::source_analysis::InfallibleResultVisitor::new(
                    crate::types::DiagnosticMessages::default(),
                ),
            );
            errors.extend(
                visitor
                    .get_errors()
                    .clone()
                    .into_iter()
                    .map(|error| format!("{}: {error}", path.display())),
            );
        },
    );
}
#[test]
fn test_infallible_result_policy_rejects_wrappers_and_free_function_results() {
    let source = [
        constants_str::VALUE_117099FD,
        constants_str::VALUE_35B13C3B,
        constants_str::VALUE_356A6CFB,
        constants_str::VALUE_73D6360C,
        constants_str::VALUE_41324880,
    ]
    .join(constants_str::PG_CRUD_EMPTY_SQL_SUFFIX);
    let ast = syn::parse_file(&source).expect(constants_str::DIAGNOSTIC_AA0BACF7);
    let visitor = crate::code_style::visit_syn_file(
        crate::types::SynFileRef::from(&ast),
        super::source_analysis::InfallibleResultVisitor::new(
            crate::types::DiagnosticMessages::default(),
        ),
    );
    assert_eq!(visitor.get_errors().len(), 2usize);
}
#[test]
fn test_no_simple_constant_aliases_in_rust_sources() {
    crate::code_style::assert_rs_ast_errors_empty_with_context(
        crate::types::StaticStr::from(constants_str::A51F0D3B),
        crate::types::SourceTextRef::from(
            constants_str::SIMPLE_CONSTANT_ALIASES_FOUND_USE_THE_SOURCE_CONSTANT_DIRECTLY,
        ),
        |path, ast, errors| {
            let visitor = crate::code_style::visit_syn_file(
                crate::types::SynFileRef::from(ast),
                super::source_analysis::ConstantAliasVisitor::new(
                    crate::types::DiagnosticMessages::default(),
                ),
            );
            errors.extend(
                visitor
                    .get_errors()
                    .clone()
                    .into_iter()
                    .map(|error| format!("{}: {error}", path.display())),
            );
        },
    );
}
#[test]
fn test_tuple_newtypes_derive_from_inner_instead_of_implementing_passthrough_from() {
    crate::code_style::assert_rs_ast_errors_empty_with_context(
        crate::types::StaticStr::from(constants_str::VALUE_CF47C890),
        crate::types::SourceTextRef::from(constants_str::VALUE_43AA05FB),
        |path, ast, errors| {
            let is_required_foundation_impl = [
                (
                    std::path::Path::new(constants_str::VALUE_E24F0FD4),
                    constants_str::VALUE_403B3BAE,
                ),
                (
                    std::path::Path::new(constants_str::VALUE_C809930D),
                    constants_str::VALUE_D0D0184F,
                ),
                (
                    std::path::Path::new(constants_str::VALUE_2900052A),
                    constants_str::VALUE_E5996CB1,
                ),
                (
                    std::path::Path::new(constants_str::VALUE_1354D9A9),
                    constants_str::VALUE_2A080280,
                ),
                (
                    std::path::Path::new(constants_str::CONSTANTS_STR_MACROS_SYN_IDENT_PATH),
                    constants_str::CONSTANTS_STR_MACROS_BOOTSTRAP_FROM_REASON,
                ),
                (
                    std::path::Path::new(constants_str::CONSTANTS_STR_MACROS_SYN_LIT_STR_PATH),
                    constants_str::CONSTANTS_STR_MACROS_BOOTSTRAP_FROM_REASON,
                ),
                (
                    std::path::Path::new(constants_str::CONSTANTS_STR_MACROS_SYN_VISIBILITY_PATH),
                    constants_str::CONSTANTS_STR_MACROS_BOOTSTRAP_FROM_REASON,
                ),
            ]
            .iter()
            .any(|(suffix, reason)| {
                !reason.is_empty()
                    && (path.ends_with(suffix)
                        || crate::code_style::declared_child_matches(
                            path.to_string_lossy().as_ref(),
                            suffix.to_string_lossy().as_ref(),
                        ))
            });
            if is_required_foundation_impl {
                return;
            }
            let visitor = crate::code_style::visit_syn_file(
                crate::types::SynFileRef::from(ast),
                super::source_analysis::PassthroughFromVisitor::new(
                    crate::types::DiagnosticMessages::default(),
                    std::collections::BTreeMap::new(),
                ),
            );
            errors.extend(
                visitor
                    .get_errors()
                    .clone()
                    .into_iter()
                    .map(|error| format!("{}: {error}", path.display())),
            );
        },
    );
}
#[test]
fn test_tuple_newtypes_derive_into_inner_from_instead_of_implementing_passthrough_from() {
    crate::code_style::assert_rs_ast_errors_empty_with_context(
        crate::types::StaticStr::from(constants_str::DIAGNOSTIC_3F8C1A72),
        crate::types::SourceTextRef::from(constants_str::MANUAL_INTO_MODEL_METHODS_FOUND),
        |path, ast, errors| {
            if crate::code_style::is_proc_macro_implementation_source_path(
                crate::types::PathRef::from(path),
            )
            .get()
            {
                return;
            }
            let is_required_foundation_impl = [
                (
                    std::path::Path::new(constants_str::VALUE_E24F0FD4),
                    constants_str::VALUE_403B3BAE,
                ),
                (
                    std::path::Path::new(constants_str::VALUE_C809930D),
                    constants_str::VALUE_D0D0184F,
                ),
                (
                    std::path::Path::new(constants_str::VALUE_2900052A),
                    constants_str::VALUE_E5996CB1,
                ),
                (
                    std::path::Path::new(constants_str::VALUE_1354D9A9),
                    constants_str::VALUE_2A080280,
                ),
            ]
            .iter()
            .any(|(suffix, reason)| {
                !reason.is_empty()
                    && (path.ends_with(suffix)
                        || crate::code_style::declared_child_matches(
                            path.to_string_lossy().as_ref(),
                            suffix.to_string_lossy().as_ref(),
                        ))
            });
            if is_required_foundation_impl {
                return;
            }
            let visitor = crate::code_style::visit_syn_file(
                crate::types::SynFileRef::from(ast),
                super::source_analysis::PassthroughIntoInnerFromVisitor::new(
                    crate::types::DiagnosticMessages::default(),
                    std::collections::BTreeMap::new(),
                ),
            );
            errors.extend(
                visitor
                    .get_errors()
                    .clone()
                    .into_iter()
                    .map(|error| format!("{}: {error}", path.display())),
            );
        },
    );
}
#[test]
fn test_tuple_newtypes_do_not_implement_manual_passthrough_into_methods() {
    crate::code_style::assert_rs_ast_errors_empty_with_context(
        crate::types::StaticStr::from(constants_str::VALUE_A1DD158B),
        crate::types::SourceTextRef::from(constants_str::VALUE_E8DA133A),
        |path, ast, errors| {
            if crate::code_style::is_proc_macro_implementation_source_path(
                crate::types::PathRef::from(path),
            )
            .get()
                || (path
                    .ends_with(constants_str::WORKSPACE_MACRO_HELPERS_TOP_LEVEL_COMMA_PART_PATH)
                    && !constants_str::WORKSPACE_MACRO_HELPERS_BOOTSTRAP_NEWTYPE_REASON.is_empty())
            {
                return;
            }
            let visitor = crate::code_style::visit_syn_file(
                crate::types::SynFileRef::from(ast),
                super::source_analysis::PassthroughIntoMethodVisitor::new(
                    crate::types::DiagnosticMessages::default(),
                    std::collections::BTreeSet::new(),
                ),
            );
            errors.extend(
                visitor
                    .get_errors()
                    .clone()
                    .into_iter()
                    .map(|error| format!("{}: {error}", path.display())),
            );
        },
    );
}
#[test]
fn test_tuple_newtype_manual_into_model_policy_rejects_passthrough_method() {
    let ast: syn::File = syn::parse_quote! {
        struct TestModelWrapper(u8);
        impl TestModelWrapper {
            fn into_model(self) -> u8 {
                self.0
            }
        }
        struct TestValueWrapper(u8);
        impl TestValueWrapper {
            fn into_value(self) -> u8 {
                self.0
            }
        }
        struct TestInnerWrapper(u8);
        impl TestInnerWrapper {
            fn into_inner(self) -> u8 {
                self.0
            }
        }
        struct TestConstInnerWrapper(u8);
        impl TestConstInnerWrapper {
            const fn into_inner(self) -> u8 {
                self.0
            }
        }
    };
    let visitor = crate::code_style::visit_syn_file(
        crate::types::SynFileRef::from(&ast),
        super::source_analysis::PassthroughIntoMethodVisitor::new(
            crate::types::DiagnosticMessages::default(),
            std::collections::BTreeSet::new(),
        ),
    );
    assert_eq!(visitor.get_errors().len(), constants_usize::TWO);
}
#[test]
fn test_tuple_newtypes_derive_into_iterator_instead_of_forwarding_into_iter() {
    crate::code_style::assert_rs_ast_errors_empty_with_context(
        crate::types::StaticStr::from(constants_str::VALUE_37F4CEF1),
        crate::types::SourceTextRef::from(constants_str::VALUE_7B891FFF),
        |path, ast, errors| {
            let required_foundation_impl = (
                std::path::Path::new(constants_str::VALUE_2900052A),
                constants_str::VALUE_E5996CB1,
            );
            if path.starts_with(constants_str::WORKSPACE_MACRO_HELPERS_SRC_PATH)
                || !required_foundation_impl.1.is_empty()
                    && (path.ends_with(required_foundation_impl.0)
                        || crate::code_style::declared_child_matches(
                            path.to_string_lossy().as_ref(),
                            required_foundation_impl.0.to_string_lossy().as_ref(),
                        ))
            {
                return;
            }
            let visitor = crate::code_style::visit_syn_file(
                crate::types::SynFileRef::from(ast),
                super::source_analysis::ForwardingIntoIteratorVisitor::new(
                    crate::types::DiagnosticMessages::default(),
                ),
            );
            errors.extend(
                visitor
                    .get_errors()
                    .clone()
                    .into_iter()
                    .map(|error| format!("{}: {error}", path.display())),
            );
        },
    );
}
#[test]
fn test_tuple_newtypes_derive_display_instead_of_implementing_forwarding_display() {
    crate::code_style::assert_rs_ast_errors_empty_with_context(
        crate::types::StaticStr::from(constants_str::VALUE_DBD6E9F5),
        crate::types::SourceTextRef::from(constants_str::VALUE_C333E174),
        |path, ast, errors| {
            let is_required_foundation_impl = [
                (
                    std::path::Path::new(constants_str::VALUE_E24F0FD4),
                    constants_str::VALUE_403B3BAE,
                ),
                (
                    std::path::Path::new(constants_str::VALUE_C809930D),
                    constants_str::VALUE_D0D0184F,
                ),
                (
                    std::path::Path::new(constants_str::VALUE_2900052A),
                    constants_str::VALUE_E5996CB1,
                ),
            ]
            .iter()
            .any(|(suffix, reason)| {
                !reason.is_empty()
                    && (path.ends_with(suffix)
                        || crate::code_style::declared_child_matches(
                            path.to_string_lossy().as_ref(),
                            suffix.to_string_lossy().as_ref(),
                        ))
            });
            if is_required_foundation_impl
                || path.starts_with(constants_str::WORKSPACE_MACRO_HELPERS_SRC_PATH)
            {
                return;
            }
            let visitor = crate::code_style::visit_syn_file(
                crate::types::SynFileRef::from(ast),
                super::source_analysis::ForwardingDisplayVisitor::new(
                    crate::types::DiagnosticMessages::default(),
                ),
            );
            errors.extend(
                visitor
                    .get_errors()
                    .clone()
                    .into_iter()
                    .map(|error| format!("{}: {error}", path.display())),
            );
        },
    );
}
#[test]
fn test_forwarding_display_visitor_detects_equivalent_forms() {
    let ast: syn::File = syn::parse_quote! {
        impl std::fmt::Display for MethodCallValue {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.0.fmt(f)
            }
        }
        impl std::fmt::Display for QualifiedCallValue {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                std::fmt::Display::fmt(&self.0, f)
            }
        }
        impl std::fmt::Display for WriteMacroValue {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }
    };
    let visitor = crate::code_style::visit_syn_file(
        crate::types::SynFileRef::from(&ast),
        super::source_analysis::ForwardingDisplayVisitor::new(
            crate::types::DiagnosticMessages::default(),
        ),
    );
    assert_eq!(
        visitor.get_errors().len(),
        constants_usize::THREE,
        "e2f1f362 forwarding display detection invariant must hold"
    );
}
#[test]
fn test_error_implementations_derive_thiserror_error() {
    crate::code_style::assert_rs_ast_errors_empty_with_context(
        crate::types::StaticStr::from(constants_str::VALUE_3FCD79E4),
        crate::types::SourceTextRef::from(constants_str::VALUE_18D7D5AB),
        |path, ast, errors| {
            let visitor = crate::code_style::visit_syn_file(
                crate::types::SynFileRef::from(ast),
                super::source_analysis::ManualErrorImplVisitor::new(
                    crate::types::DiagnosticMessages::default(),
                ),
            );
            errors.extend(
                visitor
                    .get_errors()
                    .clone()
                    .into_iter()
                    .map(|error| format!("{}: {error}", path.display())),
            );
        },
    );
}
#[test]
fn test_json_api_error_responses_originate_from_thiserror_enums() {
    crate::code_style::assert_rs_ast_errors_empty_with_context(
        crate::types::StaticStr::from(constants_str::VALUE_1800EA0D),
        crate::types::SourceTextRef::from(constants_str::VALUE_FBB3C40C),
        |path, ast, errors| {
            let thiserror_enums = crate::code_style::visit_syn_file(
                crate::types::SynFileRef::from(ast),
                super::source_analysis::ThiserrorEnumVisitor::default(),
            );
            let visitor = crate::code_style::visit_syn_file(
                crate::types::SynFileRef::from(ast),
                super::source_analysis::JsonIntoResponseErrorVisitor::new(
                    crate::types::DiagnosticMessages::default(),
                    thiserror_enums.get_names(),
                ),
            );
            errors.extend(
                visitor
                    .get_errors()
                    .clone()
                    .into_iter()
                    .map(|error| format!("{}: {error}", path.display())),
            );
        },
    );
}
#[test]
fn test_json_api_error_response_policy_rejects_structs_and_accepts_thiserror_enums() {
    let ast = syn::parse_file(constants_str::CODE_STYLE_JSON_API_ERROR_ENUM_FIXTURE)
        .expect(constants_str::DIAGNOSTIC_E45C8F09);
    let thiserror_enums = crate::code_style::visit_syn_file(
        crate::types::SynFileRef::from(&ast),
        super::source_analysis::ThiserrorEnumVisitor::default(),
    );
    let visitor = crate::code_style::visit_syn_file(
        crate::types::SynFileRef::from(&ast),
        super::source_analysis::JsonIntoResponseErrorVisitor::new(
            crate::types::DiagnosticMessages::default(),
            thiserror_enums.get_names(),
        ),
    );
    assert_eq!(visitor.get_errors().len(), 2usize);
}
#[test]
fn test_api_response_errors_keep_source_locations_out_of_public_error_enums() {
    crate::code_style::assert_rs_ast_errors_empty_with_context(
        crate::types::StaticStr::from(constants_str::VALUE_0B0251B3),
        crate::types::SourceTextRef::from(constants_str::VALUE_9DE68BBD),
        |path, ast, errors| {
            let thiserror_enums = crate::code_style::visit_syn_file(
                crate::types::SynFileRef::from(ast),
                super::source_analysis::ThiserrorEnumVisitor::default(),
            );
            let visitor = crate::code_style::visit_syn_file(
                crate::types::SynFileRef::from(ast),
                super::source_analysis::ApiErrorLocationVisitor::new(
                    crate::types::DiagnosticMessages::default(),
                    thiserror_enums.get_location_names(),
                ),
            );
            errors.extend(
                visitor
                    .get_errors()
                    .clone()
                    .into_iter()
                    .map(|error| format!("{}: {error}", path.display())),
            );
        },
    );
}
#[test]
fn test_api_response_location_policy_rejects_location_fields() {
    let ast = syn::parse_file(constants_str::CODE_STYLE_JSON_API_ERROR_ENUM_FIXTURE)
        .expect(constants_str::DIAGNOSTIC_6D8C50F1);
    let thiserror_enums = crate::code_style::visit_syn_file(
        crate::types::SynFileRef::from(&ast),
        super::source_analysis::ThiserrorEnumVisitor::default(),
    );
    let visitor = crate::code_style::visit_syn_file(
        crate::types::SynFileRef::from(&ast),
        super::source_analysis::ApiErrorLocationVisitor::new(
            crate::types::DiagnosticMessages::default(),
            thiserror_enums.get_location_names(),
        ),
    );
    assert_eq!(visitor.get_errors().len(), 2usize);
}
#[test]
fn test_api_response_error_sources_use_observed_error() {
    crate::code_style::assert_rs_ast_errors_empty_with_context(
        crate::types::StaticStr::from(constants_str::VALUE_44E97EA0),
        crate::types::SourceTextRef::from(constants_str::VALUE_7CB245E4),
        |path, ast, errors| {
            let response_types = crate::code_style::visit_syn_file(
                crate::types::SynFileRef::from(ast),
                super::source_analysis::IntoResponseTypeVisitor::default(),
            );
            let visitor = crate::code_style::visit_syn_file(
                crate::types::SynFileRef::from(ast),
                super::source_analysis::ApiErrorSourceVisitor::new(
                    response_types.get_names(),
                    crate::types::DiagnosticMessages::default(),
                ),
            );
            errors.extend(
                visitor
                    .get_errors()
                    .clone()
                    .into_iter()
                    .map(|error| format!("{}: {error}", path.display())),
            );
        },
    );
}
#[test]
fn test_api_response_error_source_policy_rejects_raw_sources() {
    let ast = syn::parse_file(constants_str::CODE_STYLE_JSON_API_ERROR_ENUM_FIXTURE)
        .expect(constants_str::DIAGNOSTIC_B26F4527);
    let response_types = crate::code_style::visit_syn_file(
        crate::types::SynFileRef::from(&ast),
        super::source_analysis::IntoResponseTypeVisitor::default(),
    );
    let visitor = crate::code_style::visit_syn_file(
        crate::types::SynFileRef::from(&ast),
        super::source_analysis::ApiErrorSourceVisitor::new(
            response_types.get_names(),
            crate::types::DiagnosticMessages::default(),
        ),
    );
    assert_eq!(visitor.get_errors().len(), constants_usize::ONE);
}
#[test]
#[allow(
    clippy::needless_for_each,
    reason = "test code style source policy uses iterator traversal to comply with the workspace no-for-loop policy"
)]
#[allow(
    clippy::option_if_let_else,
    reason = "test code style source policy uses iterator traversal to comply with the workspace no-for-loop policy"
)]
fn test_every_fallible_typed_route_operation_has_its_own_error_type() {
    super::test_code_style_snapshot::with_codebase_snapshot(|snapshot| {
        let mut groups = std::collections::BTreeMap::<
            String,
            super::source_analysis::RouteOperationErrorVisitor,
        >::new();
        snapshot.rs_files().iter().for_each(|source_file| {
            let path = source_file.path().as_ref();
            let visitor = crate::code_style::visit_syn_file(
                crate::types::SynFileRef::from(source_file.ast().as_ref()),
                super::source_analysis::RouteOperationErrorVisitor::default(),
            );
            let path_text = path.to_string_lossy();
            let normalized_path = path_text
                .trim_start_matches(constants_str::TEXT_ALT_9)
                .trim_start_matches('/');
            let declared_owner = crate::code_style::declared_children()
                .iter()
                .find_map(|(owner, child)| (child == normalized_path).then_some(owner.as_str()))
                .map(|mut owner| {
                    while let Some(parent) = crate::code_style::declared_children()
                        .iter()
                        .find_map(|(parent, child)| (child == owner).then_some(parent.as_str()))
                    {
                        owner = parent;
                    }
                    crate::types::SourceTextRef::from(owner)
                });
            let group = match declared_owner {
                Some(owner) => owner.get().to_owned(),
                None => path_text.into_owned(),
            };
            let aggregate = groups.entry(group).or_default();
            aggregate
                .get_errors_mut()
                .extend(visitor.get_errors().iter().cloned());
            aggregate
                .get_registered_mut()
                .extend(visitor.get_registered().iter().cloned());
            aggregate
                .get_operations_mut()
                .extend(visitor.get_operations().iter().cloned());
        });
        let mut errors = Vec::new();
        groups.into_iter().for_each(|(path, visitor)| {
            errors.extend(
                visitor
                    .get_errors()
                    .clone()
                    .into_iter()
                    .map(|error| format!("{path}: {error}")),
            );
            visitor
                .get_registered()
                .difference(visitor.get_operations())
                .for_each(|endpoint| {
                    errors.push(format!(
                        "{path}: registered endpoint `{endpoint}` must declare its route operation"
                    ));
                });
        });
        crate::code_style::assert_joined_errors_empty_with_context(
            crate::types::SourceTextListRef::from(errors.as_slice()),
            crate::types::StaticStr::from(constants_str::VALUE_D1557BA1),
            crate::types::SourceTextRef::from(constants_str::VALUE_50C1CC72),
        );
    });
}
#[test]
fn test_typed_route_operation_error_policy_rejects_shared_types() {
    let ast = syn::parse_file(constants_str::CODE_STYLE_ROUTE_OPERATION_ERROR_FIXTURE)
        .expect(constants_str::DIAGNOSTIC_60FF98C7);
    let visitor = crate::code_style::visit_syn_file(
        crate::types::SynFileRef::from(&ast),
        super::source_analysis::RouteOperationErrorVisitor::default(),
    );
    assert_eq!(visitor.get_errors().len(), constants_usize::ONE);
}
#[test]
fn test_error_implementation_source_uses_only_thiserror_derive() {
    let forbidden_newtype_derive = concat!("proc_macro_newtype::", "Error");
    let forbidden_manual_impl = concat!("impl std::error::", "Error for");
    let mut errors = Vec::new();
    crate::code_style::for_each_rs_file(|file| {
        let (path, source) = (file.path().as_ref(), file.content().as_ref());
        [forbidden_newtype_derive, forbidden_manual_impl]
            .into_iter()
            .filter(|forbidden| source.contains(forbidden))
            .for_each(|forbidden| {
                errors.push(format!(
                    "{}: contains forbidden `{forbidden}`",
                    path.display()
                ));
            });
    });
    crate::code_style::assert_joined_errors_empty_with_context(
        crate::types::SourceTextListRef::from(errors.as_slice()),
        crate::types::StaticStr::from(constants_str::VALUE_7729AA39),
        crate::types::SourceTextRef::from(constants_str::VALUE_2B539A50),
    );
}
#[test]
fn test_tuple_newtypes_derive_not_inner_instead_of_implementing_not() {
    crate::code_style::assert_rs_ast_errors_empty_with_context(
        crate::types::StaticStr::from(constants_str::VALUE_0E9309F2),
        crate::types::SourceTextRef::from(constants_str::VALUE_00F4142B),
        |path, ast, errors| {
            let foundation_owner = constants_str::VALUE_2900052A;
            if path.starts_with(constants_str::WORKSPACE_MACRO_HELPERS_SRC_PATH)
                || path.ends_with(std::path::Path::new(foundation_owner))
                || crate::code_style::declared_child_matches(
                    path.to_string_lossy().as_ref(),
                    foundation_owner,
                )
            {
                return;
            }
            let visitor = crate::code_style::visit_syn_file(
                crate::types::SynFileRef::from(ast),
                super::source_analysis::ManualNotImplVisitor::new(
                    crate::types::DiagnosticMessages::default(),
                ),
            );
            errors.extend(
                visitor
                    .get_errors()
                    .clone()
                    .into_iter()
                    .map(|error| format!("{}: {error}", path.display())),
            );
        },
    );
}
#[test]
fn test_constant_display_implementations_derive_display_const() {
    crate::code_style::assert_rs_ast_errors_empty_with_context(
        crate::types::StaticStr::from(constants_str::VALUE_2D6FAA55),
        crate::types::SourceTextRef::from(constants_str::VALUE_A788CCC5),
        |path, ast, errors| {
            let visitor = crate::code_style::visit_syn_file(
                crate::types::SynFileRef::from(ast),
                super::source_analysis::ConstDisplayImplVisitor::new(
                    crate::types::DiagnosticMessages::default(),
                ),
            );
            errors.extend(
                visitor
                    .get_errors()
                    .clone()
                    .into_iter()
                    .map(|error| format!("{}: {error}", path.display())),
            );
        },
    );
}
#[test]
fn test_tuple_newtypes_derive_deref_inner_instead_of_implementing_forwarding_deref() {
    crate::code_style::assert_rs_ast_errors_empty_with_context(
        crate::types::StaticStr::from(constants_str::VALUE_24B5ACA8),
        crate::types::SourceTextRef::from(constants_str::VALUE_801C5785),
        |path, ast, errors| {
            let required_foundation_impl = (
                std::path::Path::new(constants_str::VALUE_2900052A),
                constants_str::VALUE_E5996CB1,
            );
            if path.starts_with(constants_str::WORKSPACE_MACRO_HELPERS_SRC_PATH)
                || !required_foundation_impl.1.is_empty()
                    && (path.ends_with(required_foundation_impl.0)
                        || crate::code_style::declared_child_matches(
                            path.to_string_lossy().as_ref(),
                            required_foundation_impl.0.to_string_lossy().as_ref(),
                        ))
            {
                return;
            }
            let visitor = crate::code_style::visit_syn_file(
                crate::types::SynFileRef::from(ast),
                super::source_analysis::ForwardingDerefVisitor::new(
                    crate::types::DiagnosticMessages::default(),
                    std::collections::BTreeMap::new(),
                ),
            );
            errors.extend(
                visitor
                    .get_errors()
                    .clone()
                    .into_iter()
                    .map(|error| format!("{}: {error}", path.display())),
            );
        },
    );
}
#[test]
fn test_tuple_newtypes_derive_borrow_instead_of_implementing_forwarding_borrow() {
    crate::code_style::assert_rs_ast_errors_empty_with_context(
        crate::types::StaticStr::from(constants_str::VALUE_1259718C),
        crate::types::SourceTextRef::from(constants_str::VALUE_38822A0E),
        |path, ast, errors| {
            let required_foundation_impl = (
                std::path::Path::new(constants_str::VALUE_E24F0FD4),
                constants_str::VALUE_403B3BAE,
            );
            if !required_foundation_impl.1.is_empty() && path.ends_with(required_foundation_impl.0)
            {
                return;
            }
            let visitor = crate::code_style::visit_syn_file(
                crate::types::SynFileRef::from(ast),
                super::source_analysis::ForwardingBorrowVisitor::new(
                    crate::types::DiagnosticMessages::default(),
                ),
            );
            errors.extend(
                visitor
                    .get_errors()
                    .clone()
                    .into_iter()
                    .map(|error| format!("{}: {error}", path.display())),
            );
        },
    );
}
#[test]
fn test_no_duplicated_string_literals_in_non_policy_test_code() {
    let mut literal_locations_by_value = std::collections::BTreeMap::<String, Vec<String>>::new();
    crate::code_style::for_each_rs_file(|file| {
        let (path, ast) = (file.path().as_ref(), file.ast().as_ref());
        let path_text = path.display().to_string();
        if !crate::code_style::is_non_policy_test_source_path(crate::types::PathRef::from(path))
            .get()
        {
            return;
        }
        let visitor = crate::code_style::visit_syn_file(
            crate::types::SynFileRef::from(ast),
            super::source_analysis::TestStringLiteralVisitor::new(
                crate::types::SourceTextList::default(),
            ),
        );
        visitor
            .get_values()
            .clone()
            .into_iter()
            .filter(|literal_value| !literal_value.is_empty())
            .for_each(|literal_value| {
                literal_locations_by_value
                    .entry(literal_value)
                    .or_default()
                    .push(path_text.clone());
            });
    });
    let errors = literal_locations_by_value
        .into_iter()
        .filter(|(_, locations)| locations.len() > 1)
        .map(|(literal_value, locations)| {
            format!("duplicated string literal {literal_value:?} in {locations:?}")
        })
        .collect::<Vec<String>>();
    crate::code_style::assert_joined_errors_empty_with_context(
        crate::types::SourceTextListRef::from(errors.as_slice()),
        crate::types::StaticStr::from(constants_str::DE729A31),
        crate::types::SourceTextRef::from(
            constants_str::DUPLICATED_STRING_LITERALS_FOUND_IN_NON_POLICY_TEST_CODE,
        ),
    );
}
#[test]
fn test_ordinary_test_fixture_is_in_duplicate_string_policy_scope() {
    assert!(
        crate::code_style::is_non_policy_test_source_path(crate::types::PathRef::from(
            std::path::Path::new(constants_str::CODE_STYLE_DOMAIN_FIXTURE_PATH)
        ))
        .get(),
        "f2ec448d"
    );
    assert!(
        !crate::code_style::is_non_policy_test_source_path(crate::types::PathRef::from(
            std::path::Path::new(constants_str::TESTS_SRC_CODE_STYLE)
        ))
        .get(),
        "8df61a91"
    );
}
#[test]
fn test_production_string_literals_are_reused() {
    let mut literal_locations_by_value = std::collections::BTreeMap::<String, Vec<String>>::new();
    crate::code_style::for_each_rs_file(|file| {
        let (path, ast) = (file.path().as_ref(), file.ast().as_ref());
        let path_text = path.display().to_string();
        if crate::code_style::is_test_crate_source_path(crate::types::PathRef::from(path)).get()
            || crate::code_style::is_code_style_meta_harness_source_path(
                crate::types::PathRef::from(path),
            )
            .get()
            || crate::code_style::is_str_constants_source_path(crate::types::PathRef::from(path))
                .get()
        {
            return;
        }
        let visitor = crate::code_style::visit_syn_file(
            crate::types::SynFileRef::from(ast),
            super::source_analysis::ProductionStringLiteralVisitor::new(
                crate::types::SourceTextList::default(),
            ),
        );
        visitor
            .get_values()
            .clone()
            .into_iter()
            .filter(|literal_value| !literal_value.is_empty())
            .for_each(|literal_value| {
                literal_locations_by_value
                    .entry(literal_value)
                    .or_default()
                    .push(path_text.clone());
            });
    });
    let errors = literal_locations_by_value
        .into_iter()
        .filter(|(_, locations)| locations.len() > constants_usize::ONE)
        .map(|(literal_value, locations)| {
            format!("duplicated production string literal {literal_value:?} in {locations:?}")
        })
        .collect::<Vec<String>>();
    crate::code_style::assert_joined_errors_empty_with_context(
        crate::types::SourceTextListRef::from(errors.as_slice()),
        crate::types::StaticStr::from(constants_str::VALUE_9D1C7E4A),
        crate::types::SourceTextRef::from(
            constants_str::PRODUCTION_STRING_LITERALS_MUST_BE_DEFINED_ONCE_AND_REUSED,
        ),
    );
}
#[test]
#[allow(
    clippy::needless_for_each,
    clippy::useless_concat,
    reason = "the negative ownership policy must construct forbidden identifiers without declaring them in constants_str"
)]
fn test_domain_owned_string_catalogs_do_not_return_to_str_constants() {
    let source = std::fs::read_to_string(constants_str::STR_CONSTANTS_SRC_LIB_RS)
        .expect(constants_str::DIAGNOSTIC_84C15A0E);
    [
        concat!("ADMIN_SETTING_DEFAULT_ROUTE_LABEL"),
        concat!("ADMIN_DATABASE_ERROR_CODE"),
        concat!("NOTIFICATION_PERSISTENCE_ERROR_CODE"),
        concat!("SERVER_ADMIN_DB_SCHEMA_VALUE_"),
    ]
    .into_iter()
    .for_each(|identifier| assert!(!source.contains(identifier), "{identifier}"));
}

#[test]
#[allow(
    clippy::chunks_exact_to_as_chunks,
    clippy::indexing_slicing,
    clippy::missing_asserts_for_indexing,
    clippy::needless_collect,
    clippy::needless_for_each,
    clippy::shadow_reuse,
    clippy::wildcard_enum_match_arm,
    reason = "the policy test mirrors the small fixed token grammar of define_str_constants and mutates two independent inventories during one traversal"
)]
fn test_string_constants_reuse_every_repeated_word() {
    let source = std::fs::read_to_string(constants_str::STR_CONSTANTS_SRC_LIB_RS)
        .expect(constants_str::DIAGNOSTIC_4629EDBB);
    let ast = syn::parse_file(&source).expect(constants_str::DIAGNOSTIC_8B13948D);
    let macro_tokens = ast
        .items
        .iter()
        .find_map(|item| match item {
            syn::Item::Macro(item_macro)
                if item_macro.mac.path.segments.last().is_some_and(|segment| {
                    segment.ident == constants_str::SHARED_VALUES_DEFINE_STR_CONSTANTS
                }) =>
            {
                Some(item_macro.mac.tokens.clone())
            }
            _ => None,
        })
        .expect(constants_str::DIAGNOSTIC_9350BA36);
    let top_tokens = macro_tokens.into_iter().collect::<Vec<_>>();
    let fragment_group = top_tokens
        .get(constants_usize::ONE)
        .and_then(|token| match token {
            proc_macro2::TokenTree::Group(group) => Some(group),
            _ => None,
        })
        .expect(constants_str::DIAGNOSTIC_65052205);
    let fragments = fragment_group
        .stream()
        .into_iter()
        .collect::<Vec<_>>()
        .chunks_exact(4usize)
        .filter_map(|tokens| match (&tokens[0], &tokens[2]) {
            (proc_macro2::TokenTree::Ident(name), proc_macro2::TokenTree::Literal(value)) => {
                syn::parse_str::<syn::LitStr>(&value.to_string())
                    .ok()
                    .map(|value| (name.to_string(), value.value()))
            }
            _ => None,
        })
        .collect::<std::collections::BTreeMap<_, _>>();
    let rust_fragment_group = top_tokens
        .get(3usize)
        .and_then(|token| match token {
            proc_macro2::TokenTree::Group(group) => Some(group),
            _ => None,
        })
        .expect(constants_str::DIAGNOSTIC_E402ACE1);
    let rust_fragments = rust_fragment_group
        .stream()
        .into_iter()
        .collect::<Vec<_>>()
        .chunks_exact(4usize)
        .filter_map(|tokens| match (&tokens[0], &tokens[2]) {
            (proc_macro2::TokenTree::Ident(name), proc_macro2::TokenTree::Group(parts)) => {
                Some((name.to_string(), parts.stream()))
            }
            _ => None,
        })
        .collect::<std::collections::BTreeMap<_, _>>();
    let rust_constant_part_tokens = top_tokens
        .get(5usize)
        .and_then(|token| match token {
            proc_macro2::TokenTree::Group(group) => Some(group.stream()),
            _ => None,
        })
        .into_iter()
        .flatten()
        .filter_map(|token| match token {
            proc_macro2::TokenTree::Group(group) => Some(group.stream()),
            _ => None,
        })
        .flatten()
        .collect::<Vec<_>>();
    let constant_part_tokens = [3usize, 5usize, 7usize]
        .into_iter()
        .filter_map(|index| top_tokens.get(index))
        .filter_map(|token| match token {
            proc_macro2::TokenTree::Group(group) => Some(group.stream()),
            _ => None,
        })
        .flatten()
        .filter_map(|token| match token {
            proc_macro2::TokenTree::Group(group) => Some(group.stream()),
            _ => None,
        })
        .flatten()
        .collect::<Vec<_>>();
    let declared_constant_part_tokens = [5usize, 7usize]
        .into_iter()
        .filter_map(|index| top_tokens.get(index))
        .filter_map(|token| match token {
            proc_macro2::TokenTree::Group(group) => Some(group.stream()),
            _ => None,
        })
        .flatten()
        .filter_map(|token| match token {
            proc_macro2::TokenTree::Group(group) => Some(group.stream()),
            _ => None,
        })
        .flatten()
        .collect::<Vec<_>>();
    let mut fragment_use_counts = fragments
        .keys()
        .map(|name| (name.clone(), 0usize))
        .collect::<std::collections::BTreeMap<_, _>>();
    let mut literal_word_counts = std::collections::BTreeMap::<String, usize>::new();
    let mut rust_fragment_use_counts = rust_fragments
        .keys()
        .map(|name| (name.clone(), 0usize))
        .collect::<std::collections::BTreeMap<_, _>>();
    let mut rust_literal_counts = std::collections::BTreeMap::<String, usize>::new();
    let mut literal_syntax_counts = std::collections::BTreeMap::<String, usize>::new();
    rust_constant_part_tokens.into_iter().for_each(|token| {
        if let proc_macro2::TokenTree::Literal(value) = token
            && let Ok(value) = syn::parse_str::<syn::LitStr>(&value.to_string())
        {
            let count = rust_literal_counts.entry(value.value()).or_default();
            *count = count.saturating_add(constants_usize::ONE);
        }
    });
    declared_constant_part_tokens.iter().for_each(|token| {
        if let proc_macro2::TokenTree::Ident(name) = token
            && let Some(use_count) = rust_fragment_use_counts.get_mut(&name.to_string())
        {
            *use_count = use_count.saturating_add(constants_usize::ONE);
        }
    });
    constant_part_tokens
        .into_iter()
        .for_each(|token| match token {
            proc_macro2::TokenTree::Ident(name) => {
                if let Some(use_count) = fragment_use_counts.get_mut(&name.to_string()) {
                    *use_count = use_count.saturating_add(constants_usize::ONE);
                }
            }
            proc_macro2::TokenTree::Literal(value) => {
                if let Ok(value) = syn::parse_str::<syn::LitStr>(&value.to_string()) {
                    value
                        .value()
                        .split(|char: char| !char.is_ascii_alphanumeric() && char != '_')
                        .filter(|word| !word.is_empty())
                        .for_each(|word| {
                            let count = literal_word_counts.entry(word.to_owned()).or_default();
                            *count = count.saturating_add(constants_usize::ONE);
                        });
                }
            }
            _ => {}
        });
    declared_constant_part_tokens
        .into_iter()
        .filter_map(|token| match token {
            proc_macro2::TokenTree::Literal(value) => {
                syn::parse_str::<syn::LitStr>(&value.to_string()).ok()
            }
            _ => None,
        })
        .map(|value| value.value())
        .filter(|value| {
            !value.is_empty()
                && value
                    .chars()
                    .all(|char| !char.is_ascii_alphanumeric() && char != '_')
        })
        .for_each(|value| {
            let count = literal_syntax_counts.entry(value).or_default();
            *count = count.saturating_add(constants_usize::ONE);
        });
    assert!(!fragments.is_empty());
    assert!(!rust_fragments.is_empty());
    assert!(!literal_word_counts.is_empty());
    assert!(rust_fragments.values().all(|parts| parts.clone().into_iter().all(
        |token| !matches!(token, proc_macro2::TokenTree::Literal(value) if syn::parse_str::<syn::LitStr>(&value.to_string()).is_ok_and(|value| value.value().chars().any(|char| char.is_ascii_alphanumeric() || char == '_')))
    )));
    assert!(
        rust_fragment_use_counts
            .values()
            .all(|use_count| *use_count >= 2usize)
    );
    assert!(
        rust_literal_counts
            .values()
            .all(|use_count| *use_count == 1usize)
    );
    assert!(
        fragment_use_counts
            .values()
            .all(|use_count| *use_count >= 2usize)
    );
    assert!(
        literal_word_counts
            .values()
            .all(|use_count| *use_count == 1usize)
    );
    assert!(
        literal_word_counts
            .keys()
            .all(|word| !fragments.values().any(|fragment| fragment == word))
    );
    assert!(
        literal_syntax_counts
            .values()
            .all(|use_count| *use_count == constants_usize::ONE)
    );
    assert!(literal_syntax_counts.keys().all(|syntax| {
        !rust_fragments
            .values()
            .any(|parts| parts.to_string() == proc_macro2::Literal::string(syntax).to_string())
    }));
}

#[test]
fn test_str_constants_does_not_own_typed_domain_values() {
    let source = std::fs::read_to_string(constants_str::STR_CONSTANTS_SRC_LIB_RS)
        .expect(constants_str::DIAGNOSTIC_3CAA56A9);
    let errors = [
        concat!("ADMIN_API_", "PATHS_"),
        concat!("ADMIN_", "OPERATION_"),
        concat!("ADMIN_PAGE_", "PATHS_"),
        concat!("ADMIN_PERMISSION_", "VALUES_"),
    ]
    .into_iter()
    .filter(|prefix| source.contains(prefix))
    .map(str::to_owned)
    .collect::<Vec<_>>();
    crate::code_style::assert_joined_errors_empty_with_context(
        crate::types::SourceTextListRef::from(errors.as_slice()),
        crate::types::StaticStr::from(constants_str::VALUE_6B7E02A4),
        crate::types::SourceTextRef::from(
            constants_str::DOMAIN_VALUES_MUST_BE_DECLARED_BY_THEIR_OWNING_TYPED_API,
        ),
    );
}
#[test]
fn test_string_constant_visitor_checks_test_code_and_allows_reviewed_syntax_boundaries() {
    let ast = syn::parse_file(constants_str::CODE_STYLE_STRING_GUARD_ALLOWED_SYNTAX_FIXTURE)
        .expect(constants_str::DIAGNOSTIC_87C9A142);
    let visitor = crate::code_style::visit_syn_file(
        crate::types::SynFileRef::from(&ast),
        super::source_analysis::StringConstantVisitor::new(
            crate::types::DiagnosticMessages::default(),
        ),
    );
    assert_eq!(visitor.get_errors().len(), 3usize);
}
#[test]
fn test_string_constant_visitor_detects_expression_and_nested_macro_literals() {
    let ast = syn::parse_file(constants_str::CODE_STYLE_STRING_GUARD_DETECTION_FIXTURE)
        .expect(constants_str::DIAGNOSTIC_BC91574F);
    let visitor = crate::code_style::visit_syn_file(
        crate::types::SynFileRef::from(&ast),
        super::source_analysis::StringConstantVisitor::new(
            crate::types::DiagnosticMessages::default(),
        ),
    );
    assert_eq!(visitor.get_errors().len(), 2usize);
}
#[test]
fn test_string_constant_visitor_checks_assertion_operands_and_skips_messages() {
    let source = format!(
        "fn f() {{ assert!(fallible().expect(\"{}\")); assert_eq!(fallible().expect_err(\"{}\"), expected); assert_eq!(\"{}\", expected, \"{}\"); panic!(\"{}\"); }}",
        constants_str::VALUE_2DE961C6,
        constants_str::VALUE_0EF05B85,
        constants_str::VALUE_8448814D,
        constants_str::VALUE_9F8D72A1,
        constants_str::VALUE_3C31187B,
    );
    let ast = syn::parse_file(source.as_str()).expect(constants_str::DIAGNOSTIC_7C5E1A92);
    let visitor = crate::code_style::visit_syn_file(
        crate::types::SynFileRef::from(&ast),
        super::source_analysis::StringConstantVisitor::new(
            crate::types::DiagnosticMessages::default(),
        ),
    );
    assert_eq!(visitor.get_errors().len(), 4usize);
}
#[test]
fn test_tracing_message_visitor_checks_every_event_macro_and_test_module() {
    let ast = syn::parse_file(constants_str::CODE_STYLE_TRACING_MESSAGE_FIXTURE)
        .expect(constants_str::DIAGNOSTIC_5D7C4E2A);
    let visitor = crate::code_style::visit_syn_file(
        crate::types::SynFileRef::from(&ast),
        super::source_analysis::TracingMessageLiteralVisitor::default(),
    );
    assert_eq!(visitor.get_values().len(), 7usize);
}
#[test]
fn test_all_tracing_messages_are_declared_in_constants_str() {
    crate::code_style::assert_rs_ast_errors_empty_with_context(
        crate::types::StaticStr::from(constants_str::VALUE_6C2711FA),
        crate::types::SourceTextRef::from(
            constants_str::TRACING_MESSAGES_FOUND_OUTSIDE_CONSTANTS_STR,
        ),
        |path, ast, errors| {
            if crate::code_style::is_str_constants_source_path(crate::types::PathRef::from(path))
                .get()
                || crate::code_style::is_proc_macro_implementation_source_path(
                    crate::types::PathRef::from(path),
                )
                .get()
            {
                return;
            }
            let visitor = crate::code_style::visit_syn_file(
                crate::types::SynFileRef::from(ast),
                super::source_analysis::TracingMessageLiteralVisitor::default(),
            );
            errors.extend(
                visitor
                    .get_values()
                    .clone()
                    .into_iter()
                    .map(|message| format!("{}: {message:?}", path.display())),
            );
        },
    );
}
#[test]
fn test_all_string_constants_are_declared_in_str_constants() {
    crate::code_style::assert_rs_ast_errors_empty_with_context(
        crate::types::StaticStr::from(constants_str::VALUE_6C2711FA),
        crate::types::SourceTextRef::from(
            constants_str::STRING_CONSTANTS_FOUND_OUTSIDE_STR_CONSTANTS,
        ),
        |path, ast, errors| {
            if crate::code_style::is_str_constants_source_path(crate::types::PathRef::from(path))
                .get()
                || crate::code_style::is_proc_macro_implementation_source_path(
                    crate::types::PathRef::from(path),
                )
                .get()
            {
                return;
            }
            let visitor = crate::code_style::visit_syn_file(
                crate::types::SynFileRef::from(ast),
                super::source_analysis::StringConstantVisitor::new(
                    crate::types::DiagnosticMessages::default(),
                ),
            );
            errors.extend(
                visitor
                    .get_errors()
                    .clone()
                    .into_iter()
                    .map(|error| format!("{}: {error}", path.display())),
            );
            let declaration_visitor = crate::code_style::visit_syn_file(
                crate::types::SynFileRef::from(ast),
                super::source_analysis::StringConstantDeclarationVisitor::new(
                    crate::types::DiagnosticMessages::default(),
                    crate::types::AnalyzerBool::from(path.ends_with(
                        constants_str::CONSTANTS_STR_MACROS_SRC_DEFINE_STR_CONSTANTS_INPUT_RS,
                    )),
                ),
            );
            errors.extend(
                declaration_visitor
                    .get_errors()
                    .clone()
                    .into_iter()
                    .map(|error| format!("{}: {error}", path.display())),
            );
        },
    );
}
#[test]
fn test_string_constant_policy_has_only_the_constants_crate_source_directory_exception() {
    assert!(
        crate::code_style::is_str_constants_source_path(crate::types::PathRef::from(
            std::path::Path::new(constants_str::STR_CONSTANTS_SRC_LIB_RS,)
        ))
        .get()
    );
    assert!(
        [constants_str::VALUE_0143DFED, constants_str::VALUE_B278317D,]
            .into_iter()
            .all(|path| {
                !crate::code_style::is_str_constants_source_path(crate::types::PathRef::from(
                    std::path::Path::new(path),
                ))
                .get()
            })
    );
    assert!(
        crate::code_style::is_str_constants_source_path(crate::types::PathRef::from(
            std::path::Path::new(constants_str::VALUE_AE938FB0,)
        ))
        .get()
    );
}
#[test]
fn test_string_constant_declaration_policy_ignores_runtime_literals_and_rejects_all_const_forms() {
    let ast = syn::parse_file(constants_str::CODE_STYLE_STRING_CONSTANT_DECLARATION_FIXTURE)
        .expect(constants_str::DIAGNOSTIC_02EC1D16);
    let visitor = crate::code_style::visit_syn_file(
        crate::types::SynFileRef::from(&ast),
        super::source_analysis::StringConstantDeclarationVisitor::new(
            crate::types::DiagnosticMessages::default(),
            crate::types::AnalyzerBool::default(),
        ),
    );
    assert_eq!(visitor.get_errors().len(), 13usize);
}
#[test]
fn test_string_constant_declaration_policy_rejects_aliases_to_exported_constants() {
    let ast = syn::parse_file(constants_str::CODE_STYLE_STRING_CONSTANT_ALIAS_FIXTURE)
        .expect(constants_str::DIAGNOSTIC_56F8E2C1);
    let visitor = crate::code_style::visit_syn_file(
        crate::types::SynFileRef::from(&ast),
        super::source_analysis::StringConstantDeclarationVisitor::new(
            crate::types::DiagnosticMessages::default(),
            crate::types::AnalyzerBool::default(),
        ),
    );
    assert_eq!(visitor.get_errors().len(), constants_usize::ONE);
}
#[test]
fn test_no_unwrap_in_source_code() {
    crate::code_style::assert_rs_ast_errors_empty_with_context(
        crate::types::StaticStr::from(constants_str::E8B3A6D2),
        crate::types::SourceTextRef::from(constants_str::UNWRAP_FOUND),
        |path, ast, errors| {
            let visitor = crate::code_style::visit_syn_file(
                crate::types::SynFileRef::from(ast),
                super::source_analysis::UnwrapVisitor::new(crate::types::AnalyzerCount::default()),
            );
            crate::code_style::push_repeated_file_error(
                crate::types::DiagnosticMessagesMutRef::from(&mut *errors),
                crate::types::PathRef::from(path),
                crate::types::SourceTextRef::from(constants_str::UNWRAP_CALL_ALT),
                *visitor.get_found_count(),
            );
        },
    );
}
#[test]
fn test_no_unstable_sorting_methods_in_source_code() {
    super::test_code_style_snapshot::with_codebase_snapshot(|snapshot| {
        let violations = snapshot
            .rs_files()
            .iter()
            .filter(|file| {
                file.content()
                    .as_ref()
                    .contains(constants_str::SORT_UNSTABLE_METHOD_PREFIX)
            })
            .map(|file| file.path().as_ref().display().to_string())
            .collect::<Vec<_>>();
        assert!(violations.is_empty(), "f3a91c7e {violations:?}");
    });
}
#[test]
fn test_unstable_sorting_policy_covers_every_variant() {
    assert!(
        [
            constants_str::SORT_UNSTABLE_METHOD_FIXTURE,
            constants_str::SORT_UNSTABLE_BY_METHOD_FIXTURE,
            constants_str::SORT_UNSTABLE_BY_KEY_METHOD_FIXTURE,
        ]
        .iter()
        .all(|source| source.contains(constants_str::SORT_UNSTABLE_METHOD_PREFIX)),
        "8ac60d31"
    );
    assert!(
        !constants_str::SORT_STABLE_METHOD_FIXTURE
            .contains(constants_str::SORT_UNSTABLE_METHOD_PREFIX),
        "b147e9a2"
    );
}
#[test]
fn test_repository_identifiers_use_explicit_resource_names() {
    #[derive(
        proc_macro_getters::Getters, Default, proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    )]
    struct ExplicitResourceNameVisitor {
        violations: crate::types::SourceTextList,
    }

    impl<'ast_lt> syn::visit::Visit<'ast_lt> for ExplicitResourceNameVisitor {
        fn visit_ident(&mut self, ident: &'ast_lt syn::Ident) {
            let name = ident.to_string();
            let Some(vague_fragment) = stringify!(JoinHandle).strip_prefix(stringify!(Join)) else {
                return;
            };
            if name.to_ascii_lowercase().contains(vague_fragment)
                && !matches!(
                    name.as_str(),
                    stringify!(Handler)
                        | stringify!(JoinHandle)
                        | stringify!(PrometheusHandle)
                        | stringify!(ScopedJoinHandle)
                        | stringify!(handle)
                        | stringify!(handler)
                )
            {
                self.violations.push(name);
            }
            syn::visit::visit_ident(self, ident);
        }
    }

    super::test_code_style_snapshot::with_codebase_snapshot(|snapshot| {
        let violations = snapshot
            .rs_files()
            .iter()
            .flat_map(|file| {
                let mut visitor = ExplicitResourceNameVisitor::default();
                syn::visit::Visit::visit_file(&mut visitor, file.ast().as_ref());
                visitor
                    .get_violations()
                    .clone()
                    .into_iter()
                    .map(|identifier| format!("{}: {identifier}", file.path().as_ref().display()))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        assert!(
            violations.is_empty(),
            "repository identifiers must name their concrete resource or endpoint role:\n{}",
            violations.join("\n")
        );
    });
}

#[test]
fn test_trivial_new_constructors_are_generated() {
    crate::code_style::assert_rs_ast_errors_empty_with_context(
        crate::types::StaticStr::from(constants_str::DIAGNOSTIC_C5142DF8),
        crate::types::SourceTextRef::from(constants_str::CODE_STYLE_TRIVIAL_NEW_CONSTRUCTOR),
        |path, ast, errors| {
            let visitor = crate::code_style::visit_syn_file(
                crate::types::SynFileRef::from(ast),
                super::source_analysis::TrivialNewConstructorVisitor::new(
                    crate::types::DiagnosticMessages::default(),
                ),
            );
            errors.extend(
                visitor
                    .get_errors()
                    .iter()
                    .map(|error| format!("{}: {error}", path.display())),
            );
        },
    );
}

#[test]
fn test_trivial_new_constructor_policy_rejects_direct_field_initialization() {
    let ast: syn::File = syn::parse_quote! {
        struct Fixture {
            first: usize,
            second: bool,
        }
        impl Fixture {
            fn new(second_value: bool, first_value: usize) -> Self {
                Self {
                    first: first_value,
                    second: second_value,
                }
            }
        }
    };
    let visitor = crate::code_style::visit_syn_file(
        crate::types::SynFileRef::from(&ast),
        super::source_analysis::TrivialNewConstructorVisitor::new(
            crate::types::DiagnosticMessages::default(),
        ),
    );
    assert_eq!(visitor.get_errors().len(), constants_usize::ONE);
}

#[test]
fn test_trivial_new_constructor_policy_allows_constructor_logic() {
    let ast: syn::File = syn::parse_quote! {
        struct Fixture {
            value: usize,
        }
        impl Fixture {
            fn new(value: usize) -> Self {
                Self { value: value + constants_usize::ONE }
            }
        }
    };
    let visitor = crate::code_style::visit_syn_file(
        crate::types::SynFileRef::from(&ast),
        super::source_analysis::TrivialNewConstructorVisitor::new(
            crate::types::DiagnosticMessages::default(),
        ),
    );
    assert!(visitor.get_errors().is_empty());
}

#[test]
fn test_trivial_new_constructor_policy_allows_trait_implementation() {
    let ast: syn::File = syn::parse_quote! {
        trait Factory {
            fn new(value: usize) -> Self;
        }
        struct Fixture {
            value: usize,
        }
        impl Factory for Fixture {
            fn new(value: usize) -> Self {
                Self { value }
            }
        }
    };
    let visitor = crate::code_style::visit_syn_file(
        crate::types::SynFileRef::from(&ast),
        super::source_analysis::TrivialNewConstructorVisitor::new(
            crate::types::DiagnosticMessages::default(),
        ),
    );
    assert!(visitor.get_errors().is_empty());
}
#[test]
fn test_names_and_modules_start_with_test_prefix() {
    crate::code_style::assert_rs_ast_errors_empty_with_context(
        crate::types::StaticStr::from(constants_str::B6E2A9F4),
        crate::types::SourceTextRef::from(constants_str::TEST_NAME_POLICY_CONTEXT),
        |path, ast, errors| {
            let visitor = crate::code_style::visit_syn_file(
                crate::types::SynFileRef::from(ast),
                super::source_analysis::TestNameVisitor::new(
                    crate::types::DiagnosticMessages::default(),
                    crate::types::SourceTextList::default(),
                    crate::types::AnalyzerBool::default(),
                ),
            );
            errors.extend(
                visitor
                    .get_errors()
                    .iter()
                    .map(|error| format!("{}: {error}", path.display())),
            );
            if visitor.get_root_test_found().get()
                && path
                    .file_stem()
                    .and_then(std::ffi::OsStr::to_str)
                    .is_none_or(|name| !name.starts_with(constants_str::TEST_NAME_PREFIX))
            {
                errors.push(format!(
                    "{}: root test module must use a filename starting with `test_`",
                    path.display()
                ));
            }
        },
    );
}
#[test]
fn test_name_policy_rejects_unprefixed_function_and_module() {
    let ast: syn::File = syn::parse_quote! {
        mod checks {
            #[test]
            fn works() {}
        }
    };
    let visitor = crate::code_style::visit_syn_file(
        crate::types::SynFileRef::from(&ast),
        super::source_analysis::TestNameVisitor::new(
            crate::types::DiagnosticMessages::default(),
            crate::types::SourceTextList::default(),
            crate::types::AnalyzerBool::default(),
        ),
    );
    assert_eq!(visitor.get_errors().len(), constants_usize::TWO);
}

#[test]
fn test_name_policy_accepts_canonical_tests_module() {
    let ast: syn::File = syn::parse_quote! {
        mod tests {
            #[test]
            fn test_works() {}
        }
    };
    let visitor = crate::code_style::visit_syn_file(
        crate::types::SynFileRef::from(&ast),
        super::source_analysis::TestNameVisitor::new(
            crate::types::DiagnosticMessages::default(),
            crate::types::SourceTextList::default(),
            crate::types::AnalyzerBool::default(),
        ),
    );
    assert!(visitor.get_errors().is_empty());
}

#[test]
fn test_name_policy_rejects_redundant_test_tests_module() {
    let ast = syn::parse_file(constants_str::TEST_TESTS_MODULE_FIXTURE)
        .expect(constants_str::DIAGNOSTIC_60A9F21C);
    let visitor = crate::code_style::visit_syn_file(
        crate::types::SynFileRef::from(&ast),
        super::source_analysis::TestNameVisitor::new(
            crate::types::DiagnosticMessages::default(),
            crate::types::SourceTextList::default(),
            crate::types::AnalyzerBool::default(),
        ),
    );
    assert_eq!(visitor.get_errors().len(), constants_usize::ONE);
}

#[test]
#[allow(
    clippy::arbitrary_source_item_ordering,
    reason = "the comment policy tests stay beside the visitor declarations they exercise"
)]
fn test_rust_source_contains_no_comments() {
    super::test_code_style_snapshot::with_codebase_snapshot(|snapshot| {
        let violations = snapshot
            .rs_files()
            .iter()
            .filter_map(|source_file| {
                crate::source_analysis::rust_comment_position(source_file.content().as_ref()).map(
                    |position| {
                        let line = source_file
                            .content()
                            .as_ref()
                            .get(..position)
                            .into_iter()
                            .flat_map(|prefix| prefix.bytes())
                            .filter(|byte| *byte == b'\n')
                            .count()
                            .saturating_add(constants_usize::ONE);
                        format!("{}:{line}", source_file.path().as_ref().display())
                    },
                )
            })
            .collect::<Vec<String>>();
        crate::code_style::assert_joined_errors_empty(
            crate::types::SourceTextListRef::from(violations.as_slice()),
            crate::types::StaticStr::from(constants_str::RUST_SOURCE_COMMENTS_ARE_FORBIDDEN),
        );
    });
}

#[test]
#[allow(
    clippy::arbitrary_source_item_ordering,
    reason = "the comment policy tests stay beside the visitor declarations they exercise"
)]
fn test_rust_comment_policy_distinguishes_comments_from_literals() {
    let line_comment = ['/', '/'].into_iter().collect::<String>();
    let block_comment = ['/', '*'].into_iter().collect::<String>();
    assert_eq!(
        crate::source_analysis::rust_comment_position(line_comment.as_str()),
        Some(0usize)
    );
    assert_eq!(
        crate::source_analysis::rust_comment_position(block_comment.as_str()),
        Some(0usize)
    );
    assert_eq!(
        crate::source_analysis::rust_comment_position(
            format!(r##"let values = ("{line_comment}", r#"{block_comment}"#);"##).as_str(),
        ),
        None
    );
}

#[test]
fn test_empty_function_bodies_are_forbidden() {
    super::test_code_style_snapshot::with_codebase_snapshot(|snapshot| {
        let violations = snapshot
            .rs_files()
            .iter()
            .flat_map(|source_file| {
                let visitor = crate::code_style::visit_syn_file(
                    crate::types::SynFileRef::from(source_file.ast().as_ref()),
                    EmptyFunctionBodyVisitor::default(),
                );
                visitor
                    .get_violations()
                    .clone()
                    .into_iter()
                    .map(|violation| {
                        format!("{}:{violation}", source_file.path().as_ref().display())
                    })
            })
            .collect::<Vec<String>>();
        crate::code_style::assert_joined_errors_empty(
            crate::types::SourceTextListRef::from(violations.as_slice()),
            crate::types::StaticStr::from(constants_str::EMPTY_FUNCTIONS_ARE_FORBIDDEN),
        );
    });
}

#[test]
fn test_empty_function_body_policy_checks_functions_methods_and_trait_defaults() {
    let ast: syn::File = syn::parse_quote! {
        fn test_free() {}
        struct TestType;
        impl TestType {
            fn test_method() {}
        }
        trait TestTrait {
            fn test_required();
            fn test_default() {}
        }
    };
    let visitor = crate::code_style::visit_syn_file(
        crate::types::SynFileRef::from(&ast),
        EmptyFunctionBodyVisitor::default(),
    );
    assert_eq!(visitor.get_violations().len(), 3usize);
}
