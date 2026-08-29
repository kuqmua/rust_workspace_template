const PRODUCTION_MODULE_MAX_LINES: usize = 2_500usize;
const INLINE_TEST_SEPARATION_MIN_LINES: usize = 1_024usize;

fn identifier_snake_case(identifier: &syn::Ident) -> super::types::SourceText {
    let characters = identifier.to_string().chars().collect::<Vec<_>>();
    super::types::SourceText::try_from(characters.iter().enumerate().fold(
        String::new(),
        |mut output, (index, character)| {
            let uppercase = character.is_ascii_uppercase();
            let previous_is_lowercase_or_digit = index
                .checked_sub(constants_usize::ONE)
                .and_then(|previous_index| characters.get(previous_index))
                .is_some_and(|previous_character| {
                    previous_character.is_ascii_lowercase() || previous_character.is_ascii_digit()
                });
            let next_is_lowercase = characters
                .get(index.saturating_add(1))
                .is_some_and(char::is_ascii_lowercase);
            if uppercase
                && index != constants_usize::ZERO
                && (previous_is_lowercase_or_digit || next_is_lowercase)
            {
                output.push('_');
            }
            output.push(character.to_ascii_lowercase());
            output
        },
    ))
    .expect("3c8a729e identifier snake case must fit the source text bound")
}

#[test]
fn custom_type_names_are_unique_across_workspace() {
    super::code_style_snapshot::with_codebase_snapshot(|snapshot| {
        let mut declarations = std::collections::BTreeMap::<String, Vec<String>>::new();
        snapshot.rs_files().iter().for_each(|source_file| {
            let visitor = super::visit_syn_file(
                super::types::SynFileRef::from(source_file.ast().as_ref()),
                super::source_analysis::CustomTypeNameVisitor::default(),
            );
            visitor.names.into_iter().for_each(|name| {
                declarations
                    .entry(name)
                    .or_default()
                    .push(source_file.path().as_ref().display().to_string());
            });
        });
        let violations = declarations
            .into_iter()
            .filter_map(|(name, mut paths)| {
                (paths.len() > constants_usize::ONE).then(|| {
                    paths.sort();
                    format!(
                        "custom type name `{name}` is declared more than once across the workspace: {}",
                        paths.join(", ")
                    )
                })
            })
            .collect::<Vec<String>>();
        assert!(
            violations.is_empty(),
            "c036c2fb custom type names must be unique across all workspace modules and crates:\n{}",
            violations.join("\n")
        );
    });
}

#[test]
fn custom_type_name_visitor_covers_all_rust_type_declarations() {
    let ast = syn::parse_file(constants_str::CODE_STYLE_TYPE_DECLARATIONS_FIXTURE)
        .expect("a9ea85b6 custom type declaration fixture must parse");
    let visitor = super::visit_syn_file(
        super::types::SynFileRef::from(&ast),
        super::source_analysis::CustomTypeNameVisitor::default(),
    );
    assert_eq!(
        visitor.names.into_iter().collect::<Vec<String>>(),
        [
            "StructName",
            "EnumName",
            "UnionName",
            "TraitName",
            "AliasName",
            "TraitAliasName",
        ]
        .map(String::from),
        "29e8555b"
    );
}

#[test]
fn free_function_name_visitor_excludes_methods() {
    let ast = syn::parse_file(constants_str::CODE_STYLE_FREE_FUNCTION_DECLARATIONS_FIXTURE)
        .expect("31495514 free function declaration fixture must parse");
    let visitor = super::visit_syn_file(
        super::types::SynFileRef::from(&ast),
        super::source_analysis::FreeFnNameVisitor::default(),
    );
    assert_eq!(
        visitor.names.into_iter().collect::<Vec<String>>(),
        ["outer", "inner", "module_function"].map(String::from),
        "6d857f95"
    );
}

#[test]
fn free_function_names_are_unique_across_workspace() {
    super::code_style_snapshot::with_codebase_snapshot(|snapshot| {
        let mut declarations = std::collections::BTreeMap::<String, Vec<String>>::new();
        snapshot.rs_files().iter().for_each(|source_file| {
            let visitor = super::visit_syn_file(
                super::types::SynFileRef::from(source_file.ast().as_ref()),
                super::source_analysis::FreeFnNameVisitor::default(),
            );
            visitor
                .names
                .into_iter()
                .filter(|name| name != constants_str::MAIN)
                .for_each(|name| {
                    declarations
                        .entry(name)
                        .or_default()
                        .push(source_file.path().as_ref().display().to_string());
                });
        });
        let violations = declarations
            .into_iter()
            .filter_map(|(name, mut paths)| {
                (paths.len() > constants_usize::ONE).then(|| {
                    paths.sort();
                    format!(
                        "free function name `{name}` is declared more than once across the workspace: {}",
                        paths.join(", ")
                    )
                })
            })
            .collect::<Vec<String>>();
        assert!(
            violations.is_empty(),
            "31495514 free function names must be unique across all workspace modules and crates:\n{}",
            violations.join("\n")
        );
    });
}

