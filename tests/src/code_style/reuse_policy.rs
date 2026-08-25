#![allow(
    clippy::arbitrary_source_item_ordering,
    reason = "the runtime reviewed-duplicate inventory must precede its analyzer types without becoming a forbidden string constant"
)]

#[allow(
    clippy::single_call_fn,
    reason = "the named runtime constructor isolates the exact reviewed inventory while avoiding string constants outside constants_str"
)]
fn reviewed_duplicate_groups() -> Vec<ReviewedDuplicateGroup> {
    vec![
        ReviewedDuplicateGroup {
            locations: constants_str::STRING_CONSTANT_METADATA_FIXTURE_LOCATIONS,
            reason: constants_str::STRING_CONSTANT_MIGRATION_NORMALIZES_DISTINCT_FIXTURES,
        },
        ReviewedDuplicateGroup {
            locations: constants_str::STRING_CONSTANT_SOURCE_VISITOR_LOCATIONS,
            reason: constants_str::STRING_CONSTANT_MIGRATION_NORMALIZES_DISTINCT_FIXTURES,
        },
        ReviewedDuplicateGroup {
            locations: constants_str::STRING_CONSTANT_ROUTE_METADATA_FIXTURE_LOCATIONS,
            reason: constants_str::STRING_CONSTANT_MIGRATION_NORMALIZES_DISTINCT_FIXTURES,
        },
        ReviewedDuplicateGroup {
            locations: constants_str::STRING_CONSTANT_ANALYZER_VISITOR_LOCATIONS,
            reason: constants_str::STRING_CONSTANT_MIGRATION_NORMALIZES_DISTINCT_FIXTURES,
        },
        ReviewedDuplicateGroup {
            locations: constants_str::VALUE_0D652FF1,
            reason: constants_str::VALUE_99A8FB72,
        },
        ReviewedDuplicateGroup {
            locations: constants_str::VALUE_082A5401,
            reason: constants_str::VALUE_61609B06,
        },
        ReviewedDuplicateGroup {
            locations: constants_str::VALUE_3AE4AA02,
            reason: constants_str::VALUE_9DA4CB90,
        },
        ReviewedDuplicateGroup {
            locations: constants_str::VALUE_CD2A0018,
            reason: constants_str::VALUE_A7AE2844,
        },
        ReviewedDuplicateGroup {
            locations: constants_str::VALUE_A4FF3FB6,
            reason: constants_str::VALUE_39649F62,
        },
        ReviewedDuplicateGroup {
            locations: constants_str::VALUE_4FDDA503,
            reason: constants_str::VALUE_BBB02CF4,
        },
        ReviewedDuplicateGroup {
            locations: constants_str::VALUE_8443FF5D,
            reason: constants_str::VALUE_DC8C52AC,
        },
        ReviewedDuplicateGroup {
            locations: constants_str::VALUE_4793A5FE,
            reason: constants_str::VALUE_95569DAB,
        },
        ReviewedDuplicateGroup {
            locations: constants_str::VALUE_D63A5858,
            reason: constants_str::VALUE_EA3A9D65,
        },
        ReviewedDuplicateGroup {
            locations: constants_str::VALUE_422EC2EB,
            reason: constants_str::VALUE_69F67A0D,
        },
        ReviewedDuplicateGroup {
            locations: constants_str::VALUE_148FAD59,
            reason: constants_str::VALUE_0EA9A6EE,
        },
        ReviewedDuplicateGroup {
            locations: constants_str::VALUE_F9E232EF,
            reason: constants_str::VALUE_D0783800,
        },
        ReviewedDuplicateGroup {
            locations: constants_str::VALUE_7E4078D9,
            reason: constants_str::VALUE_D526A9A1,
        },
        ReviewedDuplicateGroup {
            locations: constants_str::VALUE_5BB2B57A,
            reason: constants_str::VALUE_B334A087,
        },
        ReviewedDuplicateGroup {
            locations: constants_str::VALUE_287FCBEB,
            reason: constants_str::VALUE_A6A100E2,
        },
        ReviewedDuplicateGroup {
            locations: constants_str::VALUE_DCB5D4F2,
            reason: constants_str::VALUE_349BC694,
        },
        ReviewedDuplicateGroup {
            locations: constants_str::VALUE_F487DB2D,
            reason: constants_str::VALUE_9661CEC1,
        },
        ReviewedDuplicateGroup {
            locations: constants_str::VALUE_88A7A661,
            reason: constants_str::VALUE_589704B1,
        },
        ReviewedDuplicateGroup {
            locations: constants_str::VALUE_7005B03A,
            reason: constants_str::VALUE_114A067A,
        },
        ReviewedDuplicateGroup {
            locations: constants_str::VALUE_05051852,
            reason: constants_str::VALUE_C5C34D0B,
        },
        ReviewedDuplicateGroup {
            locations: constants_str::VALUE_58C1A75F,
            reason: constants_str::VALUE_1F5A2577,
        },
        ReviewedDuplicateGroup {
            locations: constants_str::VALUE_D7049B21,
            reason: constants_str::VALUE_D7436E0E,
        },
        ReviewedDuplicateGroup {
            locations: constants_str::VALUE_522C0343,
            reason: constants_str::VALUE_586A9953,
        },
        ReviewedDuplicateGroup {
            locations: constants_str::VALUE_292E1A7F,
            reason: constants_str::VALUE_F311E43F,
        },
        ReviewedDuplicateGroup {
            locations: constants_str::VALUE_A4489C21,
            reason: constants_str::VALUE_ECC17834,
        },
        ReviewedDuplicateGroup {
            locations: constants_str::VALUE_02000EC4,
            reason: constants_str::VALUE_761A94E7,
        },
        ReviewedDuplicateGroup {
            locations: constants_str::VALUE_43BDEFF3,
            reason: constants_str::VALUE_FE253AFB,
        },
        ReviewedDuplicateGroup {
            locations: constants_str::VALUE_A744A72D,
            reason: constants_str::VALUE_064EC769,
        },
        ReviewedDuplicateGroup {
            locations: constants_str::VALUE_1C550714,
            reason: constants_str::VALUE_C1DC2D40,
        },
        ReviewedDuplicateGroup {
            locations: constants_str::VALUE_757BD453,
            reason: constants_str::VALUE_FD1E21A1,
        },
        ReviewedDuplicateGroup {
            locations: constants_str::VALUE_F0DC6ADA,
            reason: constants_str::VALUE_A6259CF3,
        },
        ReviewedDuplicateGroup {
            locations: constants_str::VALUE_689F2872,
            reason: constants_str::VALUE_BC7CFE3A,
        },
        ReviewedDuplicateGroup {
            locations: constants_str::VALUE_DBB9C433,
            reason: constants_str::VALUE_9C6E0958,
        },
        ReviewedDuplicateGroup {
            locations: constants_str::VALUE_E26644F4,
            reason: constants_str::VALUE_C9F14A66,
        },
        ReviewedDuplicateGroup {
            locations: constants_str::VALUE_4CB1E1F3,
            reason: constants_str::VALUE_9ADBC564,
        },
        ReviewedDuplicateGroup {
            locations: constants_str::VALUE_5392D537,
            reason: constants_str::VALUE_8AFCED0D,
        },
        ReviewedDuplicateGroup {
            locations: constants_str::VALUE_66B5730A,
            reason: constants_str::VALUE_D6EC9B66,
        },
        ReviewedDuplicateGroup {
            locations: constants_str::VALUE_E4A2A88A,
            reason: constants_str::VALUE_37FDC7B8,
        },
        ReviewedDuplicateGroup {
            locations: constants_str::VALUE_F43CC42D,
            reason: constants_str::VALUE_0E483FB8,
        },
        ReviewedDuplicateGroup {
            locations: constants_str::VALUE_27922A80,
            reason: constants_str::VALUE_BC659900,
        },
        ReviewedDuplicateGroup {
            locations: constants_str::VALUE_599796F1,
            reason: constants_str::VALUE_8A3C621C,
        },
        ReviewedDuplicateGroup {
            locations: constants_str::VALUE_F2B019BA,
            reason: constants_str::VALUE_BB0F504B,
        },
        ReviewedDuplicateGroup {
            locations: constants_str::VALUE_CD85A891,
            reason: constants_str::VALUE_424D0EAB,
        },
        ReviewedDuplicateGroup {
            locations: constants_str::VALUE_72A10749,
            reason: constants_str::VALUE_F6AEFC16,
        },
        ReviewedDuplicateGroup {
            locations: constants_str::VALUE_CBBA0BFF,
            reason: constants_str::VALUE_6E80E87B,
        },
        ReviewedDuplicateGroup {
            locations: constants_str::VALUE_0D4F3549,
            reason: constants_str::VALUE_BECDB8D8,
        },
        ReviewedDuplicateGroup {
            locations: constants_str::VALUE_FBAC771A,
            reason: constants_str::VALUE_DA10DE3B,
        },
        ReviewedDuplicateGroup {
            locations: constants_str::VALUE_B90EA89F,
            reason: constants_str::VALUE_3A40A71C,
        },
        ReviewedDuplicateGroup {
            locations: constants_str::VALUE_5BE6CC71,
            reason: constants_str::VALUE_BB0F504B,
        },
        ReviewedDuplicateGroup {
            locations: constants_str::VALUE_07C16E6D,
            reason: constants_str::VALUE_5A4F5CD4,
        },
        ReviewedDuplicateGroup {
            locations: constants_str::VALUE_224F7450,
            reason: constants_str::VALUE_BD024C4B,
        },
        ReviewedDuplicateGroup {
            locations: constants_str::VALUE_C78A26DB,
            reason: constants_str::VALUE_F2E12A9E,
        },
        ReviewedDuplicateGroup {
            locations: constants_str::VALUE_11C1DCC5,
            reason: constants_str::VALUE_D0150024,
        },
        ReviewedDuplicateGroup {
            locations: constants_str::VALUE_944342EF,
            reason: constants_str::VALUE_5805C05B,
        },
        ReviewedDuplicateGroup {
            locations: constants_str::VALUE_2D700ED6,
            reason: constants_str::VALUE_647D5C11,
        },
        ReviewedDuplicateGroup {
            locations: constants_str::VALUE_AE96131E,
            reason: constants_str::VALUE_879AE029,
        },
        ReviewedDuplicateGroup {
            locations: constants_str::VALUE_7A32C552,
            reason: constants_str::VALUE_23A957C9,
        },
        ReviewedDuplicateGroup {
            locations: constants_str::VALUE_2D81C306,
            reason: constants_str::VALUE_79E09A12,
        },
        ReviewedDuplicateGroup {
            locations: constants_str::VALUE_51DBE253,
            reason: constants_str::VALUE_91B4F7EC,
        },
    ]
}

