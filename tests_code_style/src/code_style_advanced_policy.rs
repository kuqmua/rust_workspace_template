#![allow(
    clippy::arbitrary_source_item_ordering,
    clippy::needless_for_each,
    clippy::wildcard_enum_match_arm,
    reason = "policy visitors stay grouped with their invariant, repository policy requires iterator methods, and syn non-exhaustive enums require fallback handling"
)]

#[derive(optimal_memory_layout::OptimalMemoryLayout, Default)]
struct AwaitVisitor {
    found: crate::types::AnalyzerBool,
}
impl<'ast> syn::visit::Visit<'ast> for AwaitVisitor {
    fn visit_expr_await(&mut self, i: &'ast syn::ExprAwait) {
        self.found.set_true();
        syn::visit::visit_expr_await(self, i);
    }
}

#[derive(optimal_memory_layout::OptimalMemoryLayout, Default)]
struct LockAcrossAwaitVisitor {
    violations: crate::types::DiagnosticMsgs,
}
impl<'ast> syn::visit::Visit<'ast> for LockAcrossAwaitVisitor {
    fn visit_block(&mut self, i: &'ast syn::Block) {
        let mut active_guards = std::collections::BTreeSet::<String>::new();
        i.stmts.iter().for_each(|statement| {
            let mut await_visitor = AwaitVisitor::default();
            syn::visit::Visit::visit_stmt(&mut await_visitor, statement);
            if await_visitor.found.get() && !active_guards.is_empty() {
                self.violations.push(format!(
                    "lock guards held across await: {}",
                    active_guards
                        .iter()
                        .map(String::as_str)
                        .collect::<Vec<&str>>()
                        .join(", ")
                ));
            }
            if let syn::Stmt::Local(local) = statement
                && local
                    .init
                    .as_ref()
                    .is_some_and(|initializer| expression_acquires_lock(initializer.expr.as_ref()))
                && let syn::Pat::Ident(identifier) = &local.pat
            {
                let _inserted = active_guards.insert(identifier.ident.to_string());
            }
            (|| {
                let syn::Stmt::Expr(syn::Expr::Call(call), _) = statement else {
                    return None;
                };
                let syn::Expr::Path(function) = call.func.as_ref() else {
                    return None;
                };
                if !matches!(
                    function
                        .path
                        .segments
                        .last()
                        .map(|segment| segment.ident.to_string())
                        .as_deref(),
                    Some(constants_str::test_fixtures::VALUE_D90EE9CC)
                ) {
                    return None;
                }
                let syn::Expr::Path(argument) = call.args.first()? else {
                    return None;
                };
                (argument.path.segments.len() == constants_usize::ONE)
                    .then(|| {
                        argument.path.segments.first().map(|segment| {
                            crate::types::SourceText::try_from(segment.ident.to_string())
                                .expect("d4f6bdce dropped identifier invariant must hold")
                        })
                    })
                    .flatten()
            })()
            .into_iter()
            .for_each(|identifier| {
                let _removed = active_guards.remove(identifier.as_ref());
            });
        });
        syn::visit::visit_block(self, i);
    }
}

fn expression_acquires_lock(expression: &syn::Expr) -> bool {
    match expression {
        syn::Expr::Await(await_expression) => {
            expression_acquires_lock(await_expression.base.as_ref())
        }
        syn::Expr::MethodCall(call) => {
            call.args.is_empty()
                && matches!(
                    call.method.to_string().as_str(),
                    constants_str::test_fixtures::VALUE_0C030586
                        | constants_str::test_fixtures::VALUE_DB488AC5
                        | constants_str::catalog::PG_CRUD_READ_PERMISSION_ACTION
                        | constants_str::test_fixtures::VALUE_35D47C1A
                        | constants_str::catalog::WRITE_ALT
                        | constants_str::test_fixtures::VALUE_FC58C841
                )
        }
        syn::Expr::Paren(paren) => expression_acquires_lock(paren.expr.as_ref()),
        syn::Expr::Try(try_expression) => expression_acquires_lock(try_expression.expr.as_ref()),
        _ => false,
    }
}

#[derive(optimal_memory_layout::OptimalMemoryLayout, Default)]
struct LeakApiVisitor {
    violations: crate::types::DiagnosticMsgs,
}
impl<'ast> syn::visit::Visit<'ast> for LeakApiVisitor {
    fn visit_expr_call(&mut self, i: &'ast syn::ExprCall) {
        if let syn::Expr::Path(function) = i.func.as_ref() {
            let path = function
                .path
                .segments
                .iter()
                .map(|segment| segment.ident.to_string())
                .collect::<Vec<String>>()
                .join(constants_str::catalog::PATH_SEPARATOR);
            if [
                constants_str::test_fixtures::VALUE_C26EBF7F,
                constants_str::test_fixtures::VALUE_5188A49C,
                constants_str::test_fixtures::VALUE_9C055078,
                constants_str::test_fixtures::VALUE_FEE41E56,
                constants_str::test_fixtures::VALUE_FA94FFC8,
                constants_str::test_fixtures::VALUE_2E8E6C33,
                constants_str::test_fixtures::VALUE_30F0E257,
                constants_str::test_fixtures::VALUE_86C84494,
                constants_str::test_fixtures::VALUE_58CAC57E,
                constants_str::test_fixtures::VALUE_36F221B5,
                constants_str::test_fixtures::VALUE_AF4FFF7C,
                constants_str::test_fixtures::VALUE_6D10B254,
            ]
            .contains(&path.as_str())
            {
                self.violations.push(path);
            }
        }
        syn::visit::visit_expr_call(self, i);
    }
    fn visit_type_path(&mut self, i: &'ast syn::TypePath) {
        if i.path
            .segments
            .last()
            .is_some_and(|segment| segment.ident == constants_str::test_fixtures::VALUE_6462221C)
        {
            self.violations
                .push(constants_str::test_fixtures::VALUE_6462221C.to_owned());
        }
        syn::visit::visit_type_path(self, i);
    }
}

#[derive(optimal_memory_layout::OptimalMemoryLayout, Default)]
struct SpawnConsumptionVisitor {
    consumed: crate::types::SourceTextBTreeSet,
}
impl SpawnConsumptionVisitor {
    fn record_path(&mut self, expression: &syn::Expr) {
        if let syn::Expr::Path(path) = expression
            && path.path.segments.len() == constants_usize::ONE
            && let Some(segment) = path.path.segments.first()
        {
            let _inserted = self.consumed.insert(segment.ident.to_string());
        }
    }
    fn record_macro_tokens(&mut self, tokens: proc_macro2::TokenStream) {
        let trees = tokens.into_iter().collect::<Vec<proc_macro2::TokenTree>>();
        trees.iter().for_each(|token| {
            if let proc_macro2::TokenTree::Group(group) = token {
                self.record_macro_tokens(group.stream());
            }
        });
        trees.windows(3usize).for_each(|window| {
            if let [
                proc_macro2::TokenTree::Ident(identifier),
                proc_macro2::TokenTree::Punct(dot),
                proc_macro2::TokenTree::Ident(operation),
            ] = window
                && dot.as_char() == '.'
                && matches!(
                    operation.to_string().as_str(),
                    constants_str::test_fixtures::VALUE_3A53DB8A
                        | constants_str::test_fixtures::VALUE_1AEFE47E
                )
            {
                let _inserted = self.consumed.insert(identifier.to_string());
            }
        });
    }
}
impl<'ast> syn::visit::Visit<'ast> for SpawnConsumptionVisitor {
    fn visit_expr_await(&mut self, i: &'ast syn::ExprAwait) {
        self.record_path(i.base.as_ref());
        syn::visit::visit_expr_await(self, i);
    }
    fn visit_expr_call(&mut self, i: &'ast syn::ExprCall) {
        let is_drop = matches!(
            i.func.as_ref(),
            syn::Expr::Path(path)
                if path.path.segments.last().is_some_and(|segment| segment.ident == constants_str::test_fixtures::VALUE_D90EE9CC)
        );
        if !is_drop {
            i.args
                .iter()
                .for_each(|argument| self.record_path(argument));
        }
        syn::visit::visit_expr_call(self, i);
    }
    fn visit_expr_method_call(&mut self, i: &'ast syn::ExprMethodCall) {
        if i.method == constants_str::test_fixtures::VALUE_3A53DB8A {
            self.record_path(i.receiver.as_ref());
        }
        syn::visit::visit_expr_method_call(self, i);
    }
    fn visit_field_value(&mut self, i: &'ast syn::FieldValue) {
        self.record_path(&i.expr);
        syn::visit::visit_field_value(self, i);
    }
    fn visit_macro(&mut self, i: &'ast syn::Macro) {
        self.record_macro_tokens(i.tokens.clone());
        syn::visit::visit_macro(self, i);
    }
}

#[derive(optimal_memory_layout::OptimalMemoryLayout, Default)]
struct SpawnLifecycleVisitor {
    violations: crate::types::DiagnosticMsgs,
}
impl<'ast> syn::visit::Visit<'ast> for SpawnLifecycleVisitor {
    fn visit_block(&mut self, i: &'ast syn::Block) {
        let mut pending = std::collections::BTreeSet::<String>::new();
        i.stmts.iter().for_each(|statement| {
            let mut consumption = SpawnConsumptionVisitor::default();
            syn::visit::Visit::visit_stmt(&mut consumption, statement);
            consumption.consumed.into_iter().for_each(|identifier| {
                let _removed = pending.remove(identifier.as_str());
            });
            if let syn::Stmt::Local(local) = statement
                && local.init.as_ref().is_some_and(|initializer| {
                    crate::code_style::unowned_spawn_expr(initializer.expr.as_ref())
                })
                && let syn::Pat::Ident(identifier) = &local.pat
                && !identifier.ident.to_string().starts_with('_')
            {
                let _inserted = pending.insert(identifier.ident.to_string());
            }
        });
        pending.into_iter().for_each(|identifier| {
            self.violations.push(format!(
                "spawned task `{identifier}` is retained but never awaited, aborted, or transferred to an owner"
            ));
        });
        syn::visit::visit_block(self, i);
    }
}

