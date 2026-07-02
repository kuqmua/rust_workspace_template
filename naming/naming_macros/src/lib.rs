struct CaseTraitBodyExpression(syn::Expr);

struct CaseTraitBoundPath(syn::Path);

struct CaseTraitSelfReferenceIdentifier(syn::Ident);

struct CaseTraitStringTraitIdentifier(syn::Ident);

struct CaseTraitTokenStreamTraitIdentifier(syn::Ident);

struct CaseTraitPairInput {
    body_expression: CaseTraitBodyExpression,
    bound_path: CaseTraitBoundPath,
    self_reference_identifier: CaseTraitSelfReferenceIdentifier,
    string_trait_identifier: CaseTraitStringTraitIdentifier,
    token_stream_trait_identifier: CaseTraitTokenStreamTraitIdentifier,
}

impl syn::parse::Parse for CaseTraitPairInput {
    fn parse(input: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
        let string_trait_identifier = input.parse::<syn::Ident>()?;
        let _first_comma_token = input.parse::<syn::Token![,]>()?;
        let token_stream_trait_identifier = input.parse::<syn::Ident>()?;
        let _second_comma_token = input.parse::<syn::Token![,]>()?;
        let bound_path = input.parse::<syn::Path>()?;
        let _third_comma_token = input.parse::<syn::Token![,]>()?;
        let _left_or_token = input.parse::<syn::Token![|]>()?;
        let self_reference_identifier = input.parse::<syn::Ident>()?;
        let _right_or_token = input.parse::<syn::Token![|]>()?;
        let body_expression = input.parse::<syn::Expr>()?;
        Ok(Self {
            body_expression: CaseTraitBodyExpression(body_expression),
            bound_path: CaseTraitBoundPath(bound_path),
            self_reference_identifier: CaseTraitSelfReferenceIdentifier(self_reference_identifier),
            string_trait_identifier: CaseTraitStringTraitIdentifier(string_trait_identifier),
            token_stream_trait_identifier: CaseTraitTokenStreamTraitIdentifier(
                token_stream_trait_identifier,
            ),
        })
    }
}

#[proc_macro]
pub fn case_trait_pair(input_token_stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input: CaseTraitPairInput = match syn::parse(input_token_stream) {
        Ok(input) => input,
        Err(error) => return error.to_compile_error().into(),
    };
    let CaseTraitPairInput {
        body_expression,
        bound_path,
        self_reference_identifier,
        string_trait_identifier,
        token_stream_trait_identifier,
    } = input;
    let CaseTraitBodyExpression(body_expression) = body_expression;
    let CaseTraitBoundPath(bound_path) = bound_path;
    let CaseTraitSelfReferenceIdentifier(self_reference_identifier) = self_reference_identifier;
    let CaseTraitStringTraitIdentifier(string_trait_identifier) = string_trait_identifier;
    let CaseTraitTokenStreamTraitIdentifier(token_stream_trait_identifier) =
        token_stream_trait_identifier;
    let bound_token_stream = if bound_path.leading_colon.is_none()
        && bound_path.segments.len() == 1
        && bound_path
            .segments
            .first()
            .is_some_and(|segment| segment.ident == "Display")
    {
        quote::quote! { core::fmt::Display }
    } else {
        quote::quote! { #bound_path }
    };
    quote::quote! {
        pub trait #string_trait_identifier {
            #[must_use]
            fn case(&self) -> impl AsRef<str>;
        }

        impl<T> #string_trait_identifier for T
        where
            T: #bound_token_stream,
        {
            fn case(&self) -> impl AsRef<str> {
                let #self_reference_identifier = self;
                #body_expression
            }
        }

        pub trait #token_stream_trait_identifier {
            #[must_use]
            fn case_or_panic(&self) -> proc_macro2::TokenStream;
        }

        impl<T> #token_stream_trait_identifier for T
        where
            T: #string_trait_identifier,
        {
            fn case_or_panic(&self) -> proc_macro2::TokenStream {
                let case_text = #string_trait_identifier::case(self);
                match case_text.as_ref().parse::<proc_macro2::TokenStream>() {
                    Ok(token_stream) => token_stream,
                    Err(parse_error) => {
                        let error_message = parse_error.to_string();
                        let escaped_message = format!("{error_message:?}");
                        let compile_error = format!("compile_error!({escaped_message})");
                        Result::unwrap_or_else(
                            compile_error.parse::<proc_macro2::TokenStream>(),
                            |_| proc_macro2::TokenStream::new(),
                        )
                    }
                }
            }
        }
    }
    .into()
}