#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, Default)]
struct FunctionBodyComplexity {
    expression_count: usize,
}

#[derive(optimal_memory_layout::OptimalMemoryLayout)]
struct FunctionBodyVisitor<'visitor_lt> {
    bodies: super::types::FunctionBodyLocationsBTreeMapMutRef<'visitor_lt>,
    identifier_pattern: super::types::RegexRegexRef<'visitor_lt>,
    path: super::types::PathRef<'visitor_lt>,
}

#[derive(optimal_memory_layout::OptimalMemoryLayout)]
struct ReviewedDuplicateGroup {
    locations: &'static str,
    reason: &'static str,
}

impl<'ast> syn::visit::Visit<'ast> for FunctionBodyComplexity {
    fn visit_expr(&mut self, i: &'ast syn::Expr) {
        self.expression_count = self.expression_count.saturating_add(constants_usize::ONE);
        syn::visit::visit_expr(self, i);
    }
}

impl FunctionBodyVisitor<'_> {
    fn record(&mut self, name: &syn::Ident, block: &syn::Block) {
        if function_body_is_substantial(block) {
            self.bodies
                .entry(function_body_hash(block, self.identifier_pattern))
                .or_default()
                .push(format!("{}::{name}", self.path.as_ref().display()));
        }
    }
}

