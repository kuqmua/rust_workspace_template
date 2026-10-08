#[test]
fn test_trait_emitters_preserve_conversion_bindings_generics_and_supplied_bodies() {
    let implementation_generics = quote::quote!(<T>);
    let identifier = quote::quote!(Value);
    let type_generics = quote::quote!(<T>);
    [
        (crate::generate_impl_display_token_stream::generate_impl_display_token_stream(&implementation_generics, &identifier, &type_generics, &quote::quote!(f.write_str(constants_str::X))), quote::quote! {
            impl<T> std::fmt::Display for Value<T> {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { f.write_str(constants_str::X) }
            }
        }),
        (crate::generate_impl_to_err_string_token_stream::generate_impl_to_err_string_token_stream(&implementation_generics, &identifier, &type_generics, &quote::quote!(String::from(constants_str::X))), quote::quote! {
            impl<T> to_err_string::to_err_string::ToErrString for Value<T> {
                fn to_err_string(&self) -> to_err_string::error_text::ErrorText {
                    to_err_string::error_text::ErrorText::try_from(String::from(constants_str::X)).unwrap_or_else(to_err_string::error_text::ErrorText::from)
                }
            }
        }),
        (crate::generate_impl_from_token_stream::generate_impl_from_token_stream(&quote::quote!(Input), &identifier, &quote::quote!(Self(v))), quote::quote! {
            impl From<Input> for Value {
                fn from(value: Input) -> Self { let v = value; Self(v) }
            }
        }),
        (crate::generate_impl_try_from_token_stream::generate_impl_try_from_token_stream(&quote::quote!(Input), &identifier, &quote::quote!(ValidationError), &quote::quote!(Ok(Self(v)))), quote::quote! {
            impl TryFrom<Input> for Value {
                type Error = ValidationError;
                fn try_from(value: Input) -> Result<Self, Self::Error> { let v = value; Ok(Self(v)) }
            }
        }),
    ].into_iter().fold((), |(), (generated, expected_tokens)| {
        let observed_result = syn::parse2::<syn::ItemImpl>(generated.as_ref().clone());
        assert!(observed_result.is_ok());
        let expected_result = syn::parse2::<syn::ItemImpl>(expected_tokens);
        assert!(expected_result.is_ok());
        assert_eq!(observed_result.ok(), expected_result.ok());
    });
}

#[test]
fn test_generated_write_error_guard_preserves_arguments_and_failure_body() {
    let message = syn::LitStr::new(constants_str::X, proc_macro2::Span::call_site());
    let parameters = quote::quote!(target, #message);
    let generated =
        crate::generate_if_write_is_error_token_stream::generate_if_write_is_error_token_stream(
            &parameters,
            &quote::quote!(return Err(WriteError);),
        );
    let observed_result = syn::parse2::<syn::ExprIf>(quote::quote!(#generated));
    assert!(observed_result.is_ok());
    let Ok(observed) = observed_result else {
        return;
    };
    assert_eq!(
        observed.then_branch,
        syn::parse_quote!({
            return Err(WriteError);
        })
    );
    assert!(observed.else_branch.is_none());
    assert!(matches!(observed.cond.as_ref(), syn::Expr::MethodCall(_)));
    let syn::Expr::MethodCall(condition) = observed.cond.as_ref() else {
        return;
    };
    assert_eq!(condition.method, stringify!(is_err));
    assert!(condition.args.is_empty());
    assert!(matches!(condition.receiver.as_ref(), syn::Expr::Block(_)));
    let syn::Expr::Block(receiver) = condition.receiver.as_ref() else {
        return;
    };
    assert!(matches!(
        receiver.block.stmts.first(),
        Some(syn::Stmt::Item(syn::Item::Use(_)))
    ));
    let Some(syn::Stmt::Item(syn::Item::Use(item_use))) = receiver.block.stmts.first() else {
        return;
    };
    assert!(matches!(&item_use.vis, syn::Visibility::Inherited));
    assert!(item_use.leading_colon.is_none());
    assert!(matches!(&item_use.tree, syn::UseTree::Path(_)));
    let syn::UseTree::Path(standard_library) = &item_use.tree else {
        return;
    };
    assert_eq!(standard_library.ident, stringify!(std));
    assert!(matches!(
        standard_library.tree.as_ref(),
        syn::UseTree::Path(_)
    ));
    let syn::UseTree::Path(formatting) = standard_library.tree.as_ref() else {
        return;
    };
    assert_eq!(formatting.ident, stringify!(fmt));
    assert!(matches!(formatting.tree.as_ref(), syn::UseTree::Rename(_)));
    let syn::UseTree::Rename(write_trait) = formatting.tree.as_ref() else {
        return;
    };
    assert_eq!(write_trait.ident, stringify!(Write));
    assert_eq!(write_trait.rename, stringify!(_));
    assert!(
        matches!(receiver.block.stmts.as_slice(), [syn::Stmt::Item(syn::Item::Use(_)), syn::Stmt::Expr(syn::Expr::Macro(statement), None)] if statement.mac.path == syn::parse_quote!(write) && statement.mac.tokens.to_string() == parameters.to_string())
    );
}
