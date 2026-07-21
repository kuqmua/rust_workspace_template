mod cargo_policy;
mod ci_policy;
mod domain_type_policy;
mod lint_sync;
mod route_contract_policy;
mod runtime_policy;
mod snapshot;
mod source_policy;
mod types;
struct ExternalLeafWrapperNameException {
    identifier: types::StaticStr,
    reason: types::StaticStr,
}
#[derive(Debug, Clone, Copy, optml::Optml)]
enum ExpectOrPanic {
    Expect,
    Panic,
}
impl ExpectOrPanic {
    fn method_name(self) -> types::StaticStr {
        match self {
            Self::Expect => types::StaticStr::from(str_constants::CODE_STYLE_EXPECT_METHOD_NAME),
            Self::Panic => types::StaticStr::from(str_constants::CODE_STYLE_PANIC_METHOD_NAME),
        }
    }
}
#[derive(Debug, Clone, Copy, optml::Optml)]
enum RustOrClippy {
    Clippy,
    Rust,
}
impl RustOrClippy {
    fn name(self) -> types::StaticStr {
        match self {
            Self::Rust => types::StaticStr::from(str_constants::RUST),
            Self::Clippy => types::StaticStr::from(str_constants::CLIPPY),
        }
    }
}
struct DbgVisitor {
    found: types::AnalyzerBool,
}
impl<'ast> syn::visit::Visit<'ast> for DbgVisitor {
    fn visit_macro(&mut self, i: &'ast syn::Macro) {
        if i.path
            .segments
            .last()
            .is_some_and(|v_4b8e1c7a| v_4b8e1c7a.ident == str_constants::DBG)
        {
            self.found.set_true();
        }
    }
}
struct TodoUnimplVisitor {
    todo_found: types::AnalyzerCount,
    unimplemented_found: types::AnalyzerCount,
}
impl<'ast> syn::visit::Visit<'ast> for TodoUnimplVisitor {
    fn visit_macro(&mut self, i: &'ast syn::Macro) {
        if let Some(last_segment) = i.path.segments.last() {
            match () {
                () if last_segment.ident == str_constants::TODO => {
                    self.todo_found.saturating_inc();
                }
                () if last_segment.ident == str_constants::UNIMPLEMENTED => {
                    self.unimplemented_found.saturating_inc();
                }
                () => {}
            }
        }
    }
}
struct UnwrapVisitor {
    found_count: types::AnalyzerCount,
}
impl<'ast> syn::visit::Visit<'ast> for UnwrapVisitor {
    fn visit_expr_method_call(&mut self, i: &'ast syn::ExprMethodCall) {
        if i.method == str_constants::UNWRAP && i.args.is_empty() {
            self.found_count.saturating_inc();
        }
        syn::visit::visit_expr_method_call(self, i);
    }
}
struct ForLoopVisitor {
    found_count: types::AnalyzerCount,
}

#[derive(Default)]
struct SourceDroppingMapErrVisitor {
    found_count: types::AnalyzerCount,
}
impl<'ast> syn::visit::Visit<'ast> for SourceDroppingMapErrVisitor {
    fn visit_expr_method_call(&mut self, i: &'ast syn::ExprMethodCall) {
        if i.method == str_constants::CODE_STYLE_MAP_ERR
            && i.args.first().is_some_and(|argument| {
                matches!(argument, syn::Expr::Closure(closure) if closure.inputs.iter().any(|input| matches!(input, syn::Pat::Wild(_))))
            })
        {
            self.found_count.saturating_inc();
        }
        syn::visit::visit_expr_method_call(self, i);
    }
}

#[derive(Default)]
struct NumericAsCastVisitor {
    found_count: types::AnalyzerCount,
}

#[derive(Default)]
struct SerdeJsonValueFieldVisitor {
    violations: types::DiagnosticMsgs,
}
impl<'ast> syn::visit::Visit<'ast> for SerdeJsonValueFieldVisitor {
    fn visit_field(&mut self, i: &'ast syn::Field) {
        let mut type_visitor = SerdeJsonValueTypeVisitor::default();
        syn::visit::Visit::visit_type(&mut type_visitor, &i.ty);
        if type_visitor.found.get() {
            self.violations
                .push(str_constants::CODE_STYLE_UNNAMED_ITEM.to_owned());
        }
        syn::visit::visit_field(self, i);
    }
    fn visit_item_struct(&mut self, i: &'ast syn::ItemStruct) {
        if has_test_only_cfg_attr(types::SynItemRef::from(&syn::Item::Struct(i.clone()))).get()
            || i.ident == str_constants::CODE_STYLE_SERDE_JSON_ADMIN_AUDIT_DETAILS
        {
            return;
        }
        syn::visit::visit_item_struct(self, i);
    }
}

#[derive(Default)]
struct SerdeJsonValueTypeVisitor {
    found: types::AnalyzerBool,
}