impl<'ast> syn::visit::Visit<'ast> for FunctionBodyVisitor<'_> {
    fn visit_impl_item_fn(&mut self, i: &'ast syn::ImplItemFn) {
        self.record(&i.sig.ident, &i.block);
        syn::visit::visit_impl_item_fn(self, i);
    }

    fn visit_item_fn(&mut self, i: &'ast syn::ItemFn) {
        if !super::item_fn_is_unit_test(super::types::SynItemFnRef::from(i)).get() {
            self.record(&i.sig.ident, &i.block);
        }
        syn::visit::visit_item_fn(self, i);
    }
}

fn function_body_is_substantial(block: &syn::Block) -> bool {
    let mut complexity = FunctionBodyComplexity::default();
    syn::visit::Visit::visit_block(&mut complexity, block);
    complexity.expression_count >= 9usize
}

fn function_body_hash(
    block: &syn::Block,
    identifier_pattern: super::types::RegexRegexRef<'_>,
) -> super::types::FunctionBodyHash {
    let body = format!("{block:?}");
    let normalized_body =
        identifier_pattern.replace_all(&body, constants_str::NORMALIZED_IDENTIFIER);
    let mut hasher = std::hash::DefaultHasher::new();
    std::hash::Hash::hash(&normalized_body, &mut hasher);
    super::types::FunctionBodyHash::from(std::hash::Hasher::finish(&hasher))
}

