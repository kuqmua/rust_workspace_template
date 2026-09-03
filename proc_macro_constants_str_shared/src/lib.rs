mod keyword {
    syn::custom_keyword!(constants);
    syn::custom_keyword!(fragments);
    syn::custom_keyword!(rust_constants);
    syn::custom_keyword!(rust_fragments);
}

const COLLECTION_MAX_LEN: usize = 10_000usize;

#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout)]
struct Constant {
    name: SynIdent,
    parts: ConstantParts,
    visibility: Option<SynVisibility>,
}

#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout)]
enum ConstantPart {
    Fragment(SynIdent),
    Literal(SynLitStr),
}

#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout)]
struct ConstantParts(Vec<ConstantPart>);

impl TryFrom<Vec<ConstantPart>> for ConstantParts {
    type Error = syn::Error;
    fn try_from(vec: Vec<ConstantPart>) -> Result<Self, Self::Error> {
        if vec.len() > COLLECTION_MAX_LEN {
            Err(syn::Error::new(
                proc_macro2::Span::call_site(),
                stringify!(c93f714a too many constant parts),
            ))
        } else {
            Ok(Self(vec))
        }
    }
}

impl syn::parse::Parse for ConstantParts {
    fn parse(parse_stream: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
        parse_stream
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

#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout)]
struct Constants(Vec<Constant>);

impl TryFrom<Vec<Constant>> for Constants {
    type Error = syn::Error;
    fn try_from(vec: Vec<Constant>) -> Result<Self, Self::Error> {
        if vec.len() > COLLECTION_MAX_LEN {
            Err(syn::Error::new(
                proc_macro2::Span::call_site(),
                stringify!(2bd1b963 too many constants),
            ))
        } else {
            Ok(Self(vec))
        }
    }
}

#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout)]
struct Fragment {
    name: SynIdent,
    value: SynLitStr,
}

#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout)]
struct RustFragment {
    name: SynIdent,
    parts: ConstantParts,
}

#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout)]
struct RustFragments(Vec<RustFragment>);

impl TryFrom<Vec<RustFragment>> for RustFragments {
    type Error = syn::Error;
    fn try_from(vec: Vec<RustFragment>) -> Result<Self, Self::Error> {
        if vec.len() <= COLLECTION_MAX_LEN {
            return Ok(Self(vec));
        }
        Err(syn::Error::new(
            proc_macro2::Span::call_site(),
            stringify!(c31f6dd7 too many Rust fragments),
        ))
    }
}

#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout)]
struct Fragments(Vec<Fragment>);

#[allow(
    clippy::useless_concat,
    reason = "the constants_str generator cannot depend on the crate that it generates"
)]
impl TryFrom<Vec<Fragment>> for Fragments {
    type Error = syn::Error;
    fn try_from(vec: Vec<Fragment>) -> Result<Self, Self::Error> {
        if vec.len() > COLLECTION_MAX_LEN {
            Err(syn::Error::new(
                proc_macro2::Span::call_site(),
                concat!("883ea6b2 too many fragments"),
            ))
        } else {
            Ok(Self(vec))
        }
    }
}

#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout)]
struct SynIdent(syn::Ident);

impl From<syn::Ident> for SynIdent {
    fn from(ident: syn::Ident) -> Self {
        Self(ident)
    }
}

impl syn::parse::Parse for SynIdent {
    fn parse(parse_stream: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
        parse_stream.parse().map(Self)
    }
}

#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout)]
struct SynLitStr(syn::LitStr);

impl From<syn::LitStr> for SynLitStr {
    fn from(lit_str: syn::LitStr) -> Self {
        Self(lit_str)
    }
}

impl syn::parse::Parse for SynLitStr {
    fn parse(parse_stream: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
        parse_stream.parse().map(Self)
    }
}

#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout)]
struct SynVisibility(syn::Visibility);

impl From<syn::Visibility> for SynVisibility {
    fn from(visibility: syn::Visibility) -> Self {
        Self(visibility)
    }
}

impl syn::parse::Parse for SynVisibility {
    fn parse(parse_stream: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
        parse_stream.parse().map(Self)
    }
}

#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout)]
pub(crate) struct DefineStrConstantsInput {
    constants: Constants,
    fragments: Fragments,
    rust_constants: Constants,
    rust_fragments: RustFragments,
}

