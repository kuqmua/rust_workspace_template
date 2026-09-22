#![allow(
    clippy::field_scoped_visibility_modifiers,
    clippy::same_name_method,
    reason = "Leptos emits sibling props fields and builder methods with framework-defined visibility and names from the single component in this module"
)]

use leptos::prelude::{AddAnyAttr, ClassAttribute, CustomAttribute, ElementChild};

#[leptos::component]
#[allow(
    unreachable_pub,
    reason = "Leptos component visibility is required for composition from the parent app module"
)]
#[allow(
    clippy::needless_pass_by_value,
    reason = "Leptos props own page data so the generated component factory can move it across reactive render closures"
)]
pub(crate) fn AdminRulesView(
    admin_rules_page: server_admin_contract::admin_rules_page::AdminRulesPage,
    admin_csr_query: super::admin_csr_query::AdminCsrQuery,
) -> impl leptos::prelude::IntoView {
    let total = admin_rules_page.total();
    let rows = admin_rules_page.items().iter().map(|item| {
        let rule_path = server_admin_contract::admin_route_path::AdminRoutePath::from(item.id()).to_string();
        let id = item.id().to_string();
        let name = item.name().to_string();
        let created_at = item.created_at().to_string();
        leptos::view! {
            <crate::table_row::TableRow><crate::table_cell::TableCell data_label="id">{id}</crate::table_cell::TableCell><crate::table_cell::TableCell data_label="name">{name}</crate::table_cell::TableCell><crate::table_cell::TableCell data_label=constants_str::CREATED_AT>{created_at}</crate::table_cell::TableCell><crate::table_cell::TableCell data_label=constants_str::ADMIN_UI_ACTIONS bool=true><singlestage::Link class=crate::admin_button_variant::AdminButtonVariant::Secondary.class() href=rule_path attr:aria-label=constants_str::PG_CRUD_READ_RULE_ACTION attr:title=constants_str::PG_CRUD_READ_RULE_ACTION>
                        <svg viewBox="0 0 24 24" aria-hidden=constants_str::TRUE fill="none" stroke="currentColor" stroke-width="2">
                            <path d="M2 12s3.5-7 10-7 10 7 10 7-3.5 7-10 7S2 12 2 12Z"></path>
                            <circle cx="12" cy="12" r="3"></circle>
                        </svg>
                    </singlestage::Link></crate::table_cell::TableCell></crate::table_row::TableRow>
        }
    }).collect::<Vec<_>>();
    let identifier_filters = [
        frontend_contract::filter_operation::FilterOperation::Eq,
        frontend_contract::filter_operation::FilterOperation::GreaterThan,
        frontend_contract::filter_operation::FilterOperation::Between,
        frontend_contract::filter_operation::FilterOperation::In,
    ]
    .map(server_admin_contract::admin_data_filter::AdminDataFilter::from);
    let identifier = server_admin_contract::admin_text::AdminText::try_from(
        constants_str::SQL_NAMES_ID.to_owned(),
    )
    .ok();
    let rules_path = server_admin_contract::admin_data_table::AdminDataTable::Rules.frontend_path();
    let identifier_filter = identifier.as_ref().map(|identifier| {
        crate::admin_column_filter::admin_column_filter(
            &rules_path,
            identifier,
            frontend_contract::input_kind::InputKind::Number,
            &identifier_filters,
            admin_csr_query.filter_field(),
            admin_csr_query.filter_operation(),
            admin_csr_query.filter_value(),
            admin_csr_query.filter_end(),
            admin_csr_query.limit(),
        )
    });
    let text_filters = [
        frontend_contract::filter_operation::FilterOperation::Eq,
        frontend_contract::filter_operation::FilterOperation::In,
    ]
    .map(server_admin_contract::admin_data_filter::AdminDataFilter::from);
    let name =
        server_admin_contract::admin_text::AdminText::try_from(constants_str::NAME.to_owned()).ok();
    let name_filter = name.as_ref().map(|name| {
        crate::admin_column_filter::admin_column_filter(
            &rules_path,
            name,
            frontend_contract::input_kind::InputKind::Text,
            &text_filters,
            admin_csr_query.filter_field(),
            admin_csr_query.filter_operation(),
            admin_csr_query.filter_value(),
            admin_csr_query.filter_end(),
            admin_csr_query.limit(),
        )
    });
    leptos::view! {
        <section class="table-admin_rules_page" data-renderer="csr">
            <crate::table_wrapper::TableWrapper><crate::table::Table><crate::table_header::TableHeader><crate::table_row::TableRow><crate::table_head::TableHead data_field=constants_str::SQL_NAMES_ID.to_owned() data_filter_count=identifier_filters.len().to_string()><div class="table-column-heading"><span>{constants_str::SQL_NAMES_ID}</span>{identifier_filter}</div></crate::table_head::TableHead><crate::table_head::TableHead data_field=constants_str::NAME.to_owned() data_filter_count=text_filters.len().to_string()><div class="table-column-heading"><span>{constants_str::NAME}</span>{name_filter}</div></crate::table_head::TableHead><crate::table_head::TableHead>{constants_str::CREATED_AT}</crate::table_head::TableHead><crate::table_head::TableHead>{constants_str::ADMIN_UI_ACTIONS}</crate::table_head::TableHead></crate::table_row::TableRow></crate::table_header::TableHeader>
            <crate::table_body::TableBody>{rows}</crate::table_body::TableBody></crate::table::Table></crate::table_wrapper::TableWrapper>
            <super::admin_pagination::AdminPagination admin_frontend_path=server_admin_contract::admin_frontend_path::AdminFrontendPath::Rules admin_csr_query=admin_csr_query admin_page_total=total />
        </section>
    }
}
