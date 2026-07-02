#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Loc {
    column: SourceColumn,
    commit_id: git_info::ProjectGitCommitId,
    file: SourceFilePath,
    line: SourceLine,
    occurrence: Occurrence,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Occr {
    column: SourceColumn,
    file: SourceFilePath,
    line: SourceLine,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Occurrence(Option<Occr>);

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct SourceColumn;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct SourceFilePath;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct SourceLine;

impl Loc {
    #[must_use]
    pub const fn new(
        file: SourceFilePath,
        line: SourceLine,
        column: SourceColumn,
        occurrence: Occurrence,
    ) -> Self {
        Self {
            column,
            commit_id: git_info::project_git_commit_id(),
            file,
            line,
            occurrence,
        }
    }
}

impl Occr {
    #[must_use]
    pub const fn new(file: SourceFilePath, line: SourceLine, column: SourceColumn) -> Self {
        Self { column, file, line }
    }
}

impl Occurrence {
    #[must_use]
    pub const fn none() -> Self {
        Self(None)
    }

    #[must_use]
    pub const fn some(occurrence: Occr) -> Self {
        Self(Some(occurrence))
    }
}

impl SourceColumn {
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

impl SourceFilePath {
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

impl SourceLine {
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

impl core::fmt::Display for Loc {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::write!(
            f,
            "{}{}{}{}{}{}{}{}{}",
            naming_constants::GITHUB_URL,
            naming_constants::GIT_BLOB_SEGMENT,
            self.commit_id.as_ref(),
            naming_constants::CHARACTER_SLASH,
            self.file,
            naming_constants::GIT_LINE_FRAGMENT_PREFIX,
            self.line,
            naming_constants::SOURCE_COLUMN_SEPARATOR,
            self.column
        )?;
        self.occurrence.0.map_or(Ok(()), |occurrence| {
            core::write!(
                f,
                "{}{}{}{}{}{}",
                naming_constants::LOCATION_OCCURRENCE_PREFIX,
                occurrence.file,
                naming_constants::SOURCE_LINE_SEPARATOR,
                occurrence.line,
                naming_constants::SOURCE_COLUMN_SEPARATOR,
                occurrence.column
            )
        })
    }
}

impl core::fmt::Display for SourceColumn {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(naming_constants::SOURCE_COLUMN_UNKNOWN)
    }
}

impl core::fmt::Display for SourceFilePath {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(naming_constants::SOURCE_FILE_UNKNOWN)
    }
}

impl core::fmt::Display for SourceLine {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(naming_constants::SOURCE_LINE_UNKNOWN)
    }
}
