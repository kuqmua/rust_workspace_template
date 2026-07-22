const SOURCE_TEXT_MAX_LEN: usize = 16 * 1024 * 1024;
#[derive(Debug, Clone, Copy, Default, newtype::FromInner)]
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
#[derive(Debug, Clone, Copy, Default, newtype::FromInner)]
pub(super) struct AnalyzerBool(bool);
impl AnalyzerBool {
    pub(super) const fn get(self) -> bool {
        self.0
    }
    pub(super) fn set_true(&mut self) {
        self.0 = true;
    }
}
#[derive(Debug, Clone, Copy, newtype::FromInner)]
pub(super) struct CargoTomlFileIdx(usize);
impl CargoTomlFileIdx {
    pub(super) const fn get(self) -> usize {
        self.0
    }
}
#[derive(Debug, Clone, Copy, newtype::FromInner)]
pub(super) struct AnalyzerChar(char);
impl AnalyzerChar {
    pub(super) const fn get(self) -> char {
        self.0
    }
}
#[derive(Debug, Clone, newtype::AsRefOwned, newtype::FromInner)]
pub(super) struct CargoMetadata(cargo_metadata::Metadata);
#[derive(Debug, Clone, Copy, newtype::AsRefInner, newtype::FromInner)]
pub(super) struct CargoMetadataRef<'metadata_lt>(&'metadata_lt cargo_metadata::Metadata);
impl<'metadata_lt> CargoMetadataRef<'metadata_lt> {
    pub(super) const fn get(self) -> &'metadata_lt cargo_metadata::Metadata {
        self.0
    }
}
#[derive(Debug, Clone, newtype::AsRefOwned, newtype::FromInner)]
pub(super) struct StdCargoPackageIdRefSet<'metadata_lt>(
    std::collections::HashSet<&'metadata_lt cargo_metadata::PackageId>,
);
#[derive(Debug, Clone, Copy, newtype::AsRefInner, newtype::FromInner)]
pub(super) struct StdProcessOutputRef<'output_lt>(&'output_lt std::process::Output);
#[derive(Debug, Clone, Copy, newtype::FromInner)]
pub(super) struct StaticStr(&'static str);
impl StaticStr {
    pub(super) const fn get(self) -> &'static str {
        self.0
    }
}
#[derive(Debug, Clone, Copy, newtype::FromInner)]
pub(super) struct StaticStrSliceRef<'text_lt>(&'text_lt [&'text_lt str]);
impl<'text_lt> StaticStrSliceRef<'text_lt> {
    pub(super) const fn get(self) -> &'text_lt [&'text_lt str] {
        self.0
    }
}
#[derive(Debug, Clone, Copy, newtype::AsRefInner, newtype::FromInner)]
pub(super) struct SourceTextRef<'text_lt>(&'text_lt str);
impl<'text_lt> SourceTextRef<'text_lt> {
    #[allow(clippy::single_call_fn)] // preserves the source lifetime where AsRef would borrow the wrapper temporary
    pub(super) const fn get(self) -> &'text_lt str {
        self.0
    }
}
#[derive(Debug, Clone, Copy, newtype::AsRefInner, newtype::FromInner)]
pub(super) struct StdSourceTextRefSet<'text_lt>(&'text_lt std::collections::HashSet<&'text_lt str>);
#[derive(Debug, Clone, newtype::AsRefOwned, newtype::FromInner)]
pub(super) struct StdSourceTextHashSet<'text_lt>(std::collections::HashSet<&'text_lt str>);
#[derive(Debug, Clone, Copy, newtype::AsRefInner, newtype::FromInner)]
pub(super) struct SynBlockRef<'block_lt>(&'block_lt syn::Block);
#[derive(
    Debug,
    Clone,
    Default,
    newtype::DerefInner,
    newtype::DerefMutInner,
    newtype::FromInner,
    newtype::IntoIterator,
)]
pub(super) struct DiagnosticMsgs(Vec<String>);
#[derive(Debug, newtype::DerefMutTarget, newtype::DerefTarget, newtype::FromInner)]
pub(super) struct DiagnosticMsgsMutRef<'msgs_lt>(&'msgs_lt mut Vec<String>);
impl<'msgs_lt> From<&'msgs_lt mut SourceTextList> for DiagnosticMsgsMutRef<'msgs_lt> {
    fn from(value: &'msgs_lt mut SourceTextList) -> Self {
        Self(&mut value.0)
    }
}
#[derive(Debug, Clone, newtype::AsRefStr)]
pub(super) struct SourceText(Box<str>);
#[derive(Debug, Clone, Copy)]
pub(super) struct SourceTextTryFromStringError {
    len: AnalyzerCount,
}
impl TryFrom<String> for SourceText {
    type Error = SourceTextTryFromStringError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() > SOURCE_TEXT_MAX_LEN {
            return Err(SourceTextTryFromStringError {
                len: AnalyzerCount::from(value.len()),
            });
        }
        Ok(Self(value.into_boxed_str()))
    }
}
impl std::fmt::Display for SourceTextTryFromStringError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "source text length {} exceeds max {}",
            self.len.get(),
            SOURCE_TEXT_MAX_LEN
        )
    }
}
impl std::error::Error for SourceTextTryFromStringError {}
impl From<SourceText> for String {
    fn from(value: SourceText) -> Self {
        value.0.into_string()
    }
}
#[derive(
    Debug,
    Clone,
    Default,
    newtype::DerefInner,
    newtype::DerefMutInner,
    newtype::FromInner,
    newtype::IntoIterator,
)]
pub(super) struct SourceTextList(Vec<String>);
#[derive(Debug, Clone, Copy, newtype::AsRefInner, newtype::FromInner)]
pub(super) struct SourceTextListRef<'text_lt>(&'text_lt [String]);
impl<'text_lt> SourceTextListRef<'text_lt> {
    pub(super) const fn get(self) -> &'text_lt [String] {
        self.0
    }
}
#[derive(
    Debug,
    Clone,
    Default,
    newtype::AsRefOwned,
    newtype::DerefInner,
    newtype::DerefMutInner,
    newtype::FromInner,
    newtype::IntoIterator,
)]
pub(super) struct StdSourceTextSet(std::collections::BTreeSet<String>);
#[derive(Debug, Clone, Copy, newtype::AsRefInner, newtype::FromInner)]
pub(super) struct StdStdSourceTextSetRef<'text_lt>(&'text_lt std::collections::BTreeSet<String>);
#[derive(
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
pub(super) struct StdPathBuf(std::path::PathBuf);
#[derive(Debug, Clone, Copy, newtype::AsRefInner, newtype::FromInner)]
pub(super) struct StdPathRef<'path_lt>(&'path_lt std::path::Path);
#[derive(Debug, Clone, newtype::AsRefOwned, newtype::FromInner)]
pub(super) struct SynFile(syn::File);
#[derive(Debug, Clone, Copy, newtype::AsRefInner, newtype::FromInner)]
pub(super) struct SynFileRef<'syn_lt>(&'syn_lt syn::File);
#[derive(Debug, Clone, Copy, newtype::AsRefInner, newtype::FromInner)]
pub(super) struct SynAttributeRef<'syn_lt>(&'syn_lt syn::Attribute);
#[derive(Debug, Clone, Copy, newtype::AsRefInner, newtype::FromInner)]
pub(super) struct SynAttributeListRef<'syn_lt>(&'syn_lt [syn::Attribute]);
#[derive(Debug, Clone, Copy, newtype::AsRefInner, newtype::FromInner)]
pub(super) struct SynExprCallRef<'syn_lt>(&'syn_lt syn::ExprCall);
impl<'syn_lt> SynExprCallRef<'syn_lt> {
    pub(super) const fn get(self) -> &'syn_lt syn::ExprCall {
        self.0
    }
}
#[derive(Debug, Clone, Copy, newtype::AsRefInner, newtype::FromInner)]
pub(super) struct SynFieldsRef<'syn_lt>(&'syn_lt syn::Fields);
#[derive(Debug, Clone, Copy, newtype::AsRefInner, newtype::FromInner)]
pub(super) struct SynGenericsRef<'syn_lt>(&'syn_lt syn::Generics);
#[derive(Debug, Clone, Copy, newtype::AsRefInner, newtype::FromInner)]
pub(super) struct SynItemImplRef<'syn_lt>(&'syn_lt syn::ItemImpl);
#[derive(Debug, Clone, Copy, newtype::AsRefInner, newtype::FromInner)]
pub(super) struct SynItemFnRef<'syn_lt>(&'syn_lt syn::ItemFn);
#[derive(Debug, Clone, Copy, newtype::AsRefInner, newtype::FromInner)]
pub(super) struct SynItemRef<'syn_lt>(&'syn_lt syn::Item);
#[derive(Debug, Clone, Copy, newtype::AsRefInner, newtype::FromInner)]
pub(super) struct SynItemStructRef<'syn_lt>(&'syn_lt syn::ItemStruct);
#[derive(Debug, Clone, Copy, newtype::AsRefInner, newtype::FromInner)]
pub(super) struct SynPathArgumentsRef<'syn_lt>(&'syn_lt syn::PathArguments);
#[derive(Debug, Clone, Copy, newtype::AsRefInner, newtype::FromInner)]
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
#[derive(Debug, Clone, Copy, newtype::AsRefInner, newtype::FromInner)]
pub(super) struct SynPathRef<'syn_lt>(&'syn_lt syn::Path);
#[derive(Debug, Clone, Copy, newtype::AsRefInner, newtype::FromInner)]
pub(super) struct SynSignatureRef<'syn_lt>(&'syn_lt syn::Signature);
#[derive(Debug, Clone, Copy, newtype::AsRefInner, newtype::FromInner)]
pub(super) struct SynTypePathRef<'syn_lt>(&'syn_lt syn::TypePath);
impl<'syn_lt> SynTypePathRef<'syn_lt> {
    pub(super) const fn get(self) -> &'syn_lt syn::TypePath {
        self.0
    }
}
#[derive(Debug, Clone, Copy, newtype::AsRefInner, newtype::FromInner)]
pub(super) struct SynTypeRef<'syn_lt>(&'syn_lt syn::Type);
#[derive(Debug, Clone, Copy, newtype::AsRefInner, newtype::FromInner)]
pub(super) struct SynUseTreeRef<'syn_lt>(&'syn_lt syn::UseTree);
#[derive(Debug, Clone, Copy, newtype::AsRefInner, newtype::FromInner)]
pub(super) struct SynIdentifierRef<'syn_lt>(&'syn_lt syn::Ident);
impl<'syn_lt> SynTypeRef<'syn_lt> {
    pub(super) const fn get(self) -> &'syn_lt syn::Type {
        self.0
    }
}
#[derive(Debug, Clone, newtype::AsRefOwned, newtype::FromInner, newtype::IntoInnerFrom)]
pub(super) struct TomlTable(toml::Table);
#[derive(Debug, Clone, Copy, newtype::AsRefInner, newtype::FromInner)]
pub(super) struct TomlTableRef<'toml_lt>(&'toml_lt toml::value::Table);
impl<'toml_lt> TomlTableRef<'toml_lt> {
    pub(super) const fn get(self) -> &'toml_lt toml::value::Table {
        self.0
    }
}
#[derive(Debug, Clone, Copy, newtype::AsRefInner, newtype::FromInner)]
pub(super) struct TomlValueRef<'toml_lt>(&'toml_lt toml::Value);
impl<'toml_lt> TomlValueRef<'toml_lt> {
    pub(super) const fn get(self) -> &'toml_lt toml::Value {
        self.0
    }
}
#[derive(Debug, Clone, newtype::FromInner, newtype::IntoInner)]
pub(super) struct TomlValue(toml::Value);
#[derive(newtype::FromInner, newtype::IntoIterator)]
pub(super) struct WalkdirWalkDir(walkdir::WalkDir);
