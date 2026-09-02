#[cfg(test)]
pub(super) trait AdminSsrViewExt {
    fn render_admin_ssr(self) -> crate::admin_ssr_html::AdminSsrHtml;
}
#[cfg(test)]
impl<View> AdminSsrViewExt for View
where
    View: leptos::prelude::IntoAny,
{
    fn render_admin_ssr(self) -> crate::admin_ssr_html::AdminSsrHtml {
        crate::render_view::render_view(self)
    }
}