#[test]
fn error_types_do_not_only_wrap_other_repository_errors() {
    super::code_style_snapshot::with_codebase_snapshot(|snapshot| {
        let repository_error_names = snapshot
            .rs_files()
            .iter()
            .flat_map(|source_file| source_file.ast().as_ref().items.iter())
            .filter_map(|item| {
                let syn::Item::Struct(item_struct) = item else {
                    return None;
                };
                item_struct
                    .ident
                    .to_string()
                    .ends_with(constants_str::ERROR)
                    .then(|| item_struct.ident.to_string())
            })
            .collect::<std::collections::BTreeSet<_>>();
        let violations = snapshot
            .rs_files()
            .iter()
            .flat_map(|source_file| {
                let source_path = source_file.path().as_ref();
                source_file
                    .ast()
                    .as_ref()
                    .items
                    .iter()
                    .filter_map(|item| {
                        let syn::Item::Struct(item_struct) = item else {
                            return None;
                        };
                        if !item_struct
                            .ident
                            .to_string()
                            .ends_with(constants_str::ERROR)
                        {
                            return None;
                        }
                        let syn::Fields::Unnamed(fields) = &item_struct.fields else {
                            return None;
                        };
                        if fields.unnamed.len() != constants_usize::ONE {
                            return None;
                        }
                        let field = fields.unnamed.first()?;
                        let syn::Type::Path(inner_type) = &field.ty else {
                            return None;
                        };
                        let repository_qualified = inner_type.path.segments.len()
                            == constants_usize::ONE
                            || inner_type.path.segments.first().is_some_and(|segment| {
                                matches!(
                                    segment.ident.to_string().as_str(),
                                    constants_str::CODE_STYLE_CRATE_PATH_SEGMENT
                                        | constants_str::CODE_STYLE_SELF_PATH_SEGMENT
                                        | constants_str::CODE_STYLE_SUPER_PATH_SEGMENT
                                )
                            })
                            || inner_type
                                .path
                                .segments
                                .iter()
                                .any(|segment| {
                                    segment.ident
                                        == constants_str::CODE_STYLE_DOMAIN_TYPES_PATH_SEGMENT
                                });
                        if !repository_qualified {
                            return None;
                        }
                        let inner_name = inner_type.path.segments.last()?.ident.to_string();
                        if !repository_error_names.contains(&inner_name) {
                            return None;
                        }
                        let adds_context = item_struct.attrs.iter().any(|attribute| {
                            if !attribute
                                .path()
                                .is_ident(constants_str::CODE_STYLE_ERROR_ATTRIBUTE)
                            {
                                return false;
                            }
                            let syn::Meta::List(list) = &attribute.meta else {
                                return false;
                            };
                            syn::parse2::<syn::LitStr>(list.tokens.clone()).is_ok_and(|message| {
                                    !matches!(
                                        message.value().as_str(),
                                        constants_str::CODE_STYLE_TRANSPARENT_DISPLAY_FORMAT
                                            | constants_str::CODE_STYLE_TRANSPARENT_DEBUG_FORMAT
                                    )
                                })
                        });
                        (!adds_context).then(|| {
                            format!(
                                "{}: error `{}` only wraps repository error `{inner_name}`; use `{inner_name}` directly",
                                source_path.display(),
                                item_struct.ident,
                            )
                        })
                    })
                    .collect::<Vec<_>>()
                    .into_iter()
            })
            .collect::<Vec<_>>();
        assert!(
            violations.is_empty(),
            "4d1a06bf repository errors must not be wrapped without added context:\n{}",
            violations.join("\n")
        );
    });
}