#[derive(optimal_memory_layout::OptimalMemoryLayout, Default)]
struct RouteLiteralVisitor {
    violations: crate::types::DiagnosticMsgs,
}
impl RouteLiteralVisitor {
    fn inspect_literal(&mut self, literal: &syn::LitStr) {
        let value = literal.value();
        if !value.starts_with('/')
            || value.starts_with(constants_str::test_fixtures::VALUE_A2C23396)
        {
            return;
        }
        if value == constants_str::test_fixtures::VALUE_702ACF7C
            || value.starts_with(constants_str::test_fixtures::VALUE_4D3A663E)
        {
            self.violations.push(format!(
                "route `{value}` must not use the removed `/api` prefix"
            ));
        }
        value
            .split('/')
            .filter(|segment| !segment.is_empty())
            .filter(|segment| !(segment.starts_with('{') && segment.ends_with('}')))
            .filter(|segment| {
                !segment.split('.').all(|part| {
                    !part.is_empty()
                        && part.bytes().all(|byte| {
                            byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'_'
                        })
                })
            })
            .for_each(|segment| {
                self.violations.push(format!(
                    "route segment `{segment}` in `{value}` is not snake_case"
                ));
            });
    }
    fn inspect_tokens(&mut self, tokens: proc_macro2::TokenStream) {
        tokens.into_iter().for_each(|token| match token {
            proc_macro2::TokenTree::Group(group) => self.inspect_tokens(group.stream()),
            proc_macro2::TokenTree::Literal(literal) => {
                if let Ok(parsed) = syn::parse_str::<syn::LitStr>(literal.to_string().as_str()) {
                    self.inspect_literal(&parsed);
                }
            }
            proc_macro2::TokenTree::Ident(_) | proc_macro2::TokenTree::Punct(_) => {}
        });
    }
}
impl<'ast> syn::visit::Visit<'ast> for RouteLiteralVisitor {
    fn visit_attribute(&mut self, i: &'ast syn::Attribute) {
        if i.path().segments.last().is_some_and(|segment| {
            matches!(
                segment.ident.to_string().as_str(),
                constants_str::test_fixtures::VALUE_BDE31E29
                    | constants_str::test_fixtures::VALUE_2466624A
            )
        }) && let syn::Meta::List(list) = &i.meta
        {
            self.inspect_tokens(list.tokens.clone());
        }
        syn::visit::visit_attribute(self, i);
    }
    fn visit_expr_method_call(&mut self, i: &'ast syn::ExprMethodCall) {
        if matches!(
            i.method.to_string().as_str(),
            constants_str::test_fixtures::VALUE_8A84E406
                | constants_str::test_fixtures::VALUE_75EF2E32
                | constants_str::test_fixtures::VALUE_84BBA14A
        ) && let Some(syn::Expr::Lit(syn::ExprLit {
            lit: syn::Lit::Str(literal),
            ..
        })) = i.args.first()
        {
            self.inspect_literal(literal);
        }
        syn::visit::visit_expr_method_call(self, i);
    }
}

#[derive(optimal_memory_layout::OptimalMemoryLayout, Default)]
struct SelectMacroVisitor {
    count: crate::types::AnalyzerCount,
    unsafe_operations: crate::types::DiagnosticMsgs,
}
impl SelectMacroVisitor {
    fn inspect_sensitive_tokens(&mut self, tokens: proc_macro2::TokenStream) {
        tokens.into_iter().for_each(|token| match token {
            proc_macro2::TokenTree::Group(group) => {
                self.inspect_sensitive_tokens(group.stream());
            }
            proc_macro2::TokenTree::Ident(identifier)
                if matches!(
                    identifier.to_string().as_str(),
                    constants_str::test_fixtures::VALUE_574C97CF
                        | constants_str::test_fixtures::VALUE_EB83DC1A
                        | constants_str::test_fixtures::VALUE_8882BF3F
                        | constants_str::test_fixtures::VALUE_6DF24C37
                        | constants_str::test_fixtures::VALUE_27CE1D1B
                        | constants_str::test_fixtures::VALUE_86F7474B
                ) =>
            {
                self.unsafe_operations.push(format!(
                    "tokio::select contains cancellation-sensitive `{identifier}`"
                ));
            }
            proc_macro2::TokenTree::Ident(_)
            | proc_macro2::TokenTree::Literal(_)
            | proc_macro2::TokenTree::Punct(_) => {}
        });
    }
}
impl<'ast> syn::visit::Visit<'ast> for SelectMacroVisitor {
    fn visit_macro(&mut self, i: &'ast syn::Macro) {
        let is_select = i
            .path
            .segments
            .last()
            .is_some_and(|segment| segment.ident == constants_str::catalog::SELECT_ALT_3);
        if is_select {
            self.count.saturating_inc();
            self.inspect_sensitive_tokens(i.tokens.clone());
        }
        syn::visit::visit_macro(self, i);
    }
}

#[derive(optimal_memory_layout::OptimalMemoryLayout, Default)]
struct ExpressionPathVisitor {
    paths: crate::types::SourceTextList,
}
impl<'ast> syn::visit::Visit<'ast> for ExpressionPathVisitor {
    fn visit_expr_path(&mut self, i: &'ast syn::ExprPath) {
        self.paths.push(
            crate::code_style::path_to_string(crate::types::SynPathRef::from(&i.path))
                .as_ref()
                .to_owned(),
        );
        syn::visit::visit_expr_path(self, i);
    }
}

#[derive(optimal_memory_layout::OptimalMemoryLayout, Default)]
struct IgnoredMapErrBindingVisitor {
    entries: crate::types::DiagnosticMsgs,
}
impl<'ast> syn::visit::Visit<'ast> for IgnoredMapErrBindingVisitor {
    fn visit_expr_method_call(&mut self, i: &'ast syn::ExprMethodCall) {
        if i.method == constants_str::catalog::CODE_STYLE_MAP_ERR
            && let Some(syn::Expr::Closure(closure)) = i.args.first()
        {
            let ignored_inputs = closure
                .inputs
                .iter()
                .filter_map(|input| match input {
                    syn::Pat::Wild(_) => Some(constants_str::catalog::UNDERSCORE.to_owned()),
                    syn::Pat::Ident(identifier)
                        if identifier.ident.to_string().starts_with('_') =>
                    {
                        Some(identifier.ident.to_string())
                    }
                    _ => None,
                })
                .collect::<Vec<String>>();
            if !ignored_inputs.is_empty() {
                let mut path_visitor = ExpressionPathVisitor::default();
                syn::visit::Visit::visit_expr(&mut path_visitor, closure.body.as_ref());
                path_visitor.paths.sort();
                self.entries.push(format!(
                    "line {}: {} => {}",
                    syn::spanned::Spanned::span(i).start().line,
                    ignored_inputs.join(","),
                    path_visitor.paths.join(",")
                ));
            }
        }
        syn::visit::visit_expr_method_call(self, i);
    }
}

#[derive(optimal_memory_layout::OptimalMemoryLayout, Default)]
struct RawVecTupleWrapperVisitor {
    identifiers: crate::types::SourceTextList,
}

#[derive(optimal_memory_layout::OptimalMemoryLayout, Default)]
struct FromVecImplVisitor {
    targets: crate::types::SourceTextList,
}
impl<'ast> syn::visit::Visit<'ast> for FromVecImplVisitor {
    fn visit_item_impl(&mut self, i: &'ast syn::ItemImpl) {
        if let Some((trait_path, _)) = &i.trait_
            && let Some(trait_segment) = trait_path.segments.last()
            && trait_segment.ident == constants_str::catalog::FROM_ALT_3
            && let syn::PathArguments::AngleBracketed(arguments) = &trait_segment.arguments
            && let Some(syn::GenericArgument::Type(syn::Type::Path(value_type))) =
                arguments.args.first()
            && value_type
                .path
                .segments
                .last()
                .is_some_and(|segment| segment.ident == constants_str::catalog::VEC)
        {
            self.targets.push(format!(
                "line {}",
                syn::spanned::Spanned::span(i).start().line
            ));
        }
        syn::visit::visit_item_impl(self, i);
    }
}
impl<'ast> syn::visit::Visit<'ast> for RawVecTupleWrapperVisitor {
    fn visit_item_struct(&mut self, i: &'ast syn::ItemStruct) {
        if let syn::Fields::Unnamed(fields) = &i.fields
            && fields.unnamed.len() == constants_usize::ONE
            && let Some(field) = fields.unnamed.first()
            && let syn::Type::Path(path) = &field.ty
            && path
                .path
                .segments
                .last()
                .is_some_and(|segment| segment.ident == constants_str::catalog::VEC)
        {
            self.identifiers.push(i.ident.to_string());
        }
        syn::visit::visit_item_struct(self, i);
    }
}

#[derive(optimal_memory_layout::OptimalMemoryLayout, Default)]
struct UsizeMaxExprVisitor {
    count: crate::types::AnalyzerCount,
}
impl<'ast> syn::visit::Visit<'ast> for UsizeMaxExprVisitor {
    fn visit_expr_path(&mut self, i: &'ast syn::ExprPath) {
        let mut segments = i.path.segments.iter();
        if segments
            .next()
            .is_some_and(|segment| segment.ident == constants_str::catalog::CODE_STYLE_USIZE)
            && segments.next().is_some_and(|segment| {
                segment.ident == constants_str::test_fixtures::VALUE_2D9C014A
            })
            && segments.next().is_none()
        {
            self.count.saturating_inc();
        }
        syn::visit::visit_expr_path(self, i);
    }

    fn visit_item_mod(&mut self, i: &'ast syn::ItemMod) {
        if crate::code_style::attrs_contain_test_only_cfg(crate::types::SynAttributeListRef::from(
            i.attrs.as_slice(),
        ))
        .get()
        {
            return;
        }
        syn::visit::visit_item_mod(self, i);
    }
}

#[derive(optimal_memory_layout::OptimalMemoryLayout, Default)]
struct SharedDispatchVisitor {
    arc_types: crate::types::AnalyzerCount,
    lock_types: crate::types::AnalyzerCount,
    trait_objects: crate::types::AnalyzerCount,
}
impl<'ast> syn::visit::Visit<'ast> for SharedDispatchVisitor {
    fn visit_type_path(&mut self, i: &'ast syn::TypePath) {
        if let Some(segment) = i.path.segments.last() {
            match segment.ident.to_string().as_str() {
                constants_str::catalog::ARC => self.arc_types.saturating_inc(),
                constants_str::catalog::MUTEX | constants_str::test_fixtures::VALUE_02DF7EC2 => {
                    self.lock_types.saturating_inc();
                }
                _ => {}
            }
        }
        syn::visit::visit_type_path(self, i);
    }
    fn visit_type_trait_object(&mut self, i: &'ast syn::TypeTraitObject) {
        self.trait_objects.saturating_inc();
        syn::visit::visit_type_trait_object(self, i);
    }
}

