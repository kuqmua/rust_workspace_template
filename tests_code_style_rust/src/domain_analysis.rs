#[derive(
    proc_macro_getters::Getters,
    proc_macro_new::New,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
)]
pub(super) struct StringWrapperNameVisitor {
    names: crate::types::SourceTextBTreeSet,
}
impl<'ast> syn::visit::Visit<'ast> for StringWrapperNameVisitor {
    fn visit_item_struct(&mut self, item_struct: &'ast syn::ItemStruct) {
        if crate::code_style::item_struct_is_single_string_wrapper(
            crate::types::SynItemStructRef::from(item_struct),
        )
        .get()
        {
            let _: bool = self.names.insert(item_struct.ident.to_string());
        }
        syn::visit::visit_item_struct(self, item_struct);
    }
}
#[derive(
    proc_macro_getters::Getters,
    proc_macro_new::New,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
)]
pub(super) struct BoundedStringStorageVisitor {
    errors: crate::types::DiagnosticMessages,
}
impl<'ast> syn::visit::Visit<'ast> for BoundedStringStorageVisitor {
    fn visit_item_struct(&mut self, item_struct: &'ast syn::ItemStruct) {
        let has_bounded_string_attr = item_struct
            .attrs
            .iter()
            .any(|attr| attr.path().is_ident(constants_str::BOUNDED_STRING));
        let derives_old_bounded_string = item_struct.attrs.iter().any(|attr| {
            if !attr.path().is_ident(constants_str::DERIVE) {
                return false;
            }
            let Ok(derive_paths) = attr.parse_args_with(
                syn::punctuated::Punctuated::<syn::Path, syn::Token![,]>::parse_terminated,
            ) else {
                return false;
            };
            derive_paths.iter().any(|path| {
                path.segments.len() == 2
                    && path
                        .segments
                        .first()
                        .is_some_and(|segment| segment.ident == stringify!(proc_macro_newtype))
                    && path
                        .segments
                        .last()
                        .is_some_and(|segment| segment.ident == constants_str::BOUNDEDSTRING)
            })
        });
        if derives_old_bounded_string {
            self.errors.push(format!(
                "`{}` derives removed `proc_macro_newtype::BoundedString`; store `bounded_types::bounded_string::BoundedString` instead",
                item_struct.ident
            ));
        }
        if has_bounded_string_attr {
            let stores_bounded_string = match &item_struct.fields {
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
            if !stores_bounded_string {
                self.errors.push(format!(
                    "`{}` uses `#[bounded_string]` but does not store `bounded_types::bounded_string::BoundedString`",
                    item_struct.ident
                ));
            }
        }
        syn::visit::visit_item_struct(self, item_struct);
    }
}
#[derive(
    proc_macro_getters::Getters,
    proc_macro_new::New,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
)]
pub(super) struct StringWrapperFromVisitor<'names_lt> {
    errors: crate::types::DiagnosticMessages,
    len_checked_function_names: &'names_lt crate::types::SourceTextBTreeSet,
    string_wrapper_names: &'names_lt crate::types::SourceTextBTreeSet,
    try_from_string_len_checked_names: crate::types::SourceTextBTreeSet,
    try_from_string_names: crate::types::SourceTextBTreeSet,
}
impl StringWrapperFromVisitor<'_> {
    fn check_bounded_string_attr(
        &mut self,
        syn_item_struct_ref: crate::types::SynItemStructRef<'_>,
    ) {
        let item_ref = syn_item_struct_ref.as_ref();
        if !crate::code_style::item_struct_is_single_string_wrapper(syn_item_struct_ref).get() {
            return;
        }
        let stores_bounded_string = match &item_ref.fields {
            syn::Fields::Unnamed(fields) => fields.unnamed.first().is_some_and(|field| {
                crate::code_style::type_path_ends_with_identifier(
                    crate::types::SynTypeRef::from(&field.ty),
                    crate::types::SourceTextRef::from(constants_str::BOUNDEDSTRING),
                )
                .get()
            }),
            syn::Fields::Named(_) | syn::Fields::Unit => false,
        };
        if stores_bounded_string {
            let _: bool = self
                .try_from_string_len_checked_names
                .insert(item_ref.ident.to_string());
        }
        let has_derive = item_ref.attrs.iter().any(|attr| {
            attr.path().is_ident(constants_str::DERIVE)
                && attr.meta.require_list().is_ok_and(|list| {
                    list.tokens
                        .to_string()
                        .contains(constants_str::BOUNDEDSTRING)
                })
        });
        let has_max_bound = item_ref.attrs.iter().any(|attr| {
            if !attr.path().is_ident(constants_str::BOUNDED_STRING) {
                return false;
            }
            let mut has_max = false;
            drop(attr.parse_nested_meta(|meta| {
                if meta.path.is_ident(constants_str::MAX) {
                    drop(meta.value()?.parse::<syn::Expr>()?);
                    has_max = true;
                    return Ok(());
                }
                Err(meta.error(constants_str::UNKNOWN_BOUNDED_STRING_OPTION))
            }));
            has_max
        });
        if has_derive && has_max_bound {
            let identifier = item_ref.ident.to_string();
            let _: bool = self.try_from_string_names.insert(identifier.clone());
            let _: bool = self.try_from_string_len_checked_names.insert(identifier);
        }
    }
    fn check_from_impl(&mut self, syn_item_impl_ref: crate::types::SynItemImplRef<'_>) {
        let is_from_string = crate::types::AnalyzerBool::from(
            syn_item_impl_ref
                .as_ref()
                .trait_
                .as_ref()
                .is_some_and(|(path, _)| {
                    crate::code_style::path_ends_with(
                        crate::types::SynPathRef::from(path),
                        crate::types::StaticStrSliceRef::from(
                            [constants_str::FROM_ALT_3].as_slice(),
                        ),
                    )
                    .get()
                        && crate::code_style::from_trait_arg_is_string(
                            crate::types::SynPathRef::from(path),
                        )
                        .get()
                }),
        );
        if !is_from_string.get() {
            return;
        }
        let identifier = crate::code_style::item_impl_self_ty_identifier(syn_item_impl_ref)
            .map_or_else(
                || String::from(constants_str::NON_PATH_TARGET),
                String::from,
            );
        self.errors.push(format!(
            "`{identifier}` implements `From<String>`; implement `TryFrom<String>` instead"
        ));
    }
    fn check_newtype_attr(&mut self, syn_item_struct_ref: crate::types::SynItemStructRef<'_>) {
        let item_ref = syn_item_struct_ref.as_ref();
        if !crate::code_style::item_struct_is_single_string_wrapper(syn_item_struct_ref).get() {
            return;
        }
        if item_ref.attrs.iter().any(|attr| {
            attr.path().is_ident(constants_str::DERIVE)
                && attr.meta.require_list().is_ok_and(|list| {
                    list.tokens
                        .to_string()
                        .contains(constants_str::NEWTYPE_FROM_INNER_DERIVE_NAME)
                })
        }) {
            self.errors.push(format!(
                        "string wrapper `{}` derives `proc_macro_newtype_from_inner::FromInner`; derive `proc_macro_newtype_try_from::TryFrom` with a length check instead",
                        item_ref.ident
                    ));
        }
        let has_try_from = item_ref.attrs.iter().any(|attr| {
            attr.path().is_ident(constants_str::DERIVE)
                && attr.meta.require_list().is_ok_and(|list| {
                    list.tokens
                        .to_string()
                        .contains(constants_str::NEWTYPE_TRY_FROM_DERIVE_NAME)
                })
        });
        let mut has_len_check = false;
        item_ref
            .attrs
            .iter()
            .filter(|attr| attr.path().is_ident(constants_str::NEWTYPE_TRY_FROM))
            .for_each(|attr| {
                drop(attr.parse_nested_meta(|meta| {
                    if meta.path.is_ident(constants_str::NEWTYPE_TRY_FROM_ERROR) {
                        let _error_type = meta.value()?.parse::<syn::Type>()?;
                        return Ok(());
                    }
                    if meta
                        .path
                        .is_ident(constants_str::NEWTYPE_TRY_FROM_VALIDATOR)
                    {
                        let expr = meta.value()?.parse::<syn::Expr>()?;
                        let mut visitor = LenMethodCallVisitor {
                            found: crate::types::AnalyzerBool::default(),
                        };
                        syn::visit::Visit::visit_expr(&mut visitor, &expr);
                        let path_is_len_checked = match &expr {
                            syn::Expr::Path(path) => {
                                let full_path = crate::code_style::path_to_string(
                                    crate::types::SynPathRef::from(&path.path),
                                );
                                self.len_checked_function_names.contains(full_path.as_ref())
                                    || path.path.segments.last().is_some_and(|segment| {
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
    fn check_try_from_impl(&mut self, syn_item_impl_ref: crate::types::SynItemImplRef<'_>) {
        let is_try_from_string = crate::types::AnalyzerBool::from(
            syn_item_impl_ref
                .as_ref()
                .trait_
                .as_ref()
                .is_some_and(|(path, _)| {
                    crate::code_style::path_ends_with(
                        crate::types::SynPathRef::from(path),
                        crate::types::StaticStrSliceRef::from([constants_str::TRYFROM].as_slice()),
                    )
                    .get()
                        && crate::code_style::from_trait_arg_is_string(
                            crate::types::SynPathRef::from(path),
                        )
                        .get()
                }),
        );
        if !is_try_from_string.get() {
            return;
        }
        let Some(identifier) = crate::code_style::item_impl_self_ty_identifier(syn_item_impl_ref)
        else {
            return;
        };
        if !self.string_wrapper_names.contains(identifier.as_ref()) {
            return;
        }
        let _: bool = self
            .try_from_string_names
            .insert(String::from(identifier.clone()));
        let mut len_call_visitor = LenMethodCallVisitor {
            found: crate::types::AnalyzerBool::default(),
        };
        syn::visit::Visit::visit_item_impl(&mut len_call_visitor, syn_item_impl_ref.as_ref());
        let mut len_checked_call_visitor = LenCheckedFunctionCallVisitor {
            found: crate::types::AnalyzerBool::default(),
            names: self.len_checked_function_names,
        };
        syn::visit::Visit::visit_item_impl(
            &mut len_checked_call_visitor,
            syn_item_impl_ref.as_ref(),
        );
        if len_call_visitor.found.get() || len_checked_call_visitor.found.get() {
            let _: bool = self
                .try_from_string_len_checked_names
                .insert(String::from(identifier));
        }
    }
}
#[derive(
    proc_macro_getters::Getters,
    proc_macro_new::New,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
)]
pub(super) struct LenCheckedFunctionCallVisitor<'names_lt> {
    found: crate::types::AnalyzerBool,
    names: &'names_lt crate::types::SourceTextBTreeSet,
}
impl<'ast> syn::visit::Visit<'ast> for LenCheckedFunctionCallVisitor<'_> {
    fn visit_expr_call(&mut self, expr_call: &'ast syn::ExprCall) {
        if let syn::Expr::Path(path) = expr_call.func.as_ref() {
            let full_path =
                crate::code_style::path_to_string(crate::types::SynPathRef::from(&path.path));
            if self.names.iter().any(|name| {
                full_path.as_ref() == name
                    || full_path
                        .as_ref()
                        .strip_suffix(name)
                        .is_some_and(|prefix| prefix.ends_with(constants_str::PATH_SEPARATOR))
            }) {
                self.found.set_true();
            }
        }
        syn::visit::visit_expr_call(self, expr_call);
    }
}
#[derive(
    proc_macro_getters::Getters,
    proc_macro_new::New,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
)]
pub(super) struct LenCheckedFunctionNameVisitor {
    names: crate::types::SourceTextBTreeSet,
}
impl<'ast> syn::visit::Visit<'ast> for LenCheckedFunctionNameVisitor {
    fn visit_item_fn(&mut self, item_fn: &'ast syn::ItemFn) {
        let mut visitor = LenMethodCallVisitor {
            found: crate::types::AnalyzerBool::default(),
        };
        syn::visit::Visit::visit_block(&mut visitor, &item_fn.block);
        if visitor.found.get() {
            let _: bool = self.names.insert(item_fn.sig.ident.to_string());
        }
        syn::visit::visit_item_fn(self, item_fn);
    }
    fn visit_item_impl(&mut self, item_impl: &'ast syn::ItemImpl) {
        let Some(type_name) = crate::code_style::item_impl_self_ty_identifier(
            crate::types::SynItemImplRef::from(item_impl),
        ) else {
            syn::visit::visit_item_impl(self, item_impl);
            return;
        };
        item_impl.items.iter().for_each(|item| {
            let syn::ImplItem::Fn(method) = item else {
                return;
            };
            let mut visitor = LenMethodCallVisitor {
                found: crate::types::AnalyzerBool::default(),
            };
            syn::visit::Visit::visit_block(&mut visitor, &method.block);
            if visitor.found.get() {
                let _: bool =
                    self.names
                        .insert(format!("{}::{}", type_name.as_ref(), method.sig.ident));
            }
        });
        syn::visit::visit_item_impl(self, item_impl);
    }
}
impl<'ast> syn::visit::Visit<'ast> for StringWrapperFromVisitor<'_> {
    fn visit_item_impl(&mut self, item_impl: &'ast syn::ItemImpl) {
        self.check_from_impl(crate::types::SynItemImplRef::from(item_impl));
        self.check_try_from_impl(crate::types::SynItemImplRef::from(item_impl));
        syn::visit::visit_item_impl(self, item_impl);
    }
    fn visit_item_struct(&mut self, item_struct: &'ast syn::ItemStruct) {
        syn::visit::visit_item_struct(self, item_struct);
        self.check_bounded_string_attr(crate::types::SynItemStructRef::from(item_struct));
        self.check_newtype_attr(crate::types::SynItemStructRef::from(item_struct));
    }
}
#[derive(
    proc_macro_getters::Getters,
    proc_macro_new::New,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
)]
pub(super) struct LenMethodCallVisitor {
    found: crate::types::AnalyzerBool,
}
impl<'ast> syn::visit::Visit<'ast> for LenMethodCallVisitor {
    fn visit_expr_method_call(&mut self, expr_method_call: &'ast syn::ExprMethodCall) {
        if expr_method_call.method == constants_str::LEN {
            self.found.set_true();
        }
        syn::visit::visit_expr_method_call(self, expr_method_call);
    }
}
#[derive(
    proc_macro_getters::Getters,
    proc_macro_new::New,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
)]
pub(super) struct PublicTupleWrapperFieldVisitor {
    errors: crate::types::DiagnosticMessages,
}
#[derive(
    proc_macro_getters::Getters,
    proc_macro_new::New,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
)]
pub(super) struct DirectDeserializeTupleWrapperVisitor {
    errors: crate::types::DiagnosticMessages,
}
#[derive(
    proc_macro_getters::Getters,
    proc_macro_new::New,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
)]
pub(super) struct DeserializeConversionCallVisitor {
    found: crate::types::AnalyzerBool,
}
#[derive(
    proc_macro_getters::Getters,
    proc_macro_new::New,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
)]
pub(super) struct ManualDeserializeTupleWrapperVisitor<'names> {
    errors: crate::types::DiagnosticMessages,
    names: &'names crate::types::SourceTextBTreeSet,
}
#[derive(
    proc_macro_getters::Getters,
    proc_macro_new::New,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
)]
pub(super) struct TupleWrapperConversionCollector {
    converted_names: crate::types::SourceTextBTreeSet,
    from_inner_names: crate::types::SourceTextBTreeSet,
    from_names: crate::types::SourceTextBTreeSet,
    inner_types: std::collections::BTreeMap<String, syn::Type>,
    names: crate::types::SourceTextBTreeSet,
    try_from_inner_names: crate::types::SourceTextBTreeSet,
    try_from_names: crate::types::SourceTextBTreeSet,
}
#[derive(
    proc_macro_getters::Getters,
    proc_macro_new::New,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
)]
pub(super) struct DirectTupleWrapperConstructorVisitor<'names> {
    current_wrapper_name: Option<String>,
    errors: crate::types::DiagnosticMessages,
    inside_conversion_impl: crate::types::AnalyzerBool,
    names: &'names crate::types::SourceTextBTreeSet,
}
impl<'ast> syn::visit::Visit<'ast> for PublicTupleWrapperFieldVisitor {
    fn visit_item_struct(&mut self, item_struct: &'ast syn::ItemStruct) {
        let inner_field_is_non_private = match &item_struct.fields {
            syn::Fields::Unnamed(fields) if fields.unnamed.len() == 1usize => fields
                .unnamed
                .first()
                .is_some_and(|field| matches!(field.vis, syn::Visibility::Public(_))),
            syn::Fields::Named(_) | syn::Fields::Unnamed(_) | syn::Fields::Unit => false,
        };
        if crate::code_style::item_struct_is_single_field_tuple_wrapper(
            crate::types::SynItemStructRef::from(item_struct),
        )
        .get()
            && inner_field_is_non_private
        {
            self.errors.push(format!(
                "tuple wrapper `{}` exposes its inner field; make the field private and initialize through From/TryFrom",
                item_struct.ident
                    ));
        }
        syn::visit::visit_item_struct(self, item_struct);
    }
}
impl<'ast> syn::visit::Visit<'ast> for DirectDeserializeTupleWrapperVisitor {
    fn visit_item_struct(&mut self, item_struct: &'ast syn::ItemStruct) {
        let derives_deserialize = item_struct.attrs.iter().any(|attr| {
            attr.path().is_ident(constants_str::DERIVE)
                && match &attr.meta {
                    syn::Meta::List(list) => list
                        .tokens
                        .to_string()
                        .contains(constants_str::CODE_STYLE_DESERIALIZE_DERIVE_NAME),
                    syn::Meta::NameValue(_) | syn::Meta::Path(_) => false,
                }
        });
        let deserialize_uses_conversion = item_struct.attrs.iter().any(|attr| {
            if !attr.path().is_ident(constants_str::SERDE) {
                return false;
            }
            match &attr.meta {
                syn::Meta::List(list) => {
                    let tokens = list.tokens.to_string();
                    tokens.contains(constants_str::CODE_STYLE_SERDE_FROM_ATTR_FRAGMENT)
                        || tokens.contains(constants_str::CODE_STYLE_SERDE_TRY_FROM_ATTR_FRAGMENT)
                }
                syn::Meta::NameValue(_) | syn::Meta::Path(_) => false,
            }
        });
        if crate::code_style::item_struct_is_single_field_tuple_wrapper(
            crate::types::SynItemStructRef::from(item_struct),
        )
        .get()
            && derives_deserialize
            && !deserialize_uses_conversion
        {
            let start = syn::spanned::Spanned::span(item_struct).start();
            self.errors.push(format!(
                "tuple wrapper `{}` derives Deserialize directly at {}:{}; this lets serde construct the inner field without using the wrapper's required conversion path and can bypass validation or other construction invariants. Deserialize a raw value through `#[serde(from = \"RawType\")]` or `#[serde(try_from = \"RawType\")]` so construction finishes in From/TryFrom",
                item_struct.ident, start.line, start.column
            ));
        }
        syn::visit::visit_item_struct(self, item_struct);
    }
}
impl<'ast> syn::visit::Visit<'ast> for DeserializeConversionCallVisitor {
    fn visit_expr_call(&mut self, expr_call: &'ast syn::ExprCall) {
        if let syn::Expr::Path(path) = expr_call.func.as_ref()
            && path.path.segments.last().is_some_and(|segment| {
                segment.ident == constants_str::FROM_ALT_4
                    || segment.ident == constants_str::NEWTYPE_TRY_FROM
            })
        {
            self.found.set_true();
        }
        syn::visit::visit_expr_call(self, expr_call);
    }
}
impl<'ast> syn::visit::Visit<'ast> for ManualDeserializeTupleWrapperVisitor<'_> {
    fn visit_item_impl(&mut self, item_impl: &'ast syn::ItemImpl) {
        let is_deserialize_impl = item_impl.trait_.as_ref().is_some_and(|(path, _)| {
            path.segments.last().is_some_and(|segment| {
                segment.ident == constants_str::CODE_STYLE_DESERIALIZE_DERIVE_NAME
            })
        });
        let Some(name) = crate::code_style::item_impl_self_ty_identifier(
            crate::types::SynItemImplRef::from(item_impl),
        ) else {
            syn::visit::visit_item_impl(self, item_impl);
            return;
        };
        if is_deserialize_impl && self.names.contains(name.as_ref()) {
            let mut visitor = DeserializeConversionCallVisitor {
                found: crate::types::AnalyzerBool::default(),
            };
            syn::visit::visit_item_impl(&mut visitor, item_impl);
            if !visitor.found.get() {
                let start = syn::spanned::Spanned::span(item_impl).start();
                self.errors.push(format!(
                    "tuple wrapper `{}` implements Deserialize without an explicit From/TryFrom call at {}:{}; returning the wrapper through another construction path can bypass validation or other invariants. Deserialize into a raw type, then finish with `Self::from(raw)` or `Self::try_from(raw)` and map conversion errors with `serde::de::Error::custom`",
                    name.as_ref(), start.line, start.column
                ));
            }
        }
        syn::visit::visit_item_impl(self, item_impl);
    }
}
impl<'ast> syn::visit::Visit<'ast> for TupleWrapperConversionCollector {
    fn visit_item_impl(&mut self, item_impl: &'ast syn::ItemImpl) {
        let item_ref = crate::types::SynItemImplRef::from(item_impl);
        let Some(name) = crate::code_style::item_impl_self_ty_identifier(item_ref) else {
            syn::visit::visit_item_impl(self, item_impl);
            return;
        };
        let is_from = crate::types::AnalyzerBool::from(
            item_ref.as_ref().trait_.as_ref().is_some_and(|(path, _)| {
                path.segments
                    .last()
                    .is_some_and(|segment| segment.ident == constants_str::FROM_ALT_3)
            }),
        );
        if is_from.get() {
            let _: bool = self.from_names.insert(name.as_ref().to_owned());
            if let Some(inner_type) = self.inner_types.get(name.as_ref())
                && crate::code_style::item_impl_input_type_is(item_ref, inner_type).get()
            {
                let _: bool = self.from_inner_names.insert(name.as_ref().to_owned());
            }
        }
        let is_try_from = crate::types::AnalyzerBool::from(
            item_ref.as_ref().trait_.as_ref().is_some_and(|(path, _)| {
                path.segments
                    .last()
                    .is_some_and(|segment| segment.ident == constants_str::TRYFROM)
            }),
        );
        if is_try_from.get() {
            let _: bool = self.try_from_names.insert(name.as_ref().to_owned());
            if let Some(inner_type) = self.inner_types.get(name.as_ref())
                && crate::code_style::item_impl_input_type_is(item_ref, inner_type).get()
            {
                let _: bool = self.try_from_inner_names.insert(name.as_ref().to_owned());
            }
        }
        if crate::code_style::item_impl_is_from_or_try_from(item_ref).get() {
            let _: bool = self.converted_names.insert(name.as_ref().to_owned());
        }
        syn::visit::visit_item_impl(self, item_impl);
    }
    fn visit_item_struct(&mut self, item_struct: &'ast syn::ItemStruct) {
        if crate::code_style::item_struct_is_single_field_tuple_wrapper(
            crate::types::SynItemStructRef::from(item_struct),
        )
        .get()
        {
            let name = item_struct.ident.to_string();
            let _: bool = self.names.insert(name.clone());
            if let syn::Fields::Unnamed(fields) = &item_struct.fields
                && fields.unnamed.len() == constants_usize::ONE
                && let Some(field) = fields.unnamed.first()
            {
                drop(self.inner_types.insert(name.clone(), field.ty.clone()));
            }
            let derives_from_inner = item_struct.attrs.iter().any(|attr| {
                if !attr.path().is_ident(constants_str::DERIVE) {
                    return false;
                }
                match &attr.meta {
                    syn::Meta::List(list) => list
                        .tokens
                        .to_string()
                        .contains(constants_str::NEWTYPE_FROM_INNER_DERIVE_NAME),
                    syn::Meta::NameValue(_) | syn::Meta::Path(_) => false,
                }
            });
            if derives_from_inner {
                let _: bool = self.from_names.insert(name.clone());
                let _: bool = self.from_inner_names.insert(name.clone());
            }
            let derives_from_getter = item_struct.attrs.iter().any(|attr| {
                if !attr.path().is_ident(constants_str::DERIVE) {
                    return false;
                }
                match &attr.meta {
                    syn::Meta::List(list) => list
                        .tokens
                        .to_string()
                        .contains(constants_str::NEWTYPE_FROM_GETTER_DERIVE_NAME),
                    syn::Meta::NameValue(_) | syn::Meta::Path(_) => false,
                }
            });
            if derives_from_getter {
                let _: bool = self.from_names.insert(name.clone());
            }
            let derives_try_from = item_struct.attrs.iter().any(|attr| {
                if !attr.path().is_ident(constants_str::DERIVE) {
                    return false;
                }
                match &attr.meta {
                    syn::Meta::List(list) => {
                        let tokens = list.tokens.to_string();
                        tokens.contains(constants_str::NEWTYPE_TRY_FROM_DERIVE_NAME)
                            || tokens.contains(constants_str::BOUNDEDSTRING)
                            || tokens.contains(constants_str::TRYFROM)
                    }
                    syn::Meta::NameValue(_) | syn::Meta::Path(_) => false,
                }
            });
            if derives_try_from {
                let _: bool = self.try_from_names.insert(name.clone());
                let _: bool = self.try_from_inner_names.insert(name.clone());
            }
            let derives_conversion = item_struct.attrs.iter().any(|attr| {
                if !attr.path().is_ident(constants_str::DERIVE) {
                    return false;
                }
                match &attr.meta {
                    syn::Meta::List(list) => {
                        let tokens = list.tokens.to_string();
                        tokens.contains(constants_str::NEWTYPE_FROM_INNER_DERIVE_NAME)
                            || tokens.contains(constants_str::NEWTYPE_FROM_GETTER_DERIVE_NAME)
                            || tokens.contains(constants_str::BOUNDEDSTRING)
                            || tokens.contains(constants_str::TRYFROM)
                    }
                    syn::Meta::NameValue(_) | syn::Meta::Path(_) => false,
                }
            });
            if derives_conversion {
                let _: bool = self.converted_names.insert(name);
            }
        }
        syn::visit::visit_item_struct(self, item_struct);
    }
}
impl<'ast> syn::visit::Visit<'ast> for DirectTupleWrapperConstructorVisitor<'_> {
    fn visit_expr_call(&mut self, expr_call: &'ast syn::ExprCall) {
        if !self.inside_conversion_impl.get()
            && let syn::Expr::Path(path) = expr_call.func.as_ref()
            && let Some(segment) = path.path.segments.last()
            && (self.names.contains(segment.ident.to_string().as_str())
                || (segment.ident == constants_str::SELF && self.current_wrapper_name.is_some()))
        {
            let span = syn::spanned::Spanned::span(expr_call.func.as_ref());
            let start = span.start();
            let end = span.end();
            let wrapper_name = self
                .current_wrapper_name
                .as_deref()
                .filter(|_| segment.ident == constants_str::SELF)
                .map_or_else(|| segment.ident.to_string(), str::to_owned);
            self.errors.push(format!(
                "tuple wrapper `{}` is initialized directly at {}:{}-{}:{}; use From/TryFrom",
                wrapper_name, start.line, start.column, end.line, end.column
            ));
        }
        syn::visit::visit_expr_call(self, expr_call);
    }
    fn visit_item_impl(&mut self, item_impl: &'ast syn::ItemImpl) {
        let previous = self.inside_conversion_impl;
        let previous_wrapper_name = self.current_wrapper_name.take();
        self.inside_conversion_impl = crate::code_style::item_impl_is_from_or_try_from(
            crate::types::SynItemImplRef::from(item_impl),
        );
        self.current_wrapper_name = crate::code_style::item_impl_self_ty_identifier(
            crate::types::SynItemImplRef::from(item_impl),
        )
        .map(|name| name.as_ref().to_owned())
        .filter(|name| self.names.contains(name));
        syn::visit::visit_item_impl(self, item_impl);
        self.inside_conversion_impl = previous;
        self.current_wrapper_name = previous_wrapper_name;
    }
}
#[derive(
    proc_macro_getters::Getters,
    proc_macro_new::New,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
)]
pub(super) struct DeclaredDomainTypeVisitor {
    names: crate::types::SourceTextBTreeSet,
}
impl<'ast> syn::visit::Visit<'ast> for DeclaredDomainTypeVisitor {
    fn visit_item(&mut self, item: &'ast syn::Item) {
        if crate::code_style::has_test_only_cfg_attr(crate::types::SynItemRef::from(item)).get() {
            return;
        }
        syn::visit::visit_item(self, item);
    }
    fn visit_item_enum(&mut self, item_enum: &'ast syn::ItemEnum) {
        let _: bool = self.names.insert(item_enum.ident.to_string());
        syn::visit::visit_item_enum(self, item_enum);
    }
    fn visit_item_struct(&mut self, item_struct: &'ast syn::ItemStruct) {
        let _: bool = self.names.insert(item_struct.ident.to_string());
        if item_struct.attrs.iter().any(|attr| {
            attr.path().is_ident(constants_str::DERIVE)
                && matches!(
                    &attr.meta,
                    syn::Meta::List(list)
                        if list.tokens.to_string().contains(constants_str::BOUNDEDSTRING)
                )
        }) {
            let mut generated_error_name = item_struct.ident.to_string();
            generated_error_name.push_str(constants_str::TRYFROMSTRINGERROR);
            let _: bool = self.names.insert(generated_error_name);
        }
        syn::visit::visit_item_struct(self, item_struct);
    }
    fn visit_item_trait(&mut self, item_trait: &'ast syn::ItemTrait) {
        let _: bool = self.names.insert(item_trait.ident.to_string());
        syn::visit::visit_item_trait(self, item_trait);
    }
    fn visit_item_union(&mut self, item_union: &'ast syn::ItemUnion) {
        let _: bool = self.names.insert(item_union.ident.to_string());
        syn::visit::visit_item_union(self, item_union);
    }
    fn visit_macro(&mut self, r#macro: &'ast syn::Macro) {
        if crate::code_style::path_ends_with(
            crate::types::SynPathRef::from(&r#macro.path),
            crate::types::StaticStrSliceRef::from(
                [
                    constants_str::CODE_STYLE_GENERATE_PG_TYPES_MACRO_NAME,
                    constants_str::CODE_STYLE_GENERATE_PG_TYPES_MACRO_NAME,
                ]
                .as_slice(),
            ),
        )
        .get()
        {
            let tokens = r#macro.tokens.to_string();
            let pattern = regex::Regex::new(constants_str::A_ZA_Z0_9_PLUS_AS_A_ZA_Z0_9_PLUS)
                .expect(constants_str::DIAGNOSTIC_F4E61B29);
            pattern
                .captures_iter(tokens.as_str())
                .filter_map(|captures| {
                    let base = captures.get(1).map(|element| element.as_str())?;
                    base.split_once(constants_str::AS)
                })
                .for_each(|(prefix, suffix)| {
                    let _: bool = self.names.insert(format!("{prefix}AsNonNull{suffix}"));
                    let _: bool = self
                        .names
                        .insert(format!("Optional{prefix}AsNullable{suffix}"));
                });
        }
        let path = crate::types::SynPathRef::from(&r#macro.path);
        let config_lib_domain_type_macro = crate::code_style::path_ends_with(
            path,
            crate::types::StaticStrSliceRef::from(
                [
                    constants_str::CONFIG_LIB_MACROS,
                    constants_str::IMPL_TRY_FROM_NON_EMPTY_STRING,
                ]
                .as_slice(),
            ),
        )
        .get()
            || crate::code_style::path_ends_with(
                path,
                crate::types::StaticStrSliceRef::from(
                    [
                        constants_str::CONFIG_LIB_MACROS,
                        constants_str::IMPL_TRY_FROM_SECRET_URL,
                    ]
                    .as_slice(),
                ),
            )
            .get()
            || crate::code_style::path_ends_with(
                path,
                crate::types::StaticStrSliceRef::from(
                    [
                        constants_str::CONFIG_LIB_MACROS,
                        constants_str::IMPL_TRY_FROM_PARSE,
                    ]
                    .as_slice(),
                ),
            )
            .get()
            || crate::code_style::path_ends_with(
                path,
                crate::types::StaticStrSliceRef::from(
                    [
                        constants_str::CONFIG_LIB_MACROS,
                        constants_str::IMPL_TRY_FROM_PARSE_STRING_ERROR,
                    ]
                    .as_slice(),
                ),
            )
            .get();
        if config_lib_domain_type_macro {
            crate::code_style::collect_first_macro_identifier_domain_name(
                crate::types::SourceTextRef::from(r#macro.tokens.to_string().as_str()),
                &mut self.names,
            );
        }
        if crate::code_style::path_ends_with(
            crate::types::SynPathRef::from(&r#macro.path),
            crate::types::StaticStrSliceRef::from(
                [constants_str::API_OPERATION_ERROR_MACRO_IDENTIFIER].as_slice(),
            ),
        )
        .get()
        {
            crate::code_style::collect_first_macro_identifier_domain_name(
                crate::types::SourceTextRef::from(r#macro.tokens.to_string().as_str()),
                &mut self.names,
            );
        }
        if crate::code_style::path_ends_with(
            crate::types::SynPathRef::from(&r#macro.path),
            crate::types::StaticStrSliceRef::from([constants_str::BOOL_ENUM_TO_TOKENS].as_slice()),
        )
        .get()
        {
            crate::code_style::collect_first_macro_identifier_domain_name(
                crate::types::SourceTextRef::from(r#macro.tokens.to_string().as_str()),
                &mut self.names,
            );
        }
        if crate::code_style::path_ends_with(
            crate::types::SynPathRef::from(&r#macro.path),
            crate::types::StaticStrSliceRef::from(
                [
                    constants_str::CODE_STYLE_GENERATE_DERIVE_TOKEN_STREAM_BUILDER_MACRO_NAME,
                    constants_str::CODE_STYLE_GENERATE_DERIVE_TOKEN_STREAM_BUILDER_MACRO_NAME,
                ]
                .as_slice(),
            ),
        )
        .get()
        {
            let _: bool = self
                .names
                .insert(String::from(constants_str::DTOKENSTREAMBUILDER));
        }
        syn::visit::visit_macro(self, r#macro);
    }
}
#[derive(
    proc_macro_getters::Getters,
    proc_macro_new::New,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
)]
pub(super) struct DomainTypePolicyVisitor<'types> {
    check_non_public: crate::types::AnalyzerBool,
    closure_body_scan_depth: crate::types::AnalyzerCount,
    errors: crate::types::DiagnosticMessages,
    generic_scopes: Vec<crate::types::SourceTextBTreeSet>,
    repo_crates: crate::types::SourceTextBTreeSetRef<'types>,
    repo_types: crate::types::SourceTextBTreeSetRef<'types>,
}
#[derive(
    proc_macro_getters::Getters,
    proc_macro_new::New,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
)]
pub(super) struct AnalyzerStateRawContainerFieldVisitor {
    errors: crate::types::DiagnosticMessages,
}
#[derive(
    proc_macro_getters::Getters,
    proc_macro_new::New,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
)]
pub(super) struct HelperRawTextReturnVisitor {
    errors: crate::types::DiagnosticMessages,
}
#[derive(
    proc_macro_getters::Getters,
    proc_macro_new::New,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
)]
pub(super) struct RawTextLocalVisitor {
    errors: crate::types::DiagnosticMessages,
}
#[derive(
    proc_macro_getters::Getters,
    proc_macro_new::New,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
)]
pub(super) struct ExternalLeafWrapperNameVisitor<'types> {
    errors: crate::types::DiagnosticMessages,
    repo_crates: crate::types::SourceTextBTreeSetRef<'types>,
}
impl DomainTypePolicyVisitor<'_> {
    fn check_fields(
        &mut self,
        syn_fields_ref: crate::types::SynFieldsRef<'_>,
        source_text_ref: crate::types::SourceTextRef<'_>,
        analyzer_bool: crate::types::AnalyzerBool,
    ) {
        let fields_ref = syn_fields_ref.as_ref();
        if analyzer_bool.get()
            && matches!(fields_ref, syn::Fields::Unnamed(unnamed_fields) if unnamed_fields.unnamed.len() == 1)
        {
            return;
        }
        fields_ref.iter().for_each(|field| {
            self.check_ty(crate::types::SynTypeRef::from(&field.ty), source_text_ref);
        });
    }
    fn check_path_arguments(
        &mut self,
        syn_path_arguments_ref: crate::types::SynPathArgumentsRef<'_>,
        source_text_ref: crate::types::SourceTextRef<'_>,
    ) {
        match syn_path_arguments_ref.as_ref() {
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
                    .for_each(|ty| {
                        self.check_ty(crate::types::SynTypeRef::from(ty), source_text_ref);
                    });
            }
            syn::PathArguments::Parenthesized(args) => {
                args.inputs.iter().for_each(|arg| {
                    self.check_ty(crate::types::SynTypeRef::from(&arg.ty), source_text_ref);
                });
                match &args.output {
                    syn::ReturnType::Default => {}
                    syn::ReturnType::Type(_, ty) => {
                        self.check_ty(crate::types::SynTypeRef::from(&**ty), source_text_ref);
                    }
                }
            }
            syn::PathArguments::None => {}
        }
    }
    fn check_sig(
        &mut self,
        syn_signature_ref: crate::types::SynSignatureRef<'_>,
        source_text_ref: crate::types::SourceTextRef<'_>,
    ) {
        let sig_ref = syn_signature_ref.as_ref();
        self.push_generics(crate::types::SynGenericsRef::from(&sig_ref.generics));
        sig_ref
            .inputs
            .iter()
            .filter_map(|input| match input {
                syn::FnArg::Receiver(_) => None,
                syn::FnArg::Typed(pat_ty) => Some(pat_ty),
            })
            .for_each(|pat_ty| {
                self.check_ty(
                    crate::types::SynTypeRef::from(&*pat_ty.ty),
                    crate::types::SourceTextRef::from(
                        format!("{} parameter", source_text_ref.as_ref()).as_str(),
                    ),
                );
            });
        match &sig_ref.output {
            syn::ReturnType::Default => {}
            syn::ReturnType::Type(_, ty) => {
                self.check_ty(
                    crate::types::SynTypeRef::from(&**ty),
                    crate::types::SourceTextRef::from(
                        format!("{} return type", source_text_ref.as_ref()).as_str(),
                    ),
                );
            }
        }
        self.pop_generics();
    }
    fn check_ty(
        &mut self,
        syn_type_ref: crate::types::SynTypeRef<'_>,
        source_text_ref: crate::types::SourceTextRef<'_>,
    ) {
        match syn_type_ref.as_ref() {
            syn::Type::Array(ty_array) => {
                self.check_ty(
                    crate::types::SynTypeRef::from(&*ty_array.elem),
                    source_text_ref,
                );
            }
            syn::Type::Group(ty_group) => {
                self.check_ty(
                    crate::types::SynTypeRef::from(&*ty_group.elem),
                    source_text_ref,
                );
            }
            syn::Type::Paren(ty_paren) => {
                self.check_ty(
                    crate::types::SynTypeRef::from(&*ty_paren.elem),
                    source_text_ref,
                );
            }
            syn::Type::Path(ty_path) => {
                self.check_ty_path(crate::types::SynTypePathRef::from(ty_path), source_text_ref);
            }
            syn::Type::Reference(ty_reference) => {
                self.check_ty(
                    crate::types::SynTypeRef::from(&*ty_reference.elem),
                    source_text_ref,
                );
            }
            syn::Type::Slice(ty_slice) => {
                self.check_ty(
                    crate::types::SynTypeRef::from(&*ty_slice.elem),
                    source_text_ref,
                );
            }
            syn::Type::Tuple(ty_tuple) => {
                ty_tuple.elems.iter().for_each(|elem| {
                    self.check_ty(crate::types::SynTypeRef::from(elem), source_text_ref);
                });
            }
            syn::Type::FnPtr(_)
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
    fn check_ty_path(
        &mut self,
        syn_type_path_ref: crate::types::SynTypePathRef<'_>,
        source_text_ref: crate::types::SourceTextRef<'_>,
    ) {
        let ty_path_ref = syn_type_path_ref.as_ref();
        if let Some(qself) = &ty_path_ref.qself {
            self.check_ty(crate::types::SynTypeRef::from(&*qself.ty), source_text_ref);
            ty_path_ref.path.segments.iter().for_each(|segment| {
                self.check_path_arguments(
                    crate::types::SynPathArgumentsRef::from(&segment.arguments),
                    source_text_ref,
                );
            });
            return;
        }
        let Some(segment) = ty_path_ref.path.segments.last() else {
            return;
        };
        let identifier = segment.ident.to_string();
        if ty_path_ref
            .path
            .segments
            .first()
            .is_some_and(|first_segment| first_segment.ident == constants_str::SELF)
        {
            self.check_path_arguments(
                crate::types::SynPathArgumentsRef::from(&segment.arguments),
                source_text_ref,
            );
            return;
        }
        if matches!(
            identifier.as_str(),
            constants_str::OPTION | constants_str::RESULT
        ) {
            self.check_path_arguments(
                crate::types::SynPathArgumentsRef::from(&segment.arguments),
                source_text_ref,
            );
            return;
        }
        if self
            .is_allowed_type_identifier(crate::types::SourceTextRef::from(identifier.as_str()))
            .get()
        {
            self.check_path_arguments(
                crate::types::SynPathArgumentsRef::from(&segment.arguments),
                source_text_ref,
            );
            return;
        }
        if self
            .path_starts_with_allowed_type_identifier(crate::types::SynPathRef::from(
                &ty_path_ref.path,
            ))
            .get()
        {
            ty_path_ref.path.segments.iter().for_each(|path_segment| {
                self.check_path_arguments(
                    crate::types::SynPathArgumentsRef::from(&path_segment.arguments),
                    source_text_ref,
                );
            });
            return;
        }
        if self
            .path_starts_with_repo_crate(crate::types::SynPathRef::from(&ty_path_ref.path))
            .get()
        {
            ty_path_ref.path.segments.iter().for_each(|path_segment| {
                self.check_path_arguments(
                    crate::types::SynPathArgumentsRef::from(&path_segment.arguments),
                    source_text_ref,
                );
            });
            return;
        }
        if self
            .path_starts_with_external_crate(crate::types::SynPathRef::from(&ty_path_ref.path))
            .get()
        {
            self.errors.push(format!(
                "{} uses `{}`; use a repository domain wrapper type and initialize it with From/TryFrom instead of exposing raw external or primitive types",
                source_text_ref.as_ref(),
                crate::code_style::path_to_string(crate::types::SynPathRef::from(&ty_path_ref.path)).as_ref()
            ));
            self.check_path_arguments(
                crate::types::SynPathArgumentsRef::from(&segment.arguments),
                source_text_ref,
            );
            return;
        }
        self.errors.push(format!(
                "{} uses `{}`; use a repository domain wrapper type and initialize it with From/TryFrom instead of exposing raw external or primitive types",
                source_text_ref.as_ref(),
                crate::code_style::path_to_string(crate::types::SynPathRef::from(&ty_path_ref.path)).as_ref()
            ));
        self.check_path_arguments(
            crate::types::SynPathArgumentsRef::from(&segment.arguments),
            source_text_ref,
        );
    }
    fn closure_body_scan_is_active(&self) -> crate::types::AnalyzerBool {
        crate::types::AnalyzerBool::from(self.closure_body_scan_depth.get() > 0)
    }
    fn is_allowed_type_identifier(
        &self,
        source_text_ref: crate::types::SourceTextRef<'_>,
    ) -> crate::types::AnalyzerBool {
        let identifier_ref = source_text_ref.as_ref();
        crate::types::AnalyzerBool::from(
            identifier_ref == constants_str::SELF
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
        syn_path_ref: crate::types::SynPathRef<'_>,
    ) -> crate::types::AnalyzerBool {
        let path_ref = syn_path_ref.as_ref();
        crate::types::AnalyzerBool::from(
            path_ref.segments.len() > 1
                && path_ref.segments.first().is_some_and(|segment| {
                    self.is_allowed_type_identifier(crate::types::SourceTextRef::from(
                        segment.ident.to_string().as_str(),
                    ))
                    .get()
                }),
        )
    }
    fn path_starts_with_external_crate(
        &self,
        syn_path_ref: crate::types::SynPathRef<'_>,
    ) -> crate::types::AnalyzerBool {
        let path_ref = syn_path_ref.as_ref();
        crate::types::AnalyzerBool::from(
            path_ref.segments.len() > 1
                && path_ref.segments.first().is_some_and(|segment| {
                    let identifier = segment.ident.to_string();
                    identifier != constants_str::CRATE
                        && identifier != constants_str::SELF_ALT
                        && identifier != constants_str::SUPER
                        && !self.repo_crates.as_ref().contains(&identifier)
                        && !self
                            .is_allowed_type_identifier(crate::types::SourceTextRef::from(
                                identifier.as_str(),
                            ))
                            .get()
                }),
        )
    }
    fn path_starts_with_repo_crate(
        &self,
        syn_path_ref: crate::types::SynPathRef<'_>,
    ) -> crate::types::AnalyzerBool {
        let path_ref = syn_path_ref.as_ref();
        crate::types::AnalyzerBool::from(
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
    fn push_generics(&mut self, syn_generics_ref: crate::types::SynGenericsRef<'_>) {
        let mut names = std::collections::BTreeSet::new();
        names.extend(
            syn_generics_ref
                .as_ref()
                .params
                .iter()
                .filter_map(|param| match param {
                    syn::GenericParam::Type(type_param) => Some(type_param.ident.to_string()),
                    syn::GenericParam::Const(_) | syn::GenericParam::Lifetime(_) => None,
                }),
        );
        self.generic_scopes
            .push(crate::types::SourceTextBTreeSet::from(names));
    }
    fn scan_block_for_closure_inputs(&mut self, syn_block_ref: crate::types::SynBlockRef<'_>) {
        self.closure_body_scan_depth.saturating_inc();
        syn::visit::visit_block(self, syn_block_ref.as_ref());
        self.closure_body_scan_depth.saturating_dec();
    }
}
impl<'ast> syn::visit::Visit<'ast> for DomainTypePolicyVisitor<'_> {
    fn visit_expr_closure(&mut self, expr_closure: &'ast syn::ExprClosure) {
        expr_closure.inputs.iter().for_each(|input| {
            if let syn::Pat::Type(pat_ty) = input {
                self.check_ty(
                    crate::types::SynTypeRef::from(&*pat_ty.ty),
                    crate::types::SourceTextRef::from(constants_str::CLOSURE_PARAMETER),
                );
            }
        });
        syn::visit::visit_expr_closure(self, expr_closure);
    }
    fn visit_item(&mut self, item: &'ast syn::Item) {
        if crate::code_style::has_test_only_cfg_attr(crate::types::SynItemRef::from(item)).get() {
            return;
        }
        if self.closure_body_scan_is_active().get() {
            return;
        }
        syn::visit::visit_item(self, item);
    }
    fn visit_item_enum(&mut self, item_enum: &'ast syn::ItemEnum) {
        if item_enum
            .ident
            .to_string()
            .ends_with(constants_str::TRYFROMSTRINGERROR)
        {
            return;
        }
        self.push_generics(crate::types::SynGenericsRef::from(&item_enum.generics));
        item_enum.variants.iter().for_each(|variant| {
            self.check_fields(
                crate::types::SynFieldsRef::from(&variant.fields),
                crate::types::SourceTextRef::from(
                    format!("enum `{}` variant", item_enum.ident).as_str(),
                ),
                crate::types::AnalyzerBool::default(),
            );
        });
        self.pop_generics();
    }
    fn visit_item_fn(&mut self, item_fn: &'ast syn::ItemFn) {
        if crate::code_style::item_fn_is_proc_macro(crate::types::SynItemFnRef::from(item_fn)).get()
        {
            return;
        }
        if self.check_non_public.get() || matches!(item_fn.vis, syn::Visibility::Public(_)) {
            self.check_sig(
                crate::types::SynSignatureRef::from(&item_fn.sig),
                crate::types::SourceTextRef::from(
                    format!("function `{}`", item_fn.sig.ident).as_str(),
                ),
            );
        }
        self.scan_block_for_closure_inputs(crate::types::SynBlockRef::from(&*item_fn.block));
    }
    fn visit_item_impl(&mut self, item_impl: &'ast syn::ItemImpl) {
        if item_impl.trait_.is_some() {
            return;
        }
        self.push_generics(crate::types::SynGenericsRef::from(&item_impl.generics));
        let check_non_public = self.check_non_public.get();
        item_impl
            .items
            .iter()
            .filter_map(|item| match item {
                syn::ImplItem::Fn(item_fn)
                    if !crate::code_style::attrs_contain_test_only_cfg(
                        crate::types::SynAttributeListRef::from(item_fn.attrs.as_slice()),
                    )
                    .get()
                        && (check_non_public
                            || matches!(item_fn.vis, syn::Visibility::Public(_))) =>
                {
                    if crate::code_style::method_is_explicit_wrapper_accessor(
                        crate::types::SynIdentifierRef::from(&item_fn.sig.ident),
                    )
                    .get()
                        || (matches!(item_fn.vis, syn::Visibility::Inherited)
                            && item_fn.sig.ident == constants_str::VALIDATE)
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
                    crate::types::SynSignatureRef::from(&item_fn.sig),
                    crate::types::SourceTextRef::from(
                        format!("method `{}`", item_fn.sig.ident).as_str(),
                    ),
                );
            });
        item_impl
            .items
            .iter()
            .filter_map(|item| match item {
                syn::ImplItem::Fn(item_fn)
                    if !crate::code_style::attrs_contain_test_only_cfg(
                        crate::types::SynAttributeListRef::from(item_fn.attrs.as_slice()),
                    )
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
                self.scan_block_for_closure_inputs(crate::types::SynBlockRef::from(&item_fn.block));
            });
        self.pop_generics();
    }
    fn visit_item_struct(&mut self, item_struct: &'ast syn::ItemStruct) {
        self.push_generics(crate::types::SynGenericsRef::from(&item_struct.generics));
        self.check_fields(
            crate::types::SynFieldsRef::from(&item_struct.fields),
            crate::types::SourceTextRef::from(
                format!("struct `{}` field", item_struct.ident).as_str(),
            ),
            crate::types::AnalyzerBool::from(true),
        );
        self.pop_generics();
    }
    fn visit_item_trait(&mut self, item_trait: &'ast syn::ItemTrait) {
        self.push_generics(crate::types::SynGenericsRef::from(&item_trait.generics));
        item_trait
            .items
            .iter()
            .filter_map(|item| match item {
                syn::TraitItem::Fn(item_fn)
                    if !crate::code_style::attrs_contain_test_only_cfg(
                        crate::types::SynAttributeListRef::from(item_fn.attrs.as_slice()),
                    )
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
                    crate::types::SynSignatureRef::from(&item_fn.sig),
                    crate::types::SourceTextRef::from(
                        format!("trait method `{}`", item_fn.sig.ident).as_str(),
                    ),
                );
            });
        self.pop_generics();
    }
}
impl AnalyzerStateRawContainerFieldVisitor {
    fn check_fields(&mut self, syn_item_struct_ref: crate::types::SynItemStructRef<'_>) {
        let item_ref = syn_item_struct_ref.as_ref();
        item_ref.fields.iter().for_each(|field| {
            if let Some((raw_ty, wrapper_ty)) = crate::code_style::analyzer_state_raw_container_ty(
                crate::types::SynTypeRef::from(&field.ty),
            ) {
                let field_name = field
                    .ident
                    .as_ref()
                    .map_or_else(|| String::from(constants_str::TUPLE), ToString::to_string);
                self.errors.push(format!(
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
    fn visit_item_struct(&mut self, item_struct: &'ast syn::ItemStruct) {
        if crate::code_style::item_struct_is_single_field_tuple_wrapper(
            crate::types::SynItemStructRef::from(item_struct),
        )
        .get()
        {
            return;
        }
        self.check_fields(crate::types::SynItemStructRef::from(item_struct));
        syn::visit::visit_item_struct(self, item_struct);
    }
}
impl HelperRawTextReturnVisitor {
    fn check_sig(
        &mut self,
        syn_signature_ref: crate::types::SynSignatureRef<'_>,
        source_text_ref: crate::types::SourceTextRef<'_>,
    ) {
        let syn::ReturnType::Type(_, ty) = &syn_signature_ref.as_ref().output else {
            return;
        };
        if let Some((raw_ty, wrapper_ty)) =
            crate::code_style::raw_text_return_ty(crate::types::SynTypeRef::from(&**ty))
        {
            self.errors.push(format!(
                "{} return type uses `{}`; use `{}`",
                source_text_ref.as_ref(),
                raw_ty.get(),
                wrapper_ty.get()
            ));
        }
    }
}
impl<'ast> syn::visit::Visit<'ast> for HelperRawTextReturnVisitor {
    fn visit_item_fn(&mut self, item_fn: &'ast syn::ItemFn) {
        if crate::code_style::item_fn_is_proc_macro(crate::types::SynItemFnRef::from(item_fn)).get()
        {
            return;
        }
        self.check_sig(
            crate::types::SynSignatureRef::from(&item_fn.sig),
            crate::types::SourceTextRef::from(format!("function `{}`", item_fn.sig.ident).as_str()),
        );
        syn::visit::visit_item_fn(self, item_fn);
    }
    fn visit_item_impl(&mut self, item_impl: &'ast syn::ItemImpl) {
        if item_impl.trait_.is_some() {
            return;
        }
        item_impl
            .items
            .iter()
            .filter_map(|item| match item {
                syn::ImplItem::Fn(item_fn)
                    if !crate::code_style::method_is_explicit_wrapper_accessor(
                        crate::types::SynIdentifierRef::from(&item_fn.sig.ident),
                    )
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
                    crate::types::SynSignatureRef::from(&item_fn.sig),
                    crate::types::SourceTextRef::from(
                        format!("method `{}`", item_fn.sig.ident).as_str(),
                    ),
                );
            });
    }
}
impl<'ast> syn::visit::Visit<'ast> for RawTextLocalVisitor {
    fn visit_local(&mut self, local: &'ast syn::Local) {
        if let syn::Pat::Type(pat_ty) = &local.pat
            && let Some((raw_ty, wrapper_ty)) =
                crate::code_style::raw_text_return_ty(crate::types::SynTypeRef::from(&*pat_ty.ty))
            && raw_ty.get() != constants_str::STR
            && raw_ty.get() != constants_str::OPTION_STR
        {
            self.errors.push(format!(
                "{} uses `{}`; use `{}`",
                constants_str::LOCAL_BINDING,
                raw_ty.get(),
                wrapper_ty.get()
            ));
        }
        syn::visit::visit_local(self, local);
    }
}
impl<'ast> syn::visit::Visit<'ast> for ExternalLeafWrapperNameVisitor<'_> {
    fn visit_item_struct(&mut self, item_struct: &'ast syn::ItemStruct) {
        if crate::code_style::attrs_contain_test_only_cfg(crate::types::SynAttributeListRef::from(
            item_struct.attrs.as_slice(),
        ))
        .get()
        {
            return;
        }
        let syn::Fields::Unnamed(fields) = &item_struct.fields else {
            syn::visit::visit_item_struct(self, item_struct);
            return;
        };
        if fields.unnamed.len() != 1 {
            syn::visit::visit_item_struct(self, item_struct);
            return;
        }
        let Some(field) = fields.unnamed.first() else {
            syn::visit::visit_item_struct(self, item_struct);
            return;
        };
        self.check_external_leaf_wrapper_name(
            crate::types::SynItemStructRef::from(item_struct),
            crate::types::SynTypeRef::from(&field.ty),
        );
        syn::visit::visit_item_struct(self, item_struct);
    }
}
#[allow(
    clippy::arbitrary_source_item_ordering,
    reason = "root and leaf path traversal helpers stay grouped by traversal direction"
)]
impl ExternalLeafWrapperNameVisitor<'_> {
    fn check_external_leaf_wrapper_name(
        &mut self,
        syn_item_struct_ref: crate::types::SynItemStructRef<'_>,
        syn_type_ref: crate::types::SynTypeRef<'_>,
    ) {
        let Some(leaf_segment) = self.external_leaf_segment(syn_type_ref) else {
            return;
        };
        let Some(root_segment) = self.external_root_segment(syn_type_ref) else {
            return;
        };
        let leaf_segment_ref = leaf_segment.get();
        let root_segment_ref = root_segment.get();
        let item_ref = syn_item_struct_ref.as_ref();
        if root_segment_ref.ident == constants_str::STD
            && leaf_segment_ref
                .ident
                .to_string()
                .starts_with(constants_str::CODE_STYLE_NON_ZERO_PREFIX)
        {
            return;
        }
        let required_segment = if root_segment_ref.ident == constants_str::STD {
            leaf_segment_ref
        } else {
            root_segment_ref
        };
        let (fragment_text, _) = required_segment.ident.to_string().chars().fold(
            (String::new(), true),
            |(mut output, mut next_upper), character| {
                if character == '_' {
                    next_upper = true;
                    return (output, next_upper);
                }
                if next_upper {
                    character
                        .to_uppercase()
                        .for_each(|uppercase| output.push(uppercase));
                    next_upper = false;
                } else {
                    output.push(character);
                }
                (output, next_upper)
            },
        );
        let expected_fragment = crate::types::SourceText::try_from(fragment_text)
            .expect(constants_str::DIAGNOSTIC_9EA072C4);
        let identifier = item_ref.ident.to_string();
        if identifier.contains(expected_fragment.as_ref()) {
            return;
        }
        self.errors.push(format!(
            "tuple wrapper `{}` wraps external type `{}::{}`; rename it so it contains `{}`",
            item_ref.ident,
            root_segment_ref.ident,
            leaf_segment_ref.ident,
            expected_fragment.as_ref()
        ));
    }
    fn external_root_segment<'ty_lt>(
        &self,
        syn_type_ref: crate::types::SynTypeRef<'ty_lt>,
    ) -> Option<crate::types::SynPathSegmentRef<'ty_lt>> {
        match syn_type_ref.get() {
            syn::Type::Array(ty_array) => {
                self.external_root_segment(crate::types::SynTypeRef::from(&*ty_array.elem))
            }
            syn::Type::Group(ty_group) => {
                self.external_root_segment(crate::types::SynTypeRef::from(&*ty_group.elem))
            }
            syn::Type::Paren(ty_paren) => {
                self.external_root_segment(crate::types::SynTypeRef::from(&*ty_paren.elem))
            }
            syn::Type::Path(ty_path) => {
                let ty_path_ref = crate::types::SynTypePathRef::from(ty_path).get();
                if let Some(qself) = &ty_path_ref.qself {
                    return self.external_root_segment(crate::types::SynTypeRef::from(&*qself.ty));
                }
                let first_segment = ty_path_ref.path.segments.first()?;
                let parse_first_identifier = first_segment.ident.to_string();
                if parse_first_identifier == constants_str::CRATE
                    || parse_first_identifier == constants_str::SELF_ALT
                    || parse_first_identifier == constants_str::SUPER
                    || self.repo_crates.as_ref().contains(&parse_first_identifier)
                {
                    return ty_path_ref.path.segments.iter().find_map(|segment| {
                        self.external_root_segment_from_arguments(
                            crate::types::SynPathArgumentsRef::from(&segment.arguments),
                        )
                    });
                }
                if ty_path_ref.path.segments.len() > 1 {
                    return Some(crate::types::SynPathSegmentRef::from(first_segment));
                }
                ty_path_ref.path.segments.iter().find_map(|segment| {
                    self.external_root_segment_from_arguments(
                        crate::types::SynPathArgumentsRef::from(&segment.arguments),
                    )
                })
            }
            syn::Type::Reference(ty_reference) => {
                self.external_root_segment(crate::types::SynTypeRef::from(&*ty_reference.elem))
            }
            syn::Type::Slice(ty_slice) => {
                self.external_root_segment(crate::types::SynTypeRef::from(&*ty_slice.elem))
            }
            syn::Type::Tuple(ty_tuple) => ty_tuple
                .elems
                .iter()
                .find_map(|elem| self.external_root_segment(crate::types::SynTypeRef::from(elem))),
            syn::Type::FnPtr(_)
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
        syn_path_arguments_ref: crate::types::SynPathArgumentsRef<'args_lt>,
    ) -> Option<crate::types::SynPathSegmentRef<'args_lt>> {
        match syn_path_arguments_ref.get() {
            syn::PathArguments::AngleBracketed(args) => {
                args.args.iter().find_map(|arg| match arg {
                    syn::GenericArgument::Type(ty) => {
                        self.external_root_segment(crate::types::SynTypeRef::from(ty))
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
                .find_map(|arg| self.external_root_segment(crate::types::SynTypeRef::from(&arg.ty)))
                .or_else(|| match &args.output {
                    syn::ReturnType::Default => None,
                    syn::ReturnType::Type(_, ty) => {
                        self.external_root_segment(crate::types::SynTypeRef::from(&**ty))
                    }
                }),
            syn::PathArguments::None => None,
        }
    }
    fn external_leaf_segment<'ty_lt>(
        &self,
        syn_type_ref: crate::types::SynTypeRef<'ty_lt>,
    ) -> Option<crate::types::SynPathSegmentRef<'ty_lt>> {
        match syn_type_ref.get() {
            syn::Type::Array(ty_array) => {
                self.external_leaf_segment(crate::types::SynTypeRef::from(&*ty_array.elem))
            }
            syn::Type::Group(ty_group) => {
                self.external_leaf_segment(crate::types::SynTypeRef::from(&*ty_group.elem))
            }
            syn::Type::Paren(ty_paren) => {
                self.external_leaf_segment(crate::types::SynTypeRef::from(&*ty_paren.elem))
            }
            syn::Type::Path(ty_path) => {
                let ty_path_ref = crate::types::SynTypePathRef::from(ty_path).get();
                if let Some(qself) = &ty_path_ref.qself {
                    return self.external_leaf_segment(crate::types::SynTypeRef::from(&*qself.ty));
                }
                let first_segment = ty_path_ref.path.segments.first()?;
                let parse_first_identifier = first_segment.ident.to_string();
                if parse_first_identifier == constants_str::CRATE
                    || parse_first_identifier == constants_str::SELF_ALT
                    || parse_first_identifier == constants_str::SUPER
                    || self.repo_crates.as_ref().contains(&parse_first_identifier)
                {
                    return ty_path_ref.path.segments.iter().find_map(|segment| {
                        self.external_leaf_segment_from_arguments(
                            crate::types::SynPathArgumentsRef::from(&segment.arguments),
                        )
                    });
                }
                if ty_path_ref.path.segments.len() > 1 {
                    return ty_path_ref
                        .path
                        .segments
                        .last()
                        .map(crate::types::SynPathSegmentRef::from);
                }
                ty_path_ref.path.segments.iter().find_map(|segment| {
                    self.external_leaf_segment_from_arguments(
                        crate::types::SynPathArgumentsRef::from(&segment.arguments),
                    )
                })
            }
            syn::Type::Reference(ty_reference) => {
                self.external_leaf_segment(crate::types::SynTypeRef::from(&*ty_reference.elem))
            }
            syn::Type::Slice(ty_slice) => {
                self.external_leaf_segment(crate::types::SynTypeRef::from(&*ty_slice.elem))
            }
            syn::Type::Tuple(ty_tuple) => ty_tuple
                .elems
                .iter()
                .find_map(|elem| self.external_leaf_segment(crate::types::SynTypeRef::from(elem))),
            syn::Type::FnPtr(_)
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
    fn external_leaf_segment_from_arguments<'args_lt>(
        &self,
        syn_path_arguments_ref: crate::types::SynPathArgumentsRef<'args_lt>,
    ) -> Option<crate::types::SynPathSegmentRef<'args_lt>> {
        match syn_path_arguments_ref.get() {
            syn::PathArguments::AngleBracketed(args) => {
                args.args.iter().find_map(|arg| match arg {
                    syn::GenericArgument::Type(ty) => {
                        self.external_leaf_segment(crate::types::SynTypeRef::from(ty))
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
                .find_map(|arg| self.external_leaf_segment(crate::types::SynTypeRef::from(&arg.ty)))
                .or_else(|| match &args.output {
                    syn::ReturnType::Default => None,
                    syn::ReturnType::Type(_, ty) => {
                        self.external_leaf_segment(crate::types::SynTypeRef::from(&**ty))
                    }
                }),
            syn::PathArguments::None => None,
        }
    }
}