#[test]
fn domain_types_do_not_add_intermediate_representation_wrappers() {
    super::code_style_snapshot::with_codebase_snapshot(|snapshot| {
        let mut representation_only_names = snapshot
            .rs_files()
            .iter()
            .flat_map(|source_file| source_file.ast().as_ref().items.iter())
            .filter_map(|item| {
                let syn::Item::Struct(item_struct) = item else {
                    return None;
                };
                let syn::Fields::Unnamed(fields) = &item_struct.fields else {
                    return None;
                };
                if fields.unnamed.len() != constants_usize::ONE {
                    return None;
                }
                let syn::Type::Path(inner_type) = &fields.unnamed.first()?.ty else {
                    return None;
                };
                let inner_name = inner_type.path.segments.last()?.ident.to_string();
                (inner_type
                    .path
                    .segments
                    .iter()
                    .any(|segment| segment.ident == constants_str::CODE_STYLE_NUM_PATH_SEGMENT)
                    && inner_name.starts_with(constants_str::CODE_STYLE_NON_ZERO_PREFIX))
                .then(|| item_struct.ident.to_string())
            })
            .collect::<std::collections::BTreeSet<_>>();
        snapshot
            .rs_files()
            .iter()
            .flat_map(|source_file| source_file.ast().as_ref().items.iter())
            .filter_map(|item| {
                let syn::Item::Impl(item_impl) = item else {
                    return None;
                };
                let syn::Type::Path(self_type) = item_impl.self_ty.as_ref() else {
                    return None;
                };
                self_type
                    .path
                    .segments
                    .last()
                    .map(|segment| segment.ident.to_string())
            })
            .for_each(|implemented_name| {
                let _removed = representation_only_names.remove(&implemented_name);
            });
        let violations = snapshot
            .rs_files()
            .iter()
            .flat_map(|source_file| {
                let source_path = source_file.path().as_ref();
                source_file
                    .ast()
                    .as_ref()
                    .items
                    .iter()
                    .filter_map(|item| {
                        let syn::Item::Struct(item_struct) = item else {
                            return None;
                        };
                        let syn::Fields::Unnamed(fields) = &item_struct.fields else {
                            return None;
                        };
                        if fields.unnamed.len() != constants_usize::ONE {
                            return None;
                        }
                        let syn::Type::Path(inner_type) = &fields.unnamed.first()?.ty else {
                            return None;
                        };
                        let inner_name = inner_type.path.segments.last()?.ident.to_string();
                        representation_only_names.contains(&inner_name).then(|| {
                                let outer_name = item_struct.ident.to_string();
                                format!(
                                    "{}: `{outer_name}` wraps representation-only `{inner_name}`; store the underlying representation directly in `{outer_name}`",
                                    source_path.display(),
                                )
                            })
                    })
                    .collect::<Vec<_>>()
                    .into_iter()
            })
            .collect::<Vec<_>>();
        assert!(
            violations.is_empty(),
            "df9a3b84 domain wrappers must not contain an intermediate representation wrapper:\n{}",
            violations.join("\n")
        );
    });
}

#[test]
fn module_declarations_do_not_use_path_attributes() {
    super::code_style_snapshot::with_codebase_snapshot(|snapshot| {
        let violations = snapshot
            .rs_files()
            .iter()
            .flat_map(|source_file| {
                let source_path = source_file.path().as_ref();
                source_file
                    .ast()
                    .as_ref()
                    .items
                    .iter()
                    .filter_map(|item| {
                        let syn::Item::Mod(item_mod) = item else {
                            return None;
                        };
                        let explicit_path_text = item_mod.attrs.iter().find_map(|attribute| {
                            if !attribute.path().is_ident(constants_str::PATH_ALT_5) {
                                return None;
                            }
                            let syn::Meta::NameValue(name_value) = &attribute.meta else {
                                return None;
                            };
                            let syn::Expr::Lit(expression_literal) = &name_value.value else {
                                return None;
                            };
                            let syn::Lit::Str(path_literal) = &expression_literal.lit else {
                                return None;
                            };
                            Some(path_literal.value())
                        })?;
                        Some(format!(
                            "{}: `#[path = \"{}\"]` on module `{}` is unnecessary: Rust automatically loads `{}` from `{}.rs`; move `mod {};` to the crate root, rename the source file to `{}.rs` if necessary, reference it through `crate::{}`, and remove the `#[path]` attribute",
                            source_path.display(),
                            explicit_path_text,
                            item_mod.ident,
                            item_mod.ident,
                            item_mod.ident,
                            item_mod.ident,
                            item_mod.ident,
                            item_mod.ident,
                        ))
                    })
                    .collect::<Vec<String>>()
                    .into_iter()
            })
            .collect::<Vec<String>>();
        assert!(violations.is_empty(), "1be5bba1 {violations:#?}");
    });
}

#[test]
fn external_module_declarations_exist_only_in_crate_roots() {
    super::code_style_snapshot::with_codebase_snapshot(|snapshot| {
        let mut violations = snapshot
            .rs_files()
            .iter()
            .filter(|source_file| {
                !matches!(
                    source_file
                        .path()
                        .as_ref()
                        .file_stem()
                        .and_then(std::ffi::OsStr::to_str),
                    Some(constants_str::LIB | constants_str::MAIN)
                )
            })
            .flat_map(|source_file| {
                source_file
                    .ast()
                    .as_ref()
                    .items
                    .iter()
                    .filter_map(|item| {
                        let syn::Item::Mod(item_mod) = item else {
                            return None;
                        };
                        item_mod.content.is_none().then(|| {
                            format!(
                                "{}: external module `{}` must be declared in lib.rs or main.rs",
                                source_file.path().as_ref().display(),
                                item_mod.ident
                            )
                        })
                    })
                    .collect::<Vec<String>>()
            })
            .collect::<Vec<String>>();
        violations.sort_unstable();
        assert!(
            violations.is_empty(),
            "c6014d50 external module declarations must exist only in crate roots:\n{}",
            violations.join("\n")
        );
    });
}

