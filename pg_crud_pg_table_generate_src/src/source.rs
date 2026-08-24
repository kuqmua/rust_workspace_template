#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, Clone, Copy, newtype::FromInner)]
struct CompileErrorMessage<'message_lt>(&'message_lt str);

impl<'message_lt> From<&'message_lt String> for CompileErrorMessage<'message_lt> {
    fn from(value: &'message_lt String) -> Self {
        Self(value.as_str())
    }
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, newtype::FromInner)]
struct TableTestNames<'value_lt>(Vec<&'value_lt str>);

fn compile_error_token_stream(
    message: CompileErrorMessage<'_>,
) -> macros_helpers::proc_macro2_tokens::ProcMacro2GeneratedRustTokenStream {
    let message_value = message.0;
    macros_helpers::proc_macro2_tokens::ProcMacro2GeneratedRustTokenStream::from(
        quote::quote! {compile_error!(#message_value);},
    )
}
//todo decide where to do error log (maybe add in some places)
//todo generate route what will return cols of the table and their rust and postgersql types
//todo created at and updated at fields + created by + updated by
//todo attrs for activation generation crud methods(like generate create, uo, dlo)
//todo authorization for returning concrete error or just minimal info(user role)
//todo generate rules and roles
//todo maybe add unnest sql types?
//todo maybe add unnest to filter params if its array ?
//todo swagger ui https://github.com/juhaku/utoipa/blob/master/examples/todo-axum/src/main.rs
//todo derive utoipa::ToSchema for what? original structs or with serialize deserialize?
//todo need to add utoipa::ToSchema ann #[schema(value_type = YourToSchemaTraitImplStruct)] for all fields
//todo remove useless derives like useless serde::Serialize and serde::Deserialize
//todo maybe generate compisite type for user defined type https://docs.rs/sqlx/0.7.3/sqlx/pg/types/index.html#rust_decimal
//todo read again some interesting thoughts about sql as api https://habr.com/ru/companies/timeweb/articles/798937/
//todo reexport all crates what logic depends on (from crates.io) (use of undclared crate or module `time`)
//todo add transaction isolation level (see pg docs)
//todo check on pg max len value of type
//todo in few cases rows affected is usefull. (update delete for example). if 0 afftected -maybe its error? or maybe use select then update\delete?(rewrite query)
//todo generate json schema from rust type https://docs.rs/schemars/laTest/schemars/
//todo support read table len
//todo what is pub what is private
#[must_use]
#[allow(non_snake_case)]
#[allow(unused_variables)]
pub fn generate_pg_table(
    input: macros_helpers::ts_writer::ProcMacro2TokenStreamRef<'_>,
) -> macros_helpers::proc_macro2_tokens::ProcMacro2GeneratedRustTokenStream {
    let validated = match crate::pipeline::parse_generate_pg_table(input)
        .and_then(crate::pipeline::build_generate_pg_table)
        .and_then(crate::pipeline::validate_generate_pg_table)
    {
        Ok(validated) => validated,
        Err(error) => {
            return macros_helpers::proc_macro2_tokens::ProcMacro2GeneratedRustTokenStream::from(
                match error {
                    crate::pipeline::GeneratePgTablePipelineError::Build(pipeline_error)
                    | crate::pipeline::GeneratePgTablePipelineError::Parse(pipeline_error)
                    | crate::pipeline::GeneratePgTablePipelineError::Validate(pipeline_error) => {
                        syn::Error::from(pipeline_error).to_compile_error()
                    }
                },
            );
        }
    };
    emit_generate_pg_table(validated)
}

#[must_use]
#[allow(non_snake_case)]
#[allow(unused_variables)]
pub fn emit_generate_pg_table(
    validated: crate::pipeline::SynValidatedGeneratePgTableInput,
) -> macros_helpers::proc_macro2_tokens::ProcMacro2GeneratedRustTokenStream {
    #[allow(clippy::arbitrary_source_item_ordering)]
    #[derive(Debug, optimal_memory_layout::OptimalMemoryLayout)]
    struct SynVariant {
        variant: syn::Variant,
        status_code: Option<macros_helpers::status_code::StatusCode>,
    }
    impl SynVariant {
        const fn get_optional_status_code(
            &self,
        ) -> Option<&macros_helpers::status_code::StatusCode> {
            self.status_code.as_ref()
        }
        const fn get_syn_variant(&self) -> &syn::Variant {
            &self.variant
        }
    }
    #[derive(optimal_memory_layout::OptimalMemoryLayout)]
    enum AddBorrow {
        False,
        True,
    }
    impl quote::ToTokens for AddBorrow {
        fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
            match &self {
                Self::False => proc_macro2::TokenStream::new().to_tokens(tokens),
                Self::True => quote::quote! {&}.to_tokens(tokens),
            }
        }
    }
    #[derive(optimal_memory_layout::OptimalMemoryLayout)]
    enum AddReturn {
        False,
        True,
    }
    #[allow(clippy::arbitrary_source_item_ordering)]
    #[derive(
        Debug,
        Clone,
        Copy,
        naming_macros::AsRefStrEnumWithUnitFieldsToUpperCamelCaseStr,
        naming_macros::AsRefStrEnumWithUnitFieldsToSnakeCaseStr,
        optimal_memory_layout::OptimalMemoryLayout,
    )]
    enum Operation {
        Cm,
        Co,
        Rm,
        Ro,
        Um,
        Uo,
        Dm,
        Dlo,
    }
    impl Operation {
        const fn derive_clone_and_copy(
            self,
        ) -> (
            macros_helpers::derive_token_stream_builder::DClone,
            macros_helpers::derive_token_stream_builder::DCopy,
        ) {
            match self {
                Self::Cm | Self::Co | Self::Rm | Self::Ro | Self::Um | Self::Uo | Self::Dm => (
                    macros_helpers::derive_token_stream_builder::DClone::False,
                    macros_helpers::derive_token_stream_builder::DCopy::False,
                ),
                Self::Dlo => (
                    macros_helpers::derive_token_stream_builder::DClone::True,
                    macros_helpers::derive_token_stream_builder::DCopy::False,
                ),
            }
        }
        const fn derive_parameters_clone_and_copy(
            self,
        ) -> (
            macros_helpers::derive_token_stream_builder::DClone,
            macros_helpers::derive_token_stream_builder::DCopy,
        ) {
            match self {
                Self::Co | Self::Dlo => (
                    macros_helpers::derive_token_stream_builder::DClone::True,
                    macros_helpers::derive_token_stream_builder::DCopy::False,
                ),
                Self::Cm | Self::Rm | Self::Ro | Self::Um | Self::Uo | Self::Dm => (
                    macros_helpers::derive_token_stream_builder::DClone::False,
                    macros_helpers::derive_token_stream_builder::DCopy::False,
                ),
            }
        }
        const fn desirable_status_code(self) -> macros_helpers::status_code::StatusCode {
            match self {
                Self::Cm | Self::Co => macros_helpers::status_code::StatusCode::Created201,
                Self::Rm | Self::Ro | Self::Um | Self::Uo | Self::Dm | Self::Dlo => {
                    macros_helpers::status_code::StatusCode::Ok200
                }
            }
        }
        const fn generate_pg_table_attr_error_variants(self) -> GeneratePgTableAttr {
            match self {
                Self::Cm => GeneratePgTableAttr::CmErrorVariants,
                Self::Co => GeneratePgTableAttr::CoErrorVariants,
                Self::Rm => GeneratePgTableAttr::RmErrorVariants,
                Self::Ro => GeneratePgTableAttr::RoErrorVariants,
                Self::Um => GeneratePgTableAttr::UmErrorVariants,
                Self::Uo => GeneratePgTableAttr::UoErrorVariants,
                Self::Dm => GeneratePgTableAttr::DmErrorVariants,
                Self::Dlo => GeneratePgTableAttr::DloErrorVariants,
            }
        }
        const fn generate_pg_table_attr_logic(self) -> GeneratePgTableAttr {
            match self {
                Self::Cm => GeneratePgTableAttr::CmLogic,
                Self::Co => GeneratePgTableAttr::CoLogic,
                Self::Rm => GeneratePgTableAttr::RmLogic,
                Self::Ro => GeneratePgTableAttr::RoLogic,
                Self::Um => GeneratePgTableAttr::UmLogic,
                Self::Uo => GeneratePgTableAttr::UoLogic,
                Self::Dm => GeneratePgTableAttr::DmLogic,
                Self::Dlo => GeneratePgTableAttr::DloLogic,
            }
        }
        const fn http_method(self) -> OperationHttpMethod {
            match self {
                Self::Cm | Self::Co | Self::Rm | Self::Ro => OperationHttpMethod::Post,
                Self::Um | Self::Uo => OperationHttpMethod::Patch,
                Self::Dm | Self::Dlo => OperationHttpMethod::Delete,
            }
        }
        fn operation_error_with_serde_snake_case(
            self,
        ) -> naming::parameter::SelfErrorWithSerdeSnakeCase {
            naming::parameter::SelfErrorWithSerdeSnakeCase::from_display(&self)
        }
        fn operation_payload_example_snake_case(self) -> impl naming::DisplayPlusToTokens {
            naming::parameter::SelfPayloadExampleSnakeCase::from_display(&self)
        }
        fn self_handle_snake_case_token_stream(self) -> proc_macro2::TokenStream {
            let v = naming::parameter::SelfHSnakeCase::from_tokens(
                &self.self_snake_case_token_stream(),
            );
            quote::quote! {#v}
        }
        fn self_snake_case_str(self) -> String {
            naming_common::AsRefStrToSnakeCaseStr::case(&self)
        }
        fn self_snake_case_token_stream(self) -> proc_macro2::TokenStream {
            let identifier = quote::format_ident!("{}", self.self_snake_case_str());
            quote::quote! {#identifier}
        }
        const fn supports_idempotency(self) -> bool {
            matches!(
                self,
                Self::Cm | Self::Co | Self::Um | Self::Uo | Self::Dm | Self::Dlo
            )
        }
        const fn supports_optimistic_concurrency(self) -> bool {
            matches!(self, Self::Uo)
        }
        fn try_self_handle_snake_case_token_stream(self) -> proc_macro2::TokenStream {
            let v = naming::parameter::TrySelfHSnakeCase::from_tokens(
                &self.self_snake_case_token_stream(),
            );
            quote::quote! {#v}
        }
        fn try_self_snake_case_token_stream(self) -> proc_macro2::TokenStream {
            let v = naming::parameter::TrySelfSnakeCase::from_tokens(
                &self.self_snake_case_token_stream(),
            );
            quote::quote! {#v}
        }
    }
    impl std::fmt::Display for Operation {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "{}",
                match &self {
                    Self::Cm => "Cm",
                    Self::Co => "Co",
                    Self::Rm => "Rm",
                    Self::Ro => "Ro",
                    Self::Um => "Um",
                    Self::Uo => "Uo",
                    Self::Dm => "Dm",
                    Self::Dlo => "Dlo",
                }
            )
        }
    }
    impl From<&CreateOrUpdateOrDm> for Operation {
        fn from(v: &CreateOrUpdateOrDm) -> Self {
            match &v {
                CreateOrUpdateOrDm::Create => Self::Cm,
                CreateOrUpdateOrDm::Update => Self::Um,
                CreateOrUpdateOrDm::Delete => Self::Dm,
            }
        }
    }
    impl From<&RmOrDm> for Operation {
        fn from(v: &RmOrDm) -> Self {
            match &v {
                RmOrDm::Rm => Self::Rm,
                RmOrDm::Dm => Self::Dm,
            }
        }
    }
    impl From<&RmOrRo> for Operation {
        fn from(v: &RmOrRo) -> Self {
            match &v {
                RmOrRo::Rm => Self::Rm,
                RmOrRo::Ro => Self::Ro,
            }
        }
    }
    impl From<&CreateOrUpdateOrDlo> for Operation {
        fn from(v: &CreateOrUpdateOrDlo) -> Self {
            match &v {
                CreateOrUpdateOrDlo::Create => Self::Co,
                CreateOrUpdateOrDlo::Update => Self::Uo,
                CreateOrUpdateOrDlo::Delete => Self::Dlo,
            }
        }
    }
    #[allow(clippy::arbitrary_source_item_ordering)]
    #[derive(
        Clone,
        Copy,
        naming_macros::AsRefStrEnumWithUnitFieldsToSnakeCaseStr,
        optimal_memory_layout::OptimalMemoryLayout,
    )]
    enum OperationHttpMethod {
        Post,
        Patch,
        Delete,
    }
    #[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy)]
    enum OperationKind {
        CreateMany,
        CreateOne,
        DeleteMany,
        DeleteOne,
        ReadMany,
        ReadOne,
        UpdateMany,
        UpdateOne,
    }
    impl
        crate::model::OperationDsc<
            bool,
            OperationHttpMethod,
            Operation,
            OperationKind,
            &'static str,
            macros_helpers::status_code::StatusCode,
        >
    {
        const ALL: [Self; 8] = [
            Self::from_operation(Operation::Cm),
            Self::from_operation(Operation::Co),
            Self::from_operation(Operation::Rm),
            Self::from_operation(Operation::Ro),
            Self::from_operation(Operation::Um),
            Self::from_operation(Operation::Uo),
            Self::from_operation(Operation::Dm),
            Self::from_operation(Operation::Dlo),
        ];
        const fn from_operation(operation: Operation) -> Self {
            Self {
                http_method: operation.http_method(),
                idempotency_capable: operation.supports_idempotency(),
                operation_kind: match operation {
                    Operation::Cm => OperationKind::CreateMany,
                    Operation::Co => OperationKind::CreateOne,
                    Operation::Dm => OperationKind::DeleteMany,
                    Operation::Dlo => OperationKind::DeleteOne,
                    Operation::Rm => OperationKind::ReadMany,
                    Operation::Ro => OperationKind::ReadOne,
                    Operation::Um => OperationKind::UpdateMany,
                    Operation::Uo => OperationKind::UpdateOne,
                },
                optimistic_concurrency_capable: operation.supports_optimistic_concurrency(),
                permission_action: match operation {
                    Operation::Cm | Operation::Co => {
                        str_constants::PG_CRUD_CREATE_PERMISSION_ACTION
                    }
                    Operation::Rm | Operation::Ro => str_constants::PG_CRUD_READ_PERMISSION_ACTION,
                    Operation::Um | Operation::Uo => {
                        str_constants::PG_CRUD_UPDATE_PERMISSION_ACTION
                    }
                    Operation::Dm | Operation::Dlo => {
                        str_constants::PG_CRUD_DELETE_PERMISSION_ACTION
                    }
                },
                success_status_code: operation.desirable_status_code(),
                operation,
            }
        }
    }
    #[derive(optimal_memory_layout::OptimalMemoryLayout)]
    #[allow(clippy::arbitrary_source_item_ordering)]
    enum RmOrDm {
        Rm,
        Dm,
    }
    #[derive(optimal_memory_layout::OptimalMemoryLayout)]
    enum RmOrRo {
        Rm,
        Ro,
    }
    #[allow(clippy::arbitrary_source_item_ordering)]
    #[derive(
        Debug,
        Clone,
        Copy,
        PartialEq,
        Eq,
        PartialOrd,
        Ord,
        strum_macros::Display,
        optimal_memory_layout::OptimalMemoryLayout,
    )]
    enum GeneratePgTableAttr {
        CmErrorVariants,
        CoErrorVariants,
        RmErrorVariants,
        RoErrorVariants,
        UmErrorVariants,
        UoErrorVariants,
        DmErrorVariants,
        DloErrorVariants,
        CommonErrorVariants,
        CmLogic,
        CoLogic,
        RmLogic,
        RoLogic,
        UmLogic,
        UoLogic,
        DmLogic,
        DloLogic,
        CommonLogic,
    }
    impl GeneratePgTableAttr {
        fn generate_path_to_attr(self) -> String {
            let attr_name: &dyn std::fmt::Display = match self {
                Self::CmErrorVariants => &naming::CmErrorVariantsSnakeCase,
                Self::CoErrorVariants => &naming::CoErrorVariantsSnakeCase,
                Self::RmErrorVariants => &naming::RmErrorVariantsSnakeCase,
                Self::RoErrorVariants => &naming::RoErrorVariantsSnakeCase,
                Self::UmErrorVariants => &naming::UmErrorVariantsSnakeCase,
                Self::UoErrorVariants => &naming::UoErrorVariantsSnakeCase,
                Self::DmErrorVariants => &naming::DmErrorVariantsSnakeCase,
                Self::DloErrorVariants => &naming::DloErrorVariantsSnakeCase,
                Self::CommonErrorVariants => &naming::CommonErrorVariantsSnakeCase,
                Self::CmLogic => &naming::CmLogicSnakeCase,
                Self::CoLogic => &naming::CoLogicSnakeCase,
                Self::RmLogic => &naming::RmLogicSnakeCase,
                Self::RoLogic => &naming::RoLogicSnakeCase,
                Self::UmLogic => &naming::UmLogicSnakeCase,
                Self::UoLogic => &naming::UoLogicSnakeCase,
                Self::DmLogic => &naming::DmLogicSnakeCase,
                Self::DloLogic => &naming::DloLogicSnakeCase,
                Self::CommonLogic => &naming::CommonLogicSnakeCase,
            };
            format!("generate_pg_table::{attr_name}")
        }
    }
    #[derive(optimal_memory_layout::OptimalMemoryLayout)]
    enum ShouldWrapIntoV {
        False,
        True,
    }
    #[derive(optimal_memory_layout::OptimalMemoryLayout)]
    #[allow(clippy::arbitrary_source_item_ordering)]
    enum CreateOrUpdateOrDm {
        Create,
        Update,
        Delete,
    }
    #[derive(optimal_memory_layout::OptimalMemoryLayout)]
    #[allow(clippy::arbitrary_source_item_ordering)]
    enum CreateOrUpdateOrDlo {
        Create,
        Update,
        Delete,
    }
    #[allow(clippy::arbitrary_source_item_ordering)]
    #[derive(Debug, serde::Deserialize, optimal_memory_layout::OptimalMemoryLayout)]
    struct GeneratePgTableConfig {
        #[serde(default)]
        cm_max_items: Option<StdBulkItemsMax>,
        #[serde(default)]
        create_exclude_fields: Option<UsizeCreateExcludeFields>,
        #[serde(default)]
        db_foreign_keys: Vec<GeneratePgTableDbForeignKey>,
        #[serde(default)]
        db_table_name: Option<String>,
        #[serde(default)]
        db_unique_keys: Vec<Vec<String>>,
        read_exclude_fields: Option<UsizeReadExcludeFields>,
        #[serde(default)]
        permission_prefix: Option<String>,
        #[serde(default)]
        um_max_items: Option<StdBulkItemsMax>,
        #[serde(default)]
        optimistic_revision_field: Option<String>,
        tests_write_into_file: macros_helpers::ts_writer::ShouldWriteTokenStreamIntoFile,
        common_write_into_file: macros_helpers::ts_writer::ShouldWriteTokenStreamIntoFile,
        whole_write_into_file: macros_helpers::ts_writer::ShouldWriteTokenStreamIntoFile,
        #[serde(default)]
        idempotent_mutations: bool,
        #[serde(default)]
        api_mode: GeneratePgTableApiMode,
    }
    #[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, serde::Deserialize)]
    struct GeneratePgTableDbForeignKey {
        columns: UsizeGeneratePgTableDbColumns,
        referenced_columns: UsizeGeneratePgTableDbColumns,
        referenced_table: String,
    }
    const GENERATE_PG_TABLE_MAX_IDENTIFIER_LEN: usize = 63;
    #[derive(
        optimal_memory_layout::OptimalMemoryLayout,
        Debug,
        Clone,
        newtype::AsRefStr,
        newtype::Display,
        newtype::BoundedString,
    )]
    #[bounded_string(max = GENERATE_PG_TABLE_MAX_IDENTIFIER_LEN, serde)]
    struct GeneratePgTableDbColumn(String);
    #[derive(
        optimal_memory_layout::OptimalMemoryLayout,
        Debug,
        Clone,
        newtype::AsRefStr,
        newtype::Display,
        newtype::BoundedString,
    )]
    #[bounded_string(max = GENERATE_PG_TABLE_MAX_IDENTIFIER_LEN, min = 1usize, serde)]
    struct GeneratePgTableExcludeField(String);
    impl std::ops::Deref for UsizeGeneratePgTableDbColumns {
        type Target = [GeneratePgTableDbColumn];
        fn deref(&self) -> &Self::Target {
            self.0.as_slice()
        }
    }
    impl std::ops::Deref for UsizeCreateExcludeFields {
        type Target = [GeneratePgTableExcludeField];
        fn deref(&self) -> &Self::Target {
            self.0.as_slice()
        }
    }
    impl std::ops::Deref for UsizeReadExcludeFields {
        type Target = [GeneratePgTableExcludeField];
        fn deref(&self) -> &Self::Target {
            self.0.as_slice()
        }
    }
    #[derive(
        optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Default, serde::Deserialize,
    )]
    enum GeneratePgTableApiMode {
        AppendOnly,
        CreateReadDelete,
        #[default]
        Crud,
        ReadOnly,
        ReadUpdate,
    }
    #[derive(
        optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, serde::Deserialize,
    )]
    #[serde(from = "usize")]
    #[derive(newtype::FromInner)]
    struct StdBulkItemsMax(usize);

    #[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, serde::Deserialize)]
    #[serde(try_from = "Vec<GeneratePgTableDbColumn>")]
    struct UsizeGeneratePgTableDbColumns(
        pg_crud_common::bounded_vec::BoundedVec<GeneratePgTableDbColumn, 0usize, { usize::MAX }>,
    );
    #[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, serde::Deserialize)]
    #[serde(try_from = "Vec<GeneratePgTableExcludeField>")]
    struct UsizeCreateExcludeFields(
        pg_crud_common::bounded_vec::BoundedVec<
            GeneratePgTableExcludeField,
            0usize,
            { usize::MAX },
        >,
    );
    #[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, serde::Deserialize)]
    #[serde(try_from = "Vec<GeneratePgTableExcludeField>")]
    struct UsizeReadExcludeFields(
        pg_crud_common::bounded_vec::BoundedVec<
            GeneratePgTableExcludeField,
            0usize,
            { usize::MAX },
        >,
    );
    impl TryFrom<Vec<GeneratePgTableDbColumn>> for UsizeGeneratePgTableDbColumns {
        type Error = pg_crud_common::bounded_vec::BoundedVecError;
        fn try_from(value: Vec<GeneratePgTableDbColumn>) -> Result<Self, Self::Error> {
            Ok(Self(pg_crud_common::bounded_vec::BoundedVec::try_from(
                value,
            )?))
        }
    }
    impl TryFrom<Vec<GeneratePgTableExcludeField>> for UsizeCreateExcludeFields {
        type Error = pg_crud_common::bounded_vec::BoundedVecError;
        fn try_from(value: Vec<GeneratePgTableExcludeField>) -> Result<Self, Self::Error> {
            Ok(Self(pg_crud_common::bounded_vec::BoundedVec::try_from(
                value,
            )?))
        }
    }
    impl TryFrom<Vec<GeneratePgTableExcludeField>> for UsizeReadExcludeFields {
        type Error = pg_crud_common::bounded_vec::BoundedVecError;
        fn try_from(value: Vec<GeneratePgTableExcludeField>) -> Result<Self, Self::Error> {
            Ok(Self(pg_crud_common::bounded_vec::BoundedVec::try_from(
                value,
            )?))
        }
    }
    #[derive(optimal_memory_layout::OptimalMemoryLayout)]
    struct GeneratePgTableEmissionModel {
        config: GeneratePgTableConfig,
        error_variants_by_attr:
            std::collections::BTreeMap<GeneratePgTableAttr, Vec<GeneratePgTableVariantEmission>>,
        logic_token_stream_by_attr:
            std::collections::BTreeMap<GeneratePgTableAttr, proc_macro2::TokenStream>,
    }
    #[derive(optimal_memory_layout::OptimalMemoryLayout, newtype::FromInner)]
    struct ProcMacro2GeneratePgTableTestsTokenStream(proc_macro2::TokenStream);

    impl ProcMacro2GeneratePgTableTestsTokenStream {
        const fn as_ref(&self) -> &proc_macro2::TokenStream {
            &self.0
        }
        fn into_inner(self) -> proc_macro2::TokenStream {
            self.0
        }
    }
    #[derive(optimal_memory_layout::OptimalMemoryLayout, newtype::FromInner)]
    struct ProcMacro2GeneratePgTableCommonTokenStream(proc_macro2::TokenStream);

    impl ProcMacro2GeneratePgTableCommonTokenStream {
        const fn as_ref(&self) -> &proc_macro2::TokenStream {
            &self.0
        }
    }
    #[derive(optimal_memory_layout::OptimalMemoryLayout, newtype::FromInner)]
    struct ProcMacro2GeneratePgTableWholeTokenStream(proc_macro2::TokenStream);

    impl ProcMacro2GeneratePgTableWholeTokenStream {
        const fn as_ref(&self) -> &proc_macro2::TokenStream {
            &self.0
        }
        fn into_inner(self) -> proc_macro2::TokenStream {
            self.0
        }
    }
    #[derive(optimal_memory_layout::OptimalMemoryLayout, newtype::FromInner)]
    struct SynGeneratePgTableDeriveInput(syn::DeriveInput);

    impl SynGeneratePgTableDeriveInput {
        const fn get(&self) -> &syn::DeriveInput {
            &self.0
        }
    }
    #[derive(optimal_memory_layout::OptimalMemoryLayout)]
    struct GeneratePgTableFieldEmissionModel {
        field: macros_helpers::field_data::SynField,
        frontend: GeneratePgTableFrontendFieldEmission,
        has_db_default: bool,
        is_primary_key: bool,
    }
    #[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Default)]
    #[allow(clippy::arbitrary_source_item_ordering)] // alignment order required by optimal_memory_layout takes precedence over alphabetical field order
    struct GeneratePgTableFrontendFieldEmission {
        label: Option<String>,
        order: Option<usize>,
        placeholder: Option<String>,
        filterable: bool,
        hidden: bool,
        sortable: bool,
    }
    #[derive(
        optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, PartialEq, Eq, PartialOrd, Ord,
    )]
    enum GeneratePgTableFrontendFlag {
        Filterable,
        Hidden,
        Sortable,
    }
    #[derive(optimal_memory_layout::OptimalMemoryLayout)]
    #[allow(clippy::arbitrary_source_item_ordering)] // alignment order required by optimal_memory_layout takes precedence over alphabetical field order
    struct GeneratePgTableVariantFieldEmission {
        identifier: syn::Ident,
        type0: syn::Type,
        location_attr: Option<macros_helpers::location_data::LocationFieldAttr>,
    }
    #[derive(optimal_memory_layout::OptimalMemoryLayout)]
    struct GeneratePgTableVariantEmission {
        fields: Vec<GeneratePgTableVariantFieldEmission>,
        identifier: syn::Ident,
    }
    #[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy)]
    enum GeneratePgTableVariantEmissionRef<'variant_lt> {
        Model(&'variant_lt GeneratePgTableVariantEmission),
        Syn(&'variant_lt syn::Variant),
    }
    impl<'variant_lt> GeneratePgTableVariantEmissionRef<'variant_lt> {
        const fn identifier(self) -> &'variant_lt syn::Ident {
            match self {
                Self::Model(v) => &v.identifier,
                Self::Syn(v) => &v.ident,
            }
        }
    }
    #[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, newtype::FromInner)]
    struct GeneratePgTableFieldIdx(usize);

    impl GeneratePgTableFieldIdx {
        const fn get(self) -> usize {
            self.0
        }
    }
    #[derive(optimal_memory_layout::OptimalMemoryLayout)]
    struct GeneratePgTableFieldsEmissionModel {
        db_default_field_idxs: Vec<GeneratePgTableFieldIdx>,
        fields: Vec<macros_helpers::field_data::SynField>,
        fields_without_primary_key_idxs: Vec<GeneratePgTableFieldIdx>,
        frontend_fields: Vec<GeneratePgTableFrontendFieldEmission>,
        primary_key_field_idx: GeneratePgTableFieldIdx,
    }
    #[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, newtype::FromInner)]
    struct SynGeneratePgTableFieldRef<'field_lt>(&'field_lt syn::Field);

    impl<'field_lt> SynGeneratePgTableFieldRef<'field_lt> {
        const fn get(self) -> &'field_lt syn::Field {
            self.0
        }
    }
    #[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, newtype::FromInner)]
    struct SynGeneratePgTableIdentifierRef<'identifier_lt>(&'identifier_lt syn::Ident);

    impl<'identifier_lt> SynGeneratePgTableIdentifierRef<'identifier_lt> {
        const fn get(self) -> &'identifier_lt syn::Ident {
            self.0
        }
    }
    #[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, newtype::FromInner)]
    struct SynGeneratePgTableTypeRef<'type_lt>(&'type_lt syn::Type);

    impl<'type_lt> SynGeneratePgTableTypeRef<'type_lt> {
        const fn get(self) -> &'type_lt syn::Type {
            self.0
        }
    }
    #[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, newtype::FromInner)]
    struct GeneratePgTableVariantLocationAttr(
        Option<macros_helpers::location_data::LocationFieldAttr>,
    );

    impl GeneratePgTableVariantLocationAttr {
        const fn get(self) -> Option<macros_helpers::location_data::LocationFieldAttr> {
            self.0
        }
    }
    #[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, newtype::FromInner)]
    struct GeneratePgTablePrimaryKeyAttrName<'name_lt>(&'name_lt str);

    impl<'name_lt> GeneratePgTablePrimaryKeyAttrName<'name_lt> {
        const fn get(self) -> &'name_lt str {
            self.0
        }
    }
    #[allow(clippy::single_call_fn)]
    fn generate_pg_table_field_model_stage(
        field_ref: SynGeneratePgTableFieldRef<'_>,
        primary_key_attr_name: GeneratePgTablePrimaryKeyAttrName<'_>,
    ) -> Result<
        GeneratePgTableFieldEmissionModel,
        macros_helpers::proc_macro2_tokens::ProcMacro2GeneratedRustTokenStream,
    > {
        let syn_field = field_ref.get();
        let Some(field_identifier) = syn_field.ident.clone() else {
            return Err(compile_error_token_stream(CompileErrorMessage::from(
                str_constants::COMPILE_ERROR_CE_026,
            )));
        };
        let field_len = field_identifier.to_string().len();
        let max_pg_column_len = 63;
        if field_len > max_pg_column_len {
            return Err(compile_error_token_stream(CompileErrorMessage::from(
                str_constants::COMPILE_ERROR_CE_002,
            )));
        }
        let field = macros_helpers::field_data::SynField {
            vis: macros_helpers::field_data::SynFieldVis::from(syn_field.vis.clone()),
            type0: macros_helpers::field_data::SynFieldType::from(syn_field.ty.clone()),
            identifier: macros_helpers::field_data::SynFieldIdentifier::from(field_identifier),
        };
        let has_db_default = syn_field.attrs.iter().any(|attribute| {
            attribute
                .path()
                .is_ident(str_constants::GENERATE_PG_TABLE_DB_DEFAULT)
        });
        let mut frontend = GeneratePgTableFrontendFieldEmission::default();
        let mut frontend_flags = workspace_macro_helpers::StdUniqueOptionSet::default();
        let mut frontend_attr_count = 0usize;
        syn_field
            .attrs
            .iter()
            .filter(|attr| {
                attr.path()
                    .is_ident(str_constants::GENERATE_PG_TABLE_FRONTEND)
            })
            .try_for_each(|attr| {
                frontend_attr_count = frontend_attr_count.saturating_add(1usize);
                if frontend_attr_count > 1usize {
                    return Err(syn::Error::new_spanned(
                        attr,
                        str_constants::DUPLICATE_GENERATE_PG_TABLE_FRONTEND_ATTRIBUTE,
                    ));
                }
                attr.parse_nested_meta(|meta| {
                    if meta.path.is_ident(str_constants::FILTERABLE) {
                        frontend_flags
                            .try_insert_with(GeneratePgTableFrontendFlag::Filterable, || {
                                meta.error(str_constants::DUPLICATE_FILTERABLE_OPTION)
                            })?;
                        frontend.filterable = true;
                        return Ok(());
                    }
                    if meta.path.is_ident(str_constants::HIDDEN) {
                        frontend_flags
                            .try_insert_with(GeneratePgTableFrontendFlag::Hidden, || {
                                meta.error(str_constants::DUPLICATE_HIDDEN_OPTION)
                            })?;
                        frontend.hidden = true;
                        return Ok(());
                    }
                    if meta.path.is_ident(str_constants::LABEL) {
                        if frontend.label.is_some() {
                            return Err(meta.error(str_constants::DUPLICATE_LABEL_OPTION));
                        }
                        let value = meta.value()?.parse::<syn::LitStr>()?.value();
                        if value.trim().is_empty() {
                            return Err(meta.error(str_constants::FRONTEND_LABEL_MUST_NOT_BE_EMPTY));
                        }
                        frontend.label = Some(value);
                        return Ok(());
                    }
                    if meta.path.is_ident(str_constants::ORDER) {
                        if frontend.order.is_some() {
                            return Err(meta.error(str_constants::DUPLICATE_ORDER_OPTION));
                        }
                        frontend.order = Some(
                            meta.value()?
                                .parse::<syn::LitInt>()?
                                .base10_parse::<usize>()?,
                        );
                        return Ok(());
                    }
                    if meta.path.is_ident(str_constants::PLACEHOLDER) {
                        if frontend.placeholder.is_some() {
                            return Err(meta.error(str_constants::DUPLICATE_PLACEHOLDER_OPTION));
                        }
                        frontend.placeholder = Some(meta.value()?.parse::<syn::LitStr>()?.value());
                        return Ok(());
                    }
                    if meta.path.is_ident(str_constants::SORTABLE) {
                        frontend_flags
                            .try_insert_with(GeneratePgTableFrontendFlag::Sortable, || {
                                meta.error(str_constants::DUPLICATE_SORTABLE_OPTION)
                            })?;
                        frontend.sortable = true;
                        return Ok(());
                    }
                    Err(meta.error(str_constants::UNSUPPORTED_GENERATE_PG_TABLE_FRONTEND_OPTION))
                })
            })
            .map_err(|error| {
                macros_helpers::proc_macro2_tokens::ProcMacro2GeneratedRustTokenStream::from(
                    error.into_compile_error(),
                )
            })?;
        let is_primary_key = syn_field
            .attrs
            .iter()
            .filter(|el0| el0.path().segments.len() == 1)
            .any(|el0| {
                el0.path()
                    .segments
                    .first()
                    .is_some_and(|first_segment| first_segment.ident == primary_key_attr_name.get())
            });
        Ok(GeneratePgTableFieldEmissionModel {
            field,
            frontend,
            has_db_default,
            is_primary_key,
        })
    }
    #[allow(clippy::single_call_fn)]
    fn generate_pg_table_variant_field_model_stage(
        syn_field: syn::Field,
    ) -> Result<
        GeneratePgTableVariantFieldEmission,
        macros_helpers::proc_macro2_tokens::ProcMacro2GeneratedRustTokenStream,
    > {
        let Some(identifier) = syn_field.ident else {
            return Err(compile_error_token_stream(CompileErrorMessage::from(
                str_constants::COMPILE_ERROR_CE_030,
            )));
        };
        let parsed_location_attr = if identifier == naming::LocationSnakeCase.to_string() {
            None
        } else {
            let mut location_attrs = syn_field.attrs.iter().filter_map(|element| {
                if element.path().segments.len() != 1 {
                    return None;
                }
                let segment = element.path().segments.first()?;
                <macros_helpers::location_data::LocationFieldAttr as std::str::FromStr>::from_str(
                    &segment.ident.to_string(),
                )
                .ok()
            });
            let location_attr = location_attrs.next();
            if location_attrs.next().is_some() {
                return Err(compile_error_token_stream(CompileErrorMessage::from(
                    str_constants::COMPILE_ERROR_CE_029,
                )));
            }
            let Some(parsed_location_attr) = location_attr else {
                return Err(compile_error_token_stream(CompileErrorMessage::from(
                    str_constants::COMPILE_ERROR_CE_023,
                )));
            };
            Some(parsed_location_attr)
        };
        Ok(GeneratePgTableVariantFieldEmission {
            identifier,
            location_attr: parsed_location_attr,
            type0: syn_field.ty,
        })
    }
    fn generate_pg_table_syn_field_location_attr_stage(
        field_ref: SynGeneratePgTableFieldRef<'_>,
    ) -> Result<
        Option<macros_helpers::location_data::LocationFieldAttr>,
        macros_helpers::proc_macro2_tokens::ProcMacro2GeneratedRustTokenStream,
    > {
        let field = field_ref.get();
        let Some(field_identifier) = field.ident.as_ref() else {
            return Err(compile_error_token_stream(CompileErrorMessage::from(
                str_constants::MACRO_DIAGNOSTICS_EXPECTED_NAMED_FIELD_A2_ERROR,
            )));
        };
        if *field_identifier == naming::LocationSnakeCase.to_string() {
            return Ok(None);
        }
        let mut location_attrs = field.attrs.iter().filter_map(|element| {
            if element.path().segments.len() != 1 {
                return None;
            }
            let segment = element.path().segments.first()?;
            <macros_helpers::location_data::LocationFieldAttr as std::str::FromStr>::from_str(
                &segment.ident.to_string(),
            )
            .ok()
        });
        let location_attr = location_attrs.next();
        if location_attrs.next().is_some() {
            return Err(compile_error_token_stream(CompileErrorMessage::from(
                str_constants::COMPILE_ERROR_CE_028,
            )));
        }
        Ok(location_attr)
    }
    #[allow(clippy::single_call_fn)]
    fn generate_pg_table_variant_model_stage(
        syn_variant: syn::Variant,
    ) -> Result<
        GeneratePgTableVariantEmission,
        macros_helpers::proc_macro2_tokens::ProcMacro2GeneratedRustTokenStream,
    > {
        let syn::Fields::Named(fields_named) = syn_variant.fields else {
            return Err(compile_error_token_stream(CompileErrorMessage::from(
                str_constants::COMPILE_ERROR_CE_004,
            )));
        };
        let fields_len = fields_named.named.len();
        let fields = fields_named.named.into_iter().try_fold(
            Vec::with_capacity(fields_len),
            |mut accumulator, field| {
                accumulator.push(generate_pg_table_variant_field_model_stage(field)?);
                Ok::<
                    Vec<GeneratePgTableVariantFieldEmission>,
                    macros_helpers::proc_macro2_tokens::ProcMacro2GeneratedRustTokenStream,
                >(accumulator)
            },
        )?;
        Ok(GeneratePgTableVariantEmission {
            fields,
            identifier: syn_variant.ident,
        })
    }
    #[allow(clippy::single_call_fn)]
    fn build_generate_pg_table_fields_model_stage(
        input: &SynGeneratePgTableDeriveInput,
        primary_key_attr_name: GeneratePgTablePrimaryKeyAttrName<'_>,
    ) -> Result<
        GeneratePgTableFieldsEmissionModel,
        macros_helpers::proc_macro2_tokens::ProcMacro2GeneratedRustTokenStream,
    > {
        match crate::parse::struct_shape(workspace_macro_helpers::SynDeriveInputRef::from(
            input.get(),
        )) {
            Ok(workspace_macro_helpers::SynStructShapeRef::Named(fields_named_ref)) => {
                let fields_named = fields_named_ref.get();
                let fields_accumulator = fields_named.named.iter().try_fold(
                    (
                        Vec::with_capacity(fields_named.named.len()),
                        None,
                        Vec::with_capacity(fields_named.named.len()),
                        Vec::with_capacity(fields_named.named.len()),
                        Vec::with_capacity(fields_named.named.len()),
                    ),
                    |(
                        mut db_default_fields,
                        mut optional_primary_key_field,
                        mut fields,
                        mut fields_without_primary_key,
                        mut frontend_fields,
                    ),
                     element| {
                        let field_model = generate_pg_table_field_model_stage(
                            SynGeneratePgTableFieldRef::from(element),
                            primary_key_attr_name,
                        )?;
                        let field_idx = GeneratePgTableFieldIdx::from(fields.len());
                        if field_model.has_db_default {
                            db_default_fields.push(field_idx);
                        }
                        if field_model.is_primary_key {
                            if optional_primary_key_field.is_some() {
                                return Err(compile_error_token_stream(CompileErrorMessage::from(
                                    str_constants::COMPILE_ERROR_CE_003,
                                )));
                            }
                            optional_primary_key_field = Some(field_idx);
                        } else {
                            fields_without_primary_key.push(field_idx);
                        }
                        fields.push(field_model.field);
                        frontend_fields.push(field_model.frontend);
                        Ok((
                            db_default_fields,
                            optional_primary_key_field,
                            fields,
                            fields_without_primary_key,
                            frontend_fields,
                        ))
                    },
                );
                let (
                    db_default_field_idxs,
                    optional_primary_key_field,
                    fields,
                    fields_without_primary_key_idxs,
                    frontend_fields,
                ) = fields_accumulator?;
                let Some(primary_key_field_idx) = optional_primary_key_field else {
                    return Err(compile_error_token_stream(CompileErrorMessage::from(
                        str_constants::COMPILE_ERROR_CE_015,
                    )));
                };
                Ok(GeneratePgTableFieldsEmissionModel {
                    db_default_field_idxs,
                    fields,
                    fields_without_primary_key_idxs,
                    frontend_fields,
                    primary_key_field_idx,
                })
            }
            Ok(
                workspace_macro_helpers::SynStructShapeRef::Tuple(_)
                | workspace_macro_helpers::SynStructShapeRef::Unit,
            ) => Err(compile_error_token_stream(CompileErrorMessage::from(
                str_constants::COMPILE_ERROR_CE_018,
            ))),
            Err(_error) => Err(compile_error_token_stream(CompileErrorMessage::from(
                str_constants::COMPILE_ERROR_CE_043,
            ))),
        }
    }
    #[allow(clippy::single_call_fn)]
    fn build_generate_pg_table_input_model_stage(
        input: &SynGeneratePgTableDeriveInput,
    ) -> Result<
        GeneratePgTableEmissionModel,
        macros_helpers::proc_macro2_tokens::ProcMacro2GeneratedRustTokenStream,
    > {
        let di = input.get();
        let config_attr =
            match macros_helpers::attr_reader::try_get_macro_attr_meta_list_token_stream(
                &di.attrs,
                str_constants::PG_CRUD_GENERATE_PG_TABLE_CONFIG_PATH,
            ) {
                Ok(config_attr) => config_attr,
                Err(error) => {
                    let message =
                        format!("failed to read GeneratePgTableConfig attribute: {error}");
                    return Err(
                    macros_helpers::proc_macro2_tokens::ProcMacro2GeneratedRustTokenStream::from(
                        quote::quote! { compile_error!(#message); },
                    ),
                );
                }
            };
        let config = match serde_json::from_str::<GeneratePgTableConfig>(&config_attr.to_string()) {
            Ok(v) => v,
            Err(error) => {
                let message = format!("failed to parse GeneratePgTableConfig: {error}");
                return Err(
                    macros_helpers::proc_macro2_tokens::ProcMacro2GeneratedRustTokenStream::from(
                        quote::quote! { compile_error!(#message); },
                    ),
                );
            }
        };
        if config
            .cm_max_items
            .into_iter()
            .chain(config.um_max_items)
            .any(|limit| limit.0 == 0usize)
        {
            return Err(compile_error_token_stream(CompileErrorMessage::from(
                str_constants::COMPILE_ERROR_CE_013,
            )));
        }
        if config.permission_prefix.as_ref().is_some_and(|prefix| {
            prefix.is_empty()
                || !prefix
                    .bytes()
                    .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'_')
        }) {
            return Err(compile_error_token_stream(CompileErrorMessage::from(
                str_constants::COMPILE_ERROR_CE_051,
            )));
        }
        if config.db_table_name.as_ref().is_some_and(|table_name| {
            table_name.is_empty()
                || !table_name
                    .bytes()
                    .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'_')
        }) {
            return Err(compile_error_token_stream(CompileErrorMessage::from(
                str_constants::COMPILE_ERROR_CE_083,
            )));
        }
        let error_variants_by_attr = [
            GeneratePgTableAttr::CmErrorVariants,
            GeneratePgTableAttr::CoErrorVariants,
            GeneratePgTableAttr::RmErrorVariants,
            GeneratePgTableAttr::RoErrorVariants,
            GeneratePgTableAttr::UmErrorVariants,
            GeneratePgTableAttr::UoErrorVariants,
            GeneratePgTableAttr::DmErrorVariants,
            GeneratePgTableAttr::DloErrorVariants,
            GeneratePgTableAttr::CommonErrorVariants,
        ]
        .into_iter()
        .try_fold(
            std::collections::BTreeMap::new(),
            |mut accumulator, generate_pg_table_attr| {
                let generate_pg_table_attr_str = generate_pg_table_attr.to_string();
                let Ok(common_error_variants_attr_token_stream) =
                    macros_helpers::attr_reader::try_get_macro_attr_meta_list_token_stream(
                        &di.attrs,
                        &generate_pg_table_attr.generate_path_to_attr(),
                    )
                else {
                    return Ok(accumulator);
                };
                let Ok(parsed_di): Result<syn::DeriveInput, _> =
                    syn::parse2((*common_error_variants_attr_token_stream).clone())
                else {
                    return Ok(accumulator);
                };
                if parsed_di.ident != generate_pg_table_attr_str {
                    return Err(compile_error_token_stream(CompileErrorMessage::from(
                        str_constants::COMPILE_ERROR_CE_022,
                    )));
                }
                if let syn::Data::Enum(data_enum) = parsed_di.data {
                    let variants_len = data_enum.variants.len();
                    let variants = data_enum.variants.into_iter().try_fold(
                        Vec::with_capacity(variants_len),
                        |mut variants_accumulator, variant| {
                            variants_accumulator.push(generate_pg_table_variant_model_stage(variant)?);
                            Ok::<
                                Vec<GeneratePgTableVariantEmission>,
                                macros_helpers::proc_macro2_tokens::ProcMacro2GeneratedRustTokenStream,
                            >(variants_accumulator)
                        },
                    )?;
                    drop(accumulator.insert(generate_pg_table_attr, variants));
                }
                Ok(accumulator)
            },
        )?;
        let logic_token_stream_by_attr = [
            GeneratePgTableAttr::CmLogic,
            GeneratePgTableAttr::CoLogic,
            GeneratePgTableAttr::RmLogic,
            GeneratePgTableAttr::RoLogic,
            GeneratePgTableAttr::UmLogic,
            GeneratePgTableAttr::UoLogic,
            GeneratePgTableAttr::DmLogic,
            GeneratePgTableAttr::DloLogic,
            GeneratePgTableAttr::CommonLogic,
        ]
        .into_iter()
        .map(|generate_pg_table_attr| {
            let logic_token_stream =
                macros_helpers::attr_reader::try_get_macro_attr_meta_list_token_stream(
                    &di.attrs,
                    &generate_pg_table_attr.generate_path_to_attr(),
                )
                .map_or_else(
                    |_error| proc_macro2::TokenStream::new(),
                    |value| (*value).clone(),
                );
            (generate_pg_table_attr, logic_token_stream)
        })
        .collect::<std::collections::BTreeMap<GeneratePgTableAttr, proc_macro2::TokenStream>>();
        Ok(GeneratePgTableEmissionModel {
            config,
            error_variants_by_attr,
            logic_token_stream_by_attr,
        })
    }
    #[allow(clippy::single_call_fn)]
    fn validate_generate_pg_table_fields_model_stage(
        model: GeneratePgTableFieldsEmissionModel,
    ) -> Result<
        GeneratePgTableFieldsEmissionModel,
        macros_helpers::proc_macro2_tokens::ProcMacro2GeneratedRustTokenStream,
    > {
        if model
            .fields
            .get(model.primary_key_field_idx.get())
            .is_none()
        {
            return Err(compile_error_token_stream(CompileErrorMessage::from(
                str_constants::MACRO_DIAGNOSTICS_PRIMARY_KEY_FIELD_INDEX_ERROR,
            )));
        }
        if model
            .fields_without_primary_key_idxs
            .iter()
            .any(|idx| model.fields.get(idx.get()).is_none())
        {
            return Err(compile_error_token_stream(CompileErrorMessage::from(
                str_constants::COMPILE_ERROR_CE_006,
            )));
        }
        if model.fields.len() != model.frontend_fields.len() {
            return Err(compile_error_token_stream(CompileErrorMessage::from(
                str_constants::COMPILE_ERROR_CE_021,
            )));
        }
        Ok(model)
    }
    #[allow(clippy::single_call_fn)]
    fn emit_generate_pg_table_tests_stage(
        config: &GeneratePgTableConfig,
        tests_token_stream: ProcMacro2GeneratePgTableTestsTokenStream,
    ) -> ProcMacro2GeneratePgTableTestsTokenStream {
        if let Err(error) = macros_helpers::ts_writer::maybe_write_token_stream_into_file(
            config.tests_write_into_file,
            str_constants::GENERATE_PG_TABLE_TESTS,
            macros_helpers::ts_writer::ProcMacro2TokenStreamRef::from(tests_token_stream.as_ref()),
            &macros_helpers::ts_writer::FormatWithCargofmt::True,
        ) {
            let message = format!("failed to write generated PG table tests: {error}");
            return ProcMacro2GeneratePgTableTestsTokenStream::from(
                quote::quote! { compile_error!(#message); },
            );
        }
        match config.tests_write_into_file {
            macros_helpers::ts_writer::ShouldWriteTokenStreamIntoFile::False => {
                ProcMacro2GeneratePgTableTestsTokenStream::from(proc_macro2::TokenStream::new())
            }
            macros_helpers::ts_writer::ShouldWriteTokenStreamIntoFile::True => tests_token_stream,
        }
    }
    #[allow(clippy::single_call_fn)]
    fn emit_generate_pg_table_final_stage(
        config: &GeneratePgTableConfig,
        common_token_stream: &ProcMacro2GeneratePgTableCommonTokenStream,
        whole_token_stream: ProcMacro2GeneratePgTableWholeTokenStream,
    ) -> macros_helpers::proc_macro2_tokens::ProcMacro2GeneratedRustTokenStream {
        if let Err(error) = macros_helpers::ts_writer::maybe_write_token_stream_into_file(
            config.common_write_into_file,
            str_constants::GENERATE_PG_TABLE_COMMON,
            macros_helpers::ts_writer::ProcMacro2TokenStreamRef::from(common_token_stream.as_ref()),
            &macros_helpers::ts_writer::FormatWithCargofmt::True,
        ) {
            let message = format!("failed to write generated PG table common output: {error}");
            return macros_helpers::proc_macro2_tokens::ProcMacro2GeneratedRustTokenStream::from(
                quote::quote! { compile_error!(#message); },
            );
        }
        if let Err(error) = macros_helpers::ts_writer::maybe_write_token_stream_into_file(
            config.whole_write_into_file,
            str_constants::GENERATE_PG_TABLE,
            macros_helpers::ts_writer::ProcMacro2TokenStreamRef::from(whole_token_stream.as_ref()),
            &macros_helpers::ts_writer::FormatWithCargofmt::True,
        ) {
            let message = format!("failed to write generated PG table output: {error}");
            return macros_helpers::proc_macro2_tokens::ProcMacro2GeneratedRustTokenStream::from(
                quote::quote! { compile_error!(#message); },
            );
        }
        macros_helpers::proc_macro2_tokens::ProcMacro2GeneratedRustTokenStream::from(
            whole_token_stream.into_inner(),
        )
    }
    panic_location::panic_location();
    let import = pg_crud_macros_common::Import::PgCrudCommon;
    let import_token_stream = quote::quote! {#import::};
    let return_err_query_part_error_write_into_buffer_token_stream =
        pg_crud_macros_common::generate_return_err_query_part_error_write_into_buffer_token_stream(
            import,
        );
    let parsed_input = SynGeneratePgTableDeriveInput::from(syn::DeriveInput::from(
        validated.into_model().into_input(),
    ));
    let di = parsed_input.get();
    let generate_pg_table_input_model =
        match build_generate_pg_table_input_model_stage(&parsed_input) {
            Ok(v) => v,
            Err(error) => return error,
        };
    let AllowClippyArbitrarySrcItemOrdering = token_patterns::AllowClippyArbitrarySrcItemOrdering;
    let AppStateSnakeCase = naming::AppStateSnakeCase;
    let BeginSnakeCase = naming::BeginSnakeCase;
    let BindedQuerySnakeCase = naming::BindedQuerySnakeCase;
    let BodyBytesSnakeCase = naming::BodyBytesSnakeCase;
    let BodySnakeCase = naming::BodySnakeCase;
    let BodySizeErrorUpperCamelCase = naming::BodySizeErrorUpperCamelCase;
    let Bool = token_patterns::Bool;
    let BySnakeCase = naming::BySnakeCase;
    let Char = token_patterns::Char;
    let CheckBodySizeSnakeCase = naming::CheckBodySizeSnakeCase;
    let CheckBodySizeUpperCamelCase = naming::CheckBodySizeUpperCamelCase;
    let CmErrorVariantsSnakeCase = naming::CmErrorVariantsSnakeCase;
    let CmLogicSnakeCase = naming::CmLogicSnakeCase;
    let CommonErrorVariantsSnakeCase = naming::CommonErrorVariantsSnakeCase;
    let CommonLogicSnakeCase = naming::CommonLogicSnakeCase;
    let CommonReadIdsFromCoSnakeCase = naming::CommonReadIdsFromCoSnakeCase;
    let CoErrorVariantsSnakeCase = naming::CoErrorVariantsSnakeCase;
    let CoLogicSnakeCase = naming::CoLogicSnakeCase;
    let ColumnSnakeCase = naming::ColumnSnakeCase;
    let ColsSnakeCase = naming::ColsSnakeCase;
    let CommitSnakeCase = naming::CommitSnakeCase;
    let ConfigSnakeCase = naming::ConfigSnakeCase;
    let CoreDefault = token_patterns::CoreDefault;
    let CreateExtensionIfNotExistsUuidOsspUpperCamelCase =
        naming::CreateExtensionIfNotExistsUuidOsspUpperCamelCase;
    let CreateQueryBindSnakeCase = naming::CreateQueryBindSnakeCase;
    let CreateQueryPartSnakeCase = naming::CreateQueryPartSnakeCase;
    let CreateSnakeCase = naming::CreateSnakeCase;
    let CreateTableColumnQueryPartSnakeCase = naming::CreateTableColumnQueryPartSnakeCase;
    let CreateUpperCamelCase = naming::CreateUpperCamelCase;
    let DeResUpperCamelCase = naming::DeResUpperCamelCase;
    let DeriveDebugSerdeSerializeSerdeDeserialize =
        token_patterns::DeriveDebugSerdeSerializeSerdeDeserialize;
    let DeriveDebugThiserrorLocation = token_patterns::DeriveDebugThiserrorLocation;
    let DesirableUpperCamelCase = naming::DesirableUpperCamelCase;
    let DefaultSomeOneElementMaxPageSizeSnakeCase =
        naming::DefaultSomeOneElementMaxPageSizeSnakeCase;
    let DefaultSomeOneElementMaxPageSizeUpperCamelCase =
        naming::DefaultSomeOneElementMaxPageSizeUpperCamelCase;
    let DefaultSomeOneElementSnakeCase = naming::DefaultSomeOneElementSnakeCase;
    let DefaultSomeOneElementUpperCamelCase = naming::DefaultSomeOneElementUpperCamelCase;
    let DloErrorVariantsSnakeCase = naming::DloErrorVariantsSnakeCase;
    let DloLogicSnakeCase = naming::DloLogicSnakeCase;
    let DmErrorVariantsSnakeCase = naming::DmErrorVariantsSnakeCase;
    let DmLogicSnakeCase = naming::DmLogicSnakeCase;
    let ElementSnakeCase = naming::ElementSnakeCase;
    let EndpointLocationSnakeCase = naming::EndpointLocationSnakeCase;
    let Error0 = token_patterns::Error0;
    let Error1 = token_patterns::Error1;
    let Error2 = token_patterns::Error2;
    let Error3 = token_patterns::Error3;
    let ErrorSnakeCase = naming::ErrorSnakeCase;
    let ExecutorAcquireSnakeCase = naming::ExecutorAcquireSnakeCase;
    let ExecutorSnakeCase = naming::ExecutorSnakeCase;
    let ExpectedResSnakeCase = naming::ExpectedResSnakeCase;
    let ExtraParametersSnakeCase = naming::ExtraParametersSnakeCase;
    let F32 = token_patterns::F32;
    let F64 = token_patterns::F64;
    let FailedToGetResTextUpperCamelCase = naming::FailedToGetResTextUpperCamelCase;
    let FalseSnakeCase = naming::FalseSnakeCase;
    let FieldAttrSerdeSkipSerializingIfOptionalIsNone =
        token_patterns::FieldAttrSerdeSkipSerializingIfOptionalIsNone;
    let FromHSnakeCase = naming::FromHSnakeCase;
    let FutureSnakeCase = naming::FutureSnakeCase;
    let GenerateColumnQuealsVCommaUoQueryPartSnakeCase =
        naming::GenerateColumnQuealsVCommaUoQueryPartSnakeCase;
    let GeneratePgTablePrimaryKeySnakeCase = naming::GeneratePgTablePrimaryKeySnakeCase;
    let GenerateSelectQueryPartSnakeCase = naming::GenerateSelectQueryPartSnakeCase;
    let GenerateWhenColumnIdThenVUmQueryPartSnakeCase =
        naming::GenerateWhenColumnIdThenVUmQueryPartSnakeCase;
    let HeaderContentTypeAppJsonNotFoundUpperCamelCase =
        naming::HeaderContentTypeAppJsonNotFoundUpperCamelCase;
    let HeadersSnakeCase = naming::HeadersSnakeCase;
    let I8 = token_patterns::I8;
    let I16 = token_patterns::I16;
    let I32 = token_patterns::I32;
    let I64 = token_patterns::I64;
    let IdentifierCreateDefaultSnakeCase = naming::IdentifierCreateDefaultSnakeCase;
    let IncrementSnakeCase = naming::IncrementSnakeCase;
    let IntoSerdeVersionSnakeCase = naming::IntoSerdeVersionSnakeCase;
    let LocationSnakeCase = naming::LocationSnakeCase;
    let MustUse = token_patterns::MustUse;
    let NoFieldsProvidedUpperCamelCase = naming::NoFieldsProvidedUpperCamelCase;
    let NotUniqueFieldSnakeCase = naming::NotUniqueFieldSnakeCase;
    let NotUniqueFieldUpperCamelCase = naming::NotUniqueFieldUpperCamelCase;
    let NotUniquePrimaryKeySnakeCase = naming::NotUniquePrimaryKeySnakeCase;
    let NotUniquePrimaryKeyUpperCamelCase = naming::NotUniquePrimaryKeyUpperCamelCase;
    let OptionalVecCreateSnakeCase = naming::OptionalVecCreateSnakeCase;
    let OrderBySnakeCase = naming::OrderBySnakeCase;
    let OrderByUpperCamelCase = naming::OrderByUpperCamelCase;
    let OrderSnakeCase = naming::OrderSnakeCase;
    let PayloadSnakeCase = naming::PayloadSnakeCase;
    let PayloadUpperCamelCase = naming::PayloadUpperCamelCase;
    let PgCrudCommonDefaultSomeOneElement = token_patterns::PgCrudCommonDefaultSomeOneElement;
    let PgCrudCommonDefaultSomeOneElementCall =
        token_patterns::PgCrudCommonDefaultSomeOneElementCall;
    let PgCrudCommonDefaultSomeOneElementMaxPageSizeCall =
        token_patterns::PgCrudCommonDefaultSomeOneElementMaxPageSizeCall;
    let PgCrudSnakeCase = str_constants::PG_CRUD_COMMON;
    let PgPoolForTokioSpawnSyncMoveSnakeCase = naming::PgPoolForTokioSpawnSyncMoveSnakeCase;
    let PgPoolSnakeCase = naming::PgPoolSnakeCase;
    let PgSnakeCase = naming::PgSnakeCase;
    let PgTypeOptionalVecWhereGreaterThanTestSnakeCase =
        naming::PgTypeOptionalVecWhereGreaterThanTestSnakeCase;
    let PgTypeUpperCamelCase = naming::PgTypeUpperCamelCase;
    let PgUpperCamelCase = naming::PgUpperCamelCase;
    let PaginationSnakeCase = naming::PaginationSnakeCase;
    let PrimaryKeyQueryPartSnakeCase = naming::PrimaryKeyQueryPartSnakeCase;
    let PrimaryKeySnakeCase = naming::PrimaryKeySnakeCase;
    let PoolConnectionSnakeCase = naming::PoolConnectionSnakeCase;
    let PoolSnakeCase = naming::PoolSnakeCase;
    let PrefixSnakeCase = naming::PrefixSnakeCase;
    let PrepExtensionsSnakeCase = naming::PrepExtensionsSnakeCase;
    let PrepPgSnakeCase = naming::PrepPgSnakeCase;
    let PrepPgTableSnakeCase = naming::PrepPgTableSnakeCase;
    let PrepPgUpperCamelCase = naming::PrepPgUpperCamelCase;
    let ParametersSnakeCase = naming::ParametersSnakeCase;
    let QueryBindSnakeCase = naming::QueryBindSnakeCase;
    let QueryPartErrorUpperCamelCase = naming::QueryPartErrorUpperCamelCase;
    let QueryPartSnakeCase = naming::QueryPartSnakeCase;
    let QueryPartUpperCamelCase = naming::QueryPartUpperCamelCase;
    let QuerySnakeCase = naming::QuerySnakeCase;
    let QueryStringSnakeCase = naming::QueryStringSnakeCase;
    let ReadIdsAndCreateIntoOptionalVecWhereEqToFieldSnakeCase =
        naming::ReadIdsAndCreateIntoOptionalVecWhereEqToFieldSnakeCase;
    let ReadIdsAndCreateIntoVecWhereEqUsingFieldsSnakeCase =
        naming::ReadIdsAndCreateIntoVecWhereEqUsingFieldsSnakeCase;
    let ReadIdsAndCreateIntoWhereEqSnakeCase = naming::ReadIdsAndCreateIntoWhereEqSnakeCase;
    let ReadIdsAndTableTypeIntoPgTypeOptionalWhereGreaterThanSnakeCase =
        naming::ReadIdsAndTableTypeIntoPgTypeOptionalWhereGreaterThanSnakeCase;
    let ReadIdsIntoReadSnakeCase = naming::ReadIdsIntoReadSnakeCase;
    let ReadIdsIntoTableTypeSnakeCase = naming::ReadIdsIntoTableTypeSnakeCase;
    let ReadIdsIntoUpdateSnakeCase = naming::ReadIdsIntoUpdateSnakeCase;
    let ReadIdsSnakeCase = naming::ReadIdsSnakeCase;
    let ReadIdsUpperCamelCase = naming::ReadIdsUpperCamelCase;
    let ReadIntoTableTypeSnakeCase = naming::ReadIntoTableTypeSnakeCase;
    let ReadUpperCamelCase = naming::ReadUpperCamelCase;
    let RefStr = token_patterns::RefStr;
    let ReqSnakeCase = naming::ReqSnakeCase;
    let ReqwestSnakeCase = naming::ReqwestSnakeCase;
    let ReqwestUpperCamelCase = naming::ReqwestUpperCamelCase;
    let ResSnakeCase = naming::ResSnakeCase;
    let ResTextSnakeCase = naming::ResTextSnakeCase;
    let RmErrorVariantsSnakeCase = naming::RmErrorVariantsSnakeCase;
    let RmLogicSnakeCase = naming::RmLogicSnakeCase;
    let RoErrorVariantsSnakeCase = naming::RoErrorVariantsSnakeCase;
    let RoLogicSnakeCase = naming::RoLogicSnakeCase;
    let RollbackSnakeCase = naming::RollbackSnakeCase;
    let RoutesHSnakeCase = naming::RoutesHSnakeCase;
    let RoutesSnakeCase = naming::RoutesSnakeCase;
    let RowAndRollbackUpperCamelCase = naming::RowAndRollbackUpperCamelCase;
    let RowSnakeCase = naming::RowSnakeCase;
    let RowsSnakeCase = naming::RowsSnakeCase;
    let SelectOnlyIdsQueryPartSnakeCase = naming::SelectOnlyIdsQueryPartSnakeCase;
    let SelectOnlyUpdatedIdsQueryPartSnakeCase = naming::SelectOnlyUpdatedIdsQueryPartSnakeCase;
    let SelectPrimaryKeySnakeCase = naming::SelectPrimaryKeySnakeCase;
    let SelectQueryPartSnakeCase = naming::SelectQueryPartSnakeCase;
    let SelectSnakeCase = naming::SelectSnakeCase;
    let SelectUpperCamelCase = naming::SelectUpperCamelCase;
    let SerdeJsonSnakeCase = naming::SerdeJsonSnakeCase;
    let SerdeJsonToStringSnakeCase = naming::SerdeJsonToStringSnakeCase;
    let SerdeJsonToStringUpperCamelCase = naming::SerdeJsonToStringUpperCamelCase;
    let SerdeJsonUpperCamelCase = naming::SerdeJsonUpperCamelCase;
    let SerdeSnakeCase = naming::SerdeSnakeCase;
    let SqlxAcquire = token_patterns::SqlxAcquire;
    let SqlxRow = token_patterns::SqlxRow;
    let StatusCodeSnakeCase = naming::StatusCodeSnakeCase;
    let StringTokenStream = token_patterns::StringTokenStream;
    let TableNameSnakeCase = naming::TableNameSnakeCase;
    let TableSnakeCase = naming::TableSnakeCase;
    let TrueSnakeCase = naming::TrueSnakeCase;
    let TryBindSnakeCase = naming::TryBindSnakeCase;
    let TryBindUpperCamelCase = naming::TryBindUpperCamelCase;
    let U8 = token_patterns::U8;
    let U16 = token_patterns::U16;
    let U32 = token_patterns::U32;
    let U64 = token_patterns::U64;
    let UmErrorVariantsSnakeCase = naming::UmErrorVariantsSnakeCase;
    let UmLogicSnakeCase = naming::UmLogicSnakeCase;
    let UoErrorVariantsSnakeCase = naming::UoErrorVariantsSnakeCase;
    let UoLogicSnakeCase = naming::UoLogicSnakeCase;
    let UpdateForQuerySnakeCase = naming::UpdateForQuerySnakeCase;
    let UpdateForQueryUpperCamelCase = naming::UpdateForQueryUpperCamelCase;
    let UpdateForQueryVecSnakeCase = naming::UpdateForQueryVecSnakeCase;
    let UpdateQueryBindSnakeCase = naming::UpdateQueryBindSnakeCase;
    let UpdateQueryPartPrimaryKeySnakeCase = naming::UpdateQueryPartPrimaryKeySnakeCase;
    let UpdateQueryPartSnakeCase = naming::UpdateQueryPartSnakeCase;
    let UpdateSnakeCase = naming::UpdateSnakeCase;
    let UpdateUpperCamelCase = naming::UpdateUpperCamelCase;
    let UrlSnakeCase = naming::UrlSnakeCase;
    let VSnakeCase = naming::VSnakeCase;
    let VUpperCamelCase = naming::VUpperCamelCase;
    let WhereManySnakeCase = naming::WhereManySnakeCase;
    let WhereUpperCamelCase = naming::WhereUpperCamelCase;
    let identifier = &di.ident;
    let identifier_snake_case_string = naming_common::ToTokensToSnakeCaseStr::case(&identifier);
    let identifier_snake_case_double_quoted_token_stream =
        generate_quotes::dq_token_stream(&identifier_snake_case_string);
    let db_table_name_double_quoted_token_stream = generate_quotes::dq_token_stream(
        generate_pg_table_input_model
            .config
            .db_table_name
            .as_deref()
            .unwrap_or(identifier_snake_case_string.as_str()),
    );
    let identifier_auth_requirement_upper_camel_case =
        quote::format_ident!("{}AuthenticationRequirement", identifier);
    let identifier_http_method_upper_camel_case = quote::format_ident!("{}HttpMethod", identifier);
    let identifier_operation_upper_camel_case = quote::format_ident!("{}Operation", identifier);
    let identifier_route_contract_upper_camel_case =
        quote::format_ident!("{}RouteContract", identifier);
    let identifier_success_status_upper_camel_case =
        quote::format_ident!("{}SuccessStatus", identifier);
    let self_table_name_call_token_stream = quote::quote! {Self::#TableNameSnakeCase()};
    let db_table_name_snake_case = quote::format_ident!("db_table_name");
    let self_db_table_name_call_token_stream = quote::quote! {Self::#db_table_name_snake_case()};
    let db_table_snake_case = quote::format_ident!("db_table");
    let generate_pg_table_primary_key_snake_case_str =
        GeneratePgTablePrimaryKeySnakeCase.to_string();
    let fields_model = match build_generate_pg_table_fields_model_stage(
        &parsed_input,
        GeneratePgTablePrimaryKeyAttrName::from(
            generate_pg_table_primary_key_snake_case_str.as_str(),
        ),
    ) {
        Ok(v) => v,
        Err(error) => return error,
    };
    let validated_fields_model = match validate_generate_pg_table_fields_model_stage(fields_model) {
        Ok(v) => v,
        Err(error) => return error,
    };
    let GeneratePgTableFieldsEmissionModel {
        db_default_field_idxs,
        fields,
        fields_without_primary_key_idxs,
        frontend_fields,
        primary_key_field_idx,
    } = validated_fields_model;
    let fields_len = fields.len();
    let fields_len_without_primary_key = fields_without_primary_key_idxs.len();
    let Some(primary_key_field) = fields.get(primary_key_field_idx.get()) else {
        return compile_error_token_stream(CompileErrorMessage::from(
            str_constants::MACRO_DIAGNOSTICS_PRIMARY_KEY_FIELD_INDEX_ERROR,
        ));
    };
    let fields_without_primary_key_iter = || {
        fields_without_primary_key_idxs
            .iter()
            .filter_map(|field_idx| fields.get(field_idx.get()))
    };
    let create_exclude_fields = generate_pg_table_input_model
        .config
        .create_exclude_fields
        .as_deref()
        .unwrap_or(&[]);
    let read_exclude_fields = generate_pg_table_input_model
        .config
        .read_exclude_fields
        .as_deref()
        .unwrap_or(&[]);
    if create_exclude_fields.iter().any(|excluded| {
        excluded.as_ref() == primary_key_field.identifier.to_string()
            || !fields_without_primary_key_iter()
                .any(|field| field.identifier.to_string() == excluded.as_ref())
    }) {
        return compile_error_token_stream(CompileErrorMessage::from(
            str_constants::COMPILE_ERROR_CE_017,
        ));
    }
    let create_field_is_excluded = |field: &macros_helpers::field_data::SynField| {
        create_exclude_fields
            .iter()
            .any(|excluded| excluded.as_ref() == field.identifier.to_string())
    };
    let create_fields_without_primary_key_iter =
        || fields_without_primary_key_iter().filter(|field| !create_field_is_excluded(field));
    let read_excluded_fields_are_unique = read_exclude_fields
        .iter()
        .map(AsRef::as_ref)
        .collect::<std::collections::HashSet<_>>()
        .len()
        == read_exclude_fields.len();
    let read_excluded_fields_are_valid = read_exclude_fields.iter().all(|excluded| {
        excluded.as_ref() != primary_key_field.identifier.to_string()
            && fields_without_primary_key_iter()
                .any(|field| field.identifier.to_string() == excluded.as_ref())
    });
    if !read_excluded_fields_are_unique || !read_excluded_fields_are_valid {
        return compile_error_token_stream(CompileErrorMessage::from(
            str_constants::COMPILE_ERROR_CE_027,
        ));
    }
    let read_field_is_excluded = |field: &macros_helpers::field_data::SynField| {
        read_exclude_fields
            .iter()
            .any(|excluded| excluded.as_ref() == field.identifier.to_string())
    };
    let read_fields_iter = || fields.iter().filter(|field| !read_field_is_excluded(field));
    let read_fields_without_primary_key_iter =
        || fields_without_primary_key_iter().filter(|field| !read_field_is_excluded(field));
    let read_fields = read_fields_iter().cloned().collect::<Vec<_>>();
    let read_fields_len = read_fields.len();
    let optimistic_revision_field_idx = if let Some(revision_field_name) =
        generate_pg_table_input_model
            .config
            .optimistic_revision_field
            .as_ref()
    {
        let Some(field_idx) = fields
            .iter()
            .position(|field| field.identifier.to_string() == *revision_field_name)
        else {
            return compile_error_token_stream(CompileErrorMessage::from(
                str_constants::COMPILE_ERROR_CE_055,
            ));
        };
        let revision_type_is_valid = fields.get(field_idx).is_some_and(|field| {
            let syn::Type::Path(type_path) = &*field.type0 else {
                return false;
            };
            type_path.path.segments.last().is_some_and(|segment| {
                matches!(
                    segment.ident.to_string().as_str(),
                    str_constants::I64ASNONNULLINT8
                        | str_constants::I64ASNONNULLBIGSERIALINITIALIZATIONBYPG
                )
            })
        });
        if field_idx == primary_key_field_idx.get() || !revision_type_is_valid {
            return compile_error_token_stream(CompileErrorMessage::from(
                str_constants::COMPILE_ERROR_CE_012,
            ));
        }
        Some(field_idx)
    } else {
        None
    };
    let optimistic_revision_field_identifier = optimistic_revision_field_idx
        .and_then(|field_idx| fields.get(field_idx))
        .map(|field| field.identifier.clone());
    let operation_is_enabled =
        |operation: &Operation| match generate_pg_table_input_model.config.api_mode {
            GeneratePgTableApiMode::Crud => {
                optimistic_revision_field_idx.is_none() || !matches!(operation, Operation::Um)
            }
            GeneratePgTableApiMode::AppendOnly => matches!(
                operation,
                Operation::Cm | Operation::Co | Operation::Rm | Operation::Ro
            ),
            GeneratePgTableApiMode::CreateReadDelete => {
                matches!(
                    operation,
                    Operation::Cm
                        | Operation::Co
                        | Operation::Rm
                        | Operation::Ro
                        | Operation::Dm
                        | Operation::Dlo
                )
            }
            GeneratePgTableApiMode::ReadOnly => matches!(operation, Operation::Rm | Operation::Ro),
            GeneratePgTableApiMode::ReadUpdate => {
                matches!(operation, Operation::Rm | Operation::Ro | Operation::Uo)
                    || (optimistic_revision_field_idx.is_none()
                        && matches!(operation, Operation::Um))
            }
        };
    let mut frontend_field_order = frontend_fields
        .iter()
        .enumerate()
        .map(|(field_idx, config)| (config.order.unwrap_or(field_idx), field_idx))
        .collect::<Vec<_>>();
    frontend_field_order.sort_unstable_by_key(|(order, _field_idx)| *order);
    let frontend_orders_are_unique = frontend_field_order
        .iter()
        .map(|(order, _field_idx)| order)
        .collect::<std::collections::HashSet<_>>()
        .len()
        == frontend_field_order.len();
    if !frontend_orders_are_unique {
        return compile_error_token_stream(CompileErrorMessage::from(
            str_constants::COMPILE_ERROR_CE_011,
        ));
    }
    let frontend_field_contracts_token_stream = frontend_field_order
        .iter()
        .filter_map(|(order, field_idx)| {
            let field = fields.get(*field_idx)?;
            let frontend = frontend_fields.get(*field_idx)?;
            let field_name = field.identifier.to_string();
            let field_name_double_quoted_token_stream = generate_quotes::dq_token_stream(&field_name);
            let label = frontend.label.clone().unwrap_or_else(|| {
                let mut value = String::with_capacity(field_name.len());
                field_name
                    .split('_')
                    .enumerate()
                    .for_each(|(part_idx, part)| {
                        if part_idx != 0usize {
                            value.push(' ');
                        }
                        let mut chars = part.chars();
                        if let Some(first) = chars.next() {
                            value.extend(first.to_uppercase());
                            value.extend(chars);
                        }
                    });
                value
            });
            let label_double_quoted_token_stream = generate_quotes::dq_token_stream(&label);
            let field_type = &field.type0;
            let primary_key_token_stream = if *field_idx == primary_key_field_idx.get() {
                quote::quote! {frontend_contract::PrimaryKeyKind::Primary}
            } else {
                quote::quote! {frontend_contract::PrimaryKeyKind::NonPrimary}
            };
            let creatable_token_stream = if *field_idx != primary_key_field_idx.get()
                && !create_field_is_excluded(field)
                && (operation_is_enabled(&Operation::Cm) || operation_is_enabled(&Operation::Co))
            {
                quote::quote! {frontend_contract::FieldCapability::Enabled}
            } else {
                quote::quote! {frontend_contract::FieldCapability::Disabled}
            };
            let readable_token_stream = if !read_field_is_excluded(field)
                && (operation_is_enabled(&Operation::Rm) || operation_is_enabled(&Operation::Ro))
            {
                quote::quote! {frontend_contract::FieldCapability::Enabled}
            } else {
                quote::quote! {frontend_contract::FieldCapability::Disabled}
            };
            let updatable_token_stream = if *field_idx != primary_key_field_idx.get()
                && (operation_is_enabled(&Operation::Um) || operation_is_enabled(&Operation::Uo))
            {
                quote::quote! {frontend_contract::FieldCapability::Enabled}
            } else {
                quote::quote! {frontend_contract::FieldCapability::Disabled}
            };
            let filterable_token_stream = if frontend.filterable {
                quote::quote! {frontend_contract::FieldCapability::Enabled}
            } else {
                quote::quote! {frontend_contract::FieldCapability::Disabled}
            };
            let filters_token_stream = quote::quote! {
                <#field_type as frontend_contract::HasFilterContracts>::filter_contracts()
            };
            let sortable_token_stream = if frontend.sortable {
                quote::quote! {frontend_contract::FieldCapability::Enabled}
            } else {
                quote::quote! {frontend_contract::FieldCapability::Disabled}
            };
            let visibility_token_stream = if frontend.hidden {
                quote::quote! {frontend_contract::FieldVisibility::Hidden}
            } else {
                quote::quote! {frontend_contract::FieldVisibility::Visible}
            };
            let placeholder_token_stream = frontend.placeholder.as_ref().map_or_else(
                || quote::quote! {frontend_contract::FieldPlaceholder::None},
                |value| {
                    let value_double_quoted_token_stream = generate_quotes::dq_token_stream(value);
                    quote::quote! {frontend_contract::FieldPlaceholder::Value(frontend_contract::ContractStr::from(#value_double_quoted_token_stream))}
                },
            );
            Some(quote::quote! {
                frontend_contract::FieldContract::new(
                    frontend_contract::ContractStr::from(#field_name_double_quoted_token_stream),
                    frontend_contract::ContractStr::from(#label_double_quoted_token_stream),
                    <#field_type as frontend_contract::HasTypeContract>::type_contract(),
                )
                .with_primary_key(#primary_key_token_stream)
                .with_creatable(#creatable_token_stream)
                .with_filterable(#filterable_token_stream)
                .with_filters(#filters_token_stream)
                .with_order(frontend_contract::FieldOrder::from(#order))
                .with_placeholder(#placeholder_token_stream)
                .with_readable(#readable_token_stream)
                .with_sortable(#sortable_token_stream)
                .with_updatable(#updatable_token_stream)
                .with_visibility(#visibility_token_stream)
            })
        })
        .collect::<Vec<_>>();
    let frontend_capability_assertions_token_stream = fields
        .iter()
        .zip(frontend_fields.iter())
        .filter_map(|(field, frontend)| {
            let field_type = &field.type0;
            let sortable_assertion = frontend.sortable.then(|| {
                quote::quote! {
                    assert!(
                        matches!(<#field_type as frontend_contract::HasTypeContract>::type_contract().supports_sorting(), frontend_contract::CapabilitySupport::Supported),
                        "c5882cc4: frontend sorting is unsupported for this field type",
                    );
                }
            });
            let filterable_assertion = frontend.filterable.then(|| {
                quote::quote! {
                    assert!(
                        matches!(<#field_type as frontend_contract::HasTypeContract>::type_contract().supports_filtering(), frontend_contract::CapabilitySupport::Supported),
                        "141942af: frontend filtering is unsupported for this field type",
                    );
                }
            });
            (sortable_assertion.is_some() || filterable_assertion.is_some()).then(|| {
                quote::quote! {
                    const _: () = {
                        #sortable_assertion
                        #filterable_assertion
                    };
                }
            })
        })
        .collect::<Vec<_>>();
    let primary_key_field_type = &primary_key_field.type0;
    if fields_without_primary_key_idxs.is_empty() {
        return macros_helpers::proc_macro2_tokens::ProcMacro2GeneratedRustTokenStream::from(
            syn::Error::new_spanned(
                &**primary_key_field_type,
                str_constants::UPDATE_OPERATIONS_REQUIRE_AT_LEAST_ONE_NON_PRIMARY_KEY_FIELD,
            )
            .into_compile_error(),
        );
    }
    if let syn::Type::Path(type_path) = &**primary_key_field_type
        && let Some(last_segment) = type_path.path.segments.last()
    {
        let primary_key_type_name = last_segment.ident.to_string();
        if primary_key_type_name.starts_with(str_constants::OPTIONAL)
            || primary_key_type_name.contains(str_constants::ASNULLABLE)
        {
            return macros_helpers::proc_macro2_tokens::ProcMacro2GeneratedRustTokenStream::from(
                syn::Error::new_spanned(
                    &**primary_key_field_type,
                    str_constants::PRIMARY_KEY_TYPE_MUST_BE_NON_NULLABLE,
                )
                .into_compile_error(),
            );
        }
    }
    //todo must remove this and use trait type instead
    let primary_key_field_type_table_type_token_stream =
        naming::parameter::SelfTableTypeUpperCamelCase::from_type_last_segment(
            &primary_key_field.type0,
        );
    let generate_as_pg_type_token_stream = |ts: &dyn quote::ToTokens| {
        quote::quote! {<#ts as #import_token_stream #PgTypeUpperCamelCase>}
    };
    let generate_as_pg_type_path_token_stream = |ts: &dyn quote::ToTokens| {
        let ts0 = generate_as_pg_type_token_stream(ts);
        quote::quote! {#ts0::}
    };
    let primary_key_field_type_as_pg_type_token_stream =
        generate_as_pg_type_path_token_stream(&primary_key_field_type);
    let primary_key_field_type_as_pg_type_read_upper_camel_case =
        quote::quote! {#primary_key_field_type_as_pg_type_token_stream #ReadUpperCamelCase};
    let primary_key_as_pg_type_token_stream =
        generate_as_pg_type_token_stream(&primary_key_field_type);
    let generate_as_pg_type_tokens_token_stream =
        |ts: &dyn quote::ToTokens, tokens: &dyn quote::ToTokens| {
            let as_pg_type_token_stream = generate_as_pg_type_path_token_stream(&ts);
            quote::quote! {#as_pg_type_token_stream #tokens}
        };
    let generate_concrete_pg_type_role_token_stream =
        |field_type: &macros_helpers::field_data::SynFieldType, role: &dyn quote::ToTokens| {
            if let syn::Type::Path(type_path) = &**field_type {
                let mut role_type_path = type_path.clone();
                if let Some(last_segment) = role_type_path.path.segments.last_mut() {
                    last_segment.ident = quote::format_ident!(
                        "{}{}",
                        last_segment.ident,
                        role.to_token_stream().to_string()
                    );
                    quote::quote! {#role_type_path}
                } else {
                    quote::quote! {compile_error!("e396fe6d: field type path has no segments")}
                }
            } else {
                quote::quote! {compile_error!("51519e44: field type must be a path")}
            }
        };
    let generate_concrete_standard_non_null_pg_type_role_token_stream =
        |field_type: &macros_helpers::field_data::SynFieldType, role: &dyn quote::ToTokens| {
            if let syn::Type::Path(type_path) = &**field_type {
                let mut role_type_path = type_path.clone();
                if let Some(last_segment) = role_type_path.path.segments.last_mut() {
                    let identifier_string = last_segment.ident.to_string();
                    let without_optional = identifier_string
                        .strip_prefix(str_constants::OPTIONAL)
                        .map_or(identifier_string.as_str(), |value| value);
                    last_segment.ident = quote::format_ident!(
                        "{}{}",
                        without_optional.replace("AsNullable", "AsNonNull"),
                        role.to_token_stream().to_string()
                    );
                    quote::quote! {#role_type_path}
                } else {
                    quote::quote! {compile_error!("bf1ea32c: field type path has no segments")}
                }
            } else {
                quote::quote! {compile_error!("f3f174d3: field type must be a path")}
            }
        };
    let generate_as_pg_type_test_cases_path_token_stream = |ts: &dyn quote::ToTokens| {
        quote::quote! {<#ts as #import_token_stream PgTypeTestCases>::}
    };
    let primary_key_as_pg_type_test_cases_path_token_stream =
        generate_as_pg_type_test_cases_path_token_stream(&primary_key_field_type);
    let generate_as_pg_type_create_token_stream = |ts: &dyn quote::ToTokens| {
        generate_as_pg_type_tokens_token_stream(&ts, &CreateUpperCamelCase)
    };
    let generate_as_pg_type_select_token_stream = |ts: &dyn quote::ToTokens| {
        generate_as_pg_type_tokens_token_stream(&ts, &SelectUpperCamelCase)
    };
    let primary_key_field_type_as_pg_type_select_token_stream =
        generate_as_pg_type_select_token_stream(&primary_key_field_type);
    let generate_as_pg_type_where_token_stream = |ts: &dyn quote::ToTokens| {
        generate_as_pg_type_tokens_token_stream(&ts, &WhereUpperCamelCase)
    };
    let primary_key_field_type_as_pg_type_where_token_stream =
        generate_as_pg_type_where_token_stream(&primary_key_field_type);
    let generate_as_pg_type_read_token_stream = |ts: &dyn quote::ToTokens| {
        generate_as_pg_type_tokens_token_stream(&ts, &ReadUpperCamelCase)
    };
    let generate_as_pg_type_read_ids_token_stream = |ts: &dyn quote::ToTokens| {
        generate_as_pg_type_tokens_token_stream(&ts, &ReadIdsUpperCamelCase)
    };
    let primary_key_field_type_as_pg_type_read_token_stream =
        generate_as_pg_type_read_token_stream(&primary_key_field_type);
    let generate_as_pg_type_update_token_stream = |ts: &dyn quote::ToTokens| {
        generate_as_pg_type_tokens_token_stream(&ts, &UpdateUpperCamelCase)
    };
    let generate_as_pg_type_update_for_query_token_stream = |ts: &dyn quote::ToTokens| {
        generate_as_pg_type_tokens_token_stream(&ts, &UpdateForQueryUpperCamelCase)
    };
    let identifier_read_ids_upper_camel_case =
        naming::parameter::SelfReadIdsUpperCamelCase::from_tokens(&identifier);
    let identifier_dm_parameters_upper_camel_case =
        naming::parameter::SelfDmParametersUpperCamelCase::from_tokens(&identifier);
    let identifier_dm_payload_upper_camel_case =
        naming::parameter::SelfDmPayloadUpperCamelCase::from_tokens(&identifier);
    let identifier_dlo_parameters_upper_camel_case =
        naming::parameter::SelfDloParametersUpperCamelCase::from_tokens(&identifier);
    let identifier_dlo_payload_upper_camel_case =
        naming::parameter::SelfDloPayloadUpperCamelCase::from_tokens(&identifier);
    let identifier_try_ro_error_upper_camel_case =
        naming::parameter::SelfTryRoErrorUpperCamelCase::from_tokens(&identifier);
    let identifier_ro_error_with_serde_upper_camel_case =
        naming::parameter::SelfRoErrorWithSerdeUpperCamelCase::from_tokens(&identifier);
    let identifier_try_dlo_error_upper_camel_case =
        naming::parameter::SelfTryDloErrorUpperCamelCase::from_tokens(&identifier);
    let identifier_dlo_error_with_serde_upper_camel_case =
        naming::parameter::SelfDloErrorWithSerdeUpperCamelCase::from_tokens(&identifier);
    let vec_primary_key_field_type_read_token_stream =
        pg_crud_macros_common::generate_vec_tokens_declaration_token_stream(
            &primary_key_field_type_as_pg_type_read_upper_camel_case,
        );
    let vec_identifier_read_ids_token_stream =
        pg_crud_macros_common::generate_vec_tokens_declaration_token_stream(
            &identifier_read_ids_upper_camel_case,
        );
    let primary_key_field_identifier = &primary_key_field.identifier;
    let primary_key_field_upper_camel_case_token_stream =
        naming_common::ToTokensToUpperCamelCaseTokenStream::case_or_panic(
            &primary_key_field_identifier,
        );
    let primary_key_field_type_update_token_stream =
        &naming::parameter::SelfUpdateUpperCamelCase::from_type_last_segment(
            primary_key_field_type,
        );
    let primary_key_field_type_update_for_query_token_stream =
        &naming::parameter::SelfUpdateForQueryUpperCamelCase::from_type_last_segment(
            primary_key_field_type,
        );
    let identifier_select_upper_camel_case =
        naming::parameter::SelfSelectUpperCamelCase::from_tokens(&identifier);
    let generate_from_handle_token_stream =
        |identifier_token_stream: &dyn quote::ToTokens, ts: &dyn quote::ToTokens| {
            quote::quote! {
                fn #FromHSnakeCase(#VSnakeCase: #identifier_token_stream) -> Self {
                    #ts
                }
            }
        };
    let generate_select_pg_crud_not_empty_unique_vec_identifier_select_token_stream =
        |add_borrow: &AddBorrow| {
            quote::quote! {#SelectSnakeCase: #add_borrow #import_token_stream NotEmptyUniqueVec<#identifier_select_upper_camel_case>}
        };
    let select_borrow_pg_crud_not_empty_unique_vec_identifier_select_token_stream =
        generate_select_pg_crud_not_empty_unique_vec_identifier_select_token_stream(
            &AddBorrow::True,
        );
    let select_pg_crud_not_empty_unique_vec_identifier_select_token_stream =
        generate_select_pg_crud_not_empty_unique_vec_identifier_select_token_stream(
            &AddBorrow::False,
        );
    let pub_select_pg_crud_not_empty_unique_vec_identifier_select_token_stream = {
        quote::quote! {pub #select_pg_crud_not_empty_unique_vec_identifier_select_token_stream}
    };
    let generate_fields_named_with_comma_token_stream: &dyn Fn(
        &dyn Fn(&macros_helpers::field_data::SynField) -> proc_macro2::TokenStream,
    ) -> proc_macro2::TokenStream = &|fn0| -> proc_macro2::TokenStream {
        let fields_token_stream = fields.iter().map(fn0);
        quote::quote! {#(#fields_token_stream),*}
    };
    let generate_fields_named_without_comma_token_stream: &dyn Fn(
        &dyn Fn(&macros_helpers::field_data::SynField) -> proc_macro2::TokenStream,
    ) -> proc_macro2::TokenStream = &|fn0| -> proc_macro2::TokenStream {
        let fields_token_stream = fields.iter().map(fn0);
        quote::quote! {#(#fields_token_stream)*}
    };
    let generate_fields_named_without_primary_key_with_comma_token_stream: &dyn Fn(
        &dyn Fn(&macros_helpers::field_data::SynField) -> proc_macro2::TokenStream,
    ) -> proc_macro2::TokenStream = &|fn0| -> proc_macro2::TokenStream {
        let fields_token_stream = fields_without_primary_key_iter().map(fn0);
        quote::quote! {#(#fields_token_stream),*}
    };
    let generate_fields_named_without_primary_key_without_comma_token_stream: &dyn Fn(
        &dyn Fn(&macros_helpers::field_data::SynField) -> proc_macro2::TokenStream,
    ) -> proc_macro2::TokenStream = &|fn0| -> proc_macro2::TokenStream {
        let fields_token_stream = fields_without_primary_key_iter().map(fn0);
        quote::quote! {#(#fields_token_stream)*}
    };
    let generate_read_fields_with_comma_token_stream =
        |fn0: &dyn Fn(&macros_helpers::field_data::SynField) -> proc_macro2::TokenStream| {
            let fields_token_stream = read_fields_iter().map(fn0);
            quote::quote! {#(#fields_token_stream),*}
        };
    let generate_read_fields_without_primary_key_with_comma_token_stream =
        |fn0: &dyn Fn(&macros_helpers::field_data::SynField) -> proc_macro2::TokenStream| {
            let fields_token_stream = read_fields_without_primary_key_iter().map(fn0);
            quote::quote! {#(#fields_token_stream),*}
        };
    let generate_read_fields_without_primary_key_without_comma_token_stream =
        |fn0: &dyn Fn(&macros_helpers::field_data::SynField) -> proc_macro2::TokenStream| {
            let fields_token_stream = read_fields_without_primary_key_iter().map(fn0);
            quote::quote! {#(#fields_token_stream)*}
        };
    let generate_read_fields_without_comma_token_stream =
        |fn0: &dyn Fn(&macros_helpers::field_data::SynField) -> proc_macro2::TokenStream| {
            let fields_token_stream = read_fields_iter().map(fn0);
            quote::quote! {#(#fields_token_stream)*}
        };
    let generate_match_ok_err_token_stream =
        |ts0: &dyn quote::ToTokens,
         ts1: &dyn quote::ToTokens,
         ts2: &dyn quote::ToTokens,
         ts3: &dyn quote::ToTokens,
         ts4: &dyn quote::ToTokens| {
            quote::quote! {
                match #ts0 {
                    Ok(#ts1) => #ts2,
                    Err(#ts3) => #ts4
                }
            }
        };
    let generate_match_ok_err_short_token_stream =
        |expr: &dyn quote::ToTokens,
         ok: &dyn quote::ToTokens,
         err_token_stream: &dyn quote::ToTokens| {
            generate_match_ok_err_token_stream(
                &expr,
                &ok,
                &ok,
                &Error0,
                &quote::quote! {{ #err_token_stream }},
            )
        };
    let none_token_stream = quote::quote! {None};
    let fields_named_with_comma_none_token_stream =
        generate_fields_named_with_comma_token_stream(&|_| -> proc_macro2::TokenStream {
            none_token_stream.clone()
        });
    let read_fields_with_comma_none_token_stream =
        generate_read_fields_with_comma_token_stream(&|_| -> proc_macro2::TokenStream {
            none_token_stream.clone()
        });
    let fields_named_without_primary_key_with_comma_none_token_stream =
        generate_fields_named_without_primary_key_with_comma_token_stream(
            &|_| -> proc_macro2::TokenStream { none_token_stream.clone() },
        );
    let generate_accumulator_string_pop_token_stream =
        |accumulator_token_stream: &dyn quote::ToTokens, ts: &dyn quote::ToTokens| {
            let optional_char_token_stream =
                pg_crud_macros_common::generate_optional_type_declaration_token_stream(&Char);
            quote::quote! {
                let mut #accumulator_token_stream = #StringTokenStream::new();
                #ts
                let _: #optional_char_token_stream = #accumulator_token_stream.pop();
            }
        };
    let generate_accumulator_string_pop_accumulator_token_stream =
        |accumulator_token_stream: &dyn quote::ToTokens, ts: &dyn quote::ToTokens| {
            let ts0 = generate_accumulator_string_pop_token_stream(accumulator_token_stream, ts);
            quote::quote! {
                #ts0
                #accumulator_token_stream
            }
        };
    let generate_accumulator_string_pop_ok_accumulator_token_stream =
        |accumulator_token_stream: &dyn quote::ToTokens, ts: &dyn quote::ToTokens| {
            let ts0 = generate_accumulator_string_pop_token_stream(accumulator_token_stream, ts);
            quote::quote! {
                #ts0
                Ok(#import_token_stream QueryPartFragment::try_from(#accumulator_token_stream).unwrap_or_else(#import_token_stream QueryPartFragment::from))
            }
        };
    let operation_count = 8usize;
    let mut impl_identifier_vec_token_stream =
        Vec::with_capacity(operation_count.saturating_add(2));
    let mut operation_routes_token_stream = Vec::with_capacity(operation_count);
    let mut content_token_stream = Vec::with_capacity(operation_count);
    let mut api_client_methods_token_stream = Vec::with_capacity(operation_count);
    let mut frontend_api_client_methods_token_stream = Vec::with_capacity(operation_count);
    let client_snake_case = quote::format_ident!("client");
    let mut open_api_path_fn_identifiers = Vec::with_capacity(operation_count);
    let mut open_api_path_token_stream = Vec::with_capacity(operation_count);
    let mut open_api_schema_types_token_stream =
        Vec::with_capacity(operation_count.saturating_mul(2));
    let error_enum_d_token_stream_builder =
        pg_crud_macros_common::token_stream_helpers::error_enum_d_token_stream_builder();
    let serde_ser_utoipa_d_token_stream_builder =
        macros_helpers::derive_token_stream_builder::DTokenStreamBuilder::new()
            .make_pub()
            .d_debug()
            .d_serde_serialize()
            .d_utoipa_to_schema();
    let identifier_prep_pg_error_upper_camel_case =
        naming::parameter::SelfPrepPgErrorUpperCamelCase::from_tokens(&identifier);
    let prep_idempotency_upper_camel_case = quote::format_ident!("PrepIdempotency");
    let identifier_prep_pg_error_token_stream =
        pg_crud_macros_common::token_stream_helpers::error_enum_d_token_stream_builder()
            .build_enum(
                &proc_macro2::TokenStream::new(),
                &identifier_prep_pg_error_upper_camel_case,
                &proc_macro2::TokenStream::new(),
                &{
                    let ts = quote::quote! {
                        #[eo_to_err_string]
                        error: sqlx::Error,
                        location: location_lib::location::Location,
                    };
                    quote::quote! {{
                        #CreateExtensionIfNotExistsUuidOsspUpperCamelCase {
                            #ts
                        },
                        #PrepPgUpperCamelCase {
                            #ts
                        },
                        #prep_idempotency_upper_camel_case {
                            #[eo_to_err_string]
                            error: pg_table::SqlxPgTableIdempotencyError,
                            location: location_lib::location::Location,
                        },
                    }}
                },
            );
    impl_identifier_vec_token_stream.push({
        let frontend_page_path_double_quoted_token_stream = generate_quotes::dq_token_stream(&format!("/{identifier_snake_case_string}"));
        let frontend_page_title = identifier_snake_case_string
            .split('_')
            .enumerate()
            .fold(String::with_capacity(identifier_snake_case_string.len()), |mut title, (index, part)| {
                if index > 0usize {
                    title.push_str(str_constants::SPACE);
                }
                let mut chars = part.chars();
                if let Some(first) = chars.next() {
                    title.extend(first.to_uppercase());
                    title.extend(chars);
                }
                title
            });
        let frontend_page_title_double_quoted_token_stream = generate_quotes::dq_token_stream(&frontend_page_title);
        let pub_fn_table_token_stream = quote::quote! {
            #MustUse
            pub const fn #TableNameSnakeCase() -> &'static str {
                #identifier_snake_case_double_quoted_token_stream
            }
            const fn #db_table_name_snake_case() -> &'static str {
                #db_table_name_double_quoted_token_stream
            }
        };
        let pub_fn_frontend_fields_token_stream = quote::quote! {
            #[must_use]
            pub fn frontend_fields() -> frontend_contract::FieldContracts {
                frontend_contract::FieldContracts::from_max_iter(vec![#(#frontend_field_contracts_token_stream),*])
            }
        };
        let frontend_filter_value_arms_token_stream = read_fields_iter().map(|field| {
            let field_name = field.identifier.to_string();
            let field_name_double_quoted_token_stream = generate_quotes::dq_token_stream(&field_name);
            let field_type = &field.type0;
            quote::quote! {
                #field_name_double_quoted_token_stream => Some(
                    <#field_type as frontend_contract::FilterFormValueContract>::parse_filter_form_value(value)
                )
            }
        });
        let pub_fn_frontend_filter_value_token_stream = quote::quote! {
            #[must_use]
            pub fn frontend_filter_value(
                field: frontend_contract::FormFieldNameRef<'_>,
                value: frontend_contract::FormValueRef<'_>,
            ) -> Option<Result<frontend_contract::FilterWireJson, frontend_contract::FormValueError>> {
                match field.as_ref() {
                    #(#frontend_filter_value_arms_token_stream),*,
                    _ => None,
                }
            }
        };
        let pub_fn_frontend_page_token_stream = quote::quote! {
            #[must_use]
            pub fn frontend_page() -> frontend_contract::PageContract {
                frontend_contract::PageContract::new(
                    #identifier_route_contract_upper_camel_case::frontend_actions(),
                    Self::frontend_fields(),
                    frontend_contract::ContractStr::from(#frontend_page_path_double_quoted_token_stream),
                    #identifier_route_contract_upper_camel_case::frontend_contracts(),
                    frontend_contract::ContractStr::from(#frontend_page_title_double_quoted_token_stream),
                )
            }
        };
        let fn_primary_key_token_stream = {
            let primary_key_field_double_quoted_token_stream = generate_quotes::dq_token_stream(&primary_key_field_identifier);
            quote::quote! {
                const fn #PrimaryKeySnakeCase() -> &'static str {
                    #primary_key_field_double_quoted_token_stream
                }
            }
        };
        let pub_async_fn_prep_extensions_token_stream = quote::quote! {
            pub async fn #PrepExtensionsSnakeCase(#PoolSnakeCase: &sqlx::Pool<sqlx::Postgres>) -> Result<(), #identifier_prep_pg_error_upper_camel_case> {
                if let Err(error) = sqlx::query("create extension if not exists \"uuid-ossp\"").execute(#PoolSnakeCase).await {
                    return Err(#identifier_prep_pg_error_upper_camel_case::#CreateExtensionIfNotExistsUuidOsspUpperCamelCase {
                        error,
                        location: location_macros::location!()
                    });
                }
                Ok(())
            }
        };
        let pub_async_fn_prep_pg_table_token_stream = {
            let prep_pg_cols_fmt = fields.iter().enumerate().fold(
                String::with_capacity(fields.len().saturating_mul(3)),
                |mut accumulator, (idx, _)| {
                    if idx != 0 {
                        accumulator.push(',');
                    }
                    accumulator.push_str(str_constants::TEXT_ALT_14);
                    accumulator
                },
            );
            let prep_pg_double_quoted_token_stream = generate_quotes::dq_token_stream(&format!(
                "create table if not exists {{table}} ({prep_pg_cols_fmt})"
            ));
            let generate_field_type_as_pg_crud_create_table_column_query_part_create_table_query_part_token_stream = |field_type, field, is_primary_key| {
                let is_primary_key_token_stream: &dyn quote::ToTokens = if is_primary_key { &TrueSnakeCase } else { &FalseSnakeCase };
                let field_double_quoted_token_stream = generate_quotes::dq_token_stream(&field);
                let field_type_pg_type_token_stream = generate_as_pg_type_path_token_stream(&field_type);
                quote::quote! {
                    #field_type_pg_type_token_stream #CreateTableColumnQueryPartSnakeCase(#import_token_stream SqlColumnRef::from(&#field_double_quoted_token_stream), #import_token_stream IsPrimaryKey::from(#is_primary_key_token_stream))
                }
            };
            let serde_json_to_string_schemars_schema_for_generic_unwrap_token_stream = std::iter::once(
                generate_field_type_as_pg_crud_create_table_column_query_part_create_table_query_part_token_stream(primary_key_field_type, &primary_key_field.identifier, true),
            )
            .chain(fields_without_primary_key_iter().map(|element| {
                generate_field_type_as_pg_crud_create_table_column_query_part_create_table_query_part_token_stream(&element.type0, &element.identifier, false)
            }));
            quote::quote! {
                pub async fn #PrepPgTableSnakeCase(#PoolSnakeCase: &sqlx::Pool<sqlx::Postgres>, table: &str) -> Result<(), #identifier_prep_pg_error_upper_camel_case> {
                    if let Err(error) = sqlx::query(sqlx::AssertSqlSafe(format!(
                        #prep_pg_double_quoted_token_stream,
                        #(#serde_json_to_string_schemars_schema_for_generic_unwrap_token_stream),*
                    ))).execute(#PoolSnakeCase).await {
                        return Err(#identifier_prep_pg_error_upper_camel_case::#PrepPgUpperCamelCase {
                            error,
                            location: location_macros::location!()
                        });
                    }
                    Ok(())
                }
            }
        };
        let prep_idempotency_token_stream = if generate_pg_table_input_model.config.idempotent_mutations {
            quote::quote! {
                if let Err(error) = pg_table::ensure_pg_table_idempotency_schema(app_state::SqlxPgPoolRef::from(#PoolSnakeCase)).await {
                    return Err(#identifier_prep_pg_error_upper_camel_case::#prep_idempotency_upper_camel_case {
                        error,
                        location: location_macros::location!(),
                    });
                }
            }
        } else {
            proc_macro2::TokenStream::new()
        };
        let pub_async_fn_prep_pg_token_stream = quote::quote! {
            pub async fn #PrepPgSnakeCase(#PoolSnakeCase: &sqlx::Pool<sqlx::Postgres>) -> Result<(), #identifier_prep_pg_error_upper_camel_case> {
                Self::#PrepExtensionsSnakeCase(#PoolSnakeCase).await?;
                Self::#PrepPgTableSnakeCase(#PoolSnakeCase, #db_table_name_double_quoted_token_stream).await?;
                #prep_idempotency_token_stream
                Ok(())
            }
        };
        let pub_fn_allow_methods_token_stream = {
            let http_method_token_stream = quote::quote! {http::Method};
            quote::quote! {
                #MustUse
                pub const fn allow_methods() -> [#http_method_token_stream;4] {[
                    #http_method_token_stream::GET,
                    #http_method_token_stream::POST,
                    #http_method_token_stream::PATCH,
                    #http_method_token_stream::DELETE
                ]}
            }
        };
        let fn_generate_select_query_part_token_stream = {
            let vrts_token_stream = generate_read_fields_with_comma_token_stream(&|element: &macros_helpers::field_data::SynField| {
                let field_upper_camel_case_token_stream = naming_common::ToTokensToUpperCamelCaseTokenStream::case_or_panic(&element.identifier);
                let initialization_token_stream = {
                    let field_string_double_quoted_token_stream = generate_quotes::dq_token_stream(&element.identifier);
                    let as_pg_crud_pg_type_pg_type_token_stream = generate_as_pg_type_path_token_stream(&element.type0);
                    let ts0 = generate_match_ok_err_short_token_stream(
                        &quote::quote! {#as_pg_crud_pg_type_pg_type_token_stream #SelectQueryPartSnakeCase(
                            #ColumnSnakeCase,
                            #import_token_stream SqlColumnRef::from(&#field_string_double_quoted_token_stream)
                        )},
                        &quote::quote! {v_820e1163},
                        &quote::quote! {{
                            return Err(#Error0);
                        }},
                    );
                    quote::quote! {=> #ts0}
                };
                quote::quote! {#identifier_select_upper_camel_case::#field_upper_camel_case_token_stream(#ColumnSnakeCase) #initialization_token_stream}
            });
            let ts0 = generate_accumulator_string_pop_ok_accumulator_token_stream(
                &quote::quote! {accumulator},
                &quote::quote! {
                    for element in #SelectSnakeCase.as_slice() {
                        accumulator.push_str(&match element {
                            #vrts_token_stream
                        });
                        accumulator.push(',');
                    }
                },
            );
            quote::quote! {
                fn #GenerateSelectQueryPartSnakeCase(#select_borrow_pg_crud_not_empty_unique_vec_identifier_select_token_stream) -> Result<#import_token_stream QueryPartFragment, #import_token_stream #QueryPartErrorUpperCamelCase> {
                    #ts0
                }
            }
        };
        quote::quote! {
            #pub_fn_table_token_stream
            #pub_fn_frontend_fields_token_stream
            #pub_fn_frontend_filter_value_token_stream
            #pub_fn_frontend_page_token_stream
            #fn_primary_key_token_stream
            #pub_async_fn_prep_extensions_token_stream
            #pub_async_fn_prep_pg_table_token_stream
            #pub_async_fn_prep_pg_token_stream
            #pub_fn_allow_methods_token_stream
            #fn_generate_select_query_part_token_stream
        }
    });
    let wrap_into_axum_res_token_stream =
        |axum_json_token_stream: &dyn quote::ToTokens,
         status_code_token_stream: &dyn quote::ToTokens,
         add_return: &AddReturn| {
            let return_token_stream = match add_return {
                AddReturn::False => quote::quote! {res},
                AddReturn::True => quote::quote! {return res;},
            };
            quote::quote! {
                let mut res = axum::response::IntoResponse::into_response(
                    axum::Json(#axum_json_token_stream)
                );
                *res.status_mut() = #status_code_token_stream;
                #return_token_stream
            }
        };
    let generate_identifier_operation_suffix_token_stream: &dyn Fn(
        &Operation,
        &str,
    )
        -> proc_macro2::TokenStream = &|operation, suffix| {
        let identifier_operation_suffix = quote::format_ident!("{identifier}{operation}{suffix}");
        quote::quote! {#identifier_operation_suffix}
    };
    let generate_identifier_operation_error_upper_camel_case = |operation: &Operation| {
        generate_identifier_operation_suffix_token_stream(operation, str_constants::ERROR)
    };
    let generate_identifier_operation_res_variants_upper_camel_case = |operation: &Operation| {
        generate_identifier_operation_suffix_token_stream(operation, str_constants::RESVARIANTS)
    };
    let generate_initialization_token_stream: &dyn Fn(
        &SynVariant,
        &'static std::panic::Location<'_>,
    ) -> proc_macro2::TokenStream = &|syn_variant, location| -> proc_macro2::TokenStream {
        let variant_identifier = &syn_variant.variant.ident;
        let fields_token_stream = if let syn::Fields::Named(v) = &syn_variant.variant.fields {
            v.named.iter().enumerate().map(|(i, element)| {
                let field = &element.ident;
                let Some(field_ref) = field.as_ref() else {
                    return compile_error_token_stream(CompileErrorMessage::from(
                        str_constants::COMPILE_ERROR_CE_054,
                    ))
                    .into();
                };
                if *field_ref == LocationSnakeCase.to_string() {
                    macros_helpers::generate_field_location_new_token_stream::generate_field_location_new_token_stream(
                        macros_helpers::generate_field_location_new_token_stream::FieldLocationFile::from(location.file()),
                        macros_helpers::generate_field_location_new_token_stream::FieldLocationLine::try_from(location.line())
                            .unwrap_or_else(|_error| macros_helpers::generate_field_location_new_token_stream::FieldLocationLine::first()),
                        macros_helpers::generate_field_location_new_token_stream::FieldLocationColumn::try_from(location.column())
                            .unwrap_or_else(|_error| macros_helpers::generate_field_location_new_token_stream::FieldLocationColumn::first()),
                    )
                    .into()
                } else {
                    let error_increment_snake_case = naming::parameter::ErrorSelfSnakeCase::from_display(&i);
                    quote::quote! {#field: #error_increment_snake_case}
                }
            })
        } else {
            return compile_error_token_stream(CompileErrorMessage::from(
                str_constants::COMPILE_ERROR_CE_001,
            ))
            .into();
        };
        quote::quote! {
            #variant_identifier {
                #(#fields_token_stream),*
            }
        }
    };
    let generate_operation_error_initialization_eprintln_res_token_stream: &dyn Fn(
        &Operation,
        &SynVariant,
        &'static std::panic::Location<'_>,
    ) -> proc_macro2::TokenStream = &|operation, syn_variant, location| -> proc_macro2::TokenStream {
        let identifier_operation_error_upper_camel_case = generate_identifier_operation_error_upper_camel_case(operation);
        let identifier_operation_res_variants_upper_camel_case = generate_identifier_operation_res_variants_upper_camel_case(operation);
        let syn_variant_initialization_token_stream = generate_initialization_token_stream(syn_variant, location);
        let ts = wrap_into_axum_res_token_stream(
            &quote::quote! {#identifier_operation_res_variants_upper_camel_case::#FromHSnakeCase(#ErrorSnakeCase)},
            &match syn_variant.get_optional_status_code() {
                Some(v) => v.to_http_status_code_token_stream(),
                None => {
                    return compile_error_token_stream(CompileErrorMessage::from(
                        str_constants::COMPILE_ERROR_CE_019,
                    ))
                    .into();
                }
            },
            &AddReturn::True,
        );
        quote::quote! {
            let #ErrorSnakeCase = #identifier_operation_error_upper_camel_case::#syn_variant_initialization_token_stream;
            #ts
        }
    };
    // This generated-code helper stays next to its only call sites inside the generation scope.
    #[allow(clippy::items_after_statements)]
    fn new_syn_variant<DisplayValue>(
        variant_name: &dyn std::fmt::Display,
        status_code: Option<macros_helpers::status_code::StatusCode>,
        variant_fields: impl IntoIterator<
            Item = (
                macros_helpers::location_data::LocationFieldAttr,
                DisplayValue,
                macros_helpers::generate_simple_syn_punct::SynPathSegments,
            ),
        >,
        is_location_first: bool,
    ) -> SynVariant
    where
        DisplayValue: std::fmt::Display,
    {
        SynVariant {
            variant: syn::Variant {
                attrs: {
                    let mut attrs = Vec::with_capacity(1);
                    if let Some(v) = status_code.as_ref() {
                        let mut segments = syn::punctuated::Punctuated::new();
                        segments.push(syn::PathSegment {
                            ident: syn::Ident::new(
                                &naming_common::AsRefStrToSnakeCaseStr::case(v),
                                proc_macro2::Span::call_site(),
                            ),
                            arguments: syn::PathArguments::None,
                        });
                        attrs.push(syn::Attribute {
                            pound_token: syn::token::Pound {
                                spans: [proc_macro2::Span::call_site()],
                            },
                            style: syn::AttrStyle::Outer,
                            bracket_token: syn::token::Bracket::default(),
                            meta: syn::Meta::Path(syn::Path {
                                leading_colon: None,
                                segments,
                            }),
                        });
                    }
                    attrs
                },
                ident: syn::Ident::new(&variant_name.to_string(), proc_macro2::Span::call_site()),
                fields: syn::Fields::Named(syn::FieldsNamed {
                    brace_token: syn::token::Brace::default(),
                    named: {
                        let initial_fields = if is_location_first {
                            let mut named_fields_accumulator = syn::punctuated::Punctuated::new();
                            named_fields_accumulator.push_value(
                                macros_helpers::location_syn_field::location_syn_field().into(),
                            );
                            named_fields_accumulator.push_punct(syn::token::Comma {
                                spans: [proc_macro2::Span::call_site()],
                            });
                            named_fields_accumulator
                        } else {
                            syn::punctuated::Punctuated::new()
                        };
                        let mut named_fields_accumulator = variant_fields.into_iter().fold(
                            initial_fields,
                            |mut named_fields_accumulator, element| {
                                named_fields_accumulator.push_value(syn::Field {
                                    attrs: vec![syn::Attribute {
                                        pound_token: syn::token::Pound {
                                            spans: [proc_macro2::Span::call_site()],
                                        },
                                        style: syn::AttrStyle::Outer,
                                        bracket_token: syn::token::Bracket::default(),
                                        meta: syn::Meta::Path(syn::Path {
                                            leading_colon: None,
                                            segments: {
                                                let mut acc0 = syn::punctuated::Punctuated::new();
                                                acc0.push(syn::PathSegment {
                                                    ident: syn::Ident::new(
                                                        macros_helpers::attr_identifier_str::AttrIdentifierStr::attr_identifier_str(&element.0).as_ref(),
                                                        proc_macro2::Span::call_site(),
                                                    ),
                                                    arguments: syn::PathArguments::None,
                                                });
                                                acc0
                                            },
                                        }),
                                    }],
                                    vis: syn::Visibility::Inherited,
                                    modifiers: syn::FieldModifiers::default(),
                                    ident: Some(syn::Ident::new(
                                        &element.1.to_string(),
                                        proc_macro2::Span::call_site(),
                                    )),
                                    colon_token: Some(syn::token::Colon {
                                        spans: [proc_macro2::Span::call_site()],
                                    }),
                                    default: None,
                                    ty: syn::Type::Path(syn::TypePath {
                                        attrs: Vec::new(),
                                        qself: None,
                                        path: syn::Path {
                                            leading_colon: None,
                                            segments: element.2.into(),
                                        },
                                    }),
                                });
                                named_fields_accumulator.push_punct(syn::token::Comma {
                                    spans: [proc_macro2::Span::call_site()],
                                });
                                named_fields_accumulator
                            },
                        );
                        if !is_location_first {
                            named_fields_accumulator.push_value(
                                macros_helpers::location_syn_field::location_syn_field().into(),
                            );
                        }
                        named_fields_accumulator
                    },
                }),
                discriminant: None,
            },
            status_code,
        }
    }
    let query_part_syn_variant = new_syn_variant(
        &QueryPartUpperCamelCase,
        Some(macros_helpers::status_code::StatusCode::BadReq400),
        vec![(
            macros_helpers::location_data::LocationFieldAttr::EoLocation,
            &ErrorSnakeCase,
            macros_helpers::generate_simple_syn_punct::generate_simple_syn_punct([
                &PgCrudSnakeCase.to_owned(),
                &QueryPartErrorUpperCamelCase.to_string(),
            ]),
        )],
        false,
    );
    let generate_select_query_part_parameters_payload_select_token_stream =
        |operation: &Operation| {
            generate_match_ok_err_short_token_stream(
                &quote::quote! {Self::#GenerateSelectQueryPartSnakeCase(&#ParametersSnakeCase.#PayloadSnakeCase.#SelectSnakeCase)},
                &quote::quote! {v_357219fb},
                &{
                    let ts = generate_operation_error_initialization_eprintln_res_token_stream(
                        operation,
                        &query_part_syn_variant,
                        std::panic::Location::caller(),
                    );
                    quote::quote! {{#ts}}
                },
            )
        };
    let identifier_read_upper_camel_case =
        naming::parameter::SelfReadUpperCamelCase::from_tokens(&identifier);
    let generate_v_declaration_ts0 = |ts: &dyn quote::ToTokens| {
        pg_crud_macros_common::generate_v_declaration_token_stream(&import, &ts)
    };
    let generate_v_initialization_ts0 = |ts: &dyn quote::ToTokens| {
        pg_crud_macros_common::generate_v_initialization_token_stream(&import, &ts)
    };
    let generate_impl_pg_crud_default_some_one_element_for_tokens_no_lt_token_stream =
        |impl_identifier: &dyn quote::ToTokens, ts: &dyn quote::ToTokens| {
            pg_crud_macros_common::generate_impl_pg_crud_default_some_one_element_token_stream(
                &impl_identifier,
                &proc_macro2::TokenStream::new(),
                &ts,
            )
        };
    let generate_field_default_some_one_element_call_token_stream =
        |ts: &dyn quote::ToTokens| quote::quote! {#ts: #PgCrudCommonDefaultSomeOneElementCall};
    let generate_match_query_bind_or_err_token_stream =
        |expr: &dyn quote::ToTokens,
         ok_binding: &dyn quote::ToTokens,
         err_token_stream: &dyn quote::ToTokens| {
            generate_match_ok_err_token_stream(
                &expr,
                &ok_binding,
                &quote::quote! {{
                    #QuerySnakeCase = #ok_binding;
                }},
                &Error0,
                &quote::quote! {{#err_token_stream}},
            )
        };
    let generate_if_let_some_token_stream =
        |ts0: &dyn quote::ToTokens, ts1: &dyn quote::ToTokens, ts2: &dyn quote::ToTokens| {
            quote::quote! {
                if let Some(#ts0) = #ts1 {
                    #ts2
                }
            }
        };
    let identifier_create_upper_camel_case =
        naming::parameter::SelfCreateUpperCamelCase::from_tokens(&identifier);
    let identifier_create_token_stream = {
        let identifier_create_token_stream =
            macros_helpers::derive_token_stream_builder::DTokenStreamBuilder::new()
                .make_pub()
                .d_debug()
                .d_clone()
                .d_serde_serialize()
                .d_serde_deserialize()
                .d_utoipa_to_schema()
                .build_struct(
                    &quote::quote! {#[serde(deny_unknown_fields)]},
                    &identifier_create_upper_camel_case,
                    &proc_macro2::TokenStream::new(),
                    &{
                        let ts = create_fields_without_primary_key_iter().map(
                            |element: &macros_helpers::field_data::SynField| {
                                let field = &element.identifier;
                                let element_syn_field_ty_as_pg_type_create_token_stream =
                                    generate_as_pg_type_create_token_stream(&element.type0);
                                let concrete_create_token_stream =
                                    generate_concrete_pg_type_role_token_stream(
                                        &element.type0,
                                        &CreateUpperCamelCase,
                                    );
                                quote::quote! {
                                    #[schema(value_type = #concrete_create_token_stream)]
                                    pub #field: #element_syn_field_ty_as_pg_type_create_token_stream
                                }
                            },
                        );
                        quote::quote! {{#(#ts),*}}
                    },
                );
        let impl_identifier_create_token_stream = {
            let primary_key_field_type_as_default_some_one_element_call_token_stream = {
                let primary_key_field_type_as_pg_type_create_token_stream =
                    generate_as_pg_type_create_token_stream(&primary_key_field_type);
                quote::quote! {
                    <
                        #primary_key_field_type_as_pg_type_create_token_stream as #import_token_stream #DefaultSomeOneElementUpperCamelCase
                    >::#DefaultSomeOneElementSnakeCase()
                }
            };
            let fn_create_query_part_token_stream = {
                let generate_match_as_pg_crud_pg_type_pg_type_create_query_part_token_stream: &dyn Fn(
                    &dyn quote::ToTokens,
                    &dyn quote::ToTokens,
                ) -> proc_macro2::TokenStream = &|field_type, ts| {
                        generate_match_ok_err_token_stream(
                            &{
                                let as_pg_crud_pg_type_pg_type_token_stream = generate_as_pg_type_path_token_stream(&field_type);
                                quote::quote! {#as_pg_crud_pg_type_pg_type_token_stream #CreateQueryPartSnakeCase(
                                    &#ts,
                                    #IncrementSnakeCase
                                )}
                            },
                            &quote::quote! {v_c3f0b59a},
                            &{
                                let if_write_is_err_token_stream = macros_helpers::generate_if_write_is_err_token_stream::generate_if_write_is_err_token_stream(
                                    &quote::quote! {accumulator, "{v_c3f0b59a},"},
                                    &return_err_query_part_error_write_into_buffer_token_stream,
                                );
                                quote::quote! {{
                                    #if_write_is_err_token_stream
                                }}
                            },
                            &Error0,
                            &quote::quote! {{
                                return Err(#Error0);
                            }},
                        )
                    };
                let primary_key_token_stream =
                    generate_match_as_pg_crud_pg_type_pg_type_create_query_part_token_stream(
                        primary_key_field_type,
                        &primary_key_field_type_as_default_some_one_element_call_token_stream,
                    );
                let column_incrs_token_stream = fields_without_primary_key_iter()
                    .map(|element: &macros_helpers::field_data::SynField| {
                        let field_value_token_stream = if create_field_is_excluded(element) {
                            let field_type_create_token_stream = generate_as_pg_type_create_token_stream(&element.type0);
                            quote::quote! {<#field_type_create_token_stream as #import_token_stream #DefaultSomeOneElementUpperCamelCase>::#DefaultSomeOneElementSnakeCase()}
                        } else {
                            let element_field = &element.identifier;
                            quote::quote! {self.#element_field}
                        };
                        generate_match_as_pg_crud_pg_type_pg_type_create_query_part_token_stream(&element.type0, &{
                            field_value_token_stream
                        })
                    });
                let ts = generate_accumulator_string_pop_ok_accumulator_token_stream(
                    &quote::quote! {accumulator},
                    &quote::quote! {
                        #primary_key_token_stream
                        #(#column_incrs_token_stream)*
                    },
                );
                quote::quote! {
                    fn #CreateQueryPartSnakeCase(&self, #IncrementSnakeCase: &mut dyn #import_token_stream QueryPartIncrementMut) -> Result<#import_token_stream QueryPartFragment, #import_token_stream #QueryPartErrorUpperCamelCase> {
                        #ts
                    }
                }
            };
            let fn_create_query_bind_token_stream = {
                let generate_query_as_pg_crud_pg_type_pg_type_create_query_bind_token_stream: &dyn Fn(
                    &dyn quote::ToTokens,
                    &dyn quote::ToTokens,
                ) -> proc_macro2::TokenStream = &|field_type, ts| {
                        generate_match_query_bind_or_err_token_stream(
                            &{
                                let as_pg_crud_pg_type_pg_type_token_stream = generate_as_pg_type_path_token_stream(&field_type);
                                quote::quote! {#as_pg_crud_pg_type_pg_type_token_stream #CreateQueryBindSnakeCase(#ts,#QuerySnakeCase)}
                            },
                            &quote::quote! {v_3c55d2e1},
                            &quote::quote! {return Err(#Error0);},
                        )
                    };
                let primary_key_token_stream =
                    generate_query_as_pg_crud_pg_type_pg_type_create_query_bind_token_stream(
                        primary_key_field_type,
                        &primary_key_field_type_as_default_some_one_element_call_token_stream,
                    );
                let binded_query_modifications_token_stream = fields_without_primary_key_iter()
                    .map(|element: &macros_helpers::field_data::SynField| {
                        let field_value_token_stream = if create_field_is_excluded(element) {
                            let field_type_create_token_stream = generate_as_pg_type_create_token_stream(&element.type0);
                            quote::quote! {<#field_type_create_token_stream as #import_token_stream #DefaultSomeOneElementUpperCamelCase>::#DefaultSomeOneElementSnakeCase()}
                        } else {
                            let field = &element.identifier;
                            quote::quote! {self.#field}
                        };
                        generate_query_as_pg_crud_pg_type_pg_type_create_query_bind_token_stream(&element.type0, &{
                            field_value_token_stream
                        })
                    });
                quote::quote! {
                    fn #CreateQueryBindSnakeCase(self, mut #QuerySnakeCase: #import_token_stream SqlxPostgresQuery<'_>) -> Result<#import_token_stream SqlxPostgresQuery<'_>, #import_token_stream SqlxPostgresQueryBindError> {
                        #primary_key_token_stream
                        #(#binded_query_modifications_token_stream)*
                        Ok(#QuerySnakeCase)
                    }
                }
            };
            quote::quote! {
                #AllowClippyArbitrarySrcItemOrdering
                impl #identifier_create_upper_camel_case {
                    #fn_create_query_part_token_stream
                    #fn_create_query_bind_token_stream
                }
            }
        };
        let impl_pg_crud_default_some_one_element_for_identifier_create_token_stream =
            generate_impl_pg_crud_default_some_one_element_for_tokens_no_lt_token_stream(
                &identifier_create_upper_camel_case,
                &{
                    let fields_initialization_without_primary_key_with_default_some_one_element_token_stream = {
                        let create_fields_token_stream = create_fields_without_primary_key_iter()
                            .map(|element: &macros_helpers::field_data::SynField| {
                                generate_field_default_some_one_element_call_token_stream(
                                    &element.identifier,
                                )
                            });
                        quote::quote! {#(#create_fields_token_stream),*}
                    };
                    quote::quote! {
                        Self{#fields_initialization_without_primary_key_with_default_some_one_element_token_stream}
                    }
                },
            );
        quote::quote! {
            #identifier_create_token_stream
            #impl_identifier_create_token_stream
            #impl_pg_crud_default_some_one_element_for_identifier_create_token_stream
        }
    };
    let generate_no_fields_provided_error_token_stream =
        |error_upper_camel_case: &dyn quote::ToTokens| {
            pg_crud_macros_common::token_stream_helpers::error_enum_d_token_stream_builder()
                .build_enum(
                    &proc_macro2::TokenStream::new(),
                    error_upper_camel_case,
                    &proc_macro2::TokenStream::new(),
                    &quote::quote! {{
                        #NoFieldsProvidedUpperCamelCase {
                            #[eo_to_err_string]
                            location: location_lib::location::Location,
                        }
                    }},
                )
        };
    let identifier_where_upper_camel_case =
        naming::parameter::SelfWhereManyUpperCamelCase::from_tokens(&identifier);
    let identifier_where_try_new_error_upper_camel_case =
        naming::parameter::SelfWhereManyTryNewErrorUpperCamelCase::from_tokens(&identifier);
    let identifier_where_token_stream = {
        let fields_schema_declaration_token_stream = generate_read_fields_with_comma_token_stream(
            &|element: &macros_helpers::field_data::SynField| -> proc_macro2::TokenStream {
                let field = &element.identifier;
                let element_syn_field_ty_as_pg_type_where_token_stream =
                    generate_as_pg_type_where_token_stream(&element.type0);
                let concrete_where_token_stream = generate_concrete_pg_type_role_token_stream(
                    &element.type0,
                    &WhereUpperCamelCase,
                );
                let field_type_token_stream =
                    pg_crud_macros_common::generate_optional_type_declaration_token_stream(
                        &quote::quote! {#import_token_stream PgTypeWhere<#element_syn_field_ty_as_pg_type_where_token_stream>},
                    );
                quote::quote! {
                    #[schema(inline, value_type = Option<#import_token_stream PgTypeWhere<#concrete_where_token_stream>>, nullable = false)]
                    #field: #field_type_token_stream
                }
            },
        );
        let fields_declaration_token_stream = generate_read_fields_with_comma_token_stream(
            &|element: &macros_helpers::field_data::SynField| -> proc_macro2::TokenStream {
                let field = &element.identifier;
                let element_syn_field_ty_as_pg_type_where_token_stream =
                    generate_as_pg_type_where_token_stream(&element.type0);
                let optional_pg_type_where_syn_field_ty_as_pg_type_where_token_stream =
                    pg_crud_macros_common::generate_optional_type_declaration_token_stream(
                        &quote::quote! {#import_token_stream PgTypeWhere<#element_syn_field_ty_as_pg_type_where_token_stream>},
                    );
                quote::quote! {
                    #field: #optional_pg_type_where_syn_field_ty_as_pg_type_where_token_stream
                }
            },
        );
        let identifier_where_token_stream = {
            let identifier_where_struct_token_stream =
                macros_helpers::derive_token_stream_builder::DTokenStreamBuilder::new()
                    .make_pub()
                    .d_debug()
                    .d_clone()
                    .d_serde_serialize()
                    .d_utoipa_to_schema()
                    .build_struct(
                        &proc_macro2::TokenStream::new(),
                        &identifier_where_upper_camel_case,
                        &proc_macro2::TokenStream::new(),
                        &quote::quote! {{#fields_schema_declaration_token_stream}},
                    );
            quote::quote! {
                #AllowClippyArbitrarySrcItemOrdering
                #identifier_where_struct_token_stream
            }
        };
        let identifier_where_try_new_error_token_stream =
            generate_no_fields_provided_error_token_stream(
                &identifier_where_try_new_error_upper_camel_case,
            );
        let impl_pub_try_new_for_identifier_where_token_stream =
            macros_helpers::generate_new_or_try_new::generate_impl_pub_try_new_for_identifier_token_stream(
                &proc_macro2::TokenStream::new(),
                &identifier_where_upper_camel_case,
                &fields_declaration_token_stream,
                &identifier_where_try_new_error_upper_camel_case,
                &{
                    let generate_fields_token_stream = |add_borrow: AddBorrow| {
                        generate_read_fields_with_comma_token_stream(
                        &|element: &macros_helpers::field_data::SynField| -> proc_macro2::TokenStream {
                            let field = &element.identifier;
                            quote::quote! {#add_borrow #field}
                        },
                    )
                    };
                    let fields_token_stream = generate_fields_token_stream(AddBorrow::True);
                    let fields_inialization_token_stream = generate_fields_token_stream(AddBorrow::False);
                    quote::quote! {
                        if matches!((#fields_token_stream), (#read_fields_with_comma_none_token_stream)) {
                            return Err(#identifier_where_try_new_error_upper_camel_case::#NoFieldsProvidedUpperCamelCase {
                                location: location_macros::location!(),
                            });
                        }
                        Ok(Self {#fields_inialization_token_stream})
                    }
                },
            );
        let impl_de_for_identifier_where_token_stream =
            pg_crud_macros_common::generate_impl_de_for_struct_by_fields_token_stream(
                &identifier_where_upper_camel_case,
                pg_crud_macros_common::SynFieldRefs::from(read_fields.as_slice()),
                pg_crud_macros_common::DeLen::from(read_fields_len),
                &|_, syn_type| {
                    let syn_type_as_pg_type_where_token_stream =
                        generate_as_pg_type_where_token_stream(&syn_type);
                    pg_crud_macros_common::generate_optional_type_declaration_token_stream(
                        &quote::quote! {#import_token_stream PgTypeWhere<#syn_type_as_pg_type_where_token_stream>},
                    )
                },
            );
        let impl_pg_crud_default_some_one_element_for_identifier_where_token_stream =
            generate_impl_pg_crud_default_some_one_element_for_tokens_no_lt_token_stream(
                &identifier_where_upper_camel_case,
                &{
                    let fields_token_stream = generate_read_fields_without_comma_token_stream(
                        &|element: &macros_helpers::field_data::SynField| {
                            let field = &element.identifier;
                            quote::quote! {
                                #field: Some(
                                    #PgCrudCommonDefaultSomeOneElementCall
                                ),
                            }
                        },
                    );
                    quote::quote! {Self{#fields_token_stream}}
                },
            );
        quote::quote! {
            #identifier_where_token_stream
            #identifier_where_try_new_error_token_stream
            #impl_pub_try_new_for_identifier_where_token_stream
            #impl_de_for_identifier_where_token_stream
            #impl_pg_crud_default_some_one_element_for_identifier_where_token_stream
        }
    };
    let optional_identifier_where_upper_camel_case =
        naming::parameter::StdOptionalOptionalSelfWhereManyUpperCamelCase::from_tokens(&identifier);
    let optional_identifier_where_token_stream = {
        let optional_identifier_where_token_stream = macros_helpers::derive_token_stream_builder::DTokenStreamBuilder::new()
            .make_pub()
            .d_debug()
            .d_clone()
            .d_serde_serialize()
            .d_serde_deserialize()
            .d_utoipa_to_schema()
            .build_struct(
                &proc_macro2::TokenStream::new(),
                &optional_identifier_where_upper_camel_case,
                &proc_macro2::TokenStream::new(),
                &{
                    let optional_identifier_read_ids_standard_non_null_token_stream =
                        pg_crud_macros_common::generate_optional_type_declaration_token_stream(&identifier_where_upper_camel_case);
                    quote::quote! {(#optional_identifier_read_ids_standard_non_null_token_stream);}
                },
            );
        let impl_optional_identifier_where_accessors_token_stream = {
            let optional_identifier_where_inner_token_stream =
                pg_crud_macros_common::generate_optional_type_declaration_token_stream(
                    &identifier_where_upper_camel_case,
                );
            quote::quote! {
                impl #optional_identifier_where_upper_camel_case {
                    #[must_use]
                    pub const fn as_ref(&self) -> Option<&#identifier_where_upper_camel_case> {
                        self.0.as_ref()
                    }
                    #[must_use]
                    pub fn into_option(self) -> #optional_identifier_where_inner_token_stream {
                        self.0
                    }
                }
            }
        };
        let impl_pg_type_where_filter_for_optional_identifier_where_token_stream =
            pg_crud_macros_common::impl_pg_type_where_filter_for_identifier_token_stream(
                &quote::quote! {<'lt>},
                &optional_identifier_where_upper_camel_case,
                &proc_macro2::TokenStream::new(),
                &pg_crud_macros_common::IncrementParameterUndrscr::False,
                &pg_crud_macros_common::ColumnParameterUndrscr::True,
                &pg_crud_macros_common::AddOperatorUndrscr::True,
                &{
                    let extra_parameters_modification_token_stream = read_fields_iter().enumerate().map(|(i, element)| {
                    let field = &element.identifier;
                    generate_if_let_some_token_stream(
                        &quote::quote! {v_da0f0616},
                        &quote::quote! {&#VSnakeCase.#field},
                        &generate_match_ok_err_token_stream(
                            &{
                                let field_double_quoted_token_stream = generate_quotes::dq_token_stream(&field);
                                quote::quote! {#import_token_stream PgTypeWhereFilter::query_part(
                                    v_da0f0616,
                                    increment,
                                    #import_token_stream SqlColumnRef::from(&#field_double_quoted_token_stream),
                                    #import_token_stream AddOperator::from(is_first_push_to_extra_parameters_already_happend),
                                )}
                            },
                            &quote::quote! {v_9e3f8fdd},
                            &{
                                let ts = if i.saturating_add(1usize) == read_fields_len {
                                    proc_macro2::TokenStream::new()
                                } else {
                                    quote::quote! {is_first_push_to_extra_parameters_already_happend = true;}
                                };
                                quote::quote! {{
                                    #ExtraParametersSnakeCase.push_str(&v_9e3f8fdd);
                                    #ts
                                }}
                            },
                            &Error0,
                            &quote::quote! {{
                                return Err(#Error0);
                            }},
                        ),
                    )
                });
                    quote::quote! {
                        Ok(#import_token_stream QueryPartFragment::try_from(match self.as_ref() {
                            Some(#VSnakeCase) => {
                                let mut #ExtraParametersSnakeCase = #StringTokenStream::from("where");
                                let mut is_first_push_to_extra_parameters_already_happend = false;
                                #(#extra_parameters_modification_token_stream)*
                                #ExtraParametersSnakeCase
                            },
                            None => #StringTokenStream::default()
                        }).unwrap_or_else(#import_token_stream QueryPartFragment::from))
                    }
                },
                &pg_crud_macros_common::IsQueryBindMut::True,
                &{
                    let ts = generate_if_let_some_token_stream(
                        &quote::quote! {v_27176ffb},
                        &quote::quote! {self.into_option()},
                        &generate_read_fields_without_comma_token_stream(
                            &|element: &macros_helpers::field_data::SynField| {
                                let field = &element.identifier;
                                generate_if_let_some_token_stream(
                                    &quote::quote! {v_b12d6fe0},
                                    &quote::quote! {v_27176ffb.#field},
                                    &generate_match_query_bind_or_err_token_stream(
                                        &quote::quote! {#import_token_stream PgTypeWhereFilter::query_bind(v_b12d6fe0, #QuerySnakeCase)},
                                        &quote::quote! {v_edaee3b2},
                                        &quote::quote! {return Err(#Error0);},
                                    ),
                                )
                            },
                        ),
                    );
                    quote::quote! {
                        #ts
                        Ok(#QuerySnakeCase)
                    }
                },
                &pg_crud_macros_common::Import::PgCrudCommon,
            );
        let impl_pg_crud_default_some_one_element_for_optional_identifier_where_token_stream =
            generate_impl_pg_crud_default_some_one_element_for_tokens_no_lt_token_stream(
                &optional_identifier_where_upper_camel_case,
                &quote::quote! {Self(Some(#PgCrudCommonDefaultSomeOneElementCall))},
            );
        quote::quote! {
            #optional_identifier_where_token_stream
            #impl_optional_identifier_where_accessors_token_stream
            #impl_pg_type_where_filter_for_optional_identifier_where_token_stream
            #impl_pg_crud_default_some_one_element_for_optional_identifier_where_token_stream
        }
    };
    let pub_where_optional_identifier_where_token_stream =
        quote::quote! {pub #WhereManySnakeCase: #optional_identifier_where_upper_camel_case};
    let where_many_pg_crud_default_some_one_element_call_token_stream =
        generate_field_default_some_one_element_call_token_stream(&WhereManySnakeCase);
    let generate_read_or_dm_extra_parameters_initialization_token_stream = |rm_or_dm: &RmOrDm| {
        generate_match_ok_err_short_token_stream(
            &quote::quote! {#import_token_stream PgTypeWhereFilter::query_part(
                &#ParametersSnakeCase.#PayloadSnakeCase.#WhereManySnakeCase,
                &mut #IncrementSnakeCase,
                #import_token_stream SqlColumnRef::from(&""),//useless //todo check if can be optimized
                #import_token_stream AddOperator::from(false)//useless
            )},
            &quote::quote! {v_d1627695},
            &{
                let operation_error_initialization_eprintln_rm_or_dm_token_stream =
                    generate_operation_error_initialization_eprintln_res_token_stream(
                        &Operation::from(rm_or_dm),
                        &query_part_syn_variant,
                        std::panic::Location::caller(),
                    );
                quote::quote! {{
                    #operation_error_initialization_eprintln_rm_or_dm_token_stream
                }}
            },
        )
    };
    let macros_helpers_location_field_attr_eo_to_err_string_serde =
        macros_helpers::location_data::LocationFieldAttr::EoToErrStringSerde;
    let string_syn_punct = macros_helpers::generate_simple_syn_punct::generate_simple_syn_punct([
        str_constants::STRING,
    ]);
    let try_bind_syn_variant = new_syn_variant(
        &TryBindUpperCamelCase,
        Some(macros_helpers::status_code::StatusCode::InternalServerError500),
        vec![(
            macros_helpers_location_field_attr_eo_to_err_string_serde,
            &TryBindSnakeCase,
            string_syn_punct.clone(),
        )],
        false,
    );
    let generate_query_pg_type_where_filter_query_bind_parameters_payload_where_query_token_stream =
        |operation: &Operation| {
            generate_match_query_bind_or_err_token_stream(
                &quote::quote! {#import_token_stream PgTypeWhereFilter::query_bind(#ParametersSnakeCase.#PayloadSnakeCase.#WhereManySnakeCase, #import_token_stream SqlxPostgresQuery::from(#QuerySnakeCase)).map(#import_token_stream SqlxPostgresQuery::into_inner).map_err(|error| error.to_string())},
                &quote::quote! {v_03a58371},
                &generate_operation_error_initialization_eprintln_res_token_stream(
                    operation,
                    &try_bind_syn_variant,
                    std::panic::Location::caller(),
                ),
            )
        };
    let try_from_sqlx_pg_pg_row_with_not_empty_unique_vec_identifier_select_snake_case =
        naming::parameter::TryFromSqlxPgPgRowWithNotEmptyUniqueVecSelfSelectSnakeCase::from_display(
            &identifier,
        );
    let simple_syn_punct_sqlx_error =
        macros_helpers::generate_simple_syn_punct::generate_simple_syn_punct([
            str_constants::SQLX,
            str_constants::ERROR,
        ]);
    let macros_helpers_location_field_attr_eo_to_err_string =
        macros_helpers::location_data::LocationFieldAttr::EoToErrString;
    let pg_syn_variant = new_syn_variant(
        &PgUpperCamelCase,
        Some(macros_helpers::status_code::StatusCode::InternalServerError500),
        vec![(
            macros_helpers_location_field_attr_eo_to_err_string,
            &PgSnakeCase,
            simple_syn_punct_sqlx_error.clone(),
        )],
        false,
    );
    let generate_match_identifier_read_try_from_sqlx_pg_pg_row_with_not_empty_unique_vec_identifier_select_token_stream =
        |rm_or_ro: &RmOrRo| {
            generate_match_ok_err_short_token_stream(
                &quote::quote! {#identifier_read_upper_camel_case::#try_from_sqlx_pg_pg_row_with_not_empty_unique_vec_identifier_select_snake_case(
                    &v_b27d7d79,
                    &#ParametersSnakeCase.#PayloadSnakeCase.#SelectSnakeCase
                )},
                &quote::quote! {v_90535a1d},
                &{
                    let operation_error_initialization_eprintln_rm_or_ro_token_stream =
                        generate_operation_error_initialization_eprintln_res_token_stream(
                            &Operation::from(rm_or_ro),
                            &pg_syn_variant,
                            std::panic::Location::caller(),
                        );
                    quote::quote! {{
                        #operation_error_initialization_eprintln_rm_or_ro_token_stream
                    }}
                },
            )
        };
    let select_token_stream = {
        let identifier_select_token_stream = {
            let identifier_select_enum_token_stream = pg_crud_macros_common::token_stream_helpers::common_d_token_stream_builder()
            .d_eq()
            .d_std_hash_hash()
            .d_utoipa_to_schema()
            .build_enum(
                &proc_macro2::TokenStream::new(),
                &identifier_select_upper_camel_case,
                &proc_macro2::TokenStream::new(),
                &{
                    let variants = generate_read_fields_with_comma_token_stream(&|element: &macros_helpers::field_data::SynField| {
                        let serde_identifier_token_stream = generate_quotes::dq_token_stream(&element.identifier);
                        let field_upper_camel_case_token_stream = naming_common::ToTokensToUpperCamelCaseTokenStream::case_or_panic(&element.identifier);
                        let element_syn_field_ty_as_pg_type_select_token_stream = generate_as_pg_type_select_token_stream(&element.type0);
                        let concrete_select_token_stream = generate_concrete_pg_type_role_token_stream(&element.type0, &SelectUpperCamelCase);
                        quote::quote! {
                            #[serde(rename(serialize = #serde_identifier_token_stream, deserialize = #serde_identifier_token_stream))]
                            #[schema(value_type = #concrete_select_token_stream)]
                            #field_upper_camel_case_token_stream(#element_syn_field_ty_as_pg_type_select_token_stream)
                        }
                    });
                    quote::quote! {{#variants}}
                }
            );
            quote::quote! {
                #AllowClippyArbitrarySrcItemOrdering
                #identifier_select_enum_token_stream
            }
        };
        let impl_display_for_identifier_select_token_stream =
            macros_helpers::generate_impl_display_token_stream::generate_impl_display_token_stream(
                &proc_macro2::TokenStream::new(),
                &identifier_select_upper_camel_case,
                &proc_macro2::TokenStream::new(),
                &quote::quote! {write!(f, "{}", serde_json::to_string(&self).unwrap_or_else(|element_2636212f|format!("cannot serialize into json: {element_2636212f:?}")))},
            );
        let impl_location_lib_to_err_string_for_identifier_select_token_stream =
            pg_crud_macros_common::generate_impl_to_err_string_no_generics_token_stream(
                &identifier_select_upper_camel_case,
                &quote::quote! {format!("{self}")},
            );
        let impl_pg_crud_all_variants_default_some_one_element_for_identifier_select_token_stream =
            pg_crud_macros_common::generate_impl_pg_crud_common_all_variants_default_some_one_element_token_stream(
                &identifier_select_upper_camel_case,
                &{
                    let els_token_stream = generate_read_fields_with_comma_token_stream(
                        &|element: &macros_helpers::field_data::SynField| {
                            let field_upper_camel_case_token_stream = naming_common::ToTokensToUpperCamelCaseTokenStream::case_or_panic(&element.identifier);
                            quote::quote! {
                                Self::#field_upper_camel_case_token_stream(#PgCrudCommonDefaultSomeOneElementCall)
                            }
                        },
                    );
                    quote::quote! {vec![#els_token_stream]}
                },
            );
        quote::quote! {
            #identifier_select_token_stream
            #impl_display_for_identifier_select_token_stream
            #impl_location_lib_to_err_string_for_identifier_select_token_stream
            #impl_pg_crud_all_variants_default_some_one_element_for_identifier_select_token_stream
        }
    };
    let select_pg_crud_default_some_one_element_call_token_stream =
        generate_field_default_some_one_element_call_token_stream(&SelectSnakeCase);
    let identifier_read_token_stream = {
        let identifier_read_token_stream = {
            let identifier_read_struct_token_stream = macros_helpers::derive_token_stream_builder::DTokenStreamBuilder::new()
                .make_pub()
                .d_debug()
                .d_partial_eq()
                .d_serde_serialize()
                .d_serde_deserialize()
                .d_utoipa_to_schema()
                .build_struct(
                    &proc_macro2::TokenStream::new(),
                    &identifier_read_upper_camel_case,
                    &proc_macro2::TokenStream::new(),
                    &{
                        let field_optional_primary_key_token_stream = {
                            let optional_v_primary_key_field_type_as_pg_type_read_token_stream =
                                pg_crud_macros_common::generate_optional_type_declaration_token_stream(&generate_v_declaration_ts0(
                                    &generate_as_pg_type_read_token_stream(&primary_key_field_type),
                                ));
                            let concrete_primary_key_read_token_stream = generate_concrete_pg_type_role_token_stream(primary_key_field_type, &ReadUpperCamelCase);
                            quote::quote! {
                                #FieldAttrSerdeSkipSerializingIfOptionalIsNone
                                #[schema(inline, value_type = Option<#import_token_stream V<#concrete_primary_key_read_token_stream>>, nullable = false)]
                                pub #primary_key_field_identifier: #optional_v_primary_key_field_type_as_pg_type_read_token_stream
                            }
                        };
                        let fields_opts_without_primary_key_token_stream = generate_read_fields_without_primary_key_with_comma_token_stream(
                            &|element: &macros_helpers::field_data::SynField| -> proc_macro2::TokenStream {
                                let field_vis = &element.vis;
                                let field = &element.identifier;
                                let optional_v_field_type_as_pg_type_read_token_stream =
                                    pg_crud_macros_common::generate_optional_type_declaration_token_stream(&generate_v_declaration_ts0(
                                        &generate_as_pg_type_read_token_stream(&element.type0),
                                    ));
                                let concrete_read_token_stream = generate_concrete_pg_type_role_token_stream(&element.type0, &ReadUpperCamelCase);
                                quote::quote! {
                                    #FieldAttrSerdeSkipSerializingIfOptionalIsNone
                                    #[schema(inline, value_type = Option<#import_token_stream V<#concrete_read_token_stream>>, nullable = false)]
                                    #field_vis #field: #optional_v_field_type_as_pg_type_read_token_stream
                                }
                            },
                        );
                        quote::quote! {{
                            #field_optional_primary_key_token_stream,
                            #fields_opts_without_primary_key_token_stream
                        }}
                    },
                );
            quote::quote! {
                #AllowClippyArbitrarySrcItemOrdering
                #identifier_read_struct_token_stream
            }
        };
        let impl_identifier_read_token_stream = {
            let fn_try_from_sqlx_pg_pg_row_with_not_empty_unique_vec_identifier_select_token_stream = {
                let declaration_primary_key_token_stream = {
                    let optional_v_primary_key_field_type_as_primary_key_token_stream =
                        pg_crud_macros_common::generate_optional_type_declaration_token_stream(
                            &generate_v_declaration_ts0(
                                &primary_key_field_type_as_pg_type_read_upper_camel_case,
                            ),
                        );
                    quote::quote! {
                        let mut #primary_key_field_identifier: #optional_v_primary_key_field_type_as_primary_key_token_stream = None;
                    }
                };
                let declaration_without_primary_key_token_stream =
                    generate_read_fields_without_primary_key_without_comma_token_stream(
                        &|element: &macros_helpers::field_data::SynField| {
                            let field = &element.identifier;
                            let optional_v_field_type_as_pg_type_read_token_stream = pg_crud_macros_common::generate_optional_type_declaration_token_stream(
                            &generate_v_declaration_ts0(&generate_as_pg_type_read_token_stream(&element.type0)),
                        );
                            quote::quote! {
                                let mut #field: #optional_v_field_type_as_pg_type_read_token_stream = None;
                            }
                        },
                    );
                let generate_assign_token_stream =
                    |variant_upper_camel_case_token_stream: &dyn quote::ToTokens,
                     pg_type_read_token_stream: &dyn quote::ToTokens,
                     field_string_double_quoted_token_stream: &dyn quote::ToTokens,
                     field: &dyn quote::ToTokens| {
                        let ts = generate_match_ok_err_token_stream(
                            &quote::quote! {sqlx::Row::try_get::<
                                #pg_type_read_token_stream,
                                #RefStr
                            >(
                                #VSnakeCase,
                                #field_string_double_quoted_token_stream
                            )},
                            &quote::quote! {v_470178a2},
                            &quote::quote! {{
                                #field = Some(#import_token_stream #VUpperCamelCase { #VSnakeCase: v_470178a2 });
                            }},
                            &Error0,
                            &quote::quote! {{
                                return Err(#Error0);
                            }},
                        );
                        quote::quote! {#identifier_select_upper_camel_case::#variant_upper_camel_case_token_stream(_) => #ts}
                    };
                let (
                    assign_variant_primary_key_token_stream,
                    assign_variants_without_primary_key_token_stream,
                ) = {
                    (
                        generate_assign_token_stream(
                            &primary_key_field_upper_camel_case_token_stream,
                            &primary_key_field_type_as_pg_type_read_upper_camel_case,
                            &generate_quotes::dq_token_stream(&primary_key_field_identifier),
                            &primary_key_field_identifier,
                        ),
                        read_fields_without_primary_key_iter().map(|element| {
                            generate_assign_token_stream(
                                &naming_common::ToTokensToUpperCamelCaseTokenStream::case_or_panic(
                                    &element.identifier,
                                ),
                                &generate_as_pg_type_read_token_stream(&element.type0),
                                &generate_quotes::dq_token_stream(&element.identifier),
                                &element.identifier,
                            )
                        }),
                    )
                };
                let fields_initialization_token_stream =
                    read_fields_iter().map(|element| element.identifier.as_ref());
                quote::quote! {
                    fn #try_from_sqlx_pg_pg_row_with_not_empty_unique_vec_identifier_select_snake_case(
                        #VSnakeCase: &sqlx::postgres::PgRow,
                        #select_borrow_pg_crud_not_empty_unique_vec_identifier_select_token_stream
                    ) -> Result<Self, sqlx::Error> {
                        #declaration_primary_key_token_stream
                        #declaration_without_primary_key_token_stream
                        for element_dca9f0b7 in #SelectSnakeCase.as_slice() {
                            match element_dca9f0b7 {
                                #assign_variant_primary_key_token_stream,
                                #(#assign_variants_without_primary_key_token_stream),*
                            }
                        }
                        Ok(Self {#(#fields_initialization_token_stream),*})
                    }
                }
            };
            quote::quote! {
                impl #identifier_read_upper_camel_case {
                    #fn_try_from_sqlx_pg_pg_row_with_not_empty_unique_vec_identifier_select_token_stream
                }
            }
        };
        quote::quote! {
            #identifier_read_token_stream
            #impl_identifier_read_token_stream
        }
    };
    let identifier_read_ids_token_stream = {
        let identifier_read_ids_token_stream = {
            let identifier_read_ids_struct_token_stream = pg_crud_macros_common::token_stream_helpers::common_d_token_stream_builder()
                .d_utoipa_to_schema()
                .build_struct(
                    &proc_macro2::TokenStream::new(),
                    &identifier_read_ids_upper_camel_case,
                    &proc_macro2::TokenStream::new(),
                    &{
                        #[derive(optimal_memory_layout::OptimalMemoryLayout)]
enum WrapIntoOptional {
                            False,
                            True,
                        }
                        let generate_field_token_stream =
                            |field: &dyn quote::ToTokens,
                             field_type: &macros_helpers::field_data::SynFieldType,
                             wrap_into_optional: &WrapIntoOptional| {
                                let field_type_token_stream = match &wrap_into_optional {
                                    WrapIntoOptional::False => generate_as_pg_type_read_ids_token_stream(&field_type),
                                    WrapIntoOptional::True => pg_crud_macros_common::generate_optional_type_declaration_token_stream(
                                        &generate_as_pg_type_read_ids_token_stream(&field_type),
                                    )
                                    .into(),
                                };
                                let schema_attr_token_stream = match wrap_into_optional {
                                    WrapIntoOptional::False => {
                                        let concrete_read_ids_token_stream = generate_concrete_pg_type_role_token_stream(field_type, &ReadIdsUpperCamelCase);
                                        quote::quote! {#[schema(value_type = #concrete_read_ids_token_stream)]}
                                    },
                                    WrapIntoOptional::True => quote::quote! {#[schema(inline, value_type = Option<#import_token_stream NonPrimaryKeyPgTypeReadIds>, nullable = false)]},
                                };
                                quote::quote! {#schema_attr_token_stream pub #field: #field_type_token_stream}
                            };
                        let primary_key_token_stream = generate_field_token_stream(&primary_key_field_identifier, primary_key_field_type, &WrapIntoOptional::False);
                        let ts = generate_fields_named_without_primary_key_with_comma_token_stream(
                            &|element: &macros_helpers::field_data::SynField| {
                                generate_field_token_stream(&element.identifier, &element.type0, &WrapIntoOptional::True)
                            },
                        );
                        quote::quote! {{
                            #primary_key_token_stream,
                            #ts
                        }}
                    },
                );
            quote::quote! {
                #AllowClippyArbitrarySrcItemOrdering
                #identifier_read_ids_struct_token_stream
            }
        };
        let impl_sqlx_row_for_identifier_read_ids_token_stream = {
            let undescore_undrscr_row = quote::quote! {__row};
            let where_fts_token_stream = generate_fields_named_with_comma_token_stream(
                &|element: &macros_helpers::field_data::SynField| {
                    let field_type = &element.type0;
                    let element_syn_field_ty_as_pg_type_read_ids_token_stream =
                        generate_as_pg_type_read_ids_token_stream(&field_type);
                    quote::quote! {
                        #element_syn_field_ty_as_pg_type_read_ids_token_stream: ::sqlx::decode::Decode<'lt, R::Database>
                    }
                },
            );
            let primary_key_token_stream = {
                let element_syn_field_ty_as_pg_type_read_ids_token_stream =
                    generate_as_pg_type_read_ids_token_stream(&primary_key_field_type);
                let field_double_quoted_token_stream =
                    generate_quotes::dq_token_stream(&primary_key_field_identifier);
                let ts = generate_match_ok_err_short_token_stream(
                    &quote::quote! {sqlx::Row::try_get::<#element_syn_field_ty_as_pg_type_read_ids_token_stream, &str>(
                        #undescore_undrscr_row,
                        #field_double_quoted_token_stream
                    )},
                    &quote::quote! {v_283179dd},
                    &quote::quote! {{
                        return Err(#Error0);
                    }},
                );
                quote::quote! {
                    let #primary_key_field_identifier = #ts;
                }
            };
            let fields_initialization_token_stream =
                generate_fields_named_without_primary_key_without_comma_token_stream(
                    &|element: &macros_helpers::field_data::SynField| {
                        let field = &element.identifier;
                        let field_type = &element.type0;
                        let field_double_quoted_token_stream =
                            generate_quotes::dq_token_stream(&quote::quote! {#field});
                        let element_syn_field_ty_as_pg_type_read_ids_token_stream =
                            generate_as_pg_type_read_ids_token_stream(&field_type);
                        quote::quote! {
                            let #field = sqlx::Row::try_get::<
                                #element_syn_field_ty_as_pg_type_read_ids_token_stream,
                                &str
                            >(#undescore_undrscr_row, #field_double_quoted_token_stream).ok();
                        }
                    },
                );
            let self_fields_token_stream = generate_fields_named_with_comma_token_stream(
                &|element: &macros_helpers::field_data::SynField| {
                    let field = &element.identifier;
                    quote::quote! {#field}
                },
            );
            quote::quote! {
                impl<'lt, R: ::sqlx::Row<Database = sqlx::Postgres>> ::sqlx::FromRow<'lt, R> for #identifier_read_ids_upper_camel_case
                where
                    &'lt ::std::primitive::str: ::sqlx::ColumnIndex<R>,
                    #where_fts_token_stream
                {
                    fn from_row(#undescore_undrscr_row: &'lt R) -> ::sqlx::Result<Self> {
                        #primary_key_token_stream
                        #fields_initialization_token_stream
                        Ok(Self { #self_fields_token_stream })
                    }
                }
            }
        };
        quote::quote! {
            #identifier_read_ids_token_stream
            #impl_sqlx_row_for_identifier_read_ids_token_stream
        }
    };
    let generate_identifier_try_operation_error_upper_camel_case = |operation: &Operation| {
        let identifier_try_operation_error =
            quote::format_ident!("{identifier}Try{operation}Error");
        quote::quote! {#identifier_try_operation_error}
    };
    let identifier_try_rm_error_upper_camel_case =
        generate_identifier_try_operation_error_upper_camel_case(&Operation::Rm);
    let generate_identifier_operation_error_with_serde_upper_camel_case =
        |operation: &Operation| {
            generate_identifier_operation_suffix_token_stream(
                operation,
                str_constants::ERRORWITHSERDE,
            )
        };
    let pg_crud_order_by_token_stream = quote::quote! {#import_token_stream #OrderByUpperCamelCase};
    let identifier_update_upper_camel_case =
        naming::parameter::SelfUpdateUpperCamelCase::from_tokens(&identifier);
    let identifier_um_parameters_upper_camel_case =
        naming::parameter::SelfUmParametersUpperCamelCase::from_tokens(&identifier);
    let identifier_um_payload_upper_camel_case =
        naming::parameter::SelfUmPayloadUpperCamelCase::from_tokens(&identifier);
    let identifier_update_try_new_error_upper_camel_case =
        naming::parameter::SelfUpdateTryNewErrorUpperCamelCase::from_tokens(&identifier);
    let identifier_update_for_query_upper_camel_case =
        naming::parameter::SelfUpdateForQueryUpperCamelCase::from_tokens(&identifier);
    let path_v_token_stream = quote::quote! {pg_crud_common::#VUpperCamelCase};
    let identifier_update_token_stream = {
        let generate_optional_v_field_type_as_pg_type_update_token_stream: &dyn Fn(
            &dyn quote::ToTokens,
        ) -> macros_helpers::proc_macro2_tokens::ProcMacro2GeneratedRustTokenStream = &|syn_type| {
            let syn_type_as_pg_type_update_token_stream = generate_as_pg_type_update_token_stream(&syn_type);
            pg_crud_macros_common::generate_optional_type_declaration_token_stream(
                &quote::quote! {#path_v_token_stream<#syn_type_as_pg_type_update_token_stream>},
            )
        };
        let fields_declaration_token_stream = {
            let fields_named_without_primary_key_token_stream =
                generate_fields_named_without_primary_key_with_comma_token_stream(
                    &|element: &macros_helpers::field_data::SynField| -> proc_macro2::TokenStream {
                        let field = &element.identifier;
                        let optional_v_field_type_as_pg_type_update_token_stream =
                            generate_optional_v_field_type_as_pg_type_update_token_stream(
                                &element.type0,
                            );
                        quote::quote! {
                            #field: #optional_v_field_type_as_pg_type_update_token_stream
                        }
                    },
                );
            quote::quote! {
                #primary_key_field_identifier: #primary_key_field_type_update_token_stream,
                #fields_named_without_primary_key_token_stream
            }
        };
        let fields_schema_declaration_token_stream = {
            let fields_named_without_primary_key_token_stream =
                generate_fields_named_without_primary_key_with_comma_token_stream(
                    &|element: &macros_helpers::field_data::SynField| -> proc_macro2::TokenStream {
                        let field = &element.identifier;
                        let optional_v_field_type_as_pg_type_update_token_stream =
                            generate_optional_v_field_type_as_pg_type_update_token_stream(
                                &element.type0,
                            );
                        let concrete_update_token_stream =
                            generate_concrete_pg_type_role_token_stream(
                                &element.type0,
                                &UpdateUpperCamelCase,
                            );
                        quote::quote! {
                            #[schema(inline, value_type = Option<#path_v_token_stream<#concrete_update_token_stream>>, nullable = false)]
                            #field: #optional_v_field_type_as_pg_type_update_token_stream
                        }
                    },
                );
            let concrete_primary_key_update_token_stream =
                generate_concrete_pg_type_role_token_stream(
                    primary_key_field_type,
                    &UpdateUpperCamelCase,
                );
            quote::quote! {
                #[schema(value_type = #concrete_primary_key_update_token_stream)]
                #primary_key_field_identifier: #primary_key_field_type_update_token_stream,
                #fields_named_without_primary_key_token_stream
            }
        };
        let identifier_update_token_stream = {
            let identifier_update_struct_token_stream = serde_ser_utoipa_d_token_stream_builder
                .build_struct(
                    &proc_macro2::TokenStream::new(),
                    &identifier_update_upper_camel_case,
                    &proc_macro2::TokenStream::new(),
                    &quote::quote! {{#fields_schema_declaration_token_stream}},
                );
            quote::quote! {
                #AllowClippyArbitrarySrcItemOrdering
                #identifier_update_struct_token_stream
            }
        };
        let identifier_update_try_new_error_token_stream =
            generate_no_fields_provided_error_token_stream(
                &identifier_update_try_new_error_upper_camel_case,
            );
        let impl_pub_try_new_for_identifier_update_token_stream =
            macros_helpers::generate_new_or_try_new::generate_impl_pub_try_new_for_identifier_token_stream(
                &proc_macro2::TokenStream::new(),
                &identifier_update_upper_camel_case,
                &fields_declaration_token_stream,
                &identifier_update_try_new_error_upper_camel_case,
                &{
                    let (left_token_stream, right_token_stream) = {
                        let generate_token_stream = |ts: &dyn quote::ToTokens| {
                            pg_crud_macros_common::maybe_wrap_into_braces_token_stream(
                                ts,
                                pg_crud_macros_common::WrapIntoBraces::from(fields_len_without_primary_key > 1),
                            )
                        };
                        (
                        generate_token_stream(&generate_fields_named_without_primary_key_with_comma_token_stream(
                            &|element: &macros_helpers::field_data::SynField| -> proc_macro2::TokenStream {
                                let field = &element.identifier;
                                quote::quote! {&#field}
                            },
                        )),
                        generate_token_stream(&fields_named_without_primary_key_with_comma_none_token_stream),
                    )
                    };
                    let fields_inialization_token_stream = generate_fields_named_with_comma_token_stream(
                        &|element: &macros_helpers::field_data::SynField| -> proc_macro2::TokenStream {
                            let field = &element.identifier;
                            quote::quote! {#field}
                        },
                    );
                    quote::quote! {
                        if matches!(#left_token_stream, #right_token_stream) {
                            return Err(#identifier_update_try_new_error_upper_camel_case::#NoFieldsProvidedUpperCamelCase {
                                location: location_macros::location!(),
                            });
                        }
                        Ok(Self {#fields_inialization_token_stream})
                    }
                },
            );
        let impl_de_for_identifier_update_token_stream =
            pg_crud_macros_common::generate_impl_de_for_struct_by_fields_token_stream(
                &identifier_update_upper_camel_case,
                pg_crud_macros_common::SynFieldRefs::from(fields.as_slice()),
                pg_crud_macros_common::DeLen::from(fields_len),
                &|syn_identifier, syn_type| {
                    if syn_identifier == primary_key_field_identifier.as_ref() {
                        quote::quote! {#primary_key_field_type_update_token_stream}.into()
                    } else {
                        generate_optional_v_field_type_as_pg_type_update_token_stream(syn_type)
                    }
                },
            );
        let impl_pg_crud_default_some_one_element_for_identifier_update_token_stream =
            generate_impl_pg_crud_default_some_one_element_for_tokens_no_lt_token_stream(
                &identifier_update_upper_camel_case,
                &{
                    let ts = generate_field_default_some_one_element_call_token_stream(
                        &primary_key_field_identifier,
                    );
                    let fields_without_primary_key_with_default_some_one_element_token_stream =
                        generate_fields_named_without_primary_key_with_comma_token_stream(
                            &|element: &macros_helpers::field_data::SynField| {
                                let field = &element.identifier;
                                let ts0 = generate_v_initialization_ts0(
                                    &PgCrudCommonDefaultSomeOneElementCall,
                                );
                                quote::quote! {#field: Some(#ts0)}
                            },
                        );
                    quote::quote! {Self{
                        #ts,
                        #fields_without_primary_key_with_default_some_one_element_token_stream
                    }}
                },
            );
        quote::quote! {
            #identifier_update_token_stream
            #identifier_update_try_new_error_token_stream
            #impl_pub_try_new_for_identifier_update_token_stream
            #impl_de_for_identifier_update_token_stream
            #impl_pg_crud_default_some_one_element_for_identifier_update_token_stream
        }
    };
    let identifier_update_for_query_token_stream = {
        let identifier_update_for_query_token_stream = {
            let identifier_update_for_query_struct_token_stream = serde_ser_utoipa_d_token_stream_builder.build_struct(
                &proc_macro2::TokenStream::new(),
                &identifier_update_for_query_upper_camel_case,
                &proc_macro2::TokenStream::new(),
                &{
                    let fields_named_without_primary_key_token_stream = generate_fields_named_without_primary_key_with_comma_token_stream(
                        &|element: &macros_helpers::field_data::SynField| -> proc_macro2::TokenStream {
                            let field = &element.identifier;
                            let optional_v_field_type_as_pg_type_update_for_query_token_stream = {
                                let syn_type_as_pg_type_update_for_query_token_stream =
                                    generate_as_pg_type_update_for_query_token_stream(&element.type0);
                                pg_crud_macros_common::generate_optional_type_declaration_token_stream(
                                    &quote::quote! {#path_v_token_stream<#syn_type_as_pg_type_update_for_query_token_stream>},
                                )
                            };
                            let concrete_update_for_query_token_stream = generate_concrete_pg_type_role_token_stream(&element.type0, &UpdateForQueryUpperCamelCase);
                            quote::quote! {
                                #[schema(inline, value_type = Option<#path_v_token_stream<#concrete_update_for_query_token_stream>>, nullable = false)]
                                #field: #optional_v_field_type_as_pg_type_update_for_query_token_stream
                            }
                        },
                    );
                    let concrete_primary_key_update_for_query_token_stream = generate_concrete_pg_type_role_token_stream(primary_key_field_type, &UpdateForQueryUpperCamelCase);
                    quote::quote! {{
                        #[schema(value_type = #concrete_primary_key_update_for_query_token_stream)]
                        #primary_key_field_identifier: #primary_key_field_type_update_for_query_token_stream,
                        #fields_named_without_primary_key_token_stream
                    }}
                },
            );
            quote::quote! {
                #AllowClippyArbitrarySrcItemOrdering
                #identifier_update_for_query_struct_token_stream
            }
        };
        let impl_identifier_update_for_query_token_stream = {
            let update_query_part_primary_key_token_stream = {
                let ts = generate_match_ok_err_token_stream(
                    &quote::quote! {#primary_key_field_type_as_pg_type_token_stream #UpdateQueryPartSnakeCase(
                        &self.#primary_key_field_identifier,
                        #import_token_stream SqlColumnRef::from(&""),
                        #import_token_stream SqlColumnRef::from(&#identifier::#PrimaryKeySnakeCase()),
                        #import_token_stream SqlColumnRef::from(&""),
                        #IncrementSnakeCase,
                    )},
                    &VSnakeCase,
                    &quote::quote! {Ok(#VSnakeCase)},
                    &Error0,
                    &quote::quote! {Err(#Error0)},
                );
                quote::quote! {
                    fn #UpdateQueryPartPrimaryKeySnakeCase(&self, #IncrementSnakeCase: &mut dyn #import_token_stream QueryPartIncrementMut) -> Result<#import_token_stream QueryPartFragment, #import_token_stream #QueryPartErrorUpperCamelCase> {
                        #ts
                    }
                }
            };
            let update_query_part_fields_token_stream =
                generate_fields_named_without_primary_key_without_comma_token_stream(
                    &|element: &macros_helpers::field_data::SynField| {
                        let field = &element.identifier;
                        let update_query_part_field_snake_case =
                            naming::parameter::UpdateQueryPartSelfSnakeCase::from_tokens(&field);
                        let field_type_as_pg_crud_pg_type_pg_type_token_stream =
                            generate_as_pg_type_path_token_stream(&element.type0);
                        let ts = generate_match_ok_err_token_stream(
                            &{
                                let field_double_quoted_token_stream =
                                    generate_quotes::dq_token_stream(&field);
                                quote::quote! {#field_type_as_pg_crud_pg_type_pg_type_token_stream #UpdateQueryPartSnakeCase(
                                    &#VSnakeCase.#VSnakeCase,
                                    #import_token_stream SqlColumnRef::from(&#field_double_quoted_token_stream),
                                    #import_token_stream SqlColumnRef::from(&#field_double_quoted_token_stream),
                                    #import_token_stream SqlColumnRef::from(&""),
                                    #IncrementSnakeCase
                                )}
                            },
                            &quote::quote! {v_f75dfd93},
                            &quote::quote! {Ok(v_f75dfd93)},
                            &Error0,
                            &quote::quote! {Err(#Error0)},
                        );
                        quote::quote! {
                            fn #update_query_part_field_snake_case(
                                #VSnakeCase: &#import_token_stream V<#field_type_as_pg_crud_pg_type_pg_type_token_stream #UpdateForQueryUpperCamelCase>,
                                #IncrementSnakeCase: &mut dyn #import_token_stream QueryPartIncrementMut
                            ) -> Result<#import_token_stream QueryPartFragment, #import_token_stream #QueryPartErrorUpperCamelCase> {
                                #ts
                            }
                        }
                    },
                );
            let select_only_updated_ids_query_part_token_stream = {
                let primary_key_token_stream = {
                    let primary_key_field_double_quoted_token_stream =
                        generate_quotes::dq_token_stream(&primary_key_field_identifier);
                    let ts = generate_match_ok_err_short_token_stream(
                        &quote::quote! {#primary_key_as_pg_type_token_stream::#SelectOnlyUpdatedIdsQueryPartSnakeCase(
                            &self.#primary_key_field_identifier,
                            #import_token_stream SqlColumnRef::from(&#primary_key_field_double_quoted_token_stream),
                            increment,
                        )},
                        &quote::quote! {v},
                        &quote::quote! {{
                            return Err(#Error0);
                        }},
                    );
                    quote::quote! {accumulator.push_str(&#ts);}
                };
                let ts = fields_without_primary_key_iter().map(|element| {
                    let field = &element.identifier;
                    generate_if_let_some_token_stream(&quote::quote! {v_90f79b11}, &quote::quote! {&self.#field}, &{
                        let ts = generate_match_ok_err_short_token_stream(
                            &{
                                let field_double_quoted_token_stream = generate_quotes::dq_token_stream(&field);
                                let field_type_as_pg_crud_pg_type_pg_type_token_stream =
                                    generate_as_pg_type_path_token_stream(&element.type0);
                                quote::quote! {#field_type_as_pg_crud_pg_type_pg_type_token_stream #SelectOnlyUpdatedIdsQueryPartSnakeCase(
                                    &v_90f79b11.#VSnakeCase,
                                    #import_token_stream SqlColumnRef::from(&#field_double_quoted_token_stream),
                                    increment,
                                )}
                            },
                            &quote::quote! {v_47a6f597},
                            &quote::quote! {{
                                return Err(#Error0);
                            }},
                        );
                        quote::quote! {accumulator.push_str(&#ts);}
                    })
                });
                let ts0 = generate_accumulator_string_pop_ok_accumulator_token_stream(
                    &quote::quote! {accumulator},
                    &quote::quote! {
                        #primary_key_token_stream
                        #(#ts)*
                    },
                );
                quote::quote! {
                    fn #SelectOnlyUpdatedIdsQueryPartSnakeCase(&self, #IncrementSnakeCase: &mut dyn #import_token_stream QueryPartIncrementMut) -> Result<#import_token_stream QueryPartFragment, #import_token_stream QueryPartError> {
                        #ts0
                    }
                }
            };
            let update_handle_token_stream = generate_from_handle_token_stream(
                &identifier_update_upper_camel_case,
                &{
                    let primary_key_field_type_as_pg_type_update_for_query_token_stream =
                        generate_as_pg_type_update_for_query_token_stream(&primary_key_field_type);
                    let fields_named_without_primary_key_token_stream = generate_fields_named_without_primary_key_with_comma_token_stream(
                    &|element: &macros_helpers::field_data::SynField| -> proc_macro2::TokenStream {
                        let field = &element.identifier;
                        let ts = generate_v_initialization_ts0(&{
                            let field_type_as_pg_type_update_for_query_token_stream =
                                generate_as_pg_type_update_for_query_token_stream(&element.type0);
                            quote::quote! {#field_type_as_pg_type_update_for_query_token_stream::from(v_0e64c53a.#VSnakeCase)}
                        });
                        quote::quote! {#field: #VSnakeCase.#field.map(|v_0e64c53a| #ts)}
                    },
                );
                    quote::quote! {
                        Self {
                            #primary_key_field_identifier: #primary_key_field_type_as_pg_type_update_for_query_token_stream::from(#VSnakeCase.#primary_key_field_identifier),
                            #fields_named_without_primary_key_token_stream
                        }
                    }
                },
            );
            quote::quote! {
                #AllowClippyArbitrarySrcItemOrdering
                impl #identifier_update_for_query_upper_camel_case {
                    #update_query_part_primary_key_token_stream
                    #update_query_part_fields_token_stream
                    #select_only_updated_ids_query_part_token_stream
                    #update_handle_token_stream
                }
            }
        };
        quote::quote! {
            #identifier_update_for_query_token_stream
            #impl_identifier_update_for_query_token_stream
        }
    };
    let generate_match_update_query_part_primary_key_token_stream =
        |operation: &Operation, ts: &dyn quote::ToTokens| {
            generate_match_ok_err_short_token_stream(
                &quote::quote! {#ts.#UpdateQueryPartPrimaryKeySnakeCase(&mut #IncrementSnakeCase)},
                &quote::quote! {v_f269a3b2},
                &{
                    let operation_error_initialization_eprintln_update_query_part_primary_key_token_stream =
                        generate_operation_error_initialization_eprintln_res_token_stream(
                            operation,
                            &query_part_syn_variant,
                            std::panic::Location::caller(),
                        );
                    quote::quote! {{
                        #operation_error_initialization_eprintln_update_query_part_primary_key_token_stream
                    }}
                },
            )
        };
    let row_field_name: &dyn std::fmt::Display = &RowSnakeCase;
    let row_and_rollback_syn_variant = new_syn_variant(
        &RowAndRollbackUpperCamelCase,
        Some(macros_helpers::status_code::StatusCode::InternalServerError500),
        vec![
            (
                macros_helpers_location_field_attr_eo_to_err_string,
                row_field_name,
                simple_syn_punct_sqlx_error.clone(),
            ),
            (
                macros_helpers_location_field_attr_eo_to_err_string,
                &RollbackSnakeCase,
                simple_syn_punct_sqlx_error,
            ),
        ],
        false,
    );
    let sqlx_query_sqlx_pg_token_stream = quote::quote! {sqlx::query::<sqlx::Postgres>};
    let (
        pg_crud_pg_type_where_filter_query_part_token_stream,
        pg_crud_pg_type_where_filter_query_bind_token_stream,
    ) = {
        let generate_token_stream =
            |ts: &dyn quote::ToTokens| quote::quote! {#import_token_stream PgTypeWhereFilter::#ts};
        (
            generate_token_stream(&QueryPartSnakeCase),
            generate_token_stream(&QueryBindSnakeCase),
        )
    };
    let vec_struct_opts_identifier_token_stream =
        pg_crud_macros_common::generate_vec_tokens_declaration_token_stream(
            &identifier_read_upper_camel_case,
        );
    let not_unique_field_syn_variant = new_syn_variant(
        &NotUniqueFieldUpperCamelCase,
        Some(macros_helpers::status_code::StatusCode::BadReq400),
        vec![(
            macros_helpers_location_field_attr_eo_to_err_string_serde,
            &NotUniqueFieldSnakeCase,
            macros_helpers::generate_simple_syn_punct::generate_simple_syn_punct([
                &identifier_select_upper_camel_case.to_string(),
            ]),
        )],
        true,
    );
    let simple_syn_punct_serde_error =
        macros_helpers::generate_simple_syn_punct::generate_simple_syn_punct([
            str_constants::SERDE_JSON,
            str_constants::ERROR,
        ]);
    let serde_json_to_string_syn_variant = new_syn_variant(
        &SerdeJsonToStringUpperCamelCase,
        None,
        vec![(
            macros_helpers_location_field_attr_eo_to_err_string,
            &SerdeJsonToStringSnakeCase,
            simple_syn_punct_serde_error.clone(),
        )],
        false,
    );
    let simple_syn_punct_reqwest_error =
        macros_helpers::generate_simple_syn_punct::generate_simple_syn_punct([
            str_constants::REQWEST,
            str_constants::ERROR,
        ]);
    let status_code_field_name: &dyn std::fmt::Display = &StatusCodeSnakeCase;
    let failed_to_get_res_text_syn_variant = new_syn_variant(
        &FailedToGetResTextUpperCamelCase,
        Some(macros_helpers::status_code::StatusCode::BadReq400),
        vec![
            (
                macros_helpers_location_field_attr_eo_to_err_string,
                status_code_field_name,
                macros_helpers::generate_simple_syn_punct::generate_simple_syn_punct([
                    str_constants::REQWEST,
                    str_constants::STATUSCODE,
                ]),
            ),
            (
                macros_helpers_location_field_attr_eo_to_err_string,
                &HeadersSnakeCase,
                macros_helpers::generate_simple_syn_punct::generate_simple_syn_punct([
                    str_constants::REQWEST,
                    str_constants::HEADER,
                    str_constants::HEADERMAP,
                ]),
            ),
            (
                macros_helpers_location_field_attr_eo_to_err_string,
                &ReqwestSnakeCase,
                simple_syn_punct_reqwest_error.clone(),
            ),
        ],
        false,
    );
    let deserialize_res_syn_variant = new_syn_variant(
        &DeResUpperCamelCase,
        None,
        vec![
            (
                macros_helpers_location_field_attr_eo_to_err_string,
                status_code_field_name,
                macros_helpers::generate_simple_syn_punct::generate_simple_syn_punct([
                    str_constants::REQWEST,
                    str_constants::STATUSCODE,
                ]),
            ),
            (
                macros_helpers_location_field_attr_eo_to_err_string,
                &HeadersSnakeCase,
                macros_helpers::generate_simple_syn_punct::generate_simple_syn_punct([
                    str_constants::REQWEST,
                    str_constants::HEADER,
                    str_constants::HEADERMAP,
                ]),
            ),
            (
                macros_helpers_location_field_attr_eo_to_err_string_serde,
                &ResTextSnakeCase,
                string_syn_punct,
            ),
            (
                macros_helpers_location_field_attr_eo_to_err_string,
                &SerdeSnakeCase,
                simple_syn_punct_serde_error.clone(),
            ),
        ],
        false,
    );
    let reqwest_syn_variant = new_syn_variant(
        &ReqwestUpperCamelCase,
        None,
        vec![(
            macros_helpers_location_field_attr_eo_to_err_string,
            &ReqwestSnakeCase,
            simple_syn_punct_reqwest_error,
        )],
        false,
    );
    let check_body_size_syn_variant = new_syn_variant(
        &CheckBodySizeUpperCamelCase,
        Some(macros_helpers::status_code::StatusCode::PayloadTooLarge413),
        vec![(
            macros_helpers::location_data::LocationFieldAttr::EoLocation,
            &CheckBodySizeSnakeCase,
            macros_helpers::generate_simple_syn_punct::generate_simple_syn_punct([
                str_constants::ROUTE_VALIDATORS,
                str_constants::CHECK_BODY_SIZE,
                &BodySizeErrorUpperCamelCase.to_string(),
            ]),
        )],
        false,
    );
    let serde_json_syn_variant = new_syn_variant(
        &SerdeJsonUpperCamelCase,
        Some(macros_helpers::status_code::StatusCode::BadReq400),
        vec![(
            macros_helpers_location_field_attr_eo_to_err_string,
            &SerdeJsonSnakeCase,
            simple_syn_punct_serde_error,
        )],
        false,
    );
    let header_cnt_type_app_json_not_found_syn_variant = new_syn_variant(
        &HeaderContentTypeAppJsonNotFoundUpperCamelCase,
        Some(macros_helpers::status_code::StatusCode::BadReq400),
        Vec::<(
            macros_helpers::location_data::LocationFieldAttr,
            &'static dyn std::fmt::Display,
            macros_helpers::generate_simple_syn_punct::SynPathSegments,
        )>::default(),
        false,
    );
    let common_http_req_syn_variants = {
        vec![
            GeneratePgTableVariantEmissionRef::Syn(
                serde_json_to_string_syn_variant.get_syn_variant(),
            ),
            GeneratePgTableVariantEmissionRef::Syn(
                failed_to_get_res_text_syn_variant.get_syn_variant(),
            ),
            GeneratePgTableVariantEmissionRef::Syn(deserialize_res_syn_variant.get_syn_variant()),
            GeneratePgTableVariantEmissionRef::Syn(reqwest_syn_variant.get_syn_variant()),
        ]
    };
    let empty_logic_token_stream = proc_macro2::TokenStream::new();
    let generate_logic_token_stream = |generate_pg_table_attr| -> &proc_macro2::TokenStream {
        generate_pg_table_input_model
            .logic_token_stream_by_attr
            .get(&generate_pg_table_attr)
            .unwrap_or(&empty_logic_token_stream)
    };
    let common_route_syn_variants = {
        let optional_common_error_variants = generate_pg_table_input_model
            .error_variants_by_attr
            .get(&GeneratePgTableAttr::CommonErrorVariants);
        let mut accumulator = Vec::with_capacity(
            4usize.saturating_add(optional_common_error_variants.map_or(0usize, Vec::len)),
        );
        accumulator.push(GeneratePgTableVariantEmissionRef::Syn(
            check_body_size_syn_variant.get_syn_variant(),
        ));
        accumulator.push(GeneratePgTableVariantEmissionRef::Syn(
            pg_syn_variant.get_syn_variant(),
        ));
        accumulator.push(GeneratePgTableVariantEmissionRef::Syn(
            serde_json_syn_variant.get_syn_variant(),
        ));
        accumulator.push(GeneratePgTableVariantEmissionRef::Syn(
            header_cnt_type_app_json_not_found_syn_variant.get_syn_variant(),
        ));
        if let Some(variants) = optional_common_error_variants {
            accumulator.extend(
                variants
                    .iter()
                    .map(GeneratePgTableVariantEmissionRef::Model),
            );
        }
        accumulator
    };
    let generate_pub_handle_token_stream = |is_pub| {
        if is_pub {
            quote::quote! {pub}
        } else {
            proc_macro2::TokenStream::new()
        }
    };
    let generate_pub_handle_primary_key_field_primary_key_inner_type_handle_token_stream =
        |ts: &dyn quote::ToTokens| {
            let is_pub = true;
            let pub_handle_token_stream = generate_pub_handle_token_stream(is_pub);
            quote::quote! {#pub_handle_token_stream #primary_key_field_identifier: #ts}
        };
    let generate_match_pg_transaction_rollback_await_token_stream =
        |operation: &Operation, location| {
            let operation_error_initialization_pg_rollback_token_stream =
                generate_operation_error_initialization_eprintln_res_token_stream(
                    operation,
                    &pg_syn_variant,
                    location,
                );
            let row_and_rollback_syn_variant_error_initialization_eprintln_res_creation_token_stream =
                generate_operation_error_initialization_eprintln_res_token_stream(
                    operation,
                    &row_and_rollback_syn_variant,
                    location,
                );
            quote::quote! {{
                if let Err(#Error1) = #ExecutorSnakeCase.#RollbackSnakeCase().await {
                    #row_and_rollback_syn_variant_error_initialization_eprintln_res_creation_token_stream
                }
                #operation_error_initialization_pg_rollback_token_stream
            }}
        };
    let generate_drop_rows_match_pg_transaction_rollback_await_handle_token_stream =
        |operation: &Operation, location| {
            let match_pg_transaction_rollback_await_token_stream =
                generate_match_pg_transaction_rollback_await_token_stream(operation, location);
            quote::quote! {
                drop(#RowsSnakeCase);
                #match_pg_transaction_rollback_await_token_stream
            }
        };
    let wrap_into_v_token_stream = |ts: &dyn quote::ToTokens| {
        quote::quote! {
            let #VSnakeCase = {
                #ts
            };
        }
    };
    let generate_fetch_token_stream =
        |fetch_token_stream: &dyn quote::ToTokens,
         some_token_stream: &dyn quote::ToTokens,
         error_token_stream: &dyn quote::ToTokens,
         should_wrap_into_v: &ShouldWrapIntoV| {
            let ts = {
                let ts = generate_match_ok_err_token_stream(
                    &quote::quote! {futures::TryStreamExt::try_next(&mut #RowsSnakeCase).await},
                    &quote::quote! {v_19f3d6e1},
                    &quote::quote! {match v_19f3d6e1 {
                        Some(v_b27d7d79) => #some_token_stream,
                        None => None,
                    }},
                    &Error0,
                    &quote::quote! {{
                        #error_token_stream
                    }},
                );
                quote::quote! {
                    let mut #RowsSnakeCase = #BindedQuerySnakeCase.fetch(#fetch_token_stream.as_mut());
                    let mut accumulator_d16ac269 = Vec::new();
                    while let Some(v_d9cc2c36) = #ts {
                        accumulator_d16ac269.push(v_d9cc2c36);
                    }
                    accumulator_d16ac269
                }
            };
            match should_wrap_into_v {
                ShouldWrapIntoV::False => ts,
                ShouldWrapIntoV::True => wrap_into_v_token_stream(&ts),
            }
        };
    let generate_fetch_one_token_stream =
        |fetch_token_stream: &dyn quote::ToTokens,
         ok_token_stream: &dyn quote::ToTokens,
         error_token_stream: &dyn quote::ToTokens| {
            generate_match_ok_err_token_stream(
                &quote::quote! {#BindedQuerySnakeCase.fetch_one(#fetch_token_stream.as_mut()).await},
                &quote::quote! {v_b27d7d79},
                &quote::quote! {{
                    #ok_token_stream
                }},
                &Error0,
                &quote::quote! {{
                    #error_token_stream
                }},
            )
        };
    let generate_sqlx_row_try_get_primary_key_token_stream =
        |sqlx_row_try_get_type_token_stream: &dyn quote::ToTokens,
         ok_token_stream: &dyn quote::ToTokens,
         err_token_stream: &dyn quote::ToTokens| {
            generate_match_ok_err_token_stream(
                &quote::quote! {#SqlxRow::try_get::<
                    #sqlx_row_try_get_type_token_stream,
                    #RefStr
                >(&v_b27d7d79, Self::#PrimaryKeySnakeCase())},
                &quote::quote! {v_69ecb6a9},
                &ok_token_stream,
                &Error0,
                &quote::quote! {{
                    #err_token_stream
                }},
            )
        };
    let wrap_into_pg_transaction_begin_commit_token_stream =
        |operation: &Operation, ts: &dyn quote::ToTokens| {
            let pg_transaction_begin_token_stream = {
                let operation_error_initialization_pg_begin_token_stream =
                    generate_operation_error_initialization_eprintln_res_token_stream(
                        operation,
                        &pg_syn_variant,
                        std::panic::Location::caller(),
                    );
                let ts0 = generate_match_ok_err_short_token_stream(
                    &quote::quote! {#SqlxAcquire::#BeginSnakeCase(#ExecutorAcquireSnakeCase).await},
                    &quote::quote! {v_1aaca28f},
                    &quote::quote! {{#operation_error_initialization_pg_begin_token_stream}},
                );
                quote::quote! {let mut #ExecutorSnakeCase = #ts0;}
            };
            let pg_transaction_commit_token_stream = {
                let pg_syn_variant_error_initialization_eprintln_res_creation_token_stream =
                    generate_operation_error_initialization_eprintln_res_token_stream(
                        operation,
                        &pg_syn_variant,
                        std::panic::Location::caller(),
                    );
                let release_token_stream =
                    if generate_pg_table_input_model.config.idempotent_mutations
                        && operation.supports_idempotency()
                    {
                        quote::quote! {
                            let _release_result = pg_table::release_pg_table_idempotency(
                                app_state::SqlxPgPoolRef::from(idempotency_pool_193acb3c.as_ref()),
                                &idempotency_request_0a0ae019,
                            ).await;
                        }
                    } else {
                        proc_macro2::TokenStream::new()
                    };
                quote::quote! {
                    if let Err(#Error0) = #ExecutorSnakeCase.#CommitSnakeCase().await {
                        #release_token_stream
                        #pg_syn_variant_error_initialization_eprintln_res_creation_token_stream
                    }
                }
            };
            let idempotency_transaction_completion_token_stream = if generate_pg_table_input_model
                .config
                .idempotent_mutations
                && operation.supports_idempotency()
            {
                let identifier_operation_res_variants_upper_camel_case =
                    generate_identifier_operation_res_variants_upper_camel_case(operation);
                let desirable_status_token_stream = operation
                    .desirable_status_code()
                    .to_http_status_code_token_stream();
                quote::quote! {
                    let response_value_1a2393ae = #identifier_operation_res_variants_upper_camel_case::#DesirableUpperCamelCase(#VSnakeCase);
                    let response_body_649297c9 = match serde_json::to_vec(&response_value_1a2393ae) {
                        Ok(value) => value,
                        Err(_error) => {
                            let _rollback_result = #ExecutorSnakeCase.#RollbackSnakeCase().await;
                            let _release_result = pg_table::release_pg_table_idempotency(app_state::SqlxPgPoolRef::from(idempotency_pool_193acb3c.as_ref()), &idempotency_request_0a0ae019).await;
                            return axum::response::IntoResponse::into_response(http::StatusCode::INTERNAL_SERVER_ERROR);
                        }
                    };
                    let _idempotency_response_reservation_f07d6371 = match server_runtime_http::GetIdempotencyResponseResourceBudget::get_idempotency_response_resource_budget(#AppStateSnakeCase.as_ref()).reserve(
                        server_runtime_http::ResourceBudgetAmount::from(response_body_649297c9.len()),
                    ) {
                        Ok(value) => value,
                        Err(_error) => {
                            let _rollback_result = #ExecutorSnakeCase.#RollbackSnakeCase().await;
                            let _release_result = pg_table::release_pg_table_idempotency(app_state::SqlxPgPoolRef::from(idempotency_pool_193acb3c.as_ref()), &idempotency_request_0a0ae019).await;
                            return axum::response::IntoResponse::into_response(http::StatusCode::TOO_MANY_REQUESTS);
                        }
                    };
                    if pg_table::complete_pg_table_idempotency_in_connection(
                        pg_table::SqlxPgTablePgConnectionRef::from(#ExecutorSnakeCase.as_mut()),
                        &idempotency_request_0a0ae019,
                        pg_table::PgTableIdempotencyResponseStatus::try_from(#desirable_status_token_stream.as_u16())
                            .unwrap_or_else(|_error| pg_table::PgTableIdempotencyResponseStatus::internal_server_error()),
                        pg_table::PgTableIdempotencyBodyRef::from(response_body_649297c9.as_slice()),
                    ).await.is_err() {
                        let _rollback_result = #ExecutorSnakeCase.#RollbackSnakeCase().await;
                        let _release_result = pg_table::release_pg_table_idempotency(app_state::SqlxPgPoolRef::from(idempotency_pool_193acb3c.as_ref()), &idempotency_request_0a0ae019).await;
                        return axum::response::IntoResponse::into_response(http::StatusCode::INTERNAL_SERVER_ERROR);
                    }
                }
            } else {
                proc_macro2::TokenStream::new()
            };
            let transaction_output_token_stream =
                if generate_pg_table_input_model.config.idempotent_mutations
                    && operation.supports_idempotency()
                {
                    quote::quote! {(response_value_1a2393ae, response_body_649297c9)}
                } else {
                    quote::quote! {#VSnakeCase}
                };
            quote::quote! {
                #pg_transaction_begin_token_stream
                #ts
                #idempotency_transaction_completion_token_stream
                #pg_transaction_commit_token_stream
                #transaction_output_token_stream
            }
        };
    let generate_location_attr_view_token_stream =
        |field_ref: SynGeneratePgTableIdentifierRef<'_>,
         location_attr: GeneratePgTableVariantLocationAttr|
         -> proc_macro2::TokenStream {
            let field = field_ref.get();
            if *field == *LocationSnakeCase.to_string() {
                proc_macro2::TokenStream::new()
            } else {
                location_attr.get().map_or_else(
                    || {
                        compile_error_token_stream(CompileErrorMessage::from(
                            str_constants::COMPILE_ERROR_CE_050,
                        ))
                        .into()
                    },
                    |v| v.to_attr_view_token_stream().into(),
                )
            }
        };
    let generate_location_variant_token_stream: &dyn Fn(
        GeneratePgTableVariantEmissionRef<'_>,
    ) -> proc_macro2::TokenStream = &|error_variant| -> proc_macro2::TokenStream {
        let variant_identifier = error_variant.identifier();
        match error_variant {
            GeneratePgTableVariantEmissionRef::Syn(syn_variant) => {
                let syn::Fields::Named(fields_named) = &syn_variant.fields else {
                    return compile_error_token_stream(CompileErrorMessage::from(
                        str_constants::COMPILE_ERROR_CE_008,
                    ))
                    .into();
                };
                let fields_mapped_into_token_stream =
                    fields_named.named.iter().map(|field| {
                        let Some(field_identifier) = field.ident.as_ref() else {
                            return compile_error_token_stream(CompileErrorMessage::from(
                                str_constants::MACRO_DIAGNOSTICS_EXPECTED_NAMED_FIELD_A2_ERROR,
                            ))
                            .into();
                        };
                        let parsed_location_attr =
                            match generate_pg_table_syn_field_location_attr_stage(
                                SynGeneratePgTableFieldRef::from(field),
                            ) {
                                Ok(v) => v,
                                Err(error) => return error.into(),
                            };
                        let location_attr_token_stream = generate_location_attr_view_token_stream(
                            SynGeneratePgTableIdentifierRef::from(field_identifier),
                            GeneratePgTableVariantLocationAttr::from(parsed_location_attr),
                        );
                        let field_type = &field.ty;
                        quote::quote! {
                            #location_attr_token_stream
                            #field_identifier: #field_type
                        }
                    });
                quote::quote! {
                    #variant_identifier {
                        #(#fields_mapped_into_token_stream),*
                    }
                }
            }
            GeneratePgTableVariantEmissionRef::Model(model_variant) => {
                let fields_mapped_into_token_stream = model_variant.fields.iter().map(|field| {
                    let field_identifier = &field.identifier;
                    let location_attr = generate_location_attr_view_token_stream(
                        SynGeneratePgTableIdentifierRef::from(field_identifier),
                        GeneratePgTableVariantLocationAttr::from(field.location_attr),
                    );
                    let field_type = &field.type0;
                    quote::quote! {
                        #location_attr
                        #field_identifier: #field_type
                    }
                });
                quote::quote! {
                    #variant_identifier {
                        #(#fields_mapped_into_token_stream),*
                    }
                }
            }
        }
    };
    let generate_serde_field_token_stream =
        |field_ref: SynGeneratePgTableIdentifierRef<'_>,
         ty_ref: SynGeneratePgTableTypeRef<'_>,
         location_attr: GeneratePgTableVariantLocationAttr|
         -> macros_helpers::proc_macro2_tokens::ProcMacro2GeneratedRustTokenStream {
            let field = field_ref.get();
            let ty = ty_ref.get();
            let string_token_stream = token_patterns::StringTokenStream;
            let with_serde_upper_camel_case = naming::WithSerdeUpperCamelCase;
            let hash_map_upper_camel_case = naming::HashMapUpperCamelCase;
            let ts = if *field == *LocationSnakeCase.to_string() {
                quote::quote! {#LocationSnakeCase: location_lib::location::Location}
            } else {
                let get_hashmap_args = || {
                    let segments = if let syn::Type::Path(syn_type_path) = ty {
                        &syn_type_path.path.segments
                    } else {
                        return None;
                    };
                    let last_segment = segments.iter().next_back()?;
                    assert!(
                        last_segment.ident == hash_map_upper_camel_case.to_string(),
                        "60f0795d"
                    );
                    let syn::PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments {
                        args,
                        ..
                    }) = &last_segment.arguments
                    else {
                        return None;
                    };
                    assert!(args.len() == 2, "76d6e450");
                    Some((args.iter().next()?, args.iter().nth(1)?))
                };
                let element_type_token_stream = quote::quote! {#ty};
                let Some(parsed_location_attr) = location_attr.get() else {
                    return compile_error_token_stream(CompileErrorMessage::from(
                        str_constants::COMPILE_ERROR_CE_042,
                    ));
                };
                let element_type_with_serde_token_stream = match parsed_location_attr {
                macros_helpers::location_data::LocationFieldAttr::EoToErrString => quote::quote! {#string_token_stream},
                macros_helpers::location_data::LocationFieldAttr::EoToErrStringSerde
                | macros_helpers::location_data::LocationFieldAttr::EoVecToErrStringSerde => element_type_token_stream,
                macros_helpers::location_data::LocationFieldAttr::EoLocation => {
                    match format!("{element_type_token_stream}{with_serde_upper_camel_case}")
                        .parse::<proc_macro2::TokenStream>()
                    {
                        Ok(parsed_token_stream) => parsed_token_stream,
                        Err(error) => {
                            return compile_error_token_stream(CompileErrorMessage::from(
                                &str_constants::COMPILE_ERROR_CE_005
                                    .replace(
                                        str_constants::COMPILE_ERROR_ERROR_PLACEHOLDER,
                                        &error.to_string(),
                                    ),
                            ));
                        }
                    }
                }
                macros_helpers::location_data::LocationFieldAttr::EoVecToErrString => {
                    quote::quote! {Vec<#string_token_stream>}
                }
                macros_helpers::location_data::LocationFieldAttr::EoVecLocation => {
                    let segments = if let syn::Type::Path(v0) = ty {
                        &v0.path.segments
                    } else {
                        return compile_error_token_stream(CompileErrorMessage::from(str_constants::COMPILE_ERROR_CE_024));
                    };
                    assert!(segments.len() == 1, "8c6c5e9d");
                    let Some(first_segment) = segments.iter().next() else {
                        return compile_error_token_stream(CompileErrorMessage::from(
                            str_constants::MACRO_DIAGNOSTICS_EXPECTED_FIRST_PATH_SEGMENT_ERROR,
                        ));
                    };
                    let syn::PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments {
                        args,
                        ..
                    }) = &first_segment.arguments
                    else {
                        return compile_error_token_stream(CompileErrorMessage::from(
                            str_constants::MACRO_DIAGNOSTICS_EXPECTED_ANGLE_BRACKETED_ARGS_ERROR,
                        ));
                    };
                    assert!(args.len() == 1, "5bf19c5d");
                    let Some(first_arg) = args.iter().next() else {
                        return compile_error_token_stream(CompileErrorMessage::from(
                            str_constants::COMPILE_ERROR_CE_053,
                        ));
                    };
                    let element_vec_type_with_serde_token_stream =
                        match format!("{}{}", quote::quote! {#first_arg}, with_serde_upper_camel_case)
                            .parse::<proc_macro2::TokenStream>()
                        {
                            Ok(parsed_token_stream) => parsed_token_stream,
                            Err(error) => {
                                return compile_error_token_stream(CompileErrorMessage::from(
                                    &str_constants::COMPILE_ERROR_CE_007
                                        .replace(
                                            str_constants::COMPILE_ERROR_ERROR_PLACEHOLDER,
                                            &error.to_string(),
                                        ),
                                ));
                            }
                        };
                    quote::quote! {Vec<#element_vec_type_with_serde_token_stream>}
                }
                macros_helpers::location_data::LocationFieldAttr::EoHashMapKStringVToErrString => {
                    if get_hashmap_args().is_none() {
                        return compile_error_token_stream(CompileErrorMessage::from(
                            str_constants::MACRO_DIAGNOSTICS_EXPECTED_HASH_MAP_C1_ERROR,
                        ));
                    }
                    quote::quote! {std::collections::HashMap<#string_token_stream, #string_token_stream>}
                }
                macros_helpers::location_data::LocationFieldAttr::EoHashMapKStringVToErrStringSerde => {
                    let Some((_, second_argument)) = get_hashmap_args() else {
                        return compile_error_token_stream(CompileErrorMessage::from(
                            str_constants::MACRO_DIAGNOSTICS_EXPECTED_HASH_MAP_E9_ERROR,
                        ));
                    };
                    quote::quote! {std::collections::HashMap<#string_token_stream, #second_argument>}
                }
                macros_helpers::location_data::LocationFieldAttr::EoHashMapKStringVLocation => {
                    let Some((_, second_argument)) = get_hashmap_args() else {
                        return compile_error_token_stream(CompileErrorMessage::from(
                            str_constants::MACRO_DIAGNOSTICS_EXPECTED_HASH_MAP_C8_ERROR,
                        ));
                    };
                    let element_hashmap_v_type_with_serde_token_stream =
                        match format!("{}{}", quote::quote! {#second_argument}, with_serde_upper_camel_case)
                            .parse::<proc_macro2::TokenStream>()
                        {
                            Ok(parsed_token_stream) => parsed_token_stream,
                            Err(error) => {
                                return compile_error_token_stream(CompileErrorMessage::from(
                                    &str_constants::COMPILE_ERROR_CE_020
                                        .replace(
                                            str_constants::COMPILE_ERROR_ERROR_PLACEHOLDER,
                                            &error.to_string(),
                                        ),
                                ));
                            }
                        };
                    quote::quote! {std::collections::HashMap<#string_token_stream, #element_hashmap_v_type_with_serde_token_stream>}
                }
            };
                quote::quote! {#field: #element_type_with_serde_token_stream}
            };
            macros_helpers::proc_macro2_tokens::ProcMacro2GeneratedRustTokenStream::from(
                quote::quote! {#ts,},
            )
        };
    let generate_serde_version_of_named_generate_pg_table_variant_token_stream =
        |error_variant: GeneratePgTableVariantEmissionRef<'_>| -> proc_macro2::TokenStream {
            let variant_identifier = error_variant.identifier();
            match error_variant {
                GeneratePgTableVariantEmissionRef::Syn(syn_variant) => {
                    let syn::Fields::Named(fields_named) = &syn_variant.fields else {
                        return compile_error_token_stream(CompileErrorMessage::from(
                            str_constants::MACRO_DIAGNOSTICS_EXPECTED_NAMED_VARIANT_FIELDS_ERROR,
                        ))
                        .into();
                    };
                    let fields_with_serde_token_stream = fields_named.named.iter().map(|field| {
                        let Some(field_identifier) = field.ident.as_ref() else {
                            return compile_error_token_stream(CompileErrorMessage::from(
                                str_constants::MACRO_DIAGNOSTICS_EXPECTED_NAMED_FIELD_ERROR,
                            ));
                        };
                        let location_attr = match generate_pg_table_syn_field_location_attr_stage(
                            SynGeneratePgTableFieldRef::from(field),
                        ) {
                            Ok(v) => v,
                            Err(error) => return error,
                        };
                        generate_serde_field_token_stream(
                            SynGeneratePgTableIdentifierRef::from(field_identifier),
                            SynGeneratePgTableTypeRef::from(&field.ty),
                            GeneratePgTableVariantLocationAttr::from(location_attr),
                        )
                    });
                    quote::quote! {
                        #variant_identifier {
                            #(#fields_with_serde_token_stream)*
                        }
                    }
                }
                GeneratePgTableVariantEmissionRef::Model(model_variant) => {
                    let fields_with_serde_token_stream = model_variant.fields.iter().map(|field| {
                        generate_serde_field_token_stream(
                            SynGeneratePgTableIdentifierRef::from(&field.identifier),
                            SynGeneratePgTableTypeRef::from(&field.type0),
                            GeneratePgTableVariantLocationAttr::from(field.location_attr),
                        )
                    });
                    quote::quote! {
                        #variant_identifier {
                            #(#fields_with_serde_token_stream)*
                        }
                    }
                }
            }
        };
    let generate_identifier_operation_payload_upper_camel_case =
        |operation: &Operation| match &operation {
            Operation::Co => quote::quote! {#identifier_create_upper_camel_case},
            Operation::Uo => quote::quote! {#identifier_update_upper_camel_case},
            Operation::Cm
            | Operation::Rm
            | Operation::Ro
            | Operation::Um
            | Operation::Dm
            | Operation::Dlo => generate_identifier_operation_suffix_token_stream(
                operation,
                &PayloadUpperCamelCase.to_string(),
            ),
        };
    let generate_identifier_operation_parameters_upper_camel_case = |operation: &Operation| {
        generate_identifier_operation_suffix_token_stream(operation, str_constants::PARAMETERS)
    };
    let std_sync_arc_combination_of_app_state_logic_traits_token_stream =
        quote::quote! {std::sync::Arc<dyn pg_table::CombinationOfAppStateLogicTraits>};
    let generate_operation_result_type_token_stream =
        |operation: &Operation| -> &dyn quote::ToTokens {
            match operation {
                Operation::Rm => &vec_struct_opts_identifier_token_stream,
                Operation::Ro => &identifier_read_upper_camel_case,
                Operation::Dm => &vec_primary_key_field_type_read_token_stream,
                Operation::Dlo => &primary_key_field_type_as_pg_type_read_upper_camel_case,
                Operation::Co | Operation::Uo => &identifier_read_ids_upper_camel_case,
                Operation::Cm | Operation::Um => &vec_identifier_read_ids_token_stream,
            }
        };
    let primary_key_field_type_origin_token_stream = if let syn::Type::Path(type_path) =
        &**primary_key_field_type
    {
        let Some(source_last_segment) = type_path.path.segments.last() else {
            return compile_error_token_stream(CompileErrorMessage::from(
                str_constants::COMPILE_ERROR_CE_016,
            ));
        };
        let origin_identifier = quote::format_ident!(
            "{}",
            naming::parameter::SelfOriginUpperCamelCase::from_tokens(&source_last_segment.ident)
                .to_string()
        );
        let mut origin_type_path = type_path.clone();
        let Some(last_segment) = origin_type_path.path.segments.last_mut() else {
            return compile_error_token_stream(CompileErrorMessage::from(
                str_constants::COMPILE_ERROR_CE_052,
            ));
        };
        last_segment.ident = origin_identifier;
        quote::quote! {#origin_type_path}
    } else {
        return compile_error_token_stream(CompileErrorMessage::from(
            str_constants::COMPILE_ERROR_CE_009,
        ));
    };
    fields.iter().fold((), |(), field| {
        let roles: [&dyn quote::ToTokens; 6] = [
            &CreateUpperCamelCase,
            &ReadUpperCamelCase,
            &SelectUpperCamelCase,
            &UpdateUpperCamelCase,
            &UpdateForQueryUpperCamelCase,
            &WhereUpperCamelCase,
        ];
        roles.into_iter().fold((), |(), role| {
            open_api_schema_types_token_stream.push(generate_concrete_pg_type_role_token_stream(
                &field.type0,
                role,
            ));
        });
        let origin_role = quote::format_ident!("Origin");
        open_api_schema_types_token_stream.push(generate_concrete_pg_type_role_token_stream(
            &field.type0,
            &origin_role,
        ));
    });
    open_api_schema_types_token_stream.push(generate_concrete_pg_type_role_token_stream(
        primary_key_field_type,
        &ReadIdsUpperCamelCase,
    ));
    open_api_schema_types_token_stream.extend([
        quote::quote! {#identifier_create_upper_camel_case},
        quote::quote! {#identifier_where_upper_camel_case},
        quote::quote! {#optional_identifier_where_upper_camel_case},
        quote::quote! {#identifier_select_upper_camel_case},
        quote::quote! {#identifier_read_upper_camel_case},
        quote::quote! {#identifier_read_ids_upper_camel_case},
        quote::quote! {#identifier_update_upper_camel_case},
        quote::quote! {#identifier_update_for_query_upper_camel_case},
        quote::quote! {#primary_key_field_type_update_token_stream},
        quote::quote! {#primary_key_field_type_origin_token_stream},
        quote::quote! {location_lib::location::Location},
        quote::quote! {location_lib::location::LocationColumn},
        quote::quote! {location_lib::location::LocationCommit},
        quote::quote! {location_lib::location::LocationFile},
        quote::quote! {location_lib::location::LocationLine},
        quote::quote! {location_lib::location::Occr},
        quote::quote! {location_lib::location::StdLocationDuration},
        quote::quote! {pg_crud_common::Order},
        quote::quote! {pg_crud_common::Operator},
        quote::quote! {pg_crud_common::PgCrudStringWrapperTryFromStringError},
        quote::quote! {pg_crud_common::PaginationBase},
        quote::quote! {pg_crud_common::PaginationLimit},
        quote::quote! {pg_crud_common::PaginationOffset},
        quote::quote! {pg_crud_common::PaginationStartsWithZero},
        quote::quote! {pg_crud_common::QueryPartErrorWithSerde},
        quote::quote! {route_validators::check_body_size::BodySizeErrorWithSerde},
        quote::quote! {route_validators::check_body_size::BodySizeLimitBytes},
        quote::quote! {frontend_contract::ApiProblem},
        quote::quote! {frontend_contract::ApiProblemDetail},
        quote::quote! {frontend_contract::ApiProblemField},
        quote::quote! {frontend_contract::ApiProblemKind},
        quote::quote! {frontend_contract::ApiProblemRequestId},
        quote::quote! {frontend_contract::ApiProblemStatus},
        quote::quote! {frontend_contract::ApiProblemViolation},
        quote::quote! {where_filters::EncodeFormat},
        quote::quote! {where_filters::RegexCase},
        quote::quote! {where_filters::RegexRegex},
        quote::quote! {pg_crud_common::NotZeroUnsignedPartOfI32},
    ]);
    crate::model::OperationDsc::ALL
    .iter()
    .fold((), |(), operation_dsc| {
        let operation = &operation_dsc.operation;
        let idempotency_enabled = generate_pg_table_input_model.config.idempotent_mutations
            && crate::sql::idempotency_capable(operation_dsc);
        let optimistic_concurrency_enabled = optimistic_revision_field_idx.is_some()
            && crate::sql::optimistic_concurrency_capable(operation_dsc);
        let operation_handle_snake_case_token_stream = operation.self_handle_snake_case_token_stream();
        let operation_snake_case_token_stream = operation.self_snake_case_token_stream();
        let operation_snake_case_string = operation.self_snake_case_str();
        let open_api_path_fn_identifier = quote::format_ident!(
            "{}_{}_open_api",
            identifier_snake_case_string,
            operation_snake_case_string
        );
        let open_api_path_type_identifier =
            quote::format_ident!("__generated_path_{open_api_path_fn_identifier}");
        let open_api_path = format!("/{identifier_snake_case_string}/{operation_snake_case_string}");
        let open_api_operation_id =
            format!("{identifier_snake_case_string}_{operation_snake_case_string}");
        let open_api_path_double_quoted_token_stream = generate_quotes::dq_token_stream(&open_api_path);
        let open_api_tag_double_quoted_token_stream = generate_quotes::dq_token_stream(&identifier_snake_case_string);
        let open_api_http_method_token_stream = match crate::openapi::http_method(operation_dsc) {
            OperationHttpMethod::Post => quote::quote! {utoipa::openapi::path::HttpMethod::Post},
            OperationHttpMethod::Patch => {
                quote::quote! {utoipa::openapi::path::HttpMethod::Patch}
            }
            OperationHttpMethod::Delete => {
                quote::quote! {utoipa::openapi::path::HttpMethod::Delete}
            }
        };
        let open_api_status = if crate::openapi::success_status(operation_dsc)
            == macros_helpers::status_code::StatusCode::Created201
        {
            "201"
        } else {
            "200"
        };
        let open_api_status_literal = proc_macro2::Literal::string(open_api_status);
        let open_api_payload_type_token_stream = generate_identifier_operation_payload_upper_camel_case(operation);
        let open_api_response_type_token_stream = generate_identifier_operation_res_variants_upper_camel_case(operation);
        let open_api_payload_schema_ref = proc_macro2::Literal::string(&format!(
            "#/components/schemas/{open_api_payload_type_token_stream}"
        ));
        let open_api_response_schema_ref = proc_macro2::Literal::string(&format!(
            "#/components/schemas/{open_api_response_type_token_stream}"
        ));
        let open_api_extra_params_token_stream = match (idempotency_enabled, optimistic_concurrency_enabled) {
            (true, true) => quote::quote! {
                operation.parameters = Some(vec![
                    utoipa::openapi::path::ParameterBuilder::new()
                        .name("Idempotency-Key")
                        .parameter_in(utoipa::openapi::path::ParameterIn::Header)
                        .required(utoipa::openapi::Required::True)
                        .description(Some("Required key for safely retrying this mutation"))
                        .schema(Some(<String as utoipa::PartialSchema>::schema()))
                        .build(),
                    utoipa::openapi::path::ParameterBuilder::new()
                        .name("If-Match")
                        .parameter_in(utoipa::openapi::path::ParameterIn::Header)
                        .required(utoipa::openapi::Required::True)
                        .description(Some("Required current non-negative row revision"))
                        .schema(Some(<i64 as utoipa::PartialSchema>::schema()))
                        .build(),
                ]);
            },
            (true, false) => quote::quote! {
                operation.parameters = Some(vec![
                    utoipa::openapi::path::ParameterBuilder::new()
                        .name("Idempotency-Key")
                        .parameter_in(utoipa::openapi::path::ParameterIn::Header)
                        .required(utoipa::openapi::Required::True)
                        .description(Some("Required key for safely retrying this mutation"))
                        .schema(Some(<String as utoipa::PartialSchema>::schema()))
                        .build(),
                ]);
            },
            (false, true) => quote::quote! {
                operation.parameters = Some(vec![
                    utoipa::openapi::path::ParameterBuilder::new()
                        .name("If-Match")
                        .parameter_in(utoipa::openapi::path::ParameterIn::Header)
                        .required(utoipa::openapi::Required::True)
                        .description(Some("Required current non-negative row revision"))
                        .schema(Some(<i64 as utoipa::PartialSchema>::schema()))
                        .build(),
                ]);
            },
            (false, false) => proc_macro2::TokenStream::new(),
        };
        let open_api_idempotency_responses_token_stream = if idempotency_enabled {
            quote::quote! {
                add_problem_response("409", "Idempotency key conflicts with another request");
                add_problem_response("425", "An identical request with this key is still running");
            }
        } else {
            proc_macro2::TokenStream::new()
        };
        let open_api_optimistic_responses_token_stream = if optimistic_concurrency_enabled {
            quote::quote! {
                add_problem_response("412", "The supplied row revision is stale");
                add_problem_response("428", "A valid If-Match row revision is required");
            }
        } else {
            proc_macro2::TokenStream::new()
        };
        let (open_api_security_token_stream, open_api_auth_responses_token_stream) = generate_pg_table_input_model
            .config
            .permission_prefix
            .as_ref()
            .map_or_else(
                || (proc_macro2::TokenStream::new(), proc_macro2::TokenStream::new()),
                |_| {
                    (
                        quote::quote! {
                            operation.security = Some(vec![
                                utoipa::openapi::security::SecurityRequirement::default()
                                    .add::<&str, [&str; 0usize], &str>("admin_cookie", []),
                                utoipa::openapi::security::SecurityRequirement::default()
                                    .add::<&str, [&str; 0usize], &str>("admin_csrf", []),
                            ]);
                        },
                        quote::quote! {
                            add_problem_response("401", "Authentication is required");
                            add_problem_response("403", "Required permission is missing");
                            add_problem_response("409", "Resource state conflict");
                            add_problem_response("422", "Request validation failed");
                            add_problem_response("429", "Request rate limit exceeded");
                        },
                    )
                },
            );
        let identifier_operation_parameters_upper_camel_case = generate_identifier_operation_parameters_upper_camel_case(operation);
        let identifier_try_operation_error_upper_camel_case = generate_identifier_try_operation_error_upper_camel_case(operation);
        let result_ok_type_token_stream = generate_operation_result_type_token_stream(operation);
        let try_operation_handle_snake_case_token_stream = operation.try_self_handle_snake_case_token_stream();
        let operation_client_method_snake_case_token_stream = operation.self_snake_case_token_stream();
        let frontend_idempotency_request_token_stream = if idempotency_enabled {
            quote::quote! {
                .with_idempotency_key(match frontend_contract::TransportIdempotencyKey::try_from(pg_table::new_pg_table_idempotency_key().as_ref().to_owned()) {
                    Ok(value) => value,
                    Err(error) => return Err(frontend_contract::ClientError::Encode(frontend_contract::FormValueError::try_from(error.to_string()).unwrap_or_default())),
                })
            }
        } else {
            proc_macro2::TokenStream::new()
        };
        let optimistic_client_param_token_stream = if optimistic_concurrency_enabled {
            quote::quote! {optimistic_revision_9f023d8e: pg_table::PgTableRevision,}
        } else {
            proc_macro2::TokenStream::new()
        };
        let optimistic_client_arg_token_stream = if optimistic_concurrency_enabled {
            quote::quote! {optimistic_revision_9f023d8e,}
        } else {
            proc_macro2::TokenStream::new()
        };
        let frontend_optimistic_request_token_stream = if optimistic_concurrency_enabled {
            quote::quote! {
                .with_if_match(match frontend_contract::TransportIfMatch::try_from(optimistic_revision_9f023d8e.to_string()) {
                    Ok(value) => value,
                    Err(error) => return Err(frontend_contract::ClientError::Encode(frontend_contract::FormValueError::try_from(error.to_string()).unwrap_or_default())),
                })
            }
        } else {
            proc_macro2::TokenStream::new()
        };
        if operation_is_enabled(operation) {
            api_client_methods_token_stream.push(quote::quote! {
                pub async fn #operation_client_method_snake_case_token_stream(
                    &self,
                    #ParametersSnakeCase: #identifier_operation_parameters_upper_camel_case,
                    #optimistic_client_param_token_stream
                ) -> Result<#result_ok_type_token_stream, #identifier_try_operation_error_upper_camel_case> {
                    #identifier::#try_operation_handle_snake_case_token_stream(
                        &self.client,
                        self.endpoint.as_url().as_str(),
                        #ParametersSnakeCase,
                        #optimistic_client_arg_token_stream
                        #identifier::#TableNameSnakeCase(),
                    ).await
                }
            });
            let operation_identifier = quote::format_ident!("{}", operation.to_string());
            let operation_payload_example_client_method =
                operation.operation_payload_example_snake_case();
            frontend_api_client_methods_token_stream.push(quote::quote! {
                pub async fn #operation_client_method_snake_case_token_stream(
                    &self,
                    #ParametersSnakeCase: #identifier_operation_parameters_upper_camel_case,
                    #optimistic_client_param_token_stream
                ) -> Result<#result_ok_type_token_stream, frontend_contract::ClientError> {
                    let route = #identifier_route_contract_upper_camel_case::ALL
                        .into_iter()
                        .find(|route| route.operation() == #identifier_operation_upper_camel_case::#operation_identifier)
                        .ok_or(frontend_contract::ClientError::UnexpectedResponse)?;
                    let body_bytes = serde_json::to_vec(&#ParametersSnakeCase.#PayloadSnakeCase)
                        .map_err(|error| frontend_contract::ClientError::Encode(frontend_contract::FormValueError::try_from(error.to_string()).unwrap_or_default()))?;
                    let body = frontend_contract::TransportBody::try_from(body_bytes)
                        .map_err(|error| frontend_contract::ClientError::Encode(frontend_contract::FormValueError::try_from(error.to_string()).unwrap_or_default()))?;
                    let request = frontend_contract::TransportRequest::new(
                        body,
                        frontend_contract::TransportPath::try_from(route.path().to_owned())
                            .map_err(|error| frontend_contract::ClientError::Encode(frontend_contract::FormValueError::try_from(error.to_string()).unwrap_or_default()))?,
                        route.frontend_contract(),
                    )#frontend_idempotency_request_token_stream #frontend_optimistic_request_token_stream;
                    let response = self
                        .transport
                        .send(request)
                        .await
                        .map_err(frontend_contract::ClientError::Transport)?;
                    let response_body = response.success_body(
                        route.frontend_contract().success_status().transport_status(),
                    )?;
                    let decoded = serde_json::from_slice::<#open_api_response_type_token_stream>(
                        response_body.as_ref(),
                    )
                    .map_err(|error| frontend_contract::ClientError::Decode(frontend_contract::FormValueError::try_from(error.to_string()).unwrap_or_default()))?;
                    match decoded {
                        #open_api_response_type_token_stream::#DesirableUpperCamelCase(value) => Ok(value),
                        _ => Err(frontend_contract::ClientError::UnexpectedResponse),
                    }
                }
                pub async fn #operation_payload_example_client_method(
                    &self,
                ) -> Result<#open_api_payload_type_token_stream, frontend_contract::ClientError> {
                    let route = #identifier_route_contract_upper_camel_case::ALL
                        .into_iter()
                        .find(|route| route.operation() == #identifier_operation_upper_camel_case::#operation_identifier)
                        .map(#identifier_route_contract_upper_camel_case::payload_example)
                        .ok_or(frontend_contract::ClientError::UnexpectedResponse)?;
                    let body = frontend_contract::TransportBody::try_from(Vec::new())
                        .map_err(|error| frontend_contract::ClientError::Encode(frontend_contract::FormValueError::try_from(error.to_string()).unwrap_or_default()))?;
                    let request = frontend_contract::TransportRequest::new(
                        body,
                        frontend_contract::TransportPath::try_from(route.path().to_owned())
                            .map_err(|error| frontend_contract::ClientError::Encode(frontend_contract::FormValueError::try_from(error.to_string()).unwrap_or_default()))?,
                        route.frontend_contract(),
                    );
                    let response = self
                        .transport
                        .send(request)
                        .await
                        .map_err(frontend_contract::ClientError::Transport)?;
                    let response_body = response.success_body(
                        route.frontend_contract().success_status().transport_status(),
                    )?;
                    serde_json::from_slice::<#open_api_payload_type_token_stream>(
                        response_body.as_ref(),
                    )
                    .map_err(|error| frontend_contract::ClientError::Decode(frontend_contract::FormValueError::try_from(error.to_string()).unwrap_or_default()))
                }
            });
            open_api_path_fn_identifiers.push(open_api_path_fn_identifier);
            open_api_schema_types_token_stream.push(open_api_payload_type_token_stream);
            open_api_schema_types_token_stream.push(open_api_response_type_token_stream);
        }
        let application_json_double_quoted_token_stream =
            generate_quotes::dq_token_stream(&str_constants::APPLICATION_JSON);
        let open_api_path_fn_token_stream = quote::quote! {
            #[allow(non_camel_case_types)]
            pub struct #open_api_path_type_identifier;
            impl utoipa::__dev::PathConfig for #open_api_path_type_identifier {
                fn methods() -> Vec<utoipa::openapi::path::HttpMethod> {
                    vec![#open_api_http_method_token_stream]
                }
                fn path() -> String {
                    #open_api_path_double_quoted_token_stream.to_owned()
                }
                fn tags_and_operation() -> (
                    Vec<&'static str>,
                    utoipa::openapi::path::Operation,
                ) {
                    let mut operation = utoipa::openapi::path::Operation::new();
                    operation.operation_id = Some(#open_api_operation_id.to_owned());
                    operation.request_body = Some(
                        utoipa::openapi::request_body::RequestBodyBuilder::new()
                            .content(
                                #application_json_double_quoted_token_stream,
                                utoipa::openapi::Content::new(Some(
                                    utoipa::openapi::Ref::new(#open_api_payload_schema_ref),
                                )),
                            )
                            .build(),
                    )
                    ;
                    operation.responses.responses.insert(
                        #open_api_status_literal.to_owned(),
                        utoipa::openapi::ResponseBuilder::new()
                            .description("Successful response")
                            .content(
                                #application_json_double_quoted_token_stream,
                                utoipa::openapi::Content::new(Some(
                                    utoipa::openapi::Ref::new(#open_api_response_schema_ref),
                                )),
                            )
                            .build()
                            .into(),
                    );
                    {
                        let mut add_problem_response = |status: &str, description: &str| {
                            operation.responses.responses.insert(
                                status.to_owned(),
                                utoipa::openapi::ResponseBuilder::new()
                                    .description(description)
                                    .content(
                                        #application_json_double_quoted_token_stream,
                                        utoipa::openapi::Content::new(Some(
                                            <frontend_contract::ApiProblem as utoipa::PartialSchema>::schema(),
                                        )),
                                    )
                                    .build()
                                    .into(),
                            );
                        };
                        add_problem_response("400", "Invalid request");
                        add_problem_response("413", "Request body is too large");
                        #open_api_idempotency_responses_token_stream
                        #open_api_optimistic_responses_token_stream
                        #open_api_auth_responses_token_stream
                        add_problem_response("500", "Internal server error");
                    }
                    #open_api_security_token_stream
                    #open_api_extra_params_token_stream
                    (vec![#open_api_tag_double_quoted_token_stream], operation)
                }
            }
        };
        open_api_path_token_stream.push(open_api_path_fn_token_stream);
        let generate_for_element_in_update_for_query_vec_token_stream = |ts: &dyn quote::ToTokens| {
            quote::quote! {
                for element_a72f3eac in &#UpdateForQueryVecSnakeCase {
                    #ts
                }
            }
        };
        let operation_error_initialization_query_part_token_stream =
            generate_operation_error_initialization_eprintln_res_token_stream(operation, &query_part_syn_variant, std::panic::Location::caller());
        let generate_match_ok_err_update_token_stream = |ts0: &dyn quote::ToTokens, ts1: &dyn quote::ToTokens| {
            generate_match_ok_err_short_token_stream(&ts0, &ts1, &quote::quote! {{#operation_error_initialization_query_part_token_stream}})
        };
        let generate_for_element_in_update_for_query_vec_field_token_stream =
            |field: &dyn quote::ToTokens, ts0: &dyn quote::ToTokens, ts1: &dyn quote::ToTokens| {
                generate_for_element_in_update_for_query_vec_token_stream(&generate_if_let_some_token_stream(
                    &ts0,
                    &quote::quote! {&element_a72f3eac.#field},
                    &ts1,
                ))
            };
        let type_variants_from_req_res_syn_variants = {
            let error_variants_len = generate_pg_table_input_model
                .error_variants_by_attr
                .get(&operation.generate_pg_table_attr_error_variants())
                .map_or(0usize, Vec::len);
            let mut accumulator = Vec::with_capacity(
                common_route_syn_variants
                    .len()
                    .saturating_add(error_variants_len)
                    .saturating_add(4usize),
            );
            accumulator.extend_from_slice(common_route_syn_variants.as_slice());
            if let Operation::Rm | Operation::Ro = &operation {
                accumulator.push(GeneratePgTableVariantEmissionRef::Syn(
                    not_unique_field_syn_variant.get_syn_variant(),
                ));
            }
            if let Operation::Cm | Operation::Rm | Operation::Ro | Operation::Co | Operation::Um | Operation::Uo | Operation::Dm = &operation {
                accumulator.push(GeneratePgTableVariantEmissionRef::Syn(query_part_syn_variant.get_syn_variant()));
            }
            if let Operation::Cm | Operation::Dlo | Operation::Co | Operation::Um | Operation::Uo | Operation::Dm = &operation {
                accumulator.push(GeneratePgTableVariantEmissionRef::Syn(
                    row_and_rollback_syn_variant.get_syn_variant(),
                ));
            }
            accumulator.push(GeneratePgTableVariantEmissionRef::Syn(try_bind_syn_variant.get_syn_variant()));
            if let Some(variants) = generate_pg_table_input_model
                .error_variants_by_attr
                .get(&operation.generate_pg_table_attr_error_variants())
            {
                accumulator.extend(variants.iter().map(GeneratePgTableVariantEmissionRef::Model));
            }
            accumulator
        };
        if operation_is_enabled(operation) {
        operation_routes_token_stream.push({
            let method_token_stream = match crate::route::http_method(operation_dsc) {
                OperationHttpMethod::Post => quote::quote! {post},
                OperationHttpMethod::Patch => quote::quote! {patch},
                OperationHttpMethod::Delete => quote::quote! {delete},
            };
            let operation_payload_example_snake_case =
                operation.operation_payload_example_snake_case();
            let (
                slash_operation_double_quoted_token_stream,
                slash_operation_payload_example_double_quoted_token_stream
            ) = {
                let generate_token_stream = |
                    v: &dyn std::fmt::Display
                | generate_quotes::dq_token_stream(&format!("/{v}"));
                (
                    generate_token_stream(&operation.self_snake_case_str()),
                    generate_token_stream(&operation_payload_example_snake_case)
                )
            };
            quote::quote! {
                .route(#slash_operation_double_quoted_token_stream, axum::routing::#method_token_stream({
                    let table_owned = #db_table_snake_case.to_owned();
                    let requests_metric = metrics::counter!("pg_table_requests_total", "table" => #identifier_snake_case_double_quoted_token_stream, "operation" => #operation_snake_case_string);
                    let duration_metric = metrics::histogram!("pg_table_request_duration_seconds", "table" => #identifier_snake_case_double_quoted_token_stream, "operation" => #operation_snake_case_string);
                    let response_200_metric = metrics::counter!("pg_table_responses_total", "table" => #identifier_snake_case_double_quoted_token_stream, "operation" => #operation_snake_case_string, "status" => "200");
                    let response_201_metric = metrics::counter!("pg_table_responses_total", "table" => #identifier_snake_case_double_quoted_token_stream, "operation" => #operation_snake_case_string, "status" => "201");
                    let response_400_metric = metrics::counter!("pg_table_responses_total", "table" => #identifier_snake_case_double_quoted_token_stream, "operation" => #operation_snake_case_string, "status" => "400");
                    let response_409_metric = metrics::counter!("pg_table_responses_total", "table" => #identifier_snake_case_double_quoted_token_stream, "operation" => #operation_snake_case_string, "status" => "409");
                    let response_412_metric = metrics::counter!("pg_table_responses_total", "table" => #identifier_snake_case_double_quoted_token_stream, "operation" => #operation_snake_case_string, "status" => "412");
                    let response_413_metric = metrics::counter!("pg_table_responses_total", "table" => #identifier_snake_case_double_quoted_token_stream, "operation" => #operation_snake_case_string, "status" => "413");
                    let response_425_metric = metrics::counter!("pg_table_responses_total", "table" => #identifier_snake_case_double_quoted_token_stream, "operation" => #operation_snake_case_string, "status" => "425");
                    let response_428_metric = metrics::counter!("pg_table_responses_total", "table" => #identifier_snake_case_double_quoted_token_stream, "operation" => #operation_snake_case_string, "status" => "428");
                    let response_500_metric = metrics::counter!("pg_table_responses_total", "table" => #identifier_snake_case_double_quoted_token_stream, "operation" => #operation_snake_case_string, "status" => "500");
                    let response_other_metric = metrics::counter!("pg_table_responses_total", "table" => #identifier_snake_case_double_quoted_token_stream, "operation" => #operation_snake_case_string, "status" => "other");
                    async move |
                        app_state_99328dfe: axum::extract::State<#std_sync_arc_combination_of_app_state_logic_traits_token_stream>,
                        req: axum::extract::Request
                    | {
                        let started_at = std::time::Instant::now();
                        requests_metric.increment(1u64);
                        let response_2b9f176e = tracing::Instrument::instrument(
                            Self::#operation_handle_snake_case_token_stream(app_state_99328dfe, req, &table_owned),
                            tracing::info_span!(
                                "pg_table.operation",
                                table = %table_owned,
                                operation = #operation_snake_case_string,
                            ),
                        ).await;
                        let response = if response_2b9f176e.status().is_success() {
                            response_2b9f176e
                        } else {
                            let status_59091f23 = response_2b9f176e.status();
                            let problem_error_0ae0baf4 = frontend_contract::ApiProblemError::from_status(
                                frontend_contract::ApiProblemStatus::try_from(status_59091f23.as_u16())
                                    .unwrap_or_else(|_error| frontend_contract::ApiProblemStatus::from(frontend_contract::KnownHttpStatus::InternalServerError)),
                            );
                            axum::response::IntoResponse::into_response(problem_error_0ae0baf4)
                        };
                        duration_metric.record(started_at.elapsed().as_secs_f64());
                        match response.status().as_u16() {
                            200u16 => response_200_metric.increment(1u64),
                            201u16 => response_201_metric.increment(1u64),
                            400u16 => response_400_metric.increment(1u64),
                            409u16 => response_409_metric.increment(1u64),
                            412u16 => response_412_metric.increment(1u64),
                            413u16 => response_413_metric.increment(1u64),
                            425u16 => response_425_metric.increment(1u64),
                            428u16 => response_428_metric.increment(1u64),
                            500u16 => response_500_metric.increment(1u64),
                            _ => response_other_metric.increment(1u64),
                        }
                        response
                    }
                }))
                .route(#slash_operation_payload_example_double_quoted_token_stream, axum::routing::get(async move||Self::#operation_payload_example_snake_case()))
            }
        });
        }
        impl_identifier_vec_token_stream.push({
            let try_operation_token_stream = {
                let try_operation_snake_case_token_stream = operation.try_self_snake_case_token_stream();
                let payload_token_stream = {
                    let ts = generate_match_ok_err_short_token_stream(
                        &quote::quote! {serde_json::to_string(&#ParametersSnakeCase.#PayloadSnakeCase)},
                        &quote::quote! {v_1772a83e},
                        &{
                            let ts = generate_initialization_token_stream(&serde_json_to_string_syn_variant, std::panic::Location::caller());
                            quote::quote! {{
                                return Err(#identifier_try_operation_error_upper_camel_case::#ts);
                            }}
                        },
                    );
                    quote::quote! {
                        let #PayloadSnakeCase = {
                            #ts
                        };
                    }
                };
                let url_token_stream = {
                    let format_token_stream = generate_quotes::dq_token_stream(&format!(
                        "{{endpoint_location}}/{{table}}/{}",
                        operation.self_snake_case_str()
                    ));
                    quote::quote! {let #UrlSnakeCase = format!(#format_token_stream);}
                };
                let future_token_stream = {
                    let operation_http_method_snake_case_token_stream =
                        naming_common::AsRefStrToSnakeCaseTokenStream::case_or_panic(&crate::client::http_method(operation_dsc));
                    let commit_header_addition_token_stream = quote::quote! {
                        .header(
                            &"commit".to_owned(),
                            git_info::project_git_info().commit().as_ref(),
                        )
                    };
                    let app_json_double_quoted_token_stream = generate_quotes::dq_token_stream(&str_constants::APPLICATION_JSON);
                    let content_type_app_json_header_addition_token_stream = quote::quote! {
                        .header(reqwest::header::CONTENT_TYPE, #app_json_double_quoted_token_stream)
                    };
                    let idempotency_header_addition_token_stream = if idempotency_enabled {
                        quote::quote! {
                            .header("idempotency-key", pg_table::new_pg_table_idempotency_key().as_ref())
                        }
                    } else {
                        proc_macro2::TokenStream::new()
                    };
                    let optimistic_header_addition_token_stream = if optimistic_concurrency_enabled {
                        quote::quote! {
                            .header(http::header::IF_MATCH, optimistic_revision_9f023d8e.to_string())
                        }
                    } else {
                        proc_macro2::TokenStream::new()
                    };
                    quote::quote! {
                        let #FutureSnakeCase = #client_snake_case
                            .#operation_http_method_snake_case_token_stream(&#UrlSnakeCase)
                            #commit_header_addition_token_stream
                            #content_type_app_json_header_addition_token_stream
                            #idempotency_header_addition_token_stream
                            #optimistic_header_addition_token_stream
                            .#BodySnakeCase(#PayloadSnakeCase)
                            .send();
                    }
                };
                let res_token_stream = {
                    let ts =
                        generate_match_ok_err_short_token_stream(&quote::quote! {#FutureSnakeCase.await}, &quote::quote! {v_180559e9}, &{
                            let ts = generate_initialization_token_stream(&reqwest_syn_variant, std::panic::Location::caller());
                            quote::quote! {{
                                return Err(#identifier_try_operation_error_upper_camel_case::#ts);
                            }}
                        });
                    quote::quote! {let #ResSnakeCase = #ts;}
                };
                let error_0_res_status_token_stream = quote::quote! {
                    let #Error0 = #ResSnakeCase.status();
                };
                let headers_token_stream = quote::quote! {
                    let #Error1 = #ResSnakeCase.headers().clone();
                };
                let res_text_token_stream = {
                    let ts = generate_match_ok_err_token_stream(
                        &quote::quote! {#ResSnakeCase.text().await},
                        &quote::quote! {v_6a62b2b9},
                        &quote::quote! {v_6a62b2b9},
                        &Error2,
                        &{
                            let failed_to_get_res_text_syn_variant_initialization_token_stream =
                                generate_initialization_token_stream(&failed_to_get_res_text_syn_variant, std::panic::Location::caller());
                            quote::quote! {{
                                return Err(#identifier_try_operation_error_upper_camel_case::#failed_to_get_res_text_syn_variant_initialization_token_stream);
                            }}
                        },
                    );
                    quote::quote! {let #Error2 = #ts;}
                };
                let identifier_operation_res_variants_upper_camel_case = generate_identifier_operation_res_variants_upper_camel_case(operation);
                let expected_res_token_stream = {
                    let deserialize_res_syn_variant_initialization_token_stream =
                        generate_initialization_token_stream(&deserialize_res_syn_variant, std::panic::Location::caller());
                    let ts = generate_match_ok_err_token_stream(
                        &quote::quote! {serde_json::from_str::<#identifier_operation_res_variants_upper_camel_case>(&#Error2)},
                        &quote::quote! {v_563d2a75},
                        &quote::quote! {v_563d2a75},
                        &Error3,
                        &quote::quote! {{
                            return Err(#identifier_try_operation_error_upper_camel_case::#deserialize_res_syn_variant_initialization_token_stream);
                        }},
                    );
                    quote::quote! {let #ExpectedResSnakeCase = #ts;}
                };
                let try_operation_logic_error_with_serde_upper_camel_case =
                    generate_identifier_operation_error_with_serde_upper_camel_case(operation);
                let operation_error_with_serde_snake_case = &operation.operation_error_with_serde_snake_case();
                let try_operation_logic_error_with_serde_token_stream = {
                    let try_operation_logic_res_variants_to_try_operation_logic_error_with_serde = type_variants_from_req_res_syn_variants.iter().map(|element| {
                            let variant_identifier = element.identifier();
                            let fields_idents_token_stream = match *element {
                                GeneratePgTableVariantEmissionRef::Syn(syn_variant) => {
                                    let syn::Fields::Named(fields_named) = &syn_variant.fields else {
                                        return compile_error_token_stream(CompileErrorMessage::from(
                                            str_constants::COMPILE_ERROR_CE_025,
                                        ))
                                        .into();
                                    };
                                    let fields_identifiers = fields_named.named.iter().map(|field| &field.ident);
                                    quote::quote! {#(#fields_identifiers),*}
                                }
                                GeneratePgTableVariantEmissionRef::Model(model_variant) => {
                                let fields_identifiers = model_variant.fields.iter().map(|field| &field.identifier);
                                    quote::quote! {#(#fields_identifiers),*}
                                }
                            };
                            quote::quote! {
                                #identifier_operation_res_variants_upper_camel_case::#variant_identifier {
                                    #fields_idents_token_stream
                                } => #try_operation_logic_error_with_serde_upper_camel_case::#variant_identifier { #fields_idents_token_stream }
                            }
                        });
                    quote::quote! {
                        let #operation_error_with_serde_snake_case = match #ExpectedResSnakeCase {
                            #identifier_operation_res_variants_upper_camel_case::#DesirableUpperCamelCase(#VSnakeCase) => {
                                return Ok(#VSnakeCase);
                            },
                            #(#try_operation_logic_res_variants_to_try_operation_logic_error_with_serde),*
                        };
                    }
                };
                let return_error_token_stream = {
                    let field_location_new_token_stream = macros_helpers::generate_field_location_new_token_stream::generate_field_location_new_token_stream(
                        macros_helpers::generate_field_location_new_token_stream::FieldLocationFile::from(file!()),
                        macros_helpers::generate_field_location_new_token_stream::FieldLocationLine::try_from(line!())
                            .unwrap_or_else(|_error| macros_helpers::generate_field_location_new_token_stream::FieldLocationLine::first()),
                        macros_helpers::generate_field_location_new_token_stream::FieldLocationColumn::try_from(column!())
                            .unwrap_or_else(|_error| macros_helpers::generate_field_location_new_token_stream::FieldLocationColumn::first()),
                    );
                    quote::quote! {
                        Err(#identifier_try_operation_error_upper_camel_case::#try_operation_logic_error_with_serde_upper_camel_case {
                            #operation_error_with_serde_snake_case,
                            #field_location_new_token_stream,
                        })
                    }
                };
                quote::quote! {
                    async fn #try_operation_handle_snake_case_token_stream(
                        #client_snake_case: &reqwest::Client,
                        #EndpointLocationSnakeCase: #RefStr,
                        #ParametersSnakeCase: #identifier_operation_parameters_upper_camel_case,
                        #optimistic_client_param_token_stream
                        #TableSnakeCase: &str,
                    ) -> Result<#result_ok_type_token_stream, #identifier_try_operation_error_upper_camel_case> {
                        #payload_token_stream
                        #url_token_stream
                        #future_token_stream
                        #res_token_stream
                        #error_0_res_status_token_stream
                        #headers_token_stream
                        #res_text_token_stream
                        #expected_res_token_stream
                        #try_operation_logic_error_with_serde_token_stream
                        #return_error_token_stream
                    }
                    pub async fn #try_operation_snake_case_token_stream(
                        #EndpointLocationSnakeCase: #RefStr,
                        #ParametersSnakeCase: #identifier_operation_parameters_upper_camel_case,
                        #optimistic_client_param_token_stream
                    ) -> Result<#result_ok_type_token_stream, #identifier_try_operation_error_upper_camel_case> {
                        let #client_snake_case = reqwest::Client::new();
                        Self::#try_operation_handle_snake_case_token_stream(
                            &#client_snake_case,
                            #EndpointLocationSnakeCase,
                            #ParametersSnakeCase,
                            #optimistic_client_arg_token_stream
                            #self_table_name_call_token_stream
                        ).await
                    }
                }
            };
            let operation_handle_token_stream = {
                let req_parts_preparation_token_stream = {
                    let idempotency_metadata_token_stream = if idempotency_enabled {
                        quote::quote! {
                            let idempotency_actor_5d99d3d2 = match parts.extensions.get::<pg_table::PgTableIdempotencyActor>() {
                                Some(value) => value.clone(),
                                None => match pg_table::PgTableIdempotencyActor::try_from("anonymous".to_owned()) {
                                    Ok(value) => value,
                                    Err(_error) => {
                                        return axum::response::IntoResponse::into_response(http::StatusCode::INTERNAL_SERVER_ERROR);
                                    }
                                },
                            };
                            let idempotency_method_4c3bc5ac = parts.method.as_str().to_owned();
                            let idempotency_route_a66541e9 = parts.uri.path().to_owned();
                        }
                    } else {
                        proc_macro2::TokenStream::new()
                    };
                    let idempotency_request_token_stream = if idempotency_enabled {
                        quote::quote! {
                            let idempotency_key_text_31ae975a = {
                                let mut values_4c39e88b = headers.get_all("idempotency-key").iter();
                                match (values_4c39e88b.next(), values_4c39e88b.next()) {
                                    (Some(value), None) => match value.to_str() {
                                        Ok(value) => value.to_owned(),
                                        Err(_error) => return axum::response::IntoResponse::into_response(http::StatusCode::BAD_REQUEST),
                                    },
                                    (None | Some(_), Some(_)) | (None, None) => return axum::response::IntoResponse::into_response(http::StatusCode::BAD_REQUEST),
                                }
                            };
                            let idempotency_scope_8af2bd7d = match (
                                pg_table::PgTableIdempotencyMethod::try_from(idempotency_method_4c3bc5ac),
                                pg_table::PgTableIdempotencyRoute::try_from(idempotency_route_a66541e9),
                                pg_table::PgTableIdempotencyKey::try_from(idempotency_key_text_31ae975a),
                            ) {
                                (Ok(method), Ok(route), Ok(key)) => pg_table::PgTableIdempotencyScope::new(idempotency_actor_5d99d3d2, method, route, key),
                                (Err(_error), _, _) | (_, Err(_error), _) | (_, _, Err(_error)) => return axum::response::IntoResponse::into_response(http::StatusCode::BAD_REQUEST),
                            };
                            let idempotency_request_0a0ae019 = pg_table::PgTableIdempotencyRequest::new(idempotency_scope_8af2bd7d, pg_table::PgTableIdempotencyBodyRef::from(body_bytes.as_ref()));
                        }
                    } else {
                        proc_macro2::TokenStream::new()
                    };
                    let optimistic_revision_token_stream = if optimistic_concurrency_enabled {
                        quote::quote! {
                            let optimistic_revision_9f023d8e = {
                                let mut values_c2818fdc = headers.get_all(http::header::IF_MATCH).iter();
                                match (values_c2818fdc.next(), values_c2818fdc.next()) {
                                    (Some(value), None) => match value.to_str().ok().map(str::to_owned).and_then(|value| pg_table::PgTableRevision::try_from(value).ok()) {
                                        Some(value) => value,
                                        None => return axum::response::IntoResponse::into_response(http::StatusCode::PRECONDITION_REQUIRED),
                                    },
                                    (None | Some(_), Some(_)) | (None, None) => return axum::response::IntoResponse::into_response(http::StatusCode::PRECONDITION_REQUIRED),
                                }
                            };
                        }
                    } else {
                        proc_macro2::TokenStream::new()
                    };
                    let ts0 = &generate_operation_error_initialization_eprintln_res_token_stream(
                        operation,
                        &header_cnt_type_app_json_not_found_syn_variant,
                        std::panic::Location::caller(),
                    );
                    let ts1 = generate_match_ok_err_short_token_stream(
                        &quote::quote! {route_validators::check_body_size::check_body_size(#BodySnakeCase, *#AppStateSnakeCase.get_maximum_size_of_http_body_in_bytes()).await},
                        &quote::quote! {v_cfac9140},
                        &{
                            let ts = generate_operation_error_initialization_eprintln_res_token_stream(
                                operation,
                                &check_body_size_syn_variant,
                                std::panic::Location::caller(),
                            );
                            quote::quote! {{#ts}}
                        },
                    );
                    quote::quote! {
                        let (parts, #BodySnakeCase) = #ReqSnakeCase.into_parts();
                        #idempotency_metadata_token_stream
                        let headers = parts.headers;
                        if !matches!(
                            headers.get(http::header::CONTENT_TYPE),
                            Some(v_e3f6eecd) if v_e3f6eecd == http::header::HeaderValue::from_static("application/json")
                        ) {
                            #ts0
                        }
                        let body_bytes = #ts1;
                        #idempotency_request_token_stream
                        #optimistic_revision_token_stream
                    }
                };
                let idempotency_begin_token_stream = if idempotency_enabled {
                    quote::quote! {
                        let idempotency_pool_193acb3c = app_state::GetSqlxPgPool::get_sqlx_pg_pool(#AppStateSnakeCase.as_ref());
                        match pg_table::begin_pg_table_idempotency(app_state::SqlxPgPoolRef::from(idempotency_pool_193acb3c.as_ref()), &idempotency_request_0a0ae019).await {
                            Ok(pg_table::PgTableIdempotencyBegin::Acquired) => {}
                            Ok(pg_table::PgTableIdempotencyBegin::Conflict) => return axum::response::IntoResponse::into_response(http::StatusCode::CONFLICT),
                            Ok(pg_table::PgTableIdempotencyBegin::InProgress) => return axum::response::IntoResponse::into_response(http::StatusCode::TOO_EARLY),
                            Ok(pg_table::PgTableIdempotencyBegin::Replay(replay)) => {
                                let (status, body) = replay.into_parts();
                                let status = match http::StatusCode::from_u16(u16::from(status)) {
                                    Ok(value) => value,
                                    Err(_error) => return axum::response::IntoResponse::into_response(http::StatusCode::INTERNAL_SERVER_ERROR),
                                };
                                let mut response = axum::response::Response::new(axum::body::Body::from(body.as_ref().to_vec()));
                                *response.status_mut() = status;
                                let _previous_content_type = response.headers_mut().insert(http::header::CONTENT_TYPE, http::HeaderValue::from_static("application/json"));
                                return response;
                            }
                            Err(_error) => return axum::response::IntoResponse::into_response(http::StatusCode::INTERNAL_SERVER_ERROR),
                        }
                    }
                } else {
                    proc_macro2::TokenStream::new()
                };
                let extra_validators_token_stream = {
                    let common_logic_token_stream = generate_logic_token_stream(GeneratePgTableAttr::CommonLogic);
                    let operation_logic_token_stream = generate_logic_token_stream(operation.generate_pg_table_attr_logic());
                    quote::quote! {
                        #common_logic_token_stream
                        #operation_logic_token_stream
                    }
                };
                let parameters_logic_token_stream = {
                    let parameters_logic_ts0 = {
                        //todo in case of large type there is a stackoverflow. for example it was a 3.5md json file gend by cm_payload_example. 3400 fields = success. 16000 = stackoverflow
                        let ts = generate_match_ok_err_short_token_stream(
                            &{
                                let identifier_operation_payload_upper_camel_case =
                                    generate_identifier_operation_payload_upper_camel_case(operation);
                                quote::quote! {serde_json::from_slice::<#identifier_operation_payload_upper_camel_case>(&#BodyBytesSnakeCase)}
                            },
                            &quote::quote! {v_9e6fcd2d},
                            &{
                                let ts = generate_operation_error_initialization_eprintln_res_token_stream(
                                    operation,
                                    &serde_json_syn_variant,
                                    std::panic::Location::caller(),
                                );
                                quote::quote! {{#ts}}
                            },
                        );
                        quote::quote! {
                            let #ParametersSnakeCase = #identifier_operation_parameters_upper_camel_case {
                                #PayloadSnakeCase: #ts
                            };
                        }
                    };
                    let bulk_reservation_token_stream = quote::quote! {
                        let _bulk_resource_reservation_6416eead = match server_runtime_http::GetBulkItemResourceBudget::get_bulk_item_resource_budget(#AppStateSnakeCase.as_ref()).reserve(
                            server_runtime_http::ResourceBudgetAmount::from(#ParametersSnakeCase.#PayloadSnakeCase.as_slice().len()),
                        ) {
                            Ok(value) => value,
                            Err(_error) => return axum::response::IntoResponse::into_response(http::StatusCode::TOO_MANY_REQUESTS),
                        };
                    };
                    match &operation {
                        Operation::Cm => quote::quote! {
                            #parameters_logic_ts0
                            #bulk_reservation_token_stream
                        },
                        Operation::Co
                        | Operation::Rm
                        | Operation::Ro
                        | Operation::Dm
                        | Operation::Dlo => parameters_logic_ts0,
                        Operation::Um => quote::quote! {
                            #parameters_logic_ts0
                            #bulk_reservation_token_stream
                            let #UpdateForQueryVecSnakeCase = #ParametersSnakeCase.#PayloadSnakeCase.into_vec().into_iter()
                            .map(#identifier_update_for_query_upper_camel_case::#FromHSnakeCase)
                            .collect::<Vec<#identifier_update_for_query_upper_camel_case>>();
                        },
                        Operation::Uo => quote::quote! {
                            #parameters_logic_ts0
                            let #UpdateForQuerySnakeCase = #identifier_update_for_query_upper_camel_case::#FromHSnakeCase(#ParametersSnakeCase.#PayloadSnakeCase);
                        },
                    }
                };
                let query_string_token_stream = {
                    let generate_match_ok_err_query_part_token_stream =
                        |ts0: &dyn quote::ToTokens, ts1: &dyn quote::ToTokens, ts2: &dyn quote::ToTokens, ts3: &dyn quote::ToTokens| {
                            generate_match_ok_err_token_stream(&ts0, &ts1, &ts2, &ts3, &quote::quote! {{#operation_error_initialization_query_part_token_stream}})
                        };
                    let write_into_buffer_query_part_syn_variant_error_initialization_eprintln_res_creation_token_stream = {
                        let query_part_error_write_into_buffer_token_stream =
                            pg_crud_macros_common::generate_query_part_error_write_into_buffer_token_stream(import);
                        quote::quote! {
                            let #Error0 = #query_part_error_write_into_buffer_token_stream;
                            #operation_error_initialization_query_part_token_stream
                        }
                    };
                    let increment_initialization_token_stream = quote::quote! {let mut #IncrementSnakeCase: u64 = 0;};
                    let column_names_double_quoted_token_stream = generate_quotes::dq_token_stream(&{
                        let mut accumulator = fields.iter().fold(
                            String::with_capacity(fields.len().saturating_mul(32)),
                            |mut acc0, element| {
                                assert!(
                                    std::fmt::Write::write_fmt(
                                        &mut acc0,
                                        format_args!("{}", element.identifier),
                                    )
                                    .is_ok(),
                                    "b9fe50dc",
                                );
                                acc0.push(',');
                                acc0
                            },
                        );
                        let _: Option<char> = accumulator.pop();
                        accumulator
                    });
                    let select_only_ids_query_part_token_stream = {
                        let select_only_ids_query_part_initialization_token_stream = fields.iter().map(|element: &macros_helpers::field_data::SynField| generate_match_ok_err_query_part_token_stream(
                            &{
                                let field_double_quoted_token_stream = generate_quotes::dq_token_stream(&element.identifier);
                                let field_type_as_pg_crud_pg_type_pg_type_token_stream = generate_as_pg_type_path_token_stream(&element.type0);
                                quote::quote! {#field_type_as_pg_crud_pg_type_pg_type_token_stream #SelectOnlyIdsQueryPartSnakeCase(#import_token_stream SqlColumnRef::from(&#field_double_quoted_token_stream))}
                            },
                            &quote::quote! {v_aa341baf},
                            &quote::quote! {{
                                accumulator_a35168d8.push_str(&v_aa341baf);
                            }},
                            &Error0
                        ));
                        let ts0 = generate_accumulator_string_pop_accumulator_token_stream(
                            &quote::quote! {accumulator_a35168d8},
                            &quote::quote! {#(#select_only_ids_query_part_initialization_token_stream)*},
                        );
                        quote::quote! {{#ts0}}
                    };
                    let generate_if_write_is_err_short_token_stream = |ts: &dyn quote::ToTokens| {
                        macros_helpers::generate_if_write_is_err_token_stream::generate_if_write_is_err_token_stream(
                            &ts,
                            &write_into_buffer_query_part_syn_variant_error_initialization_eprintln_res_creation_token_stream,
                        )
                    };
                    let generate_select_only_updated_ids_query_part_token_stream =
                        |ts: &dyn quote::ToTokens| quote::quote! {#ts.#SelectOnlyUpdatedIdsQueryPartSnakeCase(&mut #IncrementSnakeCase)};
                    match &operation {
                        Operation::Cm => {
                            let if_write_is_err_token_stream = generate_if_write_is_err_short_token_stream(&quote::quote! {
                                accumulator_8a58994e,
                                "({v_f4fdd10d}),"
                            });
                            let ts0 = generate_accumulator_string_pop_accumulator_token_stream(&quote::quote! {accumulator_8a58994e}, &{
                                let ts = generate_match_ok_err_query_part_token_stream(
                                    &quote::quote! {element_1651705d.#CreateQueryPartSnakeCase(&mut #IncrementSnakeCase)},
                                    &quote::quote! {v_f4fdd10d},
                                    &quote::quote! {{
                                        #if_write_is_err_token_stream
                                    }},
                                    &Error0,
                                );
                                quote::quote! {
                                    for element_1651705d in #ParametersSnakeCase.#PayloadSnakeCase.as_slice() {
                                        #ts
                                    }
                                }
                            });
                            quote::quote! {pg_table::generate_cm_query_string(
                                pg_table::PgTableNameRef::from(#TableSnakeCase),
                                pg_table::PgTableSqlFragmentRef::from(#column_names_double_quoted_token_stream),
                                pg_table::PgTableSqlFragmentRef::from(&{
                                    #increment_initialization_token_stream
                                    #ts0
                                }),
                                pg_table::PgTableSqlFragmentRef::from(&#select_only_ids_query_part_token_stream)
                            )}
                        }
                        Operation::Co => {
                            let ts = generate_match_ok_err_update_token_stream(
                                &quote::quote! {#ParametersSnakeCase.#PayloadSnakeCase.#CreateQueryPartSnakeCase(&mut 0)},
                                &quote::quote! {v_3267d57d},
                            );
                            quote::quote! {
                                pg_table::generate_co_query_string(
                                    pg_table::PgTableNameRef::from(#TableSnakeCase),
                                    pg_table::PgTableSqlFragmentRef::from(#column_names_double_quoted_token_stream),
                                    pg_table::PgTableSqlFragmentRef::from(&#ts),
                                    pg_table::PgTableSqlFragmentRef::from(&#select_only_ids_query_part_token_stream)
                                )
                            }
                        }
                        Operation::Rm => {
                            let select_query_part_parameters_payload_select_token_stream =
                                generate_select_query_part_parameters_payload_select_token_stream(operation);
                            let extra_parameters_initialization_token_stream = generate_read_or_dm_extra_parameters_initialization_token_stream(
                                &RmOrDm::Rm,
                            );
                            let extra_parameters_order_by_handle_token_stream =
                                generate_quotes::dq_token_stream(&format!("{{}}{OrderSnakeCase} {BySnakeCase} {{}} {{}}"));
                            let primary_key_field_double_quoted_token_stream = generate_quotes::dq_token_stream(&primary_key_field_identifier);
                            let order_by_column_match_token_stream =
                                generate_read_fields_with_comma_token_stream(&|element: &macros_helpers::field_data::SynField| {
                                    let field_upper_camel_case = naming_common::ToTokensToUpperCamelCaseTokenStream::case_or_panic(&element.identifier);
                                    let field_double_quoted_token_stream = generate_quotes::dq_token_stream(&element.identifier);
                                    quote::quote! {
                                        #identifier_select_upper_camel_case::#field_upper_camel_case(_) => #field_double_quoted_token_stream
                                    }
                                });
                            let (if_write_is_err_curly_braces_0_token_stream, if_write_is_err_curly_braces_1_token_stream) = {
                                let generate_if_write_is_err_curly_braces_short_token_stream = |ts: &dyn quote::ToTokens| {
                                    macros_helpers::generate_if_write_is_err_token_stream::generate_if_write_is_err_token_stream(
                                    &ts,
                                    &write_into_buffer_query_part_syn_variant_error_initialization_eprintln_res_creation_token_stream
                                )
                                };
                                (
                                    generate_if_write_is_err_curly_braces_short_token_stream(&quote::quote! {
                                        #ExtraParametersSnakeCase,
                                        #extra_parameters_order_by_handle_token_stream,
                                        #PrefixSnakeCase,
                                        match &#ParametersSnakeCase.#PayloadSnakeCase.#OrderBySnakeCase.#ColumnSnakeCase {
                                            #order_by_column_match_token_stream
                                        },
                                        &order_691f662a
                                    }),
                                    generate_if_write_is_err_curly_braces_short_token_stream(&{
                                        let ts = generate_match_ok_err_update_token_stream(
                                            &quote::quote! {#pg_crud_pg_type_where_filter_query_part_token_stream(
                                                &#ParametersSnakeCase.#PayloadSnakeCase.pagination,
                                                &mut #IncrementSnakeCase,
                                                #import_token_stream SqlColumnRef::from(&""),
                                                #import_token_stream AddOperator::from(bool::default())
                                            )},
                                            &quote::quote! {v_742be6cf},
                                        );
                                        quote::quote! {
                                            #ExtraParametersSnakeCase,
                                            "{prefix}{}",
                                            #ts
                                        }
                                    }),
                                )
                            };
                            let if_write_is_err_order_tie_token_stream = macros_helpers::generate_if_write_is_err_token_stream::generate_if_write_is_err_token_stream(
                                &quote::quote! {
                                    #ExtraParametersSnakeCase,
                                    ", {} {}",
                                    #primary_key_field_double_quoted_token_stream,
                                    order_691f662a
                                },
                                &write_into_buffer_query_part_syn_variant_error_initialization_eprintln_res_creation_token_stream,
                            );
                            quote::quote! {pg_table::generate_rm_query_string(
                                pg_table::PgTableNameRef::from(#TableSnakeCase),
                                pg_table::PgTableSqlFragmentRef::from(&#select_query_part_parameters_payload_select_token_stream),
                                pg_table::PgTableSqlFragmentRef::from(&{
                                    #increment_initialization_token_stream
                                    let mut #ExtraParametersSnakeCase = #extra_parameters_initialization_token_stream;
                                    let #PrefixSnakeCase = if extra_parameters.is_empty() {""} else {" "};
                                    let order_691f662a = match #ParametersSnakeCase.#PayloadSnakeCase.#OrderBySnakeCase.#OrderSnakeCase.as_ref() {
                                        Some(#import_token_stream Order::Asc) | None => "asc",
                                        Some(#import_token_stream Order::Desc) => "desc",
                                    };
                                    #if_write_is_err_curly_braces_0_token_stream
                                    if !matches!(
                                        &#ParametersSnakeCase.#PayloadSnakeCase.#OrderBySnakeCase.#ColumnSnakeCase,
                                        #identifier_select_upper_camel_case::#primary_key_field_upper_camel_case_token_stream(_)
                                    ) {
                                        #if_write_is_err_order_tie_token_stream
                                    }
                                    #if_write_is_err_curly_braces_1_token_stream
                                    #ExtraParametersSnakeCase
                                })
                            )}
                        }
                        Operation::Ro => {
                            let select_query_part_parameters_payload_select_token_stream =
                                generate_select_query_part_parameters_payload_select_token_stream(operation);
                            let ts = generate_match_ok_err_update_token_stream(
                                &quote::quote! {#pg_crud_pg_type_where_filter_query_part_token_stream(
                                    &#ParametersSnakeCase.#PayloadSnakeCase.#primary_key_field_identifier,
                                    &mut 0,
                                    #import_token_stream SqlColumnRef::from(&Self::#PrimaryKeySnakeCase()),
                                    #import_token_stream AddOperator::from(false)
                                )},
                                &quote::quote! {v_be9e7b7d},
                            );
                            quote::quote! {pg_table::generate_ro_query_string(
                                pg_table::PgTableNameRef::from(#TableSnakeCase),
                                pg_table::PgTableSqlFragmentRef::from(&#select_query_part_parameters_payload_select_token_stream),
                                pg_table::PgTableSqlFragmentRef::from(&#ts)
                            )}
                        }
                        Operation::Um => {
                            let generate_match_update_query_part_primary_key_operation_token_stream =
                                |ts: &dyn quote::ToTokens| generate_match_update_query_part_primary_key_token_stream(operation, &ts);
                            let ts0 = generate_accumulator_string_pop_accumulator_token_stream(
                                &quote::quote! {accumulator_b86a253a},
                                &generate_fields_named_without_primary_key_without_comma_token_stream(&|element: &macros_helpers::field_data::SynField| {
                                    let field = &element.identifier;
                                    let field_double_quoted_token_stream = generate_quotes::dq_token_stream(&field);
                                    let is_field_update_exists_snake_case = naming::parameter::IsSelfUpdateExistSnakeCase::from_tokens(&field);
                                    let update_query_part_field_snake_case = naming::parameter::UpdateQueryPartSelfSnakeCase::from_tokens(&field);
                                    let for_element_update_field_exists_token_stream = generate_for_element_in_update_for_query_vec_token_stream(&quote::quote! {
                                        if element_a72f3eac.#field.is_some() {
                                            #is_field_update_exists_snake_case = true;
                                            break;
                                        }
                                    });
                                    let for_element_update_field_query_part_token_stream = generate_for_element_in_update_for_query_vec_field_token_stream(
                                        &field,
                                        &quote::quote! {v_3ea04126},
                                        &{
                                            let ts0 = generate_match_ok_err_update_token_stream(
                                                &quote::quote! {element_a72f3eac.#UpdateQueryPartPrimaryKeySnakeCase(&mut #IncrementSnakeCase)},
                                                &quote::quote! {v_00890100},
                                            );
                                            let ts1 = generate_match_ok_err_update_token_stream(
                                                &quote::quote! {#identifier_update_for_query_upper_camel_case::#update_query_part_field_snake_case(v_3ea04126, &mut #IncrementSnakeCase)},
                                                &quote::quote! {v_8797585c},
                                            );
                                            quote::quote! {
                                                accumulator_8ad06c8c.push_str(&pg_table::#GenerateWhenColumnIdThenVUmQueryPartSnakeCase(
                                                    pg_table::PgTableSqlFragmentRef::from(Self::#PrimaryKeySnakeCase()),
                                                    pg_table::PgTableSqlFragmentRef::from(&#ts0),
                                                    pg_table::PgTableSqlFragmentRef::from(&#ts1)
                                                ));
                                            }
                                        },
                                    );
                                    quote::quote! {
                                        {
                                            let mut #is_field_update_exists_snake_case = false;
                                            #for_element_update_field_exists_token_stream
                                            if #is_field_update_exists_snake_case {
                                                accumulator_b86a253a.push_str(&
                                                    pg_table::generate_column_eqs_case_accumulator_else_column_end_comma_um_query_part(
                                                        pg_table::PgTableSqlFragmentRef::from(#field_double_quoted_token_stream),
                                                        pg_table::PgTableSqlFragmentRef::from(&{
                                                            let mut accumulator_8ad06c8c = #StringTokenStream::default();
                                                            #for_element_update_field_query_part_token_stream
                                                            accumulator_8ad06c8c
                                                        })
                                                    )
                                                );
                                            }
                                        }
                                    }
                                }),
                            );
                            let ts1 = generate_accumulator_string_pop_accumulator_token_stream(
                                &quote::quote! {accumulator_a95eb175},
                                &generate_for_element_in_update_for_query_vec_token_stream(&generate_if_write_is_err_short_token_stream(
                                    &{
                                        let match_update_query_part_primary_key_operation_token_stream =
                                            generate_match_update_query_part_primary_key_operation_token_stream(
                                                &quote::quote! {element_a72f3eac},
                                            );
                                        quote::quote! {
                                            accumulator_a95eb175,
                                            "{},",
                                            #match_update_query_part_primary_key_operation_token_stream
                                        }
                                    },
                                )),
                            );
                            let for_element_select_only_updated_ids_query_part_token_stream =
                                generate_for_element_in_update_for_query_vec_token_stream(&generate_match_ok_err_query_part_token_stream(
                                    &generate_select_only_updated_ids_query_part_token_stream(&quote::quote! {element_a72f3eac}),
                                    &quote::quote! {v_4f536654},
                                    &quote::quote! {{
                                        accumulator_fd44b0aa.push_str(&v_4f536654);
                                    }},
                                    &Error0,
                                ));
                            quote::quote! {
                                {
                                    #increment_initialization_token_stream
                                    let els = {
                                        #ts0
                                    };
                                    let pks = {
                                        #ts1
                                    };
                                    let return_cols = {
                                        let mut accumulator_fd44b0aa = String::with_capacity(#UpdateForQueryVecSnakeCase.len().saturating_mul(32));
                                        #for_element_select_only_updated_ids_query_part_token_stream
                                        accumulator_fd44b0aa
                                    };
                                    pg_table::generate_um_query_string(
                                        pg_table::PgTableNameRef::from(#TableSnakeCase),
                                        pg_table::PgTableSqlFragmentRef::from(&els),
                                        pg_table::PgTableSqlFragmentRef::from(Self::#PrimaryKeySnakeCase()),
                                        pg_table::PgTableSqlFragmentRef::from(&pks),
                                        pg_table::PgTableSqlFragmentRef::from(&return_cols)
                                    )
                                }
                            }
                        }
                        Operation::Uo => {
                            let extra_parameters_modification_token_stream = generate_fields_named_without_primary_key_without_comma_token_stream(
                                &|element: &macros_helpers::field_data::SynField| {
                                    let field = &element.identifier;
                                    let field_double_quoted_token_stream = generate_quotes::dq_token_stream(&field);
                                    if optimistic_revision_field_identifier.as_ref() == Some(field) {
                                        return quote::quote! {
                                            accumulator_683e37b8.push_str(concat!(#field_double_quoted_token_stream, " = ", #field_double_quoted_token_stream, " + 1,"));
                                        };
                                    }
                                    let generate_column_queals_v_comma_uo_query_part_snake_case =
                                        naming::GenerateColumnQuealsVCommaUoQueryPartSnakeCase;
                                    let update_query_part_field_snake_case = naming::parameter::UpdateQueryPartSelfSnakeCase::from_tokens(&field);
                                    generate_if_let_some_token_stream(
                                        &quote::quote! {v_2d144436},
                                        &quote::quote! {&#UpdateForQuerySnakeCase.#field},
                                        &{
                                            let ts = generate_match_ok_err_update_token_stream(
                                                &quote::quote! {#identifier_update_for_query_upper_camel_case::#update_query_part_field_snake_case(v_2d144436, &mut #IncrementSnakeCase)},
                                                &quote::quote! {v_1ec12051},
                                            );
                                            quote::quote! {
                                                accumulator_683e37b8.push_str(&pg_table::#generate_column_queals_v_comma_uo_query_part_snake_case(
                                                    pg_table::PgTableSqlFragmentRef::from(#field_double_quoted_token_stream),
                                                    pg_table::PgTableSqlFragmentRef::from(&#ts)
                                                ));
                                            }
                                        },
                                    )
                                },
                            );
                            let extra_parameters_primary_key_modification_token_stream =
                                generate_match_update_query_part_primary_key_token_stream(operation, &quote::quote! {#UpdateForQuerySnakeCase});
                            let accumulator_string_pop_cols_token_stream = generate_accumulator_string_pop_accumulator_token_stream(
                                &quote::quote! {accumulator_683e37b8},
                                &extra_parameters_modification_token_stream,
                            );
                            let ts = generate_match_ok_err_update_token_stream(
                                &generate_select_only_updated_ids_query_part_token_stream(&UpdateForQuerySnakeCase),
                                &quote::quote! {v_7f0d86a1},
                            );
                            let optimistic_query_token_stream = optimistic_revision_field_identifier
                                .as_ref()
                                .map_or_else(
                                    || quote::quote! {query_297f2e40},
                                    |revision_identifier| {
                                        let revision_identifier_double_quoted_token_stream =
                                            generate_quotes::dq_token_stream(revision_identifier);
                                        quote::quote! {
                                            let optimistic_revision_query_part_f64c18e5 = format!("${}", #IncrementSnakeCase.saturating_add(1u64));
                                            pg_table::add_uo_optimistic_revision_predicate(
                                                query_297f2e40,
                                                pg_table::PgTableSqlFragmentRef::from(#revision_identifier_double_quoted_token_stream),
                                                pg_table::PgTableSqlFragmentRef::from(&optimistic_revision_query_part_f64c18e5),
                                            )
                                        }
                                    },
                                );
                            quote::quote! {
                                {
                                    #increment_initialization_token_stream
                                    let #ColsSnakeCase = {
                                        #accumulator_string_pop_cols_token_stream
                                    };
                                    let #PrimaryKeyQueryPartSnakeCase = #extra_parameters_primary_key_modification_token_stream;
                                    let return_cols = #ts;
                                    let query_297f2e40 = pg_table::generate_uo_query_string(
                                        pg_table::PgTableNameRef::from(#TableSnakeCase),
                                        pg_table::PgTableSqlFragmentRef::from(&#ColsSnakeCase),
                                        pg_table::PgTableSqlFragmentRef::from(Self::#PrimaryKeySnakeCase()),
                                        pg_table::PgTableSqlFragmentRef::from(&#PrimaryKeyQueryPartSnakeCase),
                                        pg_table::PgTableSqlFragmentRef::from(&return_cols)
                                    );
                                    #optimistic_query_token_stream
                                }
                            }
                        }
                        Operation::Dm => {
                            let extra_parameters_initialization_token_stream = generate_read_or_dm_extra_parameters_initialization_token_stream(
                                &RmOrDm::Dm,
                            );
                            quote::quote! {pg_table::generate_dm_query_string(
                                pg_table::PgTableNameRef::from(#TableSnakeCase),
                                pg_table::PgTableSqlFragmentRef::from(&{
                                    #increment_initialization_token_stream
                                    #extra_parameters_initialization_token_stream
                                }),
                                pg_table::PgTableSqlFragmentRef::from(Self::#PrimaryKeySnakeCase()),
                            )}
                        }
                        Operation::Dlo => quote::quote! {pg_table::generate_dlo_query_string(
                            pg_table::PgTableNameRef::from(#TableSnakeCase),
                            pg_table::PgTableSqlFragmentRef::from(Self::#PrimaryKeySnakeCase()),
                        )},
                    }
                };
                let binded_query_token_stream = {
                    let operation_error_initialization_try_bind_token_stream = generate_operation_error_initialization_eprintln_res_token_stream(
                        operation,
                        &try_bind_syn_variant,
                        std::panic::Location::caller(),
                    );
                    let generate_match_query_bind_or_err_short_token_stream =
                        |ts0: &dyn quote::ToTokens, ts1: &dyn quote::ToTokens| {
                            generate_match_query_bind_or_err_token_stream(&ts0, &ts1, &operation_error_initialization_try_bind_token_stream)
                        };
                    match &operation {
                        Operation::Cm => {
                            let ts = generate_match_query_bind_or_err_short_token_stream(
                                &quote::quote! {element_7f862135.#CreateQueryBindSnakeCase(#import_token_stream SqlxPostgresQuery::from(#QuerySnakeCase)).map(#import_token_stream SqlxPostgresQuery::into_inner).map_err(|error| error.to_string())},
                                &quote::quote! {v_011a3eb4},
                            );
                            quote::quote! {
                                for element_7f862135 in #ParametersSnakeCase.#PayloadSnakeCase.into_vec() {
                                    #ts
                                }
                            }
                        }
                        Operation::Co => generate_match_query_bind_or_err_short_token_stream(
                            &quote::quote! {#ParametersSnakeCase.#PayloadSnakeCase.#CreateQueryBindSnakeCase(#import_token_stream SqlxPostgresQuery::from(#QuerySnakeCase)).map(#import_token_stream SqlxPostgresQuery::into_inner).map_err(|error| error.to_string())},
                            &quote::quote! {v_06f852cd},
                        ),
                        Operation::Rm => {
                            let query_pg_type_where_filter_query_bind_parameters_payload_where_query_token_stream = generate_query_pg_type_where_filter_query_bind_parameters_payload_where_query_token_stream(operation);
                            let ts = generate_match_query_bind_or_err_short_token_stream(
                                &quote::quote! {#pg_crud_pg_type_where_filter_query_bind_token_stream(
                                    #ParametersSnakeCase.#PayloadSnakeCase.pagination,
                                    #import_token_stream SqlxPostgresQuery::from(#QuerySnakeCase),
                                ).map(#import_token_stream SqlxPostgresQuery::into_inner).map_err(|error| error.to_string())},
                                &quote::quote! {v_9f7e487b},
                            );
                            quote::quote! {
                                #query_pg_type_where_filter_query_bind_parameters_payload_where_query_token_stream
                                #ts
                            }
                        }
                        Operation::Ro => generate_match_query_bind_or_err_short_token_stream(
                            &quote::quote! {#pg_crud_pg_type_where_filter_query_bind_token_stream(
                                #ParametersSnakeCase.#PayloadSnakeCase.#primary_key_field_identifier,
                                #import_token_stream SqlxPostgresQuery::from(#QuerySnakeCase)
                            ).map(#import_token_stream SqlxPostgresQuery::into_inner).map_err(|error| error.to_string())},
                            &quote::quote! {v_80ee6983},
                        ),
                        Operation::Um => {
                            let fields_named_without_primary_key_update_assign_token_stream =
                                generate_fields_named_without_primary_key_without_comma_token_stream(&|element: &macros_helpers::field_data::SynField| {
                                    generate_for_element_in_update_for_query_vec_field_token_stream(
                                        &element.identifier,
                                        &quote::quote! {v_2edaa480},
                                        &{
                                            let ts = generate_match_query_bind_or_err_short_token_stream(
                                                &{
                                                    let as_pg_crud_pg_type_pg_type_token_stream =
                                                        generate_as_pg_type_path_token_stream(&element.type0);
                                                    quote::quote! {#as_pg_crud_pg_type_pg_type_token_stream #UpdateQueryBindSnakeCase(
                                                        v_2edaa480.#VSnakeCase.clone(),
                                                        #import_token_stream SqlxPostgresQuery::from(#QuerySnakeCase),
                                                    ).map(#import_token_stream SqlxPostgresQuery::into_inner).map_err(|error| error.to_string())}
                                                },
                                                &quote::quote! {v_600e67dc},
                                            );
                                            quote::quote! {
                                                if let Err(error_981062db) = #QuerySnakeCase.try_bind(element_a72f3eac.#primary_key_field_identifier) {
                                                    let #Error0 = error_981062db.to_string();
                                                    #operation_error_initialization_try_bind_token_stream
                                                }
                                                #ts
                                            }
                                        },
                                    )
                                });
                            let primary_key_update_assign_token_stream = generate_for_element_in_update_for_query_vec_token_stream(
                                &generate_match_query_bind_or_err_short_token_stream(
                                    &quote::quote! {#primary_key_field_type_as_pg_type_token_stream #UpdateQueryBindSnakeCase(
                                        element_a72f3eac.#primary_key_field_identifier,
                                        #import_token_stream SqlxPostgresQuery::from(#QuerySnakeCase),
                                    ).map(#import_token_stream SqlxPostgresQuery::into_inner).map_err(|error| error.to_string())},
                                    &quote::quote! {v_c40a4522},
                                ),
                            );
                            let binded_query_select_only_updated_ids_query_bind_token_stream =
                                generate_fields_named_without_primary_key_without_comma_token_stream(&|element: &macros_helpers::field_data::SynField| {
                                    generate_for_element_in_update_for_query_vec_field_token_stream(
                                        &element.identifier,
                                        &quote::quote! {v_47030ac2},
                                        &generate_match_query_bind_or_err_short_token_stream(
                                            &{
                                                let as_pg_crud_pg_type_pg_type_token_stream =
                                                    generate_as_pg_type_path_token_stream(&element.type0);
                                                quote::quote! {#as_pg_crud_pg_type_pg_type_token_stream select_only_updated_ids_query_bind(
                                                    &v_47030ac2.#VSnakeCase,
                                                    #import_token_stream SqlxPostgresQuery::from(#QuerySnakeCase)
                                                ).map(#import_token_stream SqlxPostgresQuery::into_inner).map_err(|error| error.to_string())}
                                            },
                                            &quote::quote! {v_c5b79b95},
                                        ),
                                    )
                                });
                            quote::quote! {
                                #fields_named_without_primary_key_update_assign_token_stream
                                #primary_key_update_assign_token_stream
                                #binded_query_select_only_updated_ids_query_bind_token_stream
                            }
                        }
                        Operation::Uo => {
                            let generate_binded_query_token_stream =
                                |var_name, method_name| {
                                    generate_fields_named_without_primary_key_without_comma_token_stream(&|element: &macros_helpers::field_data::SynField| {
                                        if optimistic_revision_field_identifier.as_ref() == Some(&element.identifier) {
                                            return proc_macro2::TokenStream::new();
                                        }
                                        generate_if_let_some_token_stream(
                                            &var_name,
                                            &{
                                                let field = &element.identifier;
                                                quote::quote! {&#UpdateForQuerySnakeCase.#field}
                                            },
                                            &generate_match_query_bind_or_err_short_token_stream(
                                                &{
                                                    let as_pg_crud_pg_type_pg_type_token_stream =
                                                        generate_as_pg_type_path_token_stream(&element.type0);
                                                    quote::quote! {#as_pg_crud_pg_type_pg_type_token_stream #method_name}
                                                },
                                                &quote::quote! {v_result},
                                            ),
                                        )
                                    })
                                };
                            let binded_query_modifications_token_stream = generate_binded_query_token_stream(
                                quote::quote! {v_ed87c152},
                                quote::quote! {#UpdateQueryBindSnakeCase(v_ed87c152.#VSnakeCase.clone(), #import_token_stream SqlxPostgresQuery::from(#QuerySnakeCase)).map(#import_token_stream SqlxPostgresQuery::into_inner).map_err(|error| error.to_string())},
                            );
                            let binded_query_primary_key_modification_token_stream = generate_match_query_bind_or_err_short_token_stream(
                                &quote::quote! {#primary_key_field_type_as_pg_type_token_stream #UpdateQueryBindSnakeCase(
                                    #UpdateForQuerySnakeCase.#primary_key_field_identifier,
                                    #import_token_stream SqlxPostgresQuery::from(#QuerySnakeCase),
                                ).map(#import_token_stream SqlxPostgresQuery::into_inner).map_err(|error| error.to_string())},
                                &quote::quote! {v_d64bac39},
                            );
                            let binded_query_select_only_updated_ids_query_bind_token_stream = generate_binded_query_token_stream(
                                quote::quote! {v_b2902425},
                                quote::quote! {select_only_updated_ids_query_bind(&v_b2902425.#VSnakeCase, #import_token_stream SqlxPostgresQuery::from(#QuerySnakeCase)).map(#import_token_stream SqlxPostgresQuery::into_inner).map_err(|error| error.to_string())},
                            );
                            let binded_optimistic_revision_token_stream = if optimistic_concurrency_enabled {
                                quote::quote! {
                                    if let Err(error_8ce463be) = #QuerySnakeCase.try_bind(optimistic_revision_9f023d8e) {
                                        let #Error0 = error_8ce463be.to_string();
                                        #operation_error_initialization_try_bind_token_stream
                                    }
                                }
                            } else {
                                proc_macro2::TokenStream::new()
                            };
                            quote::quote! {
                                #binded_query_modifications_token_stream
                                #binded_query_primary_key_modification_token_stream
                                #binded_query_select_only_updated_ids_query_bind_token_stream
                                #binded_optimistic_revision_token_stream
                            }
                        }
                        Operation::Dm => {
                            generate_query_pg_type_where_filter_query_bind_parameters_payload_where_query_token_stream(
                                operation,
                            )
                        }
                        Operation::Dlo => generate_match_query_bind_or_err_short_token_stream(
                            &quote::quote! {#import_token_stream PgTypeWhereFilter::query_bind(
                                #ParametersSnakeCase.#PayloadSnakeCase.#primary_key_field_identifier,
                                #import_token_stream SqlxPostgresQuery::from(#QuerySnakeCase)
                            ).map(#import_token_stream SqlxPostgresQuery::into_inner).map_err(|error| error.to_string())},
                            &quote::quote! {v_3099ea0f},
                        ),
                    }
                };
                let acquire_pool_and_connection_token_stream = {
                    let pg_syn_variant_error_initialization_eprintln_res_creation_token_stream =
                        generate_operation_error_initialization_eprintln_res_token_stream(
                            operation,
                            &pg_syn_variant,
                            std::panic::Location::caller(),
                        );
                    let ts = generate_match_ok_err_short_token_stream(
                        &quote::quote! {#AppStateSnakeCase.get_sqlx_pg_pool().as_ref().acquire().await},
                        &quote::quote! {v_4535ee48},
                        &quote::quote! {{
                            #pg_syn_variant_error_initialization_eprintln_res_creation_token_stream
                        }},
                    );
                    let ts0 = generate_match_ok_err_short_token_stream(
                        &quote::quote! {sqlx::Acquire::acquire(&mut #PoolConnectionSnakeCase).await},
                        &quote::quote! {v_61ae8f84},
                        &quote::quote! {{
                            #pg_syn_variant_error_initialization_eprintln_res_creation_token_stream
                        }},
                    );
                    quote::quote! {
                        let mut #PoolConnectionSnakeCase = #ts;
                        let #ExecutorAcquireSnakeCase = #ts0;
                    }
                };
                let pg_logic_token_stream = {
                    let generate_match_identifier_read_ids_as_from_row_from_row_token_stream = |ts: &dyn quote::ToTokens| {
                        generate_match_ok_err_short_token_stream(
                            &quote::quote! {<#identifier_read_ids_upper_camel_case as sqlx::FromRow<'_, sqlx::postgres::PgRow>>::from_row(&v_b27d7d79)},
                            &quote::quote! {v_33759463},
                            &ts,
                        )
                    };
                    let generate_create_update_dm_fetch_token_stream =
                        |create_or_update_or_dm: &CreateOrUpdateOrDm| {
                            let operation_create_update_dm = Operation::from(create_or_update_or_dm);
                            generate_fetch_token_stream(
                                &ExecutorSnakeCase,
                                &match &create_or_update_or_dm {
                                    CreateOrUpdateOrDm::Create
                                    | CreateOrUpdateOrDm::Update => {
                                        let ts = generate_match_identifier_read_ids_as_from_row_from_row_token_stream(&generate_drop_rows_match_pg_transaction_rollback_await_handle_token_stream(
                                            &operation_create_update_dm,
                                            std::panic::Location::caller(),
                                        ));
                                        quote::quote! {Some(#ts)}
                                    }
                                    CreateOrUpdateOrDm::Delete => generate_sqlx_row_try_get_primary_key_token_stream(
                                        &primary_key_field_type_as_pg_type_read_upper_camel_case,
                                        &quote::quote! {Some(v_69ecb6a9)},
                                        &generate_drop_rows_match_pg_transaction_rollback_await_handle_token_stream(
                                            &operation_create_update_dm,
                                            std::panic::Location::caller(),
                                        ),
                                    ),
                                },
                                &generate_drop_rows_match_pg_transaction_rollback_await_handle_token_stream(
                                    &operation_create_update_dm,
                                    std::panic::Location::caller(),
                                ),
                                &ShouldWrapIntoV::True,
                            )
                        };
                    let generate_create_update_dlo_fetch_token_stream =
                        |create_or_update_or_dlo: &CreateOrUpdateOrDlo| {
                            wrap_into_v_token_stream(&{
                                let op0 = Operation::from(create_or_update_or_dlo);
                                let ts = generate_match_pg_transaction_rollback_await_token_stream(
                                    &op0,
                                    std::panic::Location::caller(),
                                );
                                generate_fetch_one_token_stream(
                                    &ExecutorSnakeCase,
                                    &match create_or_update_or_dlo {
                                        CreateOrUpdateOrDlo::Create | CreateOrUpdateOrDlo::Update => generate_match_identifier_read_ids_as_from_row_from_row_token_stream(&ts),
                                        CreateOrUpdateOrDlo::Delete => generate_sqlx_row_try_get_primary_key_token_stream(
                                            &quote::quote! {#primary_key_field_type_as_pg_type_read_upper_camel_case},
                                            &quote::quote! {v_69ecb6a9},
                                            &ts,
                                        ),
                                    },
                                    &ts,
                                )
                            })
                        };
                    match &operation {
                        Operation::Cm => wrap_into_pg_transaction_begin_commit_token_stream(
                            operation,
                            &generate_create_update_dm_fetch_token_stream(&CreateOrUpdateOrDm::Create),
                        ),
                        Operation::Co => wrap_into_pg_transaction_begin_commit_token_stream(
                            operation,
                            &generate_create_update_dlo_fetch_token_stream(&CreateOrUpdateOrDlo::Create),
                        ),
                        Operation::Rm => {
                            let fetch_token_stream = generate_fetch_token_stream(
                                &ExecutorAcquireSnakeCase,
                                &{
                                    let match_identifier_read_try_from_sqlx_pg_pg_row_with_not_empty_unique_vec_identifier_select_token_stream = generate_match_identifier_read_try_from_sqlx_pg_pg_row_with_not_empty_unique_vec_identifier_select_token_stream(&RmOrRo::Rm);
                                    quote::quote! {Some(#match_identifier_read_try_from_sqlx_pg_pg_row_with_not_empty_unique_vec_identifier_select_token_stream)}
                                },
                                &generate_operation_error_initialization_eprintln_res_token_stream(
                                    operation,
                                    &pg_syn_variant,
                                    std::panic::Location::caller(),
                                ),
                                &ShouldWrapIntoV::False,
                            );
                            quote::quote! {{
                                #fetch_token_stream
                            }}
                        },
                        Operation::Ro => generate_fetch_one_token_stream(
                            &ExecutorAcquireSnakeCase,
                            &generate_match_identifier_read_try_from_sqlx_pg_pg_row_with_not_empty_unique_vec_identifier_select_token_stream(&RmOrRo::Ro),
                            &generate_operation_error_initialization_eprintln_res_token_stream(operation, &pg_syn_variant, std::panic::Location::caller()),
                        ),
                        Operation::Um => wrap_into_pg_transaction_begin_commit_token_stream(
                            operation,
                            &generate_create_update_dm_fetch_token_stream(&CreateOrUpdateOrDm::Update),
                        ),
                        Operation::Uo => {
                            if optimistic_concurrency_enabled {
                                let rollback_error_token_stream = generate_match_pg_transaction_rollback_await_token_stream(
                                    operation,
                                    std::panic::Location::caller(),
                                );
                                let row_token_stream = generate_match_identifier_read_ids_as_from_row_from_row_token_stream(
                                    &rollback_error_token_stream,
                                );
                                let rollback_failed_token_stream = generate_operation_error_initialization_eprintln_res_token_stream(
                                    operation,
                                    &pg_syn_variant,
                                    std::panic::Location::caller(),
                                );
                                let release_token_stream = if idempotency_enabled {
                                    quote::quote! {
                                        let _release_result = pg_table::release_pg_table_idempotency(
                                            app_state::SqlxPgPoolRef::from(idempotency_pool_193acb3c.as_ref()),
                                            &idempotency_request_0a0ae019,
                                        ).await;
                                    }
                                } else {
                                    proc_macro2::TokenStream::new()
                                };
                                wrap_into_pg_transaction_begin_commit_token_stream(
                                    operation,
                                    &quote::quote! {
                                        let #VSnakeCase = match #BindedQuerySnakeCase.fetch_optional(#ExecutorSnakeCase.as_mut()).await {
                                            Ok(Some(v_b27d7d79)) => #row_token_stream,
                                            Ok(None) => {
                                                if let Err(#Error1) = #ExecutorSnakeCase.#RollbackSnakeCase().await {
                                                    let #Error0 = #Error1;
                                                    #rollback_failed_token_stream
                                                }
                                                #release_token_stream
                                                return axum::response::IntoResponse::into_response(http::StatusCode::PRECONDITION_FAILED);
                                            }
                                            Err(#Error0) => {
                                                #rollback_error_token_stream
                                            }
                                        };
                                    },
                                )
                            } else {
                                wrap_into_pg_transaction_begin_commit_token_stream(
                                    operation,
                                    &generate_create_update_dlo_fetch_token_stream(&CreateOrUpdateOrDlo::Update),
                                )
                            }
                        }
                        Operation::Dm => wrap_into_pg_transaction_begin_commit_token_stream(
                            operation,
                            &generate_create_update_dm_fetch_token_stream(&CreateOrUpdateOrDm::Delete),
                        ),
                        Operation::Dlo => wrap_into_pg_transaction_begin_commit_token_stream(
                            operation,
                            &generate_create_update_dlo_fetch_token_stream(&CreateOrUpdateOrDlo::Delete),
                        ),
                    }
                };
                let wraped_into_axum_res_token_stream = wrap_into_axum_res_token_stream(
                    &{
                        let identifier_operation_res_variants_upper_camel_case = generate_identifier_operation_res_variants_upper_camel_case(operation);
                        quote::quote! {#identifier_operation_res_variants_upper_camel_case::#DesirableUpperCamelCase(#VSnakeCase)}
                    },
                    &crate::handler::success_status(operation_dsc).to_http_status_code_token_stream(),
                    &AddReturn::False,
                );
                let success_response_token_stream = if idempotency_enabled {
                    let desirable_status_token_stream =
                        crate::handler::success_status(operation_dsc).to_http_status_code_token_stream();
                    quote::quote! {
                        let (response_value_1a2393ae, response_body_649297c9) = #VSnakeCase;
                        let mut response = axum::response::Response::new(axum::body::Body::from(response_body_649297c9));
                        *response.status_mut() = #desirable_status_token_stream;
                        let _previous_content_type = response.headers_mut().insert(http::header::CONTENT_TYPE, http::HeaderValue::from_static("application/json"));
                        response
                    }
                } else {
                    wraped_into_axum_res_token_stream
                };
                quote::quote! {
                    async fn #operation_handle_snake_case_token_stream(
                        #AppStateSnakeCase: axum::extract::State<#std_sync_arc_combination_of_app_state_logic_traits_token_stream>,
                        #ReqSnakeCase: axum::extract::Request,
                        #TableSnakeCase: &str,
                    ) -> axum::response::Response {
                        #req_parts_preparation_token_stream
                        #extra_validators_token_stream
                        #parameters_logic_token_stream
                        #idempotency_begin_token_stream
                        let #QueryStringSnakeCase = #query_string_token_stream;
                        let #BindedQuerySnakeCase = {
                            let mut #QuerySnakeCase = #sqlx_query_sqlx_pg_token_stream(
                                sqlx::AssertSqlSafe(#QueryStringSnakeCase.to_string())
                            );
                            #binded_query_token_stream
                            #QuerySnakeCase
                        };
                        #acquire_pool_and_connection_token_stream
                        let #VSnakeCase = {
                            #pg_logic_token_stream
                        };
                        #success_response_token_stream
                    }
                }
            };
            let operation_token_stream = {
                let operation_route_snake_case_token_stream =
                    quote::format_ident!("{}_route", operation.self_snake_case_str());
                let operation_route_path = format!(
                    "/{}/{}",
                    identifier_snake_case_string,
                    operation.self_snake_case_str()
                );
                quote::quote! {
                    #MustUse
                    pub fn #operation_route_snake_case_token_stream() -> frontend_contract::ContractStr {
                        frontend_contract::ContractStr::from(#operation_route_path)
                    }
                    pub async fn #operation_snake_case_token_stream(
                        #AppStateSnakeCase: axum::extract::State<#std_sync_arc_combination_of_app_state_logic_traits_token_stream>,
                        #ReqSnakeCase: axum::extract::Request,
                    ) -> axum::response::Response {
                        Self::#operation_handle_snake_case_token_stream(#AppStateSnakeCase, #ReqSnakeCase, #self_table_name_call_token_stream).await
                    }
                }
            };
            let operation_payload_example_token_stream = {
                let operation_payload_example_snake_case = operation.operation_payload_example_snake_case();
                let operation_payload_example_route_snake_case = quote::format_ident!(
                    "{}_payload_example_route",
                    operation.self_snake_case_str()
                );
                let operation_payload_example_route_path = format!(
                    "/{identifier_snake_case_string}/{operation_payload_example_snake_case}"
                );
                let ts = wrap_into_axum_res_token_stream(
                    &{
                        let identifier_operation_payload_upper_camel_case = generate_identifier_operation_payload_upper_camel_case(operation);
                        quote::quote! {<#identifier_operation_payload_upper_camel_case as #import_token_stream #DefaultSomeOneElementUpperCamelCase>::#DefaultSomeOneElementSnakeCase()}
                    },
                    &quote::quote! {http::StatusCode::OK},
                    &AddReturn::False,
                );
                quote::quote! {
                    #MustUse
                    pub fn #operation_payload_example_route_snake_case() -> frontend_contract::ContractStr {
                        frontend_contract::ContractStr::from(#operation_payload_example_route_path)
                    }
                    #MustUse
                    pub fn #operation_payload_example_snake_case() -> axum::response::Response {
                        #ts
                    }
                }
            };
            quote::quote! {
                #operation_handle_token_stream
                #operation_token_stream
                #try_operation_token_stream
                #operation_payload_example_token_stream
            }
        });
        content_token_stream.push({
            let payload_token_stream = {
                let generate_parameters_payload_and_default_token_stream =
                    |declaration_token_stream: &dyn quote::ToTokens, default_initialization_token_stream: &dyn quote::ToTokens| {
                        let identifier_operation_payload_upper_camel_case = generate_identifier_operation_payload_upper_camel_case(operation);
                        let identifier_operation_payload_token_stream = {
                            let (derive_clone, derive_copy) = operation.derive_clone_and_copy();
                            let payload_builder_without_deserialize = macros_helpers::derive_token_stream_builder::DTokenStreamBuilder::new()
                                .make_pub()
                                .d_debug()
                                .d_clone_if(derive_clone)
                                .d_copy_if(derive_copy)
                                .d_serde_serialize();
                            let payload_builder = if matches!(operation, Operation::Cm)
                                && generate_pg_table_input_model.config.cm_max_items.is_some()
                            {
                                payload_builder_without_deserialize
                            } else {
                                payload_builder_without_deserialize.d_serde_deserialize()
                            };
                            let identifier_operation_payload_struct_token_stream = payload_builder
                                .d_utoipa_to_schema()
                                .build_struct(&quote::quote! {#[serde(deny_unknown_fields)]},&identifier_operation_payload_upper_camel_case, &proc_macro2::TokenStream::new(), &declaration_token_stream);
                            quote::quote! {
                                #AllowClippyArbitrarySrcItemOrdering
                                #identifier_operation_payload_struct_token_stream
                            }
                        };
                        let impl_pg_crud_default_some_one_element_for_operation_payload_token_stream =
                            generate_impl_pg_crud_default_some_one_element_for_tokens_no_lt_token_stream(
                                &identifier_operation_payload_upper_camel_case,
                                &quote::quote! {Self #default_initialization_token_stream},
                            );
                        quote::quote! {
                            #identifier_operation_payload_token_stream
                            #impl_pg_crud_default_some_one_element_for_operation_payload_token_stream
                        }
                    };
                match &operation {
                    Operation::Cm => {
                        let identifier_operation_payload_upper_camel_case = generate_identifier_operation_payload_upper_camel_case(operation);
                        let vec_identifier_create_token_stream =
                            pg_crud_macros_common::generate_vec_tokens_declaration_token_stream(&identifier_create_upper_camel_case);
                        let vec_identifier_create_schema_token_stream = generate_pg_table_input_model.config.cm_max_items.map_or_else(
                            || quote::quote! {#vec_identifier_create_token_stream},
                            |limit| {
                                let limit_value = limit.0;
                                quote::quote! {#[schema(max_items = #limit_value)] #vec_identifier_create_token_stream}
                            },
                        );
                        let payload_token_stream = generate_parameters_payload_and_default_token_stream(
                            &quote::quote! {(#vec_identifier_create_schema_token_stream);},
                            &quote::quote! {(vec![#PgCrudCommonDefaultSomeOneElementCall])},
                        );
                        let limited_deserialize_token_stream = generate_pg_table_input_model.config.cm_max_items.map_or_else(
                            proc_macro2::TokenStream::new,
                            |limit| {
                                let limit_value = limit.0;
                                quote::quote! {
                                impl<'de> serde::Deserialize<'de> for #identifier_operation_payload_upper_camel_case {
                                    fn deserialize<Deserializer>(deserializer: Deserializer) -> Result<Self, Deserializer::Error>
                                    where
                                        Deserializer: serde::Deserializer<'de>,
                                    {
                                        let bounded = <pg_crud_common::bounded_vec::BoundedVec<#identifier_create_upper_camel_case, 0usize, #limit_value> as serde::Deserialize>::deserialize(deserializer)?;
                                        Ok(Self(Vec::from(bounded)))
                                    }
                                }
                            }
                            },
                        );
                        quote::quote! {
                            #payload_token_stream
                            #limited_deserialize_token_stream
                            impl #identifier_operation_payload_upper_camel_case {
                                #[must_use]
                                pub const fn as_slice(&self) -> &[#identifier_create_upper_camel_case] {
                                    self.0.as_slice()
                                }
                                #[must_use]
                                pub fn into_vec(self) -> #vec_identifier_create_token_stream {
                                    self.0
                                }
                            }
                        }
                    }
                    Operation::Rm => generate_parameters_payload_and_default_token_stream(
                        &quote::quote! {{
                            #pub_where_optional_identifier_where_token_stream,
                            #[schema(inline)]
                            #pub_select_pg_crud_not_empty_unique_vec_identifier_select_token_stream,
                            #[schema(inline)]
                            pub #OrderBySnakeCase: #pg_crud_order_by_token_stream<#identifier_select_upper_camel_case>,
                            pub #PaginationSnakeCase: #import_token_stream PaginationStartsWithZero,
                        }},
                        &{
                            let ts = generate_field_default_some_one_element_call_token_stream(&PaginationSnakeCase);
                            quote::quote! {{
                                #where_many_pg_crud_default_some_one_element_call_token_stream,
                                #select_pg_crud_default_some_one_element_call_token_stream,
                                #OrderBySnakeCase: #import_token_stream OrderBy {
                                    #ColumnSnakeCase: #identifier_select_upper_camel_case::#primary_key_field_upper_camel_case_token_stream(
                                        #PgCrudCommonDefaultSomeOneElementCall
                                    ),
                                    #OrderSnakeCase: Some(
                                        #PgCrudCommonDefaultSomeOneElementCall
                                    ),
                                },
                                #ts,
                            }}
                        },
                    ),
                    Operation::Ro => generate_parameters_payload_and_default_token_stream(
                        &{
                            let pub_handle_primary_key_field_primary_key_inner_type_handle_token_stream =
                                generate_pub_handle_primary_key_field_primary_key_inner_type_handle_token_stream(
                                    &naming::parameter::SelfReadUpperCamelCase::from_type_last_segment(primary_key_field_type),
                                );
                            quote::quote! {{
                                #pub_handle_primary_key_field_primary_key_inner_type_handle_token_stream,
                                #[schema(no_recursion)]
                                #pub_select_pg_crud_not_empty_unique_vec_identifier_select_token_stream,
                            }}
                        },
                        &{
                            let ts = generate_field_default_some_one_element_call_token_stream(&primary_key_field_identifier);
                            quote::quote! {{
                                #ts,
                                #select_pg_crud_default_some_one_element_call_token_stream
                            }}
                        },
                    ),
                    Operation::Um => {
                        let identifier_operation_payload_upper_camel_case = generate_identifier_operation_payload_upper_camel_case(operation);
                        let vec_identifier_update_token_stream = pg_crud_macros_common::generate_vec_tokens_declaration_token_stream(&identifier_update_upper_camel_case);
                        let vec_identifier_update_schema_token_stream = generate_pg_table_input_model.config.um_max_items.map_or_else(
                            || quote::quote! {#vec_identifier_update_token_stream},
                            |limit| {
                                let limit_value = limit.0;
                                quote::quote! {#[schema(max_items = #limit_value)] #vec_identifier_update_token_stream}
                            },
                        );
                        let identifier_operation_payload_vec_token_stream = serde_ser_utoipa_d_token_stream_builder
                            .build_struct(
                                &proc_macro2::TokenStream::new(),
                                &identifier_operation_payload_upper_camel_case,
                                &proc_macro2::TokenStream::new(),
                                &quote::quote! {(#vec_identifier_update_schema_token_stream);},
                            );
                        let identifier_operation_payload_try_new_error_upper_camel_case =
                            generate_identifier_operation_suffix_token_stream(operation, str_constants::PAYLOADTRYNEWERROR);
                        let identifier_operation_payload_try_new_error_token_stream = pg_crud_macros_common::token_stream_helpers::error_enum_d_token_stream_builder()
                        .build_enum(
                                &proc_macro2::TokenStream::new(),
                                &identifier_operation_payload_try_new_error_upper_camel_case,
                                &proc_macro2::TokenStream::new(),
                                &quote::quote! {{
                                    #NotUniquePrimaryKeyUpperCamelCase {
                                        #[eo_to_err_string]
                                        #NotUniquePrimaryKeySnakeCase: #primary_key_field_type_update_token_stream,
                                        #[eo_to_err_string]
                                        location: location_lib::location::Location,
                                    }
                                }},
                            );
                        let impl_pub_try_new_for_identifier_operation_payload_token_stream = quote::quote! {
                            impl #identifier_operation_payload_upper_camel_case {
                                #[must_use]
                                pub const fn as_slice(&self) -> &[#identifier_update_upper_camel_case] {
                                    self.0.as_slice()
                                }
                                #[must_use]
                                pub fn into_vec(self) -> #vec_identifier_update_token_stream {
                                    self.0
                                }
                                pub fn try_new(
                                    #VSnakeCase: #vec_identifier_update_token_stream,
                                ) -> Result<Self, #identifier_operation_payload_try_new_error_upper_camel_case> {
                                let mut accumulator_6bf275fc = std::collections::HashSet::with_capacity(#VSnakeCase.len());
                                for element_35facc3a in &#VSnakeCase {
                                    if !accumulator_6bf275fc.insert(&element_35facc3a.#primary_key_field_identifier) {
                                        return Err(#identifier_operation_payload_try_new_error_upper_camel_case::#NotUniquePrimaryKeyUpperCamelCase {
                                            #NotUniquePrimaryKeySnakeCase: element_35facc3a.#primary_key_field_identifier,
                                            location: location_macros::location!(),
                                        });
                                    }
                                }
                                Ok(Self(#VSnakeCase))
                                }
                            }
                        };
                        let um_deserialize_raw_token_stream = generate_pg_table_input_model.config.um_max_items.map_or_else(
                            || quote::quote! {<#vec_identifier_update_token_stream as _serde::Deserialize>::deserialize(__deserializer)?},
                            |limit| {
                                let limit_value = limit.0;
                                quote::quote! {
                                    Vec::from(<pg_crud_common::bounded_vec::BoundedVec<#identifier_update_upper_camel_case, 0usize, #limit_value> as _serde::Deserialize>::deserialize(__deserializer)?)
                                }
                            },
                        );
                        let impl_de_for_identifier_um_payload_token_stream = quote::quote! {
                            #[allow(unused_qualifications)]
                            #[allow(clippy::absolute_paths)]
                            #AllowClippyArbitrarySrcItemOrdering
                            const _: () = {
                                extern crate serde as _serde;
                                #[automatically_derived]
                                impl<'de> _serde::Deserialize<'de> for #identifier_operation_payload_upper_camel_case {
                                    fn deserialize<__D>(__deserializer: __D) -> Result<Self, __D::Error>
                                    where
                                        __D: _serde::Deserializer<'de>,
                                    {
                                        let raw = #um_deserialize_raw_token_stream;
                                        Self::try_new(raw).map_err(|error| _serde::de::Error::custom(format!("{error:?}")))
                                    }
                                }
                            };
                        };
                        let impl_pg_crud_default_some_one_element_for_operation_payload_token_stream =
                            generate_impl_pg_crud_default_some_one_element_for_tokens_no_lt_token_stream(
                                &identifier_operation_payload_upper_camel_case,
                                &quote::quote! {
                                    Self(vec![#PgCrudCommonDefaultSomeOneElementCall])
                                },
                            );
                        quote::quote! {
                            #identifier_operation_payload_vec_token_stream
                            #identifier_operation_payload_try_new_error_token_stream
                            #impl_pub_try_new_for_identifier_operation_payload_token_stream
                            #impl_de_for_identifier_um_payload_token_stream
                            #impl_pg_crud_default_some_one_element_for_operation_payload_token_stream
                        }
                    },
                    Operation::Dm => generate_parameters_payload_and_default_token_stream(
                        &quote::quote! {{#pub_where_optional_identifier_where_token_stream}},
                        &quote::quote! {{#where_many_pg_crud_default_some_one_element_call_token_stream}},
                    ),
                    Operation::Dlo => generate_parameters_payload_and_default_token_stream(
                        &{
                            let ts = generate_pub_handle_primary_key_field_primary_key_inner_type_handle_token_stream(
                                &naming::parameter::SelfReadUpperCamelCase::from_type_last_segment(primary_key_field_type),
                            );
                            quote::quote! {{#ts}}
                        },
                        &{
                            let ts = generate_field_default_some_one_element_call_token_stream(&primary_key_field_identifier);
                            quote::quote! {{#ts}}
                        },
                    ),
                    Operation::Co | Operation::Uo => proc_macro2::TokenStream::new(),
                }
            };
            let parameters_token_stream = {
                let (derive_clone, derive_copy) = operation.derive_parameters_clone_and_copy();
                let identifier_operation_parameters_struct_token_stream = macros_helpers::derive_token_stream_builder::DTokenStreamBuilder::new()
                    .make_pub()
                    .d_debug()
                    .d_clone_if(derive_clone)
                    .d_copy_if(derive_copy)
                    .build_struct(&proc_macro2::TokenStream::new(),&generate_identifier_operation_parameters_upper_camel_case(operation), &proc_macro2::TokenStream::new(), &{
                        let identifier_operation_payload_upper_camel_case = generate_identifier_operation_payload_upper_camel_case(operation);
                        quote::quote! {{
                            pub #PayloadSnakeCase: #identifier_operation_payload_upper_camel_case,
                        }}
                    });
                quote::quote! {
                    #AllowClippyArbitrarySrcItemOrdering
                    #identifier_operation_parameters_struct_token_stream
                }
            };
            let operation_token_stream = {
                let identifier_operation_res_variants_upper_camel_case = generate_identifier_operation_res_variants_upper_camel_case(operation);
                let identifier_try_operation_logic_res_variants_token_stream = {
                    let identifier_operation_res_variants_enum_token_stream = macros_helpers::derive_token_stream_builder::DTokenStreamBuilder::new()
                        .make_pub()
                        .d_debug()
                        .d_serde_serialize()
                        .d_serde_deserialize()
                        .build_enum(&proc_macro2::TokenStream::new(), &identifier_operation_res_variants_upper_camel_case, &proc_macro2::TokenStream::new(), &{
                            let vrts_token_stream = type_variants_from_req_res_syn_variants
                                .iter()
                                .copied()
                                .map(generate_serde_version_of_named_generate_pg_table_variant_token_stream);
                            let desirable_type_token_stream = generate_operation_result_type_token_stream(operation);
                            quote::quote! {{
                                #DesirableUpperCamelCase(#desirable_type_token_stream),
                                #(#vrts_token_stream),*
                            }}
                        });
                    let desirable_type_token_stream =
                        generate_operation_result_type_token_stream(operation);
                    let error_variant_schema_items_token_stream =
                        type_variants_from_req_res_syn_variants.iter().map(|variant| {
                            let variant_name = variant.identifier().to_string();
                            let field_names = match variant {
                                GeneratePgTableVariantEmissionRef::Model(model_variant) => {
                                    model_variant
                                        .fields
                                        .iter()
                                        .map(|field| field.identifier.to_string())
                                        .collect::<Vec<_>>()
                                }
                                GeneratePgTableVariantEmissionRef::Syn(syn_variant) => {
                                    let syn::Fields::Named(variant_fields) = &syn_variant.fields else {
                                        return compile_error_token_stream(
                                            CompileErrorMessage::from(
                                                str_constants::MACRO_DIAGNOSTICS_EXPECTED_NAMED_VARIANT_FIELDS_ERROR,
                                            ),
                                        )
                                        .into();
                                    };
                                    variant_fields
                                        .named
                                        .iter()
                                        .filter_map(|field| field.ident.as_ref())
                                        .map(ToString::to_string)
                                        .collect::<Vec<_>>()
                                }
                            };
                            quote::quote! {
                                .item(
                                    utoipa::openapi::ObjectBuilder::new()
                                        .property(
                                            #variant_name,
                                            utoipa::openapi::ObjectBuilder::new()
                                                #(
                                                    .property(
                                                        #field_names,
                                                        utoipa::openapi::ObjectBuilder::new(),
                                                    )
                                                    .required(#field_names)
                                                )*
                                        )
                                        .required(#variant_name),
                                )
                            }
                        });
                    let response_schema_name =
                        identifier_operation_res_variants_upper_camel_case.to_string();
                    quote::quote! {
                        #AllowClippyArbitrarySrcItemOrdering
                        #identifier_operation_res_variants_enum_token_stream
                        impl utoipa::PartialSchema for #identifier_operation_res_variants_upper_camel_case {
                            fn schema() -> utoipa::openapi::RefOr<utoipa::openapi::schema::Schema> {
                                utoipa::openapi::schema::Schema::from(
                                    utoipa::openapi::OneOfBuilder::new()
                                    .item(
                                        utoipa::openapi::ObjectBuilder::new()
                                            .property(
                                                stringify!(#DesirableUpperCamelCase),
                                                <#desirable_type_token_stream as utoipa::PartialSchema>::schema(),
                                            )
                                            .required(stringify!(#DesirableUpperCamelCase)),
                                    )
                                    #(#error_variant_schema_items_token_stream)*
                                    .build()
                                )
                                    .into()
                            }
                        }
                        impl utoipa::ToSchema for #identifier_operation_res_variants_upper_camel_case {
                            fn name() -> std::borrow::Cow<'static, str> {
                                std::borrow::Cow::Borrowed(#response_schema_name)
                            }
                        }
                    }
                };
                let identifier_operation_error_upper_camel_case = generate_identifier_operation_error_upper_camel_case(operation);
                let impl_identifier_operation_res_variants_token_stream = {
                    let from_handle_token_stream = generate_from_handle_token_stream(&identifier_operation_error_upper_camel_case, &{
                        let vrts_token_stream = type_variants_from_req_res_syn_variants.iter().map(|element| {
                            let variant_identifier = element.identifier();
                            let fields_mapped_into_token_stream = match *element {
                                GeneratePgTableVariantEmissionRef::Syn(syn_variant) => {
                                    let syn::Fields::Named(fields_named) = &syn_variant.fields else {
                                        return compile_error_token_stream(CompileErrorMessage::from(
                                            str_constants::COMPILE_ERROR_CE_000,
                                        )).into();
                                    };
                                    let fields_token_stream = fields_named.named.iter().map(|field| &field.ident);
                                    quote::quote! {#(#fields_token_stream),*}
                                }
                                GeneratePgTableVariantEmissionRef::Model(model_variant) => {
                                let fields_token_stream = model_variant.fields.iter().map(|field| &field.identifier);
                                    quote::quote! {#(#fields_token_stream),*}
                                }
                            };
                            let identifier_operation_error_with_serde_upper_camel_case =
                                generate_identifier_operation_error_with_serde_upper_camel_case(operation);
                            quote::quote! {
                                #identifier_operation_error_with_serde_upper_camel_case::#variant_identifier {
                                    #fields_mapped_into_token_stream
                                } => Self::#variant_identifier {
                                    #fields_mapped_into_token_stream
                                }
                            }
                        });
                        quote::quote! {
                            match #VSnakeCase.#IntoSerdeVersionSnakeCase() {
                                #(#vrts_token_stream),*
                            }
                        }
                    });
                    quote::quote! {
                        impl #identifier_operation_res_variants_upper_camel_case {
                            #from_handle_token_stream
                        }
                    }
                };
                let identifier_operation_error_token_stream = {
                    let identifier_operation_error_enum_token_stream = pg_crud_macros_common::token_stream_helpers::error_enum_d_token_stream_builder()
                        .build_enum(&proc_macro2::TokenStream::new(), &identifier_operation_error_upper_camel_case, &proc_macro2::TokenStream::new(), &{
                            let vrts_token_stream = type_variants_from_req_res_syn_variants
                                .iter()
                                .copied()
                                .map(generate_location_variant_token_stream);
                            quote::quote! {{#(#vrts_token_stream),*}}
                        });
                    quote::quote! {
                        #AllowClippyArbitrarySrcItemOrdering
                        #identifier_operation_error_enum_token_stream
                    }
                };
                quote::quote! {
                    #identifier_try_operation_logic_res_variants_token_stream
                    #impl_identifier_operation_res_variants_token_stream
                    #identifier_operation_error_token_stream
                }
            };
            let try_operation_token_stream = {
                let enum_token_stream = pg_crud_macros_common::token_stream_helpers::error_enum_d_token_stream_builder()
                        .build_enum(&proc_macro2::TokenStream::new(), &generate_identifier_try_operation_error_upper_camel_case(operation), &proc_macro2::TokenStream::new(), &{
                        let mut syn_variants = Vec::with_capacity(common_http_req_syn_variants.len().saturating_add(1usize));
                        syn_variants.extend_from_slice(common_http_req_syn_variants.as_slice());
                        if let Operation::Rm | Operation::Ro = &operation {
                            syn_variants.push(GeneratePgTableVariantEmissionRef::Syn(not_unique_field_syn_variant.get_syn_variant()));
                        }
                        let identifier_operation_error_with_serde_upper_camel_case =
                            generate_identifier_operation_error_with_serde_upper_camel_case(operation);
                        let operation_error_with_serde_syn_variant = new_syn_variant(
                            &identifier_operation_error_with_serde_upper_camel_case,
                            None,
                            vec![(
                                macros_helpers_location_field_attr_eo_to_err_string,
                                &operation.operation_error_with_serde_snake_case(),
                                macros_helpers::generate_simple_syn_punct::generate_simple_syn_punct([
                                    &identifier_operation_error_with_serde_upper_camel_case.to_string()
                                ]),
                            )],
                            false,
                        );
                        let vrts_token_stream = syn_variants
                            .iter()
                            .copied()
                            .chain(std::iter::once(GeneratePgTableVariantEmissionRef::Syn(operation_error_with_serde_syn_variant.get_syn_variant())))
                            .map(generate_location_variant_token_stream);
                        quote::quote! {{#(#vrts_token_stream),*}}
                    });
                quote::quote! {
                    #AllowClippyArbitrarySrcItemOrdering
                    #enum_token_stream
                }
            };
            quote::quote! {
                #payload_token_stream
                #parameters_token_stream
                #operation_token_stream
                #try_operation_token_stream
            }
        });
    });
    let identifier_api_endpoint_upper_camel_case =
        quote::format_ident!("{}ApiEndpoint", identifier);
    let identifier_api_client_upper_camel_case = quote::format_ident!("{}ApiClient", identifier);
    let identifier_frontend_api_client_upper_camel_case =
        quote::format_ident!("{}FrontendApiClient", identifier);
    let identifier_api_client_token_stream = quote::quote! {
        #[derive(Clone, Debug)]
        pub struct #identifier_api_endpoint_upper_camel_case(reqwest::Url);
        impl #identifier_api_endpoint_upper_camel_case {
            #[must_use]
            pub const fn as_url(&self) -> &reqwest::Url {
                &self.0
            }
        }
        impl From<reqwest::Url> for #identifier_api_endpoint_upper_camel_case {
            fn from(value: reqwest::Url) -> Self {
                Self(value)
            }
        }
        #[derive(Clone, Debug)]
        pub struct #identifier_api_client_upper_camel_case {
            client: reqwest::Client,
            endpoint: #identifier_api_endpoint_upper_camel_case,
        }
        impl #identifier_api_client_upper_camel_case {
            #[must_use]
            pub const fn new(client: reqwest::Client, endpoint: #identifier_api_endpoint_upper_camel_case) -> Self {
                Self { client, endpoint }
            }
            #(#api_client_methods_token_stream)*
        }
        #[derive(Clone, Debug)]
        pub struct #identifier_frontend_api_client_upper_camel_case<Transport> {
            transport: Transport,
        }
        #[allow(clippy::future_not_send)] // browser transports and WASM futures are intentionally single-threaded
        impl<Transport> #identifier_frontend_api_client_upper_camel_case<Transport>
        where
            Transport: frontend_contract::Transport,
        {
            #[must_use]
            pub const fn new(transport: Transport) -> Self {
                Self { transport }
            }
            #(#frontend_api_client_methods_token_stream)*
        }
    };
    let enabled_operation_count = crate::model::OperationDsc::ALL
        .iter()
        .filter(|operation_dsc| operation_is_enabled(&operation_dsc.operation))
        .count();
    let route_contract_items_token_stream = crate::model::OperationDsc::ALL
        .iter()
        .filter(|operation_dsc| operation_is_enabled(&operation_dsc.operation))
        .map(|operation_dsc| {
            let operation = quote::format_ident!("{}", operation_dsc.operation.to_string());
            let http_method = match crate::frontend::http_method(operation_dsc) {
                OperationHttpMethod::Post => quote::format_ident!("Post"),
                OperationHttpMethod::Patch => quote::format_ident!("Patch"),
                OperationHttpMethod::Delete => quote::format_ident!("Delete"),
            };
            let success_status = if crate::frontend::success_status(operation_dsc)
                == macros_helpers::status_code::StatusCode::Created201
            {
                quote::format_ident!("Code201")
            } else {
                quote::format_ident!("Code200")
            };
            let idempotency_required = generate_pg_table_input_model.config.idempotent_mutations
                && crate::sql::idempotency_capable(operation_dsc);
            let optimistic_revision_required = optimistic_revision_field_idx.is_some()
                && crate::sql::optimistic_concurrency_capable(operation_dsc);
            let authentication = generate_pg_table_input_model
                .config
                .permission_prefix
                .as_ref()
                .map_or_else(
                    || quote::quote! {#identifier_auth_requirement_upper_camel_case::Public},
                    |permission_prefix| {
                        let permission = format!(
                            "{permission_prefix}:{}",
                            crate::frontend::permission_action(operation_dsc)
                        );
                        quote::quote! {#identifier_auth_requirement_upper_camel_case::Permission(#permission)}
                    },
                );
            quote::quote! {
                #identifier_route_contract_upper_camel_case::new_with_capabilities(
                    #authentication,
                    #identifier_http_method_upper_camel_case::#http_method,
                    #idempotency_required,
                    #identifier_operation_upper_camel_case::#operation,
                    #optimistic_revision_required,
                    #identifier_success_status_upper_camel_case::#success_status,
                )
            }
        });
    let route_contract_path_arms_token_stream =
        crate::model::OperationDsc::ALL.iter().map(|operation_dsc| {
            let operation = quote::format_ident!("{}", operation_dsc.operation.to_string());
            let path = format!(
                "/{}/{}",
                identifier_snake_case_string,
                operation_dsc.operation.self_snake_case_str()
            );
            quote::quote! {#identifier_operation_upper_camel_case::#operation => #path}
        });
    let route_contract_payload_example_path_arms_token_stream =
        crate::model::OperationDsc::ALL.iter().map(|operation_dsc| {
            let operation = quote::format_ident!("{}", operation_dsc.operation.to_string());
            let path = format!(
                "/{}/{}",
                identifier_snake_case_string,
                operation_dsc
                    .operation
                    .operation_payload_example_snake_case()
            );
            quote::quote! {#identifier_operation_upper_camel_case::#operation => #path}
        });
    let route_contract_operation_kind_arms_token_stream = crate::model::OperationDsc::ALL.iter().map(|operation_dsc| {
        let operation = quote::format_ident!("{}", operation_dsc.operation.to_string());
        let operation_kind = match crate::frontend::operation_kind(operation_dsc) {
            OperationKind::CreateMany => quote::format_ident!("CreateMany"),
            OperationKind::CreateOne => quote::format_ident!("CreateOne"),
            OperationKind::DeleteMany => quote::format_ident!("DeleteMany"),
            OperationKind::DeleteOne => quote::format_ident!("DeleteOne"),
            OperationKind::ReadMany => quote::format_ident!("ReadMany"),
            OperationKind::ReadOne => quote::format_ident!("ReadOne"),
            OperationKind::UpdateMany => quote::format_ident!("UpdateMany"),
            OperationKind::UpdateOne => quote::format_ident!("UpdateOne"),
        };
        quote::quote! {#identifier_operation_upper_camel_case::#operation => frontend_contract::OperationKind::#operation_kind}
    });
    let identifier_route_contract_token_stream = quote::quote! {
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        pub enum #identifier_auth_requirement_upper_camel_case {
            Permission(&'static str),
            Public,
        }
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        pub enum #identifier_http_method_upper_camel_case {
            Delete,
            Get,
            Patch,
            Post,
        }
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        pub enum #identifier_operation_upper_camel_case {
            Cm,
            Co,
            Dlo,
            Dm,
            Rm,
            Ro,
            Um,
            Uo,
        }
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        pub enum #identifier_success_status_upper_camel_case {
            Code200,
            Code201,
        }
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        pub struct #identifier_route_contract_upper_camel_case {
            authentication: #identifier_auth_requirement_upper_camel_case,
            http_method: #identifier_http_method_upper_camel_case,
            idempotency_required: bool,
            operation: #identifier_operation_upper_camel_case,
            optimistic_revision_required: bool,
            success_status: #identifier_success_status_upper_camel_case,
            payload_example: bool,
        }
        impl #identifier_route_contract_upper_camel_case {
            pub const ALL: [Self; #enabled_operation_count] = [#(#route_contract_items_token_stream),*];
            #[must_use]
            pub const fn authentication(self) -> #identifier_auth_requirement_upper_camel_case {
                self.authentication
            }
            #[must_use]
            pub const fn http_method(self) -> #identifier_http_method_upper_camel_case {
                self.http_method
            }
            #[must_use]
            pub const fn new(authentication: #identifier_auth_requirement_upper_camel_case, http_method: #identifier_http_method_upper_camel_case, operation: #identifier_operation_upper_camel_case, success_status: #identifier_success_status_upper_camel_case) -> Self {
                Self { authentication, http_method, idempotency_required: false, operation, optimistic_revision_required: false, success_status, payload_example: false }
            }
            const fn new_with_capabilities(authentication: #identifier_auth_requirement_upper_camel_case, http_method: #identifier_http_method_upper_camel_case, idempotency_required: bool, operation: #identifier_operation_upper_camel_case, optimistic_revision_required: bool, success_status: #identifier_success_status_upper_camel_case) -> Self {
                Self { authentication, http_method, idempotency_required, operation, optimistic_revision_required, success_status, payload_example: false }
            }
            #[must_use]
            pub const fn payload_example(self) -> Self {
                Self {
                    authentication: self.authentication,
                    http_method: #identifier_http_method_upper_camel_case::Get,
                    idempotency_required: false,
                    operation: self.operation,
                    optimistic_revision_required: false,
                    success_status: #identifier_success_status_upper_camel_case::Code200,
                    payload_example: true,
                }
            }
            #[must_use]
            pub const fn idempotency_required(self) -> bool {
                self.idempotency_required
            }
            #[must_use]
            pub const fn operation(self) -> #identifier_operation_upper_camel_case {
                self.operation
            }
            #[must_use]
            pub const fn optimistic_revision_required(self) -> bool {
                self.optimistic_revision_required
            }
            #[must_use]
            pub fn for_path(path: &str) -> Option<Self> {
                Self::ALL
                    .into_iter()
                    .find(|contract| path.ends_with(contract.path()))
                    .or_else(|| {
                        Self::ALL
                            .into_iter()
                            .map(Self::payload_example)
                            .find(|contract| path.ends_with(contract.path()))
                    })
            }
            #[must_use]
            pub fn frontend_contract(self) -> frontend_contract::RouteContract {
                let authentication = match self.authentication {
                    #identifier_auth_requirement_upper_camel_case::Permission(permission) => frontend_contract::AuthenticationRequirement::Permission(frontend_contract::ContractStr::from(permission)),
                    #identifier_auth_requirement_upper_camel_case::Public => frontend_contract::AuthenticationRequirement::Public,
                };
                let method = match self.http_method {
                    #identifier_http_method_upper_camel_case::Delete => frontend_contract::HttpMethod::Delete,
                    #identifier_http_method_upper_camel_case::Get => frontend_contract::HttpMethod::Get,
                    #identifier_http_method_upper_camel_case::Patch => frontend_contract::HttpMethod::Patch,
                    #identifier_http_method_upper_camel_case::Post => frontend_contract::HttpMethod::Post,
                };
                let mutation = if self.mutates() {
                    frontend_contract::MutationKind::Mutating
                } else {
                    frontend_contract::MutationKind::ReadOnly
                };
                let success_status = match self.success_status {
                    #identifier_success_status_upper_camel_case::Code200 => frontend_contract::SuccessStatus::Code200,
                    #identifier_success_status_upper_camel_case::Code201 => frontend_contract::SuccessStatus::Code201,
                };
                frontend_contract::RouteContract::new(authentication, method, mutation, frontend_contract::ContractStr::from(self.path()), success_status)
            }
            #[must_use]
            pub fn frontend_contracts() -> frontend_contract::RouteContracts {
                frontend_contract::RouteContracts::from_max_iter(Self::ALL.into_iter().map(Self::frontend_contract))
            }
            #[must_use]
            pub fn frontend_action(self) -> frontend_contract::ActionContract {
                let operation = match self.operation {
                    #(#route_contract_operation_kind_arms_token_stream),*
                };
                let action = frontend_contract::ActionContract::new(operation, self.frontend_contract());
                if matches!(self.operation, #identifier_operation_upper_camel_case::Dm | #identifier_operation_upper_camel_case::Dlo) {
                    action.with_confirmation(frontend_contract::ConfirmationRequirement::Required)
                } else {
                    action
                }
            }
            #[must_use]
            pub fn frontend_actions() -> frontend_contract::ActionContracts {
                frontend_contract::ActionContracts::from_max_iter(Self::ALL.into_iter().map(Self::frontend_action))
            }
            #[must_use]
            pub const fn mutates(self) -> bool {
                !self.payload_example && matches!(self.operation, #identifier_operation_upper_camel_case::Cm | #identifier_operation_upper_camel_case::Co | #identifier_operation_upper_camel_case::Um | #identifier_operation_upper_camel_case::Uo | #identifier_operation_upper_camel_case::Dm | #identifier_operation_upper_camel_case::Dlo)
            }
            #[must_use]
            pub const fn path(self) -> &'static str {
                if self.payload_example {
                    match self.operation {
                        #(#route_contract_payload_example_path_arms_token_stream),*
                    }
                } else {
                    match self.operation {
                        #(#route_contract_path_arms_token_stream),*
                    }
                }
            }
            #[must_use]
            pub const fn permission(self) -> Option<&'static str> {
                match self.authentication {
                    #identifier_auth_requirement_upper_camel_case::Permission(permission) => Some(permission),
                    #identifier_auth_requirement_upper_camel_case::Public => None,
                }
            }
            #[must_use]
            pub const fn success_status(self) -> #identifier_success_status_upper_camel_case {
                self.success_status
            }
        }
    };
    let identifier_open_api_upper_camel_case = quote::format_ident!("{}OpenApi", identifier);
    let generate_role_schema_items_token_stream = |role: &dyn quote::ToTokens| {
        fields
            .iter()
            .map(|field| {
                let role_type_token_stream =
                    generate_concrete_pg_type_role_token_stream(&field.type0, role);
                quote::quote! {<#role_type_token_stream as utoipa::PartialSchema>::schema()}
            })
            .collect::<Vec<_>>()
    };
    let read_schema_items_token_stream =
        generate_role_schema_items_token_stream(&ReadUpperCamelCase);
    let select_schema_items_token_stream =
        generate_role_schema_items_token_stream(&SelectUpperCamelCase);
    let generate_filter_schema_items_token_stream =
        |filter_upper_camel_case: &dyn quote::ToTokens| {
            fields
            .iter()
            .map(|field| {
                let table_type_type_token_stream = generate_as_pg_type_tokens_token_stream(&field.type0, &naming::TableTypeUpperCamelCase);
                quote::quote! {<where_filters::#filter_upper_camel_case<#table_type_type_token_stream> as utoipa::PartialSchema>::schema()}
            })
            .collect::<Vec<_>>()
        };
    let generate_ordered_filter_schema_items_token_stream =
        |filter_upper_camel_case: &dyn quote::ToTokens| {
            fields
            .iter()
            .map(|field| {
                let table_type_type_token_stream =
                    generate_concrete_standard_non_null_pg_type_role_token_stream(&field.type0, &naming::TableTypeUpperCamelCase);
                quote::quote! {<where_filters::#filter_upper_camel_case<#table_type_type_token_stream> as utoipa::PartialSchema>::schema()}
            })
            .collect::<Vec<_>>()
        };
    let eq_filter_schema_items_token_stream =
        generate_filter_schema_items_token_stream(&quote::format_ident!("PgTypeWhereEq"));
    let between_filter_schema_items_token_stream =
        generate_ordered_filter_schema_items_token_stream(&quote::format_ident!(
            "PgTypeWhereBetween"
        ));
    let greater_than_filter_schema_items_token_stream =
        generate_ordered_filter_schema_items_token_stream(&quote::format_ident!(
            "PgTypeWhereGreaterThan"
        ));
    let in_filter_schema_items_token_stream =
        generate_filter_schema_items_token_stream(&quote::format_ident!("PgTypeWhereIn"));
    let before_filter_schema_items_token_stream =
        generate_filter_schema_items_token_stream(&quote::format_ident!("PgTypeWhereBefore"));
    let range_filter_schemas = [
        naming::FindRangesWithinGivenRangeUpperCamelCase.to_string(),
        naming::FindRangesThatFullyContainTheGivenRangeUpperCamelCase.to_string(),
        naming::StrictlyToLeftOfRangeUpperCamelCase.to_string(),
        naming::StrictlyToRightOfRangeUpperCamelCase.to_string(),
        naming::IncludedLowerBoundUpperCamelCase.to_string(),
        naming::ExcludedUpperBoundUpperCamelCase.to_string(),
        naming::GreaterThanIncludedLowerBoundUpperCamelCase.to_string(),
        naming::GreaterThanExcludedUpperBoundUpperCamelCase.to_string(),
        naming::OverlapWithRangeUpperCamelCase.to_string(),
        naming::AdjacentWithRangeUpperCamelCase.to_string(),
    ]
    .into_iter()
    .map(|name| {
        let schema_name = format!("where_filters.PgTypeWhere{name}");
        let type_name = quote::format_ident!("PgTypeWhere{name}");
        let items = generate_filter_schema_items_token_stream(&type_name);
        let schema = quote::quote! {
            utoipa::openapi::schema::Schema::from(
                utoipa::openapi::OneOfBuilder::new()#(.item(#items))*.build()
            ).into()
        };
        (schema_name, schema)
    })
    .collect::<Vec<_>>();
    let range_filter_schema_names = range_filter_schemas.iter().map(|(name, _items)| name);
    let range_filter_schema_values = range_filter_schemas.iter().map(|(_name, schema)| schema);
    let static_filter_schemas = [
        naming::RegexUpperCamelCase.to_string(),
        naming::CurrentDateUpperCamelCase.to_string(),
        naming::GreaterThanCurrentDateUpperCamelCase.to_string(),
        naming::CurrentTimestampUpperCamelCase.to_string(),
        naming::GreaterThanCurrentTimestampUpperCamelCase.to_string(),
        naming::CurrentTimeUpperCamelCase.to_string(),
        naming::GreaterThanCurrentTimeUpperCamelCase.to_string(),
        naming::EqToEncodedStringRepresentationUpperCamelCase.to_string(),
        naming::RangeLenUpperCamelCase.to_string(),
    ]
    .into_iter()
    .map(|name| {
        let schema_name = format!("where_filters.PgTypeWhere{name}");
        let type_name = quote::format_ident!("PgTypeWhere{name}");
        let schema = quote::quote! {<where_filters::#type_name as utoipa::PartialSchema>::schema()};
        (schema_name, schema)
    })
    .collect::<Vec<_>>();
    let static_filter_schema_names = static_filter_schemas.iter().map(|(name, _schema)| name);
    let static_filter_schema_values = static_filter_schemas.iter().map(|(_name, schema)| schema);
    let in_value_schema_items_token_stream = fields.iter().map(|field| {
        let table_type_type_token_stream = generate_as_pg_type_tokens_token_stream(&field.type0, &naming::TableTypeUpperCamelCase);
        quote::quote! {<where_filters::PgTypeNotEmptyUniqueVec<#table_type_type_token_stream> as utoipa::PartialSchema>::schema()}
    }).collect::<Vec<_>>();
    let in_value_schema_names_token_stream = fields
        .iter()
        .map(|field| {
            let table_type_type_token_stream = generate_as_pg_type_tokens_token_stream(
                &field.type0,
                &naming::TableTypeUpperCamelCase,
            );
            quote::quote! {
                format!(
                    "{}_{}",
                    <where_filters::PgTypeNotEmptyUniqueVec<#table_type_type_token_stream> as utoipa::ToSchema>::name(),
                    <#table_type_type_token_stream as utoipa::ToSchema>::name(),
                )
            }
        })
        .collect::<Vec<_>>();
    let open_api_security_schemes_token_stream = generate_pg_table_input_model
        .config
        .permission_prefix
        .as_ref()
        .map_or_else(proc_macro2::TokenStream::new, |_| {
            quote::quote! {
                components.add_security_scheme(
                    "admin_cookie",
                    utoipa::openapi::security::SecurityScheme::ApiKey(
                        utoipa::openapi::security::ApiKey::Cookie(
                            utoipa::openapi::security::ApiKeyValue::with_description(
                                "admin_access_token",
                                "HttpOnly administrator access-token cookie",
                            ),
                        ),
                    ),
                );
                components.add_security_scheme(
                    "admin_csrf",
                    utoipa::openapi::security::SecurityScheme::ApiKey(
                        utoipa::openapi::security::ApiKey::Header(
                            utoipa::openapi::security::ApiKeyValue::with_description(
                                "X-CSRF-Token",
                                "CSRF token required for mutating cookie-authenticated requests",
                            ),
                        ),
                    ),
                );
            }
        });
    let path_separator_literal = proc_macro2::Literal::string(str_constants::PATH_SEPARATOR);
    let dot_literal = proc_macro2::Literal::string(str_constants::DOT);
    let open_api_path_type_identifiers = open_api_path_fn_identifiers
        .iter()
        .map(|path_identifier| quote::format_ident!("__generated_path_{path_identifier}"))
        .collect::<Vec<_>>();
    let identifier_open_api_token_stream = quote::quote! {
        #[allow(clippy::needless_for_each)] // generated schema registration uses iterator callbacks internally
        pub struct #identifier_open_api_upper_camel_case;
        impl utoipa::OpenApi for #identifier_open_api_upper_camel_case {
            fn openapi() -> utoipa::openapi::OpenApi {
                utoipa::openapi::OpenApiBuilder::new()
                    .info(
                        utoipa::openapi::InfoBuilder::new()
                            .title(env!("CARGO_PKG_NAME"))
                            .version(env!("CARGO_PKG_VERSION"))
                            .description(Some(env!("CARGO_PKG_DESCRIPTION")))
                            .license(Some(
                                utoipa::openapi::info::LicenseBuilder::new()
                                    .name(env!("CARGO_PKG_LICENSE"))
                                    .identifier(Some(env!("CARGO_PKG_LICENSE")))
                                    .build(),
                            )),
                    )
                    .paths(utoipa::openapi::path::PathsBuilder::new())
                    .tags(Some([
                        utoipa::openapi::tag::TagBuilder::new()
                            .name(#identifier_snake_case_string)
                            .description(Some("Generated CRUD API"))
                            .build(),
                    ]))
                    .build()
            }
        }
        #[allow(clippy::needless_for_each)] // recursive schema-reference normalization is clearer as iterator traversal
        impl #identifier_open_api_upper_camel_case {
            #[must_use]
            pub fn open_api() -> utoipa::openapi::OpenApi {
                fn collect_refs(value: &serde_json::Value, refs: &mut std::collections::BTreeSet<String>) {
                    match value {
                        serde_json::Value::Array(values) => values.iter().for_each(|value| collect_refs(value, refs)),
                        serde_json::Value::Object(values) => values.iter().for_each(|(key, value)| {
                            if key == "$ref"
                                && let Some(name) = value.as_str().and_then(|value| value.strip_prefix("#/components/schemas/"))
                            {
                                refs.insert(name.to_owned());
                            }
                            collect_refs(value, refs);
                        }),
                        serde_json::Value::Null | serde_json::Value::Bool(_) | serde_json::Value::Number(_) | serde_json::Value::String(_) => {}
                    }
                }
                let mut open_api = <Self as utoipa::OpenApi>::openapi();
                #({
                    let path = <#open_api_path_type_identifiers as utoipa::Path>::path();
                    let path_item = utoipa::openapi::path::PathItem::from_http_methods(
                        <#open_api_path_type_identifiers as utoipa::Path>::methods(),
                        <#open_api_path_type_identifiers as utoipa::Path>::operation(),
                    );
                    open_api
                        .paths
                        .paths
                        .entry(path)
                        .and_modify(|existing| existing.merge_operations(path_item.clone()))
                        .or_insert(path_item);
                })*
                let components = open_api
                    .components
                    .get_or_insert_with(utoipa::openapi::Components::new);
                {
                    let mut schema_components =
                        frontend_contract::UtoipaOpenApiComponentsRefMut::from(&mut *components);
                    #(
                        frontend_contract::register_openapi_schema::<#open_api_schema_types_token_stream>(
                            &mut schema_components,
                        );
                    )*
                }
                {
                    #open_api_security_schemes_token_stream
                    components.schemas.insert("pg_crud_common.PgType.Read".to_owned(), utoipa::openapi::schema::Schema::from(utoipa::openapi::OneOfBuilder::new()#(.item(#read_schema_items_token_stream))*.build()).into());
                    components.schemas.insert("pg_crud_common.PgType.Select".to_owned(), utoipa::openapi::schema::Schema::from(utoipa::openapi::OneOfBuilder::new()#(.item(#select_schema_items_token_stream))*.build()).into());
                    components.schemas.insert("where_filters.PgTypeWhereEq".to_owned(), utoipa::openapi::schema::Schema::from(utoipa::openapi::OneOfBuilder::new()#(.item(#eq_filter_schema_items_token_stream))*.build()).into());
                    components.schemas.insert("where_filters.PgTypeWhereBetween".to_owned(), utoipa::openapi::schema::Schema::from(utoipa::openapi::OneOfBuilder::new()#(.item(#between_filter_schema_items_token_stream))*.build()).into());
                    components.schemas.insert("where_filters.PgTypeWhereGreaterThan".to_owned(), utoipa::openapi::schema::Schema::from(utoipa::openapi::OneOfBuilder::new()#(.item(#greater_than_filter_schema_items_token_stream))*.build()).into());
                    components.schemas.insert("where_filters.PgTypeWhereIn".to_owned(), utoipa::openapi::schema::Schema::from(utoipa::openapi::OneOfBuilder::new()#(.item(#in_filter_schema_items_token_stream))*.build()).into());
                    components.schemas.insert("where_filters.PgTypeWhereBefore".to_owned(), utoipa::openapi::schema::Schema::from(utoipa::openapi::OneOfBuilder::new()#(.item(#before_filter_schema_items_token_stream))*.build()).into());
                    #(components.schemas.insert(#range_filter_schema_names.to_owned(), #range_filter_schema_values);)*
                    #(components.schemas.insert(#static_filter_schema_names.to_owned(), #static_filter_schema_values);)*
                    components.schemas.insert(
                        format!(
                            "{}_{}",
                            <pg_crud_common::NotEmptyUniqueVec<#identifier_select_upper_camel_case> as utoipa::ToSchema>::name(),
                            <#identifier_select_upper_camel_case as utoipa::ToSchema>::name(),
                        ),
                        <pg_crud_common::NotEmptyUniqueVec<#identifier_select_upper_camel_case> as utoipa::PartialSchema>::schema(),
                    );
                    components.schemas.insert(
                        format!(
                            "{}_{}",
                            <pg_crud_common::OrderBy<#identifier_select_upper_camel_case> as utoipa::ToSchema>::name(),
                            <#identifier_select_upper_camel_case as utoipa::ToSchema>::name(),
                        ),
                        <pg_crud_common::OrderBy<#identifier_select_upper_camel_case> as utoipa::PartialSchema>::schema(),
                    );
                    components.schemas.insert("PgTypeNotEmptyUniqueVec".to_owned(), utoipa::openapi::schema::Schema::from(utoipa::openapi::OneOfBuilder::new()#(.item(#in_value_schema_items_token_stream))*.build()).into());
                    #(components.schemas.insert(#in_value_schema_names_token_stream, #in_value_schema_items_token_stream);)*
                }
                let mut refs = std::collections::BTreeSet::new();
                if let Ok(value) = serde_json::to_value(&open_api) {
                    collect_refs(&value, &mut refs);
                }
                if let Some(components) = open_api.components.as_mut() {
                    refs.into_iter().for_each(|name| {
                        if !components.schemas.contains_key(&name) {
                            let normalized_name = name
                                .split_whitespace()
                                .collect::<String>()
                                .replace(#path_separator_literal, #dot_literal);
                            let suffix = normalized_name
                                .rsplit(#dot_literal)
                                .next()
                                .unwrap_or(normalized_name.as_str());
                            if let Some(schema) = components.schemas.get(suffix).cloned() {
                                components.schemas.insert(name, schema);
                            }
                        }
                    });
                }
                open_api
            }
        }
    };
    let generated_contract_tests_token_stream = {
        let route_auth_assertion_token_stream = generate_pg_table_input_model
            .config
            .permission_prefix
            .as_ref()
            .map_or_else(
                || quote::quote! {
                    assert!(#identifier_route_contract_upper_camel_case::ALL.into_iter().all(|contract| contract.authentication() == #identifier_auth_requirement_upper_camel_case::Public));
                },
                |_| quote::quote! {
                    assert!(#identifier_route_contract_upper_camel_case::ALL.into_iter().all(|contract| matches!(contract.authentication(), #identifier_auth_requirement_upper_camel_case::Permission(_))));
                },
            );
        let api_mode_assertion_token_stream = match generate_pg_table_input_model.config.api_mode {
            GeneratePgTableApiMode::Crud if optimistic_revision_field_idx.is_some() => {
                quote::quote! {
                    assert_eq!(#identifier_route_contract_upper_camel_case::ALL.len(), 7usize);
                    assert!(#identifier_route_contract_upper_camel_case::ALL.into_iter().all(|contract| !matches!(contract.operation(), #identifier_operation_upper_camel_case::Um)));
                }
            }
            GeneratePgTableApiMode::Crud => quote::quote! {
                assert_eq!(#identifier_route_contract_upper_camel_case::ALL.len(), 8usize);
            },
            GeneratePgTableApiMode::AppendOnly => quote::quote! {
                assert!(#identifier_route_contract_upper_camel_case::ALL.into_iter().all(|contract| matches!(contract.operation(), #identifier_operation_upper_camel_case::Cm | #identifier_operation_upper_camel_case::Co | #identifier_operation_upper_camel_case::Rm | #identifier_operation_upper_camel_case::Ro)));
            },
            GeneratePgTableApiMode::CreateReadDelete => quote::quote! {
                assert!(#identifier_route_contract_upper_camel_case::ALL.into_iter().all(|contract| matches!(contract.operation(), #identifier_operation_upper_camel_case::Cm | #identifier_operation_upper_camel_case::Co | #identifier_operation_upper_camel_case::Rm | #identifier_operation_upper_camel_case::Ro | #identifier_operation_upper_camel_case::Dm | #identifier_operation_upper_camel_case::Dlo)));
            },
            GeneratePgTableApiMode::ReadOnly => quote::quote! {
                assert!(#identifier_route_contract_upper_camel_case::ALL.into_iter().all(|contract| matches!(contract.operation(), #identifier_operation_upper_camel_case::Rm | #identifier_operation_upper_camel_case::Ro)));
            },
            GeneratePgTableApiMode::ReadUpdate => quote::quote! {
                assert!(#identifier_route_contract_upper_camel_case::ALL.into_iter().all(|contract| matches!(contract.operation(), #identifier_operation_upper_camel_case::Rm | #identifier_operation_upper_camel_case::Ro | #identifier_operation_upper_camel_case::Um | #identifier_operation_upper_camel_case::Uo)));
            },
        };
        let round_trip_tests_token_stream = crate::model::OperationDsc::ALL.iter().map(|operation_dsc| {
            let operation = &operation_dsc.operation;
            let payload_type_token_stream = generate_identifier_operation_payload_upper_camel_case(operation);
            let test_identifier = quote::format_ident!(
                "{}_{}_payload_json_round_trip",
                identifier_snake_case_string,
                operation.self_snake_case_str()
            );
            let normalize_default_filter_token_stream = if matches!(operation, Operation::Rm | Operation::Dm) {
                quote::quote! {
                    serialized.as_object_mut().expect("58c97ca7 collect_refs invariant must hold").insert(
                        "where_many".to_owned(),
                        serde_json::Value::Null,
                    );
                }
            } else {
                proc_macro2::TokenStream::new()
            };
            let mut_token_stream = matches!(operation, Operation::Rm | Operation::Dm).then(|| quote::quote! { mut });
            quote::quote! {
                #[test]
                fn #test_identifier() {
                    let original: #payload_type_token_stream = pg_crud_common::DefaultSomeOneElement::default_some_one_element();
                    let #mut_token_stream serialized = serde_json::to_value(&original).expect("84094d13 collect_refs invariant must hold");
                    #normalize_default_filter_token_stream
                    let deserialized = serde_json::from_value::<#payload_type_token_stream>(serialized.clone()).expect("b388de0c collect_refs invariant must hold");
                    let round_trip = serde_json::to_value(deserialized).expect("570ac825 collect_refs invariant must hold");
                    assert_eq!(round_trip, serialized);
                }
            }
        });
        let unknown_field_tests_token_stream = [Operation::Rm, Operation::Ro, Operation::Dm, Operation::Dlo].into_iter().map(|operation| {
            let payload_type_token_stream = generate_identifier_operation_payload_upper_camel_case(&operation);
            let test_identifier = quote::format_ident!(
                "{}_{}_payload_rejects_unknown_field",
                identifier_snake_case_string,
                operation.self_snake_case_str()
            );
            quote::quote! {
                #[test]
                fn #test_identifier() {
                    let original: #payload_type_token_stream = pg_crud_common::DefaultSomeOneElement::default_some_one_element();
                    let mut serialized = serde_json::to_value(original).expect("aeedc9e8 collect_refs invariant must hold");
                    serialized.as_object_mut().expect("b9d4b58e collect_refs invariant must hold").insert(
                        "unknown_field".to_owned(),
                        serde_json::Value::Bool(true),
                    );
                    assert!(serde_json::from_value::<#payload_type_token_stream>(serialized).is_err());
                }
            }
        });
        let contract_tests_mod_identifier =
            quote::format_ident!("{}_contract_tests", identifier_snake_case_string);
        let api_client_owns_reusable_client_test_identifier = quote::format_ident!(
            "{}_api_client_owns_reusable_client",
            identifier_snake_case_string
        );
        let read_query_negative_contracts_test_identifier = quote::format_ident!(
            "{}_read_query_negative_contracts",
            identifier_snake_case_string
        );
        let route_open_api_parity_test_identifier =
            quote::format_ident!("{}_route_open_api_parity", identifier_snake_case_string);
        let identifier_rm_payload_upper_camel_case =
            generate_identifier_operation_payload_upper_camel_case(&Operation::Rm);
        let route_open_api_parity_assertions_token_stream = crate::model::OperationDsc::ALL
            .iter()
            .filter(|operation_dsc| operation_is_enabled(&operation_dsc.operation))
            .map(|operation_dsc| {
            let operation = operation_dsc.operation.self_snake_case_str();
            let open_api_operation_id = format!("{identifier_snake_case_string}_{operation}");
            let path = format!("/{identifier_snake_case_string}/{operation}");
            let method = match crate::contract_tests::http_method(operation_dsc) {
                OperationHttpMethod::Post => str_constants::POST_ALT,
                OperationHttpMethod::Patch => str_constants::PATCH_ALT,
                OperationHttpMethod::Delete => str_constants::PG_CRUD_DELETE_PERMISSION_ACTION,
            };
            let success_status = if crate::contract_tests::success_status(operation_dsc)
                == macros_helpers::status_code::StatusCode::Created201
            {
                str_constants::VALUE_201
            } else {
                str_constants::VALUE_200
            };
            let idempotency_required = generate_pg_table_input_model.config.idempotent_mutations
                && crate::sql::idempotency_capable(operation_dsc);
            let optimistic_revision_required = optimistic_revision_field_idx.is_some()
                && crate::sql::optimistic_concurrency_capable(operation_dsc);
            quote::quote! {
                let route_contract = #identifier_route_contract_upper_camel_case::for_path(#path).expect("80dc7c11 collect_refs invariant must hold");
                assert_eq!(route_contract.idempotency_required(), #idempotency_required);
                assert_eq!(route_contract.optimistic_revision_required(), #optimistic_revision_required);
                let operation_document = document.pointer(&format!("/paths/{}/{}", #path.replace('/', "~1"), #method)).expect("b822e594 collect_refs invariant must hold");
                assert_eq!(operation_document.get("operationId").and_then(serde_json::Value::as_str), Some(#open_api_operation_id));
                assert!(operation_document.pointer(&format!("/responses/{}", #success_status)).is_some());
                assert!(operation_document.pointer("/requestBody/content/application~1json").is_some());
                assert!(operation_document.pointer(&format!("/responses/{}/content/application~1json", #success_status)).is_some());
                assert!(operation_document.pointer("/responses/400").is_some());
                assert!(operation_document.pointer("/responses/413").is_some());
                assert!(operation_document.pointer("/responses/500").is_some());
                let parameters = operation_document.get("parameters").and_then(serde_json::Value::as_array);
                assert_eq!(parameters.is_some_and(|values| values.iter().any(|value| value.get("name").and_then(serde_json::Value::as_str) == Some("Idempotency-Key"))), #idempotency_required);
                assert_eq!(parameters.is_some_and(|values| values.iter().any(|value| value.get("name").and_then(serde_json::Value::as_str) == Some("If-Match"))), #optimistic_revision_required);
            }
        });
        let bulk_limit_tests_token_stream = [
            (Operation::Cm, generate_pg_table_input_model.config.cm_max_items),
            (Operation::Um, generate_pg_table_input_model.config.um_max_items),
        ]
        .into_iter()
        .filter_map(|(operation, optional_limit)| {
            let configured_limit = optional_limit?;
            let limit_value = configured_limit.0;
            let payload_type_token_stream = generate_identifier_operation_payload_upper_camel_case(&operation);
            let test_identifier = quote::format_ident!(
                "{}_{}_payload_enforces_item_limit",
                identifier_snake_case_string,
                operation.self_snake_case_str()
            );
            Some(quote::quote! {
                #[test]
                fn #test_identifier() {
                    let original: #payload_type_token_stream = pg_crud_common::DefaultSomeOneElement::default_some_one_element();
                    let original_value = serde_json::to_value(original).expect("d4d4cc0d collect_refs invariant must hold");
                    let item = original_value.as_array().and_then(|items| items.first()).cloned().expect("79b00707 collect_refs invariant must hold");
                    assert!(serde_json::from_value::<#payload_type_token_stream>(original_value).is_ok());
                    let above_limit = serde_json::Value::Array(std::iter::repeat_n(item, #limit_value.saturating_add(1usize)).collect());
                    match serde_json::from_value::<#payload_type_token_stream>(above_limit) {
                        Ok(_) => panic!("1a74209c"),
                        Err(error) => assert!(error.to_string().contains("exceeds limit")),
                    }
                }
            })
        });
        let create_excluded_fields_test_token_stream = if create_exclude_fields.is_empty() {
            proc_macro2::TokenStream::new()
        } else {
            let test_identifier = quote::format_ident!(
                "{}_create_excluded_fields_are_not_public",
                identifier_snake_case_string
            );
            let excluded_fields: Vec<&str> =
                create_exclude_fields.iter().map(AsRef::as_ref).collect();
            quote::quote! {
                #[test]
                fn #test_identifier() {
                    let value = serde_json::to_value(<#identifier_create_upper_camel_case as pg_crud_common::DefaultSomeOneElement>::default_some_one_element()).expect("629e2f81 collect_refs invariant must hold");
                    let properties = value.as_object().expect("e3f16d97 collect_refs invariant must hold");
                    #(
                        assert!(!properties.contains_key(#excluded_fields));
                    )*
                    let document = serde_json::to_value(#identifier_open_api_upper_camel_case::open_api()).expect("46eabc30 collect_refs invariant must hold");
                    let schema_properties = document.pointer(concat!("/components/schemas/", stringify!(#identifier_create_upper_camel_case), "/properties")).and_then(serde_json::Value::as_object).expect("b8537774 collect_refs invariant must hold");
                    #(
                        assert!(!schema_properties.contains_key(#excluded_fields));
                    )*
                }
            }
        };
        let read_excluded_fields_test_token_stream = if read_exclude_fields.is_empty() {
            proc_macro2::TokenStream::new()
        } else {
            let test_identifier = quote::format_ident!(
                "{}_read_excluded_fields_are_not_public",
                identifier_snake_case_string
            );
            let excluded_fields: Vec<&str> =
                read_exclude_fields.iter().map(AsRef::as_ref).collect();
            quote::quote! {
                #[test]
                fn #test_identifier() {
                    let document = serde_json::to_value(#identifier_open_api_upper_camel_case::open_api()).expect("5014a91c collect_refs invariant must hold");
                    let read_properties = document.pointer(concat!("/components/schemas/", stringify!(#identifier_read_upper_camel_case), "/properties")).and_then(serde_json::Value::as_object).expect("0241d202 collect_refs invariant must hold");
                    let filter_properties = document.pointer(concat!("/components/schemas/", stringify!(#identifier_where_upper_camel_case), "/properties")).and_then(serde_json::Value::as_object).expect("ad94914b collect_refs invariant must hold");
                    let selection_schema = document.pointer(concat!("/components/schemas/", stringify!(#identifier_select_upper_camel_case))).expect("fae40d82 collect_refs invariant must hold").to_string();
                    #(
                        assert!(!read_properties.contains_key(#excluded_fields));
                        assert!(!filter_properties.contains_key(#excluded_fields));
                        assert!(!selection_schema.contains(#excluded_fields));
                    )*
                }
            }
        };
        quote::quote! {
            #[cfg(test)]
            mod #contract_tests_mod_identifier {
                use super::*;
                #[test]
                #[cfg_attr(
                    miri,
                    ignore = "native TLS initialization calls OpenSSL functions that Miri does not support"
                )]
                fn #api_client_owns_reusable_client_test_identifier() {
                    let url = reqwest::Url::parse("http://127.0.0.1:3000/").expect("ca76d3e6 collect_refs invariant must hold");
                    let endpoint = #identifier_api_endpoint_upper_camel_case::from(url.clone());
                    assert_eq!(endpoint.as_url(), &url);
                    let client = #identifier_api_client_upper_camel_case::new(reqwest::Client::new(), endpoint);
                    assert!(format!("{client:?}").contains(stringify!(#identifier_api_client_upper_camel_case)));
                }
                #[test]
                fn #read_query_negative_contracts_test_identifier() {
                    let original: #identifier_rm_payload_upper_camel_case = pg_crud_common::DefaultSomeOneElement::default_some_one_element();
                    let serialized = serde_json::to_value(original).expect("bbb88adf collect_refs invariant must hold");
                    let mut empty_filter_payload = serialized.clone();
                    empty_filter_payload.as_object_mut().expect("aa1919f0 collect_refs invariant must hold").insert("where_many".to_owned(), serde_json::json!({}));
                    assert!(serde_json::from_value::<#identifier_rm_payload_upper_camel_case>(empty_filter_payload).is_err());
                    let mut unknown_filter_payload = serialized.clone();
                    unknown_filter_payload.as_object_mut().expect("42671a58 collect_refs invariant must hold").insert("where_many".to_owned(), serde_json::json!({"unknown_field": null}));
                    assert!(serde_json::from_value::<#identifier_rm_payload_upper_camel_case>(unknown_filter_payload).is_err());
                    let where_many = serialized.get("where_many").and_then(serde_json::Value::as_object).expect("e0b089c7 collect_refs invariant must hold");
                    let (field_name, field_filter) = where_many.iter().next().expect("5d781d42 collect_refs invariant must hold");
                    let filters = field_filter.get("v").and_then(serde_json::Value::as_array).expect("2ca9da9a collect_refs invariant must hold");
                    let mut multi_operator = filters.first().and_then(serde_json::Value::as_object).cloned().expect("3a86c2c9 collect_refs invariant must hold");
                    let (second_operator_name, second_operator_value) = filters.get(1usize).and_then(serde_json::Value::as_object).and_then(|value| value.iter().next()).expect("8589f0ef collect_refs invariant must hold");
                    multi_operator.insert(second_operator_name.clone(), second_operator_value.clone());
                    let mut multi_operator_field_filter = field_filter.clone();
                    let multi_operator_filters = multi_operator_field_filter.as_object_mut().and_then(|value| value.get_mut("v")).and_then(serde_json::Value::as_array_mut).expect("5df08753 collect_refs invariant must hold");
                    multi_operator_filters.clear();
                    multi_operator_filters.push(serde_json::Value::Object(multi_operator));
                    let mut multi_operator_payload = serialized.clone();
                    let mut multi_operator_where_many = serde_json::Map::new();
                    multi_operator_where_many.insert(field_name.clone(), multi_operator_field_filter);
                    multi_operator_payload.as_object_mut().expect("c92118fe collect_refs invariant must hold").insert("where_many".to_owned(), serde_json::Value::Object(multi_operator_where_many));
                    assert!(serde_json::from_value::<#identifier_rm_payload_upper_camel_case>(multi_operator_payload).is_err());
                    let duplicate_filter_json = format!("{{\"{field_name}\":{field_filter},\"{field_name}\":{field_filter}}}");
                    assert!(serde_json::from_str::<#identifier_where_upper_camel_case>(&duplicate_filter_json).is_err());
                    let mut cursor_payload = serialized;
                    cursor_payload.as_object_mut().expect("c12f9360 collect_refs invariant must hold").insert("cursor".to_owned(), serde_json::Value::String("forbidden".to_owned()));
                    assert!(serde_json::from_value::<#identifier_rm_payload_upper_camel_case>(cursor_payload).is_err());
                }
                #[test]
                fn #route_open_api_parity_test_identifier() {
                    let document = serde_json::to_value(#identifier_open_api_upper_camel_case::open_api()).expect("eb512de9 collect_refs invariant must hold");
                    assert_eq!(#identifier_route_contract_upper_camel_case::ALL.len(), #enabled_operation_count);
                    #api_mode_assertion_token_stream
                    #route_auth_assertion_token_stream
                    let operation_ids = #identifier_route_contract_upper_camel_case::ALL.into_iter().map(|contract| format!("{:?}", contract.operation())).collect::<std::collections::BTreeSet<String>>();
                    assert_eq!(operation_ids.len(), #identifier_route_contract_upper_camel_case::ALL.len());
                    assert!(#identifier_route_contract_upper_camel_case::ALL.into_iter().all(|contract| #identifier_route_contract_upper_camel_case::for_path(contract.path()) == Some(contract)));
                    assert!(#identifier_route_contract_upper_camel_case::for_path("/unknown").is_none());
                    assert_eq!(document.get("paths").and_then(serde_json::Value::as_object).map(serde_json::Map::len), Some(#identifier_route_contract_upper_camel_case::ALL.len()));
                    #(#route_open_api_parity_assertions_token_stream)*
                }
                #(#round_trip_tests_token_stream)*
                #(#unknown_field_tests_token_stream)*
                #(#bulk_limit_tests_token_stream)*
                #create_excluded_fields_test_token_stream
                #read_excluded_fields_test_token_stream
            }
        }
    };
    impl_identifier_vec_token_stream.push(quote::quote! {
        pub fn #RoutesSnakeCase(#AppStateSnakeCase: #std_sync_arc_combination_of_app_state_logic_traits_token_stream) -> axum::Router {
            Self::#RoutesHSnakeCase(
                #AppStateSnakeCase,
                #self_table_name_call_token_stream,
                #self_db_table_name_call_token_stream,
            )
        }
    });
    let (operator_or_token_stream, operator_and_token_stream) = {
        let operator_token_stream = quote::quote! {#import_token_stream Operator::};
        (
            quote::quote! {#operator_token_stream Or},
            quote::quote! {#operator_token_stream And},
        )
    };
    let generated_identifier_tests_token_stream = {
        fn generate_assert_token_stream(
            ts0: &dyn quote::ToTokens,
            ts1: &dyn quote::ToTokens,
        ) -> proc_macro2::TokenStream {
            quote::quote! {assert!(#ts0,#ts1);}
        }
        fn generate_assert_eq_token_stream(
            ts0: &dyn quote::ToTokens,
            ts1: &dyn quote::ToTokens,
            ts2: &dyn quote::ToTokens,
        ) -> proc_macro2::TokenStream {
            quote::quote! {assert_eq!(#ts0,#ts1,#ts2);}
        }
        let generate_primary_key_where_eq_token_stream = |ts0: &dyn quote::ToTokens| {
            quote::quote! {
                #primary_key_field_type_as_pg_type_where_token_stream::Eq(
                    #import_token_stream PgTypeWhereEq {
                        operator: #operator_or_token_stream,
                        #VSnakeCase: #ts0,
                    },
                )
            }
        };
        let generate_primary_key_where_eq_new_token_stream = |ts0: &dyn quote::ToTokens| {
            generate_primary_key_where_eq_token_stream(
                &quote::quote! {#primary_key_field_type_table_type_token_stream::new(#ts0)},
            )
        };
        let primary_key_where_eq_uuid_new_v_token_stream =
            generate_primary_key_where_eq_new_token_stream(
                &quote::quote! {uuid::Uuid::from_u128(2u128)},
            );
        let generate_primary_key_where_eq_into_inner_token_stream = |ts0: &dyn quote::ToTokens| {
            generate_primary_key_where_eq_new_token_stream(
                &quote::quote! {#primary_key_as_pg_type_token_stream::into_inner(#ts0)},
            )
        };
        let identifier_tests_snake_case =
            naming::parameter::SelfTestsSnakeCase::from_display(&identifier);
        let identifier_double_quoted_token_stream = generate_quotes::dq_token_stream(
            &naming_common::DisplayToSnakeCaseStr::case(&identifier),
        );
        let identifier_cm_parameters_upper_camel_case =
            generate_identifier_operation_parameters_upper_camel_case(&Operation::Cm);
        let identifier_rm_parameters_upper_camel_case =
            generate_identifier_operation_parameters_upper_camel_case(&Operation::Rm);
        let identifier_cm_payload_upper_camel_case =
            generate_identifier_operation_payload_upper_camel_case(&Operation::Cm);
        let identifier_rm_payload_upper_camel_case =
            generate_identifier_operation_payload_upper_camel_case(&Operation::Rm);
        let identifier_co_parameters_upper_camel_case =
            generate_identifier_operation_parameters_upper_camel_case(&Operation::Co);
        let identifier_ro_parameters_upper_camel_case =
            generate_identifier_operation_parameters_upper_camel_case(&Operation::Ro);
        let identifier_ro_payload_upper_camel_case =
            generate_identifier_operation_payload_upper_camel_case(&Operation::Ro);
        let identifier_uo_parameters_upper_camel_case =
            generate_identifier_operation_parameters_upper_camel_case(&Operation::Uo);
        let config_path_token_stream = quote::quote! {server_config::Config};
        let undrscr_unused_token_stream = quote::quote! {_unused};
        //todo maybe remove it?\
        let generate_some_pg_type_where_try_new_token_stream =
            |operator_token_stream: &dyn quote::ToTokens, ts: &dyn quote::ToTokens| {
                quote::quote! {
                    Some(
                        #import_token_stream PgTypeWhere::try_new(
                            #operator_token_stream,
                            (#ts).into()
                        ).expect("6b0491b2 generate_assert_eq_token_stream invariant must hold"),
                    )
                }
            };
        let generate_some_pg_type_where_try_new_and_token_stream = |ts: &dyn quote::ToTokens| {
            generate_some_pg_type_where_try_new_token_stream(&operator_and_token_stream, ts)
        };
        let generate_pg_type_where_try_new_primary_key_token_stream = quote::quote! {
            #import_token_stream PgTypeWhere::try_new(
                operator,
                vec.into()
            ).expect("fd20ad6d generate_assert_eq_token_stream invariant must hold")
        };
        let identifier_create_default_fields_initialization_without_primary_key_token_stream =
            generate_fields_named_without_primary_key_with_comma_token_stream(
                &|element: &macros_helpers::field_data::SynField| {
                    let field = &element.identifier;
                    let field_type_as_pg_type_create_token_stream =
                        generate_as_pg_type_create_token_stream(&element.type0);
                    quote::quote! {
                        #field: <#field_type_as_pg_type_create_token_stream as #import_token_stream DefaultSomeOneElement>::default_some_one_element()
                    }
                },
            );
        let fields_none_initialization_token_stream =
            generate_fields_named_without_primary_key_with_comma_token_stream(
                &|element: &macros_helpers::field_data::SynField| {
                    let field = &element.identifier;
                    quote::quote! {#field: None}
                },
            );
        //todo instead of first dropping table - check if its not exists. if exists Test must fail
        let select_default_all_with_max_page_size_not_empty_unique_vec_token_stream = {
            let ts = generate_read_fields_with_comma_token_stream(
                &|element: &macros_helpers::field_data::SynField| {
                    let field = &element.identifier;
                    let field_type = &element.type0;
                    let field_upper_camel_case =
                        naming_common::ToTokensToUpperCamelCaseTokenStream::case_or_panic(&field);
                    quote::quote! {
                        #identifier_select_upper_camel_case::#field_upper_camel_case(
                            <<#field_type as #import_token_stream PgType>::Select as #import_token_stream #DefaultSomeOneElementMaxPageSizeUpperCamelCase>::#DefaultSomeOneElementMaxPageSizeSnakeCase()
                        )
                    }
                },
            );
            quote::quote! {
                let select_default_all_with_max_page_size = #import_token_stream NotEmptyUniqueVec::try_new_by_hash(vec![
                    #ts
                ].into()).expect("5e82ac66 generate_assert_eq_token_stream invariant must hold");
            }
        };
        let primary_key_field_type_as_pg_type_primary_key_token_stream =
            quote::quote! {<#primary_key_field_type as #import_token_stream PgTypePrimaryKey>::};
        let generate_primary_key_field_type_as_pg_type_primary_key_method_call_token_stream =
            |method_token_stream: &dyn quote::ToTokens, ts0: &dyn quote::ToTokens| {
                quote::quote! {#primary_key_field_type_as_pg_type_primary_key_token_stream #method_token_stream(#ts0)}
            };
        let read_ids_element_primary_key_field_token_stream =
            quote::quote! {read_ids_element_937c5af3.#primary_key_field_identifier};
        let (
            primary_key_field_type_read_ids_into_read_element_43ab7fb5_primary_key_field_token_stream,
            primary_key_field_type_read_ids_into_read_read_ids_from_try_co_primary_key_field_token_stream,
            primary_key_field_type_read_only_is_into_read_read_ids_element_primary_key_field_token_stream,
            primary_key_field_type_read_ids_into_read_read_ids_from_co_primary_key_field_token_stream,
        ) = {
            let generate_token_stream = |ts: &dyn quote::ToTokens| {
                generate_primary_key_field_type_as_pg_type_primary_key_method_call_token_stream(
                    &ReadIdsIntoReadSnakeCase,
                    &ts,
                )
            };
            (
                generate_token_stream(
                    &quote::quote! {element_43ab7fb5.#primary_key_field_identifier},
                ),
                generate_token_stream(
                    &quote::quote! {read_ids_from_try_co.#primary_key_field_identifier},
                ),
                generate_token_stream(&read_ids_element_primary_key_field_token_stream),
                generate_token_stream(
                    &quote::quote! {read_ids_from_co.#primary_key_field_identifier},
                ),
            )
        };
        let primary_key_where_eq_iter_map_token_stream = {
            let ts = generate_primary_key_where_eq_into_inner_token_stream(&primary_key_field_type_read_ids_into_read_element_43ab7fb5_primary_key_field_token_stream);
            quote::quote! {.iter().map(|element_43ab7fb5| #ts)}
        };
        let primary_key_field_type_as_pg_type_update_as_pg_type_primary_key_read_ids_into_update_token_stream = {
            let method_call_token_stream =
                generate_primary_key_field_type_as_pg_type_primary_key_method_call_token_stream(
                    &ReadIdsIntoUpdateSnakeCase,
                    &read_ids_element_primary_key_field_token_stream,
                );
            quote::quote! {#primary_key_as_pg_type_token_stream::Update::from(#method_call_token_stream)}
        };
        let (
            field_read_ids_and_create_into_optional_v_read_read_ids_and_create_token_stream,
            field_read_ids_and_create_into_optional_v_read_read_ids_from_try_co_identifier_create_token_stream,
            field_read_ids_and_create_into_optional_v_read_read_ids_from_co_create_token_stream,
            field_read_ids_and_create_into_optional_v_read_read_ids_from_co_clone_identifier_create_clone_token_stream,
        ) = {
            #[derive(optimal_memory_layout::OptimalMemoryLayout)]
            enum AddDotClone {
                False,
                True,
            }
            let generate_token_stream =
                |read_ids_token_stream: &dyn quote::ToTokens,
                 create_token_stream: &dyn quote::ToTokens,
                 add_dot_clone: &AddDotClone| {
                    generate_fields_named_without_primary_key_with_comma_token_stream(
                        &|element: &macros_helpers::field_data::SynField| {
                            let field = &element.identifier;
                            let maybe_dot_clone_token_stream = match &add_dot_clone {
                                AddDotClone::False => proc_macro2::TokenStream::new(),
                                AddDotClone::True => quote::quote! {.clone()},
                            };
                            let field_type_token_stream =
                                generate_as_pg_type_test_cases_path_token_stream(&element.type0);
                            quote::quote! {
                                #field: #field_type_token_stream read_ids_and_create_into_optional_v_read(
                                    #read_ids_token_stream.#field #maybe_dot_clone_token_stream.expect("f967434c generate_assert_eq_token_stream invariant must hold"),
                                    #create_token_stream.#field #maybe_dot_clone_token_stream
                                )
                            }
                        },
                    )
                };
            let identifier_create_name_token_stream = quote::quote! {identifier_create};
            let read_ids_from_co_name_token_stream = quote::quote! {read_ids_from_co};
            (
                generate_token_stream(&ReadIdsSnakeCase, &CreateSnakeCase, &AddDotClone::False),
                generate_token_stream(
                    &quote::quote! {read_ids_from_try_co},
                    &identifier_create_name_token_stream,
                    &AddDotClone::False,
                ),
                generate_token_stream(
                    &read_ids_from_co_name_token_stream,
                    &quote::quote! {identifier_create_default},
                    &AddDotClone::False,
                ),
                generate_token_stream(
                    &read_ids_from_co_name_token_stream,
                    &identifier_create_name_token_stream,
                    &AddDotClone::True,
                ),
            )
        };
        let optional_identifier_where_fields_none_token_stream =
            generate_fields_named_without_primary_key_with_comma_token_stream(
                &|element: &macros_helpers::field_data::SynField| {
                    let field = &element.identifier;
                    quote::quote! {#field: None}
                },
            );
        let select_default_all_with_max_page_size_clone_token_stream =
            quote::quote! {select_default_all_with_max_page_size.clone()};
        let common_read_ids_from_co_token_stream = {
            let primary_key_read_token_stream = quote::quote! {primary_key_read};
            let primary_key_read_clone_token_stream = quote::quote! {primary_key_read.clone()};
            let ts = generate_v_initialization_ts0(&primary_key_read_clone_token_stream);
            let assert_eq_ro_primary_key_token_stream = generate_assert_eq_token_stream(
                &quote::quote! {
                    #identifier_read_upper_camel_case {
                        #primary_key_field_identifier: Some(#ts),
                        #fields_none_initialization_token_stream
                    }
                },
                &quote::quote! {
                    generate_identifier_try_ro_handle_primary_key(
                        &#UrlSnakeCase,
                        #primary_key_read_clone_token_stream,
                        #SelectPrimaryKeySnakeCase.clone(),
                        &table_initialization
                    )
                    .await
                    .expect("36b95e96 generate_assert_eq_token_stream invariant must hold")
                },
                &quote::quote! {"3d9f2ec0"},
            );
            let assert_eq_dlo_primary_key_token_stream = generate_assert_eq_token_stream(
                &quote::quote! {
                    generate_try_dlo_handle(
                        &url,
                        #primary_key_read_clone_token_stream,
                        &table_initialization,
                    ).await.expect("4d96d385 generate_assert_eq_token_stream invariant must hold")
                },
                &quote::quote! {#primary_key_read_clone_token_stream},
                &quote::quote! {"26e2058b"},
            );
            quote::quote! {
                let #CommonReadIdsFromCoSnakeCase = {
                    let read_ids_from_try_co = generate_read_ids_from_try_co_default(&#UrlSnakeCase, &table_initialization).await;
                    let primary_key_read = #primary_key_field_type_read_ids_into_read_read_ids_from_try_co_primary_key_field_token_stream;
                    #assert_eq_ro_primary_key_token_stream
                    #assert_eq_dlo_primary_key_token_stream
                    generate_check_no_rows_from_identifier_try_ro_handle_primary_key(
                        &url,
                        #primary_key_read_token_stream,
                        #select_default_all_with_max_page_size_clone_token_stream,
                        &table_initialization,
                    ).await;
                    read_ids_from_try_co
                };
            }
        };
        let generate_identifier_create_token_stream: &dyn Fn(
            &syn::Ident,
            &dyn quote::ToTokens,
        ) -> proc_macro2::TokenStream = &|field, ts| {
            generate_fields_named_without_primary_key_with_comma_token_stream(
                &|element: &macros_helpers::field_data::SynField| {
                    let fi0 = &element.identifier;
                    let ft0 = &element.type0;
                    let ts0 = if field == fi0.as_ref() {
                        quote::quote! {#ts}
                    } else {
                        let ts1 = generate_as_pg_type_path_token_stream(&ft0);
                        quote::quote! {<#ts1 Create as #import_token_stream DefaultSomeOneElement>::default_some_one_element()}
                    };
                    quote::quote! {#fi0: #ts0}
                },
            )
        };
        let generate_identifier_create_cnt_element_id_token_stream: &dyn Fn(
            &syn::Ident,
            &dyn quote::ToTokens,
        ) -> proc_macro2::TokenStream = &|field, element_token_stream| generate_identifier_create_token_stream(field, &element_token_stream);
        let generate_identifier_create_cnt_element_token_stream: &dyn Fn(&syn::Ident) -> proc_macro2::TokenStream =
            &|field| generate_identifier_create_token_stream(field, &ElementSnakeCase);
        let generate_table_test_name_field_token_stream: &dyn Fn(
            &str,
            &syn::Ident,
        )
            -> proc_macro2::TokenStream = &|test_name, field| {
            let table_test_name_field = quote::format_ident!("table_{test_name}_{field}");
            quote::quote! {#table_test_name_field}
        };
        let table_fields_test_name_count = 4usize;
        let mut table_fis_initialization_vec_token_stream = Vec::with_capacity(
            fields_len_without_primary_key.saturating_mul(table_fields_test_name_count),
        );
        let mut table_test_name_fis_vec_token_stream = Vec::with_capacity(
            fields_len_without_primary_key.saturating_mul(table_fields_test_name_count),
        );
        let fill_table_fis_vec_token_stream: &mut dyn FnMut(TableTestNames<'_>) =
            &mut |test_names| {
                test_names.0.into_iter().fold((), |(), el0| {
                let generate_initialization_variable_name_token_stream: &dyn Fn(&syn::Ident) -> proc_macro2::TokenStream =
                    &|field| {
                        let initialization_variable_name = quote::format_ident!("table_{el0}_{field}");
                        quote::quote! {#initialization_variable_name}
                    };
                table_fis_initialization_vec_token_stream.push(generate_fields_named_without_primary_key_without_comma_token_stream(
                    &|element: &macros_helpers::field_data::SynField| {
                        let field = &element.identifier;
                        let initialization_variable_name_token_stream = generate_initialization_variable_name_token_stream(field);
                        let format_token_stream = generate_quotes::dq_token_stream(&format!("{el0}_{field}"));
                        quote::quote! {
                            let #initialization_variable_name_token_stream = add_table_postfix(#format_token_stream);
                        }
                    },
                ));
                table_test_name_fis_vec_token_stream.push(generate_fields_named_without_primary_key_without_comma_token_stream(
                    &|el1: &macros_helpers::field_data::SynField| {
                        let field = &el1.identifier;
                        let initialization_variable_name_token_stream = generate_initialization_variable_name_token_stream(field);
                        quote::quote! {&#initialization_variable_name_token_stream,}
                    },
                ));
            });
            };
        let table_read_ids_and_create_into_where_eq_name = str_constants::VALUE_8E427AD7;
        let table_read_ids_and_create_into_vec_where_eq_using_fields_name = str_constants::EB24448C;
        let table_read_ids_and_create_into_optional_vec_where_eq_to_field_name =
            str_constants::VALUE_9AC6D79A;
        let table_read_ids_and_table_type_into_pg_type_optional_where_greater_than_name =
            str_constants::VALUE_5A52AF33;
        fill_table_fis_vec_token_stream(
            vec![
                table_read_ids_and_create_into_where_eq_name,
                table_read_ids_and_create_into_vec_where_eq_using_fields_name,
                table_read_ids_and_create_into_optional_vec_where_eq_to_field_name,
                table_read_ids_and_table_type_into_pg_type_optional_where_greater_than_name,
            ]
            .into(),
        );
        let select_default_all_with_max_page_size_cloned_clone_token_stream =
            quote::quote! {select_default_all_with_max_page_size_cloned.clone()};
        let read_ids_to_2_dimensions_vec_read_inner_accumulator_fields_token_stream =
            generate_fields_named_without_primary_key_without_comma_token_stream(
                &|element: &macros_helpers::field_data::SynField| {
                    let field = &element.identifier;
                    let field_read_ids_to_2_dimensions_vec_read_inner_accumulator_snake_case =
                        naming::parameter::SelfReadIdsTo2DimensionsVecReadInnerAccumulatorSnakeCase::from_tokens(&field);
                    let identifier_create_dflts_for_column_read_ids_to_2_dimensions_vec_read_inner_token_stream =
                        generate_fields_named_without_primary_key_without_comma_token_stream(
                            &|el0: &macros_helpers::field_data::SynField| {
                                let fi0 = &el0.identifier;
                                let ft0 = &el0.type0;
                                if field == fi0 {
                                    generate_if_let_some_token_stream(
                                        &quote::quote! {v_a5f7e6cd},
                                        &quote::quote! {&common_read_ids_from_co.#fi0},
                                        &{
                                            let field_type_token_stream =
                                                generate_as_pg_type_test_cases_path_token_stream(
                                                    &ft0,
                                                );
                                            quote::quote! {
                                                let read_ids_to_2_dimensions_vec_read_inner_8ef7d00b = #field_type_token_stream read_ids_to_2_dimensions_vec_read_inner(v_a5f7e6cd);
                                                accumulator_458cda9e.reserve(
                                                    read_ids_to_2_dimensions_vec_read_inner_8ef7d00b
                                                        .iter()
                                                        .map(Vec::len)
                                                        .sum::<usize>()
                                                );
                                                for element_b3522b7d in read_ids_to_2_dimensions_vec_read_inner_8ef7d00b {
                                                    for _ in element_b3522b7d {
                                                        accumulator_458cda9e.push(identifier_create_default.clone());
                                                    }
                                                }
                                            }
                                        },
                                    )
                                } else {
                                    proc_macro2::TokenStream::new()
                                }
                            },
                        );
                    quote::quote! {
                        let #field_read_ids_to_2_dimensions_vec_read_inner_accumulator_snake_case = {
                            let mut accumulator_458cda9e = Vec::new();
                            #identifier_create_dflts_for_column_read_ids_to_2_dimensions_vec_read_inner_token_stream
                            accumulator_458cda9e
                        };
                    }
                },
            );
        let generate_where_primary_key_or_token_stream =
            |vec_token_stream: &dyn quote::ToTokens| {
                quote::quote! {
                    generate_identifier_where_primary_key_others_none(
                        Some(
                            generate_pg_type_where_try_new_primary_key(
                                #operator_or_token_stream,
                                #vec_token_stream
                            )
                        )
                    )
                }
            };
        let primary_key_sort_cmp_token_stream = quote::quote! {
            |first, second| match (&first.#primary_key_field_identifier, &second.#primary_key_field_identifier) {
                (Some(first_handle), Some(second_handle)) => first_handle.#VSnakeCase.cmp(&second_handle.#VSnakeCase),
                _ => panic!("0f1d45ed"),
            }
        };
        let generate_read_ids_els_token_stream = {
            let identifier_read_fields_initialization_without_primary_key_token_stream =
                generate_fields_named_without_primary_key_with_comma_token_stream(
                    &|syn_field: &macros_helpers::field_data::SynField| {
                        let field = &syn_field.identifier;
                        let ts =
                            generate_v_initialization_ts0(&PgCrudCommonDefaultSomeOneElementCall);
                        let ts0 =
                            generate_as_pg_type_test_cases_path_token_stream(&syn_field.type0);
                        quote::quote! {
                            #field: element_f108da5a.#field.as_ref().map_or_else(
                                || Some(#ts),
                                #ts0 read_ids_to_optional_v_read_default_some_one_element
                            )
                        }
                    },
                );
            let where_primary_key_or_read_ids_els_token_stream =
                generate_where_primary_key_or_token_stream(
                    &quote::quote! {read_ids_els_efeed554 #primary_key_where_eq_iter_map_token_stream},
                );
            let assert_eq_read_ids_els_token_stream = generate_assert_eq_token_stream(
                &quote::quote! {
                    itertools::Itertools::sorted_by(
                        read_ids_els_efeed554.iter().map(|element_f108da5a| {
                            #identifier_read_upper_camel_case {
                                #primary_key_field_identifier: #primary_key_as_pg_type_test_cases_path_token_stream read_ids_to_optional_v_read_default_some_one_element(
                                    &element_f108da5a.#primary_key_field_identifier
                                ),
                                #identifier_read_fields_initialization_without_primary_key_token_stream
                            }
                        }),
                        #primary_key_sort_cmp_token_stream
                    ).collect::<Vec<#identifier_read_upper_camel_case>>()
                },
                &quote::quote! {
                    itertools::Itertools::sorted_by(
                        generate_try_rm_order_by_primary_key_with_big_pagination(
                            url,
                            #where_primary_key_or_read_ids_els_token_stream,
                            #select_default_all_with_max_page_size_clone_token_stream,
                            table_9c259e1c
                        )
                        .await
                        .expect("097d5e7d generate_assert_eq_token_stream invariant must hold")
                        .into_iter(),
                        #primary_key_sort_cmp_token_stream
                    )
                    .collect::<Vec<#identifier_read_upper_camel_case>>()
                },
                &quote::quote! {"50198a7f"},
            );
            quote::quote! {
                async fn generate_read_ids_els_8a1ef027(
                    url: &str,
                    table_9c259e1c: &str,
                    select_default_all_with_max_page_size: #import_token_stream NotEmptyUniqueVec<#identifier_select_upper_camel_case>,
                    read_ids_to_2_dimensions_vec_read_inner_accumulator: Vec<#identifier_create_upper_camel_case>
                ) -> Vec<#identifier_read_ids_upper_camel_case> {
                    const CM_CHUNK_SIZE_2EE9377B: usize = 25;
                    const CM_CONCURRENCY_7CCFD82D: usize = 5;
                    let read_ids_to_2_dimensions_vec_read_inner_accumulator_len = read_ids_to_2_dimensions_vec_read_inner_accumulator.len();
                    let read_ids_els_efeed554 = futures::StreamExt::fold(
                        futures::StreamExt::buffer_unordered(
                            futures::stream::iter(
                                itertools::Itertools::chunks(
                                    read_ids_to_2_dimensions_vec_read_inner_accumulator.into_iter(),
                                    CM_CHUNK_SIZE_2EE9377B,
                                )
                                .into_iter()
                                .map(|element_6f515764| element_6f515764.collect::<Vec<#identifier_create_upper_camel_case>>())
                                .map(|element_8e425cb1| async move { #identifier::try_cm_handle(
                                    url,
                                    #identifier_cm_parameters_upper_camel_case {
                                        payload: #identifier_cm_payload_upper_camel_case(element_8e425cb1)
                                    },
                                    table_9c259e1c
                                ).await.expect("38a24e7a generate_read_ids_els_8a1ef027 invariant must hold") })
                            ),
                            CM_CONCURRENCY_7CCFD82D
                        ),
                        Vec::with_capacity(read_ids_to_2_dimensions_vec_read_inner_accumulator_len),
                        |mut accumulator_a33fb452, read_ids_78f10a3d| async move {
                            accumulator_a33fb452.extend(read_ids_78f10a3d);
                            accumulator_a33fb452
                        }
                    )
                    .await;
                    #assert_eq_read_ids_els_token_stream
                    read_ids_els_efeed554
                }
            }
        };
        let generate_field_type_optional_vec_create_token_stream: &dyn Fn(&syn::Type) -> proc_macro2::TokenStream = &|field_type| {
            let ts = generate_as_pg_type_test_cases_path_token_stream(field_type);
            quote::quote! {#ts #OptionalVecCreateSnakeCase()}
        };
        let generate_field_type_optional_vec_create_or_vec_token_stream: &dyn Fn(&syn::Type) -> proc_macro2::TokenStream = &|field_type| {
            let ts = generate_field_type_optional_vec_create_token_stream(field_type);
            quote::quote! {#ts.unwrap_or(Vec::new())}
        };
        let generate_identifier_field_type_optional_vec_create_or_vec_token_stream: &dyn Fn(
            &syn::Ident,
            &syn::Type,
        ) -> proc_macro2::TokenStream = &|_, field_type| generate_field_type_optional_vec_create_or_vec_token_stream(field_type);
        let generate_for_in_1_2_token_stream =
            |element_token_stream: &dyn quote::ToTokens, ts: &dyn quote::ToTokens| {
                quote::quote! {
                    for #element_token_stream in [1,2] {
                        #ts
                    }
                }
            };
        let generate_vec_primary_key_sorted_read_token_stream = |ts: &dyn quote::ToTokens| {
            quote::quote! {itertools::Itertools::sorted(#ts).collect::<Vec<#primary_key_field_type_as_pg_type_read_token_stream>>()}
        };
        let vec_primary_key_sorted_read_token_stream =
            generate_vec_primary_key_sorted_read_token_stream(&quote::quote! {
                read_ids_from_try_cm
                .into_iter()
                .map(|element_43ab7fb5| {
                    #primary_key_field_type_read_ids_into_read_element_43ab7fb5_primary_key_field_token_stream
                })
            });
        let generate_try_dm_handle_token_stream =
            |ts: &dyn quote::ToTokens, table_token_stream: &dyn quote::ToTokens| {
                quote::quote! {
                    #identifier::try_dm_handle(
                        &url_cloned,
                        #identifier_dm_parameters_upper_camel_case {
                            //todo rewrite it using new\try_new?
                            payload: #identifier_dm_payload_upper_camel_case {
                                where_many: #optional_identifier_where_upper_camel_case(Some(
                                    #identifier_where_upper_camel_case {
                                        #ts
                                    }
                                ))
                            }
                        },
                        &#table_token_stream
                    )
                    .await
                    .expect("716e470e generate_read_ids_els_8a1ef027 invariant must hold")
                }
            };
        let generate_read_ids_from_try_dm_token_stream = |ts: &dyn quote::ToTokens| {
            quote::quote! {
                let read_ids_from_try_dm = #ts;
            }
        };
        let generate_read_ids_from_try_dm_sorted_primary_key_token_stream =
            |table_token_stream: &dyn quote::ToTokens, some_token_stream: &dyn quote::ToTokens| {
                generate_read_ids_from_try_dm_token_stream(
                    &generate_vec_primary_key_sorted_read_token_stream(&{
                        let ts = generate_try_dm_handle_token_stream(
                            &quote::quote! {
                                #primary_key_field_identifier: Some(#some_token_stream),
                                #optional_identifier_where_fields_none_token_stream
                            },
                            &table_token_stream,
                        );
                        quote::quote! {#ts.into_iter()}
                    }),
                )
            };
        let generate_accumulator_push_future_token_stream =
            |ts0: &dyn quote::ToTokens, ts1: &dyn quote::ToTokens, ts2: &dyn quote::ToTokens| {
                quote::quote! {
                    let #ts0 = #ts1.clone();
                    let url_cloned = url.clone();
                    let select_default_all_with_max_page_size_cloned = #select_default_all_with_max_page_size_clone_token_stream;
                    accumulator_9189f86e.push(futures::FutureExt::boxed(async move {
                        #ts2
                    }));
                }
            };
        let vec_read_from_read_ids_with_create_token_stream = quote::quote! {
            generate_vec_identifier_read_from_vec_identifier_read_ids_with_vec_identifier_create(
                read_ids_from_try_cm.clone(),
                identifier_vec_create.clone()
            )
        };
        let cm_tests_token_stream = {
            let cm_tests_token_stream =
                generate_fields_named_without_primary_key_without_comma_token_stream(
                    &|element: &macros_helpers::field_data::SynField| {
                        let field = &element.identifier;
                        let field_type = &element.type0;
                        let cm_identifier_create_cnt_element_id_token_stream =
                            generate_identifier_create_cnt_element_id_token_stream(
                                field,
                                &quote::quote! {element_03a4f4ee},
                            );
                        let field_type_optional_vec_create_or_vec_token_stream =
                            generate_field_type_optional_vec_create_or_vec_token_stream(field_type);
                        let where_primary_key_or_read_ids_cm_token_stream =
                            generate_where_primary_key_or_token_stream(
                                &quote::quote! {read_ids_from_try_cm #primary_key_where_eq_iter_map_token_stream},
                            );
                        let assert_eq_cm_rm_token_stream = generate_assert_eq_token_stream(
                            &vec_read_from_read_ids_with_create_token_stream,
                            &quote::quote! {
                                generate_try_rm_order_by_primary_key_with_big_pagination(
                                    &url_cloned,
                                    #where_primary_key_or_read_ids_cm_token_stream,
                                    #select_default_all_with_max_page_size_cloned_clone_token_stream,
                                    &table_cm_cloned
                                ).await.expect("bdb72341 generate_read_ids_els_8a1ef027 invariant must hold")
                            },
                            &quote::quote! {"d19bbbf6"},
                        );
                        let assert_eq_cm_dm_pks_token_stream = generate_assert_eq_token_stream(
                            &quote::quote! {read_ids_from_try_dm},
                            &vec_primary_key_sorted_read_token_stream,
                            &quote::quote! {"f58f5572"},
                        );
                        let assert_cm_dm_empty_token_stream = generate_assert_token_stream(
                            &{
                                let ts = generate_primary_key_where_eq_into_inner_token_stream(
                                    &quote::quote! {element_a37bca54.clone()},
                                );
                                let where_primary_key_or_dm_token_stream =
                                    generate_where_primary_key_or_token_stream(&quote::quote! {
                                        {
                                            let mut accumulator_87ea12c9 = Vec::with_capacity(read_ids_from_try_dm.len());
                                            for element_a37bca54 in &read_ids_from_try_dm {
                                                accumulator_87ea12c9.push(#ts);
                                            }
                                            accumulator_87ea12c9
                                        }
                                    });
                                quote::quote! {
                                    generate_try_rm_order_by_primary_key_with_big_pagination(
                                        &url_cloned,
                                        #where_primary_key_or_dm_token_stream,
                                        #select_default_all_with_max_page_size_cloned_clone_token_stream,
                                        &table_cm_cloned
                                    ).await
                                    .expect("24ab86d6 generate_read_ids_els_8a1ef027 invariant must hold")
                                    .is_empty()
                                }
                            },
                            &quote::quote! {"4e88679a"},
                        );
                        let cm_read_ids_from_try_dm_sorted_primary_key_token_stream =
                            generate_read_ids_from_try_dm_sorted_primary_key_token_stream(
                                &quote::quote! {table_cm_cloned},
                                &quote::quote! {
                                    generate_pg_type_where_try_new_or_pks(&read_ids_from_try_cm)
                                },
                            );
                        let cm_accumulator_push_future_token_stream =
                            generate_accumulator_push_future_token_stream(
                                &quote::quote! {table_cm_cloned},
                                &quote::quote! {table_cm},
                                &quote::quote! {
                                    let identifier_vec_create = {
                                        let mut accumulator_92d248f7 = Vec::with_capacity(element_fce0969c.len());
                                        for element_03a4f4ee in element_fce0969c {
                                            accumulator_92d248f7.push(#identifier_create_upper_camel_case {
                                                #cm_identifier_create_cnt_element_id_token_stream
                                            });
                                        }
                                        accumulator_92d248f7
                                    };
                                    let read_ids_from_try_cm = #identifier::try_cm_handle(
                                        &url_cloned,
                                        #identifier_cm_parameters_upper_camel_case {
                                            #PayloadSnakeCase: #identifier_cm_payload_upper_camel_case(identifier_vec_create.clone())
                                        },
                                        &table_cm_cloned
                                    ).await.expect("5eecedc4 generate_read_ids_els_8a1ef027 invariant must hold");
                                    #assert_eq_cm_rm_token_stream
                                    #cm_read_ids_from_try_dm_sorted_primary_key_token_stream
                                    #assert_eq_cm_dm_pks_token_stream
                                    #assert_cm_dm_empty_token_stream
                                },
                            );
                        quote::quote! {
                            const CM_CHUNK_SIZE_A13F7C92: usize = 10;
                            for element_fce0969c in #field_type_optional_vec_create_or_vec_token_stream.chunks(CM_CHUNK_SIZE_A13F7C92) {
                                #cm_accumulator_push_future_token_stream
                            }
                        }
                    },
                );
            quote::quote! {#cm_tests_token_stream}
        };
        let co_tests_token_stream = {
            let co_tests_token_stream =
                generate_fields_named_without_primary_key_without_comma_token_stream(
                    &|element: &macros_helpers::field_data::SynField| {
                        let field = &element.identifier;
                        let field_type = &element.type0;
                        let co_identifier_create_cnt_element_id_token_stream =
                            generate_identifier_create_cnt_element_id_token_stream(
                                field,
                                &quote::quote! {element_7632d698},
                            );
                        let ts = generate_v_initialization_ts0(&primary_key_field_type_read_ids_into_read_read_ids_from_try_co_primary_key_field_token_stream);
                        let field_type_optional_vec_create_or_vec_token_stream =
                            generate_field_type_optional_vec_create_or_vec_token_stream(field_type);
                        let assert_eq_co_ro_primary_key_token_stream =
                            generate_assert_eq_token_stream(
                                &quote::quote! {
                                    #identifier_read_upper_camel_case {
                                        #primary_key_field_identifier: Some(#ts),
                                        #field_read_ids_and_create_into_optional_v_read_read_ids_from_try_co_identifier_create_token_stream
                                    }
                                },
                                &quote::quote! {
                                    generate_identifier_try_ro_handle_primary_key(
                                        &url_cloned,
                                        #primary_key_field_type_read_ids_into_read_read_ids_from_try_co_primary_key_field_token_stream,
                                        #select_default_all_with_max_page_size_cloned_clone_token_stream,
                                        &table_co_cloned
                                    )
                                    .await
                                    .expect("f8e1cb88 generate_read_ids_els_8a1ef027 invariant must hold")
                                },
                                &quote::quote! {"5f2adbed"},
                            );
                        let assert_eq_co_dlo_primary_key_token_stream =
                            generate_assert_eq_token_stream(
                                &quote::quote! {
                                    generate_try_dlo_handle(
                                        &url_cloned,
                                        #primary_key_field_type_read_ids_into_read_read_ids_from_try_co_primary_key_field_token_stream,
                                        &table_co_cloned
                                    ).await.expect("20d5a40a generate_read_ids_els_8a1ef027 invariant must hold")
                                },
                                &quote::quote! {#primary_key_field_type_read_ids_into_read_read_ids_from_try_co_primary_key_field_token_stream},
                                &quote::quote! {"4f563faf"},
                            );
                        let co_accumulator_push_future_token_stream =
                            generate_accumulator_push_future_token_stream(
                                &quote::quote! {table_co_cloned},
                                &quote::quote! {table_co},
                                &quote::quote! {
                                    let identifier_create = #identifier_create_upper_camel_case {
                                        #co_identifier_create_cnt_element_id_token_stream
                                    };
                                    let read_ids_from_try_co = generate_read_ids_from_try_co(
                                        &url_cloned,
                                        identifier_create.clone(),
                                        &table_co_cloned
                                    ).await;
                                    #assert_eq_co_ro_primary_key_token_stream
                                    #assert_eq_co_dlo_primary_key_token_stream
                                    generate_check_no_rows_from_identifier_try_ro_handle_primary_key(
                                        &url_cloned,
                                        #primary_key_field_type_read_ids_into_read_read_ids_from_try_co_primary_key_field_token_stream,
                                        select_default_all_with_max_page_size_cloned,
                                        &table_co_cloned,
                                    ).await;
                                },
                            );
                        quote::quote! {
                            for element_7632d698 in #field_type_optional_vec_create_or_vec_token_stream {
                                #co_accumulator_push_future_token_stream
                            }
                        }
                    },
                );
            quote::quote! {#co_tests_token_stream}
        };
        let add_co_default_and_delete_after_just_to_add_some_data_to_be_sure_it_will_not_return_from_the_test_query_token_stream =
            |ts: &dyn quote::ToTokens| {
                quote::quote! {
                    let read_ids_from_try_co = generate_read_ids_from_try_co_default(
                        &url_cloned,
                        &table_7e35b1ce
                    ).await;
                    #ts
                    let _: #primary_key_field_type_as_pg_type_read_token_stream = generate_try_dlo_handle(
                        &url_cloned,
                        #primary_key_field_type_read_ids_into_read_read_ids_from_try_co_primary_key_field_token_stream,
                        &table_7e35b1ce
                    ).await.expect("93b4bf61 generate_read_ids_els_8a1ef027 invariant must hold");
                    generate_check_no_rows_from_identifier_try_ro_handle_primary_key(
                        &url_cloned,
                        #primary_key_field_type_read_ids_into_read_read_ids_from_try_co_primary_key_field_token_stream,
                        select_default_all_with_max_page_size_cloned,
                        &table_7e35b1ce,
                    ).await;
                }
            };
        let rm_tests_token_stream = {
            //todo extra rm checks
            let where_primary_key_or_repeat_uuid_token_stream =
                generate_where_primary_key_or_token_stream(&quote::quote! {
                    std::iter::repeat_with(|| #primary_key_where_eq_uuid_new_v_token_stream)
                    .take(element_30614c66)
                });
            let test_rm_by_non_existent_pks_token_stream = generate_for_in_1_2_token_stream(
                &quote::quote! {element_30614c66},
                &generate_accumulator_push_future_token_stream(
                    &quote::quote! {table_7e35b1ce},
                    &quote::quote! {table_test_rm_by_non_existent_pks},
                    &add_co_default_and_delete_after_just_to_add_some_data_to_be_sure_it_will_not_return_from_the_test_query_token_stream(&generate_assert_token_stream(
                        &quote::quote! {
                            generate_try_rm_order_by_primary_key_with_big_pagination(
                                &url_cloned,
                                #where_primary_key_or_repeat_uuid_token_stream,
                                select_default_all_with_max_page_size_cloned.clone(),
                                &table_7e35b1ce
                            ).await
                            .expect("e661c49b generate_read_ids_els_8a1ef027 invariant must hold")
                            .is_empty()
                        },
                        &quote::quote! {"06df4025"}
                    ))
                )
            );
            let test_rm_by_eq_to_created_pks_token_stream = generate_for_in_1_2_token_stream(
                &quote::quote! {element_a636d084},
                &{
                    let ts = generate_accumulator_push_future_token_stream(
                        &quote::quote! {table_7e35b1ce},
                        &quote::quote! {table_test_rm_by_eq_to_created_pks},
                        &add_co_default_and_delete_after_just_to_add_some_data_to_be_sure_it_will_not_return_from_the_test_query_token_stream(&{
                            let where_primary_key_or_read_ids_cm_token_stream = generate_where_primary_key_or_token_stream(&quote::quote! {read_ids_from_try_cm #primary_key_where_eq_iter_map_token_stream});
                            let assert_eq_rm_created_pks_token_stream = generate_assert_eq_token_stream(
                                &vec_read_from_read_ids_with_create_token_stream,
                                &quote::quote! {
                                    generate_try_rm_order_by_primary_key_with_big_pagination(
                                        &url_cloned,
                                        #where_primary_key_or_read_ids_cm_token_stream,
                                        select_default_all_with_max_page_size_cloned.clone(),
                                        &table_7e35b1ce
                                    ).await.expect("b8efe770 generate_read_ids_els_8a1ef027 invariant must hold")
                                },
                                &quote::quote! {"error 3b2cf1f5-2c4e-4908-ba66-f4af84fe0893"},
                            );
                            let assert_eq_rm_dm_pks_token_stream = generate_assert_eq_token_stream(
                                &quote::quote! {read_ids_from_try_dm},
                                &vec_primary_key_sorted_read_token_stream,
                                &quote::quote! {"ebbbea6e"},
                            );
                            let assert_rm_dm_empty_token_stream = generate_assert_token_stream(
                                &{
                                    let ts = generate_primary_key_where_eq_into_inner_token_stream(&quote::quote! {element_1e9c87ce.clone()});
                                    let where_primary_key_or_dm_token_stream = generate_where_primary_key_or_token_stream(&quote::quote! {
                                        read_ids_from_try_dm
                                        .iter()
                                        .map(|element_1e9c87ce| #ts)
                                    });
                                    quote::quote! {
                                        generate_try_rm_order_by_primary_key_with_big_pagination(
                                            &url_cloned,
                                            #where_primary_key_or_dm_token_stream,
                                            select_default_all_with_max_page_size_cloned.clone(),
                                            &table_7e35b1ce
                                        ).await
                                        .expect("1f079962 generate_read_ids_els_8a1ef027 invariant must hold")
                                        .is_empty()
                                    }
                                },
                                &quote::quote! {"d79c0af3"}
                            );
                            let rm_read_ids_from_try_dm_sorted_primary_key_token_stream = generate_read_ids_from_try_dm_sorted_primary_key_token_stream(
                                &quote::quote! {table_7e35b1ce},
                                &quote::quote! {
                                    generate_pg_type_where_try_new_or_pks(&read_ids_from_try_cm)
                                }
                            );
                            quote::quote! {
                                let identifier_vec_create = std::iter::repeat_n(
                                    identifier_create_default_cloned.clone(),
                                    element_a636d084
                                ).collect::<Vec<#identifier_create_upper_camel_case>>();
                                let read_ids_from_try_cm = #identifier::try_cm_handle(
                                    &url_cloned,
                                    #identifier_cm_parameters_upper_camel_case {
                                        payload: #identifier_cm_payload_upper_camel_case(identifier_vec_create.clone())
                                    },
                                    &table_7e35b1ce
                                ).await.expect("d775179f generate_read_ids_els_8a1ef027 invariant must hold");
                                #assert_eq_rm_created_pks_token_stream
                                #rm_read_ids_from_try_dm_sorted_primary_key_token_stream
                                #assert_eq_rm_dm_pks_token_stream
                                #assert_rm_dm_empty_token_stream
                            }
                        })
                    );
                    quote::quote! {
                        let identifier_create_default_cloned = identifier_create_default.clone();
                        #ts
                    }
                },
            );
            let generate_read_ids_and_create_into_where_assert_eq_token_stream =
                |ts: &dyn quote::ToTokens| {
                    generate_assert_eq_token_stream(
                        &quote::quote! {vec![
                            #identifier_read_upper_camel_case {
                                #primary_key_field_identifier: #primary_key_as_pg_type_test_cases_path_token_stream read_ids_to_optional_v_read_default_some_one_element(
                                    &read_ids_from_co.#primary_key_field_identifier
                                ),
                                #field_read_ids_and_create_into_optional_v_read_read_ids_from_co_clone_identifier_create_clone_token_stream
                            }
                        ]},
                        &quote::quote! {
                            generate_try_rm_order_by_primary_key_with_big_pagination(
                                &url_cloned,
                                #identifier_where_upper_camel_case::try_new(#ts).expect("83c2d430 generate_read_ids_els_8a1ef027 invariant must hold"),
                                #select_default_all_with_max_page_size_cloned_clone_token_stream,
                                &table_7e35b1ce
                            ).await.expect("c3e316c0 generate_read_ids_els_8a1ef027 invariant must hold")
                        },
                        &quote::quote! {"ee8d232d"},
                    )
                };
            let generate_read_test_token_stream: &dyn Fn(
                &str,
                &dyn Fn(&syn::Ident, &syn::Type) -> proc_macro2::TokenStream,
                &dyn Fn(&syn::Ident) -> proc_macro2::TokenStream,
                &dyn Fn(&macros_helpers::field_data::SynField) -> proc_macro2::TokenStream,
            ) -> proc_macro2::TokenStream =
                &|test_name,
                  generate_method_call_token_stream,
                  generate_create_token_stream,
                  generate_token_stream| {
                    generate_fields_named_without_primary_key_without_comma_token_stream(
                        &|element: &macros_helpers::field_data::SynField| {
                            let field = &element.identifier;
                            let field_type = &element.type0;
                            let method_call_token_stream =
                                generate_method_call_token_stream(field, field_type);
                            let table_test_name_field_token_stream =
                                generate_table_test_name_field_token_stream(test_name, field);
                            let where_identifier_create_token_stream =
                                generate_create_token_stream(field);
                            let ts = generate_token_stream(element);
                            let assert_eq_where_dm_pks_token_stream =
                                generate_assert_eq_token_stream(
                                    &quote::quote! {read_ids_from_try_dm},
                                    &quote::quote! {vec![#primary_key_field_type_read_ids_into_read_read_ids_from_co_primary_key_field_token_stream]},
                                    &quote::quote! {"9fc29fa5"},
                                );
                            let assert_where_dm_empty_token_stream = generate_assert_token_stream(
                                &{
                                    let where_primary_key_where_eq_into_inner_token_stream = generate_primary_key_where_eq_into_inner_token_stream(
                                    &primary_key_field_type_read_ids_into_read_read_ids_from_co_primary_key_field_token_stream,
                                );
                                    let where_primary_key_or_co_token_stream =
                                        generate_where_primary_key_or_token_stream(
                                            &quote::quote! {vec![#where_primary_key_where_eq_into_inner_token_stream]},
                                        );
                                    quote::quote! {
                                        generate_try_rm_order_by_primary_key_with_big_pagination(
                                            &url_cloned,
                                            #where_primary_key_or_co_token_stream,
                                            #select_default_all_with_max_page_size_cloned_clone_token_stream,
                                            &table_7e35b1ce
                                        ).await
                                        .expect("1817b67a generate_read_ids_els_8a1ef027 invariant must hold")
                                        .is_empty()
                                    }
                                },
                                &quote::quote! {"38187925"},
                            );
                            let where_primary_key_where_eq_co_token_stream =
                            generate_primary_key_where_eq_into_inner_token_stream(&primary_key_field_type_read_ids_into_read_read_ids_from_co_primary_key_field_token_stream);
                            let where_read_ids_from_try_dm_sorted_primary_key_token_stream =
                                generate_read_ids_from_try_dm_sorted_primary_key_token_stream(
                                    &quote::quote! {table_7e35b1ce},
                                    &quote::quote! {
                                        generate_pg_type_where_try_new_primary_key(
                                            #operator_or_token_stream,
                                            vec![#where_primary_key_where_eq_co_token_stream]
                                        )
                                    },
                                );
                            let where_accumulator_push_future_token_stream =
                                generate_accumulator_push_future_token_stream(
                                    &quote::quote! {table_7e35b1ce},
                                    &table_test_name_field_token_stream,
                                    &quote::quote! {
                                        let identifier_create = #identifier_create_upper_camel_case {
                                            #where_identifier_create_token_stream
                                        };
                                        let read_ids_from_co = generate_read_ids_from_try_co(
                                            &url_cloned,
                                            identifier_create.clone(),
                                            &table_7e35b1ce
                                        ).await;
                                        #ts
                                        #where_read_ids_from_try_dm_sorted_primary_key_token_stream
                                        #assert_eq_where_dm_pks_token_stream
                                        #assert_where_dm_empty_token_stream
                                    },
                                );
                            quote::quote! {
                                for #ElementSnakeCase in #method_call_token_stream {
                                    #where_accumulator_push_future_token_stream
                                }
                            }
                        },
                    )
                };
            let some_primary_key_where_initialization_token_stream = quote::quote! {
                Some(
                    generate_pg_type_where_try_new_primary_key(
                        #operator_and_token_stream,
                        vec![
                            #primary_key_as_pg_type_test_cases_path_token_stream read_ids_and_create_into_where_eq(
                                read_ids_from_co.#primary_key_field_identifier,
                                #PgCrudCommonDefaultSomeOneElementCall
                            )
                        ]
                    )
                )
            };
            let generate_field_where_token_stream: &dyn Fn(
                &syn::Ident,
                &dyn quote::ToTokens,
            )
                -> proc_macro2::TokenStream = &|field, ts| {
                generate_fields_named_with_comma_token_stream(
                    &|el0: &macros_helpers::field_data::SynField| {
                        let fi0 = &el0.identifier;
                        if primary_key_field_identifier == fi0 {
                            some_primary_key_where_initialization_token_stream.clone()
                        } else if fi0.as_ref() == field {
                            generate_some_pg_type_where_try_new_and_token_stream(&ts)
                        } else {
                            none_token_stream.clone()
                        }
                    },
                )
            };
            let generate_for_each_assert_eq_token_stream: &dyn Fn(
                &dyn quote::ToTokens,
                &dyn quote::ToTokens,
                &syn::Ident,
            )
                -> proc_macro2::TokenStream = &|v_token_stream, element_token_stream, field| {
                let vec_element_token_stream = quote::quote! {vec![#element_token_stream]};
                let assert_eq_token_stream =
                    generate_read_ids_and_create_into_where_assert_eq_token_stream(
                        &generate_field_where_token_stream(field, &vec_element_token_stream),
                    );
                quote::quote! {
                    for #element_token_stream in #v_token_stream.into_vec() {
                        #assert_eq_token_stream
                    }
                }
            };
            let (
                read_ids_and_create_into_where_eq_token_stream,
                read_ids_and_create_into_vec_where_eq_using_fields_token_stream,
            ) = {
                let generate_token_stream =
                    |test_name, eq_or_eq_using_fields: &pg_crud_macros_common::EqOrEqUsingFields| {
                        generate_read_test_token_stream(
                            test_name,
                            &generate_identifier_field_type_optional_vec_create_or_vec_token_stream,
                            &generate_identifier_create_cnt_element_token_stream,
                            &|element: &macros_helpers::field_data::SynField| {
                                let field = &element.identifier;
                                generate_read_ids_and_create_into_where_assert_eq_token_stream(
                                    &generate_fields_named_with_comma_token_stream(
                                        &|el0: &macros_helpers::field_data::SynField| {
                                            let fi0 = &el0.identifier;
                                            let ft0 = &el0.type0;
                                            if fi0 == primary_key_field_identifier {
                                                some_primary_key_where_initialization_token_stream.clone()
                                            } else if fi0 == field {
                                                let method_token_stream = {
                                                    let method_token_stream: &dyn quote::ToTokens =
                                                match &eq_or_eq_using_fields {
                                                    pg_crud_macros_common::EqOrEqUsingFields::Eq => &ReadIdsAndCreateIntoWhereEqSnakeCase,
                                                    pg_crud_macros_common::EqOrEqUsingFields::EqUsingFields => {
                                                        &ReadIdsAndCreateIntoVecWhereEqUsingFieldsSnakeCase
                                                    }
                                                };
                                                    let ts0 =
                                                        generate_as_pg_type_test_cases_path_token_stream(&ft0);
                                                    quote::quote! {
                                                        #ts0 #method_token_stream(
                                                            read_ids_from_co.#fi0.clone().expect("11c3740b generate_read_ids_els_8a1ef027 invariant must hold"),
                                                            identifier_create.#fi0.clone()
                                                        )
                                                    }
                                                };
                                                match &eq_or_eq_using_fields {
                                            pg_crud_macros_common::EqOrEqUsingFields::Eq => {
                                                generate_some_pg_type_where_try_new_and_token_stream(&quote::quote! {
                                                    vec![#method_token_stream]
                                                })
                                            }
                                            pg_crud_macros_common::EqOrEqUsingFields::EqUsingFields => {
                                                quote::quote! {
                                                    Some(#import_token_stream PgTypeWhere::new(
                                                        #operator_and_token_stream,
                                                        #method_token_stream
                                                    ))
                                                }
                                            }
                                        }
                                            } else {
                                                none_token_stream.clone()
                                            }
                                        },
                                    ),
                                )
                            },
                        )
                    };
                (
                    generate_token_stream(
                        table_read_ids_and_create_into_where_eq_name,
                        &pg_crud_macros_common::EqOrEqUsingFields::Eq,
                    ),
                    generate_token_stream(
                        table_read_ids_and_create_into_vec_where_eq_using_fields_name,
                        &pg_crud_macros_common::EqOrEqUsingFields::EqUsingFields,
                    ),
                )
            };
            let read_ids_and_create_into_optional_vec_where_eq_to_field_token_stream =
                generate_read_test_token_stream(
                    table_read_ids_and_create_into_optional_vec_where_eq_to_field_name,
                    &generate_identifier_field_type_optional_vec_create_or_vec_token_stream,
                    &generate_identifier_create_cnt_element_token_stream,
                    &|element: &macros_helpers::field_data::SynField| {
                        let field = &element.identifier;
                        generate_if_let_some_token_stream(
                            &quote::quote! {v_d5cd3c70},
                            &{
                                let field_type_token_stream =
                                    generate_as_pg_type_test_cases_path_token_stream(
                                        &element.type0,
                                    );
                                quote::quote! {#field_type_token_stream #ReadIdsAndCreateIntoOptionalVecWhereEqToFieldSnakeCase(
                                    read_ids_from_co.#field.clone().expect("65cef584 generate_read_ids_els_8a1ef027 invariant must hold"),
                                    identifier_create.#field.clone()
                                )}
                            },
                            &generate_for_each_assert_eq_token_stream(
                                &quote::quote! {v_d5cd3c70},
                                &quote::quote! {element_48a3d976},
                                field,
                            ),
                        )
                    },
                );
            let read_ids_and_table_type_into_pg_type_optional_where_greater_than_token_stream =
                generate_read_test_token_stream(
                    table_read_ids_and_table_type_into_pg_type_optional_where_greater_than_name,
                    &|_, field_type| {
                        quote::quote! {
                            <#field_type as #import_token_stream PgTypeTestCases>::#PgTypeOptionalVecWhereGreaterThanTestSnakeCase()
                            .map_or_else(Vec::new, Into::into)
                        }
                    },
                    &|field| {
                        generate_identifier_create_token_stream(
                            field,
                            &quote::quote! {#ElementSnakeCase.#CreateSnakeCase},
                        )
                    },
                    &|element: &macros_helpers::field_data::SynField| {
                        let field = &element.identifier;
                        generate_if_let_some_token_stream(
                            &quote::quote! {v_60baba1f},
                            &{
                                let field_type_token_stream =
                                    generate_as_pg_type_test_cases_path_token_stream(
                                        &element.type0,
                                    );
                                quote::quote! {#field_type_token_stream #ReadIdsAndTableTypeIntoPgTypeOptionalWhereGreaterThanSnakeCase(
                                    #ElementSnakeCase.variant,
                                    read_ids_from_co.#field.clone().expect("c8d34556 generate_read_ids_els_8a1ef027 invariant must hold"),
                                    #ElementSnakeCase.greater_than,
                                )}
                            },
                            &generate_read_ids_and_create_into_where_assert_eq_token_stream(
                                &generate_field_where_token_stream(
                                    field,
                                    &quote::quote! {vec![v_60baba1f]},
                                ),
                            ),
                        )
                    },
                );
            quote::quote! {
                #test_rm_by_non_existent_pks_token_stream
                #test_rm_by_eq_to_created_pks_token_stream
                #read_ids_and_create_into_where_eq_token_stream
                #read_ids_and_create_into_vec_where_eq_using_fields_token_stream
                #read_ids_and_create_into_optional_vec_where_eq_to_field_token_stream
                #read_ids_and_table_type_into_pg_type_optional_where_greater_than_token_stream
            }
        };
        let ro_tests_token_stream = generate_accumulator_push_future_token_stream(
            &quote::quote! {table_ro_cloned},
            &quote::quote! {table_ro},
            &quote::quote! {
                    generate_check_no_rows_from_identifier_try_ro_handle_primary_key(
                        &url_cloned,
                        #primary_key_field_type_as_pg_type_read_token_stream::new(uuid::Uuid::from_u128(3u128)),
                        #select_default_all_with_max_page_size_cloned_clone_token_stream,
                        &table_ro_cloned,
                    ).await;
            },
        );
        let generate_identifier_read_initialization_token_stream = |ts: &dyn quote::ToTokens| {
            let ts0 = generate_v_initialization_ts0(&primary_key_field_type_read_only_is_into_read_read_ids_element_primary_key_field_token_stream);
            quote::quote! {#identifier_read_upper_camel_case {
                #primary_key_field_identifier: Some(#ts0),
                #ts
            }}
        };
        let generate_read_inner_into_update_token_stream =
            |field: &dyn quote::ToTokens,
             field_type: &dyn quote::ToTokens,
             field_type_token_stream: &dyn quote::ToTokens,
             i_token_stream: &dyn quote::ToTokens| {
                let ts = generate_as_pg_type_test_cases_path_token_stream(&field_type);
                quote::quote! {
                    let update = #ts read_inner_into_update_with_new_or_try_new_unwraped({
                        let mut i_e0d2f9db: usize = 0;
                        let mut optional_test_case = None;
                        for element_3a9a65ee in #field_type_token_stream read_ids_to_2_dimensions_vec_read_inner(
                            &read_ids_element_937c5af3.#field.clone().expect("c4d98a71 generate_read_ids_els_8a1ef027 invariant must hold")
                        ) {
                            let mut should_break = false;
                            for element_bb734c11 in element_3a9a65ee {
                                if i_e0d2f9db == #i_token_stream {
                                    optional_test_case = Some(element_bb734c11);
                                    should_break = true;
                                    break;
                                }
                                i_e0d2f9db = i_e0d2f9db.checked_add(1).expect("326274d1 generate_read_ids_els_8a1ef027 invariant must hold");
                            }
                            if should_break {
                                break;
                            }
                        }
                        optional_test_case.expect("bd79056e generate_read_ids_els_8a1ef027 invariant must hold")
                    });
                }
            };
        let generate_read_ids_upper_fields_initialization_without_primary_key_token_stream: &dyn Fn(
            &syn::Ident,
        )
            -> proc_macro2::TokenStream = &|field| {
            generate_fields_named_without_primary_key_with_comma_token_stream(
                &|syn_field: &macros_helpers::field_data::SynField| {
                    let fi0 = &syn_field.identifier;
                    let ts = if field == fi0.as_ref() {
                        let ts0 = generate_as_pg_type_test_cases_path_token_stream(&syn_field.type0);
                        quote::quote! {Some(#ts0 update_to_read_ids(&update))}
                    } else {
                        quote::quote! {None}
                    };
                    quote::quote! {#fi0: #ts}
                },
            )
        };
        let generate_update_parameters_initialization_without_primary_key_token_stream: &dyn Fn(&syn::Ident) -> proc_macro2::TokenStream =
            &|field| {
                generate_fields_named_without_primary_key_with_comma_token_stream(
                    &|syn_field: &macros_helpers::field_data::SynField| {
                        let fi0 = &syn_field.identifier;
                        if field == fi0.as_ref() {
                            let ts = generate_v_initialization_ts0(&quote::quote! {#UpdateSnakeCase.clone()});
                            quote::quote! {Some(#ts)}
                        } else {
                            none_token_stream.clone()
                        }
                    },
                )
            };
        let generate_read_fields_after_update_token_stream: &dyn Fn(
            &syn::Ident,
            &dyn Fn(&syn::Ident) -> proc_macro2::TokenStream,
            &str,
            &str,
        )
            -> proc_macro2::TokenStream = &|field, else_fn, expect_uuid_0, expect_uuid_1| {
            generate_fields_named_without_primary_key_with_comma_token_stream(
                &|syn_field: &macros_helpers::field_data::SynField| {
                    let fi0 = &syn_field.identifier;
                    let ts = if field == fi0.as_ref() {
                        let ts0 = generate_v_initialization_ts0(&{
                            let ts1 =
                                generate_as_pg_type_test_cases_path_token_stream(&syn_field.type0);
                            let expect_0 = generate_quotes::dq_token_stream(&expect_uuid_0);
                            let expect_1 = generate_quotes::dq_token_stream(&expect_uuid_1);
                            quote::quote! {
                                #ts1 previous_read_and_optional_update_into_read(
                                    #ts1 read_ids_to_optional_v_read_default_some_one_element(
                                        &read_ids_element_937c5af3.#fi0.clone().expect(#expect_0)
                                    ).expect(#expect_1).#VSnakeCase,
                                    Some(#UpdateSnakeCase.clone())
                                )
                            }
                        });
                        quote::quote! {Some(#ts0)}
                    } else {
                        else_fn(fi0)
                    };
                    quote::quote! {#fi0: #ts}
                },
            )
        };
        let um_tests_token_stream = {
            //todo add Test for trying to update empty vec
            let um_only_one_column_tests_token_stream =
                generate_fields_named_without_primary_key_without_comma_token_stream(
                    &|element: &macros_helpers::field_data::SynField| {
                        let field = &element.identifier;
                        let field_type = &element.type0;
                        let field_type_token_stream =
                            generate_as_pg_type_test_cases_path_token_stream(field_type);
                        let is_fields_without_primary_key_len_greater_than_one =
                            fields_len_without_primary_key > 1;
                        let maybe_previous_read_token_stream =
                            if is_fields_without_primary_key_len_greater_than_one {
                                let ts =
                            generate_primary_key_where_eq_into_inner_token_stream(&primary_key_field_type_read_only_is_into_read_read_ids_element_primary_key_field_token_stream);
                                let where_primary_key_or_um_token_stream =
                                    generate_where_primary_key_or_token_stream(
                                        &quote::quote! {vec![#ts]},
                                    );
                                quote::quote! {
                                    let previous_read = itertools::Itertools::sorted_by(
                                        generate_try_rm_order_by_primary_key_with_big_pagination(
                                            &url_cloned,
                                            #where_primary_key_or_um_token_stream,
                                            #select_default_all_with_max_page_size_cloned_clone_token_stream,
                                            &table_um_cloned
                                        )
                                        .await
                                        .expect("540ec737 generate_read_ids_els_8a1ef027 invariant must hold")
                                        .into_iter(),
                                        #primary_key_sort_cmp_token_stream
                                    );
                                }
                            } else {
                                proc_macro2::TokenStream::new()
                            };
                        let field_read_ids_to_2_dimensions_vec_read_inner_accumulator_snake_case =
                        naming::parameter::SelfReadIdsTo2DimensionsVecReadInnerAccumulatorSnakeCase::from_tokens(&field);
                        let identifier_read_ids_upper_fields_initialization_without_primary_key_token_stream =
                        generate_read_ids_upper_fields_initialization_without_primary_key_token_stream(field);
                        let identifier_update_parameters_initialization_without_primary_key_token_stream = generate_update_parameters_initialization_without_primary_key_token_stream(field);
                        let identifier_read_fields_initialization_without_primary_key_after_uo_token_stream =
                            generate_read_fields_after_update_token_stream(
                                field,
                                &|fi0| quote::quote! {element_a6bc6b2f.#fi0},
                                str_constants::VALUE_96213542,
                                str_constants::BF0D6F55,
                            );
                        let expected_rm_token_stream = {
                            let ts = generate_identifier_read_initialization_token_stream(&identifier_read_fields_initialization_without_primary_key_after_uo_token_stream);
                            if is_fields_without_primary_key_len_greater_than_one {
                                quote::quote! {previous_read.into_iter().map(|element_a6bc6b2f|#ts).collect::<Vec<#identifier_read_upper_camel_case>>()}
                            } else {
                                quote::quote! {vec![#ts]}
                            }
                        };
                        let um_read_inner_into_update_token_stream =
                            generate_read_inner_into_update_token_stream(
                                &field,
                                &field_type,
                                &field_type_token_stream,
                                &quote::quote! {i_7f181188},
                            );
                        let assert_eq_um_read_ids_token_stream = generate_assert_eq_token_stream(
                            &quote::quote! {vec![
                                #identifier_read_ids_upper_camel_case {
                                    #primary_key_field_identifier: read_ids_element_937c5af3.#primary_key_field_identifier,
                                    #identifier_read_ids_upper_fields_initialization_without_primary_key_token_stream
                                }
                            ]},
                            &quote::quote! {
                                #identifier::try_um_handle(
                                    &url_cloned,
                                    #identifier_um_parameters_upper_camel_case {
                                        payload: #identifier_um_payload_upper_camel_case::try_new(vec![
                                            #identifier_update_upper_camel_case::try_new(
                                                #primary_key_field_type_as_pg_type_update_as_pg_type_primary_key_read_ids_into_update_token_stream,
                                                #identifier_update_parameters_initialization_without_primary_key_token_stream
                                            ).expect("42dc87b3 generate_read_ids_els_8a1ef027 invariant must hold")
                                        ]).expect("69e1bd8a generate_read_ids_els_8a1ef027 invariant must hold")
                                    },
                                    &table_um_cloned
                                ).await.expect("d2de0bd6 generate_read_ids_els_8a1ef027 invariant must hold")
                            },
                            &quote::quote! {"34bfb3c7"},
                        );
                        let assert_eq_um_rm_token_stream = generate_assert_eq_token_stream(
                            &quote::quote! {{#expected_rm_token_stream}},
                            &{
                                let ts = generate_primary_key_where_eq_into_inner_token_stream(
                                &primary_key_field_type_read_only_is_into_read_read_ids_element_primary_key_field_token_stream,
                            );
                                let where_primary_key_or_um_token_stream =
                                    generate_where_primary_key_or_token_stream(
                                        &quote::quote! {vec![#ts]},
                                    );
                                quote::quote! {
                                    itertools::Itertools::sorted_by(
                                        generate_try_rm_order_by_primary_key_with_big_pagination(
                                            &url_cloned,
                                            #where_primary_key_or_um_token_stream,
                                            select_default_all_with_max_page_size_cloned,
                                            &table_um_cloned
                                        )
                                        .await
                                        .expect("25c561e2 generate_read_ids_els_8a1ef027 invariant must hold")
                                        .into_iter(),
                                        #primary_key_sort_cmp_token_stream
                                    ).collect::<Vec<#identifier_read_upper_camel_case>>()
                                }
                            },
                            &quote::quote! {"ae2a2da5"},
                        );
                        let um_accumulator_push_future_token_stream =
                            generate_accumulator_push_future_token_stream(
                                &quote::quote! {table_um_cloned},
                                &quote::quote! {table_um},
                                &quote::quote! {
                                    #maybe_previous_read_token_stream
                                    #um_read_inner_into_update_token_stream
                                    #assert_eq_um_read_ids_token_stream
                                    #assert_eq_um_rm_token_stream
                                },
                            );
                        quote::quote! {
                            for (i_7f181188, read_ids_element_937c5af3) in generate_read_ids_els_8a1ef027(
                                &url,
                                &table_um,
                                #select_default_all_with_max_page_size_clone_token_stream,
                                #field_read_ids_to_2_dimensions_vec_read_inner_accumulator_snake_case.clone()
                            ).await.into_iter().enumerate() {
                                #um_accumulator_push_future_token_stream
                            }
                        }
                    },
                );
            quote::quote! {#um_only_one_column_tests_token_stream}
        };
        let uo_tests_token_stream = {
            let uo_only_one_column_tests_token_stream =
                generate_fields_named_without_primary_key_without_comma_token_stream(
                    &|element: &macros_helpers::field_data::SynField| {
                        let field = &element.identifier;
                        let field_type = &element.type0;
                        let field_type_token_stream =
                            generate_as_pg_type_test_cases_path_token_stream(field_type);
                        let maybe_previous_read_token_stream = if fields_len_without_primary_key > 1
                        {
                            quote::quote! {
                                let previous_read = generate_identifier_try_ro_handle_primary_key(
                                    &url_cloned,
                                    #primary_key_field_type_read_only_is_into_read_read_ids_element_primary_key_field_token_stream,
                                    #select_default_all_with_max_page_size_cloned_clone_token_stream,
                                    &table_uo_cloned
                                )
                                .await.expect("e6998b47 generate_read_ids_els_8a1ef027 invariant must hold");
                            }
                        } else {
                            proc_macro2::TokenStream::new()
                        };
                        let field_read_ids_to_2_dimensions_vec_read_inner_accumulator_snake_case =
                        naming::parameter::SelfReadIdsTo2DimensionsVecReadInnerAccumulatorSnakeCase::from_tokens(&field);
                        let identifier_read_ids_upper_fields_initialization_without_primary_key_token_stream =
                        generate_read_ids_upper_fields_initialization_without_primary_key_token_stream(field);
                        let identifier_update_parameters_initialization_without_primary_key_token_stream = generate_update_parameters_initialization_without_primary_key_token_stream(field);
                        let identifier_read_fields_initialization_without_primary_key_after_uo_token_stream =
                            generate_read_fields_after_update_token_stream(
                                field,
                                &|fi0| quote::quote! {previous_read.#fi0},
                                str_constants::VALUE_4F19D0D2,
                                str_constants::C7685B19,
                            );
                        let uo_read_inner_into_update_token_stream =
                            generate_read_inner_into_update_token_stream(
                                &field,
                                &field_type,
                                &field_type_token_stream,
                                &quote::quote! {i_26824592},
                            );
                        let assert_eq_uo_read_ids_token_stream = generate_assert_eq_token_stream(
                            &quote::quote! {#identifier_read_ids_upper_camel_case {
                                #primary_key_field_identifier: read_ids_element_937c5af3.#primary_key_field_identifier,
                                #identifier_read_ids_upper_fields_initialization_without_primary_key_token_stream
                            }},
                            &quote::quote! {
                                #identifier::try_uo_handle(
                                    &url_cloned,
                                    #identifier_uo_parameters_upper_camel_case {
                                        payload: #identifier_update_upper_camel_case::try_new(
                                            #primary_key_field_type_as_pg_type_update_as_pg_type_primary_key_read_ids_into_update_token_stream,
                                            #identifier_update_parameters_initialization_without_primary_key_token_stream
                                        ).expect("0e5d65a5 generate_read_ids_els_8a1ef027 invariant must hold")//todo add column identifier
                                    },
                                    &table_uo_cloned
                                ).await.expect("4d755542 generate_read_ids_els_8a1ef027 invariant must hold")
                            },
                            &quote::quote! {"564de31c"},
                        );
                        let assert_eq_uo_ro_token_stream = generate_assert_eq_token_stream(
                        &generate_identifier_read_initialization_token_stream(&identifier_read_fields_initialization_without_primary_key_after_uo_token_stream),
                        &quote::quote! {
                            generate_identifier_try_ro_handle_primary_key(
                                &url_cloned,
                                #primary_key_field_type_read_only_is_into_read_read_ids_element_primary_key_field_token_stream,
                                select_default_all_with_max_page_size_cloned,
                                &table_uo_cloned
                            )
                            .await.expect("75894c76 generate_read_ids_els_8a1ef027 invariant must hold")
                        },
                        &quote::quote! {"d5dec823"},
                    );
                        let uo_accumulator_push_future_token_stream =
                            generate_accumulator_push_future_token_stream(
                                &quote::quote! {table_uo_cloned},
                                &quote::quote! {table_uo},
                                &quote::quote! {
                                    #maybe_previous_read_token_stream
                                    #uo_read_inner_into_update_token_stream
                                    #assert_eq_uo_read_ids_token_stream
                                    #assert_eq_uo_ro_token_stream
                                },
                            );
                        quote::quote! {
                            for (i_26824592, read_ids_element_937c5af3) in generate_read_ids_els_8a1ef027(
                                &url,
                                &table_uo,
                                #select_default_all_with_max_page_size_clone_token_stream,
                                #field_read_ids_to_2_dimensions_vec_read_inner_accumulator_snake_case
                            ).await.into_iter().enumerate() {
                                #uo_accumulator_push_future_token_stream
                            }
                        }
                    },
                );
            quote::quote! {#uo_only_one_column_tests_token_stream}
        };
        let dm_tests_token_stream = {
            let test_dm_by_non_existent_pks_token_stream = generate_for_in_1_2_token_stream(
                &quote::quote! {element_39819198},
                &generate_accumulator_push_future_token_stream(
                    &quote::quote! {table_7e35b1ce},
                    &quote::quote! {table_test_rm_by_eq_to_created_pks},
                    &add_co_default_and_delete_after_just_to_add_some_data_to_be_sure_it_will_not_return_from_the_test_query_token_stream(&generate_assert_token_stream(
                        &{
                            let ts = generate_try_dm_handle_token_stream(
                                &quote::quote! {
                                    #primary_key_field_identifier: Some(
                                        generate_pg_type_where_try_new_primary_key(
                                            #operator_or_token_stream,
                                            std::iter::repeat_with(|| #primary_key_where_eq_uuid_new_v_token_stream)
                                            .take(element_39819198)
                                        )
                                    ),
                                    #fields_none_initialization_token_stream
                                },
                                &quote::quote! {table_7e35b1ce}
                            );
                            quote::quote! {#ts.is_empty()}
                        },
                        &quote::quote! {"51d14103"}
                    ))
                )
            );
            let test_dm_by_pks_token_stream = generate_for_in_1_2_token_stream(
                &quote::quote! {element_56409d32},
                &{
                    let dm_accumulator_push_future_token_stream = generate_accumulator_push_future_token_stream(
                    &quote::quote! {table_7e35b1ce},
                    &quote::quote! {table_test_rm_by_eq_to_created_pks},//todo is table name correct?
                    &add_co_default_and_delete_after_just_to_add_some_data_to_be_sure_it_will_not_return_from_the_test_query_token_stream(&{
                        let assert_eq_dm_read_ids_token_stream = generate_assert_eq_token_stream(
                            &quote::quote! {read_ids_from_try_dm},
                            &quote::quote! {{
                                read_ids_from_try_cm.iter().map(|element_ba0f6b1c|
                                    #primary_key_as_pg_type_test_cases_path_token_stream read_ids_to_optional_v_read_default_some_one_element(
                                        &element_ba0f6b1c.#primary_key_field_identifier
                                    ).expect("3ee5ee86 generate_read_ids_els_8a1ef027 invariant must hold").#VSnakeCase
                                ).collect::<Vec<#primary_key_field_type_as_pg_type_read_token_stream>>()
                            }},
                            &quote::quote! {"db5e88a6"}
                        );
                        let assert_dm_empty_token_stream = generate_assert_token_stream(
                            &{
                                let ts = generate_primary_key_where_eq_token_stream(&generate_primary_key_field_type_as_pg_type_primary_key_method_call_token_stream(&ReadIntoTableTypeSnakeCase, &quote::quote! {element_adcc8db3}));
                                let where_primary_key_or_dm_token_stream = generate_where_primary_key_or_token_stream(&quote::quote! {
                                    read_ids_from_try_dm.into_iter().map(|element_adcc8db3| #ts)
                                });
                                quote::quote! {
                                    generate_try_rm_order_by_primary_key_with_big_pagination(
                                        &url_cloned,
                                        #where_primary_key_or_dm_token_stream,
                                        select_default_all_with_max_page_size_cloned.clone(),
                                        &table_7e35b1ce
                                    ).await
                                    .expect("bcb79917 generate_read_ids_els_8a1ef027 invariant must hold")
                                    .is_empty()
                                }
                            },
                            &quote::quote! {"77f038b0"}
                        );
                        let dm_primary_key_where_eq_token_stream = generate_primary_key_where_eq_token_stream(&generate_primary_key_field_type_as_pg_type_primary_key_method_call_token_stream(
                            &ReadIdsIntoTableTypeSnakeCase,
                            &quote::quote! {element_3bb88958.#primary_key_field_identifier},
                        ));
                        let dm_read_ids_from_try_dm_token_stream = generate_read_ids_from_try_dm_token_stream(&generate_try_dm_handle_token_stream(
                            &quote::quote! {
                                #primary_key_field_identifier: Some(
                                    generate_pg_type_where_try_new_primary_key(
                                        #operator_or_token_stream,
                                        read_ids_from_try_cm.iter().map(|element_3bb88958| #dm_primary_key_where_eq_token_stream)
                                    )
                                ),
                                #fields_none_initialization_token_stream
                            },
                            &quote::quote! {table_7e35b1ce}
                        ));
                        quote::quote! {
                            let read_ids_from_try_cm = #identifier::try_cm_handle(
                                &url_cloned,
                                #identifier_cm_parameters_upper_camel_case {
                                    payload: #identifier_cm_payload_upper_camel_case(
                                        std::iter::repeat_n(identifier_create_default_cloned, element_56409d32).collect()
                                    )
                                },
                                &table_7e35b1ce
                            ).await.expect("b8695890 generate_read_ids_els_8a1ef027 invariant must hold");
                            #dm_read_ids_from_try_dm_token_stream
                            #assert_eq_dm_read_ids_token_stream
                            #assert_dm_empty_token_stream
                        }
                    })
                );
                    quote::quote! {
                        let identifier_create_default_cloned = identifier_create_default.clone();
                        #dm_accumulator_push_future_token_stream
                    }
                },
            );
            quote::quote! {
                #test_dm_by_non_existent_pks_token_stream
                #test_dm_by_pks_token_stream
            }
        };
        let dlo_tests_token_stream = {
            let ts = generate_v_initialization_ts0(&primary_key_field_type_read_ids_into_read_read_ids_from_co_primary_key_field_token_stream);
            let assert_eq_dlo_ro_primary_key_token_stream = generate_assert_eq_token_stream(
                &quote::quote! {#identifier_read_upper_camel_case {
                    #primary_key_field_identifier: Some(#ts),
                    #field_read_ids_and_create_into_optional_v_read_read_ids_from_co_create_token_stream
                }},
                &quote::quote! {
                    generate_identifier_try_ro_handle_primary_key(
                        &url,
                        #primary_key_field_type_read_ids_into_read_read_ids_from_co_primary_key_field_token_stream,
                        #select_default_all_with_max_page_size_cloned_clone_token_stream,
                        &table_dlo_cloned
                    )
                    .await.expect("c8c44c89 generate_read_ids_els_8a1ef027 invariant must hold")
                },
                &quote::quote! {"86ef08ae"},
            );
            let assert_eq_dlo_delete_primary_key_token_stream = generate_assert_eq_token_stream(
                &quote::quote! {
                    generate_try_dlo_handle(
                        &url,
                        #primary_key_field_type_read_ids_into_read_read_ids_from_co_primary_key_field_token_stream,
                        &table_dlo_cloned
                    ).await.expect("7e1d1a70 generate_read_ids_els_8a1ef027 invariant must hold")
                },
                &quote::quote! {#primary_key_field_type_read_ids_into_read_read_ids_from_co_primary_key_field_token_stream},
                &quote::quote! {"99f81971"},
            );
            let assert_dlo_no_rows_token_stream = generate_assert_token_stream(
                &quote::quote! {pg == no_rows_by_a_query_that_expected_to_return_at_least_one_row()},
                &quote::quote! {"c9261bb8"},
            );
            generate_accumulator_push_future_token_stream(
                &quote::quote! {table_dlo_cloned},
                &quote::quote! {table_dlo},
                &quote::quote! {
                        if let Err(#ErrorSnakeCase) = generate_try_dlo_handle(
                            &url_cloned,
                            #primary_key_field_type_as_pg_type_read_token_stream::new(uuid::Uuid::from_u128(4u128)),
                            &table_dlo_cloned
                        ).await {
                            if let #identifier_try_dlo_error_upper_camel_case::#identifier_dlo_error_with_serde_upper_camel_case {
                                dlo_error_with_serde,
                                ..
                            } = #ErrorSnakeCase {
                                if let #identifier_dlo_error_with_serde_upper_camel_case::Pg {
                                    pg,
                                    ..
                                } = dlo_error_with_serde {
                                    #assert_dlo_no_rows_token_stream
                                } else {
                                    panic!("e63b27a3");
                                }
                            } else {
                                panic!("47a8e0d9")
                            }
                        } else {
                            panic!("9be62f9f")
                        }
                        let read_ids_from_co = generate_read_ids_from_try_co_default(&url_cloned, &table_dlo_cloned).await;
                        #assert_eq_dlo_ro_primary_key_token_stream
                        #assert_eq_dlo_delete_primary_key_token_stream
                        generate_check_no_rows_from_identifier_try_ro_handle_primary_key(
                            &url_cloned,
                            #primary_key_field_type_read_ids_into_read_read_ids_from_co_primary_key_field_token_stream,
                            #select_default_all_with_max_page_size_cloned_clone_token_stream,
                            &table_dlo_cloned,
                        ).await;
                },
            )
        };
        let assert_table_name_len_token_stream = generate_assert_token_stream(
            &quote::quote! {v.len() <= 63},
            &quote::quote! {"77f9bfb7"},
        );
        let primary_key_where_eq_into_inner_read_ids_token_stream =
            generate_primary_key_where_eq_into_inner_token_stream(&quote::quote! {
                #primary_key_field_type_as_pg_type_primary_key_token_stream read_ids_into_read(element_9530b728.#primary_key_field_identifier)
            });
        let size_of_token_stream = {
            let ts = generate_assert_eq_token_stream(
                &quote::quote! {std::mem::size_of::<#identifier>()},
                &quote::quote! {0},
                &quote::quote! {"e8eed4b3"},
            );
            quote::quote! {
                #[test]
                fn size_of() {
                    #ts
                }
            }
        };
        let generate_identifier_where_primary_key_others_none_fn_token_stream = quote::quote! {
            fn generate_identifier_where_primary_key_others_none(
                optional_pg_type_where: Option<#import_token_stream PgTypeWhere<#primary_key_field_type_as_pg_type_where_token_stream>>,
            ) -> #identifier_where_upper_camel_case {
                #identifier_where_upper_camel_case::try_new(
                    optional_pg_type_where,
                    #fields_named_without_primary_key_with_comma_none_token_stream
                )
                .expect("5fb2b219 generate_identifier_where_primary_key_others_none invariant must hold")
            }
        };
        let generate_pg_type_where_try_new_primary_key_fn_token_stream = quote::quote! {
            fn generate_pg_type_where_try_new_primary_key<T>(
                operator: #import_token_stream Operator,
                vec: T,
            ) -> #import_token_stream PgTypeWhere<#primary_key_field_type_as_pg_type_where_token_stream>
            where
                T: IntoIterator<Item = #primary_key_field_type_as_pg_type_where_token_stream>,
            {
                let vec = vec.into_iter().collect::<Vec<#primary_key_field_type_as_pg_type_where_token_stream>>();
                #generate_pg_type_where_try_new_primary_key_token_stream
            }
        };
        let generate_pg_type_where_try_new_or_pks_fn_token_stream = quote::quote! {
            fn generate_pg_type_where_try_new_or_pks(
                vec_read_ids: &[#identifier_read_ids_upper_camel_case]
            ) -> #import_token_stream PgTypeWhere<#primary_key_field_type_as_pg_type_where_token_stream> {
                generate_pg_type_where_try_new_primary_key(
                    #operator_or_token_stream,
                    vec_read_ids.iter().map(|element_9530b728| #primary_key_where_eq_into_inner_read_ids_token_stream)
                )
            }
        };
        let generate_try_rm_order_by_primary_key_with_big_pagination_fn_token_stream = quote::quote! {
            async fn generate_try_rm_order_by_primary_key_with_big_pagination(
                endpoint_location: &str,
                identifier_where_6b1fab92: #identifier_where_upper_camel_case,
                select: #import_token_stream NotEmptyUniqueVec<#identifier_select_upper_camel_case>,
                table: &str
            ) -> Result<Vec<#identifier_read_upper_camel_case>, #identifier_try_rm_error_upper_camel_case> {
                #identifier::try_rm_handle(
                    endpoint_location,
                    #identifier_rm_parameters_upper_camel_case {
                        payload: #identifier_rm_payload_upper_camel_case {
                            where_many: #optional_identifier_where_upper_camel_case(Some(
                                identifier_where_6b1fab92
                            )),
                            select,
                            order_by: #import_token_stream OrderBy {
                                column: #identifier_select_upper_camel_case::#primary_key_field_upper_camel_case_token_stream(
                                    #primary_key_field_type_as_pg_type_select_token_stream::default()
                                ),
                                order: Some(#import_token_stream Order::Asc)
                            },
                            pagination: #import_token_stream PaginationStartsWithZero::try_new(10000, 0).expect("b0cdf0cb generate_try_rm_order_by_primary_key_with_big_pagination invariant must hold"),
                        }
                    },
                    table
                )
                .await
            }
        };
        let generate_identifier_try_ro_handle_primary_key_fn_token_stream = quote::quote! {
            async fn generate_identifier_try_ro_handle_primary_key(
                url: &str,
                primary_key_column: #primary_key_field_type_as_pg_type_read_token_stream,
                select: #import_token_stream NotEmptyUniqueVec<#identifier_select_upper_camel_case>,
                table: &str,
            ) -> Result<#identifier_read_upper_camel_case, #identifier_try_ro_error_upper_camel_case> {
                #identifier::try_ro_handle(
                    url,
                    #identifier_ro_parameters_upper_camel_case {
                        payload: #identifier_ro_payload_upper_camel_case {
                            primary_key_column,
                            select,
                        },
                    },
                    table,
                )
                .await
            }
        };
        let generate_check_no_rows_from_identifier_try_ro_handle_primary_key_fn_token_stream = {
            let ts = generate_assert_token_stream(
                &quote::quote! {pg == no_rows_by_a_query_that_expected_to_return_at_least_one_row()},
                &quote::quote! {"58b9a6a4"},
            );
            quote::quote! {
                async fn generate_check_no_rows_from_identifier_try_ro_handle_primary_key(
                    url: &str,
                    primary_key_column: #primary_key_field_type_as_pg_type_read_token_stream,
                    select: #import_token_stream NotEmptyUniqueVec<#identifier_select_upper_camel_case>,
                    table: &str,
                ) {
                    if let Err(#ErrorSnakeCase) = generate_identifier_try_ro_handle_primary_key(
                        url,
                        primary_key_column,
                        select,
                        table
                    ).await {
                        if let #identifier_try_ro_error_upper_camel_case::#identifier_ro_error_with_serde_upper_camel_case {
                            ro_error_with_serde,
                            ..
                        } = error {
                            if let #identifier_ro_error_with_serde_upper_camel_case::Pg { pg, .. } = ro_error_with_serde {
                                #ts
                            } else {
                                panic!("0ad0117b");
                            }
                        } else {
                            panic!("c6695392")
                        }
                    } else {
                        panic!("67e43b7a")
                    }
                }
            }
        };
        let identifier_create_default_fn_token_stream = quote::quote! {
            fn identifier_create_default() -> #identifier_create_upper_camel_case {
                #identifier_create_upper_camel_case {
                    #identifier_create_default_fields_initialization_without_primary_key_token_stream
                }
            }
        };
        let generate_read_ids_from_try_co_fn_token_stream = quote::quote! {
            async fn generate_read_ids_from_try_co(
                #UrlSnakeCase: &str,
                #PayloadSnakeCase: #identifier_create_upper_camel_case,
                table: &str,
            ) -> #identifier_read_ids_upper_camel_case {
                #identifier::try_co_handle(
                    #UrlSnakeCase,
                    #identifier_co_parameters_upper_camel_case {
                        #PayloadSnakeCase
                    },
                    table
                ).await.expect("32e30b87 generate_read_ids_from_try_co invariant must hold")
            }
        };
        let generate_read_ids_from_try_co_default_fn_token_stream = quote::quote! {
            async fn generate_read_ids_from_try_co_default(
                #UrlSnakeCase: &str,
                table: &str,
            ) -> #identifier_read_ids_upper_camel_case {
                generate_read_ids_from_try_co(
                    #UrlSnakeCase,
                    identifier_create_default(),
                    table
                ).await
            }
        };
        let generate_try_dlo_handle_fn_token_stream = quote::quote! {
            async fn generate_try_dlo_handle(
                #UrlSnakeCase: &str,
                #primary_key_field_identifier: #primary_key_field_type_as_pg_type_read_token_stream,
                table: &str,
            ) -> Result<#primary_key_field_type_as_pg_type_read_token_stream, #identifier_try_dlo_error_upper_camel_case> {
                #identifier::try_dlo_handle(
                    #UrlSnakeCase,
                    #identifier_dlo_parameters_upper_camel_case {
                        payload: #identifier_dlo_payload_upper_camel_case {
                            #primary_key_field_identifier
                        }
                    },
                    table
                ).await
            }
        };
        let no_rows_by_a_query_that_expected_to_return_at_least_one_row_fn_token_stream = quote::quote! {
            fn no_rows_by_a_query_that_expected_to_return_at_least_one_row() -> &'static str {
                "no rows returned by a query that expected to return at least one row"
            }
        };
        let generate_vec_identifier_read_from_vec_identifier_read_ids_with_vec_identifier_create_fn_token_stream = {
            let ts = generate_assert_eq_token_stream(
                &quote::quote! {read_ids_from_try_cm.len()},
                &quote::quote! {identifier_vec_create.len()},
                &quote::quote! {"88fb286c"},
            );
            quote::quote! {
                fn generate_vec_identifier_read_from_vec_identifier_read_ids_with_vec_identifier_create(
                    read_ids_from_try_cm: Vec<#identifier_read_ids_upper_camel_case>,
                    identifier_vec_create: Vec<#identifier_create_upper_camel_case>
                ) -> Vec<#identifier_read_upper_camel_case> {
                    let mut accumulator_1debe8fb = Vec::with_capacity(read_ids_from_try_cm.len());
                    #ts
                    for (read_ids, create) in read_ids_from_try_cm.into_iter().zip(identifier_vec_create) {
                        accumulator_1debe8fb.push(#identifier_read_upper_camel_case {
                            #primary_key_field_identifier: #primary_key_as_pg_type_test_cases_path_token_stream read_ids_to_optional_v_read_default_some_one_element(
                                &read_ids.#primary_key_field_identifier
                            ),
                            #field_read_ids_and_create_into_optional_v_read_read_ids_and_create_token_stream
                        });
                    }
                    accumulator_1debe8fb.sort_by(#primary_key_sort_cmp_token_stream);
                    accumulator_1debe8fb
                }
            }
        };
        quote::quote! {
            #[cfg(test)]
            mod #identifier_tests_snake_case {
                use super::*;
                #size_of_token_stream
                #[test]
                fn crud() {
                    #generate_identifier_where_primary_key_others_none_fn_token_stream
                    #generate_pg_type_where_try_new_primary_key_fn_token_stream
                    #generate_pg_type_where_try_new_or_pks_fn_token_stream
                    #generate_try_rm_order_by_primary_key_with_big_pagination_fn_token_stream
                    #generate_identifier_try_ro_handle_primary_key_fn_token_stream
                    #generate_check_no_rows_from_identifier_try_ro_handle_primary_key_fn_token_stream
                    #identifier_create_default_fn_token_stream
                    #generate_read_ids_from_try_co_fn_token_stream
                    #generate_read_ids_from_try_co_default_fn_token_stream
                    #generate_try_dlo_handle_fn_token_stream
                    #no_rows_by_a_query_that_expected_to_return_at_least_one_row_fn_token_stream
                    #generate_vec_identifier_read_from_vec_identifier_read_ids_with_vec_identifier_create_fn_token_stream
                    #generate_read_ids_els_token_stream
                    tracing_subscriber::fmt::initialization();
                    tokio::runtime::Builder::new_multi_thread().worker_threads(num_cpus::get()).enable_all().build().expect("38823c21 crud invariant must hold").block_on(async {
                        //todo maybe refactor
                        let database_url = "postgres://postgres:postgres@127.0.0.1:5432/rust_workspace_template_test?connect_timeout=10";
                        macros_helpers::test_database::validate_test_database_url(
                            macros_helpers::test_database::UrlRef::from(database_url)
                        ).expect("1876fb4e crud invariant must hold");
                        let mut #ConfigSnakeCase = #config_path_token_stream {
                            service_socket_address: <config_lib::ServiceSocketAddress as config_lib::TryFromStdEnvVarOk>::try_from_std_env_var_ok(config_lib::StdEnvVarOk(
                                "127.0.0.1:0".to_owned()
                            )).expect("b5b3915a crud invariant must hold").0,
                            database_url: <config_lib::DatabaseUrl as config_lib::TryFromStdEnvVarOk>::try_from_std_env_var_ok(config_lib::StdEnvVarOk(
                                database_url.to_owned()
                            )).expect("f9c20f05 crud invariant must hold").0,
                            timezone: <config_lib::ChronoTimezone as config_lib::TryFromStdEnvVarOk>::try_from_std_env_var_ok(config_lib::StdEnvVarOk(
                                "10800".to_owned()
                            )).expect("d00d8998 crud invariant must hold").0,
                            tracing_level: <config_lib::TracingLevel as config_lib::TryFromStdEnvVarOk>::try_from_std_env_var_ok(config_lib::StdEnvVarOk(
                                "error".to_owned()
                            )).expect("957178c9 crud invariant must hold").0,
                            src_place_type: <config_lib::SrcPlaceType as config_lib::TryFromStdEnvVarOk>::try_from_std_env_var_ok(config_lib::StdEnvVarOk(
                                "src".to_owned()
                            )).expect("bec0950e crud invariant must hold").0,
                            enable_api_git_commit_check: <config_lib::EnableApiGitCommitCheck as config_lib::TryFromStdEnvVarOk>::try_from_std_env_var_ok(config_lib::StdEnvVarOk(
                                "true".to_owned()
                            )).expect("31f02640 crud invariant must hold").0,
                            maximum_size_of_http_body_in_bytes: <config_lib::MaximumSizeOfHttpBodyInBytes as config_lib::TryFromStdEnvVarOk>::try_from_std_env_var_ok(config_lib::StdEnvVarOk(
                                "1048576000".to_owned()
                            )).expect("93b2f818 crud invariant must hold").0,
                            pg_pool_max_connections: <config_lib::PgPoolMaxConnections as config_lib::TryFromStdEnvVarOk>::try_from_std_env_var_ok(config_lib::StdEnvVarOk(
                                "50".to_owned()
                            )).expect("7c4e9f12 crud invariant must hold").0,
                            cors_allow_origin: <config_lib::CorsAllowOrigin as config_lib::TryFromStdEnvVarOk>::try_from_std_env_var_ok(config_lib::StdEnvVarOk(
                                "http://127.0.0.1".to_owned()
                            )).expect("a1b2c3d4 crud invariant must hold").0,
                        };
                        let #PgPoolSnakeCase = sqlx::postgres::PgPoolOptions::new()
                        .max_connections(50)
                        .connect(secrecy::ExposeSecret::expose_secret(app_state::GetDatabaseUrl::get_database_url(&#ConfigSnakeCase)))
                        .await.expect("e3044bb9 crud invariant must hold");
                        let tcp_listener = tokio::net::TcpListener::bind(app_state::GetServiceSocketAddress::get_service_socket_address(&#ConfigSnakeCase)).await.expect("663ae29e crud invariant must hold");
                        let actual_service_socket_address = tcp_listener.local_addr().expect("f31a9d0c crud invariant must hold");
                        #ConfigSnakeCase.service_socket_address = actual_service_socket_address;
                        let #UrlSnakeCase: std::sync::Arc<str> = std::sync::Arc::from(format!("http://{actual_service_socket_address}"));
                        let table = #identifier_double_quoted_token_stream;
                        let add_table_postfix = |postfix|{
                            let v = format!("{table}_{postfix}");
                            #assert_table_name_len_token_stream
                            std::sync::Arc::<str>::from(v)
                        };
                        let table_initialization = add_table_postfix("initialization");
                        let table_cm = add_table_postfix("cm");
                        let table_co = add_table_postfix("co");
                        let table_test_rm_by_non_existent_pks = add_table_postfix("Test_rm_by_non_existent_pks");
                        let table_test_rm_by_eq_to_created_pks = add_table_postfix("Test_rm_by_eq_to_created_pks");
                        #(#table_fis_initialization_vec_token_stream)*
                        let table_ro = add_table_postfix("ro");
                        let table_um = add_table_postfix("um");
                        let table_uo = add_table_postfix("uo");
                        let table_dm = add_table_postfix("dm");
                        let table_dlo = add_table_postfix("dlo");
                        let table_names = [
                            &table_initialization,
                            &table_cm,
                            &table_co,
                            &table_test_rm_by_non_existent_pks,
                            &table_test_rm_by_eq_to_created_pks,
                            #(#table_test_name_fis_vec_token_stream)*
                            &table_ro,
                            &table_um,
                            &table_uo,
                            &table_dm,
                            &table_dlo,
                        ];
                        let drop_all_test_tbls = async ||{
                            let _unused = futures::future::try_join_all(
                                table_names
                                .iter()
                                .map(|table_name|{
                                    let pg_pool_3b948340 = &pg_pool;
                                    async move {
                                        sqlx::query(sqlx::AssertSqlSafe(format!("drop table if exists {table_name}"))).execute(pg_pool_3b948340).await
                                    }
                                })
                            )
                            .await
                            .expect("b9c1eb2e crud invariant must hold");
                        };
                        drop_all_test_tbls().await;
                        #identifier::prep_extensions(&#PgPoolSnakeCase).await.expect("0633ff48 crud invariant must hold");
                        //do not make it concrnt. would be pg error: "duplicate k v violates unique constraint \"pg_class_relname_nsp_index\""
                        for element_dac43b91 in table_names {
                            #identifier::prep_pg_table(
                                &#PgPoolSnakeCase,
                                element_dac43b91,
                            ).await.expect("c7952247 crud invariant must hold");
                        }
                        let #PgPoolForTokioSpawnSyncMoveSnakeCase = #PgPoolSnakeCase.clone();
                        let table_names_cloned = table_names.map(|element_26b304d1| std::sync::Arc::<str>::clone(element_26b304d1));
                        let (started_tx, started_rx) = tokio::sync::oneshot::channel();
                        let #undrscr_unused_token_stream = tokio::spawn(async move {
                            let #AppStateSnakeCase = std::sync::Arc::new(server_app_state::ServerAppState {
                                bulk_item_budget: server_runtime_http::ResourceBudget::new(
                                    server_runtime_http::ResourceBudgetMaximum::try_from(4_096usize).expect("f9304636 crud invariant must hold"),
                                ),
                                #PgPoolSnakeCase: app_state::SqlxPgPool::from(#PgPoolForTokioSpawnSyncMoveSnakeCase.clone()),
                                #ConfigSnakeCase,
                                idempotency_response_budget: server_runtime_http::ResourceBudget::new(
                                    server_runtime_http::ResourceBudgetMaximum::try_from(67_108_864usize).expect("c75e4935 crud invariant must hold"),
                                ),
                                project_git_info: git_info::project_git_info(),
                            });
                            started_tx.send(()).expect("431a6f8d crud invariant must hold");
                            axum::serve(
                                tcp_listener,
                                {
                                    let mut router = axum::Router::new()
                                        .merge(#identifier::routes(std::sync::Arc::<server_app_state::ServerAppState<'_>>::clone(&app_state)));
                                    for element_ef09f2b0 in table_names_cloned {
                                        router = router.merge(#identifier::routes_handle(std::sync::Arc::<server_app_state::ServerAppState<'_>>::clone(&app_state), &element_ef09f2b0));
                                    }
                                    router.into_make_service()
                                },
                            )
                            .await
                            .expect("71c1bc30 crud invariant must hold");
                        });
                        started_rx.await.expect("87003141 crud invariant must hold");
                        let #SelectPrimaryKeySnakeCase = #import_token_stream NotEmptyUniqueVec::try_new_by_hash(vec![
                            #identifier_select_upper_camel_case::#primary_key_field_upper_camel_case_token_stream(
                                #primary_key_field_type_as_pg_type_select_token_stream::default(),
                            )
                        ].into())
                        .expect("0776170e crud invariant must hold");
                        let #IdentifierCreateDefaultSnakeCase = identifier_create_default();
                        #select_default_all_with_max_page_size_not_empty_unique_vec_token_stream
                        #common_read_ids_from_co_token_stream
                        #read_ids_to_2_dimensions_vec_read_inner_accumulator_fields_token_stream
                        const TEST_FUTURE_CONCURRENCY_D281414B: usize = 100;
                        const TEST_FUTURE_BASE_CAPACITY_7C87B2A1: usize = #fields_len_without_primary_key;
                        futures::StreamExt::for_each_concurrent(
                            futures::stream::iter({
                                let mut accumulator_9189f86e: Vec<futures::future::BoxFuture<'static, ()>> = Vec::with_capacity(
                                    TEST_FUTURE_BASE_CAPACITY_7C87B2A1
                                        .saturating_mul(16)
                                        .saturating_add(6)
                                );
                                #cm_tests_token_stream
                                #co_tests_token_stream
                                #rm_tests_token_stream
                                #ro_tests_token_stream
                                #um_tests_token_stream
                                #uo_tests_token_stream
                                #dm_tests_token_stream
                                #dlo_tests_token_stream
                                accumulator_9189f86e
                            }),
                            TEST_FUTURE_CONCURRENCY_D281414B,
                            async |fut| { fut.await; },
                        )
                        .await;
                        drop_all_test_tbls().await;
                    });
                }
            }
        }
    };
    let identifier_tests_token_stream = emit_generate_pg_table_tests_stage(
        &generate_pg_table_input_model.config,
        ProcMacro2GeneratePgTableTestsTokenStream::from(generated_identifier_tests_token_stream),
    )
    .into_inner();
    let identifier_create_form_upper_camel_case = quote::format_ident!("{}CreateForm", identifier);
    let identifier_update_form_upper_camel_case = quote::format_ident!("{}UpdateForm", identifier);
    let create_form_fields_token_stream = create_fields_without_primary_key_iter().map(|field| {
        let field_identifier = &field.identifier;
        quote::quote! {pub #field_identifier: frontend_contract::FormValue}
    });
    let update_form_fields_token_stream = std::iter::once({
        let field_identifier = &primary_key_field.identifier;
        quote::quote! {pub #field_identifier: frontend_contract::FormValue}
    })
    .chain(fields_without_primary_key_iter().map(|field| {
        let field_identifier = &field.identifier;
        quote::quote! {pub #field_identifier: Option<frontend_contract::FormValue>}
    }));
    let create_form_conversion_token_stream = create_fields_without_primary_key_iter().map(|field| {
        let field_identifier = &field.identifier;
        let field_name_double_quoted_token_stream = generate_quotes::dq_token_stream(&field.identifier);
        let origin_role = quote::format_ident!("Origin");
        let origin_type = generate_concrete_pg_type_role_token_stream(&field.type0, &origin_role);
        let create_type = generate_concrete_pg_type_role_token_stream(&field.type0, &CreateUpperCamelCase);
        quote::quote! {
            #field_identifier: #create_type::from(
                <#origin_type as frontend_contract::FormValueContract>::parse_form_value(
                    frontend_contract::FormValueRef::from(value.#field_identifier.as_ref()),
                )
                .map_err(|error| frontend_contract::FormFieldError::new(
                    error,
                    frontend_contract::ContractStr::from(#field_name_double_quoted_token_stream),
                ))?,
            )
        }
    });
    let update_form_conversion_token_stream = std::iter::once({
        let field_identifier = &primary_key_field.identifier;
        let field_name_double_quoted_token_stream = generate_quotes::dq_token_stream(&primary_key_field.identifier);
        let origin_role = quote::format_ident!("Origin");
        let origin_type = generate_concrete_pg_type_role_token_stream(&primary_key_field.type0, &origin_role);
        let update_type = generate_concrete_pg_type_role_token_stream(&primary_key_field.type0, &UpdateUpperCamelCase);
        quote::quote! {
            #field_identifier: #update_type::from(
                <#origin_type as frontend_contract::FormValueContract>::parse_form_value(
                    frontend_contract::FormValueRef::from(value.#field_identifier.as_ref()),
                )
                .map_err(|error| frontend_contract::FormFieldError::new(
                    error,
                    frontend_contract::ContractStr::from(#field_name_double_quoted_token_stream),
                ))?,
            )
        }
    })
    .chain(fields_without_primary_key_iter().map(|field| {
        let field_identifier = &field.identifier;
        let field_name_double_quoted_token_stream = generate_quotes::dq_token_stream(&field.identifier);
        let origin_role = quote::format_ident!("Origin");
        let origin_type = generate_concrete_pg_type_role_token_stream(&field.type0, &origin_role);
        let update_type = generate_concrete_pg_type_role_token_stream(&field.type0, &UpdateUpperCamelCase);
        quote::quote! {
            #field_identifier: value.#field_identifier
                .map(|field_value| {
                    <#origin_type as frontend_contract::FormValueContract>::parse_form_value(
                        frontend_contract::FormValueRef::from(field_value.as_ref()),
                    )
                    .map(|parsed| pg_crud_common::V { v: #update_type::from(parsed) })
                    .map_err(|error| frontend_contract::FormFieldError::new(
                        error,
                        frontend_contract::ContractStr::from(#field_name_double_quoted_token_stream),
                    ))
                })
                .transpose()?
        }
    }));
    let frontend_form_token_stream = quote::quote! {
        #[derive(Clone, Debug, Default)]
        pub struct #identifier_create_form_upper_camel_case {
            #(#create_form_fields_token_stream),*
        }
        impl TryFrom<#identifier_create_form_upper_camel_case> for #identifier_create_upper_camel_case {
            type Error = frontend_contract::FormFieldError;
            fn try_from(value: #identifier_create_form_upper_camel_case) -> Result<Self, Self::Error> {
                Ok(Self {
                    #(#create_form_conversion_token_stream),*
                })
            }
        }
        #[derive(Clone, Debug, Default)]
        pub struct #identifier_update_form_upper_camel_case {
            #(#update_form_fields_token_stream),*
        }
        impl TryFrom<#identifier_update_form_upper_camel_case> for #identifier_update_upper_camel_case {
            type Error = frontend_contract::FormFieldError;
            fn try_from(value: #identifier_update_form_upper_camel_case) -> Result<Self, Self::Error> {
                Ok(Self {
                    #(#update_form_conversion_token_stream),*
                })
            }
        }
    };
    let db_column_specs_token_stream = fields.iter().enumerate().map(|(index, field)| {
        let field_name = generate_quotes::dq_token_stream(&field.identifier);
        let field_type = &field.type0;
        let has_explicit_default = db_default_field_idxs
            .iter()
            .any(|field_idx| field_idx.get() == index);
        quote::quote! {
            pg_crud_common::DbColumnSpec::new(
                pg_crud_common::DbStaticSchemaText::from(#field_name),
                <#field_type as pg_crud_common::PgColumnSchema>::data_type(),
                pg_crud_common::DbColumnNullable::from(<#field_type as pg_crud_common::PgColumnSchema>::NULLABLE),
                pg_crud_common::DbColumnHasServerDefault::from(<#field_type as pg_crud_common::PgColumnSchema>::HAS_SERVER_DEFAULT || #has_explicit_default),
            )
        }
    });
    let primary_key_field_name = generate_quotes::dq_token_stream(&primary_key_field.identifier);
    let create_excluded_column_token_stream = create_exclude_fields
        .iter()
        .map(generate_quotes::dq_token_stream);
    let read_excluded_column_token_stream = read_exclude_fields
        .iter()
        .map(generate_quotes::dq_token_stream);
    let db_unique_key_token_stream = generate_pg_table_input_model
        .config
        .db_unique_keys
        .iter()
        .map(|columns| {
            let column_token_stream = columns.iter().map(generate_quotes::dq_token_stream);
            quote::quote! {
                pg_crud_common::DbKeySpec::Unique(vec![
                    #(pg_crud_common::DbStaticSchemaText::from(#column_token_stream)),*
                ].into())
            }
        });
    let db_foreign_key_token_stream = generate_pg_table_input_model
        .config
        .db_foreign_keys
        .iter()
        .map(|foreign_key| {
            let column_token_stream = foreign_key
                .columns
                .iter()
                .map(generate_quotes::dq_token_stream);
            let referenced_column_token_stream = foreign_key
                .referenced_columns
                .iter()
                .map(generate_quotes::dq_token_stream);
            let referenced_table_token_stream =
                generate_quotes::dq_token_stream(&foreign_key.referenced_table);
            quote::quote! {
                pg_crud_common::DbKeySpec::ForeignKey {
                    columns: vec![
                        #(pg_crud_common::DbStaticSchemaText::from(#column_token_stream)),*
                    ].into(),
                    referenced_columns: vec![
                        #(pg_crud_common::DbStaticSchemaText::from(#referenced_column_token_stream)),*
                    ].into(),
                    referenced_table: pg_crud_common::DbStaticSchemaText::from(#referenced_table_token_stream),
                }
            }
        });
    let db_table_schema_token_stream = quote::quote! {
        impl pg_crud_common::DbTableSchema for #identifier {
            fn columns() -> pg_crud_common::DbColumnSpecs {
                vec![#(#db_column_specs_token_stream),*].into()
            }
            fn create_excluded_columns() -> pg_crud_common::DbStaticSchemaTexts {
                vec![#(pg_crud_common::DbStaticSchemaText::from(#create_excluded_column_token_stream)),*].into()
            }
            fn keys() -> pg_crud_common::DbKeySpecs {
                vec![
                    pg_crud_common::DbKeySpec::PrimaryKey(vec![
                        pg_crud_common::DbStaticSchemaText::from(#primary_key_field_name)
                    ].into())
                    #(, #db_unique_key_token_stream)*
                    #(, #db_foreign_key_token_stream)*
                ].into()
            }
            fn primary_key_column() -> pg_crud_common::DbStaticSchemaText {
                pg_crud_common::DbStaticSchemaText::from(#primary_key_field_name)
            }
            fn read_excluded_columns() -> pg_crud_common::DbStaticSchemaTexts {
                vec![#(pg_crud_common::DbStaticSchemaText::from(#read_excluded_column_token_stream)),*].into()
            }
            fn schema_table_text() -> pg_crud_common::DbStaticSchemaText {
                pg_crud_common::DbStaticSchemaText::from(#db_table_name_double_quoted_token_stream)
            }
        }
    };
    let common_token_stream = quote::quote! {
        #identifier_prep_pg_error_token_stream
        #identifier_create_token_stream
        #identifier_where_token_stream
        #optional_identifier_where_token_stream
        #select_token_stream
        #identifier_read_token_stream
        #identifier_read_ids_token_stream
        #identifier_update_token_stream
        #identifier_update_for_query_token_stream
        #(#open_api_path_token_stream)*
        #identifier_open_api_token_stream
    };
    let gend = {
        let identifier_generate_pg_table_mod_snake_case =
            naming::parameter::SelfGeneratePgTableModSnakeCase::from_tokens(&identifier);
        let impl_and_content_token_stream = quote::quote! {
                #AllowClippyArbitrarySrcItemOrdering
                impl #identifier {
                    #(#impl_identifier_vec_token_stream)*
                    fn #RoutesHSnakeCase(
                        #AppStateSnakeCase: #std_sync_arc_combination_of_app_state_logic_traits_token_stream,
                        #TableSnakeCase: &str,
                        #db_table_snake_case: &str,
                    ) -> axum::Router {
                        axum::Router::new().nest(
                            &format!("/{table}"),
                            axum::Router::new()
                            #(#operation_routes_token_stream)*
                            .method_not_allowed_fallback(|| async {
                                frontend_contract::ApiProblemError::MethodNotAllowed
                            })
                            .fallback(|| async {
                                frontend_contract::ApiProblemError::NotFound
                            })
                            .with_state(#AppStateSnakeCase)
                        )
                    }
                }
                #(#content_token_stream)*
                #identifier_api_client_token_stream
                #identifier_route_contract_token_stream
                #db_table_schema_token_stream
                #frontend_form_token_stream
                #(#frontend_capability_assertions_token_stream)*
                #common_token_stream
                #generated_contract_tests_token_stream
                #identifier_tests_token_stream
        };
        quote::quote! {
            #[allow(unused_qualifications)]
            #[allow(clippy::absolute_paths)]
            mod #identifier_generate_pg_table_mod_snake_case {
                use super::#identifier;
                #impl_and_content_token_stream
            }
            pub use #identifier_generate_pg_table_mod_snake_case::*;
        }
    };
    emit_generate_pg_table_final_stage(
        &generate_pg_table_input_model.config,
        &ProcMacro2GeneratePgTableCommonTokenStream::from(common_token_stream),
        ProcMacro2GeneratePgTableWholeTokenStream::from(gend),
    )
}

#[cfg(test)]
mod pipeline_tests {
    #[test]
    fn validation_rejects_non_struct_input_without_emitting_source() {
        let input = quote::quote! { enum NotATable { Value } };
        let parsed = crate::pipeline::parse_generate_pg_table(
            macros_helpers::ts_writer::ProcMacro2TokenStreamRef::from(&input),
        )
        .expect("5d4f86a1 validation_rejects_non_struct_input_without_emitting_source invariant must hold");
        assert!(matches!(
            crate::pipeline::build_generate_pg_table(parsed),
            Err(crate::pipeline::GeneratePgTablePipelineError::Build(_error))
        ));
    }

    #[test]
    fn build_stage_exposes_typed_model_without_emitting_source() {
        let input = quote::quote! { struct Table { id: i64, name: String } };
        let parsed = crate::pipeline::parse_generate_pg_table(
            macros_helpers::ts_writer::ProcMacro2TokenStreamRef::from(&input),
        )
        .expect(
            "0f8b43d2 build_stage_exposes_typed_model_without_emitting_source invariant must hold",
        );
        let built = crate::pipeline::build_generate_pg_table(parsed).expect(
            "a715e9c4 build_stage_exposes_typed_model_without_emitting_source invariant must hold",
        );
        assert_eq!(usize::from(built.model().field_count()), 2usize);
    }

    #[test]
    fn validation_rejects_empty_table_model_without_emitting_source() {
        let input = quote::quote! { struct EmptyTable; };
        let parsed = crate::pipeline::parse_generate_pg_table(
            macros_helpers::ts_writer::ProcMacro2TokenStreamRef::from(&input),
        )
        .expect("67d029ab validation_rejects_empty_table_model_without_emitting_source invariant must hold");
        let built = crate::pipeline::build_generate_pg_table(parsed).expect("c15b8f34 validation_rejects_empty_table_model_without_emitting_source invariant must hold");
        assert!(matches!(
            crate::pipeline::validate_generate_pg_table(built),
            Err(crate::pipeline::GeneratePgTablePipelineError::Validate(
                _error
            ))
        ));
    }
}