#[derive(optimal_memory_layout::OptimalMemoryLayout)]
struct PublicApiVisitor {
    entries: crate::types::SourceTextList,
    lines: crate::types::SourceTextList,
}
impl PublicApiVisitor {
    fn source(&self, span: proc_macro2::Span) -> crate::types::SourceText {
        let start = span.start().line.saturating_sub(constants_usize::ONE);
        let end = span.end().line;
        let normalized = self
            .lines
            .get(start..end)
            .map(|lines| lines.join(constants_str::catalog::NEWLINE))
            .expect("c9d73e55 source invariant must hold")
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(constants_str::catalog::SPACE);
        crate::types::SourceText::try_from(normalized).expect("31f04bb7 source invariant must hold")
    }
    fn field_type(&self, field: &syn::Field) -> crate::types::SourceText {
        let source = self.source(syn::spanned::Spanned::span(field));
        let field_type = source
            .as_ref()
            .split_once(':')
            .map(|(_field, field_type)| field_type.trim().trim_end_matches(',').to_owned())
            .expect("5af91e82 field_type invariant must hold");
        crate::types::SourceText::try_from(field_type)
            .expect("3e2d89ef field_type invariant must hold")
    }
    fn record(&mut self, span: proc_macro2::Span, signature_only: bool) {
        let start = span.start().line.saturating_sub(constants_usize::ONE);
        let end = span.end().line;
        let source = self
            .lines
            .get(start..end)
            .map(|lines| lines.join(constants_str::catalog::NEWLINE))
            .expect("3e180abf record invariant must hold");
        let relevant = if signature_only {
            source
                .split_once('{')
                .map_or(source.as_str(), |(signature, _body)| signature)
        } else {
            source.as_str()
        };
        self.entries.push(
            relevant
                .split_whitespace()
                .collect::<Vec<&str>>()
                .join(constants_str::catalog::SPACE),
        );
    }
    fn record_contract_struct_api(&mut self, item: &syn::ItemStruct) {
        let Some(attribute) = item.attrs.iter().find(|attribute| {
            attribute
                .path()
                .is_ident(constants_str::test_fixtures::VALUE_21E85007)
        }) else {
            return;
        };
        let mut constructor = false;
        let mut into_parts = false;
        attribute
            .parse_nested_meta(|metadata| {
                if metadata.path.is_ident(constants_str::catalog::NEW) {
                    constructor = true;
                }
                if metadata
                    .path
                    .is_ident(constants_str::test_fixtures::VALUE_1E3D0F4B)
                {
                    into_parts = true;
                }
                Ok(())
            })
            .expect("d932a5f1 record_contract_struct_api invariant must hold");
        let syn::Fields::Named(fields) = &item.fields else {
            return;
        };
        let identifiers = fields
            .named
            .iter()
            .filter_map(|field| field.ident.as_ref())
            .collect::<Vec<_>>();
        let types = fields
            .named
            .iter()
            .map(|field| self.field_type(field))
            .collect::<Vec<_>>();
        if constructor {
            let parameters = identifiers
                .iter()
                .zip(types.iter())
                .map(|(identifier, field_type)| format!("{identifier}: {}", field_type.as_ref()))
                .collect::<Vec<_>>()
                .join(constants_str::catalog::TEXT_ALT_6);
            self.entries.push(format!(
                "#[must_use] pub const fn new({parameters}) -> Self"
            ));
        }
        if into_parts {
            self.entries.push(format!(
                "#[must_use] pub fn into_parts(self) -> ({})",
                types
                    .iter()
                    .map(AsRef::as_ref)
                    .collect::<Vec<&str>>()
                    .join(", ")
            ));
        }
        fields.named.iter().for_each(|field| {
            let Some(identifier) = field.ident.as_ref() else {
                return;
            };
            let wrapped_field_type = self.field_type(field);
            let field_type = wrapped_field_type.as_ref();
            field
                .attrs
                .iter()
                .filter(|field_attribute| {
                    field_attribute.path().is_ident(constants_str::test_fixtures::VALUE_21E85007)
                })
                .for_each(|field_attribute| {
                    field_attribute
                        .parse_nested_meta(|metadata| {
                            let signature = if metadata.path.is_ident(constants_str::test_fixtures::VALUE_D106CCB1) {
                                format!(
                                    "#[must_use] pub const fn {identifier}(&self) -> &{field_type}"
                                )
                            } else if metadata.path.is_ident(constants_str::test_fixtures::VALUE_6F5A6034) {
                                format!(
                                    "#[must_use] pub const fn {identifier}(self) -> {field_type}"
                                )
                            } else if metadata.path.is_ident(constants_str::test_fixtures::VALUE_8972F0EE) {
                                format!(
                                    "#[must_use] pub const fn {identifier}(&self) -> {field_type}"
                                )
                            } else if metadata.path.is_ident(constants_str::test_fixtures::VALUE_6B847A0E) {
                                format!(
                                    "#[must_use] pub fn into_{identifier}(self) -> {field_type}"
                                )
                            } else if metadata.path.is_ident(constants_str::test_fixtures::VALUE_ECA7C4E3) {
                                let inner_type = field_type
                                    .strip_prefix(constants_str::test_fixtures::VALUE_7E0FC0D7)
                                    .and_then(|value| value.strip_suffix('>'))
                                    .expect("9ba9415c into_ invariant must hold");
                                format!(
                                    "#[must_use] pub const fn {identifier}(&self) -> Option<&{inner_type}>"
                                )
                            } else if metadata.path.is_ident(constants_str::test_fixtures::VALUE_03FDB065) {
                                let parsed_element_type =
                                    metadata.value()?.parse::<syn::Type>()?;
                                let wrapped_element_type = self
                                    .source(syn::spanned::Spanned::span(&parsed_element_type));
                                let element_type = wrapped_element_type.as_ref();
                                format!(
                                    "#[must_use] pub const fn {identifier}(&self) -> &[{element_type}]"
                                )
                            } else {
                                String::new()
                            };
                            if !signature.is_empty() {
                                self.entries.push(signature);
                            }
                            Ok(())
                        })
                        .expect("206adbf7 into_ invariant must hold");
                });
        });
    }
}
impl<'ast> syn::visit::Visit<'ast> for PublicApiVisitor {
    fn visit_impl_item_fn(&mut self, i: &'ast syn::ImplItemFn) {
        if matches!(i.vis, syn::Visibility::Public(_)) {
            self.record(syn::spanned::Spanned::span(i), true);
        }
        syn::visit::visit_impl_item_fn(self, i);
    }
    fn visit_item_const(&mut self, i: &'ast syn::ItemConst) {
        if matches!(i.vis, syn::Visibility::Public(_)) {
            self.record(syn::spanned::Spanned::span(i), true);
        }
        syn::visit::visit_item_const(self, i);
    }
    fn visit_item_enum(&mut self, i: &'ast syn::ItemEnum) {
        if matches!(i.vis, syn::Visibility::Public(_)) {
            self.record(syn::spanned::Spanned::span(i), false);
            if i.attrs.iter().any(|attribute| {
                crate::code_style::derive_attr_has_terminal(
                    crate::types::SynAttributeRef::from(attribute),
                    crate::types::SourceTextRef::from(constants_str::test_fixtures::VALUE_4529EB51),
                )
                .get()
            }) {
                self.entries.push(format!(
                    "pub const COUNT: usize = {}usize;",
                    i.variants.len()
                ));
                self.entries
                    .push(String::from(constants_str::test_fixtures::VALUE_5F528A82));
            }
        }
        syn::visit::visit_item_enum(self, i);
    }
    fn visit_item_fn(&mut self, i: &'ast syn::ItemFn) {
        if matches!(i.vis, syn::Visibility::Public(_)) {
            self.record(syn::spanned::Spanned::span(i), true);
        }
        syn::visit::visit_item_fn(self, i);
    }
    fn visit_item_static(&mut self, i: &'ast syn::ItemStatic) {
        if matches!(i.vis, syn::Visibility::Public(_)) {
            self.record(syn::spanned::Spanned::span(i), true);
        }
        syn::visit::visit_item_static(self, i);
    }
    fn visit_item_struct(&mut self, i: &'ast syn::ItemStruct) {
        if matches!(i.vis, syn::Visibility::Public(_)) {
            self.record(syn::spanned::Spanned::span(i), false);
            self.record_contract_struct_api(i);
        }
        syn::visit::visit_item_struct(self, i);
    }
    fn visit_item_trait(&mut self, i: &'ast syn::ItemTrait) {
        if matches!(i.vis, syn::Visibility::Public(_)) {
            self.record(syn::spanned::Spanned::span(i), false);
        }
        syn::visit::visit_item_trait(self, i);
    }
}

#[derive(optimal_memory_layout::OptimalMemoryLayout, Default)]
struct StructErrorVisitor {
    identifiers: crate::types::SourceTextList,
}
impl<'ast> syn::visit::Visit<'ast> for StructErrorVisitor {
    fn visit_item_struct(&mut self, i: &'ast syn::ItemStruct) {
        if i.attrs.iter().any(|attribute| {
            crate::code_style::derive_attr_has_terminal(
                crate::types::SynAttributeRef::from(attribute),
                crate::types::SourceTextRef::from(constants_str::catalog::ERROR),
            )
            .get()
        }) {
            self.identifiers.push(i.ident.to_string());
        }
        syn::visit::visit_item_struct(self, i);
    }
}

#[derive(optimal_memory_layout::OptimalMemoryLayout, Default)]
struct LoopAllocationVisitor {
    depth: crate::types::AnalyzerCount,
    entries: crate::types::DiagnosticMsgs,
}
impl LoopAllocationVisitor {
    fn record(&mut self, operation: crate::types::SourceTextRef<'_>) {
        if self.depth.get() != constants_usize::ZERO {
            self.entries.push(operation.as_ref().to_owned());
        }
    }
}
impl<'ast> syn::visit::Visit<'ast> for LoopAllocationVisitor {
    fn visit_expr_call(&mut self, i: &'ast syn::ExprCall) {
        if let syn::Expr::Path(function) = i.func.as_ref() {
            let path =
                crate::code_style::path_to_string(crate::types::SynPathRef::from(&function.path));
            if [
                constants_str::test_fixtures::VALUE_EDB966EE,
                constants_str::test_fixtures::VALUE_13D4D62E,
                constants_str::test_fixtures::VALUE_6D0C4109,
                constants_str::test_fixtures::VALUE_7879C268,
                constants_str::test_fixtures::VALUE_FA4D593C,
                constants_str::test_fixtures::VALUE_F36B8CD3,
                constants_str::test_fixtures::VALUE_AA9C75B0,
                constants_str::test_fixtures::VALUE_B07BDC6E,
                constants_str::test_fixtures::VALUE_16359A6F,
                constants_str::test_fixtures::VALUE_C0ED6D49,
                constants_str::test_fixtures::VALUE_58AFC68F,
                constants_str::test_fixtures::VALUE_568F63F0,
            ]
            .contains(&path.as_ref())
            {
                self.record(crate::types::SourceTextRef::from(path.as_ref()));
            }
        }
        syn::visit::visit_expr_call(self, i);
    }
    fn visit_expr_loop(&mut self, i: &'ast syn::ExprLoop) {
        self.depth.saturating_inc();
        syn::visit::visit_expr_loop(self, i);
        self.depth.saturating_dec();
    }
    fn visit_expr_method_call(&mut self, i: &'ast syn::ExprMethodCall) {
        if matches!(
            i.method.to_string().as_str(),
            constants_str::test_fixtures::VALUE_B5D61DC8
                | constants_str::test_fixtures::VALUE_81824C90
                | constants_str::test_fixtures::VALUE_E132B7C0
                | constants_str::test_fixtures::VALUE_C5E9F49A
        ) {
            self.record(crate::types::SourceTextRef::from(
                i.method.to_string().as_str(),
            ));
        }
        syn::visit::visit_expr_method_call(self, i);
    }
    fn visit_expr_while(&mut self, i: &'ast syn::ExprWhile) {
        self.depth.saturating_inc();
        syn::visit::visit_expr_while(self, i);
        self.depth.saturating_dec();
    }
    fn visit_macro(&mut self, i: &'ast syn::Macro) {
        if let Some(segment) = i.path.segments.last()
            && matches!(
                segment.ident.to_string().as_str(),
                constants_str::catalog::SHARED_VALUES_FORMAT
                    | constants_str::test_fixtures::VALUE_38A4FDFC
            )
        {
            let operation = segment.ident.to_string();
            self.record(crate::types::SourceTextRef::from(operation.as_str()));
        }
        syn::visit::visit_macro(self, i);
    }
}

