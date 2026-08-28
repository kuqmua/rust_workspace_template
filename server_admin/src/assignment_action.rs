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
    let (auth, expected, selected) = match (|| {
        let auth = crate::form_auth_impl::form_auth_impl(auth)?;
        let expected = parse(expected)?;
        let separator = constants_str::COMMA_SPACE.trim();
        let selected = bounded_types::BoundedBTreeMap::<
            crate::AdminHtmlFormKey,
            crate::AdminHtmlFormText,
            { crate::ADMIN_HTML_FORM_SELECTED_MAX_ITEMS },
        >::from(selected);
        let capacity = selected
            .iter()
            .map(|(_key, value)| value.len().get())
            .sum::<usize>()
            .saturating_add(
                selected
                    .len()
                    .get()
                    .saturating_sub(constants_usize::ONE)
                    .saturating_mul(separator.len()),
            );
        let text = selected.into_values().enumerate().fold(
            String::with_capacity(capacity),
            |mut text, (index, value)| {
                if index > constants_usize::ZERO {
                    text.push_str(separator);
                }
                text.push_str(value.as_ref());
                text
            },
        );
        let selected_ids = crate::AdminHtmlFormText::try_from(text)
            .map_err(|_error| crate::AdminError::Validation)
            .and_then(|value| parse(&value))?;
        Ok::<_, crate::AdminError>((auth, expected, selected_ids))
    })() {
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
