#![allow(clippy::field_scoped_visibility_modifiers)] // the proc-macro entry module parses and consumes these domain models

#[derive(optimal_memory_layout::OptimalMemoryLayout, newtype::FromInner, newtype::AsRefOwned)]
pub(crate) struct SynExpr(syn::Expr);

#[derive(optimal_memory_layout::OptimalMemoryLayout, newtype::FromInner, newtype::AsRefOwned)]
pub(crate) struct SynType(syn::Type);

#[derive(optimal_memory_layout::OptimalMemoryLayout, newtype::FromInner, newtype::AsRefOwned)]
pub(crate) struct SynIdent(syn::Ident);

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Default,
    newtype::FromInner,
    newtype::IntoInnerFrom,
)]
pub(crate) struct StdBool(bool);

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, newtype::FromInner)]
pub(crate) struct SynAttributesRef<'attributes_lt>(&'attributes_lt [syn::Attribute]);

#[derive(optimal_memory_layout::OptimalMemoryLayout, Default)]
pub(crate) struct ContractStructApiArgs {
    pub(crate) into_parts: StdBool,
    pub(crate) new: StdBool,
}

#[derive(optimal_memory_layout::OptimalMemoryLayout, Default)]
#[allow(
    clippy::struct_excessive_bools,
    reason = "each flag independently opts one field into a distinct generated method"
)]
pub(crate) struct ContractStructApiFieldArgs {
    pub(crate) slice: Option<SynType>,
    pub(crate) borrow: StdBool,
    pub(crate) copy: StdBool,
    pub(crate) copy_ref: StdBool,
    pub(crate) into: StdBool,
    pub(crate) option_borrow: StdBool,
}

#[derive(optimal_memory_layout::OptimalMemoryLayout)]
pub(crate) struct RouteCatalogArgs {
    pub(crate) body_limit: SynExpr,
    pub(crate) family: SynIdent,
}

#[derive(optimal_memory_layout::OptimalMemoryLayout)]
pub(crate) struct RouteCatalogRouteArgs {
    pub(crate) contract: Option<SynExpr>,
    pub(crate) path: Option<SynExpr>,
    pub(crate) route: Option<SynType>,
    pub(crate) exclude_from_family: StdBool,
}

#[derive(optimal_memory_layout::OptimalMemoryLayout)]
pub(crate) struct PageCatalogArgs {
    pub(crate) inventory: SynIdent,
    pub(crate) path_ref: SynIdent,
    pub(crate) spec: SynIdent,
}

#[derive(optimal_memory_layout::OptimalMemoryLayout)]
pub(crate) struct PageCatalogPageArgs {
    pub(crate) capability: SynExpr,
    pub(crate) metadata: SynExpr,
    pub(crate) path: SynExpr,
    pub(crate) route: SynExpr,
    pub(crate) title: SynExpr,
}

#[derive(optimal_memory_layout::OptimalMemoryLayout)]
pub(crate) struct TypedRouteArgs {
    pub(crate) authentication: SynExpr,
    pub(crate) error_response: Option<SynType>,
    pub(crate) errors: SynTypedRouteErrors,
    pub(crate) method: SynExpr,
    pub(crate) mutation: Option<SynExpr>,
    pub(crate) obligations: Option<SynExpr>,
    pub(crate) openapi_operation_id: SynExpr,
    pub(crate) path: SynExpr,
    pub(crate) path_parameter: Option<SynType>,
    pub(crate) request: SynType,
    pub(crate) request_body: Option<SynExpr>,
    pub(crate) response: SynType,
    pub(crate) success_status: SynExpr,
    pub(crate) transport: SynType,
}

#[derive(optimal_memory_layout::OptimalMemoryLayout)]
pub(crate) enum SynTypedRouteErrors {
    Policy(SynExpr),
    Statuses(SynExpr),
}

#[derive(optimal_memory_layout::OptimalMemoryLayout)]
pub(crate) struct RouteRegistryBinding {
    pub(crate) handler: SynRouteRegistryHandler,
    pub(crate) route: SynRouteRegistryRoute,
}

#[derive(optimal_memory_layout::OptimalMemoryLayout, newtype::FromInner, newtype::AsRefOwned)]
pub(crate) struct SynRouteRegistryHandler(syn::Path);

