#[test]
fn test_session_contract_tests() {
    let page = crate::admin_sessions_page::AdminSessionsPage::new(
        crate::admin_session_views::AdminSessionViews::try_from(Vec::new())
            .expect("c31f90a6 session_page_preserves_items_and_total invariant must hold"),
        crate::admin_page_total::AdminPageTotal::from(3u64),
    );
    assert!(page.items().is_empty());
    assert_eq!(u64::from(page.total()), 3u64);
}