#[test]
fn substantial_function_bodies_have_one_source_of_truth() {
    let mut bodies = super::types::FunctionBodyLocationsBTreeMap::default();
    let identifier_pattern = regex::Regex::new(constants_str::VALUE_58523C42).expect(
        "d4a8c2f1 substantial_function_bodies_have_one_source_of_truth invariant must hold",
    );
    super::snapshot::with_codebase_snapshot(|snapshot| {
        snapshot.rs_files().iter().for_each(|file| {
            let mut visitor = FunctionBodyVisitor {
                bodies: super::types::FunctionBodyLocationsBTreeMapMutRef::from(&mut bodies),
                identifier_pattern: super::types::RegexRegexRef::from(&identifier_pattern),
                path: super::types::PathRef::from(file.path().as_ref()),
            };
            syn::visit::Visit::visit_file(&mut visitor, file.ast().as_ref());
        });
    });
    let mut reviewed = reviewed_duplicate_groups().into_iter().fold(
        std::collections::BTreeMap::<&str, &str>::new(),
        |mut reviewed_groups, group| {
            assert!(
                !group.reason.trim().is_empty(),
                "reviewed duplicate group must explain why extraction is inappropriate: {}",
                group.locations
            );
            assert!(
                reviewed_groups
                    .insert(group.locations, group.reason)
                    .is_none(),
                "reviewed duplicate group is declared more than once: {}",
                group.locations
            );
            reviewed_groups
        },
    );
    let duplicates = std::collections::BTreeMap::<
        super::types::FunctionBodyHash,
        super::types::SourceTextList,
    >::from(bodies)
    .into_values()
    .filter(|locations| locations.len() > constants_usize::ONE)
    .filter_map(|mut locations| {
        locations.sort_unstable();
        let location_signature = locations.join(constants_str::NEWLINE);
        reviewed
            .remove(location_signature.as_str())
            .is_none()
            .then_some(location_signature)
    })
    .collect::<Vec<String>>();
    assert!(
        duplicates.is_empty(),
        "substantial duplicate function bodies found; extract one source of truth:\n{}",
        duplicates.join("\n\n")
    );
    assert!(
        reviewed.is_empty(),
        "reviewed duplicate groups no longer match the source; remove or update them:\n{}",
        reviewed.keys().copied().collect::<Vec<_>>().join("\n\n")
    );
}

#[test]
fn function_body_similarity_ignores_identifier_names() {
    let first = syn::parse_str::<syn::ItemFn>(constants_str::VALUE_55C24F35)
        .expect("ca632fad first invariant must hold");
    let second = syn::parse_str::<syn::ItemFn>(constants_str::VALUE_A4EA5826)
        .expect("b608f7e1 second invariant must hold");
    let identifier_pattern = regex::Regex::new(constants_str::VALUE_58523C42)
        .expect("9658f225 second invariant must hold");
    let identifier_pattern_ref = super::types::RegexRegexRef::from(&identifier_pattern);
    assert_eq!(
        function_body_hash(&first.block, identifier_pattern_ref),
        function_body_hash(&second.block, identifier_pattern_ref)
    );
}

#[test]
fn function_body_similarity_preserves_behavioral_structure() {
    let addition = syn::parse_str::<syn::ItemFn>(constants_str::VALUE_F3BCDB38)
        .expect("cb1d077f value invariant must hold");
    let subtraction = syn::parse_str::<syn::ItemFn>(constants_str::VALUE_B28E8E9F)
        .expect("ae9313cb value invariant must hold");
    let identifier_pattern = regex::Regex::new(constants_str::VALUE_58523C42)
        .expect("fdf7075b value invariant must hold");
    let identifier_pattern_ref = super::types::RegexRegexRef::from(&identifier_pattern);
    assert_ne!(
        function_body_hash(&addition.block, identifier_pattern_ref),
        function_body_hash(&subtraction.block, identifier_pattern_ref)
    );
}

#[test]
fn short_mechanical_adapters_are_not_substantial() {
    let adapter = syn::parse_str::<syn::ItemFn>(constants_str::VALUE_EC742D93)
        .expect("9dc062d1 value invariant must hold");
    assert!(!function_body_is_substantial(&adapter.block));
}
