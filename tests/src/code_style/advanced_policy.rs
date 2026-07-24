#![allow(
    clippy::arbitrary_source_item_ordering,
    clippy::needless_for_each,
    clippy::single_call_fn,
    clippy::wildcard_enum_match_arm,
    reason = "policy visitors stay grouped with their invariant, repository policy requires iterator methods, and syn non-exhaustive enums require fallback handling"
)]

#[derive(Default)]
struct AwaitVisitor {
    found: super::types::AnalyzerBool,
}
impl<'ast> syn::visit::Visit<'ast> for AwaitVisitor {
    fn visit_expr_await(&mut self, i: &'ast syn::ExprAwait) {
        self.found.set_true();
        syn::visit::visit_expr_await(self, i);
    }
}

#[derive(Default)]
struct LockAcrossAwaitVisitor {
    violations: super::types::DiagnosticMsgs,
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
                && local_initializer_acquires_lock(local)
                && let syn::Pat::Ident(identifier) = &local.pat
            {
                let _inserted = active_guards.insert(identifier.ident.to_string());
            }
            dropped_identifier(statement)
                .into_iter()
                .for_each(|identifier| {
                    let _removed = active_guards.remove(identifier.as_ref());
                });
        });
        syn::visit::visit_block(self, i);
    }
}

fn local_initializer_acquires_lock(local: &syn::Local) -> bool {
    local
        .init
        .as_ref()
        .is_some_and(|initializer| expression_acquires_lock(initializer.expr.as_ref()))
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
                    "lock" | "lock_owned" | "read" | "read_owned" | "write" | "write_owned"
                )
        }
        syn::Expr::Paren(paren) => expression_acquires_lock(paren.expr.as_ref()),
        syn::Expr::Try(try_expression) => expression_acquires_lock(try_expression.expr.as_ref()),
        _ => false,
    }
}

fn dropped_identifier(statement: &syn::Stmt) -> Option<super::types::SourceText> {
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
        Some("drop")
    ) {
        return None;
    }
    let syn::Expr::Path(argument) = call.args.first()? else {
        return None;
    };
    (argument.path.segments.len() == 1usize)
        .then(|| {
            argument.path.segments.first().map(|segment| {
                super::types::SourceText::try_from(segment.ident.to_string()).expect("d4f6bdce")
            })
        })
        .flatten()
}

#[derive(Default)]
struct LeakApiVisitor {
    violations: super::types::DiagnosticMsgs,
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
                .join("::");
            if [
                "Box::leak",
                "std::boxed::Box::leak",
                "std::mem::forget",
                "core::mem::forget",
                "Arc::into_raw",
                "std::sync::Arc::into_raw",
                "Box::into_raw",
                "std::boxed::Box::into_raw",
                "Arc::from_raw",
                "std::sync::Arc::from_raw",
                "Box::from_raw",
                "std::boxed::Box::from_raw",
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
            .is_some_and(|segment| segment.ident == "ManuallyDrop")
        {
            self.violations.push("ManuallyDrop".to_owned());
        }
        syn::visit::visit_type_path(self, i);
    }
}

#[derive(Default)]
struct SpawnConsumptionVisitor {
    consumed: super::types::StdSourceTextSet,
}
impl SpawnConsumptionVisitor {
    fn record_path(&mut self, expression: &syn::Expr) {
        if let syn::Expr::Path(path) = expression
            && path.path.segments.len() == 1usize
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
                && matches!(operation.to_string().as_str(), "abort" | "await")
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
                if path.path.segments.last().is_some_and(|segment| segment.ident == "drop")
        );
        if !is_drop {
            i.args
                .iter()
                .for_each(|argument| self.record_path(argument));
        }
        syn::visit::visit_expr_call(self, i);
    }
    fn visit_expr_method_call(&mut self, i: &'ast syn::ExprMethodCall) {
        if i.method == "abort" {
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

#[derive(Default)]
struct SpawnLifecycleVisitor {
    violations: super::types::DiagnosticMsgs,
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
                && local
                    .init
                    .as_ref()
                    .is_some_and(|initializer| super::unowned_spawn_expr(initializer.expr.as_ref()))
                && let syn::Pat::Ident(identifier) = &local.pat
                && !identifier.ident.to_string().starts_with('_')
            {
                let _inserted = pending.insert(identifier.ident.to_string());
            }
        });
        pending.into_iter().for_each(|identifier| {
            self.violations.push(format!(
                "spawn handle `{identifier}` is retained but never awaited, aborted, or transferred to an owner"
            ));
        });
        syn::visit::visit_block(self, i);
    }
}