#[test]
#[allow(
    clippy::wildcard_enum_match_arm,
    reason = "the policy ignores current and future non-module syn items"
)]
fn single_item_modules_match_their_item_name() {
    super::code_style_snapshot::with_codebase_snapshot(|snapshot| {
        let external_module_path = |parent_path: &std::path::Path, item_mod: &syn::ItemMod| {
            let optional_explicit_path = item_mod.attrs.iter().find_map(|attribute| {
                if !attribute.path().is_ident(constants_str::PATH_ALT_5) {
                    return None;
                }
                let syn::Meta::NameValue(name_value) = &attribute.meta else {
                    return None;
                };
                let syn::Expr::Lit(expression_literal) = &name_value.value else {
                    return None;
                };
                let syn::Lit::Str(path_literal) = &expression_literal.lit else {
                    return None;
                };
                Some(path_literal.value())
            });
            let parent_directory = parent_path.parent()?;
            if let Some(explicit_path) = optional_explicit_path {
                return Some(parent_directory.join(explicit_path));
            }
            let parent_stem = parent_path.file_stem()?.to_str()?;
            let module_directory = if matches!(
                parent_stem,
                constants_str::LIB | constants_str::MAIN | constants_str::MOD
            ) {
                parent_directory.to_path_buf()
            } else {
                parent_directory.join(parent_stem)
            };
            let module_name = item_mod.ident.to_string();
            let flat_path = module_directory.join(format!("{module_name}.rs"));
            flat_path.is_file().then_some(flat_path).or_else(|| {
                Some(
                    module_directory
                        .join(module_name)
                        .join(constants_str::MOD_RS),
                )
            })
        };
        let owners_by_path = snapshot
            .rs_files()
            .iter()
            .filter(|file| !is_test_source(file.path().as_ref()))
            .filter_map(|file| {
                #[allow(
                    clippy::wildcard_enum_match_arm,
                    reason = "syn item is non-exhaustive and unnamed variants do not participate"
                )]
                let identifiers = file
                    .ast()
                    .as_ref()
                    .items
                    .iter()
                    .filter_map(|syn_item| match syn_item {
                        syn::Item::Const(item_const) => Some(&item_const.ident),
                        syn::Item::Enum(item_enum) => Some(&item_enum.ident),
                        syn::Item::Fn(item_fn) => Some(&item_fn.sig.ident),
                        syn::Item::Static(item_static) => Some(&item_static.ident),
                        syn::Item::Struct(item_struct) => Some(&item_struct.ident),
                        syn::Item::Trait(item_trait) => Some(&item_trait.ident),
                        syn::Item::TraitAlias(item_trait_alias) => Some(&item_trait_alias.ident),
                        syn::Item::Type(item_type) => Some(&item_type.ident),
                        syn::Item::Union(item_union) => Some(&item_union.ident),
                        _ => None,
                    })
                    .collect::<Vec<_>>();
                let [identifier] = identifiers.as_slice() else {
                    return None;
                };
                file.path()
                    .as_ref()
                    .canonicalize()
                    .ok()
                    .map(|path| (path, identifier.to_string()))
            })
            .collect::<std::collections::HashMap<_, _>>();
        let violations = snapshot
            .rs_files()
            .iter()
            .filter(|file| !is_test_source(file.path().as_ref()))
            .flat_map(|file| {
                file.ast()
                    .as_ref()
                    .items
                    .iter()
                    .filter_map(|item| match item {
                        syn::Item::Mod(item_mod) if item_mod.content.is_none() => Some(item_mod),
                        _ => None,
                    })
                    .filter_map(|item_mod| {
                        let module_path = external_module_path(file.path().as_ref(), item_mod)?
                            .canonicalize()
                            .ok()?;
                        let owner = owners_by_path.get(&module_path)?;
                        let expected_module_name = identifier_snake_case(&syn::Ident::new(
                            owner,
                            proc_macro2::Span::call_site(),
                        ));
                        let module_file_stem = module_path.file_stem()?.to_str()?;
                        let module_file_matches = module_file_stem == expected_module_name.as_ref();
                        (item_mod.ident != expected_module_name.as_ref()
                            || !module_file_matches)
                            .then(|| {
                            format!(
                                "{}: module `{}` in file `{}` contains single item `{owner}` and both module and complete file stem must be `{}`",
                                file.path().as_ref().display(),
                                item_mod.ident,
                                module_path.display(),
                                expected_module_name.as_ref()
                            )
                        })
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        assert!(
            violations.is_empty(),
            "single-item modules must match their item name:\n{}",
            violations.join("\n")
        );
    });
}

#[test]
#[allow(
    clippy::wildcard_enum_match_arm,
    reason = "the policy intentionally ignores every current and future non-function syn item"
)]
fn function_only_modules_contain_at_most_one_function() {
    super::code_style_snapshot::with_codebase_snapshot(|snapshot| {
        let target_roots = snapshot
            .workspace_metadata()
            .as_ref()
            .packages
            .iter()
            .flat_map(|package| package.targets.iter())
            .filter_map(|target| target.src_path.as_std_path().canonicalize().ok())
            .collect::<std::collections::HashSet<_>>();
        let violations = snapshot
            .rs_files()
            .iter()
            .filter(|file| !is_test_source(file.path().as_ref()))
            .filter(|file| {
                file.path()
                    .as_ref()
                    .canonicalize()
                    .is_ok_and(|path| !target_roots.contains(&path))
            })
            .filter_map(|file| {
                let functions = file
                    .ast()
                    .as_ref()
                    .items
                    .iter()
                    .filter_map(|item| match item {
                        syn::Item::Fn(item_fn) => Some(item_fn.sig.ident.to_string()),
                        _ => None,
                    })
                    .collect::<Vec<_>>();
                let contains_only_functions_and_module_plumbing =
                    file.ast().as_ref().items.iter().all(|item| {
                        matches!(
                            item,
                            syn::Item::Fn(_) | syn::Item::Mod(_) | syn::Item::Use(_)
                        )
                    });
                (contains_only_functions_and_module_plumbing
                    && functions.len() > constants_usize::ONE)
                    .then(|| {
                        format!(
                            "{}: {}",
                            file.path().as_ref().display(),
                            functions.join(", ")
                        )
                    })
            })
            .collect::<Vec<_>>();
        assert!(
            violations.is_empty(),
            "function-only Rust modules must place each function in its own same-named module:\n{}",
            violations.join("\n")
        );
    });
}

