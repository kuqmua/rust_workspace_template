pub(crate) async fn assignment_action<Ids, Parse, Request, BuildRequest, Target, Run, RunFuture>(
    auth: crate::AdminAuthReq,
    expected: &crate::AdminHtmlFormText,
    selected: crate::StdAdminHtmlSelected,
    parse: Parse,
    path: server_admin_contract::domain_types::AdminFrontendPath,
    build_request: BuildRequest,
    target: Target,
    run: Run,
) -> axum::response::Response
where
    Parse: Fn(&crate::AdminHtmlFormText) -> Result<Ids, crate::AdminError>,
    BuildRequest: FnOnce(Ids, Ids) -> Request,
    Run: FnOnce(crate::AdminAuthReq, Target, crate::AxumAdminJson<Request>) -> RunFuture,
    RunFuture: Future<Output = Result<crate::AxumAdminResponse, crate::AdminError>>,
{
    let (auth, expected, selected) =
        match crate::authenticated_selected_form_impl::authenticated_selected_form_impl(
            auth, expected, selected, parse,
        ) {
            Ok(values) => values,
            Err(error) => return axum::response::IntoResponse::into_response(error),
        };
    crate::action_result_impl::action_result_impl(
        run(
            auth,
            target,
            crate::AxumAdminJson(build_request(expected, selected)),
        )
        .await,
        path,
    )
}
