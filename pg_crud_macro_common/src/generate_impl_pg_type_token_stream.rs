#![allow(
    clippy::wildcard_imports,
    reason = "split owner modules import the private facade vocabulary used by the moved generator"
)]

pub fn generate_impl_pg_type_token_stream(
    import: &crate::import::Import,
    identifier: &dyn quote::ToTokens,
    identifier_table_type_upper_camel_case: &dyn quote::ToTokens,
    is_primary_key_undrscr: &crate::emission_types::IsPrimaryKeyUndrscr,
    create_table_column_query_part_token_stream: &dyn quote::ToTokens,
    identifier_create_upper_camel_case: &dyn quote::ToTokens,
    create_query_part_v_undrscr: &crate::emission_types::CreateQueryPartValueUndrscr,
    create_query_part_increment_undrscr: &crate::emission_types::CreateQueryPartIncrementUndrscr,
    create_query_part_token_stream: &dyn quote::ToTokens,
    create_query_bind_v_undrscr: &crate::emission_types::CreateQueryBindValueUndrscr,
    is_create_query_bind_mut: &crate::emission_types::IsCreateQueryBindMut,
    create_query_bind_token_stream: &dyn quote::ToTokens,
    identifier_select_upper_camel_case: &dyn quote::ToTokens,
    select_query_part_v_undrscr: &crate::emission_types::SelectQueryPartValueUndrscr,
    select_query_part_token_stream: &dyn quote::ToTokens,
    identifier_where_upper_camel_case: &dyn quote::ToTokens,
    identifier_read_upper_camel_case: &dyn quote::ToTokens,
    normalize_token_stream: &dyn quote::ToTokens,
    read_ids_token_stream: &dyn quote::ToTokens,
    select_only_ids_query_part_token_stream: &dyn quote::ToTokens,
    identifier_read_inner_upper_camel_case: &dyn quote::ToTokens,
    into_inner_token_stream: &dyn quote::ToTokens,
    identifier_update_upper_camel_case: &dyn quote::ToTokens,
    identifier_update_for_query_upper_camel_case: &dyn quote::ToTokens,
    update_query_part_v_undrscr: &crate::emission_types::UpdateQueryPartValueUndrscr,
    update_query_part_accumulator_undrscr: &crate::emission_types::UpdateQueryPartAccumulatorUndrscr,
    update_query_part_target_undrscr: &crate::emission_types::UpdateQueryPartTargetUndrscr,
    update_query_part_path_undrscr: &crate::emission_types::UpdateQueryPartPathUndrscr,
    update_query_part_token_stream: &dyn quote::ToTokens,
    is_update_query_bind_mut: &crate::emission_types::IsUpdateQueryBindMut,
    update_query_bind_token_stream: &dyn quote::ToTokens,
    select_only_updated_ids_query_part_token_stream: &dyn quote::ToTokens,
    is_select_only_updated_ids_query_bind_mut: &crate::emission_types::IsSelectOnlyUpdatedIdsQueryBindMut,
    select_only_updated_ids_query_bind_token_stream: &dyn quote::ToTokens,
) -> macro_helpers::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream {
    let names = crate::names_ctx::NamesCtx::new();
    // The owner module retains lint-sensitive semantics from the original implementation.
    #[allow(non_snake_case)]
    let (
        AllowClippyArbitrarySrcItemOrdering,
        ColumnSnakeCase,
        CreateQueryBindSnakeCase,
        CreateQueryPartSnakeCase,
        CreateTableColumnQueryPartSnakeCase,
        CreateUpperCamelCase,
        IncrementSnakeCase,
        NormalizeSnakeCase,
        PgTypeUpperCamelCase,
        QueryPartErrorUpperCamelCase,
        QuerySnakeCase,
        ReadIdsUpperCamelCase,
        ReadInnerUpperCamelCase,
        ReadUpperCamelCase,
        SelectOnlyIdsQueryPartSnakeCase,
        SelectOnlyUpdatedIdsQueryBindSnakeCase,
        SelectOnlyUpdatedIdsQueryPartSnakeCase,
        SelectQueryPartSnakeCase,
        SelectUpperCamelCase,
        TableTypeUpperCamelCase,
        UpdateForQueryUpperCamelCase,
        UpdateQueryBindSnakeCase,
        UpdateQueryPartSnakeCase,
        UpdateUpperCamelCase,
        VSnakeCase,
        WhereUpperCamelCase,
    ) = (
        names.get_allow_clippy_arbitrary_src_item_ordering(),
        names.get_column_snake_case(),
        names.get_create_query_bind_snake_case(),
        names.get_create_query_part_snake_case(),
        names.get_create_table_column_query_part_snake_case(),
        names.get_create_upper_camel_case(),
        names.get_increment_snake_case(),
        names.get_normalize_snake_case(),
        names.get_pg_type_upper_camel_case(),
        names.get_query_part_error_upper_camel_case(),
        names.get_query_snake_case(),
        names.get_read_ids_upper_camel_case(),
        names.get_read_inner_upper_camel_case(),
        names.get_read_upper_camel_case(),
        names.get_select_only_ids_query_part_snake_case(),
        names.get_select_only_updated_ids_query_bind_snake_case(),
        names.get_select_only_updated_ids_query_part_snake_case(),
        names.get_select_query_part_snake_case(),
        names.get_select_upper_camel_case(),
        names.get_table_type_upper_camel_case(),
        names.get_update_for_query_upper_camel_case(),
        names.get_update_query_bind_snake_case(),
        names.get_update_query_part_snake_case(),
        names.get_update_upper_camel_case(),
        names.get_v_snake_case(),
        names.get_where_upper_camel_case(),
    );
    quote::quote! {
        #AllowClippyArbitrarySrcItemOrdering
        impl #import::pg_type::#PgTypeUpperCamelCase for #identifier {
            type #TableTypeUpperCamelCase = #identifier_table_type_upper_camel_case;
            fn #CreateTableColumnQueryPartSnakeCase(#ColumnSnakeCase: #import::sql_column_ref::SqlColumnRef<'_>, #is_primary_key_undrscr: #import::is_primary_key::IsPrimaryKey) -> #import::query_part_fragment::QueryPartFragment {
                #create_table_column_query_part_token_stream
            }
            type #CreateUpperCamelCase = #identifier_create_upper_camel_case;
            fn #CreateQueryPartSnakeCase(
                #create_query_part_v_undrscr: &Self::#CreateUpperCamelCase,
                #create_query_part_increment_undrscr: &mut dyn #import::query_part_increment_mut::QueryPartIncrementMut
            ) -> Result<#import::query_part_fragment::QueryPartFragment, #import::query_part_error::#QueryPartErrorUpperCamelCase> {
                #create_query_part_token_stream
            }
            fn #CreateQueryBindSnakeCase(
                #create_query_bind_v_undrscr: Self::#CreateUpperCamelCase,
                #is_create_query_bind_mut #QuerySnakeCase: #import::sqlx_postgres_query::SqlxPostgresQuery<'_>
            ) -> Result<#import::sqlx_postgres_query::SqlxPostgresQuery<'_>, #import::sqlx_postgres_query_bind_error::SqlxPostgresQueryBindError> {
                #create_query_bind_token_stream
            }
            type #SelectUpperCamelCase = #identifier_select_upper_camel_case;
            fn #SelectQueryPartSnakeCase(
                #select_query_part_v_undrscr: &Self::#SelectUpperCamelCase,
                #ColumnSnakeCase: #import::sql_column_ref::SqlColumnRef<'_>,
            ) -> Result<#import::query_part_fragment::QueryPartFragment, #import::query_part_error::#QueryPartErrorUpperCamelCase> {
                #select_query_part_token_stream
            }
            type #WhereUpperCamelCase = #identifier_where_upper_camel_case;
            type #ReadUpperCamelCase = #identifier_read_upper_camel_case;
            fn #NormalizeSnakeCase(#VSnakeCase: Self::#ReadUpperCamelCase) -> Self::#ReadUpperCamelCase {
                #normalize_token_stream
            }
            type #ReadIdsUpperCamelCase = #read_ids_token_stream;
            fn #SelectOnlyIdsQueryPartSnakeCase(
                #ColumnSnakeCase: #import::sql_column_ref::SqlColumnRef<'_>
            ) -> Result<#import::query_part_fragment::QueryPartFragment, #import::query_part_error::#QueryPartErrorUpperCamelCase> {
                #select_only_ids_query_part_token_stream
            }
            type #ReadInnerUpperCamelCase = #identifier_read_inner_upper_camel_case;
            fn into_inner(#VSnakeCase: Self::#ReadUpperCamelCase) -> Self::#ReadInnerUpperCamelCase {
                #into_inner_token_stream
            }
            type #UpdateUpperCamelCase = #identifier_update_upper_camel_case;
            type #UpdateForQueryUpperCamelCase = #identifier_update_for_query_upper_camel_case;
            // The owner module retains lint-sensitive semantics from the original implementation.
            #[allow(unused_variables)]
            fn #UpdateQueryPartSnakeCase(
                #update_query_part_v_undrscr: &Self::#UpdateForQueryUpperCamelCase,
                #update_query_part_accumulator_undrscr: #import::sql_column_ref::SqlColumnRef<'_>,
                #update_query_part_target_undrscr: #import::sql_column_ref::SqlColumnRef<'_>,
                #update_query_part_path_undrscr: #import::sql_column_ref::SqlColumnRef<'_>,
                #IncrementSnakeCase: &mut dyn #import::query_part_increment_mut::QueryPartIncrementMut
            ) -> Result<#import::query_part_fragment::QueryPartFragment, #import::query_part_error::#QueryPartErrorUpperCamelCase> {
                #update_query_part_token_stream
            }
            fn #UpdateQueryBindSnakeCase(
                #VSnakeCase: Self::#UpdateForQueryUpperCamelCase,
                #is_update_query_bind_mut #QuerySnakeCase: #import::sqlx_postgres_query::SqlxPostgresQuery<'_>
            ) -> Result<#import::sqlx_postgres_query::SqlxPostgresQuery<'_>, #import::sqlx_postgres_query_bind_error::SqlxPostgresQueryBindError> {
                #update_query_bind_token_stream
            }
            fn #SelectOnlyUpdatedIdsQueryPartSnakeCase(
                #VSnakeCase: &Self::#UpdateForQueryUpperCamelCase,
                #ColumnSnakeCase: #import::sql_column_ref::SqlColumnRef<'_>,
                #IncrementSnakeCase: &mut dyn #import::query_part_increment_mut::QueryPartIncrementMut,
            ) -> Result<#import::query_part_fragment::QueryPartFragment, #import::query_part_error::#QueryPartErrorUpperCamelCase> {
                #select_only_updated_ids_query_part_token_stream
            }
            fn #SelectOnlyUpdatedIdsQueryBindSnakeCase<'lt>(
                #VSnakeCase: &'lt Self::#UpdateForQueryUpperCamelCase,
                #is_select_only_updated_ids_query_bind_mut #QuerySnakeCase: #import::sqlx_postgres_query::SqlxPostgresQuery<'lt>
            ) -> Result<#import::sqlx_postgres_query::SqlxPostgresQuery<'lt>, #import::sqlx_postgres_query_bind_error::SqlxPostgresQueryBindError> {
                #select_only_updated_ids_query_bind_token_stream
            }
        }
    }.into()
}
