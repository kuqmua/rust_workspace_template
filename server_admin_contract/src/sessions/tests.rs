#[test]
fn session_page_preserves_items_and_total() {
    let page = super::AdminSessionsPage::new(
        crate::AdminSessionViews::try_from(Vec::new()).expect("c31f90a6"),
        crate::AdminPageTotal::from(3u64),
    );
    assert!(page.items().is_empty());
    assert_eq!(u64::from(page.total()), 3u64);
}
