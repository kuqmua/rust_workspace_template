#[derive(optimal_memory_layout::OptimalMemoryLayout)]
#[allow(
    dead_code,
    non_snake_case,
    clippy::field_scoped_visibility_modifiers,
    reason = "the immutable token-name context is shared by sibling generation modules without becoming public API"
)]
#[derive(generate_accessor::Getters)]
pub(crate) struct NamesCtx {
    AddOperatorSnakeCase: naming::domain_types::AddOperatorSnakeCase,
    AllVariantsDefaultSomeOneElementMaxPageSizeSnakeCase:
        naming::domain_types::AllVariantsDefaultSomeOneElementMaxPageSizeSnakeCase,
    AllVariantsDefaultSomeOneElementSnakeCase:
        naming::domain_types::AllVariantsDefaultSomeOneElementSnakeCase,
    AllowClippyArbitrarySrcItemOrdering: token_patterns::AllowClippyArbitrarySrcItemOrdering,
    ColumnSnakeCase: naming::domain_types::ColumnSnakeCase,
    CreateQueryBindSnakeCase: naming::domain_types::CreateQueryBindSnakeCase,
    CreateQueryPartSnakeCase: naming::domain_types::CreateQueryPartSnakeCase,
    CreateSnakeCase: naming::domain_types::CreateSnakeCase,
    CreateTableColumnQueryPartSnakeCase: naming::domain_types::CreateTableColumnQueryPartSnakeCase,
    CreateUpperCamelCase: naming::domain_types::CreateUpperCamelCase,
    DefaultSomeOneElementMaxPageSizeSnakeCase:
        naming::domain_types::DefaultSomeOneElementMaxPageSizeSnakeCase,
    DefaultSomeOneElementSnakeCase: naming::domain_types::DefaultSomeOneElementSnakeCase,
    EqOperatorUpperCamelCase: naming::domain_types::EqOperatorUpperCamelCase,
    ErrorSnakeCase: naming::domain_types::ErrorSnakeCase,
    IncrementSnakeCase: naming::domain_types::IncrementSnakeCase,
    NormalizeSnakeCase: naming::domain_types::NormalizeSnakeCase,
    OptionalUpdateSnakeCase: naming::domain_types::OptionalUpdateSnakeCase,
    OptionalVecCreateSnakeCase: naming::domain_types::OptionalVecCreateSnakeCase,
    PgCrudCommonDefaultSomeOneElementCall: token_patterns::PgCrudCommonDefaultSomeOneElementCall,
    PgTypeEqOperatorUpperCamelCase: naming::domain_types::PgTypeEqOperatorUpperCamelCase,
    PgTypeNotPrimaryKeyUpperCamelCase: naming::domain_types::PgTypeNotPrimaryKeyUpperCamelCase,
    PgTypeOptionalVecWhereGreaterThanTestSnakeCase:
        naming::domain_types::PgTypeOptionalVecWhereGreaterThanTestSnakeCase,
    PgTypeTestCasesUpperCamelCase: naming::domain_types::PgTypeTestCasesUpperCamelCase,
    PgTypeUpperCamelCase: naming::domain_types::PgTypeUpperCamelCase,
    PgTypeWhereFilterUpperCamelCase: naming::domain_types::PgTypeWhereFilterUpperCamelCase,
    PreviousReadAndOptionalUpdateIntoReadSnakeCase:
        naming::domain_types::PreviousReadAndOptionalUpdateIntoReadSnakeCase,
    QueryBindSnakeCase: naming::domain_types::QueryBindSnakeCase,
    QueryPartErrorUpperCamelCase: naming::domain_types::QueryPartErrorUpperCamelCase,
    QueryPartSnakeCase: naming::domain_types::QueryPartSnakeCase,
    QuerySnakeCase: naming::domain_types::QuerySnakeCase,
    ReadIdsAndCreateIntoOptionalVReadSnakeCase:
        naming::domain_types::ReadIdsAndCreateIntoOptionalVReadSnakeCase,
    ReadIdsAndCreateIntoOptionalVecWhereEqToFieldSnakeCase:
        naming::domain_types::ReadIdsAndCreateIntoOptionalVecWhereEqToFieldSnakeCase,
    ReadIdsAndCreateIntoReadSnakeCase: naming::domain_types::ReadIdsAndCreateIntoReadSnakeCase,
    ReadIdsAndCreateIntoTableTypeSnakeCase:
        naming::domain_types::ReadIdsAndCreateIntoTableTypeSnakeCase,
    ReadIdsAndCreateIntoVecWhereEqUsingFieldsSnakeCase:
        naming::domain_types::ReadIdsAndCreateIntoVecWhereEqUsingFieldsSnakeCase,
    ReadIdsAndCreateIntoWhereEqSnakeCase:
        naming::domain_types::ReadIdsAndCreateIntoWhereEqSnakeCase,
    ReadIdsAndTableTypeIntoPgTypeOptionalWhereGreaterThanSnakeCase:
        naming::domain_types::ReadIdsAndTableTypeIntoPgTypeOptionalWhereGreaterThanSnakeCase,
    ReadIdsSnakeCase: naming::domain_types::ReadIdsSnakeCase,
    ReadIdsTo2DimensionsVecReadInnerSnakeCase:
        naming::domain_types::ReadIdsTo2DimensionsVecReadInnerSnakeCase,
    ReadIdsToOptionalVReadDefaultSomeOneElementSnakeCase:
        naming::domain_types::ReadIdsToOptionalVReadDefaultSomeOneElementSnakeCase,
    ReadIdsUpperCamelCase: naming::domain_types::ReadIdsUpperCamelCase,
    ReadInnerIntoReadWithNewOrTryNewUnwrapedSnakeCase:
        naming::domain_types::ReadInnerIntoReadWithNewOrTryNewUnwrapedSnakeCase,
    ReadInnerIntoUpdateWithNewOrTryNewUnwrapedSnakeCase:
        naming::domain_types::ReadInnerIntoUpdateWithNewOrTryNewUnwrapedSnakeCase,
    ReadInnerUpperCamelCase: naming::domain_types::ReadInnerUpperCamelCase,
    ReadSnakeCase: naming::domain_types::ReadSnakeCase,
    ReadUpperCamelCase: naming::domain_types::ReadUpperCamelCase,
    SelectOnlyIdsQueryPartSnakeCase: naming::domain_types::SelectOnlyIdsQueryPartSnakeCase,
    SelectOnlyUpdatedIdsQueryBindSnakeCase:
        naming::domain_types::SelectOnlyUpdatedIdsQueryBindSnakeCase,
    SelectOnlyUpdatedIdsQueryPartSnakeCase:
        naming::domain_types::SelectOnlyUpdatedIdsQueryPartSnakeCase,
    SelectQueryPartSnakeCase: naming::domain_types::SelectQueryPartSnakeCase,
    SelectUpperCamelCase: naming::domain_types::SelectUpperCamelCase,
    SelfUpperCamelCase: naming::domain_types::SelfUpperCamelCase,
    TableTypeSnakeCase: naming::domain_types::TableTypeSnakeCase,
    TableTypeUpperCamelCase: naming::domain_types::TableTypeUpperCamelCase,
    UpdateForQueryUpperCamelCase: naming::domain_types::UpdateForQueryUpperCamelCase,
    UpdateQueryBindSnakeCase: naming::domain_types::UpdateQueryBindSnakeCase,
    UpdateQueryPartSnakeCase: naming::domain_types::UpdateQueryPartSnakeCase,
    UpdateToReadIdsSnakeCase: naming::domain_types::UpdateToReadIdsSnakeCase,
    UpdateUpperCamelCase: naming::domain_types::UpdateUpperCamelCase,
    VSnakeCase: naming::domain_types::VSnakeCase,
    VUpperCamelCase: naming::domain_types::VUpperCamelCase,
    ValueSnakeCase: naming::domain_types::ValueSnakeCase,
    WhereUpperCamelCase: naming::domain_types::WhereUpperCamelCase,
}
#[allow(
    dead_code,
    reason = "field access is intentionally encapsulated behind uniform getters"
)]
impl NamesCtx {
    pub(crate) const fn new() -> Self {
        Self {
            AddOperatorSnakeCase: naming::domain_types::AddOperatorSnakeCase,
            AllVariantsDefaultSomeOneElementMaxPageSizeSnakeCase:
                naming::domain_types::AllVariantsDefaultSomeOneElementMaxPageSizeSnakeCase,
            AllVariantsDefaultSomeOneElementSnakeCase:
                naming::domain_types::AllVariantsDefaultSomeOneElementSnakeCase,
            AllowClippyArbitrarySrcItemOrdering:
                token_patterns::AllowClippyArbitrarySrcItemOrdering,
            ColumnSnakeCase: naming::domain_types::ColumnSnakeCase,
            CreateQueryBindSnakeCase: naming::domain_types::CreateQueryBindSnakeCase,
            CreateQueryPartSnakeCase: naming::domain_types::CreateQueryPartSnakeCase,
            CreateSnakeCase: naming::domain_types::CreateSnakeCase,
            CreateTableColumnQueryPartSnakeCase:
                naming::domain_types::CreateTableColumnQueryPartSnakeCase,
            CreateUpperCamelCase: naming::domain_types::CreateUpperCamelCase,
            DefaultSomeOneElementMaxPageSizeSnakeCase:
                naming::domain_types::DefaultSomeOneElementMaxPageSizeSnakeCase,
            DefaultSomeOneElementSnakeCase: naming::domain_types::DefaultSomeOneElementSnakeCase,
            EqOperatorUpperCamelCase: naming::domain_types::EqOperatorUpperCamelCase,
            ErrorSnakeCase: naming::domain_types::ErrorSnakeCase,
            IncrementSnakeCase: naming::domain_types::IncrementSnakeCase,
            NormalizeSnakeCase: naming::domain_types::NormalizeSnakeCase,
            OptionalUpdateSnakeCase: naming::domain_types::OptionalUpdateSnakeCase,
            OptionalVecCreateSnakeCase: naming::domain_types::OptionalVecCreateSnakeCase,
            PgCrudCommonDefaultSomeOneElementCall:
                token_patterns::PgCrudCommonDefaultSomeOneElementCall,
            PgTypeEqOperatorUpperCamelCase: naming::domain_types::PgTypeEqOperatorUpperCamelCase,
            PgTypeNotPrimaryKeyUpperCamelCase:
                naming::domain_types::PgTypeNotPrimaryKeyUpperCamelCase,
            PgTypeOptionalVecWhereGreaterThanTestSnakeCase:
                naming::domain_types::PgTypeOptionalVecWhereGreaterThanTestSnakeCase,
            PgTypeTestCasesUpperCamelCase: naming::domain_types::PgTypeTestCasesUpperCamelCase,
            PgTypeUpperCamelCase: naming::domain_types::PgTypeUpperCamelCase,
            PgTypeWhereFilterUpperCamelCase: naming::domain_types::PgTypeWhereFilterUpperCamelCase,
            PreviousReadAndOptionalUpdateIntoReadSnakeCase:
                naming::domain_types::PreviousReadAndOptionalUpdateIntoReadSnakeCase,
            QueryBindSnakeCase: naming::domain_types::QueryBindSnakeCase,
            QueryPartErrorUpperCamelCase: naming::domain_types::QueryPartErrorUpperCamelCase,
            QueryPartSnakeCase: naming::domain_types::QueryPartSnakeCase,
            QuerySnakeCase: naming::domain_types::QuerySnakeCase,
            ReadIdsAndCreateIntoOptionalVReadSnakeCase:
                naming::domain_types::ReadIdsAndCreateIntoOptionalVReadSnakeCase,
            ReadIdsAndCreateIntoOptionalVecWhereEqToFieldSnakeCase:
                naming::domain_types::ReadIdsAndCreateIntoOptionalVecWhereEqToFieldSnakeCase,
            ReadIdsAndCreateIntoReadSnakeCase:
                naming::domain_types::ReadIdsAndCreateIntoReadSnakeCase,
            ReadIdsAndCreateIntoTableTypeSnakeCase:
                naming::domain_types::ReadIdsAndCreateIntoTableTypeSnakeCase,
            ReadIdsAndCreateIntoVecWhereEqUsingFieldsSnakeCase:
                naming::domain_types::ReadIdsAndCreateIntoVecWhereEqUsingFieldsSnakeCase,
            ReadIdsAndCreateIntoWhereEqSnakeCase:
                naming::domain_types::ReadIdsAndCreateIntoWhereEqSnakeCase,
            ReadIdsAndTableTypeIntoPgTypeOptionalWhereGreaterThanSnakeCase:
                naming::domain_types::ReadIdsAndTableTypeIntoPgTypeOptionalWhereGreaterThanSnakeCase,
            ReadIdsSnakeCase: naming::domain_types::ReadIdsSnakeCase,
            ReadIdsTo2DimensionsVecReadInnerSnakeCase:
                naming::domain_types::ReadIdsTo2DimensionsVecReadInnerSnakeCase,
            ReadIdsToOptionalVReadDefaultSomeOneElementSnakeCase:
                naming::domain_types::ReadIdsToOptionalVReadDefaultSomeOneElementSnakeCase,
            ReadIdsUpperCamelCase: naming::domain_types::ReadIdsUpperCamelCase,
            ReadInnerIntoReadWithNewOrTryNewUnwrapedSnakeCase:
                naming::domain_types::ReadInnerIntoReadWithNewOrTryNewUnwrapedSnakeCase,
            ReadInnerIntoUpdateWithNewOrTryNewUnwrapedSnakeCase:
                naming::domain_types::ReadInnerIntoUpdateWithNewOrTryNewUnwrapedSnakeCase,
            ReadInnerUpperCamelCase: naming::domain_types::ReadInnerUpperCamelCase,
            ReadSnakeCase: naming::domain_types::ReadSnakeCase,
            ReadUpperCamelCase: naming::domain_types::ReadUpperCamelCase,
            SelectOnlyIdsQueryPartSnakeCase: naming::domain_types::SelectOnlyIdsQueryPartSnakeCase,
            SelectOnlyUpdatedIdsQueryBindSnakeCase:
                naming::domain_types::SelectOnlyUpdatedIdsQueryBindSnakeCase,
            SelectOnlyUpdatedIdsQueryPartSnakeCase:
                naming::domain_types::SelectOnlyUpdatedIdsQueryPartSnakeCase,
            SelectQueryPartSnakeCase: naming::domain_types::SelectQueryPartSnakeCase,
            SelectUpperCamelCase: naming::domain_types::SelectUpperCamelCase,
            SelfUpperCamelCase: naming::domain_types::SelfUpperCamelCase,
            TableTypeSnakeCase: naming::domain_types::TableTypeSnakeCase,
            TableTypeUpperCamelCase: naming::domain_types::TableTypeUpperCamelCase,
            UpdateForQueryUpperCamelCase: naming::domain_types::UpdateForQueryUpperCamelCase,
            UpdateQueryBindSnakeCase: naming::domain_types::UpdateQueryBindSnakeCase,
            UpdateQueryPartSnakeCase: naming::domain_types::UpdateQueryPartSnakeCase,
            UpdateToReadIdsSnakeCase: naming::domain_types::UpdateToReadIdsSnakeCase,
            UpdateUpperCamelCase: naming::domain_types::UpdateUpperCamelCase,
            VSnakeCase: naming::domain_types::VSnakeCase,
            VUpperCamelCase: naming::domain_types::VUpperCamelCase,
            ValueSnakeCase: naming::domain_types::ValueSnakeCase,
            WhereUpperCamelCase: naming::domain_types::WhereUpperCamelCase,
        }
    }
}