#[test]
fn non_root_workspace_modules_are_not_reexport_only_facades() {
    fn is_reexport_only(items: &[syn::Item]) -> bool {
        !items.is_empty() && items.iter().all(|item| matches!(item, syn::Item::Use(_)))
    }
    fn is_crate_root(path: &std::path::Path) -> bool {
        path.parent()
            .and_then(std::path::Path::file_name)
            .is_some_and(|directory| directory == constants_str::SRC_ALT)
            && path.file_stem().is_some_and(|file_stem| {
                file_stem == constants_str::LIB || file_stem == constants_str::MAIN
            })
    }
    let reexports = syn::parse_file(constants_str::CODE_STYLE_REEXPORT_ONLY_FIXTURE)
        .expect("f4a6c213 re-export-only module fixture must parse");
    let module_with_logic = syn::parse_file(constants_str::CODE_STYLE_REEXPORT_WITH_LOGIC_FIXTURE)
        .expect("875cd8ad module-with-logic fixture must parse");
    assert!(is_reexport_only(reexports.items.as_slice()), "fd841ceb");
    assert!(
        !is_reexport_only(module_with_logic.items.as_slice()),
        "b73e4c8a"
    );
    super::code_style_snapshot::with_codebase_snapshot(|snapshot| {
        let violations = snapshot
            .rs_files()
            .iter()
            .filter(|file| {
                !is_crate_root(file.path().as_ref())
                    && is_reexport_only(file.ast().as_ref().items.as_slice())
            })
            .map(|file| file.path().as_ref().display().to_string())
            .collect::<Vec<_>>();
        assert!(
            violations.is_empty(),
            "dc4298e7 non-root workspace modules must not contain only re-exports; move them to lib.rs or main.rs:\n{}",
            violations.join("\n")
        );
    });
}

