#[derive(
    proc_macro_getters::Getters,
    proc_macro_new::New,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
)]
pub(super) struct StringWrapperNameVisitor {
    names: crate::types::SourceTextBTreeSet,
}
impl<'ast> syn::visit::Visit<'ast> for StringWrapperNameVisitor {
    fn visit_item_struct(&mut self, i: &'ast syn::ItemStruct) {
        if crate::code_style::item_struct_is_single_string_wrapper(
            crate::types::SynItemStructRef::from(i),
        )
        .get()
        {
            let _: bool = self.names.insert(i.ident.to_string());
        }
        syn::visit::visit_item_struct(self, i);
    }
}
#[derive(
    proc_macro_getters::Getters,
    proc_macro_new::New,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
)]
pub(super) struct BoundedStringStorageVisitor {
    ers: crate::types::DiagnosticMsgs,
}
impl<'ast> syn::visit::Visit<'ast> for BoundedStringStorageVisitor {
    fn visit_item_struct(&mut self, i: &'ast syn::ItemStruct) {
        let has_bounded_string_attr = i
            .attrs
            .iter()
            .any(|attr| attr.path().is_ident(constants_str::BOUNDED_STRING));
        let derives_old_bounded_string = i.attrs.iter().any(|attr| {
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
            self.ers.push(format!(
                "`{}` derives removed `proc_macro_newtype::BoundedString`; store `bounded_types::bounded_string::BoundedString` instead",
                i.ident
            ));
        }
        if has_bounded_string_attr {
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
            if !stores_bounded_string {
                self.ers.push(format!(
                    "`{}` uses `#[bounded_string]` but does not store `bounded_types::bounded_string::BoundedString`",
                    i.ident
                ));
            }
        }
        syn::visit::visit_item_struct(self, i);
    }
}
#[derive(
    proc_macro_getters::Getters,
    proc_macro_new::New,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
)]
pub(super) struct StringWrapperFromVisitor<'names_lt> {
    ers: crate::types::DiagnosticMsgs,
    len_checked_function_names: &'names_lt crate::types::SourceTextBTreeSet,
    string_wrapper_names: &'names_lt crate::types::SourceTextBTreeSet,
    try_from_string_len_checked_names: crate::types::SourceTextBTreeSet,
    try_from_string_names: crate::types::SourceTextBTreeSet,
}
impl StringWrapperFromVisitor<'_> {
    fn check_bounded_string_attr(&mut self, item: crate::types::SynItemStructRef<'_>) {
        let item_ref = item.as_ref();
        if !crate::code_style::item_struct_is_single_string_wrapper(item).get() {
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
    fn check_from_impl(&mut self, item: crate::types::SynItemImplRef<'_>) {
        let is_from_string = crate::types::AnalyzerBool::from(
            item.as_ref().trait_.as_ref().is_some_and(|(path, _)| {
                crate::code_style::path_ends_with(
                    crate::types::SynPathRef::from(path),
                    crate::types::StaticStrSliceRef::from([constants_str::FROM_ALT_3].as_slice()),
                )
                .get()
                    && crate::code_style::from_trait_arg_is_string(crate::types::SynPathRef::from(
                        path,
                    ))
                    .get()
            }),
        );
        if !is_from_string.get() {
            return;
        }
        let identifier = crate::code_style::item_impl_self_ty_identifier(item).map_or_else(
            || String::from(constants_str::NON_PATH_TARGET),
            String::from,
        );
        self.ers.push(format!(
            "`{identifier}` implements `From<String>`; implement `TryFrom<String>` instead"
        ));
    }
    fn check_newtype_attr(&mut self, item: crate::types::SynItemStructRef<'_>) {
        let item_ref = item.as_ref();
        if !crate::code_style::item_struct_is_single_string_wrapper(item).get() {
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
            self.ers.push(format!(
                        "string wrapper `{}` derives `proc_macro_newtype::FromInner`; derive `proc_macro_newtype::TryFrom` with a length check instead",
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
    fn check_try_from_impl(&mut self, item: crate::types::SynItemImplRef<'_>) {
        let is_try_from_string = crate::types::AnalyzerBool::from(
            item.as_ref().trait_.as_ref().is_some_and(|(path, _)| {
                crate::code_style::path_ends_with(
                    crate::types::SynPathRef::from(path),
                    crate::types::StaticStrSliceRef::from([constants_str::TRYFROM].as_slice()),
                )
                .get()
                    && crate::code_style::from_trait_arg_is_string(crate::types::SynPathRef::from(
                        path,
                    ))
                    .get()
            }),
        );
        if !is_try_from_string.get() {
            return;
        }
        let Some(identifier) = crate::code_style::item_impl_self_ty_identifier(item) else {
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
        syn::visit::Visit::visit_item_impl(&mut len_call_visitor, item.as_ref());
        let mut len_checked_call_visitor = LenCheckedFunctionCallVisitor {
            found: crate::types::AnalyzerBool::default(),
            names: self.len_checked_function_names,
        };
        syn::visit::Visit::visit_item_impl(&mut len_checked_call_visitor, item.as_ref());
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
    fn visit_expr_call(&mut self, i: &'ast syn::ExprCall) {
        if let syn::Expr::Path(path) = i.func.as_ref() {
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
        syn::visit::visit_expr_call(self, i);
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
    fn visit_item_fn(&mut self, i: &'ast syn::ItemFn) {
        let mut visitor = LenMethodCallVisitor {
            found: crate::types::AnalyzerBool::default(),
        };
        syn::visit::Visit::visit_block(&mut visitor, &i.block);
        if visitor.found.get() {
            let _: bool = self.names.insert(i.sig.ident.to_string());
        }
        syn::visit::visit_item_fn(self, i);
    }
    fn visit_item_impl(&mut self, i: &'ast syn::ItemImpl) {
        let Some(type_name) =
            crate::code_style::item_impl_self_ty_identifier(crate::types::SynItemImplRef::from(i))
        else {
            syn::visit::visit_item_impl(self, i);
            return;
        };
        i.items.iter().for_each(|item| {
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
        syn::visit::visit_item_impl(self, i);
    }
}
impl<'ast> syn::visit::Visit<'ast> for StringWrapperFromVisitor<'_> {
    fn visit_item_impl(&mut self, i: &'ast syn::ItemImpl) {
        self.check_from_impl(crate::types::SynItemImplRef::from(i));
        self.check_try_from_impl(crate::types::SynItemImplRef::from(i));
        syn::visit::visit_item_impl(self, i);
    }
    fn visit_item_struct(&mut self, i: &'ast syn::ItemStruct) {
        syn::visit::visit_item_struct(self, i);
        self.check_bounded_string_attr(crate::types::SynItemStructRef::from(i));
        self.check_newtype_attr(crate::types::SynItemStructRef::from(i));
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
    fn visit_expr_method_call(&mut self, i: &'ast syn::ExprMethodCall) {
        if i.method == constants_str::LEN {
            self.found.set_true();
        }
        syn::visit::visit_expr_method_call(self, i);
    }
}
#[derive(
    proc_macro_getters::Getters,
    proc_macro_new::New,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
)]
pub(super) struct PublicTupleWrapperFieldVisitor {
    ers: crate::types::DiagnosticMsgs,
}
#[derive(
    proc_macro_getters::Getters,
    proc_macro_new::New,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
)]
pub(super) struct DirectDeserializeTupleWrapperVisitor {
    ers: crate::types::DiagnosticMsgs,
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
    ers: crate::types::DiagnosticMsgs,
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
    ers: crate::types::DiagnosticMsgs,
    inside_conversion_impl: crate::types::AnalyzerBool,
    names: &'names crate::types::SourceTextBTreeSet,
}
impl<'ast> syn::visit::Visit<'ast> for PublicTupleWrapperFieldVisitor {
    fn visit_item_struct(&mut self, i: &'ast syn::ItemStruct) {
        let inner_field_is_non_private = match &i.fields {
            syn::Fields::Unnamed(fields) if fields.unnamed.len() == 1usize => fields
                .unnamed
                .first()
                .is_some_and(|field| matches!(field.vis, syn::Visibility::Public(_))),
            syn::Fields::Named(_) | syn::Fields::Unnamed(_) | syn::Fields::Unit => false,
        };
        if crate::code_style::item_struct_is_single_field_tuple_wrapper(
            crate::types::SynItemStructRef::from(i),
        )
        .get()
            && inner_field_is_non_private
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
        let derives_deserialize = i.attrs.iter().any(|attr| {
            attr.path().is_ident(constants_str::DERIVE)
                && match &attr.meta {
                    syn::Meta::List(list) => list
                        .tokens
                        .to_string()
                        .contains(constants_str::CODE_STYLE_DESERIALIZE_DERIVE_NAME),
                    syn::Meta::NameValue(_) | syn::Meta::Path(_) => false,
                }
        });
        let deserialize_uses_conversion = i.attrs.iter().any(|attr| {
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
            crate::types::SynItemStructRef::from(i),
        )
        .get()
            && derives_deserialize
            && !deserialize_uses_conversion
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
                segment.ident == constants_str::FROM_ALT_4
                    || segment.ident == constants_str::NEWTYPE_TRY_FROM
            })
        {
            self.found.set_true();
        }
        syn::visit::visit_expr_call(self, i);
    }
}
impl<'ast> syn::visit::Visit<'ast> for ManualDeserializeTupleWrapperVisitor<'_> {
    fn visit_item_impl(&mut self, i: &'ast syn::ItemImpl) {
        let is_deserialize_impl = i.trait_.as_ref().is_some_and(|(path, _)| {
            path.segments.last().is_some_and(|segment| {
                segment.ident == constants_str::CODE_STYLE_DESERIALIZE_DERIVE_NAME
            })
        });
        let Some(name) =
            crate::code_style::item_impl_self_ty_identifier(crate::types::SynItemImplRef::from(i))
        else {
            syn::visit::visit_item_impl(self, i);
            return;
        };
        if is_deserialize_impl && self.names.contains(name.as_ref()) {
            let mut visitor = DeserializeConversionCallVisitor {
                found: crate::types::AnalyzerBool::default(),
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
        let item_ref = crate::types::SynItemImplRef::from(i);
        let Some(name) = crate::code_style::item_impl_self_ty_identifier(item_ref) else {
            syn::visit::visit_item_impl(self, i);
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
        syn::visit::visit_item_impl(self, i);
    }
    fn visit_item_struct(&mut self, i: &'ast syn::ItemStruct) {
        if crate::code_style::item_struct_is_single_field_tuple_wrapper(
            crate::types::SynItemStructRef::from(i),
        )
        .get()
        {
            let name = i.ident.to_string();
            let _: bool = self.names.insert(name.clone());
            if let syn::Fields::Unnamed(fields) = &i.fields
                && fields.unnamed.len() == constants_usize::ONE
                && let Some(field) = fields.unnamed.first()
            {
                drop(self.inner_types.insert(name.clone(), field.ty.clone()));
            }
            let derives_from_inner = i.attrs.iter().any(|attr| {
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
            let derives_from_getter = i.attrs.iter().any(|attr| {
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
            let derives_try_from = i.attrs.iter().any(|attr| {
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
            let derives_conversion = i.attrs.iter().any(|attr| {
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
        syn::visit::visit_item_struct(self, i);
    }
}
impl<'ast> syn::visit::Visit<'ast> for DirectTupleWrapperConstructorVisitor<'_> {
    fn visit_expr_call(&mut self, i: &'ast syn::ExprCall) {
        if !self.inside_conversion_impl.get()
            && let syn::Expr::Path(path) = i.func.as_ref()
            && let Some(segment) = path.path.segments.last()
            && (self.names.contains(segment.ident.to_string().as_str())
                || (segment.ident == constants_str::SELF && self.current_wrapper_name.is_some()))
        {
            let span = syn::spanned::Spanned::span(i.func.as_ref());
            let start = span.start();
            let end = span.end();
            let wrapper_name = self
                .current_wrapper_name
                .as_deref()
                .filter(|_| segment.ident == constants_str::SELF)
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
        self.inside_conversion_impl =
            crate::code_style::item_impl_is_from_or_try_from(crate::types::SynItemImplRef::from(i));
        self.current_wrapper_name =
            crate::code_style::item_impl_self_ty_identifier(crate::types::SynItemImplRef::from(i))
                .map(|name| name.as_ref().to_owned())
                .filter(|name| self.names.contains(name));
        syn::visit::visit_item_impl(self, i);
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
    fn visit_item(&mut self, i: &'ast syn::Item) {
        if crate::code_style::has_test_only_cfg_attr(crate::types::SynItemRef::from(i)).get() {
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
        if i.attrs.iter().any(|attr| {
            attr.path().is_ident(constants_str::DERIVE)
                && matches!(
                    &attr.meta,
                    syn::Meta::List(list)
                        if list.tokens.to_string().contains(constants_str::BOUNDEDSTRING)
                )
        }) {
            let mut generated_error_name = i.ident.to_string();
            generated_error_name.push_str(constants_str::TRYFROMSTRINGERROR);
            let _: bool = self.names.insert(generated_error_name);
        }
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
        if crate::code_style::path_ends_with(
            crate::types::SynPathRef::from(&i.path),
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
            let tokens = i.tokens.to_string();
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
        let path = crate::types::SynPathRef::from(&i.path);
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
                crate::types::SourceTextRef::from(i.tokens.to_string().as_str()),
                &mut self.names,
            );
        }
        if crate::code_style::path_ends_with(
            crate::types::SynPathRef::from(&i.path),
            crate::types::StaticStrSliceRef::from(
                [constants_str::API_OPERATION_ERROR_MACRO_IDENTIFIER].as_slice(),
            ),
        )
        .get()
        {
            crate::code_style::collect_first_macro_identifier_domain_name(
                crate::types::SourceTextRef::from(i.tokens.to_string().as_str()),
                &mut self.names,
            );
        }
        if crate::code_style::path_ends_with(
            crate::types::SynPathRef::from(&i.path),
            crate::types::StaticStrSliceRef::from([constants_str::BOOL_ENUM_TO_TOKENS].as_slice()),
        )
        .get()
        {
            crate::code_style::collect_first_macro_identifier_domain_name(
                crate::types::SourceTextRef::from(i.tokens.to_string().as_str()),
                &mut self.names,
            );
        }
        if crate::code_style::path_ends_with(
            crate::types::SynPathRef::from(&i.path),
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
        syn::visit::visit_macro(self, i);
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
    ers: crate::types::DiagnosticMsgs,
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
    ers: crate::types::DiagnosticMsgs,
}
#[derive(
    proc_macro_getters::Getters,
    proc_macro_new::New,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
)]
pub(super) struct HelperRawTextReturnVisitor {
    ers: crate::types::DiagnosticMsgs,
}
#[derive(
    proc_macro_getters::Getters,
    proc_macro_new::New,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
)]
pub(super) struct RawTextLocalVisitor {
    ers: crate::types::DiagnosticMsgs,
}
#[derive(
    proc_macro_getters::Getters,
    proc_macro_new::New,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
)]
pub(super) struct ExternalLeafWrapperNameVisitor<'types> {
    ers: crate::types::DiagnosticMsgs,
    repo_crates: crate::types::SourceTextBTreeSetRef<'types>,
}
impl DomainTypePolicyVisitor<'_> {
    fn check_fields(
        &mut self,
        fields: crate::types::SynFieldsRef<'_>,
        ctx: crate::types::SourceTextRef<'_>,
        allow_single_newtype_raw: crate::types::AnalyzerBool,
    ) {
        let fields_ref = fields.as_ref();
        if allow_single_newtype_raw.get()
            && matches!(fields_ref, syn::Fields::Unnamed(unnamed_fields) if unnamed_fields.unnamed.len() == 1)
        {
            return;
        }
        fields_ref
            .iter()
            .for_each(|field| self.check_ty(crate::types::SynTypeRef::from(&field.ty), ctx));
    }
    fn check_path_arguments(
        &mut self,
        arguments: crate::types::SynPathArgumentsRef<'_>,
        ctx: crate::types::SourceTextRef<'_>,
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
                    .for_each(|ty| self.check_ty(crate::types::SynTypeRef::from(ty), ctx));
            }
            syn::PathArguments::Parenthesized(args) => {
                args.inputs
                    .iter()
                    .for_each(|arg| self.check_ty(crate::types::SynTypeRef::from(&arg.ty), ctx));
                match &args.output {
                    syn::ReturnType::Default => {}
                    syn::ReturnType::Type(_, ty) => {
                        self.check_ty(crate::types::SynTypeRef::from(&**ty), ctx);
                    }
                }
            }
            syn::PathArguments::None => {}
        }
    }
    fn check_sig(
        &mut self,
        sig: crate::types::SynSignatureRef<'_>,
        ctx: crate::types::SourceTextRef<'_>,
    ) {
        let sig_ref = sig.as_ref();
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
                        format!("{} parameter", ctx.as_ref()).as_str(),
                    ),
                );
            });
        match &sig_ref.output {
            syn::ReturnType::Default => {}
            syn::ReturnType::Type(_, ty) => {
                self.check_ty(
                    crate::types::SynTypeRef::from(&**ty),
                    crate::types::SourceTextRef::from(
                        format!("{} return type", ctx.as_ref()).as_str(),
                    ),
                );
            }
        }
        self.pop_generics();
    }
    fn check_ty(&mut self, ty: crate::types::SynTypeRef<'_>, ctx: crate::types::SourceTextRef<'_>) {
        match ty.as_ref() {
            syn::Type::Array(ty_array) => {
                self.check_ty(crate::types::SynTypeRef::from(&*ty_array.elem), ctx);
            }
            syn::Type::Group(ty_group) => {
                self.check_ty(crate::types::SynTypeRef::from(&*ty_group.elem), ctx);
            }
            syn::Type::Paren(ty_paren) => {
                self.check_ty(crate::types::SynTypeRef::from(&*ty_paren.elem), ctx);
            }
            syn::Type::Path(ty_path) => {
                self.check_ty_path(crate::types::SynTypePathRef::from(ty_path), ctx);
            }
            syn::Type::Reference(ty_reference) => {
                self.check_ty(crate::types::SynTypeRef::from(&*ty_reference.elem), ctx);
            }
            syn::Type::Slice(ty_slice) => {
                self.check_ty(crate::types::SynTypeRef::from(&*ty_slice.elem), ctx);
            }
            syn::Type::Tuple(ty_tuple) => {
                ty_tuple
                    .elems
                    .iter()
                    .for_each(|elem| self.check_ty(crate::types::SynTypeRef::from(elem), ctx));
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
        ty_path: crate::types::SynTypePathRef<'_>,
        ctx: crate::types::SourceTextRef<'_>,
    ) {
        let ty_path_ref = ty_path.as_ref();
        if let Some(qself) = &ty_path_ref.qself {
            self.check_ty(crate::types::SynTypeRef::from(&*qself.ty), ctx);
            ty_path_ref.path.segments.iter().for_each(|segment| {
                self.check_path_arguments(
                    crate::types::SynPathArgumentsRef::from(&segment.arguments),
                    ctx,
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
                ctx,
            );
            return;
        }
        if matches!(
            identifier.as_str(),
            constants_str::OPTION | constants_str::RESULT
        ) {
            self.check_path_arguments(
                crate::types::SynPathArgumentsRef::from(&segment.arguments),
                ctx,
            );
            return;
        }
        if self
            .is_allowed_type_identifier(crate::types::SourceTextRef::from(identifier.as_str()))
            .get()
        {
            self.check_path_arguments(
                crate::types::SynPathArgumentsRef::from(&segment.arguments),
                ctx,
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
                    ctx,
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
                    ctx,
                );
            });
            return;
        }
        if self
            .path_starts_with_external_crate(crate::types::SynPathRef::from(&ty_path_ref.path))
            .get()
        {
            self.ers.push(format!(
                "{} uses `{}`; use a repository domain wrapper type and initialize it with From/TryFrom instead of exposing raw external or primitive types",
                ctx.as_ref(),
                crate::code_style::path_to_string(crate::types::SynPathRef::from(&ty_path_ref.path)).as_ref()
            ));
            self.check_path_arguments(
                crate::types::SynPathArgumentsRef::from(&segment.arguments),
                ctx,
            );
            return;
        }
        self.ers.push(format!(
                "{} uses `{}`; use a repository domain wrapper type and initialize it with From/TryFrom instead of exposing raw external or primitive types",
                ctx.as_ref(),
                crate::code_style::path_to_string(crate::types::SynPathRef::from(&ty_path_ref.path)).as_ref()
            ));
        self.check_path_arguments(
            crate::types::SynPathArgumentsRef::from(&segment.arguments),
            ctx,
        );
    }
    fn closure_body_scan_is_active(&self) -> crate::types::AnalyzerBool {
        crate::types::AnalyzerBool::from(self.closure_body_scan_depth.get() > 0)
    }
    fn is_allowed_type_identifier(
        &self,
        identifier: crate::types::SourceTextRef<'_>,
    ) -> crate::types::AnalyzerBool {
        let identifier_ref = identifier.as_ref();
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
        path: crate::types::SynPathRef<'_>,
    ) -> crate::types::AnalyzerBool {
        let path_ref = path.as_ref();
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
        path: crate::types::SynPathRef<'_>,
    ) -> crate::types::AnalyzerBool {
        let path_ref = path.as_ref();
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
        path: crate::types::SynPathRef<'_>,
    ) -> crate::types::AnalyzerBool {
        let path_ref = path.as_ref();
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
    fn push_generics(&mut self, generics: crate::types::SynGenericsRef<'_>) {
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
            .push(crate::types::SourceTextBTreeSet::from(names));
    }
    fn scan_block_for_closure_inputs(&mut self, block: crate::types::SynBlockRef<'_>) {
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
                    crate::types::SynTypeRef::from(&*pat_ty.ty),
                    crate::types::SourceTextRef::from(constants_str::CLOSURE_PARAMETER),
                );
            }
        });
        syn::visit::visit_expr_closure(self, i);
    }
    fn visit_item(&mut self, i: &'ast syn::Item) {
        if crate::code_style::has_test_only_cfg_attr(crate::types::SynItemRef::from(i)).get() {
            return;
        }
        if self.closure_body_scan_is_active().get() {
            return;
        }
        syn::visit::visit_item(self, i);
    }
    fn visit_item_enum(&mut self, i: &'ast syn::ItemEnum) {
        if i.ident
            .to_string()
            .ends_with(constants_str::TRYFROMSTRINGERROR)
        {
            return;
        }
        self.push_generics(crate::types::SynGenericsRef::from(&i.generics));
        i.variants.iter().for_each(|variant| {
            self.check_fields(
                crate::types::SynFieldsRef::from(&variant.fields),
                crate::types::SourceTextRef::from(format!("enum `{}` variant", i.ident).as_str()),
                crate::types::AnalyzerBool::default(),
            );
        });
        self.pop_generics();
    }
    fn visit_item_fn(&mut self, i: &'ast syn::ItemFn) {
        if crate::code_style::item_fn_is_proc_macro(crate::types::SynItemFnRef::from(i)).get() {
            return;
        }
        if self.check_non_public.get() || matches!(i.vis, syn::Visibility::Public(_)) {
            self.check_sig(
                crate::types::SynSignatureRef::from(&i.sig),
                crate::types::SourceTextRef::from(format!("function `{}`", i.sig.ident).as_str()),
            );
        }
        self.scan_block_for_closure_inputs(crate::types::SynBlockRef::from(&*i.block));
    }
    fn visit_item_impl(&mut self, i: &'ast syn::ItemImpl) {
        if i.trait_.is_some() {
            return;
        }
        self.push_generics(crate::types::SynGenericsRef::from(&i.generics));
        let check_non_public = self.check_non_public.get();
        i.items
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
        i.items
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
    fn visit_item_struct(&mut self, i: &'ast syn::ItemStruct) {
        self.push_generics(crate::types::SynGenericsRef::from(&i.generics));
        self.check_fields(
            crate::types::SynFieldsRef::from(&i.fields),
            crate::types::SourceTextRef::from(format!("struct `{}` field", i.ident).as_str()),
            crate::types::AnalyzerBool::from(true),
        );
        self.pop_generics();
    }
    fn visit_item_trait(&mut self, i: &'ast syn::ItemTrait) {
        self.push_generics(crate::types::SynGenericsRef::from(&i.generics));
        i.items
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
    fn check_fields(&mut self, item: crate::types::SynItemStructRef<'_>) {
        let item_ref = item.as_ref();
        item_ref.fields.iter().for_each(|field| {
            if let Some((raw_ty, wrapper_ty)) = crate::code_style::analyzer_state_raw_container_ty(
                crate::types::SynTypeRef::from(&field.ty),
            ) {
                let field_name = field
                    .ident
                    .as_ref()
                    .map_or_else(|| String::from(constants_str::TUPLE), ToString::to_string);
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
        if crate::code_style::item_struct_is_single_field_tuple_wrapper(
            crate::types::SynItemStructRef::from(i),
        )
        .get()
        {
            return;
        }
        self.check_fields(crate::types::SynItemStructRef::from(i));
        syn::visit::visit_item_struct(self, i);
    }
}
impl HelperRawTextReturnVisitor {
    fn check_sig(
        &mut self,
        sig: crate::types::SynSignatureRef<'_>,
        ctx: crate::types::SourceTextRef<'_>,
    ) {
        let syn::ReturnType::Type(_, ty) = &sig.as_ref().output else {
            return;
        };
        if let Some((raw_ty, wrapper_ty)) =
            crate::code_style::raw_text_return_ty(crate::types::SynTypeRef::from(&**ty))
        {
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
        if crate::code_style::item_fn_is_proc_macro(crate::types::SynItemFnRef::from(i)).get() {
            return;
        }
        self.check_sig(
            crate::types::SynSignatureRef::from(&i.sig),
            crate::types::SourceTextRef::from(format!("function `{}`", i.sig.ident).as_str()),
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
    fn visit_local(&mut self, i: &'ast syn::Local) {
        if let syn::Pat::Type(pat_ty) = &i.pat
            && let Some((raw_ty, wrapper_ty)) =
                crate::code_style::raw_text_return_ty(crate::types::SynTypeRef::from(&*pat_ty.ty))
            && raw_ty.get() != constants_str::STR
            && raw_ty.get() != constants_str::OPTION_STR
        {
            self.ers.push(format!(
                "{} uses `{}`; use `{}`",
                constants_str::LOCAL_BINDING,
                raw_ty.get(),
                wrapper_ty.get()
            ));
        }
        syn::visit::visit_local(self, i);
    }
}
impl<'ast> syn::visit::Visit<'ast> for ExternalLeafWrapperNameVisitor<'_> {
    fn visit_item_struct(&mut self, i: &'ast syn::ItemStruct) {
        if crate::code_style::attrs_contain_test_only_cfg(crate::types::SynAttributeListRef::from(
            i.attrs.as_slice(),
        ))
        .get()
        {
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
            crate::types::SynItemStructRef::from(i),
            crate::types::SynTypeRef::from(&field.ty),
        );
        syn::visit::visit_item_struct(self, i);
    }
}
#[allow(
    clippy::arbitrary_source_item_ordering,
    reason = "root and leaf path traversal helpers stay grouped by traversal direction"
)]
impl ExternalLeafWrapperNameVisitor<'_> {
    fn check_external_leaf_wrapper_name(
        &mut self,
        item: crate::types::SynItemStructRef<'_>,
        ty: crate::types::SynTypeRef<'_>,
    ) {
        let Some(leaf_segment) = self.external_leaf_segment(ty) else {
            return;
        };
        let Some(root_segment) = self.external_root_segment(ty) else {
            return;
        };
        let leaf_segment_ref = leaf_segment.get();
        let root_segment_ref = root_segment.get();
        let item_ref = item.as_ref();
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
        self.ers.push(format!(
            "tuple wrapper `{}` wraps external type `{}::{}`; rename it so it contains `{}`",
            item_ref.ident,
            root_segment_ref.ident,
            leaf_segment_ref.ident,
            expected_fragment.as_ref()
        ));
    }
    fn external_root_segment<'ty_lt>(
        &self,
        ty: crate::types::SynTypeRef<'ty_lt>,
    ) -> Option<crate::types::SynPathSegmentRef<'ty_lt>> {
        match ty.get() {
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
        arguments: crate::types::SynPathArgumentsRef<'args_lt>,
    ) -> Option<crate::types::SynPathSegmentRef<'args_lt>> {
        match arguments.get() {
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
        ty: crate::types::SynTypeRef<'ty_lt>,
    ) -> Option<crate::types::SynPathSegmentRef<'ty_lt>> {
        match ty.get() {
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
        arguments: crate::types::SynPathArgumentsRef<'args_lt>,
    ) -> Option<crate::types::SynPathSegmentRef<'args_lt>> {
        match arguments.get() {
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