#[test]
fn lock_guards_are_not_held_across_await() {
    super::code_style_snapshot::with_codebase_snapshot(|snapshot| {
        let violations = snapshot
            .rs_files()
            .iter()
            .flat_map(|source_file| {
                let visitor = crate::code_style::visit_syn_file(
                    crate::types::SynFileRef::from(source_file.ast().as_ref()),
                    LockAcrossAwaitVisitor::default(),
                );
                visitor.violations.into_iter().map(|violation| {
                    format!("{}: {violation}", source_file.path().as_ref().display())
                })
            })
            .collect::<Vec<String>>();
        assert!(violations.is_empty(), "ce73f4a1 {violations:#?}");
    });
}

#[test]
fn allocations_inside_loops_match_reviewed_inventory() {
    let reviewed = std::collections::BTreeMap::from([
        (
            constants_str::test_fixtures::VALUE_B7558033,
            (
                constants_usize::ONE,
                constants_str::test_fixtures::VALUE_F3EA9A31,
            ),
        ),
        (
            constants_str::test_fixtures::VALUE_BC495D5D,
            (
                constants_usize::ONE,
                constants_str::test_fixtures::VALUE_BAC5F80E,
            ),
        ),
        (
            constants_str::test_fixtures::VALUE_E841E205,
            (
                constants_usize::ONE,
                constants_str::test_fixtures::VALUE_870DAE5B,
            ),
        ),
        (
            constants_str::test_fixtures::VALUE_BE04A453,
            (
                constants_usize::ONE,
                constants_str::test_fixtures::VALUE_D1CA6996,
            ),
        ),
        (
            constants_str::test_fixtures::VALUE_94FCEDB7,
            (
                constants_usize::ONE,
                constants_str::test_fixtures::VALUE_855EA4C0,
            ),
        ),
        (
            constants_str::test_fixtures::VALUE_63BD6017,
            (
                constants_usize::ONE,
                constants_str::test_fixtures::VALUE_A2531714,
            ),
        ),
    ]);
    super::code_style_snapshot::with_codebase_snapshot(|snapshot| {
        let observed = snapshot
            .rs_files()
            .iter()
            .filter(|source_file| {
                !crate::code_style::is_test_source_path(crate::types::PathRef::from(
                    std::borrow::Borrow::<std::path::Path>::borrow(source_file.path()),
                ))
                .get()
            })
            .flat_map(|source_file| {
                let visitor = crate::code_style::visit_syn_file(
                    crate::types::SynFileRef::from(source_file.ast().as_ref()),
                    LoopAllocationVisitor::default(),
                );
                let path = source_file.path().as_ref().display().to_string();
                visitor
                    .entries
                    .into_iter()
                    .map(move |entry| format!("{path}:{entry}"))
            })
            .fold(
                std::collections::BTreeMap::<String, usize>::new(),
                |mut counts, entry| {
                    let _count = counts
                        .entry(entry)
                        .and_modify(|count| *count = count.saturating_add(constants_usize::ONE))
                        .or_insert(constants_usize::ONE);
                    counts
                },
            );
        let expected = reviewed
            .iter()
            .map(|(entry, (count, reason))| {
                assert!(!reason.is_empty(), "418fe0af");
                ((*entry).to_owned(), *count)
            })
            .collect::<std::collections::BTreeMap<String, usize>>();
        assert_eq!(
            observed, expected,
            "418fe0af loop allocation inventory changed"
        );
    });
}

#[test]
fn struct_error_exceptions_match_reviewed_snapshot() {
    super::code_style_snapshot::with_codebase_snapshot(|snapshot| {
        let mut entries = snapshot
            .rs_files()
            .iter()
            .flat_map(|source_file| {
                let visitor = crate::code_style::visit_syn_file(
                    crate::types::SynFileRef::from(source_file.ast().as_ref()),
                    StructErrorVisitor::default(),
                );
                let path = source_file.path().as_ref().display().to_string();
                visitor
                    .identifiers
                    .into_iter()
                    .map(move |identifier| format!("{path}:{identifier}"))
            })
            .collect::<Vec<String>>();
        entries.sort();
        let mut current_snapshot = String::from(constants_str::test_fixtures::VALUE_C746CC87);
        entries.into_iter().for_each(|entry| {
            current_snapshot.push_str(entry.as_str());
            current_snapshot.push('\n');
        });
        let snapshot_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join(constants_str::test_fixtures::STRUCT_ERROR_SNAPSHOT_PATH);
        if std::env::var_os(constants_str::test_fixtures::UPDATE_CODE_STYLE_SNAPSHOTS).is_some() {
            std::fs::write(snapshot_path.as_path(), current_snapshot.as_bytes()).expect(
                "65e1d4f0 struct_error_exceptions_match_reviewed_snapshot invariant must hold",
            );
        }
        let expected_snapshot = std::fs::read_to_string(snapshot_path)
            .expect("ba047d32 struct_error_exceptions_match_reviewed_snapshot invariant must hold");
        assert_eq!(
            current_snapshot, expected_snapshot,
            "731ffc35 struct error inventory changed"
        );
    });
}

#[test]
fn contract_public_api_matches_reviewed_snapshot() {
    let reviewed = [
        (
            constants_str::test_fixtures::VALUE_A766D43E,
            constants_str::test_fixtures::VALUE_FE05288C,
        ),
        (
            constants_str::test_fixtures::VALUE_C34A5FE6,
            constants_str::test_fixtures::VALUE_8733430F,
        ),
        (
            constants_str::test_fixtures::VALUE_0C5CC511,
            constants_str::test_fixtures::VALUE_CF3D8D33,
        ),
    ];
    super::code_style_snapshot::with_codebase_snapshot(|snapshot| {
        let mut current_snapshot = String::from(constants_str::test_fixtures::VALUE_1F3A1C37);
        reviewed.iter().for_each(|(directory_suffix, reason)| {
            assert!(!reason.is_empty(), "505a0cf7");
            let mut entries = snapshot
                .rs_files()
                .iter()
                .filter(|source_file| {
                    source_file
                        .path()
                        .as_ref()
                        .to_string_lossy()
                        .contains(directory_suffix)
                })
                .flat_map(|source_file| {
                    let visitor = crate::code_style::visit_syn_file(
                        crate::types::SynFileRef::from(source_file.ast().as_ref()),
                        PublicApiVisitor {
                            entries: crate::types::SourceTextList::default(),
                            lines: crate::types::SourceTextList::from(
                                source_file
                                    .content()
                                    .as_ref()
                                    .lines()
                                    .map(str::to_owned)
                                    .collect::<Vec<String>>(),
                            ),
                        },
                    );
                    visitor.entries
                })
                .collect::<Vec<String>>();
            entries.sort();
            current_snapshot.push_str(format!("\n[{directory_suffix}] # {reason}\n").as_str());
            entries.into_iter().for_each(|entry| {
                current_snapshot.push_str(entry.as_str());
                current_snapshot.push('\n');
            });
        });
        let snapshot_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join(constants_str::test_fixtures::CONTRACT_PUBLIC_API_SNAPSHOT_PATH);
        if std::env::var_os(constants_str::test_fixtures::UPDATE_CODE_STYLE_SNAPSHOTS).is_some()
            || std::env::var_os(constants_str::test_fixtures::UPDATE_CONTRACT_PUBLIC_API_SNAPSHOT)
                .is_some()
        {
            std::fs::write(snapshot_path.as_path(), current_snapshot.as_bytes()).expect(
                "e2c6b190 contract_public_api_matches_reviewed_snapshot invariant must hold",
            );
        }
        let expected_snapshot = std::fs::read_to_string(snapshot_path)
            .expect("fd9130e7 contract_public_api_matches_reviewed_snapshot invariant must hold");
        assert_eq!(
            current_snapshot, expected_snapshot,
            "505a0cf7 contract public API snapshot changed"
        );
    });
}