#[test]
fn production_modules_contain_at_most_one_named_owner() {
    super::code_style_snapshot::with_codebase_snapshot(|snapshot| {
        let target_roots = snapshot
            .workspace_metadata()
            .as_ref()
            .packages
            .iter()
            .flat_map(|package| package.targets.iter())
            .filter_map(|target| target.src_path.as_std_path().canonicalize().ok())
            .collect::<std::collections::HashSet<_>>();
        let violations = snapshot
            .rs_files()
            .iter()
            .filter(|file| !is_test_source(file.path().as_ref()))
            .filter(|file| {
                file.path()
                    .as_ref()
                    .canonicalize()
                    .is_ok_and(|path| !target_roots.contains(&path))
            })
            .filter_map(|file| {
                let owner = |item: &syn::Item| match item {
                    syn::Item::Enum(item_enum) => {
                        Some((constants_str::ITEM_KIND_ENUM, item_enum.ident.to_string()))
                    }
                    syn::Item::Fn(item_fn) => {
                        Some((constants_str::ITEM_KIND_FN, item_fn.sig.ident.to_string()))
                    }
                    syn::Item::Struct(item_struct) => Some((
                        constants_str::ITEM_KIND_STRUCT,
                        item_struct.ident.to_string(),
                    )),
                    syn::Item::Trait(item_trait) => {
                        Some((constants_str::ITEM_KIND_TRAIT, item_trait.ident.to_string()))
                    }
                    syn::Item::Const(_)
                    | syn::Item::ExternCrate(_)
                    | syn::Item::ForeignMod(_)
                    | syn::Item::Impl(_)
                    | syn::Item::Macro(_)
                    | syn::Item::Mod(_)
                    | syn::Item::Static(_)
                    | syn::Item::TraitAlias(_)
                    | syn::Item::Type(_)
                    | syn::Item::Union(_)
                    | syn::Item::Use(_)
                    | syn::Item::Verbatim(_)
                    | _ => None,
                };
                let owners = file
                    .ast()
                    .as_ref()
                    .items
                    .iter()
                    .filter_map(owner)
                    .collect::<Vec<_>>();
                if owners.len() > constants_usize::ONE {
                    return Some(format!(
                        "{}: {}",
                        file.path().as_ref().display(),
                        owners
                            .iter()
                            .map(|(kind, identifier)| format!("{kind} {identifier}"))
                            .collect::<Vec<_>>()
                            .join(", ")
                    ));
                }
                let (_kind, identifier) = owners.first()?;
                let expected_file_stem = identifier_snake_case(&syn::Ident::new(
                    identifier,
                    proc_macro2::Span::call_site(),
                ));
                let file_stem = file.path().as_ref().file_stem()?.to_str()?;
                (file_stem != expected_file_stem.as_ref()).then(|| {
                    format!(
                        "{}: named owner `{identifier}` requires file stem `{}`",
                        file.path().as_ref().display(),
                        expected_file_stem.as_ref()
                    )
                })
            })
            .collect::<Vec<_>>();
        assert!(
            violations.is_empty(),
            "production Rust modules must place each top-level function, struct, enum, and trait in its own same-named module:\n{}",
            violations.join("\n")
        );
    });
}
fn large_module_exceptions() -> [&'static str; 2] {
    [constants_str::VALUE_7FE2AF02, constants_str::VALUE_D405F3E1]
}

fn is_test_source(path: &std::path::Path) -> bool {
    super::is_test_source_path(super::types::PathRef::from(path)).get()
        || path.file_stem().is_some_and(|file_stem| {
            file_stem
                .to_string_lossy()
                .ends_with(constants_str::TEST_FIXTURES_MODULE_SUFFIX)
        })
        || path
            .components()
            .any(|component| component.as_os_str() == constants_str::VALUE_D0549AF3)
}

#[test]
fn production_modules_have_bounded_responsibility() {
    super::code_style_snapshot::with_codebase_snapshot(|snapshot| {
        let violations = snapshot
            .rs_files()
            .iter()
            .filter(|file| !is_test_source(file.path().as_ref()))
            .filter_map(|file| {
                let line_count = file.content().as_ref().lines().count();
                (line_count > PRODUCTION_MODULE_MAX_LINES
                    && !large_module_exceptions()
                        .iter()
                        .any(|exception| file.path().as_ref().ends_with(exception)))
                .then(|| format!("{}: {line_count} lines", file.path().as_ref().display()))
            })
            .collect::<Vec<_>>();
        assert!(
            violations.is_empty(),
            "production modules above {PRODUCTION_MODULE_MAX_LINES} lines must be split by responsibility:\n{}",
            violations.join("\n")
        );
    });
}

#[test]
fn large_production_modules_keep_tests_in_separate_files() {
    super::code_style_snapshot::with_codebase_snapshot(|snapshot| {
        let violations = snapshot
            .rs_files()
            .iter()
            .filter(|file| !is_test_source(file.path().as_ref()))
            .filter(|file| {
                file.content().as_ref().lines().count() > INLINE_TEST_SEPARATION_MIN_LINES
            })
            .filter(|file| {
                file.ast().as_ref().items.iter().any(|item| {
                    let syn::Item::Mod(module) = item else {
                        return false;
                    };
                    module.ident == constants_str::TESTS_ALT
                        && module.content.is_some()
                        && module.attrs.iter().any(|attribute| {
                            super::attr_is_test_only_cfg(super::types::SynAttributeRef::from(
                                attribute,
                            ))
                            .get()
                        })
                })
            })
            .map(|file| file.path().as_ref().display().to_string())
            .collect::<Vec<_>>();
        assert!(
            violations.is_empty(),
            "large production modules must keep tests in separate files:\n{}",
            violations.join("\n")
        );
    });
}

#[test]
#[allow(
    clippy::needless_for_each,
    reason = "the iterator form follows the workspace no-for-loop policy"
)]
fn large_module_exceptions_are_exact_and_still_needed() {
    super::code_style_snapshot::with_codebase_snapshot(|snapshot| {
        large_module_exceptions().iter().for_each(|exception| {
            let matching_file = snapshot
                .rs_files()
                .iter()
                .find(|file| file.path().as_ref().ends_with(exception));
            let Some(file) = matching_file else {
                panic!("2d8f4a61 missing large-module exception target: {exception}");
            };
            assert!(
                file.content().as_ref().lines().count() > PRODUCTION_MODULE_MAX_LINES,
                "9a71c5e3 stale large-module exception: {exception}"
            );
        });
    });
}

