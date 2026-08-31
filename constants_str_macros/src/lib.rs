mod keyword {
    syn::custom_keyword!(constants);
    syn::custom_keyword!(fragments);
    syn::custom_keyword!(rust_constants);
    syn::custom_keyword!(rust_fragments);
}

const COLLECTION_MAX_LEN: usize = 10_000usize;

#[derive(optimal_memory_layout::OptimalMemoryLayout)]
struct Constant {
    name: SynIdent,
    parts: ConstantParts,
    visibility: Option<SynVisibility>,
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

impl syn::parse::Parse for ConstantParts {
    fn parse(input: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
        input
            .parse_terminated(
                |part_input| {
                    if part_input.peek(syn::LitStr) {
                        part_input.parse().map(ConstantPart::Literal)
                    } else {
                        part_input.parse().map(ConstantPart::Fragment)
                    }
                },
                syn::Token![,],
            )
            .map(|parsed_parts| {
                parsed_parts
                    .into_iter()
                    .fold(Vec::new(), |mut parts, part| {
                        parts.push(part);
                        parts
                    })
            })
            .and_then(Self::try_from)
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
struct Fragment {
    name: SynIdent,
    value: SynLitStr,
}

#[derive(optimal_memory_layout::OptimalMemoryLayout)]
struct RustFragment {
    name: SynIdent,
    parts: ConstantParts,
}

#[derive(optimal_memory_layout::OptimalMemoryLayout)]
struct Fragments(Vec<Fragment>);

#[allow(
    clippy::useless_concat,
    reason = "the constants_str generator cannot depend on the crate that it generates"
)]
impl TryFrom<Vec<Fragment>> for Fragments {
    type Error = syn::Error;
    fn try_from(value: Vec<Fragment>) -> Result<Self, Self::Error> {
        if value.len() > COLLECTION_MAX_LEN {
            Err(syn::Error::new(
                proc_macro2::Span::call_site(),
                concat!("883ea6b2 too many fragments"),
            ))
        } else {
            Ok(Self(value))
        }
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
pub(crate) struct DefineStrConstantsInput {
    constants: Constants,
    fragments: Fragments,
    rust_constants: Constants,
    rust_fragments: Vec<RustFragment>,
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

        let _: keyword::rust_fragments = input.parse()?;
        let rust_fragment_content;
        let _ = syn::braced!(rust_fragment_content in input);
        let mut rust_fragments = Vec::new();
        while !rust_fragment_content.is_empty() {
            let name = rust_fragment_content.parse()?;
            let _: syn::Token![=] = rust_fragment_content.parse()?;
            let part_content;
            let _ = syn::bracketed!(part_content in rust_fragment_content);
            let parts = part_content.parse()?;
            let _: syn::Token![;] = rust_fragment_content.parse()?;
            rust_fragments.push(RustFragment { name, parts });
        }
        if rust_fragments.len() > COLLECTION_MAX_LEN {
            return Err(syn::Error::new(
                proc_macro2::Span::call_site(),
                stringify!(c31f6dd7 too many Rust fragments),
            ));
        }

        let parse_constants = |content: syn::parse::ParseStream<'_>| {
            let mut constants = Vec::new();
            while !content.is_empty() {
                let visibility = if content.peek(syn::Token![pub]) {
                    Some(content.parse()?)
                } else {
                    None
                };
                let name = content.parse()?;
                let _: syn::Token![=] = content.parse()?;
                let part_content;
                let _ = syn::bracketed!(part_content in content);
                let parts = part_content.parse()?;
                let _: syn::Token![;] = content.parse()?;
                constants.push(Constant {
                    name,
                    parts,
                    visibility,
                });
            }
            Ok::<Vec<Constant>, syn::Error>(constants)
        };

        let _: keyword::rust_constants = input.parse()?;
        let rust_constant_content;
        let _ = syn::braced!(rust_constant_content in input);
        let rust_constants = parse_constants(&rust_constant_content)?;

        let _: keyword::constants = input.parse()?;
        let constant_content;
        let _ = syn::braced!(constant_content in input);
        let constants = parse_constants(&constant_content)?;
        if input.is_empty() {
            Ok(Self {
                constants: Constants::try_from(constants)?,
                fragments: Fragments::try_from(fragments)?,
                rust_constants: Constants::try_from(rust_constants)?,
                rust_fragments,
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
            let fragments = parsed.fragments.0.into_iter().try_fold(
                std::collections::BTreeMap::new(),
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
            let mut fragment_values = std::collections::HashSet::with_capacity(fragments.len());
            fragments.iter().try_for_each(|(name, value)| {
                if value.is_empty()
                    || !value
                        .chars()
                        .all(|char| char.is_ascii_alphanumeric() || char == '_')
                {
                    Err(syn::Error::new(
                        proc_macro2::Span::call_site(),
                        format!("3bc0da90: string fragment {name} must contain exactly one word"),
                    ))
                } else if !fragment_values.insert(value.clone()) {
                    Err(syn::Error::new(
                        proc_macro2::Span::call_site(),
                        format!(
                            "0566e947: string fragment value {value} is declared more than once"
                        ),
                    ))
                } else {
                    Ok(())
                }
            })?;
            let mut literal_word_counts = std::collections::BTreeMap::<String, usize>::new();
            let mut fragment_use_counts =
                fragments
                    .keys()
                    .fold(std::collections::BTreeMap::new(), |mut use_counts, name| {
                        let _: Option<usize> = use_counts.insert(name.clone(), 0usize);
                        use_counts
                    });
            let mut rust_fragment_use_counts = parsed.rust_fragments.iter().fold(
                std::collections::BTreeMap::new(),
                |mut use_counts, fragment| {
                    let _: Option<usize> = use_counts.insert(fragment.name.0.to_string(), 0usize);
                    use_counts
                },
            );
            let rust_fragments = parsed.rust_fragments.into_iter().try_fold(
                std::collections::BTreeMap::new(),
                |mut rust_fragments, fragment| {
                    let name = fragment.name.0.to_string();
                    if fragments.contains_key(&name) || rust_fragments.contains_key(&name) {
                        return Err(syn::Error::new(
                            fragment.name.0.span(),
                            stringify!(750ff794 duplicate Rust string fragment),
                        ));
                    }
                    let value = fragment.parts.0.into_iter().try_fold(
                        String::new(),
                        |mut value, part| match part {
                            ConstantPart::Fragment(identifier) => {
                                let identifier_name = identifier.0.to_string();
                                let Some(fragment_value) = fragments.get(&identifier_name) else {
                                    return Err(syn::Error::new(
                                        identifier.0.span(),
                                        stringify!(38b81d16 Rust fragments may reference only word fragments),
                                    ));
                                };
                                if let Some(use_count) = fragment_use_counts.get_mut(&identifier_name) {
                                    *use_count = use_count.checked_add(1usize).expect(
                                        "70a6cff0 fragment use count must fit into usize",
                                    );
                                }
                                value.push_str(fragment_value);
                                Ok(value)
                            }
                            ConstantPart::Literal(literal) => {
                                let literal_value = literal.0.value();
                                if literal_value.chars().any(|char| char.is_ascii_alphanumeric() || char == '_') {
                                    return Err(syn::Error::new(
                                        literal.0.span(),
                                        stringify!(9cf0b14e Rust fragment literals must contain syntax only),
                                    ));
                                }
                                value.push_str(&literal_value);
                                Ok(value)
                            }
                        },
                    )?;
                    if value.is_empty() {
                        return Err(syn::Error::new(
                            fragment.name.0.span(),
                            stringify!(f9805250 Rust string fragment must not be empty),
                        ));
                    }
                    drop(rust_fragments.insert(name, value));
                    Ok(rust_fragments)
                },
            )?;
            let constant_count = parsed
                .constants
                .0
                .len()
                .checked_add(parsed.rust_constants.0.len())
                .expect("c92fd20a constant count must fit into usize");
            let mut constants = parsed
                .rust_constants
                .0
                .into_iter()
                .chain(parsed.constants.0);
            let (_, _, generated) = constants.try_fold(
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
                                let identifier_name = identifier.0.to_string();
                                let fragment = if let Some(fragment) =
                                    fragments.get(&identifier_name)
                                {
                                    if let Some(use_count) =
                                        fragment_use_counts.get_mut(&identifier_name)
                                    {
                                        *use_count = use_count.checked_add(1usize).expect(
                                            "8a3f36c7 fragment use count must fit into usize",
                                        );
                                    }
                                    fragment
                                } else if let Some(fragment) = rust_fragments.get(&identifier_name)
                                {
                                    if let Some(use_count) =
                                        rust_fragment_use_counts.get_mut(&identifier_name)
                                    {
                                        *use_count = use_count.checked_add(1usize).expect(
                                            "29ffdc5e Rust fragment use count must fit into usize",
                                        );
                                    }
                                    fragment
                                } else {
                                    return Err(syn::Error::new(
                                        identifier.0.span(),
                                        stringify!(bb09ab55 unknown string fragment),
                                    ));
                                };
                                value.push_str(fragment);
                                Ok(value)
                            }
                            ConstantPart::Literal(literal) => {
                                let literal_value = literal.0.value();
                                literal_value
                                    .split(|char: char| {
                                        !char.is_ascii_alphanumeric() && char != '_'
                                    })
                                    .filter(|word| !word.is_empty())
                                    .for_each(|word| {
                                        let word_count = literal_word_counts
                                            .entry(word.to_owned())
                                            .or_insert(0usize);
                                        *word_count = word_count.checked_add(1usize).expect(
                                            "52d520cb literal word count must fit into usize",
                                        );
                                    });
                                value.push_str(&literal_value);
                                Ok(value)
                            }
                        },
                    )?;
                    let literal = syn::LitStr::new(&value, proc_macro2::Span::call_site());
                    let rhs = if let Some(previous_name) = values.get(&value) {
                        quote::quote! { #previous_name }
                    } else {
                        drop(values.insert(value, constant.name.0.clone()));
                        quote::quote! { #literal }
                    };
                    let name = constant.name.0;
                    if let Some(visibility) = constant.visibility {
                        let syn_visibility = visibility.0;
                        generated.push(quote::quote! {
                            #syn_visibility const #name: &str = #rhs;
                        });
                    } else {
                        generated.push(quote::quote! {
                            const #name: &str = #rhs;
                        });
                    }
                    Ok((names, values, generated))
                },
            )?;
            if let Some((name, _)) = fragment_use_counts
                .iter()
                .find(|(_, use_count)| **use_count < 2usize)
            {
                return Err(syn::Error::new(
                    proc_macro2::Span::call_site(),
                    format!("34090e38: string fragment {name} must be reused"),
                ));
            }
            if let Some((name, _)) = rust_fragment_use_counts
                .iter()
                .find(|(_, use_count)| **use_count < 2usize)
            {
                return Err(syn::Error::new(
                    proc_macro2::Span::call_site(),
                    format!("485d9907: Rust string fragment {name} must be reused"),
                ));
            }
            if let Some((word, _)) = literal_word_counts
                .iter()
                .find(|(_, use_count)| **use_count > 1usize)
            {
                return Err(syn::Error::new(
                    proc_macro2::Span::call_site(),
                    format!("5515a1e9: repeated word {word} must use a string fragment"),
                ));
            }
            if let Some(word) = literal_word_counts
                .keys()
                .find(|word| fragment_values.contains(*word))
            {
                return Err(syn::Error::new(
                    proc_macro2::Span::call_site(),
                    format!("fe0fa60a: word {word} must use its declared string fragment"),
                ));
            }
            Ok::<Self, syn::Error>(quote::quote! { #(#generated)* })
        })();
        match generated {
            Ok(tokens) => tokens,
            Err(error) => error.into_compile_error(),
        }
    }
}

#[proc_macro]
pub fn define_str_constants(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    match syn::parse::<DefineStrConstantsInput>(input) {
        Ok(parsed) => proc_macro::TokenStream::from(proc_macro2::TokenStream::from(parsed)),
        Err(error) => proc_macro::TokenStream::from(error.into_compile_error()),
    }
}

#[proc_macro]
pub fn define_git_info_constants(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    if input.is_empty() {
        proc_macro::TokenStream::from(quote::quote! {
            pub const GIT_INFO_PROJECT_GIT_COMMIT_ID: &str =
                git_version::git_version!(args = ["--always", "--abbrev=40"]);
            pub const GIT_INFO_PROJECT_GIT_COMMIT_LINK: &str = git_version::git_version!(
                args = ["--always", "--abbrev=40"],
                prefix = "https://github.com/kuqmua/rust_workspace_template/tree/"
            );
        })
    } else {
        proc_macro::TokenStream::from(
            syn::Error::new(
                proc_macro2::Span::call_site(),
                stringify!(78de8960 define_git_info_constants does not accept input),
            )
            .into_compile_error(),
        )
    }
}