impl syn::parse::Parse for DefineStrConstantsInput {
    fn parse(parse_stream: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
        let _: keyword::fragments = parse_stream.parse()?;
        let fragment_content;
        let _ = syn::braced!(fragment_content in parse_stream);
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

        let _: keyword::rust_fragments = parse_stream.parse()?;
        let rust_fragment_content;
        let _ = syn::braced!(rust_fragment_content in parse_stream);
        let mut raw_rust_fragments = Vec::new();
        while !rust_fragment_content.is_empty() {
            let name = rust_fragment_content.parse()?;
            let _: syn::Token![=] = rust_fragment_content.parse()?;
            let part_content;
            let _ = syn::bracketed!(part_content in rust_fragment_content);
            let parts = part_content.parse()?;
            let _: syn::Token![;] = rust_fragment_content.parse()?;
            raw_rust_fragments.push(RustFragment { name, parts });
        }
        let rust_fragments = RustFragments::try_from(raw_rust_fragments)?;

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

        let _: keyword::rust_constants = parse_stream.parse()?;
        let rust_constant_content;
        let _ = syn::braced!(rust_constant_content in parse_stream);
        let rust_constants = parse_constants(&rust_constant_content)?;

        let _: keyword::constants = parse_stream.parse()?;
        let constant_content;
        let _ = syn::braced!(constant_content in parse_stream);
        let constants = parse_constants(&constant_content)?;
        if parse_stream.is_empty() {
            Ok(Self {
                constants: Constants::try_from(constants)?,
                fragments: Fragments::try_from(fragments)?,
                rust_constants: Constants::try_from(rust_constants)?,
                rust_fragments,
            })
        } else {
            Err(parse_stream.error(stringify!(
                d53e729b unexpected tokens after constants block
            )))
        }
    }
}

