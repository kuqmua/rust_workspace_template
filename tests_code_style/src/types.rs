const SOURCE_TEXT_MAX_LEN: usize = 16 * 1024 * 1024;
#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Debug, Clone, Copy, Default, newtype::FromInner,
)]
pub(super) struct AnalyzerCount(usize);
impl AnalyzerCount {
    pub(super) const fn get(self) -> usize {
        self.0
    }
    pub(super) fn saturating_dec(&mut self) {
        self.0 = self.0.saturating_sub(1);
    }
    pub(super) fn saturating_inc(&mut self) {
        self.0 = self.0.saturating_add(1);
    }
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Debug, Clone, Copy, Default, newtype::FromInner,
)]
pub(super) struct AnalyzerBool(bool);
impl AnalyzerBool {
    pub(super) const fn get(self) -> bool {
        self.0
    }
    pub(super) fn set_true(&mut self) {
        self.0 = true;
    }
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, Clone, Copy, newtype::FromInner)]
pub(super) struct CargoTomlFileIdx(usize);
impl CargoTomlFileIdx {
    pub(super) const fn get(self) -> usize {
        self.0
    }
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    newtype::AsRefOwned,
    newtype::FromInner,
)]
pub(super) struct CargoMetadata(cargo_metadata::Metadata);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    newtype::AsRefInner,
    newtype::FromInner,
)]
pub(super) struct CargoMetadataRef<'metadata_lt>(&'metadata_lt cargo_metadata::Metadata);
impl<'metadata_lt> CargoMetadataRef<'metadata_lt> {
    pub(super) const fn get(self) -> &'metadata_lt cargo_metadata::Metadata {
        self.0
    }
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    newtype::AsRefOwned,
    newtype::FromInner,
)]
pub(super) struct CargoPackageIdRefHashSet<'metadata_lt>(
    std::collections::HashSet<&'metadata_lt cargo_metadata::PackageId>,
);
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, Clone, Copy, newtype::FromInner)]
pub(super) struct StaticStr(&'static str);
impl StaticStr {
    pub(super) const fn get(self) -> &'static str {
        self.0
    }
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, Clone, Copy, newtype::FromInner)]
pub(super) struct StaticStrSliceRef<'text_lt>(&'text_lt [&'text_lt str]);
impl<'text_lt> StaticStrSliceRef<'text_lt> {
    pub(super) const fn get(self) -> &'text_lt [&'text_lt str] {
        self.0
    }
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    newtype::AsRefInner,
    newtype::FromInner,
)]
pub(super) struct SourceTextRef<'text_lt>(&'text_lt str);
impl<'text_lt> SourceTextRef<'text_lt> {
    // The owner module retains lint-sensitive semantics from the original implementation.