#[test]
fn arc_lock_and_trait_object_usage_matches_reviewed_inventory() {
    let reviewed = std::collections::BTreeMap::from([
        (
            constants_str::test_fixtures::CODE_STYLE_COMMON_ROUTES_OWNER,
            (
                3,
                0,
                2,
                constants_str::test_fixtures::CODE_STYLE_SHARED_DISPATCH_OWNER_REASON,
            ),
        ),
        (
            constants_str::test_fixtures::CODE_STYLE_MACRO_HELPERS_OWNER,
            (
                0,
                0,
                83,
                constants_str::test_fixtures::CODE_STYLE_SHARED_DISPATCH_OWNER_REASON,
            ),
        ),
        (
            constants_str::test_fixtures::CODE_STYLE_NOTIFICATION_SERVICE_OWNER,
            (
                2,
                0,
                1,
                constants_str::test_fixtures::CODE_STYLE_SHARED_DISPATCH_OWNER_REASON,
            ),
        ),
        (
            constants_str::test_fixtures::CODE_STYLE_PG_CRUD_COMMON_OWNER,
            (
                0,
                0,
                12,
                constants_str::test_fixtures::CODE_STYLE_SHARED_DISPATCH_OWNER_REASON,
            ),
        ),
        (
            constants_str::test_fixtures::CODE_STYLE_PG_CRUD_MACRO_COMMON_OWNER,
            (
                0,
                0,
                140,
                constants_str::test_fixtures::CODE_STYLE_SHARED_DISPATCH_OWNER_REASON,
            ),
        ),
        (
            constants_str::test_fixtures::CODE_STYLE_PG_CRUD_PG_TYPES_COMMON_OWNER,
            (
                0,
                0,
                1,
                constants_str::test_fixtures::CODE_STYLE_SHARED_DISPATCH_OWNER_REASON,
            ),
        ),
        (
            constants_str::test_fixtures::CODE_STYLE_PG_CRUD_WHERE_FILTERS_OWNER,
            (
                0,
                0,
                4,
                constants_str::test_fixtures::CODE_STYLE_SHARED_DISPATCH_OWNER_REASON,
            ),
        ),
        (
            constants_str::test_fixtures::CODE_STYLE_PG_CRUD_WHERE_FILTERS_GENERATE_SRC_OWNER,
            (
                0,
                0,
                31,
                constants_str::test_fixtures::CODE_STYLE_SHARED_DISPATCH_OWNER_REASON,
            ),
        ),
        (
            constants_str::test_fixtures::CODE_STYLE_SERVER_OWNER,
            (
                3,
                0,
                1,
                constants_str::test_fixtures::CODE_STYLE_SHARED_DISPATCH_OWNER_REASON,
            ),
        ),
        (
            constants_str::test_fixtures::CODE_STYLE_SERVER_ADMIN_OWNER,
            (
                3,
                0,
                1,
                constants_str::test_fixtures::CODE_STYLE_SHARED_DISPATCH_OWNER_REASON,
            ),
        ),
        (
            constants_str::test_fixtures::CODE_STYLE_SERVER_RUNTIME_CORE_OWNER,
            (
                4,
                3,
                0,
                constants_str::test_fixtures::CODE_STYLE_SHARED_DISPATCH_OWNER_REASON,
            ),
        ),
        (
            constants_str::test_fixtures::CODE_STYLE_SERVER_RUNTIME_HTTP_OWNER,
            (
                1,
                1,
                4,
                constants_str::test_fixtures::CODE_STYLE_SHARED_DISPATCH_OWNER_REASON,
            ),
        ),
        (
            constants_str::test_fixtures::VALUE_20A65589,
            (
                constants_usize::ZERO,
                constants_usize::ZERO,
                3usize,
                constants_str::test_fixtures::VALUE_F75AB320,
            ),
        ),
        (
            constants_str::test_fixtures::VALUE_823EE954,
            (0, 0, 4, constants_str::test_fixtures::VALUE_E98F8E33),
        ),
        (
            constants_str::test_fixtures::VALUE_D11679FC,
            (0, 0, 2, constants_str::test_fixtures::VALUE_E98F8E33),
        ),
        (
            constants_str::test_fixtures::VALUE_794839A7,
            (0, 0, 2, constants_str::test_fixtures::VALUE_E98F8E33),
        ),
        (
            constants_str::test_fixtures::VALUE_642AA8AC,
            (0, 0, 4, constants_str::test_fixtures::VALUE_E98F8E33),
        ),
        (
            constants_str::test_fixtures::VALUE_31BDEFD7,
            (0, 0, 2, constants_str::test_fixtures::VALUE_E98F8E33),
        ),
        (
            constants_str::test_fixtures::VALUE_95F11308,
            (0, 0, 3, constants_str::test_fixtures::VALUE_E98F8E33),
        ),
        (
            constants_str::test_fixtures::VALUE_BDEB5C57,
            (0, 0, 1, constants_str::test_fixtures::VALUE_40349028),
        ),
        (
            constants_str::test_fixtures::VALUE_8F0CF86A,
            (0, 0, 4, constants_str::test_fixtures::VALUE_E98F8E33),
        ),
        (
            constants_str::test_fixtures::VALUE_427B03A1,
            (0, 0, 5, constants_str::test_fixtures::VALUE_D4BDC80F),
        ),
        (
            constants_str::test_fixtures::VALUE_30B1AC8C,
            (0, 0, 1, constants_str::test_fixtures::VALUE_DC7573CC),
        ),
        (
            constants_str::test_fixtures::VALUE_8CD81F6A,
            (0, 0, 10, constants_str::test_fixtures::VALUE_AC5E426F),
        ),
        (
            constants_str::test_fixtures::VALUE_9C3011F7,
            (1, 0, 0, constants_str::test_fixtures::VALUE_60EE0A5C),
        ),
        (
            constants_str::test_fixtures::VALUE_315FCF0A,
            (1, 0, 0, constants_str::test_fixtures::VALUE_60EE0A5C),
        ),
        (
            constants_str::test_fixtures::VALUE_778833C1,
            (1, 0, 0, constants_str::test_fixtures::VALUE_60EE0A5C),
        ),
        (
            constants_str::test_fixtures::VALUE_EAC3A6DC,
            (0, 0, 3, constants_str::test_fixtures::VALUE_0CD339A2),
        ),
        (
            constants_str::test_fixtures::VALUE_7FE2AF02,
            (0, 0, 179, constants_str::test_fixtures::VALUE_7FA1ACFA),
        ),
        (
            constants_str::test_fixtures::VALUE_D405F3E1,
            (0, 0, 177, constants_str::test_fixtures::VALUE_FB2CE6C2),
        ),
        (
            constants_str::test_fixtures::VALUE_9D0FC67D,
            (0, 0, 1, constants_str::test_fixtures::VALUE_E7909B41),
        ),
        (
            constants_str::test_fixtures::VALUE_206B48D7,
            (0, 0, 1, constants_str::test_fixtures::VALUE_F5E028C2),
        ),
    ]);
    super::code_style_snapshot::with_codebase_snapshot(|snapshot| {
        let mut observed_by_owner = std::collections::BTreeMap::new();
        let mut violations = Vec::new();
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
                    SharedDispatchVisitor::default(),
                );
                let observed = (
                    visitor.arc_types.get(),
                    visitor.lock_types.get(),
                    visitor.trait_objects.get(),
                );
                if observed
                    == (
                        constants_usize::ZERO,
                        constants_usize::ZERO,
                        constants_usize::ZERO,
                    )
                {
                    return;
                }
                let path = source_file.path().as_ref().display().to_string();
                let reviewed_entry = reviewed
                    .iter()
                    .filter(|(suffix, (_arc, _lock, _traits, reason))| {
                        (path.ends_with(**suffix)
                            || crate::code_style::declared_child_matches(path.as_str(), suffix))
                            && !reason.is_empty()
                    })
                    .max_by_key(|(suffix, _expected)| suffix.len());
                match reviewed_entry {
                    Some((suffix, _expected)) => {
                        let entry = observed_by_owner.entry(*suffix).or_insert((0, 0, 0));
                        entry.0 += observed.0;
                        entry.1 += observed.1;
                        entry.2 += observed.2;
                    }
                    None => violations.push(format!(
                        "{path}: unreviewed Arc/lock/trait-object usage {observed:?}"
                    )),
                }
            });
        reviewed.iter().for_each(|(suffix, expected)| {
            let observed = observed_by_owner.get(suffix).copied().unwrap_or((0, 0, 0));
            if observed != (expected.0, expected.1, expected.2) {
                violations.push(format!(
                    "shared-dispatch inventory changed for {suffix}: expected={:?}, observed={observed:?}",
                    (expected.0, expected.1, expected.2)
                ));
            }
        });
        assert!(violations.is_empty(), "66b91e7a {violations:#?}");
    });
}

#[test]
fn ignored_map_err_bindings_match_reviewed_inventory() {
    let reviewed = std::collections::BTreeMap::from([
        (
            constants_str::test_fixtures::CODE_STYLE_COMMON_ROUTES_OWNER,
            (
                1usize,
                constants_str::test_fixtures::CODE_STYLE_MAP_ERR_OWNER_REASON,
            ),
        ),
        (
            constants_str::test_fixtures::CODE_STYLE_CONFIG_LIB_OWNER,
            (
                4usize,
                constants_str::test_fixtures::CODE_STYLE_MAP_ERR_OWNER_REASON,
            ),
        ),
        (
            constants_str::test_fixtures::CODE_STYLE_EXTERNAL_SERVICE_EMULATORS_OWNER,
            (
                1usize,
                constants_str::test_fixtures::CODE_STYLE_MAP_ERR_OWNER_REASON,
            ),
        ),
        (
            constants_str::test_fixtures::CODE_STYLE_FILE_STORAGE_OWNER,
            (
                1usize,
                constants_str::test_fixtures::CODE_STYLE_MAP_ERR_OWNER_REASON,
            ),
        ),
        (
            constants_str::test_fixtures::CODE_STYLE_FRONTEND_CONTRACT_VALIDATION_OWNER,
            (
                2usize,
                constants_str::test_fixtures::CODE_STYLE_MAP_ERR_OWNER_REASON,
            ),
        ),
        (
            constants_str::test_fixtures::CODE_STYLE_MACRO_HELPERS_OWNER,
            (
                3usize,
                constants_str::test_fixtures::CODE_STYLE_MAP_ERR_OWNER_REASON,
            ),
        ),
        (
            constants_str::test_fixtures::CODE_STYLE_NOTIFICATION_SERVICE_OWNER,
            (
                2usize,
                constants_str::test_fixtures::CODE_STYLE_MAP_ERR_OWNER_REASON,
            ),
        ),
        (
            constants_str::test_fixtures::CODE_STYLE_PG_CRUD_COMMON_OWNER,
            (
                1usize,
                constants_str::test_fixtures::CODE_STYLE_MAP_ERR_OWNER_REASON,
            ),
        ),
        (
            constants_str::test_fixtures::CODE_STYLE_PG_CRUD_WHERE_FILTERS_OWNER,
            (
                1usize,
                constants_str::test_fixtures::CODE_STYLE_MAP_ERR_OWNER_REASON,
            ),
        ),
        (
            constants_str::test_fixtures::CODE_STYLE_SERVER_ADMIN_OWNER,
            (
                110usize,
                constants_str::test_fixtures::CODE_STYLE_MAP_ERR_OWNER_REASON,
            ),
        ),
        (
            constants_str::test_fixtures::CODE_STYLE_SERVER_ADMIN_CONTRACT_OWNER,
            (
                1usize,
                constants_str::test_fixtures::CODE_STYLE_MAP_ERR_OWNER_REASON,
            ),
        ),
        (
            constants_str::test_fixtures::CODE_STYLE_SERVER_ADMIN_CORE_OWNER,
            (
                4usize,
                constants_str::test_fixtures::CODE_STYLE_MAP_ERR_OWNER_REASON,
            ),
        ),
        (
            constants_str::test_fixtures::CODE_STYLE_SERVER_ADMIN_FRONTEND_OWNER,
            (
                11usize,
                constants_str::test_fixtures::CODE_STYLE_MAP_ERR_OWNER_REASON,
            ),
        ),
        (
            constants_str::test_fixtures::CODE_STYLE_SERVER_RUNTIME_HTTP_OWNER,
            (
                11usize,
                constants_str::test_fixtures::CODE_STYLE_MAP_ERR_OWNER_REASON,
            ),
        ),
        (
            constants_str::test_fixtures::CODE_STYLE_WORKSPACE_SCAFFOLD_OWNER,
            (
                16usize,
                constants_str::test_fixtures::CODE_STYLE_MAP_ERR_OWNER_REASON,
            ),
        ),
        (
            constants_str::test_fixtures::CODE_STYLE_WORKSPACE_TEST_RUNNER_OWNER,
            (
                1usize,
                constants_str::test_fixtures::CODE_STYLE_MAP_ERR_OWNER_REASON,
            ),
        ),
        (
            constants_str::test_fixtures::VALUE_AC7A6F68,
            (2usize, constants_str::test_fixtures::VALUE_3995FF01),
        ),
        (
            constants_str::test_fixtures::VALUE_769125D7,
            (
                constants_usize::ONE,
                constants_str::test_fixtures::VALUE_EB67E2C6,
            ),
        ),
        (
            constants_str::test_fixtures::VALUE_46FA1B05,
            (2usize, constants_str::test_fixtures::VALUE_C98C08E2),
        ),
        (
            constants_str::test_fixtures::VALUE_96B90C9B,
            (
                constants_usize::ONE,
                constants_str::test_fixtures::VALUE_9111728C,
            ),
        ),
        (
            constants_str::test_fixtures::VALUE_E5C6D18E,
            (8usize, constants_str::test_fixtures::VALUE_9111728C),
        ),
        (
            constants_str::test_fixtures::VALUE_11F5A276,
            (2usize, constants_str::test_fixtures::VALUE_C1819A84),
        ),
        (
            constants_str::test_fixtures::VALUE_2CBAA4F4,
            (
                constants_usize::ONE,
                constants_str::test_fixtures::VALUE_53588272,
            ),
        ),
        (
            constants_str::test_fixtures::VALUE_D191EE7F,
            (2usize, constants_str::test_fixtures::VALUE_53588272),
        ),
        (
            constants_str::test_fixtures::VALUE_B29D07A8,
            (
                constants_usize::ONE,
                constants_str::test_fixtures::VALUE_099B4392,
            ),
        ),
        (
            constants_str::test_fixtures::VALUE_A1750307,
            (
                constants_usize::ONE,
                constants_str::test_fixtures::VALUE_099B4392,
            ),
        ),
        (
            constants_str::test_fixtures::VALUE_3930BC5E,
            (
                constants_usize::ONE,
                constants_str::test_fixtures::VALUE_7B6389D8,
            ),
        ),
        (
            constants_str::test_fixtures::VALUE_E24F0FD4,
            (
                constants_usize::ONE,
                constants_str::test_fixtures::VALUE_01371493,
            ),
        ),
        (
            constants_str::test_fixtures::VALUE_4C6F4532,
            (8usize, constants_str::test_fixtures::VALUE_80247FE1),
        ),
        (
            constants_str::test_fixtures::VALUE_DC454021,
            (4usize, constants_str::test_fixtures::VALUE_FD41C49E),
        ),
        (
            constants_str::test_fixtures::VALUE_DC37304F,
            (
                constants_usize::ONE,
                constants_str::test_fixtures::VALUE_FD41C49E,
            ),
        ),
        (
            constants_str::test_fixtures::VALUE_939FFBC6,
            (6usize, constants_str::test_fixtures::VALUE_FD41C49E),
        ),
        (
            constants_str::test_fixtures::VALUE_F3169686,
            (5usize, constants_str::test_fixtures::VALUE_FAE4D1C8),
        ),
        (
            constants_str::test_fixtures::VALUE_27AB06E9,
            (2usize, constants_str::test_fixtures::VALUE_B1E73CDD),
        ),
        (
            constants_str::test_fixtures::VALUE_9E7DB142,
            (10usize, constants_str::test_fixtures::VALUE_0B70A676),
        ),
        (
            constants_str::test_fixtures::VALUE_BEBEC57E,
            (
                constants_usize::ONE,
                constants_str::test_fixtures::VALUE_9CA4EAEB,
            ),
        ),
    ]);
    super::code_style_snapshot::with_codebase_snapshot(|snapshot| {
        let mut observed_by_owner = std::collections::BTreeMap::new();
        let mut violations = Vec::new();
        snapshot.rs_files().iter().for_each(|source_file| {
            let visitor = crate::code_style::visit_syn_file(
                crate::types::SynFileRef::from(source_file.ast().as_ref()),
                IgnoredMapErrBindingVisitor::default(),
            );
            if visitor.entries.is_empty() {
                return;
            }
            let path = source_file.path().as_ref().display().to_string();
            let reviewed_entry = reviewed
                .iter()
                .filter(|(suffix, (_count, reason))| {
                    (path.ends_with(**suffix)
                        || crate::code_style::declared_child_matches(path.as_str(), suffix))
                        && !reason.is_empty()
                })
                .max_by_key(|(suffix, _expected)| suffix.len());
            match reviewed_entry {
                Some((suffix, _expected)) => {
                    *observed_by_owner.entry(*suffix).or_insert(0usize) += visitor.entries.len();
                }
                None => violations.push(format!(
                    "{path}: unreviewed ignored map_err bindings: count={}",
                    visitor.entries.len()
                )),
            }
        });
        reviewed.iter().for_each(|(suffix, (expected, _reason))| {
            let observed = observed_by_owner.get(suffix).copied().unwrap_or(0usize);
            if observed != *expected {
                violations.push(format!(
                    "ignored map_err inventory changed for {suffix}: expected count={expected}; observed count={observed}"
                ));
            }
        });
        assert!(violations.is_empty(), "bb0dbc1f {violations:#?}");
    });
}