#[derive(optimal_memory_layout::OptimalMemoryLayout, newtype::FromInner, newtype::AsRefOwned)]
pub(crate) struct SynRouteRegistryRoute(syn::Type);

#[derive(optimal_memory_layout::OptimalMemoryLayout, newtype::FromInner, newtype::AsRefOwned)]
pub(crate) struct SynRouteRegistryBindings(
    syn::punctuated::Punctuated<RouteRegistryBinding, syn::Token![,]>,
);

#[derive(optimal_memory_layout::OptimalMemoryLayout, newtype::FromInner, newtype::AsRefOwned)]
pub(crate) struct SynRouteRegistrySchemas(Vec<syn::Type>);

#[derive(optimal_memory_layout::OptimalMemoryLayout, newtype::FromInner, newtype::AsRefOwned)]
pub(crate) struct SynRouteRegistryState(syn::Type);

#[derive(optimal_memory_layout::OptimalMemoryLayout, newtype::FromInner, newtype::AsRefOwned)]
pub(crate) struct SynRouteRegistryFamily(syn::Type);

#[derive(optimal_memory_layout::OptimalMemoryLayout)]
pub(crate) struct HandlerRegistryBinding {
    pub(crate) contract: SynHandlerRegistryContract,
    pub(crate) handler: SynHandlerRegistryHandler,
}

#[derive(optimal_memory_layout::OptimalMemoryLayout, newtype::FromInner, newtype::AsRefOwned)]
pub(crate) struct SynHandlerRegistryContract(syn::Expr);

#[derive(optimal_memory_layout::OptimalMemoryLayout, newtype::FromInner, newtype::AsRefOwned)]
pub(crate) struct SynHandlerRegistryHandler(syn::Path);

#[derive(optimal_memory_layout::OptimalMemoryLayout, newtype::FromInner, newtype::AsRefOwned)]
pub(crate) struct SynHandlerRegistryBindings(
    syn::punctuated::Punctuated<HandlerRegistryBinding, syn::Token![,]>,
);

#[derive(optimal_memory_layout::OptimalMemoryLayout, newtype::FromInner, newtype::AsRefOwned)]
pub(crate) struct SynHandlerRegistryState(syn::Type);

#[derive(optimal_memory_layout::OptimalMemoryLayout)]
pub(crate) struct HandlerRegistryArgs {
    pub(crate) bindings: SynHandlerRegistryBindings,
    pub(crate) state: SynHandlerRegistryState,
}

#[derive(optimal_memory_layout::OptimalMemoryLayout)]
pub(crate) struct RouteRegistryArgs {
    pub(crate) authenticated_security: SynExpr,
    pub(crate) bindings: SynRouteRegistryBindings,
    pub(crate) csrf_security: SynExpr,
    pub(crate) family: SynRouteRegistryFamily,
    pub(crate) schemas: SynRouteRegistrySchemas,
    pub(crate) state: SynRouteRegistryState,
}

impl SynExpr {
    pub(crate) fn into_inner(self) -> syn::Expr {
        self.0
    }
}

impl SynType {
    #[allow(clippy::single_call_fn)] // this conversion keeps the wrapped syn type private at the proc-macro boundary
    pub(crate) fn into_inner(self) -> syn::Type {
        self.0
    }
}

impl SynIdent {
    pub(crate) fn into_inner(self) -> syn::Ident {
        self.0
    }
}

impl StdBool {
    pub(crate) const fn get(self) -> bool {
        self.0
    }
}

impl<'attributes_lt> SynAttributesRef<'attributes_lt> {
    pub(crate) const fn get(self) -> &'attributes_lt [syn::Attribute] {
        self.0
    }
}

impl SynRouteRegistrySchemas {
    pub(crate) fn into_inner(self) -> Vec<syn::Type> {
        self.0
    }
}

impl SynRouteRegistryState {
    pub(crate) fn into_inner(self) -> syn::Type {
        self.0
    }
}

impl SynRouteRegistryFamily {
    pub(crate) fn into_inner(self) -> syn::Type {
        self.0
    }
}

impl SynHandlerRegistryState {
    pub(crate) fn into_inner(self) -> syn::Type {
        self.0
    }
}
