struct FunctionBodyVisitor<'visitor_lt> {
    bodies: super::types::StdFunctionBodyLocationsMapMutRef<'visitor_lt>,
    identifier_pattern: super::types::RegexRegexRef<'visitor_lt>,
    path: super::types::StdPathRef<'visitor_lt>,
}

#[derive(Debug, Default)]
struct FunctionBodyComplexity {
    expression_count: usize,
}

impl<'ast> syn::visit::Visit<'ast> for FunctionBodyComplexity {
    fn visit_expr(&mut self, i: &'ast syn::Expr) {
        self.expression_count = self.expression_count.saturating_add(1usize);
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
    complexity.expression_count >= 50usize
}

fn function_body_hash(
    block: &syn::Block,
    identifier_pattern: super::types::RegexRegexRef<'_>,
) -> super::types::FunctionBodyHash {
    let body = format!("{block:?}");
    let normalized_body =
        identifier_pattern.replace_all(&body, str_constants::NORMALIZED_IDENTIFIER);
    let mut hasher = std::hash::DefaultHasher::new();
    std::hash::Hash::hash(&normalized_body, &mut hasher);
    super::types::FunctionBodyHash::from(std::hash::Hasher::finish(&hasher))
}

#[test]
fn substantial_function_bodies_have_one_source_of_truth() {
    let mut bodies = super::types::StdFunctionBodyLocationsMap::default();
    let identifier_pattern =
        regex::Regex::new(r"Ident \{ sym: [^,]+, span: [^}]+ \}").expect("d4a8c2f1");
    super::snapshot::with_codebase_snapshot(|snapshot| {
        snapshot.rs_files().iter().for_each(|file| {
            let mut visitor = FunctionBodyVisitor {
                bodies: super::types::StdFunctionBodyLocationsMapMutRef::from(&mut bodies),
                identifier_pattern: super::types::RegexRegexRef::from(&identifier_pattern),
                path: super::types::StdPathRef::from(file.path().as_ref()),
            };
            syn::visit::Visit::visit_file(&mut visitor, file.ast().as_ref());
        });
    });
    let duplicates = std::collections::BTreeMap::<
        super::types::FunctionBodyHash,
        super::types::SourceTextList,
    >::from(bodies)
    .into_values()
    .filter(|locations| locations.len() > 1usize)
    .map(|locations| locations.join("\n"))
    .collect::<Vec<String>>();
    assert!(
        duplicates.is_empty(),
        "substantial duplicate function bodies found; extract one source of truth:\n{}",
        duplicates.join("\n\n")
    );
}

#[test]
fn function_body_similarity_ignores_identifier_names() {
    let first = syn::parse_str::<syn::ItemFn>("fn first(input: u32) { let value = input + 1; }")
        .expect("ca632fad");
    let second =
        syn::parse_str::<syn::ItemFn>("fn second(source: u32) { let result = source + 1; }")
            .expect("b608f7e1");
    let identifier_pattern =
        regex::Regex::new(r"Ident \{ sym: [^,]+, span: [^}]+ \}").expect("9658f225");
    let identifier_pattern_ref = super::types::RegexRegexRef::from(&identifier_pattern);
    assert_eq!(
        function_body_hash(&first.block, identifier_pattern_ref),
        function_body_hash(&second.block, identifier_pattern_ref)
    );
}

#[test]
fn function_body_similarity_preserves_behavioral_structure() {
    let addition = syn::parse_str::<syn::ItemFn>("fn value(input: u32) { let value = input + 1; }")
        .expect("cb1d077f");
    let subtraction =
        syn::parse_str::<syn::ItemFn>("fn value(input: u32) { let value = input - 1; }")
            .expect("ae9313cb");
    let identifier_pattern =
        regex::Regex::new(r"Ident \{ sym: [^,]+, span: [^}]+ \}").expect("fdf7075b");
    let identifier_pattern_ref = super::types::RegexRegexRef::from(&identifier_pattern);
    assert_ne!(
        function_body_hash(&addition.block, identifier_pattern_ref),
        function_body_hash(&subtraction.block, identifier_pattern_ref)
    );
}

#[test]
fn short_mechanical_adapters_are_not_substantial() {
    let adapter = syn::parse_str::<syn::ItemFn>(
        "fn value(input: Option<u32>) -> u32 { input.map(|value| value + 1).unwrap_or_default() }",
    )
    .expect("9dc062d1");
    assert!(!function_body_is_substantial(&adapter.block));
}