#[test]
fn raw_vec_tuple_wrappers_match_reviewed_inventory() {
    let reviewed = std::collections::BTreeMap::from([
        (
            constants_str::test_fixtures::VALUE_7630EBEC,
            constants_str::test_fixtures::VALUE_C2C65C68,
        ),
        (
            constants_str::test_fixtures::VALUE_86D03626,
            constants_str::test_fixtures::VALUE_E3D9A7E6,
        ),
        (
            constants_str::test_fixtures::VALUE_6C761A40,
            constants_str::test_fixtures::VALUE_BC91BCEF,
        ),
        (
            constants_str::test_fixtures::VALUE_F68E036F,
            constants_str::test_fixtures::VALUE_D17C5423,
        ),
        (
            constants_str::test_fixtures::VALUE_090096ED,
            constants_str::test_fixtures::VALUE_07FFA47C,
        ),
        (
            constants_str::test_fixtures::VALUE_94E2B4FA,
            constants_str::test_fixtures::VALUE_63229E70,
        ),
        (
            constants_str::test_fixtures::VALUE_D9B93146,
            constants_str::test_fixtures::VALUE_FC3332AB,
        ),
        (
            constants_str::test_fixtures::VALUE_6F5D2E20,
            constants_str::test_fixtures::VALUE_0901EA34,
        ),
        (
            constants_str::test_fixtures::VALUE_9DFC7A97,
            constants_str::test_fixtures::VALUE_FDB078C8,
        ),
        (
            constants_str::test_fixtures::VALUE_0525E2BF,
            constants_str::test_fixtures::VALUE_FBBB4FDC,
        ),
        (
            constants_str::test_fixtures::VALUE_CAE88716,
            constants_str::test_fixtures::VALUE_FBBB4FDC,
        ),
        (
            constants_str::test_fixtures::VALUE_D51ADF29,
            constants_str::test_fixtures::VALUE_16D85132,
        ),
        (
            constants_str::test_fixtures::VALUE_975B0C21,
            constants_str::test_fixtures::VALUE_FBBB4FDC,
        ),
        (
            constants_str::test_fixtures::VALUE_AA7EE094,
            constants_str::test_fixtures::VALUE_16D85132,
        ),
        (
            constants_str::test_fixtures::VALUE_5879251A,
            constants_str::test_fixtures::VALUE_FBBB4FDC,
        ),
        (
            constants_str::test_fixtures::VALUE_51CC135E,
            constants_str::test_fixtures::VALUE_16D85132,
        ),
        (
            constants_str::test_fixtures::VALUE_B1A7F284,
            constants_str::test_fixtures::VALUE_16D85132,
        ),
        (
            constants_str::test_fixtures::VALUE_8C2154B5,
            constants_str::test_fixtures::VALUE_55AE895C,
        ),
        (
            constants_str::test_fixtures::VALUE_7314D06D,
            constants_str::test_fixtures::VALUE_EF8D5AF4,
        ),
        (
            constants_str::test_fixtures::VALUE_9AE03CB2,
            constants_str::test_fixtures::VALUE_986BBD24,
        ),
        (
            constants_str::test_fixtures::VALUE_6BF051A2,
            constants_str::test_fixtures::VALUE_A5952628,
        ),
        (
            constants_str::test_fixtures::VALUE_C7F27415,
            constants_str::test_fixtures::VALUE_C3214518,
        ),
        (
            constants_str::test_fixtures::VALUE_A417488B,
            constants_str::test_fixtures::VALUE_D39882F4,
        ),
        (
            constants_str::test_fixtures::VALUE_919ACACB,
            constants_str::test_fixtures::VALUE_1FBF1A7A,
        ),
        (
            constants_str::test_fixtures::VALUE_9DB8F65B,
            constants_str::test_fixtures::VALUE_7C37CACC,
        ),
        (
            constants_str::test_fixtures::VALUE_671231A3,
            constants_str::test_fixtures::VALUE_D82FE516,
        ),
        (
            constants_str::test_fixtures::VALUE_DEB830DD,
            constants_str::test_fixtures::VALUE_DCAEE23B,
        ),
        (
            constants_str::test_fixtures::VALUE_DD337AC0,
            constants_str::test_fixtures::VALUE_C9221A63,
        ),
        (
            constants_str::test_fixtures::VALUE_06C235F4,
            constants_str::test_fixtures::VALUE_211A1405,
        ),
        (
            constants_str::test_fixtures::VALUE_2316F647,
            constants_str::test_fixtures::VALUE_0EFD8ED8,
        ),
        (
            constants_str::test_fixtures::VALUE_5D687FEA,
            constants_str::test_fixtures::VALUE_0C7973A9,
        ),
        (
            constants_str::test_fixtures::VALUE_7E7B2B37,
            constants_str::test_fixtures::VALUE_0F84F758,
        ),
        (
            constants_str::test_fixtures::VALUE_CB780650,
            constants_str::test_fixtures::VALUE_8FEB779E,
        ),
        (
            constants_str::test_fixtures::VALUE_1D2594F2,
            constants_str::test_fixtures::VALUE_5D972838,
        ),
        (
            constants_str::test_fixtures::VALUE_A48AAE67,
            constants_str::test_fixtures::VALUE_2DCAD87D,
        ),
        (
            constants_str::test_fixtures::VALUE_B9937202,
            constants_str::test_fixtures::VALUE_13C920C3,
        ),
        (
            constants_str::test_fixtures::VALUE_2941B657,
            constants_str::test_fixtures::VALUE_13C920C3,
        ),
        (
            constants_str::test_fixtures::VALUE_FAB1545F,
            constants_str::test_fixtures::VALUE_64EA6158,
        ),
        (
            constants_str::test_fixtures::VALUE_9FB992E8,
            constants_str::test_fixtures::VALUE_3F51E18F,
        ),
        (
            constants_str::test_fixtures::VALUE_D200D86F,
            constants_str::test_fixtures::VALUE_352F4313,
        ),
        (
            constants_str::test_fixtures::VALUE_413BDF99,
            constants_str::test_fixtures::VALUE_28A55761,
        ),
        (
            constants_str::test_fixtures::VALUE_EA3B0668,
            constants_str::test_fixtures::VALUE_82F6C375,
        ),
    ]);
    reviewed
        .values()
        .for_each(|reason| assert!(!reason.is_empty(), "f8c9471a"));
    super::code_style_snapshot::with_codebase_snapshot(|snapshot| {
        let observed = snapshot
            .rs_files()
            .iter()
            .filter(|source_file| {
                !crate::code_style::is_test_source_path(crate::types::PathRef::from(
                    std::borrow::Borrow::<std::path::Path>::borrow(source_file.path()),
                ))
                .get()
            })
            .flat_map(|source_file| {
                let visitor = crate::code_style::visit_syn_file(
                    crate::types::SynFileRef::from(source_file.ast().as_ref()),
                    RawVecTupleWrapperVisitor::default(),
                );
                let path = source_file.path().as_ref().display().to_string();
                visitor
                    .identifiers
                    .into_iter()
                    .map(move |identifier| format!("{path}:{identifier}"))
            })
            .collect::<std::collections::BTreeSet<String>>();
        let expected = reviewed
            .keys()
            .map(|entry| (*entry).to_owned())
            .collect::<std::collections::BTreeSet<String>>();
        assert_eq!(
            observed, expected,
            "f8c9471a raw Vec tuple wrapper inventory changed"
        );
    });
}

