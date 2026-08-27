use super::{AdminSsrHtml, render_view};

#[cfg(test)]
pub(super) trait AdminSsrViewExt {
    fn render_admin_ssr(self) -> AdminSsrHtml;
}
#[cfg(test)]
impl<View> AdminSsrViewExt for View
where
    View: leptos::prelude::IntoAny,
{
    fn render_admin_ssr(self) -> AdminSsrHtml {
        render_view(self)
    }
}