    pub(super) const fn get(self) -> &'text_lt str {
        self.0
    }
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    newtype::AsRefInner,
    newtype::FromInner,
)]
pub(super) struct SourceTextRefHashSet<'text_lt>(
    &'text_lt std::collections::HashSet<&'text_lt str>,
);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    newtype::AsRefOwned,
    newtype::FromInner,
)]
pub(super) struct SourceTextHashSet<'text_lt>(std::collections::HashSet<&'text_lt str>);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    newtype::AsRefInner,
    newtype::FromInner,
)]
pub(super) struct SynBlockRef<'block_lt>(&'block_lt syn::Block);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Default,
    newtype::DerefInner,
    newtype::DerefMutInner,
    newtype::FromInner,
    newtype::IntoIterator,
)]
pub(super) struct DiagnosticMsgs(Vec<String>);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    newtype::DerefMutTarget,
    newtype::DerefTarget,
    newtype::FromInner,
)]
pub(super) struct DiagnosticMsgsMutRef<'msgs_lt>(&'msgs_lt mut Vec<String>);
impl<'msgs_lt> From<&'msgs_lt mut SourceTextList> for DiagnosticMsgsMutRef<'msgs_lt> {
    fn from(value: &'msgs_lt mut SourceTextList) -> Self {
        Self(&mut value.0)
    }
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, Clone, newtype::AsRefStr)]
pub(super) struct SourceText(Box<str>);
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, Clone, Copy, thiserror::Error)]
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
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() > SOURCE_TEXT_MAX_LEN {
            return Err(SourceTextTryFromStringError::TooLong {
                len: AnalyzerCount::from(value.len()),
            });
        }
        Ok(Self(value.into_boxed_str()))
    }
}
impl From<SourceText> for String {
    fn from(value: SourceText) -> Self {
        value.0.into_string()
    }
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Default,
    newtype::DerefInner,
    newtype::DerefMutInner,
    newtype::FromInner,
    newtype::IntoIterator,
)]
pub(super) struct SourceTextList(Vec<String>);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    newtype::AsRefInner,
    newtype::FromInner,
)]
pub(super) struct SourceTextListRef<'text_lt>(&'text_lt [String]);
impl<'text_lt> SourceTextListRef<'text_lt> {
    pub(super) const fn get(self) -> &'text_lt [String] {
        self.0
    }
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Default,
    newtype::AsRefOwned,
    newtype::DerefInner,
    newtype::DerefMutInner,
    newtype::FromInner,
    newtype::IntoIterator,
)]
pub(super) struct SourceTextBTreeSet(std::collections::BTreeSet<String>);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    newtype::FromInner,
)]
pub(super) struct FunctionBodyHash(u64);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    newtype::AsRefInner,
    newtype::DerefTarget,
    newtype::FromInner,
)]
pub(super) struct RegexRegexRef<'regex_lt>(&'regex_lt regex::Regex);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Default,
    newtype::DerefInner,
    newtype::FromInner,
    newtype::IntoInnerFrom,
)]
pub(super) struct FunctionBodyLocationsBTreeMap(
    std::collections::BTreeMap<FunctionBodyHash, SourceTextList>,
);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    newtype::DerefMutTarget,
    newtype::DerefTarget,
    newtype::FromInner,
)]
pub(super) struct FunctionBodyLocationsBTreeMapMutRef<'map_lt>(
    &'map_lt mut std::collections::BTreeMap<FunctionBodyHash, SourceTextList>,
);
impl<'map_lt> From<&'map_lt mut FunctionBodyLocationsBTreeMap>
    for FunctionBodyLocationsBTreeMapMutRef<'map_lt>
{
    fn from(value: &'map_lt mut FunctionBodyLocationsBTreeMap) -> Self {
        Self(&mut value.0)
    }
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    newtype::AsRefInner,
    newtype::FromInner,
)]
pub(super) struct SourceTextBTreeSetRef<'text_lt>(&'text_lt std::collections::BTreeSet<String>);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    newtype::AsRefTarget,
    newtype::BorrowPath,
    newtype::DerefTarget,
    newtype::FromInner,
)]
pub(super) struct OwnedPathBuf(std::path::PathBuf);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    newtype::AsRefInner,
    newtype::FromInner,
)]
pub(super) struct PathRef<'path_lt>(&'path_lt std::path::Path);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    newtype::AsRefOwned,
    newtype::FromInner,
)]
pub(super) struct SynFile(syn::File);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    newtype::AsRefInner,
    newtype::FromInner,
)]
pub(super) struct SynFileRef<'syn_lt>(&'syn_lt syn::File);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    newtype::AsRefInner,
    newtype::FromInner,
)]
pub(super) struct SynAttributeRef<'syn_lt>(&'syn_lt syn::Attribute);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    newtype::AsRefInner,
    newtype::FromInner,
)]
pub(super) struct SynAttributeListRef<'syn_lt>(&'syn_lt [syn::Attribute]);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    newtype::AsRefInner,
    newtype::FromInner,
)]
pub(super) struct SynExprCallRef<'syn_lt>(&'syn_lt syn::ExprCall);
impl<'syn_lt> SynExprCallRef<'syn_lt> {
    pub(super) const fn get(self) -> &'syn_lt syn::ExprCall {
        self.0
    }
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    newtype::AsRefInner,
    newtype::FromInner,
)]
pub(super) struct SynFieldsRef<'syn_lt>(&'syn_lt syn::Fields);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    newtype::AsRefInner,
    newtype::FromInner,
)]
pub(super) struct SynGenericsRef<'syn_lt>(&'syn_lt syn::Generics);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    newtype::AsRefInner,
    newtype::FromInner,
)]
pub(super) struct SynItemImplRef<'syn_lt>(&'syn_lt syn::ItemImpl);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    newtype::AsRefInner,
    newtype::FromInner,
)]
pub(super) struct SynItemFnRef<'syn_lt>(&'syn_lt syn::ItemFn);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    newtype::AsRefInner,
    newtype::FromInner,
)]
pub(super) struct SynItemRef<'syn_lt>(&'syn_lt syn::Item);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    newtype::AsRefInner,
    newtype::FromInner,
)]
pub(super) struct SynItemStructRef<'syn_lt>(&'syn_lt syn::ItemStruct);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    newtype::AsRefInner,
    newtype::FromInner,
)]
pub(super) struct SynPathArgumentsRef<'syn_lt>(&'syn_lt syn::PathArguments);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    newtype::AsRefInner,
    newtype::FromInner,
)]
pub(super) struct SynPathSegmentRef<'syn_lt>(&'syn_lt syn::PathSegment);
impl<'syn_lt> SynPathSegmentRef<'syn_lt> {
    pub(super) const fn get(self) -> &'syn_lt syn::PathSegment {
        self.0
    }
}
impl<'syn_lt> SynPathArgumentsRef<'syn_lt> {
    pub(super) const fn get(self) -> &'syn_lt syn::PathArguments {
        self.0
    }
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    newtype::AsRefInner,
    newtype::FromInner,
)]
pub(super) struct SynPathRef<'syn_lt>(&'syn_lt syn::Path);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    newtype::AsRefInner,
    newtype::FromInner,
)]
pub(super) struct SynSignatureRef<'syn_lt>(&'syn_lt syn::Signature);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    newtype::AsRefInner,
    newtype::FromInner,
)]
pub(super) struct SynTypePathRef<'syn_lt>(&'syn_lt syn::TypePath);
impl<'syn_lt> SynTypePathRef<'syn_lt> {
    pub(super) const fn get(self) -> &'syn_lt syn::TypePath {
        self.0
    }
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    newtype::AsRefInner,
    newtype::FromInner,
)]
pub(super) struct SynTypeRef<'syn_lt>(&'syn_lt syn::Type);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    newtype::AsRefInner,
    newtype::FromInner,
)]
pub(super) struct SynUseTreeRef<'syn_lt>(&'syn_lt syn::UseTree);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    newtype::AsRefInner,
    newtype::FromInner,
)]
pub(super) struct SynIdentifierRef<'syn_lt>(&'syn_lt syn::Ident);
impl<'syn_lt> SynTypeRef<'syn_lt> {
    pub(super) const fn get(self) -> &'syn_lt syn::Type {
        self.0
    }
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    newtype::AsRefOwned,
    newtype::FromInner,
    newtype::IntoInnerFrom,
)]
pub(super) struct TomlTable(toml::Table);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    newtype::AsRefInner,
    newtype::FromInner,
)]
pub(super) struct TomlTableRef<'toml_lt>(&'toml_lt toml::value::Table);
impl<'toml_lt> TomlTableRef<'toml_lt> {
    pub(super) const fn get(self) -> &'toml_lt toml::value::Table {
        self.0
    }
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    newtype::AsRefInner,
    newtype::FromInner,
)]
pub(super) struct TomlValueRef<'toml_lt>(&'toml_lt toml::Value);
impl<'toml_lt> TomlValueRef<'toml_lt> {
    pub(super) const fn get(self) -> &'toml_lt toml::Value {
        self.0
    }
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, newtype::FromInner, newtype::IntoIterator)]
pub(super) struct WalkdirWalkDir(walkdir::WalkDir);