#[test]
fn from_vec_implementations_are_forbidden() {
    super::code_style_snapshot::with_codebase_snapshot(|snapshot| {
        let violations = snapshot
            .rs_files()
            .iter()
            .flat_map(|source_file| {
                let visitor = crate::code_style::visit_syn_file(
                    crate::types::SynFileRef::from(source_file.ast().as_ref()),
                    FromVecImplVisitor::default(),
                );
                let path = source_file.path().as_ref().display().to_string();
                visitor
                    .targets
                    .into_iter()
                    .map(move |target| format!("{path}:{target}"))
            })
            .collect::<Vec<String>>();
        assert!(violations.is_empty(), "6cc64ce8 {violations:#?}");
    });
}

#[test]
fn raw_vec_tuple_wrapper_visitor_detects_qualified_and_nested_types() {
    let file: syn::File = syn::parse_quote! {
        struct Qualified(std::vec::Vec<u8>);
        struct Named {
            values: Vec<u8>,
        }
        mod nested {
            struct Nested(Vec<u8>);
        }
    };
    let visitor = crate::code_style::visit_syn_file(
        crate::types::SynFileRef::from(&file),
        RawVecTupleWrapperVisitor::default(),
    );
    assert_eq!(visitor.identifiers.len(), 2usize);
}

#[test]
fn usize_max_usage_matches_reviewed_inventory() {
    let reviewed = std::collections::BTreeMap::from([
        (
            constants_str::test_fixtures::VALUE_08DBA674,
            (
                constants_usize::ONE,
                constants_str::test_fixtures::VALUE_CEE3C893,
            ),
        ),
        (
            constants_str::test_fixtures::VALUE_2483AEA6,
            (3usize, constants_str::test_fixtures::VALUE_245849CA),
        ),
        (
            constants_str::test_fixtures::VALUE_7FE2AF02,
            (3usize, constants_str::test_fixtures::VALUE_211A1405),
        ),
        (
            constants_str::test_fixtures::PROCESS_ARGUMENTS_PATH,
            (
                constants_usize::ONE,
                constants_str::test_fixtures::VALUE_28A0F9A4,
            ),
        ),
        (
            constants_str::test_fixtures::PROCESS_COMMANDS_PATH,
            (
                constants_usize::ONE,
                constants_str::test_fixtures::VALUE_28A0F9A4,
            ),
        ),
        (
            constants_str::test_fixtures::CHILD_DIAGNOSTIC_PATH,
            (
                constants_usize::ONE,
                constants_str::test_fixtures::DIAGNOSTIC_BUFFER_MAX_REASON,
            ),
        ),
        (
            constants_str::test_fixtures::STD_COLLECTIONS_CHILD_PROCESS_MAP_PATH,
            (
                constants_usize::ONE,
                constants_str::test_fixtures::RUNTIME_LIMITED_STORAGE_MAX_REASON,
            ),
        ),
        (
            constants_str::test_fixtures::CHILD_PROCESS_REPORTS_PATH,
            (
                constants_usize::ONE,
                constants_str::test_fixtures::DIAGNOSTIC_BUFFER_MAX_REASON,
            ),
        ),
        (
            constants_str::test_fixtures::LEASE_REGISTRY_INNER_PATH,
            (2usize, constants_str::test_fixtures::VALUE_E55D8523),
        ),
        (
            constants_str::test_fixtures::LEASE_IDS_PATH,
            (
                constants_usize::ONE,
                constants_str::test_fixtures::VALUE_E55D8523,
            ),
        ),
        (
            constants_str::test_fixtures::SINGLE_FLIGHT_INNER_PATH,
            (
                constants_usize::ONE,
                constants_str::test_fixtures::VALUE_845FE7CB,
            ),
        ),
        (
            constants_str::test_fixtures::DISK_CACHE_EVICTION_PLAN_PATH,
            (
                constants_usize::ONE,
                constants_str::test_fixtures::RUNTIME_LIMITED_STORAGE_MAX_REASON,
            ),
        ),
        (
            constants_str::test_fixtures::ENV_KEYS_PATH,
            (
                constants_usize::ONE,
                constants_str::test_fixtures::COMPILE_TIME_CATALOG_MAX_REASON,
            ),
        ),
        (
            constants_str::test_fixtures::INIT_ENTRIES_PATH,
            (
                constants_usize::ONE,
                constants_str::test_fixtures::COMPILE_TIME_CATALOG_MAX_REASON,
            ),
        ),
        (
            constants_str::test_fixtures::RUN_COMMANDS_PATH,
            (
                constants_usize::ONE,
                constants_str::test_fixtures::FAILURE_SENTINEL_MAX_REASON,
            ),
        ),
        (
            constants_str::test_fixtures::COMMAND_TEXTS_PATH,
            (
                constants_usize::ONE,
                constants_str::test_fixtures::VALUE_28A0F9A4,
            ),
        ),
        (
            constants_str::test_fixtures::ROUTE_CONTRACTS_PATH,
            (
                constants_usize::ONE,
                constants_str::test_fixtures::COMPILE_TIME_CATALOG_MAX_REASON,
            ),
        ),
        (
            constants_str::test_fixtures::ROUTE_COVERAGE_DESCRIPTORS_PATH,
            (
                constants_usize::ONE,
                constants_str::test_fixtures::COMPILE_TIME_CATALOG_MAX_REASON,
            ),
        ),
        (
            constants_str::test_fixtures::ACTION_CONTRACTS_PATH,
            (
                constants_usize::ONE,
                constants_str::test_fixtures::COMPILE_TIME_CATALOG_MAX_REASON,
            ),
        ),
        (
            constants_str::test_fixtures::ROUTE_METADATA_LIST_PATH,
            (
                constants_usize::ONE,
                constants_str::test_fixtures::COMPILE_TIME_CATALOG_MAX_REASON,
            ),
        ),
        (
            constants_str::test_fixtures::ROUTE_TEST_CATEGORIES_PATH,
            (
                constants_usize::ONE,
                constants_str::test_fixtures::COMPILE_TIME_CATALOG_MAX_REASON,
            ),
        ),
        (
            constants_str::test_fixtures::ROUTE_SCHEMA_CONTRACTS_PATH,
            (
                constants_usize::ONE,
                constants_str::test_fixtures::COMPILE_TIME_CATALOG_MAX_REASON,
            ),
        ),
        (
            constants_str::test_fixtures::FIELD_CONTRACTS_PATH,
            (
                constants_usize::ONE,
                constants_str::test_fixtures::COMPILE_TIME_CATALOG_MAX_REASON,
            ),
        ),
        (
            constants_str::test_fixtures::ROUTE_CONTRACT_MISMATCHES_PATH,
            (
                constants_usize::ONE,
                constants_str::test_fixtures::RUNTIME_LIMITED_STORAGE_MAX_REASON,
            ),
        ),
        (
            constants_str::test_fixtures::SERVICE_CATALOG_ENTRIES_PATH,
            (
                constants_usize::ONE,
                constants_str::test_fixtures::VALUE_0BF03626,
            ),
        ),
    ]);
    reviewed
        .values()
        .for_each(|(_count, reason)| assert!(!reason.is_empty(), "cfc5175f"));
    super::code_style_snapshot::with_codebase_snapshot(|snapshot| {
        let mut observed = std::collections::BTreeMap::<&str, usize>::new();
        let mut violations = Vec::new();
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
                    UsizeMaxExprVisitor::default(),
                );
                let count = visitor.count.get();
                if count == constants_usize::ZERO {
                    return;
                }
                let path = source_file.path().as_ref().display().to_string();
                let reviewed_owner = reviewed
                    .keys()
                    .filter(|suffix| {
                        path.ends_with(**suffix)
                            || crate::code_style::declared_child_matches(path.as_str(), suffix)
                    })
                    .max_by_key(|suffix| suffix.len());
                if let Some(owner) = reviewed_owner {
                    let _observed_count_entry = observed
                        .entry(owner)
                        .and_modify(|observed_count| {
                            *observed_count = observed_count.saturating_add(count);
                        })
                        .or_insert(count);
                } else {
                    violations.push(format!("unreviewed usize::MAX owner: {path}"));
                }
            });
        let expected = reviewed
            .iter()
            .map(|(path, (count, _reason))| (*path, *count))
            .collect::<std::collections::BTreeMap<&str, usize>>();
        assert_eq!(observed, expected, "cfc5175f usize::MAX inventory changed");
        assert!(violations.is_empty(), "cfc5175f {violations:#?}");
    });
}

#[test]
fn usize_max_expression_visitor_skips_test_modules() {
    let file: syn::File = syn::parse_quote! {
        const PRODUCTION_MAXIMUM: usize = usize::MAX;
        #[cfg(test)]
        mod tests {
            const TEST_MAXIMUM: usize = usize::MAX;
        }
    };
    let visitor = crate::code_style::visit_syn_file(
        crate::types::SynFileRef::from(&file),
        UsizeMaxExprVisitor::default(),
    );
    assert_eq!(visitor.count.get(), constants_usize::ONE);
}

