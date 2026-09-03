const SOURCE_TEXT_MAX_LEN: usize = 16 * 1024 * 1024;
#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    Default,
    proc_macro_newtype::FromInner,
    proc_macro_newtype::GetInner,
)]
pub(super) struct AnalyzerCount(usize);
impl AnalyzerCount {
    pub(super) fn saturating_dec(&mut self) {
        self.0 = self.0.saturating_sub(1);
    }
    pub(super) fn saturating_inc(&mut self) {
        self.0 = self.0.saturating_add(1);
    }
}
#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    Default,
    proc_macro_newtype::FromInner,
    proc_macro_newtype::GetInner,
)]
pub(super) struct AnalyzerBool(bool);
impl AnalyzerBool {
    pub(super) fn set_true(&mut self) {
        self.0 = true;
    }
}
#[derive(
    proc_macro_getters::Getters,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    proc_macro_newtype::FromInner,
    proc_macro_newtype::GetInner,
)]
pub(super) struct CargoTomlFileIndex(usize);
#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    proc_macro_newtype::AsRefOwned,
    proc_macro_newtype::FromInner,
)]
pub(super) struct CargoMetadata(cargo_metadata::Metadata);
#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    proc_macro_newtype::AsRefInner,
    proc_macro_newtype::FromInner,
    proc_macro_newtype::GetInner,
)]
pub(super) struct CargoMetadataRef<'metadata_lt>(&'metadata_lt cargo_metadata::Metadata);
#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    proc_macro_newtype::AsRefOwned,
    proc_macro_newtype::FromInner,
)]
pub(super) struct CargoPackageIdRefHashSet<'metadata_lt>(
    std::collections::HashSet<&'metadata_lt cargo_metadata::PackageId>,
);
#[derive(
    proc_macro_getters::Getters,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    proc_macro_newtype::FromInner,
    proc_macro_newtype::GetInner,
)]
pub(super) struct StaticStr(&'static str);
#[derive(
    proc_macro_getters::Getters,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    proc_macro_newtype::FromInner,
    proc_macro_newtype::GetInner,
)]
pub(super) struct StaticStrSliceRef<'text_lt>(&'text_lt [&'text_lt str]);
#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    proc_macro_newtype::AsRefInner,
    proc_macro_newtype::FromInner,
    proc_macro_newtype::GetInner,
)]
pub(super) struct SourceTextRef<'text_lt>(&'text_lt str);
#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    proc_macro_newtype::AsRefInner,
    proc_macro_newtype::FromInner,
)]
pub(super) struct SourceTextRefHashSet<'text_lt>(
    &'text_lt std::collections::HashSet<&'text_lt str>,
);
#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    proc_macro_newtype::AsRefOwned,
    proc_macro_newtype::FromInner,
)]
pub(super) struct SourceTextHashSet<'text_lt>(std::collections::HashSet<&'text_lt str>);
#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    proc_macro_newtype::AsRefInner,
    proc_macro_newtype::FromInner,
)]
pub(super) struct SynBlockRef<'block_lt>(&'block_lt syn::Block);
#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Default,
    proc_macro_newtype::DerefInner,
    proc_macro_newtype::DerefMutInner,
    proc_macro_newtype::FromInner,
    proc_macro_newtype::IntoIterator,
)]
pub(super) struct DiagnosticMessages(Vec<String>);
#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    proc_macro_newtype::DerefMutTarget,
    proc_macro_newtype::DerefTarget,
    proc_macro_newtype::FromInner,
)]
pub(super) struct DiagnosticMessagesMutRef<'msgs_lt>(&'msgs_lt mut Vec<String>);
impl<'msgs_lt> From<&'msgs_lt mut SourceTextList> for DiagnosticMessagesMutRef<'msgs_lt> {
    fn from(source_text_list: &'msgs_lt mut SourceTextList) -> Self {
        Self(&mut source_text_list.0)
    }
}
#[derive(
    proc_macro_getters::Getters,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    proc_macro_newtype::AsRefStr,
)]
pub(super) struct SourceText(Box<str>);
#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout, Debug, Clone, Copy, thiserror::Error,
)]
pub(super) enum SourceTextTryFromStringError {
    #[error(
        "source text length {} exceeds max {}",
        .len.get(),
        SOURCE_TEXT_MAX_LEN
    )]
    TooLong { len: AnalyzerCount },
}
impl TryFrom<String> for SourceText {
    type Error = SourceTextTryFromStringError;
    fn try_from(string: String) -> Result<Self, Self::Error> {
        if string.len() > SOURCE_TEXT_MAX_LEN {
            return Err(SourceTextTryFromStringError::TooLong {
                len: AnalyzerCount::from(string.len()),
            });
        }
        Ok(Self(string.into_boxed_str()))
    }
}
impl From<SourceText> for String {
    fn from(source_text: SourceText) -> Self {
        source_text.0.into_string()
    }
}
#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Default,
    proc_macro_newtype::DerefInner,
    proc_macro_newtype::DerefMutInner,
    proc_macro_newtype::FromInner,
    proc_macro_newtype::IntoIterator,
)]
pub(super) struct SourceTextList(Vec<String>);
#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    proc_macro_newtype::AsRefInner,
    proc_macro_newtype::FromInner,
    proc_macro_newtype::GetInner,
)]
pub(super) struct SourceTextListRef<'text_lt>(&'text_lt [String]);
#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Default,
    proc_macro_newtype::AsRefOwned,
    proc_macro_newtype::DerefInner,
    proc_macro_newtype::DerefMutInner,
    proc_macro_newtype::FromInner,
    proc_macro_newtype::IntoIterator,
)]
pub(super) struct SourceTextBTreeSet(std::collections::BTreeSet<String>);
#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    proc_macro_newtype::FromInner,
)]
pub(super) struct FunctionBodyHash(u64);
#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    proc_macro_newtype::AsRefInner,
    proc_macro_newtype::DerefTarget,
    proc_macro_newtype::FromInner,
)]
pub(super) struct RegexRegexRef<'regex_lt>(&'regex_lt regex::Regex);
#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Default,
    proc_macro_newtype::DerefInner,
    proc_macro_newtype::FromInner,
    proc_macro_newtype::IntoInnerFrom,
)]
pub(super) struct FunctionBodyLocationsBTreeMap(
    std::collections::BTreeMap<FunctionBodyHash, SourceTextList>,
);
#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    proc_macro_newtype::DerefMutTarget,
    proc_macro_newtype::DerefTarget,
    proc_macro_newtype::FromInner,
)]
pub(super) struct FunctionBodyLocationsBTreeMapMutRef<'map_lt>(
    &'map_lt mut std::collections::BTreeMap<FunctionBodyHash, SourceTextList>,
);
impl<'map_lt> From<&'map_lt mut FunctionBodyLocationsBTreeMap>
    for FunctionBodyLocationsBTreeMapMutRef<'map_lt>
{
    fn from(
        function_body_locations_b_tree_map: &'map_lt mut FunctionBodyLocationsBTreeMap,
    ) -> Self {
        Self(&mut function_body_locations_b_tree_map.0)
    }
}
#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    proc_macro_newtype::AsRefInner,
    proc_macro_newtype::FromInner,
)]
pub(super) struct SourceTextBTreeSetRef<'text_lt>(&'text_lt std::collections::BTreeSet<String>);
#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    proc_macro_newtype::AsRefTarget,
    proc_macro_newtype::BorrowPath,
    proc_macro_newtype::DerefTarget,
    proc_macro_newtype::FromInner,
)]
pub(super) struct OwnedPathBuf(std::path::PathBuf);
#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    proc_macro_newtype::AsRefInner,
    proc_macro_newtype::FromInner,
)]
pub(super) struct PathRef<'path_lt>(&'path_lt std::path::Path);
#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    proc_macro_newtype::AsRefOwned,
    proc_macro_newtype::FromInner,
)]
pub(super) struct SynFile(syn::File);
#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    proc_macro_newtype::AsRefInner,
    proc_macro_newtype::FromInner,
)]
pub(super) struct SynFileRef<'syn_lt>(&'syn_lt syn::File);
#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    proc_macro_newtype::AsRefInner,
    proc_macro_newtype::FromInner,
)]
pub(super) struct SynAttributeRef<'syn_lt>(&'syn_lt syn::Attribute);
#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    proc_macro_newtype::AsRefInner,
    proc_macro_newtype::FromInner,
)]
pub(super) struct SynAttributeListRef<'syn_lt>(&'syn_lt [syn::Attribute]);
#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    proc_macro_newtype::AsRefInner,
    proc_macro_newtype::FromInner,
    proc_macro_newtype::GetInner,
)]
pub(super) struct SynExprCallRef<'syn_lt>(&'syn_lt syn::ExprCall);
#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    proc_macro_newtype::AsRefInner,
    proc_macro_newtype::FromInner,
)]
pub(super) struct SynFieldsRef<'syn_lt>(&'syn_lt syn::Fields);
#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    proc_macro_newtype::AsRefInner,
    proc_macro_newtype::FromInner,
)]
pub(super) struct SynGenericsRef<'syn_lt>(&'syn_lt syn::Generics);
#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    proc_macro_newtype::AsRefInner,
    proc_macro_newtype::FromInner,
)]
pub(super) struct SynItemImplRef<'syn_lt>(&'syn_lt syn::ItemImpl);
#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    proc_macro_newtype::AsRefInner,
    proc_macro_newtype::FromInner,
)]
pub(super) struct SynItemFnRef<'syn_lt>(&'syn_lt syn::ItemFn);
#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    proc_macro_newtype::AsRefInner,
    proc_macro_newtype::FromInner,
)]
pub(super) struct SynItemRef<'syn_lt>(&'syn_lt syn::Item);
#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    proc_macro_newtype::AsRefInner,
    proc_macro_newtype::FromInner,
)]
pub(super) struct SynItemStructRef<'syn_lt>(&'syn_lt syn::ItemStruct);
#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    proc_macro_newtype::AsRefInner,
    proc_macro_newtype::FromInner,
    proc_macro_newtype::GetInner,
)]
pub(super) struct SynPathArgumentsRef<'syn_lt>(&'syn_lt syn::PathArguments);
#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    proc_macro_newtype::AsRefInner,
    proc_macro_newtype::FromInner,
    proc_macro_newtype::GetInner,
)]
pub(super) struct SynPathSegmentRef<'syn_lt>(&'syn_lt syn::PathSegment);
#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    proc_macro_newtype::AsRefInner,
    proc_macro_newtype::FromInner,
)]
pub(super) struct SynPathRef<'syn_lt>(&'syn_lt syn::Path);
#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    proc_macro_newtype::AsRefInner,
    proc_macro_newtype::FromInner,
)]
pub(super) struct SynSignatureRef<'syn_lt>(&'syn_lt syn::Signature);
#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    proc_macro_newtype::AsRefInner,
    proc_macro_newtype::FromInner,
    proc_macro_newtype::GetInner,
)]
pub(super) struct SynTypePathRef<'syn_lt>(&'syn_lt syn::TypePath);
#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    proc_macro_newtype::AsRefInner,
    proc_macro_newtype::FromInner,
    proc_macro_newtype::GetInner,
)]
pub(super) struct SynTypeRef<'syn_lt>(&'syn_lt syn::Type);
#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    proc_macro_newtype::AsRefInner,
    proc_macro_newtype::FromInner,
)]
pub(super) struct SynUseTreeRef<'syn_lt>(&'syn_lt syn::UseTree);
#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    proc_macro_newtype::AsRefInner,
    proc_macro_newtype::FromInner,
)]
pub(super) struct SynIdentifierRef<'syn_lt>(&'syn_lt syn::Ident);
#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    proc_macro_newtype::AsRefOwned,
    proc_macro_newtype::FromInner,
    proc_macro_newtype::IntoInnerFrom,
)]
pub(super) struct TomlTable(toml::Table);
#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    proc_macro_newtype::AsRefInner,
    proc_macro_newtype::FromInner,
    proc_macro_newtype::GetInner,
)]
pub(super) struct TomlTableRef<'toml_lt>(&'toml_lt toml::value::Table);
#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    proc_macro_newtype::AsRefInner,
    proc_macro_newtype::FromInner,
    proc_macro_newtype::GetInner,
)]
pub(super) struct TomlValueRef<'toml_lt>(&'toml_lt toml::Value);
#[derive(
    proc_macro_getters::Getters,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    proc_macro_newtype::FromInner,
    proc_macro_newtype::IntoIterator,
)]
pub(super) struct WalkdirWalkDir(walkdir::WalkDir);