#[test]
#[allow(
    clippy::needless_for_each,
    reason = "the iterator form follows the workspace no-for-loop policy"
)]
fn notification_service_domain_types_exclude_application_and_adapter_workflows() {
    super::code_style_snapshot::with_codebase_snapshot(|snapshot| {
        let source_with_tests = snapshot
            .rs_files()
            .iter()
            .find(|file| {
                file.path()
                    .as_ref()
                    .ends_with(constants_str::VALUE_8E41EC63)
            })
            .expect("2a6298c3 notification service domain types source must exist")
            .content()
            .as_ref();
        let source = source_with_tests
            .split(constants_str::VALUE_3BA26FB4)
            .next()
            .expect("3ae48239 split always returns the production source prefix");
        [
            constants_str::VALUE_18A392BE,
            constants_str::VALUE_99D94433,
            constants_str::VALUE_96E8A555,
            constants_str::VALUE_5015C549,
            constants_str::VALUE_9B877603,
            constants_str::VALUE_8B0F112C,
            constants_str::VALUE_3A4EDC2D,
        ]
        .iter()
        .for_each(|forbidden| {
            assert!(
                !source.contains(forbidden),
                "8e620507 notification service domain_types contains adapter or application workflow `{forbidden}`"
            );
        });
    });
}

#[test]
#[allow(
    clippy::needless_for_each,
    reason = "the iterator form follows the workspace no-for-loop policy"
)]
fn administrator_account_initialization_and_password_reset_domain_types_exclude_application_workflows()
 {
    super::code_style_snapshot::with_codebase_snapshot(|snapshot| {
        let source = snapshot
            .rs_files()
            .iter()
            .find(|file| {
                file.path()
                    .as_ref()
                    .ends_with(constants_str::VALUE_2C978AB0)
            })
            .expect("f49a25d6 initial administrator creation domain types source must exist")
            .content()
            .as_ref();
        [
            constants_str::VALUE_E7118A3C,
            constants_str::VALUE_3349907E,
            constants_str::VALUE_FF3A4973,
            constants_str::VALUE_EB9EA192,
            constants_str::VALUE_9B877603,
        ]
        .iter()
        .for_each(|forbidden| {
            assert!(
                !source.contains(forbidden),
                "2f5b1520 initial administrator creation domain_types contains application workflow `{forbidden}`"
            );
        });
    });
}

#[test]
#[allow(
    clippy::needless_for_each,
    reason = "the iterator form follows the workspace no-for-loop policy"
)]
fn common_route_domain_types_exclude_http_and_database_workflows() {
    super::code_style_snapshot::with_codebase_snapshot(|snapshot| {
        let source_with_tests = snapshot
            .rs_files()
            .iter()
            .find(|file| {
                file.path()
                    .as_ref()
                    .ends_with(constants_str::VALUE_30296F9B)
            })
            .expect("1ef73397 common route domain types source must exist")
            .content()
            .as_ref();
        let source = source_with_tests
            .split(constants_str::VALUE_3BA26FB4)
            .next()
            .expect("90fc214f split always returns the production source prefix");
        [
            constants_str::VALUE_99D94433,
            constants_str::VALUE_2E84067B,
            constants_str::VALUE_C5EAB055,
            constants_str::VALUE_0E48D7B1,
            constants_str::VALUE_1812E35F,
        ]
        .iter()
        .for_each(|forbidden| {
            assert!(
                !source.contains(forbidden),
                "6e3c94a8 common route domain_types contains HTTP or database workflow `{forbidden}`"
            );
        });
    });
}

#[test]
#[allow(
    clippy::needless_for_each,
    reason = "the iterator form follows the workspace no-for-loop policy"
)]
fn server_domain_types_exclude_application_and_adapter_workflows() {
    super::code_style_snapshot::with_codebase_snapshot(|snapshot| {
        let source_with_tests = snapshot
            .rs_files()
            .iter()
            .find(|file| {
                file.path()
                    .as_ref()
                    .ends_with(constants_str::VALUE_47325207)
            })
            .expect("2a49fec1 server domain types source must exist")
            .content()
            .as_ref();
        let source = source_with_tests
            .split(constants_str::VALUE_3BA26FB4)
            .next()
            .expect("82d0ffa2 split always returns the production source prefix");
        [
            constants_str::VALUE_58D8E00E,
            constants_str::VALUE_4BB60066,
            constants_str::VALUE_5BE799DC,
            constants_str::VALUE_062AEA27,
            constants_str::VALUE_69B22E2A,
            constants_str::VALUE_9B877603,
        ]
        .iter()
        .for_each(|forbidden| {
            assert!(
                !source.contains(forbidden),
                "f8d1eb2d server domain_types contains application or adapter workflow `{forbidden}`"
            );
        });
    });
}

