mod keyword {
    syn::custom_keyword!(constants);
    syn::custom_keyword!(fragments);
}

struct SynIdent(syn::Ident);

impl syn::parse::Parse for SynIdent {
    fn parse(input: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
        input.parse().map(Self)
    }
}

struct SynLitStr(syn::LitStr);

impl syn::parse::Parse for SynLitStr {
    fn parse(input: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
        input.parse().map(Self)
    }
}

struct SynVisibility(syn::Visibility);

impl syn::parse::Parse for SynVisibility {
    fn parse(input: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
        input.parse().map(Self)
    }
}

struct Fragment {
    name: SynIdent,
    value: SynLitStr,
}

enum ConstantPart {
    Fragment(SynIdent),
    Literal(SynLitStr),
}

struct Constant {
    name: SynIdent,
    parts: Vec<ConstantPart>,
    visibility: Option<SynVisibility>,
}

struct DefineStrConstantsInput {
    constants: Vec<Constant>,
    fragments: Vec<Fragment>,
}

impl syn::parse::Parse for DefineStrConstantsInput {
    fn parse(input: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
        let _: keyword::fragments = input.parse()?;
        let fragment_content;
        let _ = syn::braced!(fragment_content in input);
        let mut fragments = Vec::new();
        while !fragment_content.is_empty() {
            fragments.push(Fragment {
                name: fragment_content.parse()?,
                value: {
                    let _: syn::Token![=] = fragment_content.parse()?;
                    fragment_content.parse()?
                },
            });
            let _: syn::Token![;] = fragment_content.parse()?;
        }

        let _: keyword::constants = input.parse()?;
        let constant_content;
        let _ = syn::braced!(constant_content in input);
        let mut constants = Vec::new();
        while !constant_content.is_empty() {
            let visibility = if constant_content.peek(syn::Token![pub]) {
                Some(constant_content.parse()?)
            } else {
                None
            };
            let name = constant_content.parse()?;
            let _: syn::Token![=] = constant_content.parse()?;
            let part_content;
            let _ = syn::bracketed!(part_content in constant_content);
            let parts = part_content
                .parse_terminated(
                    |part_input| {
                        if part_input.peek(syn::LitStr) {
                            part_input.parse().map(ConstantPart::Literal)
                        } else {
                            part_input.parse().map(ConstantPart::Fragment)
                        }
                    },
                    syn::Token![,],
                )?
                .into_iter()
                .collect();
            let _: syn::Token![;] = constant_content.parse()?;
            constants.push(Constant {
                name,
                parts,
                visibility,
            });
        }
        if input.is_empty() {
            Ok(Self {
                constants,
                fragments,
            })
        } else {
            Err(input.error(stringify!(
                d53e729b unexpected tokens after constants block
            )))
        }
    }
}

#[proc_macro]
pub fn define_str_constants(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let expand = |parsed: DefineStrConstantsInput| {
        let fragments = parsed.fragments.into_iter().try_fold(
            std::collections::HashMap::new(),
            |mut fragments, fragment| {
                let name = fragment.name.0.to_string();
                if fragments.insert(name, fragment.value.0.value()).is_some() {
                    Err(syn::Error::new(
                        fragment.name.0.span(),
                        stringify!(5bbbde57 duplicate string fragment),
                    ))
                } else {
                    Ok(fragments)
                }
            },
        )?;

        let (_, _, generated) = parsed.constants.into_iter().try_fold(
            (
                std::collections::HashSet::new(),
                std::collections::HashMap::new(),
                Vec::new(),
            ),
            |(mut names, mut values, mut generated), constant| {
                if !names.insert(constant.name.0.to_string()) {
                    return Err(syn::Error::new(
                        constant.name.0.span(),
                        stringify!(ad857256 duplicate string constant name),
                    ));
                }
                let value = constant.parts.into_iter().try_fold(
                    String::new(),
                    |mut value, part| match part {
                        ConstantPart::Fragment(identifier) => {
                            let Some(fragment) = fragments.get(&identifier.0.to_string()) else {
                                return Err(syn::Error::new(
                                    identifier.0.span(),
                                    stringify!(bb09ab55 unknown string fragment),
                                ));
                            };
                            value.push_str(fragment);
                            Ok(value)
                        }
                        ConstantPart::Literal(literal) => {
                            value.push_str(&literal.0.value());
                            Ok(value)
                        }
                    },
                )?;
                if let Some(previous_name) =
                    values.insert(value.clone(), constant.name.0.to_string())
                {
                    return Err(syn::Error::new(
                        constant.name.0.span(),
                        format!(
                            "2370f7b3: string constant duplicates the value of {previous_name}"
                        ),
                    ));
                }
                let name = constant.name.0;
                let literal = syn::LitStr::new(&value, proc_macro2::Span::call_site());
                if let Some(visibility) = constant.visibility {
                    let syn_visibility = visibility.0;
                    generated.push(quote::quote! {
                        #syn_visibility const #name: &str = #literal;
                    });
                } else {
                    generated.push(quote::quote! {
                        pub const #name: &str = #literal;
                    });
                }
                Ok((names, values, generated))
            },
        )?;
        Ok(quote::quote! { #(#generated)* })
    };
    match syn::parse::<DefineStrConstantsInput>(input).and_then(expand) {
        Ok(generated) => generated.into(),
        Err(error) => error.into_compile_error().into(),
    }
}
