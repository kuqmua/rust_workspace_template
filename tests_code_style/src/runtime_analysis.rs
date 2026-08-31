#[derive(
    generate_accessor::Getters,
    generate_constructor::New,
    optimal_memory_layout::OptimalMemoryLayout,
)]
pub(super) struct RuntimePanicExpectUnwrapVisitor {
    ers: crate::types::DiagnosticMsgs,
}
impl<'ast> syn::visit::Visit<'ast> for RuntimePanicExpectUnwrapVisitor {
    fn visit_expr_method_call(&mut self, i: &'ast syn::ExprMethodCall) {
        if i.method == constants_str::CODE_STYLE_EXPECT_METHOD_NAME {
            self.ers.push(constants_str::EXPECT_CALL.to_owned());
        }
        if i.method == constants_str::UNWRAP {
            self.ers.push(constants_str::UNWRAP_CALL.to_owned());
        }
        syn::visit::visit_expr_method_call(self, i);
    }
    fn visit_item(&mut self, i: &'ast syn::Item) {
        if crate::code_style::has_test_only_cfg_attr(crate::types::SynItemRef::from(i)).get() {
            return;
        }
        syn::visit::visit_item(self, i);
    }
    fn visit_macro(&mut self, i: &'ast syn::Macro) {
        if i.path
            .segments
            .last()
            .is_some_and(|segment| segment.ident == constants_str::CODE_STYLE_PANIC_METHOD_NAME)
        {
            self.ers.push(constants_str::PANIC_CALL.to_owned());
        }
        syn::visit::visit_macro(self, i);
    }
}
#[derive(
    generate_accessor::Getters,
    generate_constructor::New,
    optimal_memory_layout::OptimalMemoryLayout,
)]
pub(super) struct RuntimeMutexVisitor {
    found_count: crate::types::AnalyzerCount,
}
impl<'ast> syn::visit::Visit<'ast> for RuntimeMutexVisitor {
    fn visit_item(&mut self, i: &'ast syn::Item) {
        if crate::code_style::has_test_only_cfg_attr(crate::types::SynItemRef::from(i)).get() {
            return;
        }
        syn::visit::visit_item(self, i);
    }
    fn visit_type_path(&mut self, i: &'ast syn::TypePath) {
        if crate::code_style::path_has_segment(
            crate::types::SynPathRef::from(&i.path),
            crate::types::SourceTextRef::from(constants_str::MUTEX),
        )
        .get()
        {
            self.found_count.saturating_inc();
        }
        syn::visit::visit_type_path(self, i);
    }
}
#[derive(
    generate_accessor::Getters,
    generate_constructor::New,
    optimal_memory_layout::OptimalMemoryLayout,
)]
// The owner module retains lint-sensitive semantics from the original implementation.
#[allow(clippy::arbitrary_source_item_ordering)] // alignment order required by optimal_memory_layout takes precedence over alphabetical field order
pub(super) struct RuntimeArcVisitor {
    ers: crate::types::DiagnosticMsgs,
    allow_arc_value_usage: crate::types::AnalyzerBool,
}
impl<'ast> syn::visit::Visit<'ast> for RuntimeArcVisitor {
    fn visit_expr_call(&mut self, i: &'ast syn::ExprCall) {
        if crate::code_style::expr_call_path(crate::types::SynExprCallRef::from(i)).is_some_and(
            |path| {
                crate::code_style::path_ends_with(
                    path,
                    crate::types::StaticStrSliceRef::from(
                        [constants_str::ARC, constants_str::NEW].as_slice(),
                    ),
                )
                .get()
            },
        ) && !self.allow_arc_value_usage.get()
        {
            self.ers.push(
                constants_str::ARC_PATH_NEW_OUTSIDE_APPROVED_CROSS_THREAD_STATE_CONSTRUCTION
                    .to_owned(),
            );
        }
        syn::visit::visit_expr_call(self, i);
    }
    fn visit_item(&mut self, i: &'ast syn::Item) {
        if crate::code_style::has_test_only_cfg_attr(crate::types::SynItemRef::from(i)).get() {
            return;
        }
        syn::visit::visit_item(self, i);
    }
    fn visit_item_type(&mut self, i: &'ast syn::ItemType) {
        let contains_arc = match i.ty.as_ref() {
            syn::Type::Path(path) => crate::code_style::path_has_segment(
                crate::types::SynPathRef::from(&path.path),
                crate::types::SourceTextRef::from(constants_str::ARC),
            )
            .get(),
            syn::Type::Array(_)
            | syn::Type::FnPtr(_)
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
        };
        if contains_arc {
            let name = i.ident.to_string();
            if !name.contains(constants_str::SHARED) && !name.contains(constants_str::DYNARC) {
                self.ers.push(format!(
                    "Arc type alias `{name}` must be explicitly named as shared cross-thread state"
                ));
            }
        }
        syn::visit::visit_item_type(self, i);
    }
}
#[derive(
    generate_accessor::Getters,
    generate_constructor::New,
    optimal_memory_layout::OptimalMemoryLayout,
)]
pub(super) struct AsyncBlockingCallVisitor {
    async_fn_depth: crate::types::AnalyzerCount,
    ers: crate::types::DiagnosticMsgs,
}
impl<'ast> syn::visit::Visit<'ast> for AsyncBlockingCallVisitor {
    fn visit_expr_async(&mut self, i: &'ast syn::ExprAsync) {
        self.async_fn_depth.saturating_inc();
        syn::visit::visit_expr_async(self, i);
        self.async_fn_depth.saturating_dec();
    }
    fn visit_expr_call(&mut self, i: &'ast syn::ExprCall) {
        if self.async_fn_depth.get() != 0
            && crate::code_style::expr_call_path(crate::types::SynExprCallRef::from(i)).is_some_and(
                |path| {
                    let path_text = crate::code_style::path_to_string(path);
                    crate::code_style::path_ends_with(
                        path,
                        crate::types::StaticStrSliceRef::from(
                            [
                                constants_str::FUTURES,
                                constants_str::EXECUTOR,
                                constants_str::BLOCK_ON,
                            ]
                            .as_slice(),
                        ),
                    )
                    .get()
                        || crate::code_style::path_ends_with(
                            path,
                            crate::types::StaticStrSliceRef::from(
                                [
                                    constants_str::TOKIO,
                                    constants_str::TASK,
                                    constants_str::BLOCK_IN_PLACE,
                                ]
                                .as_slice(),
                            ),
                        )
                        .get()
                        || crate::code_style::path_ends_with(
                            path,
                            crate::types::StaticStrSliceRef::from(
                                [
                                    constants_str::STD,
                                    constants_str::THREAD,
                                    constants_str::SLEEP,
                                ]
                                .as_slice(),
                            ),
                        )
                        .get()
                        || constants_str::BLOCKING_STD_FS_CALLS.contains(&path_text.as_ref())
                        || constants_str::BLOCKING_STD_NET_CALLS.contains(&path_text.as_ref())
                },
            )
        {
            self.ers
                .push(constants_str::BLOCKING_CALL_INSIDE_ASYNC_FUNCTION.to_owned());
        }
        syn::visit::visit_expr_call(self, i);
    }
    fn visit_expr_closure(&mut self, i: &'ast syn::ExprClosure) {
        let is_async = i.asyncness.is_some();
        if is_async {
            self.async_fn_depth.saturating_inc();
        }
        syn::visit::visit_expr_closure(self, i);
        if is_async {
            self.async_fn_depth.saturating_dec();
        }
    }
    fn visit_expr_method_call(&mut self, i: &'ast syn::ExprMethodCall) {
        let method = i.method.to_string();
        if self.async_fn_depth.get() != 0
            && matches!(
                method.as_str(),
                constants_str::BLOCK_ON
                    | constants_str::BLOCK_IN_PLACE
                    | constants_str::BLOCKING_RECV
                    | constants_str::BLOCKING_SEND
            )
        {
            self.ers.push(format!(
                ".{}() blocking method call inside async function",
                i.method
            ));
        }
        syn::visit::visit_expr_method_call(self, i);
    }
    fn visit_impl_item_fn(&mut self, i: &'ast syn::ImplItemFn) {
        let is_async = i.sig.asyncness.is_some();
        if is_async {
            self.async_fn_depth.saturating_inc();
        }
        syn::visit::visit_impl_item_fn(self, i);
        if is_async {
            self.async_fn_depth.saturating_dec();
        }
    }
    fn visit_item(&mut self, i: &'ast syn::Item) {
        if crate::code_style::has_test_only_cfg_attr(crate::types::SynItemRef::from(i)).get() {
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
    fn visit_trait_item_fn(&mut self, i: &'ast syn::TraitItemFn) {
        let is_async = i.sig.asyncness.is_some();
        if is_async {
            self.async_fn_depth.saturating_inc();
        }
        syn::visit::visit_trait_item_fn(self, i);
        if is_async {
            self.async_fn_depth.saturating_dec();
        }
    }
}
#[derive(
    generate_accessor::Getters,
    generate_constructor::New,
    optimal_memory_layout::OptimalMemoryLayout,
)]
pub(super) struct UnitTestExternalServiceVisitor {
    ers: crate::types::DiagnosticMsgs,
    test_depth: crate::types::AnalyzerCount,
}
impl<'ast> syn::visit::Visit<'ast> for UnitTestExternalServiceVisitor {
    fn visit_expr_method_call(&mut self, i: &'ast syn::ExprMethodCall) {
        if self.test_depth.get() != 0
            && matches!(
                i.method.to_string().as_str(),
                constants_str::CONNECT | constants_str::VALUE_3DFFA238
            )
        {
            self.ers.push(format!(
                "unit tests must not connect to an external service with `.{}()`",
                i.method
            ));
        }
        syn::visit::visit_expr_method_call(self, i);
    }
    fn visit_expr_path(&mut self, i: &'ast syn::ExprPath) {
        let path = crate::types::SynPathRef::from(&i.path);
        let path_text = crate::code_style::path_to_string(path);
        let is_external_service_client = [
            [
                constants_str::REQWEST,
                constants_str::CLIENT,
                constants_str::NEW,
            ]
            .as_slice(),
            [
                constants_str::STD,
                constants_str::NET,
                constants_str::TCPSTREAM,
                constants_str::CONNECT,
            ]
            .as_slice(),
            [
                constants_str::STD,
                constants_str::NET,
                constants_str::TCPLISTENER,
                constants_str::BIND,
            ]
            .as_slice(),
            [
                constants_str::STD,
                constants_str::NET,
                constants_str::UDPSOCKET,
                constants_str::BIND,
            ]
            .as_slice(),
            [
                constants_str::TOKIO,
                constants_str::NET,
                constants_str::TCPSTREAM,
                constants_str::CONNECT,
            ]
            .as_slice(),
            [
                constants_str::TOKIO,
                constants_str::NET,
                constants_str::TCPLISTENER,
                constants_str::BIND,
            ]
            .as_slice(),
            [
                constants_str::TOKIO,
                constants_str::NET,
                constants_str::UDPSOCKET,
                constants_str::BIND,
            ]
            .as_slice(),
        ]
        .into_iter()
        .any(|segments| {
            crate::code_style::path_ends_with(path, crate::types::StaticStrSliceRef::from(segments))
                .get()
        }) || [
            constants_str::VALUE_364F9D39,
            constants_str::VALUE_BDB563EC,
            constants_str::VALUE_FE4D84FC,
            constants_str::VALUE_2FCCA7C7,
        ]
        .contains(&path_text.as_ref());
        if self.test_depth.get() != 0 && is_external_service_client {
            self.ers.push(format!(
                "unit tests must not depend on external service client `{}`",
                crate::code_style::path_to_string(crate::types::SynPathRef::from(&i.path)).as_ref()
            ));
        }
        syn::visit::visit_expr_path(self, i);
    }
    fn visit_item_fn(&mut self, i: &'ast syn::ItemFn) {
        if i.attrs.iter().any(|attribute| {
            attribute.path().is_ident(constants_str::VALUE_5F0AF516)
                && matches!(
                    &attribute.meta,
                    syn::Meta::NameValue(name_value)
                        if matches!(
                            &name_value.value,
                            syn::Expr::Lit(syn::ExprLit {
                                lit: syn::Lit::Str(reason),
                                ..
                            }) if !reason.value().trim().is_empty()
                        )
                )
        }) {
            return;
        }
        let is_test = self.test_depth.get() != 0
            || crate::code_style::item_fn_is_unit_test(crate::types::SynItemFnRef::from(i)).get();
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
            || i.attrs.iter().any(|attr| {
                crate::code_style::attr_is_test_only_cfg(crate::types::SynAttributeRef::from(attr))
                    .get()
            });
        if is_test {
            self.test_depth.saturating_inc();
        }
        syn::visit::visit_item_mod(self, i);
        if is_test {
            self.test_depth.saturating_dec();
        }
    }
}