impl From<DefineStrConstantsInput> for proc_macro2::TokenStream {
    fn from(define_str_constants_input: DefineStrConstantsInput) -> Self {
        let generated = (|| {
            let fragments = define_str_constants_input
                .fragments
                .0
                .into_iter()
                .try_fold(
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
            fragments.iter().try_for_each(|(name, fragment_value)| {
                if fragment_value.is_empty()
                    || !fragment_value
                        .chars()
                        .all(|char| char.is_ascii_alphanumeric() || char == '_')
                {
                    Err(syn::Error::new(
                        proc_macro2::Span::call_site(),
                        format!("3bc0da90: string fragment {name} must contain exactly one word"),
                    ))
                } else if !fragment_values.insert(fragment_value.clone()) {
                    Err(syn::Error::new(
                        proc_macro2::Span::call_site(),
                        format!(
                            "0566e947: string fragment value {fragment_value} is declared more than once"
                        ),
                    ))
                } else {
                    Ok(())
                }
            })?;
            let mut literal_word_counts = std::collections::BTreeMap::<String, usize>::new();
            let mut literal_syntax_counts = std::collections::BTreeMap::<String, usize>::new();
            let mut fragment_use_counts =
                fragments
                    .keys()
                    .fold(std::collections::BTreeMap::new(), |mut use_counts, name| {
                        let _: Option<usize> = use_counts.insert(name.clone(), 0usize);
                        use_counts
                    });
            let mut rust_fragment_use_counts =
                define_str_constants_input.rust_fragments.0.iter().fold(
                    std::collections::BTreeMap::new(),
                    |mut use_counts, fragment| {
                        let _: Option<usize> =
                            use_counts.insert(fragment.name.0.to_string(), 0usize);
                        use_counts
                    },
                );
            let rust_fragments = define_str_constants_input.rust_fragments.0.into_iter().try_fold(
                std::collections::BTreeMap::new(),
                |mut rust_fragments, fragment| {
                    let name = fragment.name.0.to_string();
                    if fragments.contains_key(&name) || rust_fragments.contains_key(&name) {
                        return Err(syn::Error::new(
                            fragment.name.0.span(),
                            stringify!(750ff794 duplicate Rust string fragment),
                        ));
                    }
                    let fragment_value = fragment.parts.0.into_iter().try_fold(
                        String::new(),
                        |mut accumulated, part| match part {
                            ConstantPart::Fragment(identifier) => {
                                let identifier_name = identifier.0.to_string();
                                let Some(fragment_value) = fragments.get(&identifier_name) else {
                                    return Err(syn::Error::new(
                                        identifier.0.span(),
                                        stringify!(38b81d16 Rust fragments may reference only word fragments),
                                    ));
                                };
                                if let Some(use_count) = fragment_use_counts.get_mut(&identifier_name) {
                                    *use_count = use_count.saturating_add(1usize);
                                }
                                accumulated.push_str(fragment_value);
                                Ok(accumulated)
                            }
                            ConstantPart::Literal(literal) => {
                                let literal_value = literal.0.value();
                                if literal_value.chars().any(|char| char.is_ascii_alphanumeric() || char == '_') {
                                    return Err(syn::Error::new(
                                        literal.0.span(),
                                        stringify!(9cf0b14e Rust fragment literals must contain syntax only),
                                    ));
                                }
                                accumulated.push_str(&literal_value);
                                Ok(accumulated)
                            }
                        },
                    )?;
                    if fragment_value.is_empty() {
                        return Err(syn::Error::new(
                            fragment.name.0.span(),
                            stringify!(f9805250 Rust string fragment must not be empty),
                        ));
                    }
                    drop(rust_fragments.insert(name, fragment_value));
                    Ok(rust_fragments)
                },
            )?;
            let rust_fragment_values = rust_fragments
                .values()
                .cloned()
                .collect::<std::collections::HashSet<_>>();
            let constant_count = define_str_constants_input
                .constants
                .0
                .len()
                .saturating_add(define_str_constants_input.rust_constants.0.len());
            let mut constants = define_str_constants_input
                .rust_constants
                .0
                .into_iter()
                .chain(define_str_constants_input.constants.0);
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
                    let constant_value = constant.parts.0.into_iter().try_fold(
                        String::new(),
                        |mut accumulated, part| match part {
                            ConstantPart::Fragment(identifier) => {
                                let identifier_name = identifier.0.to_string();
                                let fragment = if let Some(fragment) =
                                    fragments.get(&identifier_name)
                                {
                                    if let Some(use_count) =
                                        fragment_use_counts.get_mut(&identifier_name)
                                    {
                                        *use_count = use_count.saturating_add(1usize);
                                    }
                                    fragment
                                } else if let Some(fragment) = rust_fragments.get(&identifier_name)
                                {
                                    if let Some(use_count) =
                                        rust_fragment_use_counts.get_mut(&identifier_name)
                                    {
                                        *use_count = use_count.saturating_add(1usize);
                                    }
                                    fragment
                                } else {
                                    return Err(syn::Error::new(
                                        identifier.0.span(),
                                        stringify!(bb09ab55 unknown string fragment),
                                    ));
                                };
                                accumulated.push_str(fragment);
                                Ok(accumulated)
                            }
                            ConstantPart::Literal(literal) => {
                                let literal_value = literal.0.value();
                                if !literal_value.is_empty()
                                    && literal_value
                                        .chars()
                                        .all(|char| !char.is_ascii_alphanumeric() && char != '_')
                                {
                                    let syntax_count = literal_syntax_counts
                                        .entry(literal_value.clone())
                                        .or_insert(0usize);
                                    *syntax_count = syntax_count.saturating_add(1usize);
                                }
                                literal_value
                                    .split(|char: char| {
                                        !char.is_ascii_alphanumeric() && char != '_'
                                    })
                                    .filter(|word| !word.is_empty())
                                    .for_each(|word| {
                                        let word_count = literal_word_counts
                                            .entry(word.to_owned())
                                            .or_insert(0usize);
                                        *word_count = word_count.saturating_add(1usize);
                                    });
                                accumulated.push_str(&literal_value);
                                Ok(accumulated)
                            }
                        },
                    )?;
                    let literal = syn::LitStr::new(&constant_value, proc_macro2::Span::call_site());
                    let rhs = if let Some(previous_name) = values.get(&constant_value) {
                        quote::quote! { #previous_name }
                    } else {
                        drop(values.insert(constant_value, constant.name.0.clone()));
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
            if let Some((syntax, _)) = literal_syntax_counts
                .iter()
                .find(|(_, use_count)| **use_count > 1usize)
            {
                return Err(syn::Error::new(
                    proc_macro2::Span::call_site(),
                    format!(
                        "f37cb2a6: repeated syntax substring {syntax:?} must use a Rust string fragment"
                    ),
                ));
            }
            if let Some(syntax) = literal_syntax_counts
                .keys()
                .find(|syntax| rust_fragment_values.contains(*syntax))
            {
                return Err(syn::Error::new(
                    proc_macro2::Span::call_site(),
                    format!(
                        "c84d79e1: syntax substring {syntax:?} must use its declared Rust string fragment"
                    ),
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

#[must_use]
pub fn define_str_constants(token_stream: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    match syn::parse2::<DefineStrConstantsInput>(token_stream) {
        Ok(parsed) => proc_macro2::TokenStream::from(parsed),
        Err(error) => error.into_compile_error(),
    }
}

#[must_use]
pub fn define_git_info_constants(
    token_stream: &proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    if token_stream.is_empty() {
        quote::quote! {
            pub const GIT_INFO_PROJECT_GIT_COMMIT_ID: &str =
                git_version::git_version!(args = ["--always", "--abbrev=40"]);
            pub const GIT_INFO_PROJECT_GIT_COMMIT_LINK: &str = git_version::git_version!(
                args = ["--always", "--abbrev=40"],
                prefix = "https://github.com/kuqmua/rust_workspace_template/tree/"
            );
        }
    } else {
        syn::Error::new(
            proc_macro2::Span::call_site(),
            stringify!(78de8960 define_git_info_constants does not accept input),
        )
        .into_compile_error()
    }
}