#[test]
fn server_admin_domain_types_exclude_repository_workflows() {
    super::code_style_snapshot::with_codebase_snapshot(|snapshot| {
        let violations = snapshot
            .rs_files()
            .iter()
            .filter(|file| {
                let path = file.path().as_ref().to_string_lossy();
                path.contains(constants_str::VALUE_5A1A4545)
                    || path.contains(constants_str::VALUE_43EF539D)
                    || path.ends_with(constants_str::VALUE_9CAC1060)
                    || path.ends_with(constants_str::VALUE_AAA5BED8)
            })
            .map(|file| file.path().as_ref().display().to_string())
            .collect::<Vec<_>>();
        assert!(
            violations.is_empty(),
            "5aee4dc0 server admin domain_types contains application or persistence workflows: {violations:?}"
        );
    });
}

#[test]
#[allow(
    clippy::needless_for_each,
    reason = "the iterator form follows the workspace no-for-loop policy"
)]
fn workspace_scaffold_domain_types_exclude_entrypoint_and_template_filesystem_workflows() {
    super::code_style_snapshot::with_codebase_snapshot(|snapshot| {
        let source_with_tests = snapshot
            .rs_files()
            .iter()
            .find(|file| {
                file.path()
                    .as_ref()
                    .ends_with(constants_str::VALUE_1A456B0D)
            })
            .expect("3119b009 workspace scaffold domain types source must exist")
            .content()
            .as_ref();
        let source = source_with_tests
            .split(constants_str::VALUE_3BA26FB4)
            .next()
            .expect("1e5e6186 split always returns the production source prefix");
        [
            constants_str::VALUE_59CAD555,
            constants_str::VALUE_E72B634A,
            constants_str::VALUE_731FDA74,
            constants_str::VALUE_522C24E5,
            constants_str::VALUE_C36F32EE,
            constants_str::VALUE_BCDC0F38,
        ]
        .iter()
        .for_each(|forbidden| {
            assert!(
                !source.contains(forbidden),
                "762cf3b3 workspace scaffold domain_types contains entrypoint or template filesystem workflow `{forbidden}`"
            );
        });
    });
}

#[test]
#[allow(
    clippy::needless_for_each,
    reason = "the iterator form follows the workspace no-for-loop policy"
)]
fn file_storage_domain_types_exclude_filesystem_workflows() {
    super::code_style_snapshot::with_codebase_snapshot(|snapshot| {
        let source_with_tests = snapshot
            .rs_files()
            .iter()
            .find(|file| {
                file.path()
                    .as_ref()
                    .ends_with(constants_str::VALUE_712F68AD)
            })
            .expect("a081579c file storage domain types source must exist")
            .content()
            .as_ref();
        let source = source_with_tests
            .split(constants_str::VALUE_3BA26FB4)
            .next()
            .expect("622c12de split always returns the production source prefix");
        [
            constants_str::TOKIO_PATH_FS_PATH,
            constants_str::VALUE_BAA7CB12,
            constants_str::VALUE_303C9B02,
            constants_str::VALUE_D30B72A0,
            constants_str::VALUE_B863F79E,
        ]
        .iter()
        .for_each(|forbidden| {
            assert!(
                !source.contains(forbidden),
                "894ee8e8 file storage domain_types contains filesystem workflow `{forbidden}`"
            );
        });
    });
}

#[test]
#[allow(
    clippy::needless_for_each,
    reason = "the iterator form follows the workspace no-for-loop policy"
)]
fn workspace_test_runner_domain_types_exclude_application_and_adapter_workflows() {
    super::code_style_snapshot::with_codebase_snapshot(|snapshot| {
        let source_with_tests = snapshot
            .rs_files()
            .iter()
            .find(|file| {
                file.path()
                    .as_ref()
                    .ends_with(constants_str::VALUE_F45EC0EE)
            })
            .expect("8b3cb235 workspace test runner domain types source must exist")
            .content()
            .as_ref();
        let source = source_with_tests
            .split(constants_str::VALUE_3BA26FB4)
            .next()
            .expect("4c2a6281 split always returns the production source prefix");
        [
            constants_str::VALUE_32E64619,
            constants_str::VALUE_B9D99DED,
            constants_str::VALUE_7C10C158,
            constants_str::VALUE_E63A5758,
            constants_str::VALUE_9B877603,
            constants_str::VALUE_CC4BBDCE,
        ]
        .iter()
        .for_each(|forbidden| {
            assert!(
                !source.contains(forbidden),
                "f071d2cf workspace test runner domain_types contains application or adapter workflow `{forbidden}`"
            );
        });
    });
}