#[derive(Default)]
struct RouteLiteralVisitor {
    violations: super::types::DiagnosticMsgs,
}
impl RouteLiteralVisitor {
    fn inspect_literal(&mut self, literal: &syn::LitStr) {
        let value = literal.value();
        if !value.starts_with('/') || value.starts_with("//") {
            return;
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
            matches!(segment.ident.to_string().as_str(), "typed_route" | "strum")
        }) && let syn::Meta::List(list) = &i.meta
        {
            self.inspect_tokens(list.tokens.clone());
        }
        syn::visit::visit_attribute(self, i);
    }
    fn visit_expr_method_call(&mut self, i: &'ast syn::ExprMethodCall) {
        if matches!(
            i.method.to_string().as_str(),
            "route" | "nest" | "route_service"
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

#[derive(Default)]
struct SelectMacroVisitor {
    count: super::types::AnalyzerCount,
    unsafe_operations: super::types::DiagnosticMsgs,
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
                    "read_exact"
                        | "read_to_end"
                        | "read_to_string"
                        | "reserve"
                        | "send"
                        | "write_all"
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
            .is_some_and(|segment| segment.ident == "select");
        if is_select {
            self.count.saturating_inc();
            self.inspect_sensitive_tokens(i.tokens.clone());
        }
        syn::visit::visit_macro(self, i);
    }
}

#[derive(Default)]
struct ExpressionPathVisitor {
    paths: super::types::SourceTextList,
}
impl<'ast> syn::visit::Visit<'ast> for ExpressionPathVisitor {
    fn visit_expr_path(&mut self, i: &'ast syn::ExprPath) {
        self.paths.push(
            super::path_to_string(super::types::SynPathRef::from(&i.path))
                .as_ref()
                .to_owned(),
        );
        syn::visit::visit_expr_path(self, i);
    }
}

#[derive(Default)]
struct IgnoredMapErrBindingVisitor {
    entries: super::types::DiagnosticMsgs,
}
impl<'ast> syn::visit::Visit<'ast> for IgnoredMapErrBindingVisitor {
    fn visit_expr_method_call(&mut self, i: &'ast syn::ExprMethodCall) {
        if i.method == "map_err"
            && let Some(syn::Expr::Closure(closure)) = i.args.first()
        {
            let ignored_inputs = closure
                .inputs
                .iter()
                .filter_map(|input| match input {
                    syn::Pat::Wild(_) => Some("_".to_owned()),
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

#[derive(Default)]
struct SharedDispatchVisitor {
    arc_types: super::types::AnalyzerCount,
    lock_types: super::types::AnalyzerCount,
    trait_objects: super::types::AnalyzerCount,
}
impl<'ast> syn::visit::Visit<'ast> for SharedDispatchVisitor {
    fn visit_type_path(&mut self, i: &'ast syn::TypePath) {
        if let Some(segment) = i.path.segments.last() {
            match segment.ident.to_string().as_str() {
                "Arc" => self.arc_types.saturating_inc(),
                "Mutex" | "RwLock" => self.lock_types.saturating_inc(),
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

struct PublicApiVisitor {
    entries: super::types::SourceTextList,
    lines: super::types::SourceTextList,
}
impl PublicApiVisitor {
    fn record(&mut self, span: proc_macro2::Span, signature_only: bool) {
        let start = span.start().line.saturating_sub(1usize);
        let end = span.end().line;
        let source = self
            .lines
            .get(start..end)
            .map(|lines| lines.join("\n"))
            .expect("3e180abf");
        let relevant = if signature_only {
            source
                .split_once('{')
                .map_or(source.as_str(), |(signature, _body)| signature)
        } else {
            source.as_str()
        };
        self.entries
            .push(relevant.split_whitespace().collect::<Vec<&str>>().join(" "));
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

#[derive(Default)]
struct PublicGenericVisitor {
    entries: super::types::SourceTextList,
}
impl PublicGenericVisitor {
    fn record(
        &mut self,
        kind: super::types::SourceTextRef<'_>,
        identifier: &syn::Ident,
        generics: &syn::Generics,
    ) {
        if !generics.params.is_empty() {
            self.entries.push(format!(
                "{}:{}:{}",
                kind.as_ref(),
                identifier,
                generics.params.len()
            ));
        }
    }
}
impl<'ast> syn::visit::Visit<'ast> for PublicGenericVisitor {
    fn visit_impl_item_fn(&mut self, i: &'ast syn::ImplItemFn) {
        if matches!(i.vis, syn::Visibility::Public(_)) {
            self.record(
                super::types::SourceTextRef::from("method"),
                &i.sig.ident,
                &i.sig.generics,
            );
        }
        syn::visit::visit_impl_item_fn(self, i);
    }
    fn visit_item_enum(&mut self, i: &'ast syn::ItemEnum) {
        if matches!(i.vis, syn::Visibility::Public(_)) {
            self.record(
                super::types::SourceTextRef::from("enum"),
                &i.ident,
                &i.generics,
            );
        }
        syn::visit::visit_item_enum(self, i);
    }
    fn visit_item_fn(&mut self, i: &'ast syn::ItemFn) {
        if matches!(i.vis, syn::Visibility::Public(_)) {
            self.record(
                super::types::SourceTextRef::from("function"),
                &i.sig.ident,
                &i.sig.generics,
            );
        }
        syn::visit::visit_item_fn(self, i);
    }
    fn visit_item_struct(&mut self, i: &'ast syn::ItemStruct) {
        if matches!(i.vis, syn::Visibility::Public(_)) {
            self.record(
                super::types::SourceTextRef::from("struct"),
                &i.ident,
                &i.generics,
            );
        }
        syn::visit::visit_item_struct(self, i);
    }
    fn visit_item_trait(&mut self, i: &'ast syn::ItemTrait) {
        if matches!(i.vis, syn::Visibility::Public(_)) {
            self.record(
                super::types::SourceTextRef::from("trait"),
                &i.ident,
                &i.generics,
            );
        }
        syn::visit::visit_item_trait(self, i);
    }
}

#[derive(Default)]
struct StructErrorVisitor {
    identifiers: super::types::SourceTextList,
}
impl<'ast> syn::visit::Visit<'ast> for StructErrorVisitor {
    fn visit_item_struct(&mut self, i: &'ast syn::ItemStruct) {
        if i.attrs.iter().any(|attribute| {
            super::derive_attr_has_terminal(
                super::types::SynAttributeRef::from(attribute),
                super::types::SourceTextRef::from("Error"),
            )
            .get()
        }) {
            self.identifiers.push(i.ident.to_string());
        }
        syn::visit::visit_item_struct(self, i);
    }
}

#[derive(Default)]
struct LoopAllocationVisitor {
    depth: super::types::AnalyzerCount,
    entries: super::types::DiagnosticMsgs,
}
impl LoopAllocationVisitor {
    fn record(&mut self, operation: super::types::SourceTextRef<'_>, span: proc_macro2::Span) {
        if self.depth.get() != 0usize {
            self.entries.push(format!(
                "line {}: {}",
                span.start().line,
                operation.as_ref()
            ));
        }
    }
}
impl<'ast> syn::visit::Visit<'ast> for LoopAllocationVisitor {
    fn visit_expr_call(&mut self, i: &'ast syn::ExprCall) {
        if let syn::Expr::Path(function) = i.func.as_ref() {
            let path = super::path_to_string(super::types::SynPathRef::from(&function.path));
            if [
                "Box::new",
                "String::from",
                "String::new",
                "String::with_capacity",
                "Vec::new",
                "Vec::with_capacity",
                "std::boxed::Box::new",
                "std::string::String::from",
                "std::string::String::new",
                "std::string::String::with_capacity",
                "std::vec::Vec::new",
                "std::vec::Vec::with_capacity",
            ]
            .contains(&path.as_ref())
            {
                self.record(
                    super::types::SourceTextRef::from(path.as_ref()),
                    syn::spanned::Spanned::span(i),
                );
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
            "clone" | "collect" | "to_owned" | "to_string"
        ) {
            self.record(
                super::types::SourceTextRef::from(i.method.to_string().as_str()),
                syn::spanned::Spanned::span(i),
            );
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
            && matches!(segment.ident.to_string().as_str(), "format" | "vec")
        {
            let operation = segment.ident.to_string();
            self.record(
                super::types::SourceTextRef::from(operation.as_str()),
                syn::spanned::Spanned::span(i),
            );
        }
        syn::visit::visit_macro(self, i);
    }
}

#[derive(Default)]
struct MutableBindingVisitor {
    entries: super::types::DiagnosticMsgs,
}
impl<'ast> syn::visit::Visit<'ast> for MutableBindingVisitor {
    fn visit_pat_ident(&mut self, i: &'ast syn::PatIdent) {
        if i.mutability.is_some() {
            self.entries.push(format!(
                "line {}: {}",
                syn::spanned::Spanned::span(i).start().line,
                i.ident
            ));
        }
        syn::visit::visit_pat_ident(self, i);
    }
}

#[test]
fn lock_guards_are_not_held_across_await() {
    super::snapshot::with_codebase_snapshot(|snapshot| {
        let violations = snapshot
            .rs_files()
            .iter()
            .flat_map(|source_file| {
                let visitor = super::visit_syn_file(
                    super::types::SynFileRef::from(source_file.ast().as_ref()),
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
fn mutable_bindings_match_reviewed_snapshot() {
    let expected_count = 453usize;
    let expected_hash = 1_912_037_368_430_502_475u64;
    let reason = "existing mutable bindings are reviewed; new mutation requires an explicit state-transition review";
    super::snapshot::with_codebase_snapshot(|snapshot| {
        let mut entries = snapshot
            .rs_files()
            .iter()
            .filter(|source_file| {
                !source_file
                    .path()
                    .as_ref()
                    .components()
                    .any(|component| component.as_os_str() == "tests")
            })
            .flat_map(|source_file| {
                let visitor = super::visit_syn_file(
                    super::types::SynFileRef::from(source_file.ast().as_ref()),
                    MutableBindingVisitor::default(),
                );
                let path = source_file.path().as_ref().display().to_string();
                visitor
                    .entries
                    .into_iter()
                    .map(move |entry| format!("{path}:{entry}"))
            })
            .collect::<Vec<String>>();
        entries.sort();
        let observed_hash = entries
            .iter()
            .fold(14_695_981_039_346_656_037u64, |hash, entry| {
                entry
                    .bytes()
                    .chain(std::iter::once(0xffu8))
                    .fold(hash, |inner_hash, byte| {
                        (inner_hash ^ u64::from(byte)).wrapping_mul(1_099_511_628_211u64)
                    })
            });
        assert!(
            !reason.is_empty() && entries.len() == expected_count && observed_hash == expected_hash,
            "a4e02941 mutable binding snapshot changed: count={}, hash={observed_hash}",
            entries.len()
        );
    });
}

#[test]
fn allocations_inside_loops_match_reviewed_snapshot() {
    let expected_count = 8usize;
    let expected_hash = 15_663_531_209_084_868_204u64;
    let reason = "existing loop allocations are reviewed; new allocations require hoisting or an explicit hot-path review";
    super::snapshot::with_codebase_snapshot(|snapshot| {
        let mut entries = snapshot
            .rs_files()
            .iter()
            .filter(|source_file| {
                !source_file
                    .path()
                    .as_ref()
                    .components()
                    .any(|component| component.as_os_str() == "tests")
            })
            .flat_map(|source_file| {
                let visitor = super::visit_syn_file(
                    super::types::SynFileRef::from(source_file.ast().as_ref()),
                    LoopAllocationVisitor::default(),
                );
                let path = source_file.path().as_ref().display().to_string();
                visitor
                    .entries
                    .into_iter()
                    .map(move |entry| format!("{path}:{entry}"))
            })
            .collect::<Vec<String>>();
        entries.sort();
        let observed_hash = entries
            .iter()
            .fold(14_695_981_039_346_656_037u64, |hash, entry| {
                entry
                    .bytes()
                    .chain(std::iter::once(0xffu8))
                    .fold(hash, |inner_hash, byte| {
                        (inner_hash ^ u64::from(byte)).wrapping_mul(1_099_511_628_211u64)
                    })
            });
        assert!(
            !reason.is_empty() && entries.len() == expected_count && observed_hash == expected_hash,
            "418fe0af loop allocation snapshot changed: count={}, hash={observed_hash}",
            entries.len()
        );
    });
}

#[test]
fn struct_error_exceptions_match_reviewed_snapshot() {
    let expected_count = 126usize;
    let expected_hash = 10_317_420_352_954_407_195u64;
    let reason = "existing single-case and transparent errors are reviewed; new multi-case errors must use enums";
    super::snapshot::with_codebase_snapshot(|snapshot| {
        let mut entries = snapshot
            .rs_files()
            .iter()
            .flat_map(|source_file| {
                let visitor = super::visit_syn_file(
                    super::types::SynFileRef::from(source_file.ast().as_ref()),
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
        let observed_hash = entries
            .iter()
            .fold(14_695_981_039_346_656_037u64, |hash, entry| {
                entry
                    .bytes()
                    .chain(std::iter::once(0xffu8))
                    .fold(hash, |inner_hash, byte| {
                        (inner_hash ^ u64::from(byte)).wrapping_mul(1_099_511_628_211u64)
                    })
            });
        assert!(
            !reason.is_empty() && entries.len() == expected_count && observed_hash == expected_hash,
            "731ffc35 struct error inventory changed: count={}, hash={observed_hash}",
            entries.len()
        );
    });
}

#[test]
fn public_generic_surface_matches_reviewed_snapshot() {
    let expected_count = 248usize;
    let expected_hash = 1_706_126_233_691_996_212u64;
    let reason = "existing generic APIs are reviewed; additions require explicit API review";
    super::snapshot::with_codebase_snapshot(|snapshot| {
        let mut entries = snapshot
            .rs_files()
            .iter()
            .flat_map(|source_file| {
                let visitor = super::visit_syn_file(
                    super::types::SynFileRef::from(source_file.ast().as_ref()),
                    PublicGenericVisitor::default(),
                );
                let path = source_file.path().as_ref().display().to_string();
                visitor
                    .entries
                    .into_iter()
                    .map(move |entry| format!("{path}:{entry}"))
            })
            .collect::<Vec<String>>();
        entries.sort();
        let observed_hash = entries
            .iter()
            .fold(14_695_981_039_346_656_037u64, |hash, entry| {
                entry
                    .bytes()
                    .chain(std::iter::once(0xffu8))
                    .fold(hash, |inner_hash, byte| {
                        (inner_hash ^ u64::from(byte)).wrapping_mul(1_099_511_628_211u64)
                    })
            });
        assert!(
            !reason.is_empty() && entries.len() == expected_count && observed_hash == expected_hash,
            "151d6963 public generic API snapshot changed: count={}, hash={observed_hash}",
            entries.len()
        );
    });
}

#[test]
fn contract_public_api_matches_reviewed_snapshot() {
    let reviewed = std::collections::BTreeMap::from([
        (
            "common_routes/src",
            (
                26usize,
                4_784_818_472_386_725_300u64,
                "common route contract",
            ),
        ),
        (
            "frontend_contract/src",
            (
                302usize,
                14_613_832_698_209_345_402u64,
                "generic frontend route contract",
            ),
        ),
        (
            "server_admin_contract/src",
            (
                358usize,
                16_941_927_145_104_310_330u64,
                "administrator wire contract",
            ),
        ),
    ]);
    super::snapshot::with_codebase_snapshot(|snapshot| {
        let mut violations = Vec::new();
        reviewed
            .iter()
            .for_each(|(directory_suffix, (expected_count, expected_hash, reason))| {
                if reason.is_empty() {
                    violations.push(format!(
                        "public API snapshot `{directory_suffix}` has no reason"
                    ));
                }
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
                        let visitor = super::visit_syn_file(
                            super::types::SynFileRef::from(source_file.ast().as_ref()),
                            PublicApiVisitor {
                                entries: super::types::SourceTextList::default(),
                                lines: super::types::SourceTextList::from(
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
                let observed_hash = entries.iter().fold(
                    14_695_981_039_346_656_037u64,
                    |hash, entry| {
                        entry
                            .bytes()
                            .chain(std::iter::once(0xffu8))
                            .fold(hash, |inner_hash, byte| {
                                (inner_hash ^ u64::from(byte))
                                    .wrapping_mul(1_099_511_628_211u64)
                            })
                    },
                );
                if entries.len() != *expected_count || observed_hash != *expected_hash {
                    violations.push(format!(
                        "{directory_suffix}: public API snapshot changed: count={}, hash={observed_hash}",
                        entries.len()
                    ));
                }
            });
        assert!(violations.is_empty(), "505a0cf7 {violations:#?}");
    });
}

#[test]
fn arc_lock_and_trait_object_usage_matches_reviewed_inventory() {
    let reviewed = std::collections::BTreeMap::from([
        (
            "location_lib/location/src/lib.rs",
            (
                0usize,
                0usize,
                3usize,
                "location formatting accepts heterogeneous display values",
            ),
        ),
        (
            "macros_helpers/src/generate_impl_to_err_string_token_stream.rs",
            (
                0,
                0,
                4,
                "token generation accepts heterogeneous ToTokens inputs",
            ),
        ),
        (
            "macros_helpers/src/generate_pub_type_alias_token_stream.rs",
            (
                0,
                0,
                2,
                "token generation accepts heterogeneous ToTokens inputs",
            ),
        ),
        (
            "macros_helpers/src/generate_if_write_is_err_token_stream.rs",
            (
                0,
                0,
                2,
                "token generation accepts heterogeneous ToTokens inputs",
            ),
        ),
        (
            "macros_helpers/src/generate_impl_try_from_token_stream.rs",
            (
                0,
                0,
                4,
                "token generation accepts heterogeneous ToTokens inputs",
            ),
        ),
        (
            "macros_helpers/src/generate_impl_default_token_stream.rs",
            (
                0,
                0,
                2,
                "token generation accepts heterogeneous ToTokens inputs",
            ),
        ),
        (
            "macros_helpers/src/generate_impl_from_token_stream.rs",
            (
                0,
                0,
                3,
                "token generation accepts heterogeneous ToTokens inputs",
            ),
        ),
        (
            "macros_helpers/src/generate_new_or_try_new.rs",
            (
                0,
                0,
                70,
                "the generator composes heterogeneous token fragments without exposing generics",
            ),
        ),
        (
            "macros_helpers/src/pagination_start_end_initialization_token_stream.rs",
            (
                0,
                0,
                1,
                "token generation accepts a heterogeneous ToTokens input",
            ),
        ),
        (
            "macros_helpers/src/generate_impl_display_token_stream.rs",
            (
                0,
                0,
                4,
                "token generation accepts heterogeneous ToTokens inputs",
            ),
        ),
        (
            "macros_helpers/generate_derive_token_stream_builder/src/lib.rs",
            (
                0,
                0,
                5,
                "the derive builder composes heterogeneous token fragments",
            ),
        ),
        (
            "optml/src/lib.rs",
            (
                0,
                0,
                1,
                "the proc-macro forwards a heterogeneous ToTokens input",
            ),
        ),
        (
            "naming/naming_macros/src/lib.rs",
            (
                0,
                0,
                10,
                "naming generators format heterogeneous token values",
            ),
        ),
        (
            "common_routes/src/lib.rs",
            (
                4,
                0,
                3,
                "route state is shared across threads behind its parameter trait",
            ),
        ),
        (
            "server/src/main.rs",
            (
                3,
                0,
                1,
                "the server shares application state across worker threads",
            ),
        ),
        (
            "server_runtime/src/health.rs",
            (1, 0, 0, "health state is shared across request tasks"),
        ),
        (
            "server_runtime/src/limits.rs",
            (1, 0, 0, "the semaphore is shared across request tasks"),
        ),
        (
            "server_runtime/src/lib.rs",
            (
                2,
                0,
                6,
                "runtime middleware shares state and erases heterogeneous service errors",
            ),
        ),
        (
            "server_runtime/src/resource_budget.rs",
            (
                1,
                0,
                0,
                "the resource budget semaphore is shared across tasks",
            ),
        ),
        (
            "server_runtime/src/single_flight.rs",
            (
                1,
                1,
                0,
                "single-flight ownership requires shared synchronized state",
            ),
        ),
        (
            "server_runtime/src/bounded_read.rs",
            (1, 0, 0, "the read limiter is shared across request tasks"),
        ),
        (
            "server_runtime/src/history.rs",
            (
                1,
                1,
                0,
                "run history is shared and asynchronously synchronized",
            ),
        ),
        (
            "server_runtime/src/lease_registry.rs",
            (
                1,
                1,
                0,
                "lease state is shared and asynchronously synchronized",
            ),
        ),
        (
            "server_runtime/src/metrics_layer.rs",
            (
                1,
                1,
                1,
                "the bounded metrics cache is shared across request threads",
            ),
        ),
        (
            "pg_crud/where_filters/src/lib.rs",
            (
                0,
                0,
                4,
                "query fragments use dynamic dispatch over heterogeneous SQL parts",
            ),
        ),
        (
            "pg_crud/where_filters/generate_where_filters_src/src/source.rs",
            (
                0,
                0,
                31,
                "the generator composes heterogeneous token fragments",
            ),
        ),
        (
            "pg_crud/pg_crud_macros_common/src/filters.rs",
            (
                0,
                0,
                2,
                "filter generation accepts heterogeneous token fragments",
            ),
        ),
        (
            "pg_crud/pg_crud_macros_common/src/lib.rs",
            (
                0,
                0,
                96,
                "CRUD generation composes heterogeneous token fragments",
            ),
        ),
        (
            "pg_crud/pg_crud_macros_common/src/pg_type_test_cases.rs",
            (
                0,
                0,
                51,
                "generated fixtures compose heterogeneous token fragments",
            ),
        ),
        (
            "pg_crud/pg_crud_macros_common/src/token_stream_helpers.rs",
            (
                0,
                0,
                11,
                "token helpers accept heterogeneous token fragments",
            ),
        ),
        (
            "pg_crud/pg_crud_common/src/query_fragment.rs",
            (
                0,
                0,
                1,
                "query fragments require heterogeneous SQL part dispatch",
            ),
        ),
        (
            "pg_crud/pg_crud_common/src/lib.rs",
            (
                0,
                0,
                11,
                "CRUD contracts operate on heterogeneous query parts",
            ),
        ),
        (
            "pg_crud/pg_table/generate_pg_table_src/src/source.rs",
            (
                0,
                0,
                179,
                "table generation composes heterogeneous token fragments",
            ),
        ),
        (
            "pg_crud/pg_types/generate_pg_types_src/src/source.rs",
            (
                0,
                0,
                177,
                "type generation composes heterogeneous token fragments",
            ),
        ),
        (
            "pg_crud/pg_types/pg_types_common/src/lib.rs",
            (0, 0, 1, "PostgreSQL query parts use dynamic dispatch"),
        ),
        (
            "workspace_test_runner/src/main.rs",
            (0, 0, 1, "the runner prints heterogeneous command errors"),
        ),
        (
            "server_admin/src/generated_tables.rs",
            (1, 0, 1, "generated handlers share erased application state"),
        ),
        (
            "server_admin/src/lib.rs",
            (
                1,
                0,
                0,
                "administrator state is shared across request tasks",
            ),
        ),
        (
            "server_admin/src/generated_auth.rs",
            (
                0,
                0,
                1,
                "generated authentication accepts heterogeneous service implementations",
            ),
        ),
        (
            "server_admin/src/auth.rs",
            (
                1,
                0,
                0,
                "authentication state is shared across request tasks",
            ),
        ),
    ]);
    super::snapshot::with_codebase_snapshot(|snapshot| {
        let mut matched = std::collections::BTreeSet::new();
        let mut violations = Vec::new();
        snapshot
            .rs_files()
            .iter()
            .filter(|source_file| {
                !source_file
                    .path()
                    .as_ref()
                    .components()
                    .any(|component| component.as_os_str() == "tests")
            })
            .for_each(|source_file| {
                let visitor = super::visit_syn_file(
                    super::types::SynFileRef::from(source_file.ast().as_ref()),
                    SharedDispatchVisitor::default(),
                );
                let observed = (
                    visitor.arc_types.get(),
                    visitor.lock_types.get(),
                    visitor.trait_objects.get(),
                );
                if observed == (0usize, 0usize, 0usize) {
                    return;
                }
                let path = source_file.path().as_ref().display().to_string();
                let reviewed_entry =
                    reviewed
                        .iter()
                        .find(|(suffix, (_arc, _lock, _traits, reason))| {
                            path.ends_with(**suffix) && !reason.is_empty()
                        });
                match reviewed_entry {
                    Some((suffix, (arc, lock, traits, _reason)))
                        if observed == (*arc, *lock, *traits) =>
                    {
                        let _inserted = matched.insert((*suffix).to_owned());
                    }
                    Some((suffix, expected)) => violations.push(format!(
                        "{path}: shared-dispatch inventory changed for {suffix}: expected={:?}, observed={observed:?}",
                        (expected.0, expected.1, expected.2)
                    )),
                    None => violations.push(format!(
                        "{path}: unreviewed Arc/lock/trait-object usage {observed:?}"
                    )),
                }
            });
        if matched.len() != reviewed.len() {
            violations.push(format!(
                "stale Arc/lock/trait-object inventory: matched={matched:#?}"
            ));
        }
        assert!(violations.is_empty(), "66b91e7a {violations:#?}");
    });
}

#[test]
fn ignored_map_err_bindings_match_reviewed_inventory() {
    let reviewed = std::collections::BTreeMap::from([
        (
            "external_service_emulators/src/lib.rs",
            (
                1usize,
                18_044_733_430_376_104_380u64,
                "the emulator maps channel closure to its domain error",
            ),
        ),
        (
            "route_validators/src/hdr_val.rs",
            (
                2usize,
                8_307_973_547_828_984_310u64,
                "header parse details are intentionally mapped to validation errors",
            ),
        ),
        (
            "macros_helpers/src/test_database.rs",
            (
                1usize,
                17_615_506_122_534_003_937u64,
                "the test database helper maps setup failure to its fixture error",
            ),
        ),
        (
            "macros_helpers/src/write_string_into_file.rs",
            (
                1usize,
                15_850_734_906_468_416_334u64,
                "the file helper maps conversion failure to its domain error",
            ),
        ),
        (
            "notification_service/src/main.rs",
            (
                2usize,
                10_624_237_169_294_773_147u64,
                "service bootstrap classifies configuration failures",
            ),
        ),
        (
            "file_storage/src/lib.rs",
            (
                1usize,
                16_295_999_883_235_516_952u64,
                "storage input failure is classified at the boundary",
            ),
        ),
        (
            "server_runtime/src/outbound_url.rs",
            (
                1usize,
                10_756_600_543_045_051_859u64,
                "URL parse details are intentionally hidden by the domain error",
            ),
        ),
        (
            "server_runtime/src/wire_token.rs",
            (
                1usize,
                6_379_813_932_589_802_468u64,
                "wire token part failures map to a stable public category",
            ),
        ),
        (
            "server_runtime/src/origin.rs",
            (
                1usize,
                5_273_783_931_958_932_094u64,
                "origin parsing maps to a stable validation error",
            ),
        ),
        (
            "server_runtime/src/secure_cookie.rs",
            (
                1usize,
                15_429_310_994_391_673_823u64,
                "cookie header details are intentionally redacted",
            ),
        ),
        (
            "server_runtime/src/multipart.rs",
            (
                1usize,
                14_453_656_047_801_799_853u64,
                "multipart path validation exposes a stable domain error",
            ),
        ),
        (
            "server_runtime/src/lib.rs",
            (
                1usize,
                16_149_390_479_206_234_825u64,
                "timeout details map to the public shutdown timeout variant",
            ),
        ),
        (
            "server_runtime/src/bounded_read.rs",
            (
                1usize,
                5_207_021_504_593_722_365u64,
                "closed limiter state maps to a stable read error",
            ),
        ),
        (
            "server_runtime/src/child_process.rs",
            (
                1usize,
                15_752_444_618_703_500_496u64,
                "elapsed timeout details map to the child timeout variant",
            ),
        ),
        (
            "server_runtime/src/http_header_policy.rs",
            (
                3usize,
                5_326_244_696_809_361_619u64,
                "header construction errors are intentionally classified",
            ),
        ),
        (
            "server_runtime/src/exclusive_run.rs",
            (
                1usize,
                1_027_271_799_673_455_782u64,
                "the atomic compare failure maps to already active",
            ),
        ),
        (
            "pg_crud/pg_crud_common/src/read_query_plan.rs",
            (
                3usize,
                8_623_507_500_622_176_715u64,
                "query plan validation maps to stable contract errors",
            ),
        ),
        (
            "pg_crud/pg_crud_common/src/cursor.rs",
            (
                8usize,
                7_882_507_617_510_199_337u64,
                "cursor parsing maps low-level failures to wire categories",
            ),
        ),
        (
            "pg_crud/pg_crud_common/src/date_sql_filter.rs",
            (
                2usize,
                8_624_832_464_391_947_517u64,
                "date filter parsing maps to contract validation errors",
            ),
        ),
        (
            "pg_crud/pg_crud_common/src/advisory_lock.rs",
            (
                1usize,
                8_862_452_122_470_076_736u64,
                "advisory lock conversion maps to its bounded domain error",
            ),
        ),
        (
            "pg_crud/pg_table/src/lib.rs",
            (
                1usize,
                11_558_424_024_508_308_790u64,
                "table validation maps generated failures to a public category",
            ),
        ),
        (
            "config_lib/src/lib.rs",
            (
                4usize,
                13_436_572_168_389_940_861u64,
                "configuration parsing exposes stable field errors",
            ),
        ),
        (
            "workspace_test_runner/src/main.rs",
            (
                1usize,
                11_490_469_477_730_315_910u64,
                "runner input conversion maps to a command error",
            ),
        ),
        (
            "workspace_test_runner/src/execution.rs",
            (
                1usize,
                9_728_249_661_848_494_227u64,
                "summary initialization maps to the runner error",
            ),
        ),
        (
            "frontend_contract/src/json_snapshot.rs",
            (
                2usize,
                15_245_506_429_194_649_089u64,
                "serialization details map to snapshot contract errors",
            ),
        ),
        (
            "workspace_scaffold/src/main.rs",
            (
                5usize,
                17_957_165_660_958_926_640u64,
                "catalog parsing maps to stable scaffold errors",
            ),
        ),
        (
            "newtype/src/lib.rs",
            (
                1usize,
                12_531_204_604_735_508_744u64,
                "invalid derive input maps to the macro diagnostic",
            ),
        ),
        (
            "server_admin/src/domain.rs",
            (
                4usize,
                3_525_000_150_470_207_167u64,
                "domain conversion failures map to administrator validation errors",
            ),
        ),
        (
            "server_admin/src/generated_tables.rs",
            (
                1usize,
                14_655_213_785_439_917_732u64,
                "generated table conformance maps to its public error",
            ),
        ),
        (
            "server_admin/src/repository/users.rs",
            (
                12usize,
                7_657_170_105_147_143_872u64,
                "repository row conversions map to typed repository errors",
            ),
        ),
        (
            "server_admin/src/repository/settings.rs",
            (
                8usize,
                14_432_114_671_816_167_631u64,
                "settings row conversions map to typed repository errors",
            ),
        ),
        (
            "server_admin/src/repository/data_tables.rs",
            (
                21usize,
                6_200_526_850_154_800_565u64,
                "data table parsing maps to typed repository errors",
            ),
        ),
        (
            "server_admin/src/repository/audit.rs",
            (
                10usize,
                14_095_644_810_635_619_870u64,
                "audit row conversions map to typed repository errors",
            ),
        ),
        (
            "server_admin/src/repository/rate_limits.rs",
            (
                4usize,
                10_064_388_524_006_650_716u64,
                "rate-limit row conversions map to typed repository errors",
            ),
        ),
        (
            "server_admin/src/repository/sessions.rs",
            (
                5usize,
                9_999_204_811_137_137_062u64,
                "session row conversions map to typed repository errors",
            ),
        ),
        (
            "server_admin/src/repository/cleanup.rs",
            (
                1usize,
                5_154_652_375_526_262_556u64,
                "cleanup conversion maps to a typed repository error",
            ),
        ),
        (
            "server_admin/src/repository/permissions.rs",
            (
                6usize,
                17_592_933_992_679_763_826u64,
                "permission row conversions map to typed repository errors",
            ),
        ),
        (
            "server_admin/src/repository/roles.rs",
            (
                10usize,
                15_602_821_806_272_741_138u64,
                "role row conversions map to typed repository errors",
            ),
        ),
        (
            "server_admin/src/repository.rs",
            (
                1usize,
                12_885_392_875_274_270_006u64,
                "repository acquisition maps to the administrator database error",
            ),
        ),
        (
            "server_admin/src/auth/audit.rs",
            (
                3usize,
                10_626_838_172_502_960_176u64,
                "audit request validation maps to stable API errors",
            ),
        ),
        (
            "server_admin/src/auth/html.rs",
            (
                9usize,
                4_683_197_711_457_340_835u64,
                "HTML form parsing maps details to stable API errors",
            ),
        ),
        (
            "server_admin/src/auth/session.rs",
            (
                1usize,
                14_647_519_906_466_647_223u64,
                "system clock failure maps to the session category",
            ),
        ),
        (
            "server_admin/src/auth/handlers.rs",
            (
                15usize,
                1_778_387_811_993_442_835u64,
                "handler input failures map to stable API categories",
            ),
        ),
        (
            "server_admin/src/auth.rs",
            (
                12usize,
                45_155_265_412_043_747u64,
                "authentication failures map to stable and redacted API categories",
            ),
        ),
        (
            "server_admin_frontend/src/app.rs",
            (
                29usize,
                12_202_620_818_695_603_365u64,
                "browser failures map to serializable UI error categories",
            ),
        ),
    ]);
    super::snapshot::with_codebase_snapshot(|snapshot| {
        let mut matched = std::collections::BTreeSet::new();
        let mut violations = Vec::new();
        snapshot.rs_files().iter().for_each(|source_file| {
            let visitor = super::visit_syn_file(
                super::types::SynFileRef::from(source_file.ast().as_ref()),
                IgnoredMapErrBindingVisitor::default(),
            );
            if visitor.entries.is_empty() {
                return;
            }
            let fingerprint = visitor.entries.iter().fold(
                14_695_981_039_346_656_037u64,
                |hash, entry| {
                    entry
                        .bytes()
                        .chain(std::iter::once(0xffu8))
                        .fold(hash, |inner_hash, byte| {
                            (inner_hash ^ u64::from(byte))
                                .wrapping_mul(1_099_511_628_211u64)
                        })
                },
            );
            let path = source_file.path().as_ref().display().to_string();
            let reviewed_entry = reviewed.iter().find(|(suffix, (_count, _hash, reason))| {
                path.ends_with(**suffix) && !reason.is_empty()
            });
            match reviewed_entry {
                Some((suffix, (count, expected_fingerprint, _reason)))
                    if *count == visitor.entries.len()
                        && *expected_fingerprint == fingerprint =>
                {
                    let _inserted = matched.insert((*suffix).to_owned());
                }
                Some((suffix, (count, expected_fingerprint, _reason))) => {
                    violations.push(format!(
                        "{path}: ignored map_err inventory changed for {suffix}: expected count={count}, fingerprint={expected_fingerprint}; observed count={}, fingerprint={fingerprint}",
                        visitor.entries.len()
                    ));
                }
                None => violations.push(format!(
                    "{path}: unreviewed ignored map_err bindings: count={}, fingerprint={fingerprint}",
                    visitor.entries.len()
                )),
            }
        });
        if matched.len() != reviewed.len() {
            violations.push(format!(
                "stale ignored map_err inventory: matched={matched:#?}"
            ));
        }
        assert!(violations.is_empty(), "bb0dbc1f {violations:#?}");
    });
}

#[test]
fn select_sites_match_reviewed_cancellation_inventory() {
    let reviewed = [
        (
            "server/src/main.rs",
            1usize,
            "the shutdown signal races two cancellation-safe signal receivers",
        ),
        (
            "server_runtime/src/lib.rs",
            1usize,
            "the pinned server future is resumed after the shutdown notification branch",
        ),
        (
            "server_runtime/src/lifecycle.rs",
            1usize,
            "the interval tick and oneshot shutdown receiver are cancellation-safe",
        ),
    ];
    super::snapshot::with_codebase_snapshot(|snapshot| {
        let mut observed = std::collections::BTreeMap::<String, usize>::new();
        let mut violations = snapshot
            .rs_files()
            .iter()
            .flat_map(|source_file| {
                let visitor = super::visit_syn_file(
                    super::types::SynFileRef::from(source_file.ast().as_ref()),
                    SelectMacroVisitor::default(),
                );
                if visitor.count.get() != 0usize {
                    let _previous = observed.insert(
                        source_file.path().as_ref().display().to_string(),
                        visitor.count.get(),
                    );
                }
                visitor.unsafe_operations.into_iter().map(|violation| {
                    format!("{}: {violation}", source_file.path().as_ref().display())
                })
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
    let ast = syn::parse_file(
        "async fn invalid(reader: &mut Reader, sender: &Sender) {
            tokio::select! {
                value = reader.read_exact(&mut [0u8; 8]) => drop(value),
                value = sender.send(Message) => drop(value),
            }
        }",
    )
    .expect("714c620f");
    let visitor = super::visit_syn_file(
        super::types::SynFileRef::from(&ast),
        SelectMacroVisitor::default(),
    );
    assert_eq!(visitor.unsafe_operations.len(), 2usize, "c4267f0a");
}

#[test]
fn architectural_boundary_dependencies_match_reviewed_sets() {
    let reviewed = [
        (
            "frontend_contract",
            [
                "frontend_contract_macros",
                "newtype",
                "str_constants",
                "to_err_string",
            ]
            .as_slice(),
            "the generic frontend contract depends only on its derive and shared value crates",
        ),
        (
            "server_admin_contract",
            [
                "frontend_contract",
                "newtype",
                "str_constants",
                "text_policy",
            ]
            .as_slice(),
            "the administrator contract builds on the generic contract and shared value crates",
        ),
        (
            "server_runtime",
            ["newtype", "str_constants", "text_policy"].as_slice(),
            "the runtime foundation must not depend on application or route crates",
        ),
    ];
    super::snapshot::with_codebase_snapshot(|snapshot| {
        let workspace_names = snapshot.workspace_crate_names();
        let mut violations = Vec::new();
        reviewed
            .iter()
            .for_each(|(package_name, expected_dependencies, reason)| {
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
                    .expect("010e6a3f");
                let observed = package
                    .dependencies
                    .iter()
                    .filter(|dependency| {
                        workspace_names.as_ref().contains(dependency.name.as_str())
                    })
                    .map(|dependency| dependency.name.clone())
                    .collect::<std::collections::BTreeSet<String>>();
                let expected = expected_dependencies
                    .iter()
                    .map(|dependency| (*dependency).to_owned())
                    .collect::<std::collections::BTreeSet<String>>();
                if observed != expected {
                    violations.push(format!(
                        "{package_name} dependencies changed: expected={expected:?}, observed={observed:?}"
                    ));
                }
            });
        assert!(violations.is_empty(), "2fdc155b {violations:#?}");
    });
}

#[test]
fn lock_across_await_policy_requires_explicit_drop() {
    let invalid = syn::parse_file(
        "async fn invalid(lock: &tokio::sync::Mutex<u8>) {
            let guard = lock.lock().await;
            operation().await;
            drop(guard);
        }",
    )
    .expect("b57df6a3");
    let valid = syn::parse_file(
        "async fn valid(lock: &tokio::sync::Mutex<u8>) {
            let guard = lock.lock().await;
            drop(guard);
            operation().await;
        }",
    )
    .expect("a62f1ce9");
    let invalid_visitor = super::visit_syn_file(
        super::types::SynFileRef::from(&invalid),
        LockAcrossAwaitVisitor::default(),
    );
    let valid_visitor = super::visit_syn_file(
        super::types::SynFileRef::from(&valid),
        LockAcrossAwaitVisitor::default(),
    );
    assert_eq!(invalid_visitor.violations.len(), 1usize, "bbfce72c");
    assert!(valid_visitor.violations.is_empty(), "4b732bd1");
}

#[test]
fn production_code_does_not_use_explicit_leak_apis() {
    super::snapshot::with_codebase_snapshot(|snapshot| {
        let violations = snapshot
            .rs_files()
            .iter()
            .filter(|source_file| !source_file.path().as_ref().starts_with("tests"))
            .flat_map(|source_file| {
                let visitor = super::visit_syn_file(
                    super::types::SynFileRef::from(source_file.ast().as_ref()),
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
fn retained_spawn_handles_are_supervised() {
    super::snapshot::with_codebase_snapshot(|snapshot| {
        let violations = snapshot
            .rs_files()
            .iter()
            .flat_map(|source_file| {
                let visitor = super::visit_syn_file(
                    super::types::SynFileRef::from(source_file.ast().as_ref()),
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
fn spawn_lifecycle_policy_rejects_unconsumed_handles() {
    let ast = syn::parse_file(
        "async fn tasks() {
            let forgotten = tokio::spawn(async {});
            let awaited = tokio::spawn(async {});
            awaited.await;
            let transferred = tokio::spawn(async {});
            supervise(transferred);
        }",
    )
    .expect("834138af");
    let visitor = super::visit_syn_file(
        super::types::SynFileRef::from(&ast),
        SpawnLifecycleVisitor::default(),
    );
    assert_eq!(
        visitor.violations.as_slice(),
        [
            "spawn handle `forgotten` is retained but never awaited, aborted, or transferred to an owner"
        ],
        "a1680c46"
    );
}

#[test]
fn route_path_segments_use_snake_case() {
    super::snapshot::with_codebase_snapshot(|snapshot| {
        let violations = snapshot
            .rs_files()
            .iter()
            .flat_map(|source_file| {
                let visitor = super::visit_syn_file(
                    super::types::SynFileRef::from(source_file.ast().as_ref()),
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
    let ast = syn::parse_file(
        "#[typed_route(path = \"/admin/swagger_ui/{user_id}\")]
         struct Valid;
         #[typed_route(path = \"/admin/swagger-ui\")]
         struct Invalid;",
    )
    .expect("9aa037dc");
    let visitor = super::visit_syn_file(
        super::types::SynFileRef::from(&ast),
        RouteLiteralVisitor::default(),
    );
    assert_eq!(visitor.violations.len(), 1usize, "d15287e9");
}
