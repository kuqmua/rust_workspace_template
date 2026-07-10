const SOURCE_TEXT_MAX_LEN: usize = 16 * 1024 * 1024;
#[derive(Debug, Clone, Copy, Default)]
pub(super) struct AnalyzerCount(usize);
impl From<usize> for AnalyzerCount {
    fn from(value: usize) -> Self {
        Self(value)
    }
}
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
#[derive(Debug, Clone, Copy, Default)]
pub(super) struct AnalyzerBool(bool);
impl From<bool> for AnalyzerBool {
    fn from(value: bool) -> Self {
        Self(value)
    }
}
impl AnalyzerBool {
    pub(super) const fn get(self) -> bool {
        self.0
    }
    pub(super) fn set_true(&mut self) {
        self.0 = true;
    }
}
#[derive(Debug, Clone, Copy)]
pub(super) struct CargoTomlFileIdx(usize);
impl From<usize> for CargoTomlFileIdx {
    fn from(value: usize) -> Self {
        Self(value)
    }
}
impl CargoTomlFileIdx {
    pub(super) const fn get(self) -> usize {
        self.0
    }
}
#[derive(Debug, Clone, Copy)]
pub(super) struct AnalyzerChar(char);
impl From<char> for AnalyzerChar {
    fn from(value: char) -> Self {
        Self(value)
    }
}
impl AnalyzerChar {
    pub(super) const fn get(self) -> char {
        self.0
    }
}
#[derive(Debug, Clone)]
pub(super) struct CargoMetadata(cargo_metadata::Metadata);
impl From<cargo_metadata::Metadata> for CargoMetadata {
    fn from(value: cargo_metadata::Metadata) -> Self {
        Self(value)
    }
}
#[derive(Debug, Clone, Copy)]
pub(super) struct CargoMetadataRef<'metadata_lt>(&'metadata_lt cargo_metadata::Metadata);
impl<'metadata_lt> From<&'metadata_lt cargo_metadata::Metadata> for CargoMetadataRef<'metadata_lt> {
    fn from(value: &'metadata_lt cargo_metadata::Metadata) -> Self {
        Self(value)
    }
}
impl AsRef<cargo_metadata::Metadata> for CargoMetadataRef<'_> {
    fn as_ref(&self) -> &cargo_metadata::Metadata {
        self.0
    }
}
impl<'metadata_lt> CargoMetadataRef<'metadata_lt> {
    pub(super) const fn get(self) -> &'metadata_lt cargo_metadata::Metadata {
        self.0
    }
}
#[derive(Debug, Clone)]
pub(super) struct StdCargoPackageIdRefSet<'metadata_lt>(
    std::collections::HashSet<&'metadata_lt cargo_metadata::PackageId>,
);
impl<'metadata_lt> From<std::collections::HashSet<&'metadata_lt cargo_metadata::PackageId>>
    for StdCargoPackageIdRefSet<'metadata_lt>
{
    fn from(value: std::collections::HashSet<&'metadata_lt cargo_metadata::PackageId>) -> Self {
        Self(value)
    }
}
impl<'metadata_lt> AsRef<std::collections::HashSet<&'metadata_lt cargo_metadata::PackageId>>
    for StdCargoPackageIdRefSet<'metadata_lt>
{
    fn as_ref(&self) -> &std::collections::HashSet<&'metadata_lt cargo_metadata::PackageId> {
        &self.0
    }
}
#[derive(Debug, Clone, Copy)]
pub(super) struct StdOsStrRef<'os_lt>(&'os_lt std::ffi::OsStr);
impl<'os_lt> From<&'os_lt std::ffi::OsStr> for StdOsStrRef<'os_lt> {
    fn from(value: &'os_lt std::ffi::OsStr) -> Self {
        Self(value)
    }
}
#[derive(Debug, Clone, Copy)]
pub(super) struct StdProcessOutputRef<'output_lt>(&'output_lt std::process::Output);
impl<'output_lt> From<&'output_lt std::process::Output> for StdProcessOutputRef<'output_lt> {
    fn from(value: &'output_lt std::process::Output) -> Self {
        Self(value)
    }
}
#[derive(Debug, Clone, Copy)]
pub(super) struct StaticStr(pub &'static str);
impl From<&'static str> for StaticStr {
    fn from(value: &'static str) -> Self {
        Self(value)
    }
}
impl StaticStr {
    pub(super) const fn get(self) -> &'static str {
        self.0
    }
}
#[derive(Debug, Clone, Copy)]
pub(super) struct StaticStrSliceRef<'text_lt>(&'text_lt [&'text_lt str]);
impl<'text_lt> From<&'text_lt [&'text_lt str]> for StaticStrSliceRef<'text_lt> {
    fn from(value: &'text_lt [&'text_lt str]) -> Self {
        Self(value)
    }
}
impl<'text_lt> StaticStrSliceRef<'text_lt> {
    pub(super) const fn get(self) -> &'text_lt [&'text_lt str] {
        self.0
    }
}
#[derive(Debug, Clone, Copy)]
pub(super) struct SourceTextRef<'text_lt>(&'text_lt str);
impl<'text_lt> From<&'text_lt str> for SourceTextRef<'text_lt> {
    fn from(value: &'text_lt str) -> Self {
        Self(value)
    }
}
impl AsRef<str> for SourceTextRef<'_> {
    fn as_ref(&self) -> &str {
        self.0
    }
}
impl<'text_lt> SourceTextRef<'text_lt> {
    #[allow(clippy::single_call_fn)] // preserves the source lifetime where AsRef would borrow the wrapper temporary
    pub(super) const fn get(self) -> &'text_lt str {
        self.0
    }
}
#[derive(Debug, Clone, Copy)]
pub(super) struct StdSourceTextRefSet<'text_lt>(&'text_lt std::collections::HashSet<&'text_lt str>);
impl<'text_lt> From<&'text_lt std::collections::HashSet<&'text_lt str>>
    for StdSourceTextRefSet<'text_lt>
{
    fn from(value: &'text_lt std::collections::HashSet<&'text_lt str>) -> Self {
        Self(value)
    }
}
#[derive(Debug, Clone)]
pub(super) struct StdSourceTextHashSet<'text_lt>(std::collections::HashSet<&'text_lt str>);
impl<'text_lt> From<std::collections::HashSet<&'text_lt str>> for StdSourceTextHashSet<'text_lt> {
    fn from(value: std::collections::HashSet<&'text_lt str>) -> Self {
        Self(value)
    }
}
impl<'text_lt> AsRef<std::collections::HashSet<&'text_lt str>> for StdSourceTextHashSet<'text_lt> {
    fn as_ref(&self) -> &std::collections::HashSet<&'text_lt str> {
        &self.0
    }
}
impl<'text_lt> AsRef<std::collections::HashSet<&'text_lt str>> for StdSourceTextRefSet<'text_lt> {
    fn as_ref(&self) -> &std::collections::HashSet<&'text_lt str> {
        self.0
    }
}
impl AsRef<std::ffi::OsStr> for StdOsStrRef<'_> {
    fn as_ref(&self) -> &std::ffi::OsStr {
        self.0
    }
}
impl AsRef<std::process::Output> for StdProcessOutputRef<'_> {
    fn as_ref(&self) -> &std::process::Output {
        self.0
    }
}
#[derive(Debug, Clone, Copy)]
pub(super) struct SynBlockRef<'block_lt>(&'block_lt syn::Block);
impl<'block_lt> From<&'block_lt syn::Block> for SynBlockRef<'block_lt> {
    fn from(value: &'block_lt syn::Block) -> Self {
        Self(value)
    }
}
impl AsRef<syn::Block> for SynBlockRef<'_> {
    fn as_ref(&self) -> &syn::Block {
        self.0
    }
}
impl AsRef<cargo_metadata::Metadata> for CargoMetadata {
    fn as_ref(&self) -> &cargo_metadata::Metadata {
        &self.0
    }
}
#[derive(Debug, Clone, Default)]
pub(super) struct DiagnosticMsgs(Vec<String>);
impl std::ops::Deref for DiagnosticMsgs {
    type Target = Vec<String>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl std::ops::DerefMut for DiagnosticMsgs {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl IntoIterator for DiagnosticMsgs {
    type IntoIter = std::vec::IntoIter<String>;
    type Item = String;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
#[derive(Debug)]
pub(super) struct DiagnosticMsgsMutRef<'msgs_lt>(&'msgs_lt mut Vec<String>);
impl<'msgs_lt> From<&'msgs_lt mut Vec<String>> for DiagnosticMsgsMutRef<'msgs_lt> {
    fn from(value: &'msgs_lt mut Vec<String>) -> Self {
        Self(value)
    }
}
impl<'msgs_lt> From<&'msgs_lt mut SourceTextList> for DiagnosticMsgsMutRef<'msgs_lt> {
    fn from(value: &'msgs_lt mut SourceTextList) -> Self {
        Self(&mut value.0)
    }
}
impl std::ops::Deref for DiagnosticMsgsMutRef<'_> {
    type Target = Vec<String>;
    fn deref(&self) -> &Self::Target {
        self.0
    }
}
impl std::ops::DerefMut for DiagnosticMsgsMutRef<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0
    }
}
#[derive(Debug, Clone)]
pub(super) struct SourceText(Box<str>);
#[derive(Debug, Clone, Copy)]
pub(super) struct SourceTextTryFromStringEr {
    len: AnalyzerCount,
}
impl TryFrom<String> for SourceText {
    type Error = SourceTextTryFromStringEr;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() > SOURCE_TEXT_MAX_LEN {
            return Err(SourceTextTryFromStringEr {
                len: AnalyzerCount::from(value.len()),
            });
        }
        Ok(Self(value.into_boxed_str()))
    }
}
impl std::fmt::Display for SourceTextTryFromStringEr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "source text length {} exceeds max {}",
            self.len.get(),
            SOURCE_TEXT_MAX_LEN
        )
    }
}
impl std::error::Error for SourceTextTryFromStringEr {}
impl AsRef<str> for SourceText {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
impl From<SourceText> for String {
    fn from(value: SourceText) -> Self {
        value.0.into_string()
    }
}
#[derive(Debug, Clone, Default)]
pub(super) struct SourceTextList(Vec<String>);
impl From<Vec<String>> for SourceTextList {
    fn from(value: Vec<String>) -> Self {
        Self(value)
    }
}
impl std::ops::Deref for SourceTextList {
    type Target = Vec<String>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl std::ops::DerefMut for SourceTextList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl IntoIterator for SourceTextList {
    type IntoIter = std::vec::IntoIter<String>;
    type Item = String;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
#[derive(Debug, Clone, Copy)]
pub(super) struct SourceTextListRef<'text_lt>(&'text_lt [String]);
impl<'text_lt> From<&'text_lt [String]> for SourceTextListRef<'text_lt> {
    fn from(value: &'text_lt [String]) -> Self {
        Self(value)
    }
}
impl AsRef<[String]> for SourceTextListRef<'_> {
    fn as_ref(&self) -> &[String] {
        self.0
    }
}
impl<'text_lt> SourceTextListRef<'text_lt> {
    pub(super) const fn get(self) -> &'text_lt [String] {
        self.0
    }
}
#[derive(Debug, Clone, Default)]
pub(super) struct StdSourceTextSet(std::collections::BTreeSet<String>);
impl From<std::collections::BTreeSet<String>> for StdSourceTextSet {
    fn from(value: std::collections::BTreeSet<String>) -> Self {
        Self(value)
    }
}
impl AsRef<std::collections::BTreeSet<String>> for StdSourceTextSet {
    fn as_ref(&self) -> &std::collections::BTreeSet<String> {
        &self.0
    }
}
impl std::ops::Deref for StdSourceTextSet {
    type Target = std::collections::BTreeSet<String>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl std::ops::DerefMut for StdSourceTextSet {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl IntoIterator for StdSourceTextSet {
    type IntoIter = std::collections::btree_set::IntoIter<String>;
    type Item = String;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
#[derive(Debug, Clone, Copy)]
pub(super) struct StdStdSourceTextSetRef<'text_lt>(&'text_lt std::collections::BTreeSet<String>);
impl<'text_lt> From<&'text_lt std::collections::BTreeSet<String>>
    for StdStdSourceTextSetRef<'text_lt>
{
    fn from(value: &'text_lt std::collections::BTreeSet<String>) -> Self {
        Self(value)
    }
}
impl AsRef<std::collections::BTreeSet<String>> for StdStdSourceTextSetRef<'_> {
    fn as_ref(&self) -> &std::collections::BTreeSet<String> {
        self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(super) struct StdPathBuf(std::path::PathBuf);
impl From<std::path::PathBuf> for StdPathBuf {
    fn from(value: std::path::PathBuf) -> Self {
        Self(value)
    }
}
impl AsRef<std::path::Path> for StdPathBuf {
    fn as_ref(&self) -> &std::path::Path {
        &self.0
    }
}
impl std::borrow::Borrow<std::path::Path> for StdPathBuf {
    fn borrow(&self) -> &std::path::Path {
        &self.0
    }
}
impl std::ops::Deref for StdPathBuf {
    type Target = std::path::Path;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[derive(Debug, Clone, Copy)]
pub(super) struct StdPathRef<'path_lt>(&'path_lt std::path::Path);
impl<'path_lt> From<&'path_lt std::path::Path> for StdPathRef<'path_lt> {
    fn from(value: &'path_lt std::path::Path) -> Self {
        Self(value)
    }
}
impl AsRef<std::path::Path> for StdPathRef<'_> {
    fn as_ref(&self) -> &std::path::Path {
        self.0
    }
}
#[derive(Debug, Clone)]
pub(super) struct SynFile(syn::File);
impl From<syn::File> for SynFile {
    fn from(value: syn::File) -> Self {
        Self(value)
    }
}
#[derive(Debug, Clone, Copy)]
pub(super) struct SynFileRef<'syn_lt>(&'syn_lt syn::File);
impl<'syn_lt> From<&'syn_lt syn::File> for SynFileRef<'syn_lt> {
    fn from(value: &'syn_lt syn::File) -> Self {
        Self(value)
    }
}
impl AsRef<syn::File> for SynFileRef<'_> {
    fn as_ref(&self) -> &syn::File {
        self.0
    }
}
#[derive(Debug, Clone, Copy)]
pub(super) struct SynAttributeRef<'syn_lt>(&'syn_lt syn::Attribute);
impl<'syn_lt> From<&'syn_lt syn::Attribute> for SynAttributeRef<'syn_lt> {
    fn from(value: &'syn_lt syn::Attribute) -> Self {
        Self(value)
    }
}
#[derive(Debug, Clone, Copy)]
pub(super) struct SynAttributeListRef<'syn_lt>(&'syn_lt [syn::Attribute]);
impl<'syn_lt> From<&'syn_lt [syn::Attribute]> for SynAttributeListRef<'syn_lt> {
    fn from(value: &'syn_lt [syn::Attribute]) -> Self {
        Self(value)
    }
}
impl AsRef<[syn::Attribute]> for SynAttributeListRef<'_> {
    fn as_ref(&self) -> &[syn::Attribute] {
        self.0
    }
}
impl AsRef<syn::Attribute> for SynAttributeRef<'_> {
    fn as_ref(&self) -> &syn::Attribute {
        self.0
    }
}
#[derive(Debug, Clone, Copy)]
pub(super) struct SynExprCallRef<'syn_lt>(&'syn_lt syn::ExprCall);
impl<'syn_lt> From<&'syn_lt syn::ExprCall> for SynExprCallRef<'syn_lt> {
    fn from(value: &'syn_lt syn::ExprCall) -> Self {
        Self(value)
    }
}
impl AsRef<syn::ExprCall> for SynExprCallRef<'_> {
    fn as_ref(&self) -> &syn::ExprCall {
        self.0
    }
}
impl<'syn_lt> SynExprCallRef<'syn_lt> {
    pub(super) const fn get(self) -> &'syn_lt syn::ExprCall {
        self.0
    }
}
#[derive(Debug, Clone, Copy)]
pub(super) struct SynFieldsRef<'syn_lt>(&'syn_lt syn::Fields);
impl<'syn_lt> From<&'syn_lt syn::Fields> for SynFieldsRef<'syn_lt> {
    fn from(value: &'syn_lt syn::Fields) -> Self {
        Self(value)
    }
}
impl AsRef<syn::Fields> for SynFieldsRef<'_> {
    fn as_ref(&self) -> &syn::Fields {
        self.0
    }
}
#[derive(Debug, Clone, Copy)]
pub(super) struct SynGenericsRef<'syn_lt>(&'syn_lt syn::Generics);
impl<'syn_lt> From<&'syn_lt syn::Generics> for SynGenericsRef<'syn_lt> {
    fn from(value: &'syn_lt syn::Generics) -> Self {
        Self(value)
    }
}
#[derive(Debug, Clone, Copy)]
pub(super) struct SynItemImplRef<'syn_lt>(&'syn_lt syn::ItemImpl);
impl<'syn_lt> From<&'syn_lt syn::ItemImpl> for SynItemImplRef<'syn_lt> {
    fn from(value: &'syn_lt syn::ItemImpl) -> Self {
        Self(value)
    }
}
#[derive(Debug, Clone, Copy)]
pub(super) struct SynItemFnRef<'syn_lt>(&'syn_lt syn::ItemFn);
impl<'syn_lt> From<&'syn_lt syn::ItemFn> for SynItemFnRef<'syn_lt> {
    fn from(value: &'syn_lt syn::ItemFn) -> Self {
        Self(value)
    }
}
impl AsRef<syn::ItemFn> for SynItemFnRef<'_> {
    fn as_ref(&self) -> &syn::ItemFn {
        self.0
    }
}
#[derive(Debug, Clone, Copy)]
pub(super) struct SynItemRef<'syn_lt>(&'syn_lt syn::Item);
impl<'syn_lt> From<&'syn_lt syn::Item> for SynItemRef<'syn_lt> {
    fn from(value: &'syn_lt syn::Item) -> Self {
        Self(value)
    }
}
impl AsRef<syn::Item> for SynItemRef<'_> {
    fn as_ref(&self) -> &syn::Item {
        self.0
    }
}
impl AsRef<syn::ItemImpl> for SynItemImplRef<'_> {
    fn as_ref(&self) -> &syn::ItemImpl {
        self.0
    }
}
#[derive(Debug, Clone, Copy)]
pub(super) struct SynItemStructRef<'syn_lt>(&'syn_lt syn::ItemStruct);
impl<'syn_lt> From<&'syn_lt syn::ItemStruct> for SynItemStructRef<'syn_lt> {
    fn from(value: &'syn_lt syn::ItemStruct) -> Self {
        Self(value)
    }
}
impl AsRef<syn::ItemStruct> for SynItemStructRef<'_> {
    fn as_ref(&self) -> &syn::ItemStruct {
        self.0
    }
}
impl AsRef<syn::Generics> for SynGenericsRef<'_> {
    fn as_ref(&self) -> &syn::Generics {
        self.0
    }
}
#[derive(Debug, Clone, Copy)]
pub(super) struct SynPathArgumentsRef<'syn_lt>(&'syn_lt syn::PathArguments);
impl<'syn_lt> From<&'syn_lt syn::PathArguments> for SynPathArgumentsRef<'syn_lt> {
    fn from(value: &'syn_lt syn::PathArguments) -> Self {
        Self(value)
    }
}
#[derive(Debug, Clone, Copy)]
pub(super) struct SynPathSegmentRef<'syn_lt>(&'syn_lt syn::PathSegment);
impl<'syn_lt> From<&'syn_lt syn::PathSegment> for SynPathSegmentRef<'syn_lt> {
    fn from(value: &'syn_lt syn::PathSegment) -> Self {
        Self(value)
    }
}
impl AsRef<syn::PathSegment> for SynPathSegmentRef<'_> {
    fn as_ref(&self) -> &syn::PathSegment {
        self.0
    }
}
impl<'syn_lt> SynPathSegmentRef<'syn_lt> {
    pub(super) const fn get(self) -> &'syn_lt syn::PathSegment {
        self.0
    }
}
impl AsRef<syn::PathArguments> for SynPathArgumentsRef<'_> {
    fn as_ref(&self) -> &syn::PathArguments {
        self.0
    }
}
impl<'syn_lt> SynPathArgumentsRef<'syn_lt> {
    pub(super) const fn get(self) -> &'syn_lt syn::PathArguments {
        self.0
    }
}
#[derive(Debug, Clone, Copy)]
pub(super) struct SynPathRef<'syn_lt>(&'syn_lt syn::Path);
impl<'syn_lt> From<&'syn_lt syn::Path> for SynPathRef<'syn_lt> {
    fn from(value: &'syn_lt syn::Path) -> Self {
        Self(value)
    }
}
impl AsRef<syn::Path> for SynPathRef<'_> {
    fn as_ref(&self) -> &syn::Path {
        self.0
    }
}
#[derive(Debug, Clone, Copy)]
pub(super) struct SynSignatureRef<'syn_lt>(&'syn_lt syn::Signature);
impl<'syn_lt> From<&'syn_lt syn::Signature> for SynSignatureRef<'syn_lt> {
    fn from(value: &'syn_lt syn::Signature) -> Self {
        Self(value)
    }
}
impl AsRef<syn::Signature> for SynSignatureRef<'_> {
    fn as_ref(&self) -> &syn::Signature {
        self.0
    }
}
#[derive(Debug, Clone, Copy)]
pub(super) struct SynTypePathRef<'syn_lt>(&'syn_lt syn::TypePath);
impl<'syn_lt> From<&'syn_lt syn::TypePath> for SynTypePathRef<'syn_lt> {
    fn from(value: &'syn_lt syn::TypePath) -> Self {
        Self(value)
    }
}
impl AsRef<syn::TypePath> for SynTypePathRef<'_> {
    fn as_ref(&self) -> &syn::TypePath {
        self.0
    }
}
impl<'syn_lt> SynTypePathRef<'syn_lt> {
    pub(super) const fn get(self) -> &'syn_lt syn::TypePath {
        self.0
    }
}
#[derive(Debug, Clone, Copy)]
pub(super) struct SynTypeRef<'syn_lt>(&'syn_lt syn::Type);
impl<'syn_lt> From<&'syn_lt syn::Type> for SynTypeRef<'syn_lt> {
    fn from(value: &'syn_lt syn::Type) -> Self {
        Self(value)
    }
}
#[derive(Debug, Clone, Copy)]
pub(super) struct SynUseTreeRef<'syn_lt>(&'syn_lt syn::UseTree);
impl<'syn_lt> From<&'syn_lt syn::UseTree> for SynUseTreeRef<'syn_lt> {
    fn from(value: &'syn_lt syn::UseTree) -> Self {
        Self(value)
    }
}
#[derive(Debug, Clone, Copy)]
pub(super) struct SynIdentRef<'syn_lt>(&'syn_lt syn::Ident);
impl<'syn_lt> From<&'syn_lt syn::Ident> for SynIdentRef<'syn_lt> {
    fn from(value: &'syn_lt syn::Ident) -> Self {
        Self(value)
    }
}
impl AsRef<syn::Ident> for SynIdentRef<'_> {
    fn as_ref(&self) -> &syn::Ident {
        self.0
    }
}
impl AsRef<syn::UseTree> for SynUseTreeRef<'_> {
    fn as_ref(&self) -> &syn::UseTree {
        self.0
    }
}
impl AsRef<syn::Type> for SynTypeRef<'_> {
    fn as_ref(&self) -> &syn::Type {
        self.0
    }
}
impl<'syn_lt> SynTypeRef<'syn_lt> {
    pub(super) const fn get(self) -> &'syn_lt syn::Type {
        self.0
    }
}
impl AsRef<syn::File> for SynFile {
    fn as_ref(&self) -> &syn::File {
        &self.0
    }
}
#[derive(Debug, Clone)]
pub(super) struct TomlTable(toml::Table);
impl From<toml::Table> for TomlTable {
    fn from(value: toml::Table) -> Self {
        Self(value)
    }
}
#[derive(Debug, Clone, Copy)]
pub(super) struct TomlTableRef<'toml_lt>(&'toml_lt toml::value::Table);
impl<'toml_lt> From<&'toml_lt toml::value::Table> for TomlTableRef<'toml_lt> {
    fn from(value: &'toml_lt toml::value::Table) -> Self {
        Self(value)
    }
}
impl AsRef<toml::value::Table> for TomlTableRef<'_> {
    fn as_ref(&self) -> &toml::value::Table {
        self.0
    }
}
impl<'toml_lt> TomlTableRef<'toml_lt> {
    pub(super) const fn get(self) -> &'toml_lt toml::value::Table {
        self.0
    }
}
#[derive(Debug, Clone, Copy)]
pub(super) struct TomlValueRef<'toml_lt>(&'toml_lt toml::Value);
impl<'toml_lt> From<&'toml_lt toml::Value> for TomlValueRef<'toml_lt> {
    fn from(value: &'toml_lt toml::Value) -> Self {
        Self(value)
    }
}
impl AsRef<toml::Value> for TomlValueRef<'_> {
    fn as_ref(&self) -> &toml::Value {
        self.0
    }
}
impl<'toml_lt> TomlValueRef<'toml_lt> {
    pub(super) const fn get(self) -> &'toml_lt toml::Value {
        self.0
    }
}
impl AsRef<toml::Table> for TomlTable {
    fn as_ref(&self) -> &toml::Table {
        &self.0
    }
}
impl From<TomlTable> for toml::Table {
    fn from(value: TomlTable) -> Self {
        value.0
    }
}
#[derive(Debug, Clone)]
pub(super) struct TomlValue(toml::Value);
impl From<toml::Value> for TomlValue {
    fn from(value: toml::Value) -> Self {
        Self(value)
    }
}
impl TomlValue {
    pub(super) fn into_inner(self) -> toml::Value {
        self.0
    }
}
pub(super) struct WalkdirWalkDir(walkdir::WalkDir);
impl From<walkdir::WalkDir> for WalkdirWalkDir {
    fn from(value: walkdir::WalkDir) -> Self {
        Self(value)
    }
}
impl IntoIterator for WalkdirWalkDir {
    type IntoIter = walkdir::IntoIter;
    type Item = walkdir::Result<walkdir::DirEntry>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
