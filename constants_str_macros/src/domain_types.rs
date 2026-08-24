mod keyword {
    syn::custom_keyword!(constants);
    syn::custom_keyword!(fragments);
}
const COLLECTION_MAX_LEN: usize = 10_000usize;

#[derive(optimal_memory_layout::OptimalMemoryLayout)]
pub(crate) struct ProcMacroDefineStrConstantsInput(proc_macro::TokenStream);

impl From<proc_macro::TokenStream> for ProcMacroDefineStrConstantsInput {
    fn from(value: proc_macro::TokenStream) -> Self {
        Self(value)
    }
}

impl From<ProcMacroDefineStrConstantsInput> for proc_macro::TokenStream {
    fn from(value: ProcMacroDefineStrConstantsInput) -> Self {
        value.0
    }
}

#[derive(optimal_memory_layout::OptimalMemoryLayout)]
pub(crate) struct ProcMacroDefineStrConstantsOutput(proc_macro::TokenStream);

impl From<proc_macro::TokenStream> for ProcMacroDefineStrConstantsOutput {
    fn from(value: proc_macro::TokenStream) -> Self {
        Self(value)
    }
}

impl From<ProcMacroDefineStrConstantsOutput> for proc_macro::TokenStream {
    fn from(value: ProcMacroDefineStrConstantsOutput) -> Self {
        value.0
    }
}

#[derive(optimal_memory_layout::OptimalMemoryLayout)]
struct SynIdent(syn::Ident);
impl From<syn::Ident> for SynIdent {
    fn from(value: syn::Ident) -> Self {
        Self(value)
    }
}

impl syn::parse::Parse for SynIdent {
    fn parse(input: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
        input.parse().map(Self)
    }
}

#[derive(optimal_memory_layout::OptimalMemoryLayout)]
struct SynLitStr(syn::LitStr);
impl From<syn::LitStr> for SynLitStr {
    fn from(value: syn::LitStr) -> Self {
        Self(value)
    }
}

impl syn::parse::Parse for SynLitStr {
    fn parse(input: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
        input.parse().map(Self)
    }
}

#[derive(optimal_memory_layout::OptimalMemoryLayout)]
struct SynVisibility(syn::Visibility);
impl From<syn::Visibility> for SynVisibility {
    fn from(value: syn::Visibility) -> Self {
        Self(value)
    }
}

impl syn::parse::Parse for SynVisibility {
    fn parse(input: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
        input.parse().map(Self)
    }
}

#[derive(optimal_memory_layout::OptimalMemoryLayout)]
struct Fragment {
    name: SynIdent,
    value: SynLitStr,
}

#[derive(optimal_memory_layout::OptimalMemoryLayout)]
enum ConstantPart {
    Fragment(SynIdent),
    Literal(SynLitStr),
}
#[derive(optimal_memory_layout::OptimalMemoryLayout)]
struct ConstantParts(Vec<ConstantPart>);
impl TryFrom<Vec<ConstantPart>> for ConstantParts {
    type Error = syn::Error;
    fn try_from(value: Vec<ConstantPart>) -> Result<Self, Self::Error> {
        if value.len() > COLLECTION_MAX_LEN {
            Err(syn::Error::new(
                proc_macro2::Span::call_site(),
                stringify!(c93f714a too many constant parts),
            ))
        } else {
            Ok(Self(value))
        }
    }
}
#[derive(optimal_memory_layout::OptimalMemoryLayout)]
struct Constants(Vec<Constant>);
impl TryFrom<Vec<Constant>> for Constants {
    type Error = syn::Error;
    fn try_from(value: Vec<Constant>) -> Result<Self, Self::Error> {
        if value.len() > COLLECTION_MAX_LEN {
            Err(syn::Error::new(
                proc_macro2::Span::call_site(),
                stringify!(2bd1b963 too many constants),
            ))
        } else {
            Ok(Self(value))
        }
    }
}
#[derive(optimal_memory_layout::OptimalMemoryLayout)]
struct Fragments(Vec<Fragment>);
impl TryFrom<Vec<Fragment>> for Fragments {
    type Error = syn::Error;
    fn try_from(value: Vec<Fragment>) -> Result<Self, Self::Error> {
        if value.len() > COLLECTION_MAX_LEN {
            Err(syn::Error::new(
                proc_macro2::Span::call_site(),
                "883ea6b2 too many fragments",
            ))
        } else {
            Ok(Self(value))
        }
    }
}

#[derive(optimal_memory_layout::OptimalMemoryLayout)]
struct Constant {
    name: SynIdent,
    parts: ConstantParts,
    visibility: Option<SynVisibility>,
}

#[derive(optimal_memory_layout::OptimalMemoryLayout)]
struct DefineStrConstantsInput {
    constants: Constants,
    fragments: Fragments,
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
                .collect::<Vec<ConstantPart>>();
            let _: syn::Token![;] = constant_content.parse()?;
            constants.push(Constant {
                name,
                parts: ConstantParts::try_from(parts)?,
                visibility,
            });
        }
        if input.is_empty() {
            Ok(Self {
                constants: Constants::try_from(constants)?,
                fragments: Fragments::try_from(fragments)?,
            })
        } else {
            Err(input.error(stringify!(
                d53e729b unexpected tokens after constants block
            )))
        }
    }
}

#[allow(
    clippy::single_call_fn,
    reason = "the proc-macro entrypoint delegates parsing and expansion to the private domain model"
)]
pub(crate) fn expand(input: ProcMacroDefineStrConstantsInput) -> ProcMacroDefineStrConstantsOutput {
    let expand = |parsed: DefineStrConstantsInput| {
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
        let (_, _, generated) =
            parsed.constants.0.into_iter().try_fold(
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
                            ConstantPart::Fragment(identifier) => {
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
                            ConstantPart::Literal(literal) => {
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
        Ok(quote::quote! { #(#generated)* })
    };
    let output = match syn::parse::<DefineStrConstantsInput>(proc_macro::TokenStream::from(input))
        .and_then(expand)
    {
        Ok(generated) => proc_macro::TokenStream::from(generated),
        Err(error) => proc_macro::TokenStream::from(error.into_compile_error()),
    };
    ProcMacroDefineStrConstantsOutput::from(output)
}