#[derive(Default)]
struct PublicStructFieldVisitor {
    violations: types::DiagnosticMsgs,
}
impl<'ast> syn::visit::Visit<'ast> for PublicStructFieldVisitor {
    fn visit_field(&mut self, i: &'ast syn::Field) {
        if matches!(i.vis, syn::Visibility::Public(_)) {
            self.violations
                .push(str_constants::CODE_STYLE_UNNAMED_ITEM.to_owned());
        }
        syn::visit::visit_field(self, i);
    }
    fn visit_item_struct(&mut self, i: &'ast syn::ItemStruct) {
        if has_test_only_cfg_attr(types::SynItemRef::from(&syn::Item::Struct(i.clone()))).get() {
            return;
        }
        syn::visit::visit_item_struct(self, i);
    }
}
impl<'ast> syn::visit::Visit<'ast> for SerdeJsonValueTypeVisitor {
    fn visit_type_path(&mut self, i: &'ast syn::TypePath) {
        let mut segments = i.path.segments.iter().rev();
        if segments
            .next()
            .is_some_and(|segment| segment.ident == str_constants::CODE_STYLE_VALUE)
            && segments
                .next()
                .is_some_and(|segment| segment.ident == str_constants::SERDE_JSON)
        {
            self.found.set_true();
        }
        syn::visit::visit_type_path(self, i);
    }
}
impl<'ast> syn::visit::Visit<'ast> for NumericAsCastVisitor {
    fn visit_expr_cast(&mut self, i: &'ast syn::ExprCast) {
        let numeric = if let syn::Type::Path(path) = i.ty.as_ref() {
            if path.qself.is_none() {
                path.path.segments.last().is_some_and(|segment| {
                    matches!(
                        segment.ident.to_string().as_str(),
                        str_constants::CODE_STYLE_I8
                            | str_constants::CODE_STYLE_I16
                            | str_constants::CODE_STYLE_I32
                            | str_constants::CODE_STYLE_I64
                            | str_constants::CODE_STYLE_I128
                            | str_constants::CODE_STYLE_ISIZE
                            | str_constants::CODE_STYLE_U8
                            | str_constants::CODE_STYLE_U16
                            | str_constants::CODE_STYLE_U32
                            | str_constants::CODE_STYLE_U64
                            | str_constants::CODE_STYLE_U128
                            | str_constants::CODE_STYLE_USIZE
                            | str_constants::CODE_STYLE_F32
                            | str_constants::CODE_STYLE_F64
                    )
                })
            } else {
                false
            }
        } else {
            false
        };
        if numeric {
            self.found_count.saturating_inc();
        }
        syn::visit::visit_expr_cast(self, i);
    }
}
impl<'ast> syn::visit::Visit<'ast> for ForLoopVisitor {
    fn visit_expr_for_loop(&mut self, i: &'ast syn::ExprForLoop) {
        self.found_count.saturating_inc();
        syn::visit::visit_expr_for_loop(self, i);
    }
}
struct RuntimePanicExpectUnwrapVisitor {
    ers: types::DiagnosticMsgs,
}
impl<'ast> syn::visit::Visit<'ast> for RuntimePanicExpectUnwrapVisitor {
    fn visit_expr_method_call(&mut self, i: &'ast syn::ExprMethodCall) {
        if i.method == str_constants::CODE_STYLE_EXPECT_METHOD_NAME {
            self.ers.push(str_constants::EXPECT_CALL.to_owned());
        }
        if i.method == str_constants::UNWRAP {
            self.ers.push(str_constants::UNWRAP_CALL.to_owned());
        }
        syn::visit::visit_expr_method_call(self, i);
    }
    fn visit_item(&mut self, i: &'ast syn::Item) {
        if has_test_only_cfg_attr(types::SynItemRef::from(i)).get() {
            return;
        }
        syn::visit::visit_item(self, i);
    }
    fn visit_macro(&mut self, i: &'ast syn::Macro) {
        if i.path
            .segments
            .last()
            .is_some_and(|segment| segment.ident == str_constants::CODE_STYLE_PANIC_METHOD_NAME)
        {
            self.ers.push(str_constants::PANIC_CALL.to_owned());
        }
        syn::visit::visit_macro(self, i);
    }
}
struct RuntimeMutexVisitor {
    found_count: types::AnalyzerCount,
}
impl<'ast> syn::visit::Visit<'ast> for RuntimeMutexVisitor {
    fn visit_item(&mut self, i: &'ast syn::Item) {
        if has_test_only_cfg_attr(types::SynItemRef::from(i)).get() {
            return;
        }
        syn::visit::visit_item(self, i);
    }
    fn visit_type_path(&mut self, i: &'ast syn::TypePath) {
        if path_has_segment(
            types::SynPathRef::from(&i.path),
            types::SourceTextRef::from(str_constants::MUTEX),
        )
        .get()
        {
            self.found_count.saturating_inc();
        }
        syn::visit::visit_type_path(self, i);
    }
}
struct RuntimeArcVisitor {
    allow_arc_value_usage: types::AnalyzerBool,
    ers: types::DiagnosticMsgs,
}
impl<'ast> syn::visit::Visit<'ast> for RuntimeArcVisitor {
    fn visit_expr_call(&mut self, i: &'ast syn::ExprCall) {
        if expr_call_path(types::SynExprCallRef::from(i)).is_some_and(|path| {
            path_ends_with(
                path,
                types::StaticStrSliceRef::from([str_constants::ARC, str_constants::NEW].as_slice()),
            )
            .get()
        }) && !self.allow_arc_value_usage.get()
        {
            self.ers.push(
                str_constants::ARC_PATH_NEW_OUTSIDE_APPROVED_CROSS_THREAD_STATE_CONSTRUCTION
                    .to_owned(),
            );
        }
        syn::visit::visit_expr_call(self, i);
    }
    fn visit_item(&mut self, i: &'ast syn::Item) {
        if has_test_only_cfg_attr(types::SynItemRef::from(i)).get() {
            return;
        }
        syn::visit::visit_item(self, i);
    }
    fn visit_item_type(&mut self, i: &'ast syn::ItemType) {
        if type_contains_segment(
            types::SynTypeRef::from(&*i.ty),
            types::SourceTextRef::from(str_constants::ARC),
        )
        .get()
        {
            let name = i.ident.to_string();
            if !name.contains(str_constants::SHARED) && !name.contains(str_constants::DYNARC) {
                self.ers.push(format!(
                    "Arc type alias `{name}` must be explicitly named as shared cross-thread state"
                ));
            }
        }
        syn::visit::visit_item_type(self, i);
    }
}
struct AsyncBlockingCallVisitor {
    async_fn_depth: types::AnalyzerCount,
    ers: types::DiagnosticMsgs,
}
impl<'ast> syn::visit::Visit<'ast> for AsyncBlockingCallVisitor {
    fn visit_expr_call(&mut self, i: &'ast syn::ExprCall) {
        if self.async_fn_depth.get() != 0
            && expr_call_path(types::SynExprCallRef::from(i))
                .is_some_and(|path| path_is_blocking_async_call(path).get())
        {
            self.ers
                .push(str_constants::BLOCKING_CALL_INSIDE_ASYNC_FUNCTION.to_owned());
        }
        syn::visit::visit_expr_call(self, i);
    }
    fn visit_expr_method_call(&mut self, i: &'ast syn::ExprMethodCall) {
        if self.async_fn_depth.get() != 0
            && method_is_blocking_async_call(types::SourceTextRef::from(
                i.method.to_string().as_str(),
            ))
            .get()
        {
            self.ers.push(format!(
                ".{}() blocking method call inside async function",
                i.method
            ));
        }
        syn::visit::visit_expr_method_call(self, i);
    }
    fn visit_item(&mut self, i: &'ast syn::Item) {
        if has_test_only_cfg_attr(types::SynItemRef::from(i)).get() {
            return;
        }
        syn::visit::visit_item(self, i);
    }
    fn visit_item_fn(&mut self, i: &'ast syn::ItemFn) {
        let is_async = i.sig.asyncness.is_some();
        if is_async {
            self.async_fn_depth.saturating_inc();
        }
        syn::visit::visit_item_fn(self, i);
        if is_async {
            self.async_fn_depth.saturating_dec();
        }
    }
}
struct UnitTestExternalServiceVisitor {
    ers: types::DiagnosticMsgs,
    test_depth: types::AnalyzerCount,
}
impl<'ast> syn::visit::Visit<'ast> for UnitTestExternalServiceVisitor {
    fn visit_expr_path(&mut self, i: &'ast syn::ExprPath) {
        if self.test_depth.get() != 0
            && path_is_external_service_client(types::SynPathRef::from(&i.path)).get()
        {
            self.ers.push(format!(
                "unit tests must not depend on external service client `{}`",
                path_to_string(types::SynPathRef::from(&i.path)).as_ref()
            ));
        }
        syn::visit::visit_expr_path(self, i);
    }
    fn visit_item_fn(&mut self, i: &'ast syn::ItemFn) {
        let is_test =
            self.test_depth.get() != 0 || item_fn_is_unit_test(types::SynItemFnRef::from(i)).get();
        if is_test {
            self.test_depth.saturating_inc();
        }
        syn::visit::visit_item_fn(self, i);
        if is_test {
            self.test_depth.saturating_dec();
        }
    }
    fn visit_item_mod(&mut self, i: &'ast syn::ItemMod) {
        let is_test = self.test_depth.get() != 0
            || i.attrs
                .iter()
                .any(|attr| attr_is_test_only_cfg(types::SynAttributeRef::from(attr)).get());
        if is_test {
            self.test_depth.saturating_inc();
        }
        syn::visit::visit_item_mod(self, i);
        if is_test {
            self.test_depth.saturating_dec();
        }
    }
}
struct IncludeAssetMacroVisitor {
    ers: types::DiagnosticMsgs,
}
impl<'ast> syn::visit::Visit<'ast> for IncludeAssetMacroVisitor {
    fn visit_macro(&mut self, i: &'ast syn::Macro) {
        if let Some(segment) = i.path.segments.last()
            && (segment.ident == str_constants::INCLUDE_STR
                || segment.ident == str_constants::INCLUDE_BYTES)
        {
            self.ers.push(format!("contains {}!()", segment.ident));
        }
        syn::visit::visit_macro(self, i);
    }
}
struct DirectPathCallVisitor {
    calls: types::DiagnosticMsgs,
}
struct UnboundedReadVisitor {
    calls: types::DiagnosticMsgs,
}
impl<'ast> syn::visit::Visit<'ast> for UnboundedReadVisitor {
    fn visit_expr_call(&mut self, i: &'ast syn::ExprCall) {
        if let Some(path) = expr_call_path(types::SynExprCallRef::from(i)) {
            let call = path_to_string(path);
            if matches!(
                call.as_ref(),
                str_constants::STD_PATH_FS_PATH_READ
                    | str_constants::STD_PATH_FS_PATH_READ_TO_STRING
                    | str_constants::TOKIO_PATH_FS_PATH_READ
                    | str_constants::TOKIO_PATH_FS_PATH_READ_TO_STRING
            ) {
                self.calls.push(call.as_ref().to_owned());
            }
        }
        syn::visit::visit_expr_call(self, i);
    }
    fn visit_expr_method_call(&mut self, i: &'ast syn::ExprMethodCall) {
        if i.method == str_constants::PG_CRUD_PG_TEXT {
            self.calls.push(format!(".{}()", i.method));
        }
        syn::visit::visit_expr_method_call(self, i);
    }
}
impl<'ast> syn::visit::Visit<'ast> for DirectPathCallVisitor {
    fn visit_expr_call(&mut self, i: &'ast syn::ExprCall) {
        if let Some(path) = expr_call_path(types::SynExprCallRef::from(i)) {
            self.calls.push(path_to_string(path).as_ref().to_owned());
        }
        syn::visit::visit_expr_call(self, i);
    }
}
struct LostSpawnVisitor {
    ers: types::DiagnosticMsgs,
}
impl<'ast> syn::visit::Visit<'ast> for LostSpawnVisitor {
    fn visit_stmt(&mut self, i: &'ast syn::Stmt) {
        if let syn::Stmt::Expr(syn::Expr::Call(call), _) = i
            && expr_call_path(types::SynExprCallRef::from(call)).is_some_and(|path| {
                let text = path_to_string(path);
                matches!(
                    text.as_ref(),
                    str_constants::TOKIO_PATH_SPAWN
                        | str_constants::TOKIO_PATH_TASK_PATH_SPAWN_BLOCKING
                        | str_constants::STD_PATH_THREAD_PATH_SPAWN
                )
            })
        {
            self.ers.push(
                str_constants::SPAWN_RESULT_IS_DISCARDED_RETAIN_AND_SUPERVISE_ITS_HANDLE.to_owned(),
            );
        }
        syn::visit::visit_stmt(self, i);
    }
}
struct TestNondeterminismVisitor {
    calls: types::DiagnosticMsgs,
    test_depth: types::AnalyzerCount,
}
impl<'ast> syn::visit::Visit<'ast> for TestNondeterminismVisitor {
    fn visit_expr_call(&mut self, i: &'ast syn::ExprCall) {
        if self.test_depth.get() != 0
            && let Some(path) = expr_call_path(types::SynExprCallRef::from(i))
        {
            let text = path_to_string(path);
            if matches!(
                text.as_ref(),
                str_constants::RAND_PATH_THREAD_RANGE
                    | str_constants::STD_PATH_THREAD_PATH_SLEEP
                    | str_constants::STD_PATH_TIME_PATH_SYSTEMTIME_PATH_NOW
                    | str_constants::TOKIO_PATH_TIME_PATH_SLEEP
                    | str_constants::UUID_PATH_UUID_PATH_NEW_V4
            ) || text.as_ref().ends_with(str_constants::PATH_UTC_PATH_NOW)
            {
                self.calls.push(text.as_ref().to_owned());
            }
        }
        syn::visit::visit_expr_call(self, i);
    }
    fn visit_item_fn(&mut self, i: &'ast syn::ItemFn) {
        let is_test = item_fn_is_unit_test(types::SynItemFnRef::from(i)).get();
        if is_test {
            self.test_depth.saturating_inc();
        }
        syn::visit::visit_item_fn(self, i);
        if is_test {
            self.test_depth.saturating_dec();
        }
    }
    fn visit_item_mod(&mut self, i: &'ast syn::ItemMod) {
        let is_test = i
            .attrs
            .iter()
            .any(|attr| attr_is_test_only_cfg(types::SynAttributeRef::from(attr)).get());
        if is_test {
            self.test_depth.saturating_inc();
        }
        syn::visit::visit_item_mod(self, i);
        if is_test {
            self.test_depth.saturating_dec();
        }
    }
}
struct UseImportVisitor {
    found_non_public_use_import: types::AnalyzerBool,
    found_use_rename: types::AnalyzerBool,
    public_use_roots: types::SourceTextList,
}
impl UseImportVisitor {
    fn use_tree_contains_rename(use_tree: types::SynUseTreeRef<'_>) -> types::AnalyzerBool {
        types::AnalyzerBool::from(match use_tree.as_ref() {
            syn::UseTree::Path(use_path) => {
                Self::use_tree_contains_rename(types::SynUseTreeRef::from(&*use_path.tree)).get()
            }
            syn::UseTree::Name(_) | syn::UseTree::Glob(_) => false,
            syn::UseTree::Rename(_) => true,
            syn::UseTree::Group(use_group) => use_group
                .items
                .iter()
                .any(|item| Self::use_tree_contains_rename(types::SynUseTreeRef::from(item)).get()),
        })
    }
}
impl<'ast> syn::visit::Visit<'ast> for UseImportVisitor {
    fn visit_item_use(&mut self, i: &'ast syn::ItemUse) {
        if matches!(i.vis, syn::Visibility::Public(_)) {
            if let Some(root) = match &i.tree {
                syn::UseTree::Path(use_path) => Some(use_path.ident.to_string()),
                syn::UseTree::Rename(use_rename) => Some(use_rename.ident.to_string()),
                syn::UseTree::Name(use_name) => Some(use_name.ident.to_string()),
                syn::UseTree::Glob(_) | syn::UseTree::Group(_) => None,
            } {
                self.public_use_roots.push(root);
            } else {
                self.public_use_roots
                    .push(String::from(str_constants::ASTERISK));
            }
        } else {
            self.found_non_public_use_import.set_true();
        }
        if Self::use_tree_contains_rename(types::SynUseTreeRef::from(&i.tree)).get() {
            self.found_use_rename.set_true();
        }
        syn::visit::visit_item_use(self, i);
    }
}
struct TypeAliasVisitor {
    ers: types::DiagnosticMsgs,
}
impl<'ast> syn::visit::Visit<'ast> for TypeAliasVisitor {
    fn visit_item_type(&mut self, i: &'ast syn::ItemType) {
        self.ers.push(format!(
                "type alias `{}` found; use the explicit type at usage sites instead of creating a type alias",
                i.ident
            ));
        syn::visit::visit_item_type(self, i);
    }
}
struct ConstantAliasVisitor {
    ers: types::DiagnosticMsgs,
}
impl<'ast> syn::visit::Visit<'ast> for ConstantAliasVisitor {
    fn visit_item_const(&mut self, i: &'ast syn::ItemConst) {
        let local_constant_name = i.ident.to_string();
        if local_constant_name == str_constants::UNDERSCORE {
            return;
        }
        if let syn::Expr::Path(expression_path) = i.expr.as_ref()
            && expression_path.qself.is_none()
            && expression_path.path.segments.last().is_some_and(|segment| {
                let segment_identifier = segment.ident.to_string();
                segment_identifier
                    .chars()
                    .any(|symbol| symbol.is_ascii_alphabetic())
                    && segment_identifier.chars().all(|symbol| {
                        symbol.is_ascii_uppercase() || symbol.is_ascii_digit() || symbol == '_'
                    })
            })
        {
            self.ers.push(format!(
                "`{local_constant_name}` aliases `{}`; use the source constant directly",
                path_to_string(types::SynPathRef::from(&expression_path.path)).as_ref()
            ));
        }
        syn::visit::visit_item_const(self, i);
    }
}
struct TestStringLiteralVisitor {
    values: types::SourceTextList,
}
impl<'ast> syn::visit::Visit<'ast> for TestStringLiteralVisitor {
    fn visit_expr_lit(&mut self, i: &'ast syn::ExprLit) {
        if let syn::Lit::Str(literal_string) = &i.lit {
            self.values.push(literal_string.value());
        }
        syn::visit::visit_expr_lit(self, i);
    }
}
struct ProductionStringLiteralVisitor {
    values: types::SourceTextList,
}
impl<'ast> syn::visit::Visit<'ast> for ProductionStringLiteralVisitor {
    fn visit_expr_lit(&mut self, i: &'ast syn::ExprLit) {
        if let syn::Lit::Str(literal_string) = &i.lit {
            self.values.push(literal_string.value());
        }
        syn::visit::visit_expr_lit(self, i);
    }
    fn visit_item_fn(&mut self, i: &'ast syn::ItemFn) {
        if i.attrs.iter().any(|attribute| {
            attribute
                .path()
                .segments
                .last()
                .is_some_and(|segment| segment.ident == str_constants::TEST_ALT_3)
        }) {
            return;
        }
        syn::visit::visit_item_fn(self, i);
    }
    fn visit_item_mod(&mut self, i: &'ast syn::ItemMod) {
        if i.attrs.iter().any(|attribute| {
            matches!(
                &attribute.meta,
                syn::Meta::List(list)
                    if list.path.is_ident(str_constants::CFG_ALT)
                        && list
                            .tokens
                            .to_string()
                            .split(|symbol: char| !symbol.is_ascii_alphanumeric() && symbol != '_')
                            .any(|condition| condition == str_constants::TEST_ALT_3)
            )
        }) {
            return;
        }
        syn::visit::visit_item_mod(self, i);
    }
}
struct StringConstantVisitor {
    ers: types::DiagnosticMsgs,
}
impl<'ast> syn::visit::Visit<'ast> for StringConstantVisitor {
    fn visit_attribute(&mut self, _i: &'ast syn::Attribute) {}
    fn visit_expr_call(&mut self, i: &'ast syn::ExprCall) {
        if matches!(
            i.func.as_ref(),
            syn::Expr::Path(path)
                if path.path.segments.last().is_some_and(|segment| {
                    segment.ident == str_constants::COMPILE_ERROR_TOKEN_STREAM
                })
        ) {
            let mut literal_visitor = TestStringLiteralVisitor {
                values: types::SourceTextList::default(),
            };
            i.args.iter().for_each(|arg| {
                syn::visit::Visit::visit_expr(&mut literal_visitor, arg);
            });
            if !literal_visitor.values.is_empty() {
                self.ers.push(
                    str_constants::COMPILE_ERROR_TOKEN_STREAM_CALL_CONTAINS_STRING_LITERALS
                        .to_owned(),
                );
            }
        }
        syn::visit::visit_expr_call(self, i);
    }
    fn visit_expr_lit(&mut self, i: &'ast syn::ExprLit) {
        if let syn::Lit::Str(value) = &i.lit {
            let start = value.span().start();
            let end = value.span().end();
            self.ers.push(format!(
                "{}:{}-{}:{}: string literal {:?} must be declared in str_constants",
                start.line,
                start.column,
                end.line,
                end.column,
                value.value()
            ));
        }
        syn::visit::visit_expr_lit(self, i);
    }
    fn visit_expr_method_call(&mut self, i: &'ast syn::ExprMethodCall) {
        if i.method == str_constants::CODE_STYLE_EXPECT_METHOD_NAME {
            syn::visit::Visit::visit_expr(self, i.receiver.as_ref());
            return;
        }
        syn::visit::visit_expr_method_call(self, i);
    }
    fn visit_impl_item_const(&mut self, i: &'ast syn::ImplItemConst) {
        let mut literal_visitor = TestStringLiteralVisitor {
            values: types::SourceTextList::default(),
        };
        syn::visit::Visit::visit_expr(&mut literal_visitor, &i.expr);
        if !literal_visitor.values.is_empty() {
            self.ers.push(format!(
                "associated constant `{}` contains string literals",
                i.ident
            ));
        }
        syn::visit::visit_impl_item_const(self, i);
    }
    fn visit_impl_item_fn(&mut self, i: &'ast syn::ImplItemFn) {
        if i.sig.constness.is_some() {
            let mut literal_visitor = TestStringLiteralVisitor {
                values: types::SourceTextList::default(),
            };
            syn::visit::Visit::visit_block(&mut literal_visitor, &i.block);
            if !literal_visitor.values.is_empty() {
                self.ers.push(format!(
                    "const method `{}` contains string literals",
                    i.sig.ident
                ));
            }
        }
        syn::visit::visit_impl_item_fn(self, i);
    }
    fn visit_item_const(&mut self, i: &'ast syn::ItemConst) {
        let mut literal_visitor = TestStringLiteralVisitor {
            values: types::SourceTextList::default(),
        };
        syn::visit::Visit::visit_expr(&mut literal_visitor, i.expr.as_ref());
        if !literal_visitor.values.is_empty() {
            self.ers
                .push(format!("constant `{}` contains string literals", i.ident));
        }
        syn::visit::visit_item_const(self, i);
    }
    fn visit_item_fn(&mut self, i: &'ast syn::ItemFn) {
        if i.attrs.iter().any(|attribute| {
            attribute
                .path()
                .segments
                .last()
                .is_some_and(|segment| segment.ident == str_constants::TEST_ALT_3)
        }) {
            return;
        }
        if i.sig.constness.is_some() {
            let mut literal_visitor = TestStringLiteralVisitor {
                values: types::SourceTextList::default(),
            };
            syn::visit::Visit::visit_block(&mut literal_visitor, &i.block);
            if !literal_visitor.values.is_empty() {
                self.ers.push(format!(
                    "const function `{}` contains string literals",
                    i.sig.ident
                ));
            }
        }
        syn::visit::visit_item_fn(self, i);
    }
    fn visit_item_mod(&mut self, i: &'ast syn::ItemMod) {
        if i.attrs.iter().any(|attribute| {
            matches!(
                &attribute.meta,
                syn::Meta::List(list)
                    if list.path.is_ident(str_constants::CFG_ALT)
                        && list
                            .tokens
                            .to_string()
                            .split(|symbol: char| !symbol.is_ascii_alphanumeric() && symbol != '_')
                            .any(|condition| condition == str_constants::TEST_ALT_3)
            )
        }) {
            return;
        }
        syn::visit::visit_item_mod(self, i);
    }
    fn visit_item_static(&mut self, i: &'ast syn::ItemStatic) {
        let mut literal_visitor = TestStringLiteralVisitor {
            values: types::SourceTextList::default(),
        };
        syn::visit::Visit::visit_expr(&mut literal_visitor, i.expr.as_ref());
        if !literal_visitor.values.is_empty() {
            self.ers
                .push(format!("static `{}` contains string literals", i.ident));
        }
        syn::visit::visit_item_static(self, i);
    }
    fn visit_macro(&mut self, i: &'ast syn::Macro) {
        let is_syntax_boundary = i.path.segments.last().is_some_and(|segment| {
            str_constants::CODE_STYLE_STRING_LITERAL_MACRO_BOUNDARIES
                .contains(&segment.ident.to_string().as_str())
        });
        if !is_syntax_boundary {
            let mut streams = vec![i.tokens.clone()];
            let mut values = Vec::new();
            while let Some(stream) = streams.pop() {
                stream.into_iter().for_each(|token| match token {
                    proc_macro2::TokenTree::Group(group) => streams.push(group.stream()),
                    proc_macro2::TokenTree::Literal(literal) => {
                        if let Ok(value) = syn::parse_str::<syn::LitStr>(&literal.to_string()) {
                            let start = literal.span().start();
                            let end = literal.span().end();
                            values.push((
                                value.value(),
                                start.line,
                                start.column,
                                end.line,
                                end.column,
                            ));
                        }
                    }
                    proc_macro2::TokenTree::Ident(_) | proc_macro2::TokenTree::Punct(_) => {}
                });
            }
            let macro_name = i
                .path
                .segments
                .last()
                .map_or_else(String::new, |segment| segment.ident.to_string());
            while let Some((value, start_line, start_column, end_line, end_column)) = values.pop() {
                self.ers.push(format!(
                    "{start_line}:{start_column}-{end_line}:{end_column}: macro `{macro_name}` string literal {value:?} must be declared in str_constants"
                ));
            }
        }
        syn::visit::visit_macro(self, i);
    }
    fn visit_trait_item_const(&mut self, i: &'ast syn::TraitItemConst) {
        if let Some((_equals, expr)) = &i.default {
            let mut literal_visitor = TestStringLiteralVisitor {
                values: types::SourceTextList::default(),
            };
            syn::visit::Visit::visit_expr(&mut literal_visitor, expr);
            if !literal_visitor.values.is_empty() {
                self.ers.push(format!(
                    "trait constant `{}` contains string literals",
                    i.ident
                ));
            }
        }
        syn::visit::visit_trait_item_const(self, i);
    }
    fn visit_trait_item_fn(&mut self, i: &'ast syn::TraitItemFn) {
        if i.sig.constness.is_some()
            && let Some(block) = &i.default
        {
            let mut literal_visitor = TestStringLiteralVisitor {
                values: types::SourceTextList::default(),
            };
            syn::visit::Visit::visit_block(&mut literal_visitor, block);
            if !literal_visitor.values.is_empty() {
                self.ers.push(format!(
                    "const trait method `{}` contains string literals",
                    i.sig.ident
                ));
            }
        }
        syn::visit::visit_trait_item_fn(self, i);
    }
}
struct StringWrapperNameVisitor {
    names: types::StdSourceTextSet,
}
impl<'ast> syn::visit::Visit<'ast> for StringWrapperNameVisitor {
    fn visit_item_struct(&mut self, i: &'ast syn::ItemStruct) {
        if item_struct_is_single_string_wrapper(types::SynItemStructRef::from(i)).get() {
            let _: bool = self.names.insert(i.ident.to_string());
        }
        syn::visit::visit_item_struct(self, i);
    }
}
struct StringWrapperFromVisitor<'names_lt> {
    ers: types::DiagnosticMsgs,
    len_checked_function_names: &'names_lt types::StdSourceTextSet,
    string_wrapper_names: &'names_lt types::StdSourceTextSet,
    try_from_string_len_checked_names: types::StdSourceTextSet,
    try_from_string_names: types::StdSourceTextSet,
}
impl StringWrapperFromVisitor<'_> {
    fn check_bounded_string_attr(&mut self, item: types::SynItemStructRef<'_>) {
        let item_ref = item.as_ref();
        if !item_struct_is_single_string_wrapper(item).get() {
            return;
        }
        let has_derive = item_ref
            .attrs
            .iter()
            .any(|attr| attr_has_bounded_string_derive(types::SynAttributeRef::from(attr)).get());
        let has_max_bound = item_ref.attrs.iter().any(|attr| {
            attr_has_bounded_string_max_bound(types::SynAttributeRef::from(attr)).get()
        });
        if has_derive && has_max_bound {
            let identifier = item_ref.ident.to_string();
            let _: bool = self.try_from_string_names.insert(identifier.clone());
            let _: bool = self.try_from_string_len_checked_names.insert(identifier);
        }
    }
    fn check_from_impl(&mut self, item: types::SynItemImplRef<'_>) {
        if !item_impl_is_from_string(item).get() {
            return;
        }
        let identifier = item_impl_self_ty_identifier(item).map_or_else(
            || String::from(str_constants::NON_PATH_TARGET),
            String::from,
        );
        self.ers.push(format!(
            "`{identifier}` implements `From<String>`; implement `TryFrom<String>` instead"
        ));
    }
    fn check_newtype_attr(&mut self, item: types::SynItemStructRef<'_>) {
        let item_ref = item.as_ref();
        if !item_struct_is_single_string_wrapper(item).get() {
            return;
        }
        if item_ref
            .attrs
            .iter()
            .any(|attr| attr_has_newtype_from_option(types::SynAttributeRef::from(attr)).get())
        {
            self.ers.push(format!(
                        "string wrapper `{}` derives `newtype::FromInner`; derive `newtype::TryFrom` with a length check instead",
                        item_ref.ident
                    ));
        }
        let has_try_from = item_ref.attrs.iter().any(|attr| {
            attr.path().is_ident(str_constants::DERIVE)
                && attr.meta.require_list().is_ok_and(|list| {
                    list.tokens
                        .to_string()
                        .contains(str_constants::NEWTYPE_TRY_FROM_DERIVE_NAME)
                })
        });
        let mut has_len_check = false;
        item_ref
            .attrs
            .iter()
            .filter(|attr| attr.path().is_ident(str_constants::NEWTYPE_TRY_FROM))
            .for_each(|attr| {
                drop(attr.parse_nested_meta(|meta| {
                    if meta.path.is_ident(str_constants::NEWTYPE_TRY_FROM_ERROR) {
                        let _error_type = meta.value()?.parse::<syn::Type>()?;
                        return Ok(());
                    }
                    if meta
                        .path
                        .is_ident(str_constants::NEWTYPE_TRY_FROM_VALIDATOR)
                    {
                        let expr = meta.value()?.parse::<syn::Expr>()?;
                        let mut visitor = LenMethodCallVisitor {
                            found: types::AnalyzerBool::default(),
                        };
                        syn::visit::Visit::visit_expr(&mut visitor, &expr);
                        let path_is_len_checked = match &expr {
                            syn::Expr::Path(path) => {
                                path.path.segments.last().is_some_and(|segment| {
                                    self.len_checked_function_names
                                        .contains(segment.ident.to_string().as_str())
                                })
                            }
                            syn::Expr::Array(_)
                            | syn::Expr::Assign(_)
                            | syn::Expr::Async(_)
                            | syn::Expr::Await(_)
                            | syn::Expr::Binary(_)
                            | syn::Expr::Block(_)
                            | syn::Expr::Break(_)
                            | syn::Expr::Call(_)
                            | syn::Expr::Cast(_)
                            | syn::Expr::Closure(_)
                            | syn::Expr::Const(_)
                            | syn::Expr::Continue(_)
                            | syn::Expr::Field(_)
                            | syn::Expr::ForLoop(_)
                            | syn::Expr::Group(_)
                            | syn::Expr::If(_)
                            | syn::Expr::Index(_)
                            | syn::Expr::Infer(_)
                            | syn::Expr::Let(_)
                            | syn::Expr::Lit(_)
                            | syn::Expr::Loop(_)
                            | syn::Expr::Macro(_)
                            | syn::Expr::Match(_)
                            | syn::Expr::MethodCall(_)
                            | syn::Expr::Paren(_)
                            | syn::Expr::Range(_)
                            | syn::Expr::RawAddr(_)
                            | syn::Expr::Reference(_)
                            | syn::Expr::Repeat(_)
                            | syn::Expr::Return(_)
                            | syn::Expr::Struct(_)
                            | syn::Expr::Try(_)
                            | syn::Expr::TryBlock(_)
                            | syn::Expr::Tuple(_)
                            | syn::Expr::Unary(_)
                            | syn::Expr::Unsafe(_)
                            | syn::Expr::Verbatim(_)
                            | syn::Expr::While(_)
                            | syn::Expr::Yield(_)
                            | _ => false,
                        };
                        if visitor.found.get() || path_is_len_checked {
                            has_len_check = true;
                        }
                    }
                    Ok(())
                }));
            });
        if has_try_from {
            let identifier = item_ref.ident.to_string();
            let _: bool = self.try_from_string_names.insert(identifier.clone());
            if has_len_check {
                let _: bool = self.try_from_string_len_checked_names.insert(identifier);
            }
        }
    }
    fn check_try_from_impl(&mut self, item: types::SynItemImplRef<'_>) {
        if !item_impl_is_try_from_string(item).get() {
            return;
        }
        let Some(identifier) = item_impl_self_ty_identifier(item) else {
            return;
        };
        if !self.string_wrapper_names.contains(identifier.as_ref()) {
            return;
        }
        let _: bool = self
            .try_from_string_names
            .insert(String::from(identifier.clone()));
        if item_impl_contains_len_call(item).get() {
            let _: bool = self
                .try_from_string_len_checked_names
                .insert(String::from(identifier));
        }
    }
}
struct LenCheckedFunctionNameVisitor {
    names: types::StdSourceTextSet,
}
impl<'ast> syn::visit::Visit<'ast> for LenCheckedFunctionNameVisitor {
    fn visit_item_fn(&mut self, i: &'ast syn::ItemFn) {
        let mut visitor = LenMethodCallVisitor {
            found: types::AnalyzerBool::default(),
        };
        syn::visit::Visit::visit_block(&mut visitor, &i.block);
        if visitor.found.get() {
            let _: bool = self.names.insert(i.sig.ident.to_string());
        }
        syn::visit::visit_item_fn(self, i);
    }
}
impl<'ast> syn::visit::Visit<'ast> for StringWrapperFromVisitor<'_> {
    fn visit_item_impl(&mut self, i: &'ast syn::ItemImpl) {
        self.check_from_impl(types::SynItemImplRef::from(i));
        self.check_try_from_impl(types::SynItemImplRef::from(i));
        syn::visit::visit_item_impl(self, i);
    }
    fn visit_item_struct(&mut self, i: &'ast syn::ItemStruct) {
        self.check_bounded_string_attr(types::SynItemStructRef::from(i));
        self.check_newtype_attr(types::SynItemStructRef::from(i));
        syn::visit::visit_item_struct(self, i);
    }
}
struct LenMethodCallVisitor {
    found: types::AnalyzerBool,
}
impl<'ast> syn::visit::Visit<'ast> for LenMethodCallVisitor {
    fn visit_expr_method_call(&mut self, i: &'ast syn::ExprMethodCall) {
        if i.method == str_constants::LEN {
            self.found.set_true();
        }
        syn::visit::visit_expr_method_call(self, i);
    }
}
struct PublicTupleWrapperFieldVisitor {
    ers: types::DiagnosticMsgs,
}
struct DirectDeserializeTupleWrapperVisitor {
    ers: types::DiagnosticMsgs,
}
struct DeserializeConversionCallVisitor {
    found: types::AnalyzerBool,
}
struct ManualDeserializeTupleWrapperVisitor<'names> {
    ers: types::DiagnosticMsgs,
    names: &'names types::StdSourceTextSet,
}
struct TupleWrapperConversionCollector {
    converted_names: types::StdSourceTextSet,
    names: types::StdSourceTextSet,
}
struct DirectTupleWrapperConstructorVisitor<'names> {
    current_wrapper_name: Option<String>,
    ers: types::DiagnosticMsgs,
    inside_conversion_impl: types::AnalyzerBool,
    names: &'names types::StdSourceTextSet,
}
impl<'ast> syn::visit::Visit<'ast> for PublicTupleWrapperFieldVisitor {
    fn visit_item_struct(&mut self, i: &'ast syn::ItemStruct) {
        if item_struct_is_single_field_tuple_wrapper(types::SynItemStructRef::from(i)).get()
            && item_struct_single_field_is_non_private(types::SynItemStructRef::from(i)).get()
        {
            self.ers.push(format!(
                "tuple wrapper `{}` exposes its inner field; make the field private and initialize through From/TryFrom",
                i.ident
                    ));
        }
        syn::visit::visit_item_struct(self, i);
    }
}
impl<'ast> syn::visit::Visit<'ast> for DirectDeserializeTupleWrapperVisitor {
    fn visit_item_struct(&mut self, i: &'ast syn::ItemStruct) {
        if item_struct_is_single_field_tuple_wrapper(types::SynItemStructRef::from(i)).get()
            && item_struct_derives_deserialize(types::SynItemStructRef::from(i)).get()
            && !item_struct_deserialize_uses_conversion(types::SynItemStructRef::from(i)).get()
        {
            let start = syn::spanned::Spanned::span(i).start();
            self.ers.push(format!(
                "tuple wrapper `{}` derives Deserialize directly at {}:{}; this lets serde construct the inner field without using the wrapper's required conversion path and can bypass validation or other construction invariants. Deserialize a raw value through `#[serde(from = \"RawType\")]` or `#[serde(try_from = \"RawType\")]` so construction finishes in From/TryFrom",
                i.ident, start.line, start.column
            ));
        }
        syn::visit::visit_item_struct(self, i);
    }
}
impl<'ast> syn::visit::Visit<'ast> for DeserializeConversionCallVisitor {
    fn visit_expr_call(&mut self, i: &'ast syn::ExprCall) {
        if let syn::Expr::Path(path) = i.func.as_ref()
            && path.path.segments.last().is_some_and(|segment| {
                segment.ident == str_constants::FROM_ALT_4
                    || segment.ident == str_constants::NEWTYPE_TRY_FROM
            })
        {
            self.found.set_true();
        }
        syn::visit::visit_expr_call(self, i);
    }
}
impl<'ast> syn::visit::Visit<'ast> for ManualDeserializeTupleWrapperVisitor<'_> {
    fn visit_item_impl(&mut self, i: &'ast syn::ItemImpl) {
        let is_deserialize_impl = i.trait_.as_ref().is_some_and(|(_, path, _)| {
            path.segments.last().is_some_and(|segment| {
                segment.ident == str_constants::CODE_STYLE_DESERIALIZE_DERIVE_NAME
            })
        });
        let Some(name) = item_impl_self_ty_identifier(types::SynItemImplRef::from(i)) else {
            syn::visit::visit_item_impl(self, i);
            return;
        };
        if is_deserialize_impl && self.names.contains(name.as_ref()) {
            let mut visitor = DeserializeConversionCallVisitor {
                found: types::AnalyzerBool::default(),
            };
            syn::visit::visit_item_impl(&mut visitor, i);
            if !visitor.found.get() {
                let start = syn::spanned::Spanned::span(i).start();
                self.ers.push(format!(
                    "tuple wrapper `{}` implements Deserialize without an explicit From/TryFrom call at {}:{}; returning the wrapper through another construction path can bypass validation or other invariants. Deserialize into a raw type, then finish with `Self::from(raw)` or `Self::try_from(raw)` and map conversion errors with `serde::de::Error::custom`",
                    name.as_ref(), start.line, start.column
                ));
            }
        }
        syn::visit::visit_item_impl(self, i);
    }
}
impl<'ast> syn::visit::Visit<'ast> for TupleWrapperConversionCollector {
    fn visit_item_impl(&mut self, i: &'ast syn::ItemImpl) {
        if item_impl_is_from_or_try_from(types::SynItemImplRef::from(i)).get()
            && let Some(name) = item_impl_self_ty_identifier(types::SynItemImplRef::from(i))
        {
            let _: bool = self.converted_names.insert(name.as_ref().to_owned());
        }
        syn::visit::visit_item_impl(self, i);
    }
    fn visit_item_struct(&mut self, i: &'ast syn::ItemStruct) {
        if item_struct_is_single_field_tuple_wrapper(types::SynItemStructRef::from(i)).get() {
            let name = i.ident.to_string();
            let _: bool = self.names.insert(name.clone());
            if item_struct_derives_conversion(types::SynItemStructRef::from(i)).get() {
                let _: bool = self.converted_names.insert(name);
            }
        }
        syn::visit::visit_item_struct(self, i);
    }
}
impl<'ast> syn::visit::Visit<'ast> for DirectTupleWrapperConstructorVisitor<'_> {
    fn visit_expr_call(&mut self, i: &'ast syn::ExprCall) {
        if !self.inside_conversion_impl.get()
            && let syn::Expr::Path(path) = i.func.as_ref()
            && let Some(segment) = path.path.segments.last()
            && (self.names.contains(segment.ident.to_string().as_str())
                || (segment.ident == str_constants::SELF && self.current_wrapper_name.is_some()))
        {
            let span = syn::spanned::Spanned::span(i.func.as_ref());
            let start = span.start();
            let end = span.end();
            let wrapper_name = self
                .current_wrapper_name
                .as_deref()
                .filter(|_| segment.ident == str_constants::SELF)
                .map_or_else(|| segment.ident.to_string(), str::to_owned);
            self.ers.push(format!(
                "tuple wrapper `{}` is initialized directly at {}:{}-{}:{}; use From/TryFrom",
                wrapper_name, start.line, start.column, end.line, end.column
            ));
        }
        syn::visit::visit_expr_call(self, i);
    }
    fn visit_item_impl(&mut self, i: &'ast syn::ItemImpl) {
        let previous = self.inside_conversion_impl;
        let previous_wrapper_name = self.current_wrapper_name.take();
        self.inside_conversion_impl = item_impl_is_from_or_try_from(types::SynItemImplRef::from(i));
        self.current_wrapper_name = item_impl_self_ty_identifier(types::SynItemImplRef::from(i))
            .map(|name| name.as_ref().to_owned())
            .filter(|name| self.names.contains(name));
        syn::visit::visit_item_impl(self, i);
        self.inside_conversion_impl = previous;
        self.current_wrapper_name = previous_wrapper_name;
    }
}
struct DeclaredDomainTypeVisitor {
    names: types::StdSourceTextSet,
}
impl<'ast> syn::visit::Visit<'ast> for DeclaredDomainTypeVisitor {
    fn visit_item(&mut self, i: &'ast syn::Item) {
        if has_test_only_cfg_attr(types::SynItemRef::from(i)).get() {
            return;
        }
        syn::visit::visit_item(self, i);
    }
    fn visit_item_enum(&mut self, i: &'ast syn::ItemEnum) {
        let _: bool = self.names.insert(i.ident.to_string());
        syn::visit::visit_item_enum(self, i);
    }
    fn visit_item_struct(&mut self, i: &'ast syn::ItemStruct) {
        let _: bool = self.names.insert(i.ident.to_string());
        syn::visit::visit_item_struct(self, i);
    }
    fn visit_item_trait(&mut self, i: &'ast syn::ItemTrait) {
        let _: bool = self.names.insert(i.ident.to_string());
        syn::visit::visit_item_trait(self, i);
    }
    fn visit_item_union(&mut self, i: &'ast syn::ItemUnion) {
        let _: bool = self.names.insert(i.ident.to_string());
        syn::visit::visit_item_union(self, i);
    }
    fn visit_macro(&mut self, i: &'ast syn::Macro) {
        if path_ends_with(
            types::SynPathRef::from(&i.path),
            types::StaticStrSliceRef::from(
                [
                    str_constants::CODE_STYLE_GENERATE_PG_TYPES_MACRO_NAME,
                    str_constants::CODE_STYLE_GENERATE_PG_TYPES_MACRO_NAME,
                ]
                .as_slice(),
            ),
        )
        .get()
        {
            collect_generate_pg_types_domain_names(
                types::SourceTextRef::from(i.tokens.to_string().as_str()),
                &mut self.names,
            );
        }
        if config_lib_domain_type_macro_path(types::SynPathRef::from(&i.path)).get() {
            collect_first_macro_identifier_domain_name(
                types::SourceTextRef::from(i.tokens.to_string().as_str()),
                &mut self.names,
            );
        }
        if path_ends_with(
            types::SynPathRef::from(&i.path),
            types::StaticStrSliceRef::from([str_constants::BOOL_ENUM_TO_TOKENS].as_slice()),
        )
        .get()
        {
            collect_first_macro_identifier_domain_name(
                types::SourceTextRef::from(i.tokens.to_string().as_str()),
                &mut self.names,
            );
        }
        if path_ends_with(
            types::SynPathRef::from(&i.path),
            types::StaticStrSliceRef::from(
                [
                    str_constants::CODE_STYLE_GENERATE_DERIVE_TOKEN_STREAM_BUILDER_MACRO_NAME,
                    str_constants::CODE_STYLE_GENERATE_DERIVE_TOKEN_STREAM_BUILDER_MACRO_NAME,
                ]
                .as_slice(),
            ),
        )
        .get()
        {
            let _: bool = self
                .names
                .insert(String::from(str_constants::DTOKENSTREAMBUILDER));
        }
        syn::visit::visit_macro(self, i);
    }
}
struct DomainTypePolicyVisitor<'types> {
    closure_body_scan_depth: types::AnalyzerCount,
    ers: types::DiagnosticMsgs,
    generic_scopes: Vec<types::StdSourceTextSet>,
    repo_crates: types::StdStdSourceTextSetRef<'types>,
    repo_types: types::StdStdSourceTextSetRef<'types>,
}
struct AnalyzerStateRawContainerFieldVisitor {
    ers: types::DiagnosticMsgs,
}
struct HelperRawTextReturnVisitor {
    ers: types::DiagnosticMsgs,
}
struct ExternalLeafWrapperNameVisitor<'types> {
    ers: types::DiagnosticMsgs,
    repo_crates: types::StdStdSourceTextSetRef<'types>,
}
impl DomainTypePolicyVisitor<'_> {
    fn check_fields(
        &mut self,
        fields: types::SynFieldsRef<'_>,
        ctx: types::SourceTextRef<'_>,
        allow_single_newtype_raw: types::AnalyzerBool,
    ) {
        let fields_ref = fields.as_ref();
        if allow_single_newtype_raw.get()
            && matches!(fields_ref, syn::Fields::Unnamed(unnamed_fields) if unnamed_fields.unnamed.len() == 1)
        {
            return;
        }
        fields_ref
            .iter()
            .for_each(|field| self.check_ty(types::SynTypeRef::from(&field.ty), ctx));
    }
    fn check_path_arguments(
        &mut self,
        arguments: types::SynPathArgumentsRef<'_>,
        ctx: types::SourceTextRef<'_>,
    ) {
        match arguments.as_ref() {
            syn::PathArguments::AngleBracketed(args) => {
                args.args
                    .iter()
                    .filter_map(|arg| match arg {
                        syn::GenericArgument::Type(ty) => Some(ty),
                        syn::GenericArgument::AssocConst(_)
                        | syn::GenericArgument::AssocType(_)
                        | syn::GenericArgument::Constraint(_)
                        | syn::GenericArgument::Const(_)
                        | syn::GenericArgument::Lifetime(_)
                        | _ => None,
                    })
                    .for_each(|ty| self.check_ty(types::SynTypeRef::from(ty), ctx));
            }
            syn::PathArguments::Parenthesized(args) => {
                args.inputs
                    .iter()
                    .for_each(|ty| self.check_ty(types::SynTypeRef::from(ty), ctx));
                match &args.output {
                    syn::ReturnType::Default => {}
                    syn::ReturnType::Type(_, ty) => {
                        self.check_ty(types::SynTypeRef::from(&**ty), ctx);
                    }
                }
            }
            syn::PathArguments::None => {}
        }
    }
    fn check_sig(&mut self, sig: types::SynSignatureRef<'_>, ctx: types::SourceTextRef<'_>) {
        let sig_ref = sig.as_ref();
        self.push_generics(types::SynGenericsRef::from(&sig_ref.generics));
        sig_ref
            .inputs
            .iter()
            .filter_map(|input| match input {
                syn::FnArg::Receiver(_) => None,
                syn::FnArg::Typed(pat_ty) => Some(pat_ty),
            })
            .for_each(|pat_ty| {
                self.check_ty(
                    types::SynTypeRef::from(&*pat_ty.ty),
                    types::SourceTextRef::from(format!("{} parameter", ctx.as_ref()).as_str()),
                );
            });
        match &sig_ref.output {
            syn::ReturnType::Default => {}
            syn::ReturnType::Type(_, ty) => {
                self.check_ty(
                    types::SynTypeRef::from(&**ty),
                    types::SourceTextRef::from(format!("{} return type", ctx.as_ref()).as_str()),
                );
            }
        }
        self.pop_generics();
    }
    fn check_ty(&mut self, ty: types::SynTypeRef<'_>, ctx: types::SourceTextRef<'_>) {
        match ty.as_ref() {
            syn::Type::Array(ty_array) => {
                self.check_ty(types::SynTypeRef::from(&*ty_array.elem), ctx);
            }
            syn::Type::Group(ty_group) => {
                self.check_ty(types::SynTypeRef::from(&*ty_group.elem), ctx);
            }
            syn::Type::Paren(ty_paren) => {
                self.check_ty(types::SynTypeRef::from(&*ty_paren.elem), ctx);
            }
            syn::Type::Path(ty_path) => {
                self.check_ty_path(types::SynTypePathRef::from(ty_path), ctx);
            }
            syn::Type::Reference(ty_reference) => {
                self.check_ty(types::SynTypeRef::from(&*ty_reference.elem), ctx);
            }
            syn::Type::Slice(ty_slice) => {
                self.check_ty(types::SynTypeRef::from(&*ty_slice.elem), ctx);
            }
            syn::Type::Tuple(ty_tuple) => {
                ty_tuple
                    .elems
                    .iter()
                    .for_each(|elem| self.check_ty(types::SynTypeRef::from(elem), ctx));
            }
            syn::Type::BareFn(_)
            | syn::Type::ImplTrait(_)
            | syn::Type::Infer(_)
            | syn::Type::Macro(_)
            | syn::Type::Never(_)
            | syn::Type::Ptr(_)
            | syn::Type::TraitObject(_)
            | syn::Type::Verbatim(_)
            | _ => {}
        }
    }
    fn check_ty_path(&mut self, ty_path: types::SynTypePathRef<'_>, ctx: types::SourceTextRef<'_>) {
        let ty_path_ref = ty_path.as_ref();
        if let Some(qself) = &ty_path_ref.qself {
            self.check_ty(types::SynTypeRef::from(&*qself.ty), ctx);
            ty_path_ref.path.segments.iter().for_each(|segment| {
                self.check_path_arguments(
                    types::SynPathArgumentsRef::from(&segment.arguments),
                    ctx,
                );
            });
            return;
        }
        let Some(segment) = ty_path_ref.path.segments.last() else {
            return;
        };
        let identifier = segment.ident.to_string();
        if path_first_segment_is_self(types::SynPathRef::from(&ty_path_ref.path)).get() {
            self.check_path_arguments(types::SynPathArgumentsRef::from(&segment.arguments), ctx);
            return;
        }
        if is_structural_generic_container(types::SourceTextRef::from(identifier.as_str())).get() {
            self.check_path_arguments(types::SynPathArgumentsRef::from(&segment.arguments), ctx);
            return;
        }
        if self
            .is_allowed_type_identifier(types::SourceTextRef::from(identifier.as_str()))
            .get()
        {
            self.check_path_arguments(types::SynPathArgumentsRef::from(&segment.arguments), ctx);
            return;
        }
        if self
            .path_starts_with_allowed_type_identifier(types::SynPathRef::from(&ty_path_ref.path))
            .get()
        {
            ty_path_ref.path.segments.iter().for_each(|path_segment| {
                self.check_path_arguments(
                    types::SynPathArgumentsRef::from(&path_segment.arguments),
                    ctx,
                );
            });
            return;
        }
        if self
            .path_starts_with_repo_crate(types::SynPathRef::from(&ty_path_ref.path))
            .get()
        {
            ty_path_ref.path.segments.iter().for_each(|path_segment| {
                self.check_path_arguments(
                    types::SynPathArgumentsRef::from(&path_segment.arguments),
                    ctx,
                );
            });
            return;
        }
        if self
            .path_starts_with_external_crate(types::SynPathRef::from(&ty_path_ref.path))
            .get()
        {
            self.ers.push(format!(
                "{} uses `{}`; use a repository domain wrapper type and initialize it with From/TryFrom instead of exposing raw external or primitive types",
                ctx.as_ref(),
                path_to_string(types::SynPathRef::from(&ty_path_ref.path)).as_ref()
            ));
            self.check_path_arguments(types::SynPathArgumentsRef::from(&segment.arguments), ctx);
            return;
        }
        self.ers.push(format!(
                "{} uses `{}`; use a repository domain wrapper type and initialize it with From/TryFrom instead of exposing raw external or primitive types",
                ctx.as_ref(),
                path_to_string(types::SynPathRef::from(&ty_path_ref.path)).as_ref()
            ));
        self.check_path_arguments(types::SynPathArgumentsRef::from(&segment.arguments), ctx);
    }
    fn closure_body_scan_is_active(&self) -> types::AnalyzerBool {
        types::AnalyzerBool::from(self.closure_body_scan_depth.get() > 0)
    }
    fn is_allowed_type_identifier(
        &self,
        identifier: types::SourceTextRef<'_>,
    ) -> types::AnalyzerBool {
        let identifier_ref = identifier.as_ref();
        types::AnalyzerBool::from(
            identifier_ref == str_constants::SELF
                || self.repo_types.as_ref().contains(identifier_ref)
                || self
                    .generic_scopes
                    .iter()
                    .rev()
                    .any(|scope| scope.contains(identifier_ref)),
        )
    }
    fn path_starts_with_allowed_type_identifier(
        &self,
        path: types::SynPathRef<'_>,
    ) -> types::AnalyzerBool {
        let path_ref = path.as_ref();
        types::AnalyzerBool::from(
            path_ref.segments.len() > 1
                && path_ref.segments.first().is_some_and(|segment| {
                    self.is_allowed_type_identifier(types::SourceTextRef::from(
                        segment.ident.to_string().as_str(),
                    ))
                    .get()
                }),
        )
    }
    fn path_starts_with_external_crate(&self, path: types::SynPathRef<'_>) -> types::AnalyzerBool {
        let path_ref = path.as_ref();
        types::AnalyzerBool::from(
            path_ref.segments.len() > 1
                && path_ref.segments.first().is_some_and(|segment| {
                    let identifier = segment.ident.to_string();
                    identifier != str_constants::CRATE
                        && identifier != str_constants::SELF_ALT
                        && identifier != str_constants::SUPER
                        && !self.repo_crates.as_ref().contains(&identifier)
                        && !self
                            .is_allowed_type_identifier(types::SourceTextRef::from(
                                identifier.as_str(),
                            ))
                            .get()
                }),
        )
    }
    fn path_starts_with_repo_crate(&self, path: types::SynPathRef<'_>) -> types::AnalyzerBool {
        let path_ref = path.as_ref();
        types::AnalyzerBool::from(
            path_ref.segments.len() > 1
                && path_ref.segments.first().is_some_and(|segment| {
                    let identifier = segment.ident.to_string();
                    self.repo_crates.as_ref().contains(&identifier)
                }),
        )
    }
    fn pop_generics(&mut self) {
        let popped = self.generic_scopes.pop();
        assert!(popped.is_some(), "1cb23b63");
    }
    fn push_generics(&mut self, generics: types::SynGenericsRef<'_>) {
        let mut names = std::collections::BTreeSet::new();
        names.extend(
            generics
                .as_ref()
                .params
                .iter()
                .filter_map(|param| match param {
                    syn::GenericParam::Type(type_param) => Some(type_param.ident.to_string()),
                    syn::GenericParam::Const(_) | syn::GenericParam::Lifetime(_) => None,
                }),
        );
        self.generic_scopes
            .push(types::StdSourceTextSet::from(names));
    }
    fn scan_block_for_closure_inputs(&mut self, block: types::SynBlockRef<'_>) {
        self.closure_body_scan_depth.saturating_inc();
        syn::visit::visit_block(self, block.as_ref());
        self.closure_body_scan_depth.saturating_dec();
    }
}
impl<'ast> syn::visit::Visit<'ast> for DomainTypePolicyVisitor<'_> {
    fn visit_expr_closure(&mut self, i: &'ast syn::ExprClosure) {
        i.inputs.iter().for_each(|input| {
            if let syn::Pat::Type(pat_ty) = input {
                self.check_ty(
                    types::SynTypeRef::from(&*pat_ty.ty),
                    types::SourceTextRef::from(str_constants::CLOSURE_PARAMETER),
                );
            }
        });
        syn::visit::visit_expr_closure(self, i);
    }
    fn visit_item(&mut self, i: &'ast syn::Item) {
        if has_test_only_cfg_attr(types::SynItemRef::from(i)).get() {
            return;
        }
        if self.closure_body_scan_is_active().get() {
            return;
        }
        syn::visit::visit_item(self, i);
    }
    fn visit_item_enum(&mut self, i: &'ast syn::ItemEnum) {
        if identifier_is_diagnostic_try_from_string_error(types::SynIdentifierRef::from(&i.ident))
            .get()
        {
            return;
        }
        self.push_generics(types::SynGenericsRef::from(&i.generics));
        i.variants.iter().for_each(|variant| {
            self.check_fields(
                types::SynFieldsRef::from(&variant.fields),
                types::SourceTextRef::from(format!("enum `{}` variant", i.ident).as_str()),
                types::AnalyzerBool::default(),
            );
        });
        self.pop_generics();
    }
    fn visit_item_fn(&mut self, i: &'ast syn::ItemFn) {
        if item_fn_is_proc_macro(types::SynItemFnRef::from(i)).get() {
            return;
        }
        self.check_sig(
            types::SynSignatureRef::from(&i.sig),
            types::SourceTextRef::from(format!("function `{}`", i.sig.ident).as_str()),
        );
        self.scan_block_for_closure_inputs(types::SynBlockRef::from(&*i.block));
    }
    fn visit_item_impl(&mut self, i: &'ast syn::ItemImpl) {
        if i.trait_.is_some() {
            return;
        }
        self.push_generics(types::SynGenericsRef::from(&i.generics));
        i.items
            .iter()
            .filter_map(|item| match item {
                syn::ImplItem::Fn(item_fn)
                    if !attrs_contain_test_only_cfg(types::SynAttributeListRef::from(
                        item_fn.attrs.as_slice(),
                    ))
                    .get() =>
                {
                    if method_is_explicit_wrapper_accessor(types::SynIdentifierRef::from(
                        &item_fn.sig.ident,
                    ))
                    .get()
                    {
                        None
                    } else {
                        Some(item_fn)
                    }
                }
                syn::ImplItem::Const(_)
                | syn::ImplItem::Macro(_)
                | syn::ImplItem::Type(_)
                | syn::ImplItem::Verbatim(_)
                | _ => None,
            })
            .for_each(|item_fn| {
                self.check_sig(
                    types::SynSignatureRef::from(&item_fn.sig),
                    types::SourceTextRef::from(format!("method `{}`", item_fn.sig.ident).as_str()),
                );
            });
        i.items
            .iter()
            .filter_map(|item| match item {
                syn::ImplItem::Fn(item_fn)
                    if !attrs_contain_test_only_cfg(types::SynAttributeListRef::from(
                        item_fn.attrs.as_slice(),
                    ))
                    .get() =>
                {
                    Some(item_fn)
                }
                syn::ImplItem::Const(_)
                | syn::ImplItem::Macro(_)
                | syn::ImplItem::Type(_)
                | syn::ImplItem::Verbatim(_)
                | _ => None,
            })
            .for_each(|item_fn| {
                self.scan_block_for_closure_inputs(types::SynBlockRef::from(&item_fn.block));
            });
        self.pop_generics();
    }
    fn visit_item_struct(&mut self, i: &'ast syn::ItemStruct) {
        self.push_generics(types::SynGenericsRef::from(&i.generics));
        self.check_fields(
            types::SynFieldsRef::from(&i.fields),
            types::SourceTextRef::from(format!("struct `{}` field", i.ident).as_str()),
            types::AnalyzerBool::from(true),
        );
        self.pop_generics();
    }
    fn visit_item_trait(&mut self, i: &'ast syn::ItemTrait) {
        self.push_generics(types::SynGenericsRef::from(&i.generics));
        i.items
            .iter()
            .filter_map(|item| match item {
                syn::TraitItem::Fn(item_fn)
                    if !attrs_contain_test_only_cfg(types::SynAttributeListRef::from(
                        item_fn.attrs.as_slice(),
                    ))
                    .get() =>
                {
                    Some(item_fn)
                }
                syn::TraitItem::Const(_)
                | syn::TraitItem::Macro(_)
                | syn::TraitItem::Type(_)
                | syn::TraitItem::Verbatim(_)
                | _ => None,
            })
            .for_each(|item_fn| {
                self.check_sig(
                    types::SynSignatureRef::from(&item_fn.sig),
                    types::SourceTextRef::from(
                        format!("trait method `{}`", item_fn.sig.ident).as_str(),
                    ),
                );
            });
        self.pop_generics();
    }
}
impl AnalyzerStateRawContainerFieldVisitor {
    fn check_fields(&mut self, item: types::SynItemStructRef<'_>) {
        let item_ref = item.as_ref();
        item_ref.fields.iter().for_each(|field| {
            if let Some((raw_ty, wrapper_ty)) =
                analyzer_state_raw_container_ty(types::SynTypeRef::from(&field.ty))
            {
                let field_name = field
                    .ident
                    .as_ref()
                    .map_or_else(|| String::from(str_constants::TUPLE), ToString::to_string);
                self.ers.push(format!(
                    "struct `{}` field `{}` uses `{}`; use `{}`",
                    item_ref.ident,
                    field_name,
                    raw_ty.get(),
                    wrapper_ty.get()
                ));
            }
        });
    }
}
impl<'ast> syn::visit::Visit<'ast> for AnalyzerStateRawContainerFieldVisitor {
    fn visit_item_struct(&mut self, i: &'ast syn::ItemStruct) {
        if item_struct_is_single_field_tuple_wrapper(types::SynItemStructRef::from(i)).get() {
            return;
        }
        self.check_fields(types::SynItemStructRef::from(i));
        syn::visit::visit_item_struct(self, i);
    }
}
impl HelperRawTextReturnVisitor {
    fn check_sig(&mut self, sig: types::SynSignatureRef<'_>, ctx: types::SourceTextRef<'_>) {
        let syn::ReturnType::Type(_, ty) = &sig.as_ref().output else {
            return;
        };
        if let Some((raw_ty, wrapper_ty)) = raw_text_return_ty(types::SynTypeRef::from(&**ty)) {
            self.ers.push(format!(
                "{} return type uses `{}`; use `{}`",
                ctx.as_ref(),
                raw_ty.get(),
                wrapper_ty.get()
            ));
        }
    }
}
impl<'ast> syn::visit::Visit<'ast> for HelperRawTextReturnVisitor {
    fn visit_item_fn(&mut self, i: &'ast syn::ItemFn) {
        if item_fn_is_proc_macro(types::SynItemFnRef::from(i)).get() {
            return;
        }
        self.check_sig(
            types::SynSignatureRef::from(&i.sig),
            types::SourceTextRef::from(format!("function `{}`", i.sig.ident).as_str()),
        );
        syn::visit::visit_item_fn(self, i);
    }
    fn visit_item_impl(&mut self, i: &'ast syn::ItemImpl) {
        if i.trait_.is_some() {
            return;
        }
        i.items
            .iter()
            .filter_map(|item| match item {
                syn::ImplItem::Fn(item_fn)
                    if !method_is_explicit_wrapper_accessor(types::SynIdentifierRef::from(
                        &item_fn.sig.ident,
                    ))
                    .get() =>
                {
                    Some(item_fn)
                }
                syn::ImplItem::Const(_)
                | syn::ImplItem::Macro(_)
                | syn::ImplItem::Type(_)
                | syn::ImplItem::Verbatim(_)
                | _ => None,
            })
            .for_each(|item_fn| {
                self.check_sig(
                    types::SynSignatureRef::from(&item_fn.sig),
                    types::SourceTextRef::from(format!("method `{}`", item_fn.sig.ident).as_str()),
                );
            });
    }
}
impl<'ast> syn::visit::Visit<'ast> for ExternalLeafWrapperNameVisitor<'_> {
    fn visit_item_struct(&mut self, i: &'ast syn::ItemStruct) {
        if attrs_contain_test_only_cfg(types::SynAttributeListRef::from(i.attrs.as_slice())).get() {
            return;
        }
        let syn::Fields::Unnamed(fields) = &i.fields else {
            syn::visit::visit_item_struct(self, i);
            return;
        };
        if fields.unnamed.len() != 1 {
            syn::visit::visit_item_struct(self, i);
            return;
        }
        let Some(field) = fields.unnamed.first() else {
            syn::visit::visit_item_struct(self, i);
            return;
        };
        self.check_external_leaf_wrapper_name(
            types::SynItemStructRef::from(i),
            types::SynTypeRef::from(&field.ty),
        );
        syn::visit::visit_item_struct(self, i);
    }
}
impl ExternalLeafWrapperNameVisitor<'_> {
    fn check_external_leaf_wrapper_name(
        &mut self,
        item: types::SynItemStructRef<'_>,
        ty: types::SynTypeRef<'_>,
    ) {
        let Some(first_segment) = self.external_root_segment(ty) else {
            return;
        };
        let first_segment_ref = first_segment.get();
        let item_ref = item.as_ref();
        let expected_prefix = identifier_to_upper_camel_fragment(types::SynIdentifierRef::from(
            &first_segment_ref.ident,
        ));
        let identifier = item_ref.ident.to_string();
        if is_external_leaf_wrapper_name_exception(types::SourceTextRef::from(identifier.as_str()))
            .get()
        {
            return;
        }
        if identifier.starts_with(expected_prefix.as_ref()) {
            return;
        }
        self.ers.push(format!(
            "tuple wrapper `{}` wraps external crate `{}`; rename it so it starts with `{}`",
            item_ref.ident,
            first_segment_ref.ident,
            expected_prefix.as_ref()
        ));
    }
    fn external_root_segment<'ty_lt>(
        &self,
        ty: types::SynTypeRef<'ty_lt>,
    ) -> Option<types::SynPathSegmentRef<'ty_lt>> {
        match ty.get() {
            syn::Type::Array(ty_array) => {
                self.external_root_segment(types::SynTypeRef::from(&*ty_array.elem))
            }
            syn::Type::Group(ty_group) => {
                self.external_root_segment(types::SynTypeRef::from(&*ty_group.elem))
            }
            syn::Type::Paren(ty_paren) => {
                self.external_root_segment(types::SynTypeRef::from(&*ty_paren.elem))
            }
            syn::Type::Path(ty_path) => {
                self.external_root_segment_from_path(types::SynTypePathRef::from(ty_path))
            }
            syn::Type::Reference(ty_reference) => {
                self.external_root_segment(types::SynTypeRef::from(&*ty_reference.elem))
            }
            syn::Type::Slice(ty_slice) => {
                self.external_root_segment(types::SynTypeRef::from(&*ty_slice.elem))
            }
            syn::Type::Tuple(ty_tuple) => ty_tuple
                .elems
                .iter()
                .find_map(|elem| self.external_root_segment(types::SynTypeRef::from(elem))),
            syn::Type::BareFn(_)
            | syn::Type::ImplTrait(_)
            | syn::Type::Infer(_)
            | syn::Type::Macro(_)
            | syn::Type::Never(_)
            | syn::Type::Ptr(_)
            | syn::Type::TraitObject(_)
            | syn::Type::Verbatim(_)
            | _ => None,
        }
    }
    fn external_root_segment_from_arguments<'args_lt>(
        &self,
        arguments: types::SynPathArgumentsRef<'args_lt>,
    ) -> Option<types::SynPathSegmentRef<'args_lt>> {
        match arguments.get() {
            syn::PathArguments::AngleBracketed(args) => {
                args.args.iter().find_map(|arg| match arg {
                    syn::GenericArgument::Type(ty) => {
                        self.external_root_segment(types::SynTypeRef::from(ty))
                    }
                    syn::GenericArgument::AssocConst(_)
                    | syn::GenericArgument::AssocType(_)
                    | syn::GenericArgument::Constraint(_)
                    | syn::GenericArgument::Const(_)
                    | syn::GenericArgument::Lifetime(_)
                    | _ => None,
                })
            }
            syn::PathArguments::Parenthesized(args) => args
                .inputs
                .iter()
                .find_map(|ty| self.external_root_segment(types::SynTypeRef::from(ty)))
                .or_else(|| match &args.output {
                    syn::ReturnType::Default => None,
                    syn::ReturnType::Type(_, ty) => {
                        self.external_root_segment(types::SynTypeRef::from(&**ty))
                    }
                }),
            syn::PathArguments::None => None,
        }
    }
    fn external_root_segment_from_path<'path_lt>(
        &self,
        ty_path: types::SynTypePathRef<'path_lt>,
    ) -> Option<types::SynPathSegmentRef<'path_lt>> {
        let ty_path_ref = ty_path.get();
        if let Some(qself) = &ty_path_ref.qself {
            return self.external_root_segment(types::SynTypeRef::from(&*qself.ty));
        }
        let first_segment = ty_path_ref.path.segments.first()?;
        let first_identifier = first_segment.ident.to_string();
        if first_identifier == str_constants::CRATE
            || first_identifier == str_constants::SELF_ALT
            || first_identifier == str_constants::SUPER
            || self.repo_crates.as_ref().contains(&first_identifier)
        {
            return ty_path_ref.path.segments.iter().find_map(|segment| {
                self.external_root_segment_from_arguments(types::SynPathArgumentsRef::from(
                    &segment.arguments,
                ))
            });
        }
        if ty_path_ref.path.segments.len() > 1 {
            return Some(types::SynPathSegmentRef::from(first_segment));
        }
        ty_path_ref.path.segments.iter().find_map(|segment| {
            self.external_root_segment_from_arguments(types::SynPathArgumentsRef::from(
                &segment.arguments,
            ))
        })
    }
}
#[allow(clippy::single_call_fn)] // named exception source keeps policy data separate from its validation loop
fn external_leaf_wrapper_name_exceptions() -> [ExternalLeafWrapperNameException; 1] {
    [ExternalLeafWrapperNameException {
        identifier: types::StaticStr::from(
            str_constants::CODE_STYLE_GENERATED_RUST_TOKEN_STREAM_IDENTIFIER,
        ),
        reason: types::StaticStr::from(
            str_constants::CODE_STYLE_GENERATED_RUST_TOKEN_STREAM_REASON,
        ),
    }]
}
#[allow(clippy::single_call_fn)] // validates every external wrapper naming exception has an explicit reason before matching idents
fn is_external_leaf_wrapper_name_exception(
    identifier: types::SourceTextRef<'_>,
) -> types::AnalyzerBool {
    types::AnalyzerBool::from(
        external_leaf_wrapper_name_exceptions()
            .iter()
            .any(|exception| {
                assert!(!exception.reason.get().is_empty(), "c7ab0f62");
                exception.identifier.get() == identifier.as_ref()
            }),
    )
}
fn check_expect_or_panic_contains_only_unique_uuid_v4(expect_or_panic: ExpectOrPanic) {
    struct ExpectVisitor {
        ers: types::DiagnosticMsgs,
        method_name: types::StaticStr,
        uuids: types::SourceTextList,
    }
    impl<'ast> syn::visit::Visit<'ast> for ExpectVisitor {
        fn visit_expr_method_call(&mut self, i: &'ast syn::ExprMethodCall) {
            if i.method == self.method_name.get() {
                if i.args.len() == 1 {
                    if let Some(syn::Expr::Lit(syn::ExprLit {
                        lit: syn::Lit::Str(lit_str),
                        ..
                    })) = i.args.first()
                    {
                        let v = lit_str.value();
                        if v.len() == 8 {
                            self.uuids.push(v);
                        } else {
                            self.ers.push(format!("arg len is not 8: {v}"));
                        }
                    } else {
                        self.ers
                            .push(str_constants::ARG_IS_NOT_STRING_LITERAL.to_owned());
                    }
                } else {
                    self.ers
                        .push(str_constants::WITH_NOT_EQUALS_1_ARG.to_owned());
                }
            }
            syn::visit::visit_expr_method_call(self, i);
        }
    }
    let mut all_uuids = Vec::new();
    let mut all_ers = Vec::new();
    for_each_rs_syn_file(|path, ast| {
        let visitor = visit_syn_file(
            types::SynFileRef::from(ast),
            ExpectVisitor {
                method_name: expect_or_panic.method_name(),
                uuids: types::SourceTextList::default(),
                ers: types::DiagnosticMsgs::default(),
            },
        );
        all_uuids.extend(visitor.uuids);
        all_ers.extend(
            visitor
                .ers
                .into_iter()
                .map(|element_2b9891bd| format!("{path:?}: {element_2b9891bd}")),
        );
    });
    let duplicates = find_duplicate_strings(types::SourceTextListRef::from(all_uuids.as_slice()));
    if !duplicates.is_empty() {
        all_ers.push(format!("duplicate UUIDs found: {duplicates:?}"));
    }
    assert!(all_ers.is_empty(), "6062a9e9 {all_ers:#?}");
}
#[allow(clippy::single_call_fn)] // shared lint-compare wrapper keeps clippy/rust lint test flow aligned and reduces duplicate wiring
fn assert_workspace_lints_match(
    rust_or_clippy: RustOrClippy,
    tool: types::StaticStr,
    parse_only_clippy: types::AnalyzerBool,
    exp_id: types::StaticStr,
    exceptions: types::StaticStrSliceRef<'_>,
) {
    let lints_vec_from_cargo_toml = lints_vec_from_cargo_toml_workspace(rust_or_clippy);
    let lints_from_cmd = lints_from_help_cmd(tool, parse_only_clippy, exp_id);
    compare_lints_vecs(
        rust_or_clippy,
        types::SourceTextListRef::from(lints_vec_from_cargo_toml.as_slice()),
        types::SourceTextListRef::from(lints_from_cmd.as_slice()),
        exceptions,
    );
}
#[allow(clippy::single_call_fn)] // helper intentionally stays extracted so command parsing remains decoupled from lint comparison orchestration
fn lints_from_help_cmd(
    tool: types::StaticStr,
    parse_only_clippy: types::AnalyzerBool,
    exp_id: types::StaticStr,
) -> types::SourceTextList {
    let output = macros_helpers::tool_command::ToolCommand::new(
        macros_helpers::tool_command::ToolProgramRef::from(tool.get()),
    )
    .args(macros_helpers::tool_command::ToolArgsRef::from(
        [str_constants::W, str_constants::HELP].as_slice(),
    ))
    .output()
    .unwrap_or_else(|_| panic!("{}", exp_id.get()));
    assert_cmd_output_ok(
        types::StdProcessOutputRef::from(output.as_ref()),
        types::StaticStr::from(str_constants::VALUE_95D4595A),
        types::StaticStr::from(str_constants::CC4670A2),
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    let regex = if parse_only_clippy.get() {
        regex::Regex::new(str_constants::QUESTION_M_S_ASTERISK_CLIPPY_PATH_A_Z0_9_A_Z0_9)
            .expect("fbf14346")
    } else {
        regex::Regex::new(str_constants::QUESTION_M_S_ASTERISK_A_Z0_9_A_Z0_9_PLUS_S)
            .expect("60d99c87")
    };
    regex
        .captures_iter(&stdout)
        .map(|element_70833f93| {
            String::from(normalize_lint_name(types::SourceTextRef::from(
                &element_70833f93[1],
            )))
        })
        .collect::<Vec<String>>()
        .into()
}
#[allow(clippy::single_call_fn)] // shared command-output assertions keep status/stderr checks reusable for command-driven tests
fn assert_cmd_output_ok(
    output: types::StdProcessOutputRef<'_>,
    status_exp_id: types::StaticStr,
    stderr_exp_id: types::StaticStr,
) {
    assert!(output.as_ref().status.success(), "{}", status_exp_id.get());
    let stderr = String::from_utf8_lossy(&output.as_ref().stderr);
    assert!(stderr.trim().is_empty(), "{}", stderr_exp_id.get());
}
#[allow(clippy::single_call_fn)] // centralizes lint-name normalization used by command output parsing
fn normalize_lint_name(v: types::SourceTextRef<'_>) -> types::SourceText {
    types::SourceText::try_from(v.as_ref().replace('-', str_constants::UNDERSCORE))
        .expect("f3d821a6")
}
#[allow(clippy::single_call_fn)] // keeps workspace-dependency shape checks reusable and focused in one helper
fn validate_workspace_dep_spec(v: types::TomlValueRef<'_>) {
    let v_table = toml_val_as_table_ref(v, types::StaticStr::from(str_constants::CB693A3F));
    if let Some(path_v) = v_table.get().get(str_constants::PATH_ALT_5) {
        match path_v {
            toml::Value::String(_) => {
                validate_workspace_path_dep_version(v_table);
                match v_table.get().len() {
                    2 => (),
                    3 => validate_workspace_dep_default_features(v_table),
                    _ => panic!("f6a3b9d1 {v_table:#?}"),
                }
                return;
            }
            toml::Value::Table(_)
            | toml::Value::Integer(_)
            | toml::Value::Float(_)
            | toml::Value::Boolean(_)
            | toml::Value::Datetime(_)
            | toml::Value::Array(_) => panic!("6ca03a1f"),
        }
    }
    validate_workspace_dep_version(v_table);
    match v_table.get().len() {
        1 => {}
        2 => validate_workspace_dep_features_or_default_features(v_table),
        3 => {
            validate_workspace_dep_features(v_table);
            match v_table
                .get()
                .get(str_constants::DEFAULT_FEATURES)
                .expect("847a138f")
            {
                &toml::Value::Boolean(_) => (),
                &toml::Value::String(_)
                | &toml::Value::Table(_)
                | &toml::Value::Integer(_)
                | &toml::Value::Float(_)
                | &toml::Value::Datetime(_)
                | &toml::Value::Array(_) => panic!("b320164b"),
            }
        }
        _ => panic!("f1139378 {v_table:#?}"),
    }
}
#[allow(clippy::single_call_fn)] // path workspace deps must keep concrete package versions for external tooling policy checks
fn validate_workspace_path_dep_version(v_table: types::TomlTableRef<'_>) {
    match v_table
        .get()
        .get(str_constants::VERSION_ALT_3)
        .expect("bf2e4a7c")
    {
        toml::Value::String(version_string) => {
            assert_eq!(version_string, "0.1.0", "8c3d5f91");
        }
        toml::Value::Table(_)
        | toml::Value::Integer(_)
        | toml::Value::Float(_)
        | toml::Value::Boolean(_)
        | toml::Value::Datetime(_)
        | toml::Value::Array(_) => panic!("a6c7e3d2"),
    }
}
#[allow(clippy::single_call_fn)] // keeps two-key dependency tables strict while allowing featureless default-features opt-out
fn validate_workspace_dep_features_or_default_features(v_table: types::TomlTableRef<'_>) {
    if v_table.get().contains_key(str_constants::FEATURES_ALT) {
        validate_workspace_dep_features(v_table);
    } else {
        validate_workspace_dep_default_features(v_table);
    }
}
#[allow(clippy::single_call_fn)] // shared shape check for dependency tables that explicitly opt out of default features
fn validate_workspace_dep_default_features(v_table: types::TomlTableRef<'_>) {
    match v_table
        .get()
        .get(str_constants::DEFAULT_FEATURES)
        .expect("d2a8c4e1")
    {
        &toml::Value::Boolean(_) => (),
        &toml::Value::String(_)
        | &toml::Value::Table(_)
        | &toml::Value::Integer(_)
        | &toml::Value::Float(_)
        | &toml::Value::Datetime(_)
        | &toml::Value::Array(_) => panic!("e5f7b1c3"),
    }
}
#[allow(clippy::single_call_fn)] // separates version shape assertion from dependency-table flow and keeps IDs stable
fn validate_workspace_dep_version(v_table: types::TomlTableRef<'_>) {
    match v_table
        .get()
        .get(str_constants::VERSION_ALT_3)
        .expect("d5b2b269")
    {
        toml::Value::String(version_string) => {
            assert!(
                is_exact_three_part_version(types::SourceTextRef::from(version_string.as_str()))
                    .get(),
                "6640b9bf"
            );
        }
        toml::Value::Table(_)
        | toml::Value::Integer(_)
        | toml::Value::Float(_)
        | toml::Value::Boolean(_)
        | toml::Value::Datetime(_)
        | toml::Value::Array(_) => panic!("a3410a37"),
    }
}
#[allow(clippy::single_call_fn)] // extracted to avoid repeated feature-type checks for dependency tables
fn validate_workspace_dep_features(v_table: types::TomlTableRef<'_>) {
    match v_table
        .get()
        .get(str_constants::FEATURES_ALT)
        .expect("473577d5")
    {
        &toml::Value::Array(_) => (),
        &toml::Value::String(_)
        | &toml::Value::Table(_)
        | &toml::Value::Integer(_)
        | &toml::Value::Float(_)
        | &toml::Value::Boolean(_)
        | &toml::Value::Datetime(_) => panic!("38ba32e9"),
    }
}
#[allow(clippy::single_call_fn)] // isolates exact-version parsing so version-format checks are reusable and testable
fn is_exact_three_part_version(v: types::SourceTextRef<'_>) -> types::AnalyzerBool {
    let Some(rest) = v.as_ref().strip_prefix('=') else {
        return types::AnalyzerBool::default();
    };
    let mut iter = rest.split('.');
    types::AnalyzerBool::from(
        (0..3).all(|_| {
            iter.next()
                .and_then(|part| part.parse::<u64>().ok())
                .is_some()
        }) && iter.next().is_none(),
    )
}
#[allow(clippy::single_call_fn)] // helper intentionally stays extracted so lint diff logic remains reusable and independently readable
fn compare_lints_vecs(
    rust_or_clippy: RustOrClippy,
    lints_vec_from_cargo_toml: types::SourceTextListRef<'_>,
    lints_to_check: types::SourceTextListRef<'_>,
    lints_not_in_cargo_toml_vec_exceptions: types::StaticStrSliceRef<'_>,
) {
    let rust_or_clippy_name = rust_or_clippy.name().get();
    let lints_from_cargo_set = str_set(lints_vec_from_cargo_toml);
    let lints_to_check_set = str_set(lints_to_check);
    let lints_exceptions_set = lints_not_in_cargo_toml_vec_exceptions
        .get()
        .iter()
        .copied()
        .collect::<std::collections::HashSet<&str>>();
    let (lints_not_in_cargo_toml, lints_missing_by_exception) = split_lints_missing_from_cargo(
        lints_to_check,
        types::StdSourceTextRefSet::from(lints_from_cargo_set.as_ref()),
        types::StdSourceTextRefSet::from(&lints_exceptions_set),
    );
    let missing_by_exception_message = lints_missing_by_exception
        .into_iter()
        .map(|lint| {
            format!("todo!() {rust_or_clippy_name} {lint} 158b5c43-05fa-4b8f-b6fe-9cda49d26997")
        })
        .collect::<Vec<String>>()
        .join(str_constants::NEWLINE);
    if !missing_by_exception_message.is_empty() {
        println!("{missing_by_exception_message}");
    }
    assert!(
        lints_not_in_cargo_toml.is_empty(),
        "d2b7ba9f {lints_not_in_cargo_toml:?}"
    );
    let outdated_lints_in_file = collect_missing_items(
        lints_vec_from_cargo_toml,
        types::StdSourceTextRefSet::from(lints_to_check_set.as_ref()),
    );
    assert!(outdated_lints_in_file.is_empty(), "93787d2d");
}
#[allow(clippy::single_call_fn)] // shared parser keeps .env line-to-key extraction reusable and test behavior centralized
fn parse_env_key_line(line: types::SourceTextRef<'_>) -> Option<types::SourceTextRef<'_>> {
    let trimmed = line.get().trim();
    if trimmed.is_empty() || trimmed.starts_with('#') {
        return None;
    }
    trimmed
        .split_once('=')
        .map(|(key, _)| types::SourceTextRef::from(key))
}
fn env_keys_from_file(path: types::StaticStr) -> types::SourceTextList {
    std::fs::read_to_string(path.get())
        .expect("b3a7c1e4")
        .lines()
        .filter_map(|line| parse_env_key_line(types::SourceTextRef::from(line)))
        .map(|key| key.as_ref().to_owned())
        .collect::<Vec<String>>()
        .into()
}
#[allow(clippy::single_call_fn)] // shared set-difference collector keeps missing-item checks reusable across lint and env-key tests
fn collect_missing_items(
    items: types::SourceTextListRef<'_>,
    present_set: types::StdSourceTextRefSet<'_>,
) -> types::SourceTextList {
    types::SourceTextList::from(
        items
            .get()
            .iter()
            .map(String::as_str)
            .filter(|item| !present_set.as_ref().contains(item))
            .map(str::to_owned)
            .collect::<Vec<String>>(),
    )
}
#[allow(clippy::single_call_fn)] // centralized formatter keeps env key mismatch diagnostics consistent
fn collect_missing_key_ers(
    source_keys: types::SourceTextListRef<'_>,
    target_set: types::StdSourceTextRefSet<'_>,
    source_file: types::StaticStr,
    target_file: types::StaticStr,
) -> types::SourceTextList {
    types::SourceTextList::from(
        collect_missing_items(source_keys, target_set)
            .into_iter()
            .map(|key| {
                format!(
                    "key `{key}` in {} but missing from {}",
                    source_file.get(),
                    target_file.get()
                )
            })
            .collect::<Vec<String>>(),
    )
}
#[allow(clippy::single_call_fn)] // split keeps lint exception handling explicit while reusing missing-item collection
fn split_lints_missing_from_cargo(
    lints_to_check: types::SourceTextListRef<'_>,
    lints_from_cargo_set: types::StdSourceTextRefSet<'_>,
    lints_exceptions_set: types::StdSourceTextRefSet<'_>,
) -> (types::SourceTextList, types::SourceTextList) {
    let (lints_missing_by_exception, lints_not_in_cargo_toml) =
        collect_missing_items(lints_to_check, lints_from_cargo_set)
            .into_iter()
            .partition::<Vec<String>, _>(|lint| {
                lints_exceptions_set.as_ref().contains(lint.as_str())
            });
    (
        types::SourceTextList::from(lints_not_in_cargo_toml),
        types::SourceTextList::from(lints_missing_by_exception),
    )
}
#[allow(clippy::single_call_fn)] // helper intentionally stays extracted so workspace-lints table parsing remains separate from test driver wiring
fn lints_vec_from_cargo_toml_workspace(rust_or_clippy: RustOrClippy) -> types::SourceTextList {
    let workspace = workspace_table_from_cargo_toml();
    let lints = toml_val_as_table_ref(
        types::TomlValueRef::from(
            workspace
                .as_ref()
                .get(str_constants::LINTS)
                .expect("82eaea37"),
        ),
        types::StaticStr::from(str_constants::CAE226CD),
    );
    let toml_v_table = toml_val_as_table_ref(
        types::TomlValueRef::from(
            lints
                .as_ref()
                .get(rust_or_clippy.name().get())
                .expect("dbd02f72"),
        ),
        types::StaticStr::from(str_constants::VALUE_6F4580CE),
    );
    toml_v_table
        .as_ref()
        .keys()
        .cloned()
        .collect::<Vec<String>>()
        .into()
}
#[allow(clippy::single_call_fn)] // reusable collector stays split from assertion helper for callsites that need raw error vectors
fn collect_cargo_toml_ers(
    mut mk_ers: impl FnMut(&std::path::Path, &toml::Table, &mut Vec<String>),
) -> types::SourceTextList {
    let mut ers = Vec::new();
    for_each_crate_manifest_file(|path| {
        let Some(parsed) = read_toml_table(types::StdPathRef::from(path)) else {
            return;
        };
        mk_ers(path, parsed.as_ref(), &mut ers);
    });
    types::SourceTextList::from(ers)
}
#[allow(clippy::single_call_fn)] // centralizes repeated cargo-toml assertion shape used by multiple tests
fn assert_cargo_toml_ers_empty(
    exp_id: types::StaticStr,
    mut mk_ers: impl FnMut(&std::path::Path, &toml::Table, &mut Vec<String>),
) {
    let ers = collect_cargo_toml_ers(|path, parsed, ers| {
        mk_ers(path, parsed, ers);
    });
    assert_joined_ers_empty(types::SourceTextListRef::from(ers.as_slice()), exp_id);
}
#[allow(clippy::single_call_fn)] // shared crate-manifest cargo policy assertion keeps joined-diagnostic behavior consistent across package-metadata checks
fn assert_crate_manifest_cargo_policy(
    exp_id: types::StaticStr,
    mut mk_ers: impl FnMut(&std::path::Path, &toml::Table, &mut Vec<String>),
) {
    assert_cargo_toml_ers_empty(exp_id, |path, parsed, ers| {
        mk_ers(path, parsed, ers);
    });
}
#[allow(clippy::single_call_fn)] // shared joined-error assertion keeps multi-line diagnostics consistent across workspace policy tests
fn assert_joined_ers_empty(ers: types::SourceTextListRef<'_>, exp_id: types::StaticStr) {
    assert_joined_ers_empty_with_ctx(
        ers,
        exp_id,
        types::SourceTextRef::from(str_constants::PG_CRUD_EMPTY_SQL_SUFFIX),
    );
}
#[allow(clippy::single_call_fn)] // shared assertion with context keeps multiline diagnostics reusable without duplicating message-format glue
fn assert_joined_ers_empty_with_ctx(
    ers: types::SourceTextListRef<'_>,
    exp_id: types::StaticStr,
    ctx: types::SourceTextRef<'_>,
) {
    if ctx.as_ref().is_empty() {
        assert!(
            ers.as_ref().is_empty(),
            "{}\n{}",
            exp_id.get(),
            ers.as_ref().join("\n")
        );
    } else {
        assert!(
            ers.as_ref().is_empty(),
            "{} {}\n{}",
            exp_id.get(),
            ctx.as_ref(),
            ers.as_ref().join("\n")
        );
    }
}
#[allow(clippy::single_call_fn)] // shared sort+assert helper keeps joined diagnostics deterministic for tests that accumulate path-dependent errors
fn assert_joined_ers_empty_sorted(
    mut ers: types::DiagnosticMsgsMutRef<'_>,
    exp_id: types::StaticStr,
) {
    ers.sort();
    assert_joined_ers_empty(types::SourceTextListRef::from(ers.as_slice()), exp_id);
}
#[allow(clippy::single_call_fn)] // shared helper avoids repeated conversion of vec<string> into set<&str>
fn str_set(v: types::SourceTextListRef<'_>) -> types::StdSourceTextHashSet<'_> {
    types::StdSourceTextHashSet::from(
        v.get()
            .iter()
            .map(String::as_str)
            .collect::<std::collections::HashSet<&str>>(),
    )
}
#[allow(clippy::single_call_fn)] // shared duplicate finder keeps uniqueness checks reusable and consistent
fn find_duplicate_strings(v: types::SourceTextListRef<'_>) -> types::SourceTextList {
    let mut seen = std::collections::HashSet::new();
    types::SourceTextList::from(
        v.get()
            .iter()
            .filter(|element_45f4b8bc| !seen.insert(element_45f4b8bc.as_str()))
            .cloned()
            .collect::<Vec<String>>(),
    )
}
#[allow(clippy::single_call_fn)] // reusable collector stays available for AST-policy tests and keeps collection logic separate from assertion wrappers
fn collect_rs_ast_ers(
    mut mk_ers: impl FnMut(&std::path::Path, &syn::File, &mut Vec<String>),
) -> types::SourceTextList {
    let mut ers = Vec::new();
    for_each_rs_syn_file(|path, ast| {
        mk_ers(path, ast, &mut ers);
    });
    types::SourceTextList::from(ers)
}
#[allow(clippy::single_call_fn)] // shared visitor runner keeps AST test callsites focused on assertion logic rather than visit boilerplate
fn visit_syn_file<V>(ast: types::SynFileRef<'_>, mut visitor: V) -> V
where
    V: for<'ast> syn::visit::Visit<'ast>,
{
    syn::visit::Visit::visit_file(&mut visitor, ast.as_ref());
    visitor
}
#[allow(clippy::single_call_fn)] // shared assertion wrapper keeps AST-policy tests focused on visitor logic while reusing collection and joined-report formatting
fn assert_rs_ast_ers_empty_with_ctx(
    exp_id: types::StaticStr,
    ctx: types::SourceTextRef<'_>,
    mut mk_ers: impl FnMut(&std::path::Path, &syn::File, &mut Vec<String>),
) {
    let ers = collect_rs_ast_ers(|path, ast, ers| {
        mk_ers(path, ast, ers);
    });
    assert_joined_ers_empty_with_ctx(types::SourceTextListRef::from(ers.as_slice()), exp_id, ctx);
}
#[allow(clippy::single_call_fn)] // shared parser keeps Cargo.toml read+parse behavior centralized for policy collectors
fn read_toml_table(path: types::StdPathRef<'_>) -> Option<types::TomlTable> {
    snapshot::with_codebase_snapshot(|snapshot| snapshot.read_toml_table(path))
}
#[allow(clippy::single_call_fn)] // shared lookup avoids rereading crate manifests in text-based Cargo.toml style checks
fn cargo_toml_content(path: types::StdPathRef<'_>) -> Option<types::SourceText> {
    snapshot::with_codebase_snapshot(|snapshot| snapshot.cargo_toml_content(path))
}
#[allow(clippy::single_call_fn)] // isolates non-english diagnostics so file-level test stays focused on traversal and assertion
fn collect_non_english_symbol_ers(
    path: types::StdPathRef<'_>,
    v: types::SourceTextRef<'_>,
) -> types::SourceTextList {
    types::SourceTextList::from(
        v.as_ref()
            .lines()
            .enumerate()
            .flat_map(|(line_idx, line)| {
                let line_number = line_idx.saturating_add(1);
                line.chars()
                    .filter(|ch| !is_allowed_english_char(types::AnalyzerChar::from(*ch)).get())
                    .map(move |ch| {
                        format!(
                            "{}:{} non-english symbol `{}` (U+{:04X})",
                            path.as_ref().display(),
                            line_number,
                            ch,
                            u32::from(ch)
                        )
                    })
            })
            .collect::<Vec<String>>(),
    )
}
#[allow(clippy::single_call_fn)] // shared character predicate keeps english-only symbol policy centralized
fn is_allowed_english_char(ch: types::AnalyzerChar) -> types::AnalyzerBool {
    let ch_value = ch.get();
    types::AnalyzerBool::from(
        matches!(ch_value, '\n' | '\r' | '\t' | '\u{2014}' | '\u{2194}') || ch_value.is_ascii(),
    )
}
#[allow(clippy::single_call_fn)] // shared repeated-file error helper keeps AST visitor diagnostics consistent
fn push_repeated_file_error(
    mut ers: types::DiagnosticMsgsMutRef<'_>,
    path: types::StdPathRef<'_>,
    message: types::SourceTextRef<'_>,
    times: types::AnalyzerCount,
) {
    ers.extend(
        std::iter::repeat_with(|| format!("{}: {}", path.as_ref().display(), message.as_ref()))
            .take(times.get()),
    );
}
#[allow(clippy::single_call_fn)] // package names are used to distinguish workspace paths from external crate paths
fn workspace_crate_names() -> types::StdSourceTextSet {
    snapshot::with_codebase_snapshot(snapshot::CodebaseSnapshot::workspace_crate_names)
}
#[allow(clippy::single_call_fn)] // shared traversal uses cargo metadata so crate manifests match Cargo's view of workspace packages
fn for_each_crate_manifest_file(on_file: impl FnMut(&std::path::Path)) {
    snapshot::with_codebase_snapshot(|snapshot| {
        snapshot.crate_manifest_paths().for_each(on_file);
    });
}
#[allow(clippy::single_call_fn)] // shared extension predicate keeps source-policy file-kind checks consistent
fn is_allowed_english_check_ext(ext: Option<types::SourceTextRef<'_>>) -> types::AnalyzerBool {
    types::AnalyzerBool::from(matches!(
        ext.map(types::SourceTextRef::get),
        Some(
            str_constants::RS
                | str_constants::TOML
                | str_constants::TXT
                | str_constants::YML
                | str_constants::YAML
                | str_constants::JSON
        )
    ))
}
fn path_has_segment(
    path: types::SynPathRef<'_>,
    segment: types::SourceTextRef<'_>,
) -> types::AnalyzerBool {
    types::AnalyzerBool::from(
        path.as_ref()
            .segments
            .iter()
            .any(|element| element.ident == segment.as_ref()),
    )
}
#[allow(clippy::single_call_fn)] // names the From<String> trait-shape check for the string-wrapper policy visitor
fn item_impl_is_from_string(item: types::SynItemImplRef<'_>) -> types::AnalyzerBool {
    types::AnalyzerBool::from(item.as_ref().trait_.as_ref().is_some_and(|(_, path, _)| {
        path_ends_with(
            types::SynPathRef::from(path),
            types::StaticStrSliceRef::from([str_constants::FROM_ALT_3].as_slice()),
        )
        .get()
            && from_trait_arg_is_string(types::SynPathRef::from(path)).get()
    }))
}
#[allow(clippy::single_call_fn)] // names the TryFrom<String> trait-shape check for the string-wrapper policy visitor
fn item_impl_is_try_from_string(item: types::SynItemImplRef<'_>) -> types::AnalyzerBool {
    types::AnalyzerBool::from(item.as_ref().trait_.as_ref().is_some_and(|(_, path, _)| {
        path_ends_with(
            types::SynPathRef::from(path),
            types::StaticStrSliceRef::from([str_constants::TRYFROM].as_slice()),
        )
        .get()
            && from_trait_arg_is_string(types::SynPathRef::from(path)).get()
    }))
}
#[allow(clippy::single_call_fn)] // keeps length-check detection local to the string-wrapper TryFrom policy
fn item_impl_contains_len_call(item: types::SynItemImplRef<'_>) -> types::AnalyzerBool {
    let mut visitor = LenMethodCallVisitor {
        found: types::AnalyzerBool::default(),
    };
    syn::visit::Visit::visit_item_impl(&mut visitor, item.as_ref());
    visitor.found
}
#[allow(clippy::single_call_fn)] // extracts impl target type name for string-wrapper diagnostics
fn item_impl_self_ty_identifier(item: types::SynItemImplRef<'_>) -> Option<types::SourceText> {
    match item.as_ref().self_ty.as_ref() {
        syn::Type::Path(ty_path) => ty_path.path.segments.last().map(|segment| {
            types::SourceText::try_from(segment.ident.to_string()).expect("6a9f03d2")
        }),
        syn::Type::Array(_)
        | syn::Type::BareFn(_)
        | syn::Type::Group(_)
        | syn::Type::ImplTrait(_)
        | syn::Type::Infer(_)
        | syn::Type::Macro(_)
        | syn::Type::Never(_)
        | syn::Type::Paren(_)
        | syn::Type::Ptr(_)
        | syn::Type::Reference(_)
        | syn::Type::Slice(_)
        | syn::Type::TraitObject(_)
        | syn::Type::Tuple(_)
        | syn::Type::Verbatim(_)
        | _ => None,
    }
}
#[allow(clippy::single_call_fn)] // isolates From<String> generic-argument parsing from impl visitor flow
fn from_trait_arg_is_string(path: types::SynPathRef<'_>) -> types::AnalyzerBool {
    types::AnalyzerBool::from(path.as_ref().segments.last().is_some_and(|segment| {
                match &segment.arguments {
                    syn::PathArguments::AngleBracketed(args) => {
                        args.args.iter().any(|arg| {
                            matches!(arg, syn::GenericArgument::Type(ty) if type_path_ends_with_identifier(types::SynTypeRef::from(ty), types::SourceTextRef::from(str_constants::STRING)).get())
                        })
                    }
                    syn::PathArguments::Parenthesized(_) | syn::PathArguments::None => false,
                }
            }))
}
fn item_struct_is_single_string_wrapper(item: types::SynItemStructRef<'_>) -> types::AnalyzerBool {
    types::AnalyzerBool::from(match &item.as_ref().fields {
        syn::Fields::Unnamed(fields) if fields.unnamed.len() == 1 => {
            fields.unnamed.first().is_some_and(|field| {
                type_path_ends_with_identifier(
                    types::SynTypeRef::from(&field.ty),
                    types::SourceTextRef::from(str_constants::STRING),
                )
                .get()
            })
        }
        syn::Fields::Named(_) | syn::Fields::Unnamed(_) | syn::Fields::Unit => false,
    })
}
#[allow(clippy::single_call_fn)] // names the tuple-newtype shape used by the wrapper field visibility policy
fn item_struct_is_single_field_tuple_wrapper(
    item: types::SynItemStructRef<'_>,
) -> types::AnalyzerBool {
    types::AnalyzerBool::from(
        matches!(&item.as_ref().fields, syn::Fields::Unnamed(fields) if fields.unnamed.len() == 1),
    )
}
#[allow(clippy::single_call_fn)] // isolates tuple field visibility parsing from policy diagnostics
fn item_struct_single_field_is_non_private(
    item: types::SynItemStructRef<'_>,
) -> types::AnalyzerBool {
    types::AnalyzerBool::from(match &item.as_ref().fields {
        syn::Fields::Unnamed(fields) if fields.unnamed.len() == 1 => fields
            .unnamed
            .first()
            .is_some_and(|field| !matches!(field.vis, syn::Visibility::Inherited)),
        syn::Fields::Named(_) | syn::Fields::Unnamed(_) | syn::Fields::Unit => false,
    })
}
#[allow(clippy::single_call_fn)] // isolated serde derive detection keeps the visitor condition declarative
fn item_struct_derives_deserialize(item: types::SynItemStructRef<'_>) -> types::AnalyzerBool {
    types::AnalyzerBool::from(item.as_ref().attrs.iter().any(|attr| {
        attr.path().is_ident(str_constants::DERIVE)
            && match &attr.meta {
                syn::Meta::List(list) => list
                    .tokens
                    .to_string()
                    .contains(str_constants::CODE_STYLE_DESERIALIZE_DERIVE_NAME),
                syn::Meta::NameValue(_) | syn::Meta::Path(_) => false,
            }
    }))
}
#[allow(clippy::single_call_fn)] // isolated serde conversion detection keeps attribute parsing reusable and testable
fn item_struct_deserialize_uses_conversion(
    item: types::SynItemStructRef<'_>,
) -> types::AnalyzerBool {
    types::AnalyzerBool::from(item.as_ref().attrs.iter().any(|attr| {
        if !attr.path().is_ident(str_constants::SERDE) {
            return false;
        }
        match &attr.meta {
            syn::Meta::List(list) => {
                let tokens = list.tokens.to_string();
                tokens.contains(str_constants::CODE_STYLE_SERDE_FROM_ATTR_FRAGMENT)
                    || tokens.contains(str_constants::CODE_STYLE_SERDE_TRY_FROM_ATTR_FRAGMENT)
            }
            syn::Meta::NameValue(_) | syn::Meta::Path(_) => false,
        }
    }))
}
#[allow(clippy::single_call_fn)] // conversion derive recognition is kept separate from wrapper collection
fn item_struct_derives_conversion(item: types::SynItemStructRef<'_>) -> types::AnalyzerBool {
    types::AnalyzerBool::from(item.as_ref().attrs.iter().any(|attr| {
        if !attr.path().is_ident(str_constants::DERIVE) {
            return false;
        }
        match &attr.meta {
            syn::Meta::List(list) => {
                let tokens = list.tokens.to_string();
                tokens.contains(str_constants::NEWTYPE_FROM_INNER_DERIVE_NAME)
                    || tokens.contains(str_constants::BOUNDEDSTRING)
                    || tokens.contains(str_constants::TRYFROM)
            }
            syn::Meta::NameValue(_) | syn::Meta::Path(_) => false,
        }
    }))
}
fn item_impl_is_from_or_try_from(item: types::SynItemImplRef<'_>) -> types::AnalyzerBool {
    types::AnalyzerBool::from(item.as_ref().trait_.as_ref().is_some_and(|(_, path, _)| {
        path.segments.last().is_some_and(|segment| {
            segment.ident == str_constants::FROM_ALT_3 || segment.ident == str_constants::TRYFROM
        })
    }))
}
#[allow(clippy::single_call_fn)] // diagnostic conversion errors intentionally carry raw length metadata
fn identifier_is_diagnostic_try_from_string_error(
    identifier: types::SynIdentifierRef<'_>,
) -> types::AnalyzerBool {
    types::AnalyzerBool::from(
        identifier
            .as_ref()
            .to_string()
            .ends_with(str_constants::TRYFROMSTRINGERROR),
    )
}
#[allow(clippy::single_call_fn)] // explicit wrapper escape hatches are allowed to expose their inner representation
fn method_is_explicit_wrapper_accessor(
    identifier: types::SynIdentifierRef<'_>,
) -> types::AnalyzerBool {
    types::AnalyzerBool::from(matches!(
        identifier.as_ref().to_string().as_str(),
        str_constants::GET_ALT | str_constants::INTO_INNER
    ))
}
fn type_path_ends_with_identifier(
    ty: types::SynTypeRef<'_>,
    identifier: types::SourceTextRef<'_>,
) -> types::AnalyzerBool {
    types::AnalyzerBool::from(match ty.as_ref() {
        syn::Type::Path(ty_path) => ty_path
            .path
            .segments
            .last()
            .is_some_and(|segment| segment.ident == identifier.as_ref()),
        syn::Type::Array(_)
        | syn::Type::BareFn(_)
        | syn::Type::Group(_)
        | syn::Type::ImplTrait(_)
        | syn::Type::Infer(_)
        | syn::Type::Macro(_)
        | syn::Type::Never(_)
        | syn::Type::Paren(_)
        | syn::Type::Ptr(_)
        | syn::Type::Reference(_)
        | syn::Type::Slice(_)
        | syn::Type::TraitObject(_)
        | syn::Type::Tuple(_)
        | syn::Type::Verbatim(_)
        | _ => false,
    })
}
#[allow(clippy::single_call_fn)] // keeps FromInner derive detection reusable inside the string-wrapper policy
fn attr_has_newtype_from_option(attr: types::SynAttributeRef<'_>) -> types::AnalyzerBool {
    let attr_ref = attr.as_ref();
    if !attr_ref.path().is_ident(str_constants::DERIVE) {
        return types::AnalyzerBool::default();
    }
    types::AnalyzerBool::from(attr_ref.meta.require_list().is_ok_and(|list| {
        list.tokens
            .to_string()
            .contains(str_constants::NEWTYPE_FROM_INNER_DERIVE_NAME)
    }))
}
#[allow(clippy::single_call_fn)] // keeps BoundedString derive parsing reusable inside the string-wrapper policy
fn attr_has_bounded_string_derive(attr: types::SynAttributeRef<'_>) -> types::AnalyzerBool {
    let attr_ref = attr.as_ref();
    if !attr_ref.path().is_ident(str_constants::DERIVE) {
        return types::AnalyzerBool::default();
    }
    types::AnalyzerBool::from(attr_ref.meta.require_list().is_ok_and(|list| {
        list.tokens
            .to_string()
            .contains(str_constants::BOUNDEDSTRING)
    }))
}
#[allow(clippy::single_call_fn)] // bounded string wrappers satisfy length policy only when max is explicit
fn attr_has_bounded_string_max_bound(attr: types::SynAttributeRef<'_>) -> types::AnalyzerBool {
    let attr_ref = attr.as_ref();
    if !attr_ref.path().is_ident(str_constants::BOUNDED_STRING) {
        return types::AnalyzerBool::default();
    }
    let mut has_max = false;
    drop(attr_ref.parse_nested_meta(|meta| {
        if meta.path.is_ident(str_constants::MAX) {
            drop(meta.value()?.parse::<syn::Expr>()?);
            has_max = true;
            return Ok(());
        }
        Err(meta.error(str_constants::UNKNOWN_BOUNDED_STRING_OPTION))
    }));
    types::AnalyzerBool::from(has_max)
}
fn path_ends_with(
    path: types::SynPathRef<'_>,
    segments: types::StaticStrSliceRef<'_>,
) -> types::AnalyzerBool {
    let path_ref = path.as_ref();
    types::AnalyzerBool::from(
        path_ref.segments.len() >= segments.get().len()
            && path_ref
                .segments
                .iter()
                .rev()
                .zip(segments.get().iter().rev())
                .all(|(got, exp)| got.ident == *exp),
    )
}
#[allow(clippy::single_call_fn)] // names Self-path handling separately from domain type path traversal
fn path_first_segment_is_self(path: types::SynPathRef<'_>) -> types::AnalyzerBool {
    types::AnalyzerBool::from(
        path.as_ref()
            .segments
            .first()
            .is_some_and(|segment| segment.ident == str_constants::SELF),
    )
}
fn expr_call_path(call: types::SynExprCallRef<'_>) -> Option<types::SynPathRef<'_>> {
    match call.get().func.as_ref() {
        syn::Expr::Path(path) => Some(types::SynPathRef::from(&path.path)),
        syn::Expr::Array(_)
        | syn::Expr::Assign(_)
        | syn::Expr::Async(_)
        | syn::Expr::Await(_)
        | syn::Expr::Binary(_)
        | syn::Expr::Block(_)
        | syn::Expr::Break(_)
        | syn::Expr::Call(_)
        | syn::Expr::Cast(_)
        | syn::Expr::Closure(_)
        | syn::Expr::Const(_)
        | syn::Expr::Continue(_)
        | syn::Expr::Field(_)
        | syn::Expr::ForLoop(_)
        | syn::Expr::Group(_)
        | syn::Expr::If(_)
        | syn::Expr::Index(_)
        | syn::Expr::Infer(_)
        | syn::Expr::Let(_)
        | syn::Expr::Lit(_)
        | syn::Expr::Loop(_)
        | syn::Expr::Macro(_)
        | syn::Expr::Match(_)
        | syn::Expr::MethodCall(_)
        | syn::Expr::Paren(_)
        | syn::Expr::Range(_)
        | syn::Expr::RawAddr(_)
        | syn::Expr::Reference(_)
        | syn::Expr::Repeat(_)
        | syn::Expr::Return(_)
        | syn::Expr::Struct(_)
        | syn::Expr::Try(_)
        | syn::Expr::TryBlock(_)
        | syn::Expr::Tuple(_)
        | syn::Expr::Unary(_)
        | syn::Expr::Unsafe(_)
        | syn::Expr::Verbatim(_)
        | syn::Expr::While(_)
        | syn::Expr::Yield(_)
        | _ => None,
    }
}
#[allow(clippy::single_call_fn)] // extracts repo macro domain type discovery from the visitor traversal
fn collect_generate_pg_types_domain_names(
    tokens: types::SourceTextRef<'_>,
    names: &mut types::StdSourceTextSet,
) {
    let re = regex::Regex::new(str_constants::A_ZA_Z0_9_PLUS_AS_A_ZA_Z0_9_PLUS).expect("f4e61b29");
    re.captures_iter(tokens.as_ref())
        .filter_map(|captures| {
            let base = captures.get(1).map(|element| element.as_str())?;
            base.split_once(str_constants::AS)
        })
        .for_each(|(prefix, suffix)| {
            let _: bool = names.insert(format!("{prefix}AsNonNull{suffix}"));
            let _: bool = names.insert(format!("Optional{prefix}AsNullable{suffix}"));
        });
}
#[allow(clippy::single_call_fn)] // config_lib helper macros declare domain wrapper structs from their first argument
fn config_lib_domain_type_macro_path(path: types::SynPathRef<'_>) -> types::AnalyzerBool {
    types::AnalyzerBool::from(
        path_ends_with(
            path,
            types::StaticStrSliceRef::from(
                [
                    str_constants::CONFIG_LIB_MACROS,
                    str_constants::IMPL_TRY_FROM_NON_EMPTY_STRING,
                ]
                .as_slice(),
            ),
        )
        .get()
            || path_ends_with(
                path,
                types::StaticStrSliceRef::from(
                    [
                        str_constants::CONFIG_LIB_MACROS,
                        str_constants::IMPL_TRY_FROM_SECRET_URL,
                    ]
                    .as_slice(),
                ),
            )
            .get()
            || path_ends_with(
                path,
                types::StaticStrSliceRef::from(
                    [
                        str_constants::CONFIG_LIB_MACROS,
                        str_constants::IMPL_TRY_FROM_PARSE,
                    ]
                    .as_slice(),
                ),
            )
            .get()
            || path_ends_with(
                path,
                types::StaticStrSliceRef::from(
                    [
                        str_constants::CONFIG_LIB_MACROS,
                        str_constants::IMPL_TRY_FROM_PARSE_STRING_ERROR,
                    ]
                    .as_slice(),
                ),
            )
            .get(),
    )
}
#[allow(clippy::single_call_fn)] // macro-generated domain wrapper names are the first identifier in these macro inputs
fn collect_first_macro_identifier_domain_name(
    tokens: types::SourceTextRef<'_>,
    names: &mut types::StdSourceTextSet,
) {
    let re = regex::Regex::new(str_constants::S_ASTERISK_A_ZA_Z_A_ZA_Z0_9_ASTERISK_S_ASTERISK)
        .expect("fc65b7c4");
    if let Some(name) = re
        .captures(tokens.as_ref())
        .and_then(|captures| captures.get(1))
        .map(|name| name.as_str())
    {
        let _: bool = names.insert(name.to_owned());
    }
}
#[allow(clippy::single_call_fn)] // keeps Arc type policy readable apart from syn type matching
fn type_contains_segment(
    ty: types::SynTypeRef<'_>,
    segment: types::SourceTextRef<'_>,
) -> types::AnalyzerBool {
    types::AnalyzerBool::from(match ty.as_ref() {
        syn::Type::Path(path) => {
            path_has_segment(types::SynPathRef::from(&path.path), segment).get()
        }
        syn::Type::Array(_)
        | syn::Type::BareFn(_)
        | syn::Type::Group(_)
        | syn::Type::ImplTrait(_)
        | syn::Type::Infer(_)
        | syn::Type::Macro(_)
        | syn::Type::Never(_)
        | syn::Type::Paren(_)
        | syn::Type::Ptr(_)
        | syn::Type::Reference(_)
        | syn::Type::Slice(_)
        | syn::Type::TraitObject(_)
        | syn::Type::Tuple(_)
        | syn::Type::Verbatim(_)
        | _ => false,
    })
}
#[allow(clippy::single_call_fn)] // names the async-blocking method policy separately from traversal code
fn method_is_blocking_async_call(method: types::SourceTextRef<'_>) -> types::AnalyzerBool {
    types::AnalyzerBool::from(matches!(
        method.as_ref(),
        str_constants::BLOCK_ON
            | str_constants::BLOCK_IN_PLACE
            | str_constants::BLOCKING_RECV
            | str_constants::BLOCKING_SEND
    ))
}
#[allow(clippy::single_call_fn)] // names the async-blocking function policy separately from traversal code
fn path_is_blocking_async_call(path: types::SynPathRef<'_>) -> types::AnalyzerBool {
    types::AnalyzerBool::from(
        path_ends_with(
            path,
            types::StaticStrSliceRef::from(
                [
                    str_constants::FUTURES,
                    str_constants::EXECUTOR,
                    str_constants::BLOCK_ON,
                ]
                .as_slice(),
            ),
        )
        .get()
            || path_ends_with(
                path,
                types::StaticStrSliceRef::from(
                    [
                        str_constants::TOKIO,
                        str_constants::TASK,
                        str_constants::BLOCK_IN_PLACE,
                    ]
                    .as_slice(),
                ),
            )
            .get()
            || path_ends_with(
                path,
                types::StaticStrSliceRef::from(
                    [
                        str_constants::STD,
                        str_constants::THREAD,
                        str_constants::SLEEP,
                    ]
                    .as_slice(),
                ),
            )
            .get(),
    )
}
#[allow(clippy::single_call_fn)] // names the external-service unit-test policy separately from traversal code
fn path_is_external_service_client(path: types::SynPathRef<'_>) -> types::AnalyzerBool {
    types::AnalyzerBool::from(
        path_ends_with(
            path,
            types::StaticStrSliceRef::from(
                [
                    str_constants::REQWEST,
                    str_constants::CLIENT,
                    str_constants::NEW,
                ]
                .as_slice(),
            ),
        )
        .get()
            || path_ends_with(
                path,
                types::StaticStrSliceRef::from(
                    [
                        str_constants::STD,
                        str_constants::NET,
                        str_constants::TCPSTREAM,
                        str_constants::CONNECT,
                    ]
                    .as_slice(),
                ),
            )
            .get()
            || path_ends_with(
                path,
                types::StaticStrSliceRef::from(
                    [
                        str_constants::STD,
                        str_constants::NET,
                        str_constants::TCPLISTENER,
                        str_constants::BIND,
                    ]
                    .as_slice(),
                ),
            )
            .get()
            || path_ends_with(
                path,
                types::StaticStrSliceRef::from(
                    [
                        str_constants::STD,
                        str_constants::NET,
                        str_constants::UDPSOCKET,
                        str_constants::BIND,
                    ]
                    .as_slice(),
                ),
            )
            .get()
            || path_ends_with(
                path,
                types::StaticStrSliceRef::from(
                    [
                        str_constants::TOKIO,
                        str_constants::NET,
                        str_constants::TCPSTREAM,
                        str_constants::CONNECT,
                    ]
                    .as_slice(),
                ),
            )
            .get()
            || path_ends_with(
                path,
                types::StaticStrSliceRef::from(
                    [
                        str_constants::TOKIO,
                        str_constants::NET,
                        str_constants::TCPLISTENER,
                        str_constants::BIND,
                    ]
                    .as_slice(),
                ),
            )
            .get()
            || path_ends_with(
                path,
                types::StaticStrSliceRef::from(
                    [
                        str_constants::TOKIO,
                        str_constants::NET,
                        str_constants::UDPSOCKET,
                        str_constants::BIND,
                    ]
                    .as_slice(),
                ),
            )
            .get(),
    )
}
#[allow(clippy::single_call_fn)] // extracted to keep the domain policy test focused on assertion flow
fn declared_domain_type_names() -> types::StdSourceTextSet {
    let mut names = std::collections::BTreeSet::new();
    for_each_rs_syn_file(|_, ast| {
        let visitor = visit_syn_file(
            types::SynFileRef::from(ast),
            DeclaredDomainTypeVisitor {
                names: types::StdSourceTextSet::default(),
            },
        );
        names.extend(visitor.names);
    });
    types::StdSourceTextSet::from(names)
}
#[allow(clippy::single_call_fn)] // collects tuple String wrapper names before checking From<String> impls
fn len_checked_function_names(file: types::SynFileRef<'_>) -> types::StdSourceTextSet {
    let mut visitor = LenCheckedFunctionNameVisitor {
        names: types::StdSourceTextSet::default(),
    };
    syn::visit::Visit::visit_file(&mut visitor, file.as_ref());
    visitor.names
}
fn string_wrapper_names(ast: types::SynFileRef<'_>) -> types::StdSourceTextSet {
    visit_syn_file(
        ast,
        StringWrapperNameVisitor {
            names: types::StdSourceTextSet::default(),
        },
    )
    .names
}
#[allow(clippy::single_call_fn)] // keeps domain policy exception handling centralized and documented
fn domain_type_policy_should_check_path(path: types::StdPathRef<'_>) -> types::AnalyzerBool {
    let location_test_component = format!(
        "{}{}",
        str_constants::LOCATION.to_ascii_lowercase(),
        str_constants::TEST
    );
    if path
        .as_ref()
        .starts_with(str_constants::WORKSPACE_TEST_RUNNER_SRC)
        || path
            .as_ref()
            .starts_with(str_constants::WORKSPACE_SCAFFOLD_SRC)
        || path
            .as_ref()
            .starts_with(str_constants::INITIALIZE_ENVIRONMENT_FILES_SRC)
        || path
            .as_ref()
            .components()
            .any(|component| component.as_os_str() == str_constants::BENCHES)
        || path.as_ref().components().any(|component| {
            component.as_os_str() == std::ffi::OsStr::new(location_test_component.as_str())
        })
    {
        return types::AnalyzerBool::default();
    }
    if path
        .as_ref()
        .ends_with(str_constants::SERVER_ADMIN_FRONTEND_SRC_APP_RS)
        || path
            .as_ref()
            .starts_with(str_constants::SERVER_ADMIN_FRONTEND_SRC_APP)
    {
        return types::AnalyzerBool::default();
    }
    let Some(cargo_toml_path) = nearest_cargo_toml_path(path) else {
        return types::AnalyzerBool::default();
    };
    let Some(manifest) = read_toml_table(types::StdPathRef::from(cargo_toml_path.as_ref())) else {
        return types::AnalyzerBool::default();
    };
    let is_proc_macro = manifest
        .as_ref()
        .get(str_constants::LIB)
        .and_then(toml::Value::as_table)
        .and_then(|lib| lib.get(str_constants::PROC_MACRO))
        .and_then(toml::Value::as_bool)
        .unwrap_or(false);
    types::AnalyzerBool::from(!is_proc_macro)
}
#[allow(clippy::single_call_fn)] // helper-return text wrappers live in the code-style meta harness types module
fn is_code_style_meta_harness_source_path(path: types::StdPathRef<'_>) -> types::AnalyzerBool {
    types::AnalyzerBool::from(
        path.as_ref()
            .starts_with(str_constants::TESTS_SRC_CODE_STYLE),
    )
}
#[allow(clippy::single_call_fn)] // keeps transparent container policy separate from path validation
fn is_structural_generic_container(identifier: types::SourceTextRef<'_>) -> types::AnalyzerBool {
    types::AnalyzerBool::from(matches!(
        identifier.as_ref(),
        str_constants::OPTION | str_constants::RESULT
    ))
}
fn analyzer_state_raw_container_ty(
    ty: types::SynTypeRef<'_>,
) -> Option<(types::StaticStr, types::StaticStr)> {
    match ty.get() {
        syn::Type::Group(ty_group) => {
            analyzer_state_raw_container_ty(types::SynTypeRef::from(&*ty_group.elem))
        }
        syn::Type::Paren(ty_paren) => {
            analyzer_state_raw_container_ty(types::SynTypeRef::from(&*ty_paren.elem))
        }
        syn::Type::Path(ty_path) => {
            analyzer_state_raw_container_ty_path(types::SynTypePathRef::from(ty_path))
        }
        syn::Type::Reference(ty_reference) => {
            analyzer_state_raw_container_ty(types::SynTypeRef::from(&*ty_reference.elem))
        }
        syn::Type::Array(_)
        | syn::Type::BareFn(_)
        | syn::Type::ImplTrait(_)
        | syn::Type::Infer(_)
        | syn::Type::Macro(_)
        | syn::Type::Never(_)
        | syn::Type::Ptr(_)
        | syn::Type::Slice(_)
        | syn::Type::TraitObject(_)
        | syn::Type::Tuple(_)
        | syn::Type::Verbatim(_)
        | _ => None,
    }
}
fn raw_text_return_ty(ty: types::SynTypeRef<'_>) -> Option<(types::StaticStr, types::StaticStr)> {
    match ty.get() {
        syn::Type::Group(ty_group) => raw_text_return_ty(types::SynTypeRef::from(&*ty_group.elem)),
        syn::Type::Paren(ty_paren) => raw_text_return_ty(types::SynTypeRef::from(&*ty_paren.elem)),
        syn::Type::Path(ty_path) => raw_text_return_ty_path(types::SynTypePathRef::from(ty_path)),
        syn::Type::Reference(_) if type_is_str_ref(ty).get() => Some((
            types::StaticStr::from(str_constants::STR),
            types::StaticStr::from(str_constants::TYPES_PATH_SOURCETEXTREF),
        )),
        syn::Type::Reference(ty_reference) => {
            raw_text_return_ty(types::SynTypeRef::from(&*ty_reference.elem))
        }
        syn::Type::Array(_)
        | syn::Type::BareFn(_)
        | syn::Type::ImplTrait(_)
        | syn::Type::Infer(_)
        | syn::Type::Macro(_)
        | syn::Type::Never(_)
        | syn::Type::Ptr(_)
        | syn::Type::Slice(_)
        | syn::Type::TraitObject(_)
        | syn::Type::Tuple(_)
        | syn::Type::Verbatim(_)
        | _ => None,
    }
}
#[allow(clippy::single_call_fn)] // separates return path matching from nested raw text return traversal
fn raw_text_return_ty_path(
    ty_path: types::SynTypePathRef<'_>,
) -> Option<(types::StaticStr, types::StaticStr)> {
    let ty_path_ref = ty_path.get();
    let segment = ty_path_ref.path.segments.last()?;
    let identifier = segment.ident.to_string();
    match identifier.as_str() {
        str_constants::STRING => Some((
            types::StaticStr::from(str_constants::STRING),
            types::StaticStr::from(str_constants::TYPES_PATH_SOURCETEXT),
        )),
        str_constants::VEC
            if single_angle_type_arg(types::SynPathArgumentsRef::from(&segment.arguments))
                .is_some_and(|ty| type_is_string(types::SynTypeRef::from(ty.get())).get()) =>
        {
            Some((
                types::StaticStr::from(str_constants::VEC_STRING),
                types::StaticStr::from(str_constants::TYPES_PATH_SOURCETEXTLIST),
            ))
        }
        str_constants::OPTION
            if single_angle_type_arg(types::SynPathArgumentsRef::from(&segment.arguments))
                .is_some_and(|ty| type_is_str_ref(types::SynTypeRef::from(ty.get())).get()) =>
        {
            Some((
                types::StaticStr::from(str_constants::OPTION_STR),
                types::StaticStr::from(str_constants::OPTION_TYPES_PATH_SOURCETEXTREF),
            ))
        }
        str_constants::OPTION
        | str_constants::RESULT
        | str_constants::BOX
        | str_constants::COW
        | str_constants::ARC
        | str_constants::RC
        | str_constants::PIN
        | str_constants::PHANTOMDATA
        | str_constants::HASHMAP
        | str_constants::BTREEMAP
        | str_constants::HASHSET
        | str_constants::BTREESET => {
            raw_text_return_path_arguments(types::SynPathArgumentsRef::from(&segment.arguments))
        }
        _ => None,
    }
}
#[allow(clippy::single_call_fn)] // keeps nested helper-return traversal independent from field-state diagnostics
fn raw_text_return_path_arguments(
    arguments: types::SynPathArgumentsRef<'_>,
) -> Option<(types::StaticStr, types::StaticStr)> {
    match arguments.get() {
        syn::PathArguments::AngleBracketed(args) => args.args.iter().find_map(|arg| match arg {
            syn::GenericArgument::Type(ty) => raw_text_return_ty(types::SynTypeRef::from(ty)),
            syn::GenericArgument::AssocConst(_)
            | syn::GenericArgument::AssocType(_)
            | syn::GenericArgument::Constraint(_)
            | syn::GenericArgument::Const(_)
            | syn::GenericArgument::Lifetime(_)
            | _ => None,
        }),
        syn::PathArguments::Parenthesized(args) => args
            .inputs
            .iter()
            .find_map(|ty| raw_text_return_ty(types::SynTypeRef::from(ty)))
            .or_else(|| match &args.output {
                syn::ReturnType::Default => None,
                syn::ReturnType::Type(_, ty) => raw_text_return_ty(types::SynTypeRef::from(&**ty)),
            }),
        syn::PathArguments::None => None,
    }
}
#[allow(clippy::single_call_fn)] // separates path-shape matching from recursive wrapper/state field traversal
fn analyzer_state_raw_container_ty_path(
    ty_path: types::SynTypePathRef<'_>,
) -> Option<(types::StaticStr, types::StaticStr)> {
    let ty_path_ref = ty_path.get();
    let segment = ty_path_ref.path.segments.last()?;
    let identifier = segment.ident.to_string();
    match identifier.as_str() {
        str_constants::VEC
            if single_angle_type_arg(types::SynPathArgumentsRef::from(&segment.arguments))
                .is_some_and(|ty| type_is_string(types::SynTypeRef::from(ty.get())).get()) =>
        {
            Some((
                types::StaticStr::from(str_constants::VEC_STRING),
                types::StaticStr::from(str_constants::TYPES_PATH_SOURCETEXTLIST),
            ))
        }
        str_constants::BTREESET
            if single_angle_type_arg(types::SynPathArgumentsRef::from(&segment.arguments))
                .is_some_and(|ty| type_is_string(types::SynTypeRef::from(ty.get())).get()) =>
        {
            Some((
                types::StaticStr::from(str_constants::BTREESET_STRING),
                types::StaticStr::from(str_constants::TYPES_PATH_STDSOURCETEXTSET),
            ))
        }
        str_constants::HASHSET
            if single_angle_type_arg(types::SynPathArgumentsRef::from(&segment.arguments))
                .is_some_and(|ty| type_is_str_ref(types::SynTypeRef::from(ty.get())).get()) =>
        {
            Some((
                types::StaticStr::from(str_constants::HASHSET_STR),
                types::StaticStr::from(str_constants::TYPES_PATH_STDSOURCETEXTHASHSET_OR_TYPES_PATH_STDSOURCETEXTREFSET),
            ))
        }
        str_constants::OPTION
        | str_constants::RESULT
        | str_constants::BOX
        | str_constants::COW
        | str_constants::ARC
        | str_constants::RC
        | str_constants::PIN
        | str_constants::PHANTOMDATA
        | str_constants::HASHMAP
        | str_constants::BTREEMAP => analyzer_state_raw_container_path_arguments(
            types::SynPathArgumentsRef::from(&segment.arguments),
        ),
        _ => None,
    }
}
#[allow(clippy::single_call_fn)] // keeps nested container traversal readable where state fields are diagnosed
fn analyzer_state_raw_container_path_arguments(
    arguments: types::SynPathArgumentsRef<'_>,
) -> Option<(types::StaticStr, types::StaticStr)> {
    match arguments.get() {
        syn::PathArguments::AngleBracketed(args) => args.args.iter().find_map(|arg| match arg {
            syn::GenericArgument::Type(ty) => {
                analyzer_state_raw_container_ty(types::SynTypeRef::from(ty))
            }
            syn::GenericArgument::AssocConst(_)
            | syn::GenericArgument::AssocType(_)
            | syn::GenericArgument::Constraint(_)
            | syn::GenericArgument::Const(_)
            | syn::GenericArgument::Lifetime(_)
            | _ => None,
        }),
        syn::PathArguments::Parenthesized(args) => args
            .inputs
            .iter()
            .find_map(|ty| analyzer_state_raw_container_ty(types::SynTypeRef::from(ty)))
            .or_else(|| match &args.output {
                syn::ReturnType::Default => None,
                syn::ReturnType::Type(_, ty) => {
                    analyzer_state_raw_container_ty(types::SynTypeRef::from(&**ty))
                }
            }),
        syn::PathArguments::None => None,
    }
}
fn single_angle_type_arg(
    arguments: types::SynPathArgumentsRef<'_>,
) -> Option<types::SynTypeRef<'_>> {
    let syn::PathArguments::AngleBracketed(args) = arguments.get() else {
        return None;
    };
    let mut type_args = args.args.iter().filter_map(|arg| match arg {
        syn::GenericArgument::Type(ty) => Some(types::SynTypeRef::from(ty)),
        syn::GenericArgument::AssocConst(_)
        | syn::GenericArgument::AssocType(_)
        | syn::GenericArgument::Constraint(_)
        | syn::GenericArgument::Const(_)
        | syn::GenericArgument::Lifetime(_)
        | _ => None,
    });
    let first = type_args.next()?;
    if type_args.next().is_some() {
        return None;
    }
    Some(first)
}
fn type_is_string(ty: types::SynTypeRef<'_>) -> types::AnalyzerBool {
    types::AnalyzerBool::from(match ty.get() {
        syn::Type::Path(ty_path) => ty_path
            .path
            .segments
            .last()
            .is_some_and(|segment| segment.ident == str_constants::STRING),
        syn::Type::Array(_)
        | syn::Type::BareFn(_)
        | syn::Type::Group(_)
        | syn::Type::ImplTrait(_)
        | syn::Type::Infer(_)
        | syn::Type::Macro(_)
        | syn::Type::Never(_)
        | syn::Type::Paren(_)
        | syn::Type::Ptr(_)
        | syn::Type::Reference(_)
        | syn::Type::Slice(_)
        | syn::Type::TraitObject(_)
        | syn::Type::Tuple(_)
        | syn::Type::Verbatim(_)
        | _ => false,
    })
}
#[allow(clippy::single_call_fn)] // names the HashSet<&str> state-field shape independently from container matching
fn type_is_str_ref(ty: types::SynTypeRef<'_>) -> types::AnalyzerBool {
    types::AnalyzerBool::from(match ty.get() {
        syn::Type::Reference(ty_reference) => match &*ty_reference.elem {
            syn::Type::Path(ty_path) => ty_path
                .path
                .segments
                .last()
                .is_some_and(|segment| segment.ident == str_constants::STR_ALT),
            syn::Type::Array(_)
            | syn::Type::BareFn(_)
            | syn::Type::Group(_)
            | syn::Type::ImplTrait(_)
            | syn::Type::Infer(_)
            | syn::Type::Macro(_)
            | syn::Type::Never(_)
            | syn::Type::Paren(_)
            | syn::Type::Ptr(_)
            | syn::Type::Reference(_)
            | syn::Type::Slice(_)
            | syn::Type::TraitObject(_)
            | syn::Type::Tuple(_)
            | syn::Type::Verbatim(_)
            | _ => false,
        },
        syn::Type::Array(_)
        | syn::Type::BareFn(_)
        | syn::Type::Group(_)
        | syn::Type::ImplTrait(_)
        | syn::Type::Infer(_)
        | syn::Type::Macro(_)
        | syn::Type::Never(_)
        | syn::Type::Paren(_)
        | syn::Type::Path(_)
        | syn::Type::Ptr(_)
        | syn::Type::Slice(_)
        | syn::Type::TraitObject(_)
        | syn::Type::Tuple(_)
        | syn::Type::Verbatim(_)
        | _ => false,
    })
}
#[allow(clippy::single_call_fn)] // proc-macro entrypoints must keep the compiler-required TokenStream ABI
fn item_fn_is_proc_macro(item: types::SynItemFnRef<'_>) -> types::AnalyzerBool {
    types::AnalyzerBool::from(item.as_ref().attrs.iter().any(|attr| {
        attr.path().is_ident(str_constants::PROC_MACRO_ALT)
            || attr.path().is_ident(str_constants::PROC_MACRO_DERIVE)
            || attr.path().is_ident(str_constants::PROC_MACRO_ATTRIBUTE)
    }))
}
#[allow(clippy::single_call_fn)] // shared attr-list wrapper avoids exposing cfg-test matching at visitor callsites
fn attrs_contain_test_only_cfg(attrs: types::SynAttributeListRef<'_>) -> types::AnalyzerBool {
    types::AnalyzerBool::from(
        attrs
            .as_ref()
            .iter()
            .any(|attr| attr_is_test_only_cfg(types::SynAttributeRef::from(attr)).get()),
    )
}
#[allow(clippy::single_call_fn)] // keeps unit-test detection reusable inside nested test module traversal
fn item_fn_is_unit_test(item: types::SynItemFnRef<'_>) -> types::AnalyzerBool {
    types::AnalyzerBool::from(item.as_ref().attrs.iter().any(|attr| {
        attr.path().is_ident(str_constants::TEST_ALT_3)
            || attr_is_test_only_cfg(types::SynAttributeRef::from(attr)).get()
    }))
}
#[allow(clippy::single_call_fn)] // keeps external-service error messages stable and readable
fn path_to_string(path: types::SynPathRef<'_>) -> types::SourceText {
    types::SourceText::try_from(
        path.as_ref()
            .segments
            .iter()
            .map(|segment| segment.ident.to_string())
            .collect::<Vec<String>>()
            .join(str_constants::PATH_SEPARATOR),
    )
    .expect("50c1e4a8")
}
#[allow(clippy::single_call_fn)] // keeps external-wrapper naming suggestion generation readable at the call site
fn identifier_to_upper_camel_fragment(
    identifier: types::SynIdentifierRef<'_>,
) -> types::SourceText {
    let (out, _) = identifier.as_ref().to_string().chars().fold(
        (String::new(), true),
        |(mut out, mut next_upper), ch| {
            if ch == '_' {
                next_upper = true;
                return (out, next_upper);
            }
            if next_upper {
                ch.to_uppercase().for_each(|upper| out.push(upper));
                next_upper = false;
            } else {
                out.push(ch);
            }
            (out, next_upper)
        },
    );
    types::SourceText::try_from(out).expect("9ea072c4")
}
#[allow(clippy::single_call_fn)] // centralizes production-source filtering for panic/expect/unwrap policy
fn is_runtime_policy_source_path(path: types::StdPathRef<'_>) -> types::AnalyzerBool {
    if path.as_ref().file_name().and_then(std::ffi::OsStr::to_str)
        == Some(str_constants::TEST_HLP_RS)
    {
        return types::AnalyzerBool::default();
    }
    if !path
        .as_ref()
        .components()
        .any(|component| component.as_os_str() == str_constants::SRC_ALT)
    {
        return types::AnalyzerBool::default();
    }
    if path
        .as_ref()
        .starts_with(str_constants::INITIALIZE_ENVIRONMENT_FILES_SRC)
    {
        return types::AnalyzerBool::default();
    }
    let Some(cargo_toml_path) = nearest_cargo_toml_path(path) else {
        return types::AnalyzerBool::default();
    };
    let Some(parsed) = read_toml_table(types::StdPathRef::from(cargo_toml_path.as_ref())) else {
        return types::AnalyzerBool::default();
    };
    types::AnalyzerBool::from(
        !is_proc_macro_crate(types::TomlTableRef::from(parsed.as_ref())).get()
            && !is_test_crate(types::TomlTableRef::from(parsed.as_ref())).get(),
    )
}
#[allow(clippy::single_call_fn)] // walks upward from a source file to the owning crate manifest
fn nearest_cargo_toml_path(path: types::StdPathRef<'_>) -> Option<types::StdPathBuf> {
    path.as_ref()
        .ancestors()
        .map(|ancestor| ancestor.join(str_constants::CARGO_TOML))
        .find(|cargo_toml_path| cargo_toml_path.exists())
        .map(types::StdPathBuf::from)
}
#[allow(clippy::single_call_fn)] // package-name based test crate filter keeps generated/test-only crates outside runtime policy
fn is_test_crate(parsed: types::TomlTableRef<'_>) -> types::AnalyzerBool {
    types::AnalyzerBool::from(
        parsed
            .as_ref()
            .get(str_constants::PACKAGE)
            .and_then(toml::Value::as_table)
            .and_then(|package| package.get(str_constants::NAME))
            .and_then(toml::Value::as_str)
            .is_some_and(|name| {
                name == str_constants::TESTS_ALT
                    || name.contains(str_constants::TEST)
                    || name.ends_with(str_constants::TEST_ALT_3)
            }),
    )
}
#[allow(clippy::single_call_fn)] // proc-macro crates are allowed to panic by repository policy
fn is_proc_macro_crate(parsed: types::TomlTableRef<'_>) -> types::AnalyzerBool {
    types::AnalyzerBool::from(
        parsed
            .as_ref()
            .get(str_constants::LIB)
            .and_then(toml::Value::as_table)
            .and_then(|lib| lib.get(str_constants::PROC_MACRO))
            == Some(&toml::Value::Boolean(true)),
    )
}
#[allow(clippy::single_call_fn)] // keeps cfg(test) handling local to runtime AST policy visitor
fn has_test_only_cfg_attr(i: types::SynItemRef<'_>) -> types::AnalyzerBool {
    types::AnalyzerBool::from(match i.as_ref() {
        syn::Item::Const(item) => item
            .attrs
            .iter()
            .any(|attr| attr_is_test_only_cfg(types::SynAttributeRef::from(attr)).get()),
        syn::Item::Enum(item) => item
            .attrs
            .iter()
            .any(|attr| attr_is_test_only_cfg(types::SynAttributeRef::from(attr)).get()),
        syn::Item::ExternCrate(item) => item
            .attrs
            .iter()
            .any(|attr| attr_is_test_only_cfg(types::SynAttributeRef::from(attr)).get()),
        syn::Item::Fn(item) => item
            .attrs
            .iter()
            .any(|attr| attr_is_test_only_cfg(types::SynAttributeRef::from(attr)).get()),
        syn::Item::ForeignMod(item) => item
            .attrs
            .iter()
            .any(|attr| attr_is_test_only_cfg(types::SynAttributeRef::from(attr)).get()),
        syn::Item::Impl(item) => item
            .attrs
            .iter()
            .any(|attr| attr_is_test_only_cfg(types::SynAttributeRef::from(attr)).get()),
        syn::Item::Macro(item) => item
            .attrs
            .iter()
            .any(|attr| attr_is_test_only_cfg(types::SynAttributeRef::from(attr)).get()),
        syn::Item::Mod(item) => item
            .attrs
            .iter()
            .any(|attr| attr_is_test_only_cfg(types::SynAttributeRef::from(attr)).get()),
        syn::Item::Static(item) => item
            .attrs
            .iter()
            .any(|attr| attr_is_test_only_cfg(types::SynAttributeRef::from(attr)).get()),
        syn::Item::Struct(item) => item
            .attrs
            .iter()
            .any(|attr| attr_is_test_only_cfg(types::SynAttributeRef::from(attr)).get()),
        syn::Item::Trait(item) => item
            .attrs
            .iter()
            .any(|attr| attr_is_test_only_cfg(types::SynAttributeRef::from(attr)).get()),
        syn::Item::TraitAlias(item) => item
            .attrs
            .iter()
            .any(|attr| attr_is_test_only_cfg(types::SynAttributeRef::from(attr)).get()),
        syn::Item::Type(item) => item
            .attrs
            .iter()
            .any(|attr| attr_is_test_only_cfg(types::SynAttributeRef::from(attr)).get()),
        syn::Item::Union(item) => item
            .attrs
            .iter()
            .any(|attr| attr_is_test_only_cfg(types::SynAttributeRef::from(attr)).get()),
        syn::Item::Use(item) => item
            .attrs
            .iter()
            .any(|attr| attr_is_test_only_cfg(types::SynAttributeRef::from(attr)).get()),
        syn::Item::Verbatim(_) | _ => false,
    })
}
#[allow(clippy::single_call_fn)] // accepts cfg(test) and cfg(feature = "test-utils") as non-production code
fn attr_is_test_only_cfg(attr: types::SynAttributeRef<'_>) -> types::AnalyzerBool {
    let attr_ref = attr.as_ref();
    if !attr_ref.path().is_ident(str_constants::CFG_ALT) {
        return types::AnalyzerBool::default();
    }
    let mut is_test_only_cfg = false;
    drop(attr_ref.parse_nested_meta(|meta| {
        if meta.path.is_ident(str_constants::TEST_ALT_3) {
            is_test_only_cfg = true;
        }
        if meta.path.is_ident(str_constants::FEATURE) {
            let value = meta.value()?;
            let lit: syn::LitStr = value.parse()?;
            if lit.value() == str_constants::TEST_UTILS {
                is_test_only_cfg = true;
            }
        }
        Ok(())
    }));
    types::AnalyzerBool::from(is_test_only_cfg)
}
#[allow(clippy::single_call_fn)] // shared rust-file reader keeps skip-on-read-error behavior centralized across source policy checks
fn for_each_rs_file_content(mut on_file: impl FnMut(&std::path::Path, &str)) {
    snapshot::with_codebase_snapshot(|snapshot| {
        snapshot
            .rs_files()
            .iter()
            .for_each(|file| on_file(file.path().as_ref(), file.content().as_ref()));
    });
}
#[allow(clippy::single_call_fn)] // shared rust-file parser keeps read+parse flow reusable for AST-based checks and visitors
fn for_each_rs_syn_file(mut on_file: impl FnMut(&std::path::Path, &syn::File)) {
    snapshot::with_codebase_snapshot(|snapshot| {
        snapshot
            .rs_files()
            .iter()
            .for_each(|file| on_file(file.path().as_ref(), file.ast().as_ref()));
    });
}
fn workspace_table_from_cargo_toml() -> types::TomlTable {
    let mut table = std::fs::read_to_string(str_constants::CODE_STYLE_WORKSPACE_MANIFEST_PATH)
        .expect("39a0d238")
        .parse::<toml::Table>()
        .expect("beb11586");
    toml_val_as_table(
        types::TomlValue::from(table.remove(str_constants::WORKSPACE).expect("f728192d")),
        types::StaticStr::from(str_constants::VALUE_2BFB0B62),
    )
}
#[allow(clippy::single_call_fn)] // shared owned-value table extractor keeps table-shape validation reusable where ownership is required
fn toml_val_as_table(v: types::TomlValue, uuid: types::StaticStr) -> types::TomlTable {
    match v.into_inner() {
        toml::Value::Table(t) => types::TomlTable::from(t),
        toml::Value::String(_)
        | toml::Value::Integer(_)
        | toml::Value::Float(_)
        | toml::Value::Boolean(_)
        | toml::Value::Datetime(_)
        | toml::Value::Array(_) => panic!("{}", uuid.get()),
    }
}
fn toml_val_as_table_ref(
    v: types::TomlValueRef<'_>,
    uuid: types::StaticStr,
) -> types::TomlTableRef<'_> {
    match v.get() {
        toml::Value::Table(t) => types::TomlTableRef::from(t),
        toml::Value::String(_)
        | toml::Value::Integer(_)
        | toml::Value::Float(_)
        | toml::Value::Boolean(_)
        | toml::Value::Datetime(_)
        | toml::Value::Array(_) => panic!("{}", uuid.get()),
    }
}
#[allow(clippy::single_call_fn)] // shared collector keeps workspace-dependency policy checks reusable and centralized
fn collect_non_workspace_dep_ers(
    path: types::StdPathRef<'_>,
    parsed: types::TomlTableRef<'_>,
    mut ers: types::DiagnosticMsgsMutRef<'_>,
) {
    ers.extend(
        [
            str_constants::DEPENDENCIES,
            str_constants::DEV_DEPENDENCIES,
            str_constants::BUILD_DEPENDENCIES,
        ]
        .into_iter()
        .filter_map(|dep_section| {
            parsed
                .as_ref()
                .get(dep_section)
                .and_then(toml::Value::as_table)
                .map(|deps| (dep_section, deps))
        })
        .flat_map(|(dep_section, deps)| {
            deps.iter()
                .filter(move |(_, dep_value)| {
                    !workspace_dep_entry_is_valid(types::TomlValueRef::from(*dep_value)).get()
                })
                .map(move |(dep_name, _)| {
                    String::from(workspace_dep_entry_error(
                        path,
                        types::SourceTextRef::from(dep_name.as_str()),
                        types::SourceTextRef::from(dep_section),
                    ))
                })
        }),
    );
}
#[allow(clippy::single_call_fn)] // keeps dependency-policy validation centralized for dependencies/dev-dependencies/build-dependencies checks
fn workspace_dep_entry_is_valid(dep_value: types::TomlValueRef<'_>) -> types::AnalyzerBool {
    types::AnalyzerBool::from(match dep_value.as_ref() {
        toml::Value::Table(dep_table) => {
            dep_table.contains_key(str_constants::PATH_ALT_5)
                || dep_table.get(str_constants::WORKSPACE) == Some(&toml::Value::Boolean(true))
        }
        toml::Value::String(_)
        | toml::Value::Integer(_)
        | toml::Value::Float(_)
        | toml::Value::Boolean(_)
        | toml::Value::Datetime(_)
        | toml::Value::Array(_) => false,
    })
}
#[allow(clippy::single_call_fn)] // shared message builder keeps dependency-policy errors identical across call sites
fn workspace_dep_entry_error(
    path: types::StdPathRef<'_>,
    dep_name: types::SourceTextRef<'_>,
    dep_section: types::SourceTextRef<'_>,
) -> types::SourceText {
    types::SourceText::try_from(format!(
        "{}: dependency `{dep_name}` in [{dep_section}] must use `dep = {{ workspace = true }}` (only `path = ...` is allowed as exception)",
        path.as_ref().display(),
        dep_name = dep_name.as_ref(),
        dep_section = dep_section.as_ref(),
    ))
    .expect("c836ad25")
}
#[allow(clippy::single_call_fn)] // dedicated collector keeps workspace-members existence diagnostics reusable and deterministic with caller-managed sorting
fn collect_workspace_member_missing_cargo_toml_ers(
    members: types::SourceTextListRef<'_>,
) -> types::SourceTextList {
    types::SourceTextList::from(
        members
            .as_ref()
            .iter()
            .filter_map(|member_str| {
                let path = std::path::Path::new(str_constants::TEXT_ALT_8)
                    .join(member_str)
                    .join(str_constants::CARGO_TOML);
                (!path.exists()).then(|| {
                    format!(
                        "member `{member_str}` Cargo.toml not found at {}",
                        path.display()
                    )
                })
            })
            .collect::<Vec<String>>(),
    )
}
#[allow(clippy::single_call_fn)] // central member extraction keeps workspace-members readers strict and reusable across membership checks
fn workspace_members_as_strs(
    workspace: types::TomlTableRef<'_>,
    exp_id: types::StaticStr,
) -> types::SourceTextList {
    let Some(members) = workspace
        .as_ref()
        .get(str_constants::MEMBERS)
        .and_then(toml::Value::as_array)
    else {
        panic!("{}", exp_id.get());
    };
    members
        .iter()
        .map(|member| {
            member
                .as_str()
                .unwrap_or_else(|| panic!("{}", exp_id.get()))
                .to_owned()
        })
        .collect::<Vec<String>>()
        .into()
}
