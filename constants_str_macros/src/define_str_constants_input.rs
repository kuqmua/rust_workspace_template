#[derive(optimal_memory_layout::OptimalMemoryLayout)]
pub(crate) struct DefineStrConstantsInput {
    constants: crate::constants::Constants,
    fragments: crate::fragments::Fragments,
}

impl syn::parse::Parse for DefineStrConstantsInput {
    fn parse(input: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
        let _: super::keyword::fragments = input.parse()?;
        let fragment_content;
        let _ = syn::braced!(fragment_content in input);
        let mut fragments = Vec::new();
        while !fragment_content.is_empty() {
            fragments.push(crate::fragment::Fragment {
                name: fragment_content.parse()?,
                value: {
                    let _: syn::Token![=] = fragment_content.parse()?;
                    fragment_content.parse()?
                },
            });
            let _: syn::Token![;] = fragment_content.parse()?;
        }

        let _: super::keyword::constants = input.parse()?;
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
                            part_input
                                .parse()
                                .map(crate::constant_part::ConstantPart::Literal)
                        } else {
                            part_input
                                .parse()
                                .map(crate::constant_part::ConstantPart::Fragment)
                        }
                    },
                    syn::Token![,],
                )?
                .into_iter()
                .collect::<Vec<crate::constant_part::ConstantPart>>();
            let _: syn::Token![;] = constant_content.parse()?;
            constants.push(crate::constant::Constant {
                name,
                parts: crate::constant_parts::ConstantParts::try_from(parts)?,
                visibility,
            });
        }
        if input.is_empty() {
            Ok(Self {
                constants: crate::constants::Constants::try_from(constants)?,
                fragments: crate::fragments::Fragments::try_from(fragments)?,
            })
        } else {
            Err(input.error(stringify!(
                d53e729b unexpected tokens after constants block
            )))
        }
    }
}

impl From<DefineStrConstantsInput> for proc_macro2::TokenStream {
    fn from(parsed: DefineStrConstantsInput) -> Self {
        let generated = (|| {
            let fragment_count = parsed.fragments.0.len();
            let fragments = parsed.fragments.0.into_iter().try_fold(
                std::collections::HashMap::with_capacity(fragment_count),
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
            let constant_count = parsed.constants.0.len();
            let (_, _, generated) = parsed.constants.0.into_iter().try_fold(
                (
                    std::collections::HashSet::with_capacity(constant_count),
                    std::collections::HashMap::with_capacity(constant_count),
                    Vec::with_capacity(constant_count),
                ),
                |(mut names, mut values, mut generated), constant| {
                    if !names.insert(constant.name.0.to_string()) {
                        return Err(syn::Error::new(
                            constant.name.0.span(),
                            stringify!(ad857256 duplicate string constant name),
                        ));
                    }
                    let value = constant.parts.0.into_iter().try_fold(
                        String::new(),
                        |mut value, part| match part {
                            crate::constant_part::ConstantPart::Fragment(identifier) => {
                                let Some(fragment) = fragments.get(&identifier.0.to_string())
                                else {
                                    return Err(syn::Error::new(
                                        identifier.0.span(),
                                        stringify!(bb09ab55 unknown string fragment),
                                    ));
                                };
                                value.push_str(fragment);
                                Ok(value)
                            }
                            crate::constant_part::ConstantPart::Literal(literal) => {
                                value.push_str(&literal.0.value());
                                Ok(value)
                            }
                        },
                    )?;
                    let literal = syn::LitStr::new(&value, proc_macro2::Span::call_site());
                    if let Some(previous_name) = values.insert(value, constant.name.0.to_string()) {
                        return Err(syn::Error::new(
                            constant.name.0.span(),
                            format!(
                                "2370f7b3: string constant duplicates the value of {previous_name}"
                            ),
                        ));
                    }
                    let name = constant.name.0;
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
            Ok::<Self, syn::Error>(quote::quote! { #(#generated)* })
        })();
        match generated {
            Ok(tokens) => tokens,
            Err(error) => error.into_compile_error(),
        }
    }
}