#[test]
fn select_sites_match_reviewed_cancellation_inventory() {
    let reviewed = [
        (
            constants_str::test_fixtures::SERVE_WITH_GRACEFUL_SHUTDOWN_PATH,
            constants_usize::ONE,
            constants_str::test_fixtures::SERVE_WITH_GRACEFUL_SHUTDOWN_SELECT_REASON,
        ),
        (
            constants_str::test_fixtures::SPAWN_INTERVAL_TASK_PATH,
            constants_usize::ONE,
            constants_str::test_fixtures::VALUE_5337167F,
        ),
        (
            constants_str::test_fixtures::WAIT_FOR_SERVICE_SHUTDOWN_SIGNAL_PATH,
            constants_usize::ONE,
            constants_str::test_fixtures::VALUE_C8647B8D,
        ),
    ];
    super::code_style_snapshot::with_codebase_snapshot(|snapshot| {
        let mut observed = std::collections::BTreeMap::<String, usize>::new();
        let mut violations = snapshot
            .rs_files()
            .iter()
            .flat_map(|source_file| {
                let visitor = crate::code_style::visit_syn_file(
                    crate::types::SynFileRef::from(source_file.ast().as_ref()),
                    SelectMacroVisitor::default(),
                );
                if visitor.count.get() != constants_usize::ZERO {
                    let path = source_file.path().as_ref().display().to_string();
                    let reviewed_owner = reviewed
                        .iter()
                        .map(|(reviewed_path, _count, _reason)| *reviewed_path)
                        .filter(|reviewed_path| {
                            path.ends_with(reviewed_path)
                                || crate::code_style::declared_child_matches(
                                    path.as_str(),
                                    reviewed_path,
                                )
                        })
                        .max_by_key(|reviewed_path| reviewed_path.len());
                    if let Some(owner_suffix) = reviewed_owner {
                        let owner_path = format!("../{owner_suffix}");
                        let _observed_entry = observed
                            .entry(owner_path)
                            .and_modify(|count| {
                                *count = count.saturating_add(visitor.count.get());
                            })
                            .or_insert_with(|| visitor.count.get());
                    } else {
                        return std::iter::once(format!(
                            "{}: unreviewed select owner",
                            source_file.path().as_ref().display()
                        ))
                        .chain(visitor.unsafe_operations.into_iter().map(|violation| {
                            format!("{}: {violation}", source_file.path().as_ref().display())
                        }))
                        .collect::<Vec<String>>();
                    }
                }
                visitor
                    .unsafe_operations
                    .into_iter()
                    .map(|violation| {
                        format!("{}: {violation}", source_file.path().as_ref().display())
                    })
                    .collect::<Vec<String>>()
            })
            .collect::<Vec<String>>();
        reviewed
            .iter()
            .filter(|(_path, _count, reason)| reason.is_empty())
            .for_each(|(path, _count, _reason)| {
                violations.push(format!("select inventory `{path}` has no reason"));
            });
        let expected = reviewed
            .iter()
            .map(|(path, count, _reason)| (format!("../{path}"), *count))
            .collect::<std::collections::BTreeMap<String, usize>>();
        if observed != expected {
            violations.push(format!(
                "select inventory changed: expected={expected:#?}, observed={observed:#?}"
            ));
        }
        assert!(violations.is_empty(), "29fa10f5 {violations:#?}");
    });
}

#[test]
fn select_policy_rejects_cancellation_sensitive_operations() {
    let ast = syn::parse_file(constants_str::test_fixtures::VALUE_F6958372)
        .expect("714c620f invalid invariant must hold");
    let visitor = crate::code_style::visit_syn_file(
        crate::types::SynFileRef::from(&ast),
        SelectMacroVisitor::default(),
    );
    assert_eq!(visitor.unsafe_operations.len(), 2usize, "c4267f0a");
}

#[test]
fn architectural_boundaries_reject_upward_dependencies() {
    let boundaries = [
        (
            constants_str::test_fixtures::VALUE_9A26B6D6,
            constants_str::test_fixtures::VALUE_D54C0026,
        ),
        (
            constants_str::test_fixtures::VALUE_5906FF0B,
            constants_str::test_fixtures::VALUE_64313A40,
        ),
        (
            constants_str::test_fixtures::VALUE_B29A11B9,
            constants_str::test_fixtures::VALUE_FB301D46,
        ),
        (
            constants_str::test_fixtures::VALUE_E1717E8B,
            constants_str::test_fixtures::VALUE_72104B4E,
        ),
        (
            constants_str::test_fixtures::VALUE_B4F499E2,
            constants_str::test_fixtures::VALUE_2773E6CE,
        ),
    ];
    super::code_style_snapshot::with_codebase_snapshot(|snapshot| {
        let workspace_names = snapshot.workspace_crate_names();
        let mut violations = Vec::new();
        boundaries.iter().for_each(|(package_name, reason)| {
            if reason.is_empty() {
                violations.push(format!(
                    "architecture boundary `{package_name}` has no reason"
                ));
            }
            let package = snapshot
                .workspace_metadata()
                .get()
                .packages
                .iter()
                .find(|package| package.name == *package_name)
                .expect("010e6a3f architectural_boundaries_reject_upward_dependencies invariant must hold");
            let observed = package
                .dependencies
                .iter()
                .filter(|dependency| workspace_names.as_ref().contains(dependency.name.as_str()))
                .filter(|dependency| match *package_name {
                    constants_str::test_fixtures::VALUE_9A26B6D6 => {
                        dependency.name == constants_str::test_fixtures::VALUE_CA3132B2
                            || dependency.name.starts_with(constants_str::test_fixtures::VALUE_AAADEE66)
                            || dependency.name.starts_with(constants_str::test_fixtures::VALUE_D11DB134)
                            || (dependency.name.starts_with(constants_str::test_fixtures::VALUE_B3EACD33)
                                && dependency.name != constants_str::test_fixtures::VALUE_EC36C4C9)
                    }
                    constants_str::test_fixtures::VALUE_5906FF0B => {
                        dependency.name == constants_str::test_fixtures::VALUE_CA3132B2
                            || dependency.name == constants_str::test_fixtures::VALUE_B3EACD33
                            || dependency.name == constants_str::test_fixtures::VALUE_6D090579
                            || dependency.name == constants_str::test_fixtures::VALUE_D0393EDD
                            || dependency.name.ends_with(constants_str::test_fixtures::VALUE_93CEFD0B)
                            || dependency.name.starts_with(constants_str::test_fixtures::VALUE_AAADEE66)
                            || dependency.name.starts_with(constants_str::test_fixtures::VALUE_D11DB134)
                    }
                    constants_str::test_fixtures::VALUE_B29A11B9 | constants_str::test_fixtures::VALUE_E1717E8B | constants_str::test_fixtures::VALUE_B4F499E2 => {
                        dependency.name == constants_str::test_fixtures::VALUE_CA3132B2
                            || dependency.name == constants_str::test_fixtures::VALUE_B3EACD33
                            || dependency.name.starts_with(constants_str::test_fixtures::VALUE_6D090579)
                            || dependency.name.starts_with(constants_str::test_fixtures::VALUE_AAADEE66)
                            || dependency.name.ends_with(constants_str::test_fixtures::VALUE_14AD1127)
                            || dependency.name.starts_with(constants_str::test_fixtures::VALUE_D11DB134)
                    }
                    _ => true,
                })
                .map(|dependency| dependency.name.clone())
                .collect::<std::collections::BTreeSet<String>>();
            if !observed.is_empty() {
                violations.push(format!(
                    "{package_name} has upward workspace dependencies: {observed:?}"
                ));
            }
        });
        assert!(violations.is_empty(), "2fdc155b {violations:#?}");
    });
}

#[test]
fn lock_across_await_policy_requires_explicit_drop() {
    let invalid = syn::parse_file(constants_str::test_fixtures::VALUE_6F786FC4)
        .expect("b57df6a3 invalid invariant must hold");
    let valid = syn::parse_file(constants_str::test_fixtures::VALUE_D481790B)
        .expect("a62f1ce9 valid invariant must hold");
    let invalid_visitor = crate::code_style::visit_syn_file(
        crate::types::SynFileRef::from(&invalid),
        LockAcrossAwaitVisitor::default(),
    );
    let valid_visitor = crate::code_style::visit_syn_file(
        crate::types::SynFileRef::from(&valid),
        LockAcrossAwaitVisitor::default(),
    );
    assert_eq!(
        invalid_visitor.violations.len(),
        constants_usize::ONE,
        "bbfce72c"
    );
    assert!(valid_visitor.violations.is_empty(), "4b732bd1");
}

#[test]
fn production_code_does_not_use_explicit_leak_apis() {
    super::code_style_snapshot::with_codebase_snapshot(|snapshot| {
        let violations = snapshot
            .rs_files()
            .iter()
            .filter(|source_file| {
                !source_file
                    .path()
                    .as_ref()
                    .starts_with(constants_str::catalog::TESTS_CODE_STYLE)
            })
            .flat_map(|source_file| {
                let visitor = crate::code_style::visit_syn_file(
                    crate::types::SynFileRef::from(source_file.ast().as_ref()),
                    LeakApiVisitor::default(),
                );
                visitor.violations.into_iter().map(|violation| {
                    format!("{}: {violation}", source_file.path().as_ref().display())
                })
            })
            .collect::<Vec<String>>();
        assert!(violations.is_empty(), "8522eda3 {violations:#?}");
    });
}

#[test]
fn retained_spawn_tasks_are_supervised() {
    super::code_style_snapshot::with_codebase_snapshot(|snapshot| {
        let violations = snapshot
            .rs_files()
            .iter()
            .flat_map(|source_file| {
                let visitor = crate::code_style::visit_syn_file(
                    crate::types::SynFileRef::from(source_file.ast().as_ref()),
                    SpawnLifecycleVisitor::default(),
                );
                visitor.violations.into_iter().map(|violation| {
                    format!("{}: {violation}", source_file.path().as_ref().display())
                })
            })
            .collect::<Vec<String>>();
        assert!(violations.is_empty(), "aa16974d {violations:#?}");
    });
}

#[test]
fn spawn_lifecycle_policy_rejects_unconsumed_tasks() {
    let ast = syn::parse_file(constants_str::test_fixtures::VALUE_9F18A090)
        .expect("834138af tasks invariant must hold");
    let visitor = crate::code_style::visit_syn_file(
        crate::types::SynFileRef::from(&ast),
        SpawnLifecycleVisitor::default(),
    );
    assert_eq!(
        visitor.violations.as_slice(),
        [
            "spawned task `forgotten` is retained but never awaited, aborted, or transferred to an owner"
        ],
        "a1680c46"
    );
}

#[test]
fn route_path_segments_use_snake_case() {
    super::code_style_snapshot::with_codebase_snapshot(|snapshot| {
        let violations = snapshot
            .rs_files()
            .iter()
            .flat_map(|source_file| {
                let visitor = crate::code_style::visit_syn_file(
                    crate::types::SynFileRef::from(source_file.ast().as_ref()),
                    RouteLiteralVisitor::default(),
                );
                visitor.violations.into_iter().map(|violation| {
                    format!("{}: {violation}", source_file.path().as_ref().display())
                })
            })
            .collect::<Vec<String>>();
        assert!(violations.is_empty(), "ebde2ab8 {violations:#?}");
    });
}

#[test]
fn route_path_policy_rejects_kebab_case() {
    let ast = syn::parse_file(constants_str::test_fixtures::VALUE_72E2834F)
        .expect("9aa037dc route_path_policy_rejects_kebab_case invariant must hold");
    let visitor = crate::code_style::visit_syn_file(
        crate::types::SynFileRef::from(&ast),
        RouteLiteralVisitor::default(),
    );
    assert_eq!(visitor.violations.len(), constants_usize::ONE, "d15287e9");
}

#[test]
fn route_path_policy_rejects_api_prefix() {
    let ast = syn::parse_file(constants_str::test_fixtures::VALUE_D7270E5B)
        .expect("3eaa623d route_path_policy_rejects_api_prefix invariant must hold");
    let visitor = crate::code_style::visit_syn_file(
        crate::types::SynFileRef::from(&ast),
        RouteLiteralVisitor::default(),
    );
    assert_eq!(visitor.violations.len(), constants_usize::ONE, "5caaea72");
}
