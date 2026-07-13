#[derive(Debug, Clone, Copy)]
struct CompileErrorMsg<'msg_lt>(&'msg_lt str);
fn compile_error_ts(
    msg: CompileErrorMsg<'_>,
) -> macros_helpers::generated_rust_ts::GeneratedRustTs {
    let msg_value = msg.0;
    macros_helpers::generated_rust_ts::GeneratedRustTs::from(
        quote::quote! {compile_error!(#msg_value);},
    )
}
//todo decide wh to do er log (mb add in some places)
//todo gen route what will return cols of the tbl and their rust and postgersql types
//todo crd at and updd at fields + crd by + updd by
//todo attrs for activation generation crud methods(like gen cr, uo, dlo)
//todo authorization for returning concrete er or just minimal info(user role)
//todo gen rules and roles
//todo mb add unnest sql types?
//todo mb add unnest to flt params if its arr ?
//todo swagger ui https://github.com/juhaku/utoipa/blob/master/examples/todo-axum/src/main.rs
//todo derive utoipa::ToSchema for what? original structs or with serialize deserialize?
//todo need to add utoipa::ToSchema ann #[schema(value_type = YourToSchemaTraitImplStruct)] for all fields
//todo remove useless derives like useless serde::Serialize and serde::Deserialize
//todo mb gen compisite type for user defined type https://docs.rs/sqlx/0.7.3/sqlx/pg/types/index.html#rust_decimal
//todo rd again some interesting thoughts about sql as api https://habr.com/ru/companies/timeweb/articles/798937/
//todo reexport all crates what logic depends on (from crates.io) (use of undclared crate or module `time`)
//todo add transaction isolation level (see pg docs)
//todo check on pg max len value of type
//todo in few cases rows affected is usefull. (upd del for example). if 0 afftected -mb its er? or mb use sel then upd\del?(rewrite query)
//todo gen json schema from rust type https://docs.rs/schemars/laTest/schemars/
//todo support rd tbl len
//todo what is pub what is private
//todo header Retry-After logic
#[must_use]
#[allow(non_snake_case)]
#[allow(unused_variables)]
pub fn gen_pg_tbl(
    input: macros_helpers::ts_writer::ProcMacro2TsRef<'_>,
) -> macros_helpers::generated_rust_ts::GeneratedRustTs {
    #[allow(clippy::arbitrary_source_item_ordering)]
    #[derive(Debug, optml::Optml)]
    struct SynVrt {
        vrt: syn::Variant,
        status_code: Option<macros_helpers::status_code::StatusCode>,
    }
    impl SynVrt {
        const fn get_opt_status_code(&self) -> Option<&macros_helpers::status_code::StatusCode> {
            self.status_code.as_ref()
        }
        const fn get_syn_vrt(&self) -> &syn::Variant {
            &self.vrt
        }
    }
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
    enum AddReturn {
        False,
        True,
    }
    #[allow(clippy::arbitrary_source_item_ordering)]
    #[derive(
        Debug,
        Clone,
        Copy,
        naming_macros::AsRefStrEnumWithUnitFieldsToUccStr,
        naming_macros::AsRefStrEnumWithUnitFieldsToScStr,
        optml::Optml,
    )]
    enum Op {
        Cm,
        Co,
        Rm,
        Ro,
        Um,
        Uo,
        Dm,
        Dlo,
    }
    impl Op {
        const fn derive_clone_and_copy(
            self,
        ) -> (
            macros_helpers::derive_ts_builder::DClone,
            macros_helpers::derive_ts_builder::DCopy,
        ) {
            match self {
                Self::Cm | Self::Co | Self::Rm | Self::Ro | Self::Um | Self::Uo | Self::Dm => (
                    macros_helpers::derive_ts_builder::DClone::False,
                    macros_helpers::derive_ts_builder::DCopy::False,
                ),
                Self::Dlo => (
                    macros_helpers::derive_ts_builder::DClone::True,
                    macros_helpers::derive_ts_builder::DCopy::True,
                ),
            }
        }
        const fn derive_prms_clone_and_copy(
            self,
        ) -> (
            macros_helpers::derive_ts_builder::DClone,
            macros_helpers::derive_ts_builder::DCopy,
        ) {
            match self {
                Self::Co | Self::Dlo => (
                    macros_helpers::derive_ts_builder::DClone::True,
                    macros_helpers::derive_ts_builder::DCopy::True,
                ),
                Self::Cm | Self::Rm | Self::Ro | Self::Um | Self::Uo | Self::Dm => (
                    macros_helpers::derive_ts_builder::DClone::False,
                    macros_helpers::derive_ts_builder::DCopy::False,
                ),
            }
        }
        const fn desirable_status_code(self) -> macros_helpers::status_code::StatusCode {
            match self {
                Self::Cm | Self::Co => macros_helpers::status_code::StatusCode::Crd201,
                Self::Rm | Self::Ro | Self::Um | Self::Uo | Self::Dm | Self::Dlo => {
                    macros_helpers::status_code::StatusCode::Ok200
                }
            }
        }
        const fn gen_pg_tbl_attr_er_vrts(self) -> GenPgTblAttr {
            match self {
                Self::Cm => GenPgTblAttr::CmErVrts,
                Self::Co => GenPgTblAttr::CoErVrts,
                Self::Rm => GenPgTblAttr::RmErVrts,
                Self::Ro => GenPgTblAttr::RoErVrts,
                Self::Um => GenPgTblAttr::UmErVrts,
                Self::Uo => GenPgTblAttr::UoErVrts,
                Self::Dm => GenPgTblAttr::DmErVrts,
                Self::Dlo => GenPgTblAttr::DloErVrts,
            }
        }
        const fn gen_pg_tbl_attr_logic(self) -> GenPgTblAttr {
            match self {
                Self::Cm => GenPgTblAttr::CmLogic,
                Self::Co => GenPgTblAttr::CoLogic,
                Self::Rm => GenPgTblAttr::RmLogic,
                Self::Ro => GenPgTblAttr::RoLogic,
                Self::Um => GenPgTblAttr::UmLogic,
                Self::Uo => GenPgTblAttr::UoLogic,
                Self::Dm => GenPgTblAttr::DmLogic,
                Self::Dlo => GenPgTblAttr::DloLogic,
            }
        }
        const fn http_method(self) -> OpHttpMethod {
            match self {
                Self::Cm | Self::Co | Self::Rm | Self::Ro => OpHttpMethod::Post,
                Self::Um | Self::Uo => OpHttpMethod::Patch,
                Self::Dm | Self::Dlo => OpHttpMethod::Delete,
            }
        }
        fn op_er_with_serde_sc(self) -> naming::prm::SelfErWithSerdeSc {
            naming::prm::SelfErWithSerdeSc::from_display(&self)
        }
        fn op_payload_example_sc(self) -> impl naming::DisplayPlusToTokens {
            naming::prm::SelfPayloadExampleSc::from_display(&self)
        }
        fn self_h_sc_ts(self) -> proc_macro2::TokenStream {
            let v = naming::prm::SelfHSc::from_tokens(&self.self_sc_ts());
            quote::quote! {#v}
        }
        fn self_sc_str(self) -> String {
            naming_cmn::AsRefStrToScStr::case(&self)
        }
        fn self_sc_ts(self) -> proc_macro2::TokenStream {
            let ident = quote::format_ident!("{}", self.self_sc_str());
            quote::quote! {#ident}
        }
        fn try_self_h_sc_ts(self) -> proc_macro2::TokenStream {
            let v = naming::prm::TrySelfHSc::from_tokens(&self.self_sc_ts());
            quote::quote! {#v}
        }
        fn try_self_sc_ts(self) -> proc_macro2::TokenStream {
            let v = naming::prm::TrySelfSc::from_tokens(&self.self_sc_ts());
            quote::quote! {#v}
        }
    }
    impl std::fmt::Display for Op {
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
    impl From<&CrOrUpdOrDm> for Op {
        fn from(v: &CrOrUpdOrDm) -> Self {
            match &v {
                CrOrUpdOrDm::Cr => Self::Cm,
                CrOrUpdOrDm::Upd => Self::Um,
                CrOrUpdOrDm::Del => Self::Dm,
            }
        }
    }
    impl From<&RmOrDm> for Op {
        fn from(v: &RmOrDm) -> Self {
            match &v {
                RmOrDm::Rm => Self::Rm,
                RmOrDm::Dm => Self::Dm,
            }
        }
    }
    impl From<&RmOrRo> for Op {
        fn from(v: &RmOrRo) -> Self {
            match &v {
                RmOrRo::Rm => Self::Rm,
                RmOrRo::Ro => Self::Ro,
            }
        }
    }
    impl From<&CrOrUpdOrDlo> for Op {
        fn from(v: &CrOrUpdOrDlo) -> Self {
            match &v {
                CrOrUpdOrDlo::Cr => Self::Co,
                CrOrUpdOrDlo::Upd => Self::Uo,
                CrOrUpdOrDlo::Del => Self::Dlo,
            }
        }
    }
    #[allow(clippy::arbitrary_source_item_ordering)]
    #[derive(Clone, Copy, naming_macros::AsRefStrEnumWithUnitFieldsToScStr, optml::Optml)]
    enum OpHttpMethod {
        Post,
        Patch,
        Delete,
    }
    #[derive(Clone, Copy)]
    struct OpDsc {
        http_method: OpHttpMethod,
        op: Op,
        success_status_code: macros_helpers::status_code::StatusCode,
    }
    impl OpDsc {
        const ALL: [Self; 8] = [
            Self::from_op(Op::Cm),
            Self::from_op(Op::Co),
            Self::from_op(Op::Rm),
            Self::from_op(Op::Ro),
            Self::from_op(Op::Um),
            Self::from_op(Op::Uo),
            Self::from_op(Op::Dm),
            Self::from_op(Op::Dlo),
        ];
        const fn from_op(op: Op) -> Self {
            Self {
                http_method: op.http_method(),
                success_status_code: op.desirable_status_code(),
                op,
            }
        }
    }
    #[allow(clippy::arbitrary_source_item_ordering)]
    enum RmOrDm {
        Rm,
        Dm,
    }
    enum RmOrRo {
        Rm,
        Ro,
    }
    #[allow(clippy::arbitrary_source_item_ordering)]
    #[derive(
        Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, strum_macros::Display, optml::Optml,
    )]
    enum GenPgTblAttr {
        CmErVrts,
        CoErVrts,
        RmErVrts,
        RoErVrts,
        UmErVrts,
        UoErVrts,
        DmErVrts,
        DloErVrts,
        CmnErVrts,
        CmLogic,
        CoLogic,
        RmLogic,
        RoLogic,
        UmLogic,
        UoLogic,
        DmLogic,
        DloLogic,
        CmnLogic,
    }
    impl GenPgTblAttr {
        fn gen_path_to_attr(self) -> String {
            let attr_name: &dyn std::fmt::Display = match self {
                Self::CmErVrts => &naming::CmErVrtsSc,
                Self::CoErVrts => &naming::CoErVrtsSc,
                Self::RmErVrts => &naming::RmErVrtsSc,
                Self::RoErVrts => &naming::RoErVrtsSc,
                Self::UmErVrts => &naming::UmErVrtsSc,
                Self::UoErVrts => &naming::UoErVrtsSc,
                Self::DmErVrts => &naming::DmErVrtsSc,
                Self::DloErVrts => &naming::DloErVrtsSc,
                Self::CmnErVrts => &naming::CmnErVrtsSc,
                Self::CmLogic => &naming::CmLogicSc,
                Self::CoLogic => &naming::CoLogicSc,
                Self::RmLogic => &naming::RmLogicSc,
                Self::RoLogic => &naming::RoLogicSc,
                Self::UmLogic => &naming::UmLogicSc,
                Self::UoLogic => &naming::UoLogicSc,
                Self::DmLogic => &naming::DmLogicSc,
                Self::DloLogic => &naming::DloLogicSc,
                Self::CmnLogic => &naming::CmnLogicSc,
            };
            format!("gen_pg_tbl::{attr_name}")
        }
    }
    enum ShouldWrapIntoV {
        False,
        True,
    }
    #[allow(clippy::arbitrary_source_item_ordering)]
    enum CrOrUpdOrDm {
        Cr,
        Upd,
        Del,
    }
    #[allow(clippy::arbitrary_source_item_ordering)]
    enum CrOrUpdOrDlo {
        Cr,
        Upd,
        Del,
    }
    #[allow(clippy::arbitrary_source_item_ordering)]
    #[derive(Debug, serde::Deserialize, optml::Optml)]
    struct GenPgTblConfig {
        #[serde(default)]
        cm_max_items: Option<StdBulkItemsMax>,
        #[serde(default)]
        um_max_items: Option<StdBulkItemsMax>,
        tests_write_into_file: macros_helpers::ts_writer::ShouldWriteTsIntoFile,
        cmn_write_into_file: macros_helpers::ts_writer::ShouldWriteTsIntoFile,
        whole_write_into_file: macros_helpers::ts_writer::ShouldWriteTsIntoFile,
    }
    #[derive(Clone, Copy, Debug, serde::Deserialize)]
    struct StdBulkItemsMax(usize);
    struct GenPgTblInputModel {
        config: GenPgTblConfig,
        er_vrts_by_attr: std::collections::BTreeMap<GenPgTblAttr, Vec<GenPgTblVariantModel>>,
        logic_ts_by_attr: std::collections::BTreeMap<GenPgTblAttr, proc_macro2::TokenStream>,
    }
    struct ProcMacro2GenPgTblTestsTs(proc_macro2::TokenStream);
    impl ProcMacro2GenPgTblTestsTs {
        const fn as_ref(&self) -> &proc_macro2::TokenStream {
            &self.0
        }
        fn into_inner(self) -> proc_macro2::TokenStream {
            self.0
        }
    }
    struct ProcMacro2GenPgTblCmnTs(proc_macro2::TokenStream);
    impl ProcMacro2GenPgTblCmnTs {
        const fn as_ref(&self) -> &proc_macro2::TokenStream {
            &self.0
        }
    }
    struct ProcMacro2GenPgTblWholeTs(proc_macro2::TokenStream);
    impl ProcMacro2GenPgTblWholeTs {
        const fn as_ref(&self) -> &proc_macro2::TokenStream {
            &self.0
        }
        fn into_inner(self) -> proc_macro2::TokenStream {
            self.0
        }
    }
    struct SynGenPgTblDeriveInput(syn::DeriveInput);
    impl SynGenPgTblDeriveInput {
        const fn get(&self) -> &syn::DeriveInput {
            &self.0
        }
    }
    struct GenPgTblFieldModel {
        field: macros_helpers::field_data::SynField,
        is_pk: bool,
    }
    struct GenPgTblVariantFieldModel {
        ident: syn::Ident,
        loc_attr: Option<macros_helpers::loc_data::LocFieldAttr>,
        type0: syn::Type,
    }
    struct GenPgTblVariantModel {
        fields: Vec<GenPgTblVariantFieldModel>,
        ident: syn::Ident,
    }
    #[derive(Clone, Copy)]
    enum GenPgTblVariantRef<'variant_lt> {
        Model(&'variant_lt GenPgTblVariantModel),
        Syn(&'variant_lt syn::Variant),
    }
    impl<'variant_lt> GenPgTblVariantRef<'variant_lt> {
        const fn ident(self) -> &'variant_lt syn::Ident {
            match self {
                Self::Model(v) => &v.ident,
                Self::Syn(v) => &v.ident,
            }
        }
    }
    #[derive(Clone, Copy)]
    struct GenPgTblFieldIdx(usize);
    impl From<usize> for GenPgTblFieldIdx {
        fn from(value: usize) -> Self {
            Self(value)
        }
    }
    impl GenPgTblFieldIdx {
        const fn get(self) -> usize {
            self.0
        }
    }
    struct GenPgTblFieldsModel {
        fields: Vec<macros_helpers::field_data::SynField>,
        fields_without_pk_idxs: Vec<GenPgTblFieldIdx>,
        pk_field_idx: GenPgTblFieldIdx,
    }
    #[derive(Clone, Copy)]
    struct SynGenPgTblFieldRef<'field_lt>(&'field_lt syn::Field);
    impl<'field_lt> From<&'field_lt syn::Field> for SynGenPgTblFieldRef<'field_lt> {
        fn from(value: &'field_lt syn::Field) -> Self {
            Self(value)
        }
    }
    impl<'field_lt> SynGenPgTblFieldRef<'field_lt> {
        const fn get(self) -> &'field_lt syn::Field {
            self.0
        }
    }
    #[derive(Clone, Copy)]
    struct SynGenPgTblIdentRef<'ident_lt>(&'ident_lt syn::Ident);
    impl<'ident_lt> From<&'ident_lt syn::Ident> for SynGenPgTblIdentRef<'ident_lt> {
        fn from(value: &'ident_lt syn::Ident) -> Self {
            Self(value)
        }
    }
    impl<'ident_lt> SynGenPgTblIdentRef<'ident_lt> {
        const fn get(self) -> &'ident_lt syn::Ident {
            self.0
        }
    }
    #[derive(Clone, Copy)]
    struct SynGenPgTblTypeRef<'type_lt>(&'type_lt syn::Type);
    impl<'type_lt> From<&'type_lt syn::Type> for SynGenPgTblTypeRef<'type_lt> {
        fn from(value: &'type_lt syn::Type) -> Self {
            Self(value)
        }
    }
    impl<'type_lt> SynGenPgTblTypeRef<'type_lt> {
        const fn get(self) -> &'type_lt syn::Type {
            self.0
        }
    }
    #[derive(Clone, Copy)]
    struct GenPgTblVariantLocAttr(Option<macros_helpers::loc_data::LocFieldAttr>);
    impl From<Option<macros_helpers::loc_data::LocFieldAttr>> for GenPgTblVariantLocAttr {
        fn from(value: Option<macros_helpers::loc_data::LocFieldAttr>) -> Self {
            Self(value)
        }
    }
    impl GenPgTblVariantLocAttr {
        const fn get(self) -> Option<macros_helpers::loc_data::LocFieldAttr> {
            self.0
        }
    }
    #[derive(Clone, Copy)]
    struct GenPgTblPkAttrName<'name_lt>(&'name_lt str);
    impl<'name_lt> GenPgTblPkAttrName<'name_lt> {
        const fn get(self) -> &'name_lt str {
            self.0
        }
    }
    #[allow(clippy::single_call_fn)]
    fn parse_gen_pg_tbl_input_stage(
        input: macros_helpers::ts_writer::ProcMacro2TsRef<'_>,
    ) -> Result<SynGenPgTblDeriveInput, macros_helpers::generated_rust_ts::GeneratedRustTs> {
        match syn::parse2(input.as_ref().clone()) {
            Ok(v) => Ok(SynGenPgTblDeriveInput(v)),
            Err(er) => Err(macros_helpers::generated_rust_ts::GeneratedRustTs::from(
                er.to_compile_error(),
            )),
        }
    }
    #[allow(clippy::single_call_fn)]
    fn gen_pg_tbl_field_model_stage(
        field_ref: SynGenPgTblFieldRef<'_>,
        pk_attr_name: GenPgTblPkAttrName<'_>,
    ) -> Result<GenPgTblFieldModel, macros_helpers::generated_rust_ts::GeneratedRustTs> {
        let syn_field = field_ref.get();
        let Some(fi) = syn_field.ident.clone() else {
            return Err(compile_error_ts(CompileErrorMsg(
                "915ef2ce: expected named field ident",
            )));
        };
        let fi_len = fi.to_string().len();
        let max_pg_col_len = 63;
        if fi_len > max_pg_col_len {
            return Err(compile_error_ts(CompileErrorMsg(
                "1266ae5a: field ident is longer than PostgreSQL column name limit",
            )));
        }
        let field = macros_helpers::field_data::SynField {
            vis: macros_helpers::field_data::SynFieldVis::from(syn_field.vis.clone()),
            type0: macros_helpers::field_data::SynFieldType::from(syn_field.ty.clone()),
            ident: macros_helpers::field_data::SynFieldIdent::from(fi),
        };
        let is_pk = syn_field
            .attrs
            .iter()
            .filter(|el0| el0.path().segments.len() == 1)
            .any(|el0| {
                el0.path()
                    .segments
                    .first()
                    .is_some_and(|first_segment| first_segment.ident == pk_attr_name.get())
            });
        Ok(GenPgTblFieldModel { field, is_pk })
    }
    #[allow(clippy::single_call_fn)]
    fn gen_pg_tbl_variant_field_model_stage(
        syn_field: syn::Field,
    ) -> Result<GenPgTblVariantFieldModel, macros_helpers::generated_rust_ts::GeneratedRustTs> {
        let Some(ident) = syn_field.ident else {
            return Err(compile_error_ts(CompileErrorMsg(
                "ae8e173b: expected named variant field ident",
            )));
        };
        let parsed_loc_attr = if ident == naming::LocSc.to_string() {
            None
        } else {
            let mut loc_attrs = syn_field.attrs.iter().filter_map(|el| {
                if el.path().segments.len() != 1 {
                    return None;
                }
                let segment = el.path().segments.first()?;
                <macros_helpers::loc_data::LocFieldAttr as std::str::FromStr>::from_str(
                    &segment.ident.to_string(),
                )
                .ok()
            });
            let loc_attr = loc_attrs.next();
            if loc_attrs.next().is_some() {
                return Err(compile_error_ts(CompileErrorMsg(
                    "9a4d65c9: duplicate loc field attr",
                )));
            }
            let Some(parsed_loc_attr) = loc_attr else {
                return Err(compile_error_ts(CompileErrorMsg(
                    "8af68998: loc field attr not found",
                )));
            };
            Some(parsed_loc_attr)
        };
        Ok(GenPgTblVariantFieldModel {
            ident,
            loc_attr: parsed_loc_attr,
            type0: syn_field.ty,
        })
    }
    fn gen_pg_tbl_syn_field_loc_attr_stage(
        field_ref: SynGenPgTblFieldRef<'_>,
    ) -> Result<
        Option<macros_helpers::loc_data::LocFieldAttr>,
        macros_helpers::generated_rust_ts::GeneratedRustTs,
    > {
        let field = field_ref.get();
        let Some(fi) = field.ident.as_ref() else {
            return Err(compile_error_ts(CompileErrorMsg(
                "a21dc807: expected named field ident",
            )));
        };
        if *fi == naming::LocSc.to_string() {
            return Ok(None);
        }
        let mut loc_attrs = field.attrs.iter().filter_map(|el| {
            if el.path().segments.len() != 1 {
                return None;
            }
            let segment = el.path().segments.first()?;
            <macros_helpers::loc_data::LocFieldAttr as std::str::FromStr>::from_str(
                &segment.ident.to_string(),
            )
            .ok()
        });
        let loc_attr = loc_attrs.next();
        if loc_attrs.next().is_some() {
            return Err(compile_error_ts(CompileErrorMsg(
                "9a469d36: duplicate loc field attr",
            )));
        }
        Ok(loc_attr)
    }
    #[allow(clippy::single_call_fn)]
    fn gen_pg_tbl_variant_model_stage(
        syn_vrt: syn::Variant,
    ) -> Result<GenPgTblVariantModel, macros_helpers::generated_rust_ts::GeneratedRustTs> {
        let syn::Fields::Named(fields_named) = syn_vrt.fields else {
            return Err(compile_error_ts(CompileErrorMsg(
                "1be4a6e2: expected named variant fields",
            )));
        };
        let fields_len = fields_named.named.len();
        let fields = fields_named.named.into_iter().try_fold(
            Vec::with_capacity(fields_len),
            |mut acc, field| {
                acc.push(gen_pg_tbl_variant_field_model_stage(field)?);
                Ok::<
                    Vec<GenPgTblVariantFieldModel>,
                    macros_helpers::generated_rust_ts::GeneratedRustTs,
                >(acc)
            },
        )?;
        Ok(GenPgTblVariantModel {
            fields,
            ident: syn_vrt.ident,
        })
    }
    #[allow(clippy::single_call_fn)]
    fn build_gen_pg_tbl_fields_model_stage(
        input: &SynGenPgTblDeriveInput,
        pk_attr_name: GenPgTblPkAttrName<'_>,
    ) -> Result<GenPgTblFieldsModel, macros_helpers::generated_rust_ts::GeneratedRustTs> {
        if let syn::Data::Struct(data_struct) = &input.get().data {
            if let syn::Fields::Named(fields_named) = &data_struct.fields {
                let fields_acc = fields_named.named.iter().try_fold(
                    (
                        None,
                        Vec::with_capacity(fields_named.named.len()),
                        Vec::with_capacity(fields_named.named.len()),
                    ),
                    |(mut opt_pk_field, mut fields, mut fields_without_pk), el| {
                        let field_model = gen_pg_tbl_field_model_stage(
                            SynGenPgTblFieldRef::from(el),
                            pk_attr_name,
                        )?;
                        let field_idx = GenPgTblFieldIdx::from(fields.len());
                        if field_model.is_pk {
                            if opt_pk_field.is_some() {
                                return Err(compile_error_ts(CompileErrorMsg(
                                    "1a75cea1: duplicate primary key field",
                                )));
                            }
                            opt_pk_field = Some(field_idx);
                        } else {
                            fields_without_pk.push(field_idx);
                        }
                        fields.push(field_model.field);
                        Ok((opt_pk_field, fields, fields_without_pk))
                    },
                );
                let (opt_pk_field, fields, fields_without_pk_idxs) = fields_acc?;
                let Some(pk_field_idx) = opt_pk_field else {
                    return Err(compile_error_ts(CompileErrorMsg(
                        "6a529a99: primary key field not found",
                    )));
                };
                Ok(GenPgTblFieldsModel {
                    fields,
                    fields_without_pk_idxs,
                    pk_field_idx,
                })
            } else {
                Err(compile_error_ts(CompileErrorMsg(
                    "7f31872d: expected named struct fields",
                )))
            }
        } else {
            Err(compile_error_ts(CompileErrorMsg(
                "bd4718d0: expected struct input",
            )))
        }
    }
    #[allow(clippy::single_call_fn)]
    fn build_gen_pg_tbl_input_model_stage(
        input: &SynGenPgTblDeriveInput,
    ) -> Result<GenPgTblInputModel, macros_helpers::generated_rust_ts::GeneratedRustTs> {
        let di = input.get();
        let config = match serde_json::from_str::<GenPgTblConfig>(
            &macros_helpers::attr_reader::get_macro_attr_meta_list_ts(
                &di.attrs,
                "gen_pg_tbl::gen_pg_tbl_config",
            )
            .to_string(),
        ) {
            Ok(v) => v,
            Err(er) => {
                let msg = format!("failed to parse GenPgTblConfig: {er}");
                return Err(macros_helpers::generated_rust_ts::GeneratedRustTs::from(
                    quote::quote! { compile_error!(#msg); },
                ));
            }
        };
        if config
            .cm_max_items
            .into_iter()
            .chain(config.um_max_items)
            .any(|limit| limit.0 == 0usize)
        {
            return Err(compile_error_ts(CompileErrorMsg(
                "536203b7: bulk item limit must be greater than zero",
            )));
        }
        let er_vrts_by_attr = [
            GenPgTblAttr::CmErVrts,
            GenPgTblAttr::CoErVrts,
            GenPgTblAttr::RmErVrts,
            GenPgTblAttr::RoErVrts,
            GenPgTblAttr::UmErVrts,
            GenPgTblAttr::UoErVrts,
            GenPgTblAttr::DmErVrts,
            GenPgTblAttr::DloErVrts,
            GenPgTblAttr::CmnErVrts,
        ]
        .into_iter()
        .try_fold(
            std::collections::BTreeMap::new(),
            |mut acc, gen_pg_tbl_attr| {
                let gen_pg_tbl_attr_str = gen_pg_tbl_attr.to_string();
                let cmn_er_vrts_attr_ts = macros_helpers::attr_reader::get_macro_attr_meta_list_ts(
                    &di.attrs,
                    &gen_pg_tbl_attr.gen_path_to_attr(),
                );
                let Ok(parsed_di): Result<syn::DeriveInput, _> =
                    syn::parse2((*cmn_er_vrts_attr_ts).clone())
                else {
                    return Ok(acc);
                };
                if parsed_di.ident != gen_pg_tbl_attr_str {
                    return Err(compile_error_ts(CompileErrorMsg(
                        "8a66c852: error variant attr ident does not match attr name",
                    )));
                }
                if let syn::Data::Enum(data_enum) = parsed_di.data {
                    let variants_len = data_enum.variants.len();
                    let vrts = data_enum.variants.into_iter().try_fold(
                        Vec::with_capacity(variants_len),
                        |mut variants_acc, variant| {
                            variants_acc.push(gen_pg_tbl_variant_model_stage(variant)?);
                            Ok::<
                                Vec<GenPgTblVariantModel>,
                                macros_helpers::generated_rust_ts::GeneratedRustTs,
                            >(variants_acc)
                        },
                    )?;
                    drop(acc.insert(gen_pg_tbl_attr, vrts));
                }
                Ok(acc)
            },
        )?;
        let logic_ts_by_attr = [
            GenPgTblAttr::CmLogic,
            GenPgTblAttr::CoLogic,
            GenPgTblAttr::RmLogic,
            GenPgTblAttr::RoLogic,
            GenPgTblAttr::UmLogic,
            GenPgTblAttr::UoLogic,
            GenPgTblAttr::DmLogic,
            GenPgTblAttr::DloLogic,
            GenPgTblAttr::CmnLogic,
        ]
        .into_iter()
        .map(|gen_pg_tbl_attr| {
            let logic_ts = macros_helpers::attr_reader::get_macro_attr_meta_list_ts(
                &di.attrs,
                &gen_pg_tbl_attr.gen_path_to_attr(),
            );
            (gen_pg_tbl_attr, (*logic_ts).clone())
        })
        .collect::<std::collections::BTreeMap<GenPgTblAttr, proc_macro2::TokenStream>>();
        Ok(GenPgTblInputModel {
            config,
            er_vrts_by_attr,
            logic_ts_by_attr,
        })
    }
    #[allow(clippy::single_call_fn)]
    fn validate_gen_pg_tbl_fields_model_stage(
        model: GenPgTblFieldsModel,
    ) -> Result<GenPgTblFieldsModel, macros_helpers::generated_rust_ts::GeneratedRustTs> {
        if model.fields.get(model.pk_field_idx.get()).is_none() {
            return Err(compile_error_ts(CompileErrorMsg(
                "878d3f9b: primary key field index not found",
            )));
        }
        if model
            .fields_without_pk_idxs
            .iter()
            .any(|idx| model.fields.get(idx.get()).is_none())
        {
            return Err(compile_error_ts(CompileErrorMsg(
                "22bc6672: non-primary-key field index not found",
            )));
        }
        Ok(model)
    }
    #[allow(clippy::single_call_fn)]
    fn emit_gen_pg_tbl_tests_stage(
        config: &GenPgTblConfig,
        tests_ts: ProcMacro2GenPgTblTestsTs,
    ) -> ProcMacro2GenPgTblTestsTs {
        macros_helpers::ts_writer::mb_write_ts_into_file(
            config.tests_write_into_file,
            "gen_pg_tbl_Tests",
            macros_helpers::ts_writer::ProcMacro2TsRef::from(tests_ts.as_ref()),
            &macros_helpers::ts_writer::FormatWithCargofmt::True,
        );
        match config.tests_write_into_file {
            macros_helpers::ts_writer::ShouldWriteTsIntoFile::False => {
                ProcMacro2GenPgTblTestsTs(proc_macro2::TokenStream::new())
            }
            macros_helpers::ts_writer::ShouldWriteTsIntoFile::True => tests_ts,
        }
    }
    #[allow(clippy::single_call_fn)]
    fn emit_gen_pg_tbl_final_stage(
        config: &GenPgTblConfig,
        cmn_ts: &ProcMacro2GenPgTblCmnTs,
        whole_ts: ProcMacro2GenPgTblWholeTs,
    ) -> macros_helpers::generated_rust_ts::GeneratedRustTs {
        macros_helpers::ts_writer::mb_write_ts_into_file(
            config.cmn_write_into_file,
            "gen_pg_tbl_cmn",
            macros_helpers::ts_writer::ProcMacro2TsRef::from(cmn_ts.as_ref()),
            &macros_helpers::ts_writer::FormatWithCargofmt::True,
        );
        macros_helpers::ts_writer::mb_write_ts_into_file(
            config.whole_write_into_file,
            "gen_pg_tbl",
            macros_helpers::ts_writer::ProcMacro2TsRef::from(whole_ts.as_ref()),
            &macros_helpers::ts_writer::FormatWithCargofmt::True,
        );
        macros_helpers::generated_rust_ts::GeneratedRustTs::from(whole_ts.into_inner())
    }
    panic_loc::panic_loc();
    let import = pg_crud_macros_cmn::Import::PgCrudCmn;
    let import_ts = quote::quote! {#import::};
    let return_err_qp_er_write_into_buffer_ts =
        pg_crud_macros_cmn::gen_return_err_qp_er_write_into_buffer_ts(import);
    let parsed_input = match parse_gen_pg_tbl_input_stage(input) {
        Ok(v) => v,
        Err(er) => return er,
    };
    let di = parsed_input.get();
    let gen_pg_tbl_input_model = match build_gen_pg_tbl_input_model_stage(&parsed_input) {
        Ok(v) => v,
        Err(er) => return er,
    };
    let AllowClippyArbitrarySrcItemOrdering = token_patterns::AllowClippyArbitrarySrcItemOrdering;
    let AppStateSc = naming::AppStateSc;
    let BeginSc = naming::BeginSc;
    let BindedQuerySc = naming::BindedQuerySc;
    let BodyBytesSc = naming::BodyBytesSc;
    let BodySc = naming::BodySc;
    let BodySizeErUcc = naming::BodySizeErUcc;
    let Bool = token_patterns::Bool;
    let BySc = naming::BySc;
    let Char = token_patterns::Char;
    let CheckBodySizeSc = naming::CheckBodySizeSc;
    let CheckBodySizeUcc = naming::CheckBodySizeUcc;
    let CmErVrtsSc = naming::CmErVrtsSc;
    let CmLogicSc = naming::CmLogicSc;
    let CmnErVrtsSc = naming::CmnErVrtsSc;
    let CmnLogicSc = naming::CmnLogicSc;
    let CmnRdIdsFromCoSc = naming::CmnRdIdsFromCoSc;
    let CoErVrtsSc = naming::CoErVrtsSc;
    let CoLogicSc = naming::CoLogicSc;
    let ColSc = naming::ColSc;
    let ColsSc = naming::ColsSc;
    let CommitSc = naming::CommitSc;
    let ConfigSc = naming::ConfigSc;
    let CoreDefault = token_patterns::CoreDefault;
    let CrExtensionIfNotExistsUuidOsspUcc = naming::CrExtensionIfNotExistsUuidOsspUcc;
    let CrQbSc = naming::CrQbSc;
    let CrQpSc = naming::CrQpSc;
    let CrSc = naming::CrSc;
    let CrTblColQpSc = naming::CrTblColQpSc;
    let CrUcc = naming::CrUcc;
    let DeResUcc = naming::DeResUcc;
    let DeriveDebugSerdeSerializeSerdeDeserialize =
        token_patterns::DeriveDebugSerdeSerializeSerdeDeserialize;
    let DeriveDebugThiserrorLoc = token_patterns::DeriveDebugThiserrorLoc;
    let DesirableUcc = naming::DesirableUcc;
    let DfltSomeOneElMaxPageSizeSc = naming::DfltSomeOneElMaxPageSizeSc;
    let DfltSomeOneElMaxPageSizeUcc = naming::DfltSomeOneElMaxPageSizeUcc;
    let DfltSomeOneElSc = naming::DfltSomeOneElSc;
    let DfltSomeOneElUcc = naming::DfltSomeOneElUcc;
    let DloErVrtsSc = naming::DloErVrtsSc;
    let DloLogicSc = naming::DloLogicSc;
    let DmErVrtsSc = naming::DmErVrtsSc;
    let DmLogicSc = naming::DmLogicSc;
    let ElSc = naming::ElSc;
    let EndpointLocSc = naming::EndpointLocSc;
    let Er0 = token_patterns::Er0;
    let Er1 = token_patterns::Er1;
    let Er2 = token_patterns::Er2;
    let Er3 = token_patterns::Er3;
    let ErSc = naming::ErSc;
    let ExecutorAcquireSc = naming::ExecutorAcquireSc;
    let ExecutorSc = naming::ExecutorSc;
    let ExpectedResSc = naming::ExpectedResSc;
    let ExtraPrmsSc = naming::ExtraPrmsSc;
    let F32 = token_patterns::F32;
    let F64 = token_patterns::F64;
    let FailedToGetResTextUcc = naming::FailedToGetResTextUcc;
    let FalseSc = naming::FalseSc;
    let FieldAttrSerdeSkipSerializingIfOptIsNone =
        token_patterns::FieldAttrSerdeSkipSerializingIfOptIsNone;
    let FromHSc = naming::FromHSc;
    let FutureSc = naming::FutureSc;
    let GenColQuealsVCommaUoQpSc = naming::GenColQuealsVCommaUoQpSc;
    let GenPgTblPkSc = naming::GenPgTblPkSc;
    let GenSelQpSc = naming::GenSelQpSc;
    let GenWhenColIdThenVUmQpSc = naming::GenWhenColIdThenVUmQpSc;
    let HeaderContentTypeAppJsonNotFoundUcc = naming::HeaderContentTypeAppJsonNotFoundUcc;
    let HeadersSc = naming::HeadersSc;
    let I8 = token_patterns::I8;
    let I16 = token_patterns::I16;
    let I32 = token_patterns::I32;
    let I64 = token_patterns::I64;
    let IdentCrDfltSc = naming::IdentCrDfltSc;
    let IncrSc = naming::IncrSc;
    let IntoSerdeVersionSc = naming::IntoSerdeVersionSc;
    let LocSc = naming::LocSc;
    let MustUse = token_patterns::MustUse;
    let NoFieldsProvidedUcc = naming::NoFieldsProvidedUcc;
    let NotUnqFieldSc = naming::NotUnqFieldSc;
    let NotUnqFieldUcc = naming::NotUnqFieldUcc;
    let NotUnqPkSc = naming::NotUnqPkSc;
    let NotUnqPkUcc = naming::NotUnqPkUcc;
    let OptVecCrSc = naming::OptVecCrSc;
    let OrderBySc = naming::OrderBySc;
    let OrderByUcc = naming::OrderByUcc;
    let OrderSc = naming::OrderSc;
    let PayloadSc = naming::PayloadSc;
    let PayloadUcc = naming::PayloadUcc;
    let PgCrudCmnDfltSomeOneEl = token_patterns::PgCrudCmnDfltSomeOneEl;
    let PgCrudCmnDfltSomeOneElCall = token_patterns::PgCrudCmnDfltSomeOneElCall;
    let PgCrudCmnDfltSomeOneElMaxPageSizeCall =
        token_patterns::PgCrudCmnDfltSomeOneElMaxPageSizeCall;
    let PgCrudSc = "pg_crud_cmn";
    let PgPoolForTokioSpawnSyncMoveSc = naming::PgPoolForTokioSpawnSyncMoveSc;
    let PgPoolSc = naming::PgPoolSc;
    let PgSc = naming::PgSc;
    let PgTypeOptVecWhGreaterThanTestSc = naming::PgTypeOptVecWhGreaterThanTestSc;
    let PgTypeUcc = naming::PgTypeUcc;
    let PgUcc = naming::PgUcc;
    let PgnSc = naming::PgnSc;
    let PkQpSc = naming::PkQpSc;
    let PkSc = naming::PkSc;
    let PoolConnectionSc = naming::PoolConnectionSc;
    let PoolSc = naming::PoolSc;
    let PrefixSc = naming::PrefixSc;
    let PrepExtensionsSc = naming::PrepExtensionsSc;
    let PrepPgSc = naming::PrepPgSc;
    let PrepPgTblSc = naming::PrepPgTblSc;
    let PrepPgUcc = naming::PrepPgUcc;
    let PrmsSc = naming::PrmsSc;
    let QbSc = naming::QbSc;
    let QpErUcc = naming::QpErUcc;
    let QpSc = naming::QpSc;
    let QpUcc = naming::QpUcc;
    let QuerySc = naming::QuerySc;
    let QueryStringSc = naming::QueryStringSc;
    let RdIdsAndCrIntoOptVecWhEqToFieldSc = naming::RdIdsAndCrIntoOptVecWhEqToFieldSc;
    let RdIdsAndCrIntoVecWhEqUsingFieldsSc = naming::RdIdsAndCrIntoVecWhEqUsingFieldsSc;
    let RdIdsAndCrIntoWhEqSc = naming::RdIdsAndCrIntoWhEqSc;
    let RdIdsAndTtIntoPgTypeOptWhGreaterThanSc = naming::RdIdsAndTtIntoPgTypeOptWhGreaterThanSc;
    let RdIdsIntoRdSc = naming::RdIdsIntoRdSc;
    let RdIdsIntoTtSc = naming::RdIdsIntoTtSc;
    let RdIdsIntoUpdSc = naming::RdIdsIntoUpdSc;
    let RdIdsSc = naming::RdIdsSc;
    let RdIdsUcc = naming::RdIdsUcc;
    let RdIntoTtSc = naming::RdIntoTtSc;
    let RdUcc = naming::RdUcc;
    let RefStr = token_patterns::RefStr;
    let ReqSc = naming::ReqSc;
    let ReqwestSc = naming::ReqwestSc;
    let ReqwestUcc = naming::ReqwestUcc;
    let ResSc = naming::ResSc;
    let ResTextSc = naming::ResTextSc;
    let RmErVrtsSc = naming::RmErVrtsSc;
    let RmLogicSc = naming::RmLogicSc;
    let RoErVrtsSc = naming::RoErVrtsSc;
    let RoLogicSc = naming::RoLogicSc;
    let RollbackSc = naming::RollbackSc;
    let RoutesHSc = naming::RoutesHSc;
    let RoutesSc = naming::RoutesSc;
    let RowAndRollbackUcc = naming::RowAndRollbackUcc;
    let RowSc = naming::RowSc;
    let RowsSc = naming::RowsSc;
    let SelOnlyIdsQpSc = naming::SelOnlyIdsQpSc;
    let SelOnlyUpddIdsQpSc = naming::SelOnlyUpddIdsQpSc;
    let SelPkSc = naming::SelPkSc;
    let SelQpSc = naming::SelQpSc;
    let SelSc = naming::SelSc;
    let SelUcc = naming::SelUcc;
    let SerdeJsonSc = naming::SerdeJsonSc;
    let SerdeJsonToStringSc = naming::SerdeJsonToStringSc;
    let SerdeJsonToStringUcc = naming::SerdeJsonToStringUcc;
    let SerdeJsonUcc = naming::SerdeJsonUcc;
    let SerdeSc = naming::SerdeSc;
    let SqlxAcquire = token_patterns::SqlxAcquire;
    let SqlxRow = token_patterns::SqlxRow;
    let StatusCodeSc = naming::StatusCodeSc;
    let StringTs = token_patterns::StringTs;
    let TblNameSc = naming::TblNameSc;
    let TblSc = naming::TblSc;
    let TrueSc = naming::TrueSc;
    let TryBindSc = naming::TryBindSc;
    let TryBindUcc = naming::TryBindUcc;
    let U8 = token_patterns::U8;
    let U16 = token_patterns::U16;
    let U32 = token_patterns::U32;
    let U64 = token_patterns::U64;
    let UmErVrtsSc = naming::UmErVrtsSc;
    let UmLogicSc = naming::UmLogicSc;
    let UoErVrtsSc = naming::UoErVrtsSc;
    let UoLogicSc = naming::UoLogicSc;
    let UpdForQuerySc = naming::UpdForQuerySc;
    let UpdForQueryUcc = naming::UpdForQueryUcc;
    let UpdForQueryVecSc = naming::UpdForQueryVecSc;
    let UpdQbSc = naming::UpdQbSc;
    let UpdQpPkSc = naming::UpdQpPkSc;
    let UpdQpSc = naming::UpdQpSc;
    let UpdSc = naming::UpdSc;
    let UpdUcc = naming::UpdUcc;
    let UrlSc = naming::UrlSc;
    let VSc = naming::VSc;
    let VUcc = naming::VUcc;
    let WhManySc = naming::WhManySc;
    let WhUcc = naming::WhUcc;
    let ident = &di.ident;
    let ident_sc_string = naming_cmn::ToTokensToScStr::case(&ident);
    let ident_sc_dq_ts = gen_quotes::dq_ts(&ident_sc_string);
    let self_tbl_name_call_ts = quote::quote! {Self::#TblNameSc()};
    let gen_pg_tbl_pk_sc_str = GenPgTblPkSc.to_string();
    let fields_model = match build_gen_pg_tbl_fields_model_stage(
        &parsed_input,
        GenPgTblPkAttrName(gen_pg_tbl_pk_sc_str.as_str()),
    ) {
        Ok(v) => v,
        Err(er) => return er,
    };
    let validated_fields_model = match validate_gen_pg_tbl_fields_model_stage(fields_model) {
        Ok(v) => v,
        Err(er) => return er,
    };
    let GenPgTblFieldsModel {
        fields,
        fields_without_pk_idxs,
        pk_field_idx,
    } = validated_fields_model;
    let fields_len = fields.len();
    let fields_len_without_pk = fields_without_pk_idxs.len();
    let Some(pk_field) = fields.get(pk_field_idx.get()) else {
        return compile_error_ts(CompileErrorMsg(
            "878d3f9b: primary key field index not found",
        ));
    };
    let fields_without_pk_iter = || {
        fields_without_pk_idxs
            .iter()
            .filter_map(|field_idx| fields.get(field_idx.get()))
    };
    let pk_ft = &pk_field.type0;
    if fields_without_pk_idxs.is_empty() {
        return macros_helpers::generated_rust_ts::GeneratedRustTs::from(
            syn::Error::new_spanned(
                &**pk_ft,
                "09a11adc: update operations require at least one non-primary-key field",
            )
            .into_compile_error(),
        );
    }
    if let syn::Type::Path(type_path) = &**pk_ft
        && let Some(last_segment) = type_path.path.segments.last()
    {
        let pk_type_name = last_segment.ident.to_string();
        if pk_type_name.starts_with("Opt") || pk_type_name.contains("AsNl") {
            return macros_helpers::generated_rust_ts::GeneratedRustTs::from(
                syn::Error::new_spanned(
                    &**pk_ft,
                    "d3b03ca2: primary key type must be non-nullable",
                )
                .into_compile_error(),
            );
        }
    }
    //todo must remove this and use trait type instead
    let pk_ft_tt_ts = naming::prm::SelfTtUcc::from_type_last_segment(&pk_field.type0);
    let gen_as_pg_type_ts = |ts: &dyn quote::ToTokens| {
        quote::quote! {<#ts as #import_ts #PgTypeUcc>}
    };
    let gen_as_pg_type_path_ts = |ts: &dyn quote::ToTokens| {
        let ts0 = gen_as_pg_type_ts(ts);
        quote::quote! {#ts0::}
    };
    let pk_ft_as_pg_type_ts = gen_as_pg_type_path_ts(&pk_ft);
    let pk_ft_as_pg_type_rd_ucc = quote::quote! {#pk_ft_as_pg_type_ts #RdUcc};
    let pk_as_pg_type_ts = gen_as_pg_type_ts(&pk_ft);
    let gen_as_pg_type_tokens_ts = |ts: &dyn quote::ToTokens, tokens: &dyn quote::ToTokens| {
        let as_pg_type_ts = gen_as_pg_type_path_ts(&ts);
        quote::quote! {#as_pg_type_ts #tokens}
    };
    let gen_concrete_pg_type_role_ts =
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
    let gen_concrete_stdrt_nn_pg_type_role_ts =
        |field_type: &macros_helpers::field_data::SynFieldType, role: &dyn quote::ToTokens| {
            if let syn::Type::Path(type_path) = &**field_type {
                let mut role_type_path = type_path.clone();
                if let Some(last_segment) = role_type_path.path.segments.last_mut() {
                    let ident_string = last_segment.ident.to_string();
                    let without_opt = ident_string
                        .strip_prefix("Opt")
                        .map_or(ident_string.as_str(), |value| value);
                    last_segment.ident = quote::format_ident!(
                        "{}{}",
                        without_opt.replace("AsNl", "AsNn"),
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
    let gen_as_pg_type_test_cases_path_ts = |ts: &dyn quote::ToTokens| {
        quote::quote! {<#ts as #import_ts PgTypeTestCases>::}
    };
    let pk_as_pg_type_test_cases_path_ts = gen_as_pg_type_test_cases_path_ts(&pk_ft);
    let gen_as_pg_type_cr_ts = |ts: &dyn quote::ToTokens| gen_as_pg_type_tokens_ts(&ts, &CrUcc);
    let gen_as_pg_type_sel_ts = |ts: &dyn quote::ToTokens| gen_as_pg_type_tokens_ts(&ts, &SelUcc);
    let pk_ft_as_pg_type_sel_ts = gen_as_pg_type_sel_ts(&pk_ft);
    let gen_as_pg_type_wh_ts = |ts: &dyn quote::ToTokens| gen_as_pg_type_tokens_ts(&ts, &WhUcc);
    let pk_ft_as_pg_type_wh_ts = gen_as_pg_type_wh_ts(&pk_ft);
    let gen_as_pg_type_rd_ts = |ts: &dyn quote::ToTokens| gen_as_pg_type_tokens_ts(&ts, &RdUcc);
    let gen_as_pg_type_rd_ids_ts =
        |ts: &dyn quote::ToTokens| gen_as_pg_type_tokens_ts(&ts, &RdIdsUcc);
    let pk_ft_as_pg_type_rd_ts = gen_as_pg_type_rd_ts(&pk_ft);
    let gen_as_pg_type_upd_ts = |ts: &dyn quote::ToTokens| gen_as_pg_type_tokens_ts(&ts, &UpdUcc);
    let gen_as_pg_type_upd_for_query_ts =
        |ts: &dyn quote::ToTokens| gen_as_pg_type_tokens_ts(&ts, &UpdForQueryUcc);
    let ident_rd_ids_ucc = naming::prm::SelfRdIdsUcc::from_tokens(&ident);
    let ident_dm_prms_ucc = naming::prm::SelfDmPrmsUcc::from_tokens(&ident);
    let ident_dm_payload_ucc = naming::prm::SelfDmPayloadUcc::from_tokens(&ident);
    let ident_dlo_prms_ucc = naming::prm::SelfDloPrmsUcc::from_tokens(&ident);
    let ident_dlo_payload_ucc = naming::prm::SelfDloPayloadUcc::from_tokens(&ident);
    let ident_try_ro_er_ucc = naming::prm::SelfTryRoErUcc::from_tokens(&ident);
    let ident_ro_er_with_serde_ucc = naming::prm::SelfRoErWithSerdeUcc::from_tokens(&ident);
    let ident_try_dlo_er_ucc = naming::prm::SelfTryDloErUcc::from_tokens(&ident);
    let ident_dlo_er_with_serde_ucc = naming::prm::SelfDloErWithSerdeUcc::from_tokens(&ident);
    let vec_pk_ft_rd_ts = pg_crud_macros_cmn::gen_vec_tokens_dcl_ts(&pk_ft_as_pg_type_rd_ucc);
    let vec_ident_rd_ids_ts = pg_crud_macros_cmn::gen_vec_tokens_dcl_ts(&ident_rd_ids_ucc);
    let pk_fi = &pk_field.ident;
    let pk_fi_ucc_ts = naming_cmn::ToTokensToUccTs::case_or_panic(&pk_fi);
    let pk_ft_upd_ts = &naming::prm::SelfUpdUcc::from_type_last_segment(pk_ft);
    let pk_ft_upd_for_query_ts = &naming::prm::SelfUpdForQueryUcc::from_type_last_segment(pk_ft);
    let ident_sel_ucc = naming::prm::SelfSelUcc::from_tokens(&ident);
    let gen_from_h_ts = |ident_ts: &dyn quote::ToTokens, ts: &dyn quote::ToTokens| {
        quote::quote! {
            fn #FromHSc(#VSc: #ident_ts) -> Self {
                #ts
            }
        }
    };
    let gen_sel_pg_crud_not_empty_unq_vec_ident_sel_ts = |add_borrow: &AddBorrow| {
        quote::quote! {#SelSc: #add_borrow #import_ts NotEmptyUnqVec<#ident_sel_ucc>}
    };
    let sel_borrow_pg_crud_not_empty_unq_vec_ident_sel_ts =
        gen_sel_pg_crud_not_empty_unq_vec_ident_sel_ts(&AddBorrow::True);
    let sel_pg_crud_not_empty_unq_vec_ident_sel_ts =
        gen_sel_pg_crud_not_empty_unq_vec_ident_sel_ts(&AddBorrow::False);
    let pub_sel_pg_crud_not_empty_unq_vec_ident_sel_ts = {
        quote::quote! {pub #sel_pg_crud_not_empty_unq_vec_ident_sel_ts}
    };
    let gen_fields_named_with_comma_ts: &dyn Fn(
        &dyn Fn(&macros_helpers::field_data::SynField) -> proc_macro2::TokenStream,
    ) -> proc_macro2::TokenStream = &|fn0| -> proc_macro2::TokenStream {
        let fields_ts = fields.iter().map(fn0);
        quote::quote! {#(#fields_ts),*}
    };
    let gen_fields_named_without_comma_ts: &dyn Fn(
        &dyn Fn(&macros_helpers::field_data::SynField) -> proc_macro2::TokenStream,
    ) -> proc_macro2::TokenStream = &|fn0| -> proc_macro2::TokenStream {
        let fields_ts = fields.iter().map(fn0);
        quote::quote! {#(#fields_ts)*}
    };
    let gen_fields_named_without_pk_with_comma_ts: &dyn Fn(
        &dyn Fn(&macros_helpers::field_data::SynField) -> proc_macro2::TokenStream,
    ) -> proc_macro2::TokenStream = &|fn0| -> proc_macro2::TokenStream {
        let fields_ts = fields_without_pk_iter().map(fn0);
        quote::quote! {#(#fields_ts),*}
    };
    let gen_fields_named_without_pk_without_comma_ts: &dyn Fn(
        &dyn Fn(&macros_helpers::field_data::SynField) -> proc_macro2::TokenStream,
    ) -> proc_macro2::TokenStream = &|fn0| -> proc_macro2::TokenStream {
        let fields_ts = fields_without_pk_iter().map(fn0);
        quote::quote! {#(#fields_ts)*}
    };
    let gen_match_ok_err_ts = |ts0: &dyn quote::ToTokens,
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
    let gen_match_ok_err_short_ts =
        |expr: &dyn quote::ToTokens, ok: &dyn quote::ToTokens, err_ts: &dyn quote::ToTokens| {
            gen_match_ok_err_ts(&expr, &ok, &ok, &Er0, &quote::quote! {{ #err_ts }})
        };
    let none_ts = quote::quote! {None};
    let fields_named_with_comma_none_ts =
        gen_fields_named_with_comma_ts(&|_| -> proc_macro2::TokenStream { none_ts.clone() });
    let fields_named_without_pk_with_comma_none_ts =
        gen_fields_named_without_pk_with_comma_ts(&|_| -> proc_macro2::TokenStream {
            none_ts.clone()
        });
    let gen_acc_string_pop_ts = |acc_ts: &dyn quote::ToTokens, ts: &dyn quote::ToTokens| {
        let opt_char_ts = pg_crud_macros_cmn::gen_opt_type_dcl_ts(&Char);
        quote::quote! {
            let mut #acc_ts = #StringTs::new();
            #ts
            let _: #opt_char_ts = #acc_ts.pop();
        }
    };
    let gen_acc_string_pop_acc_ts = |acc_ts: &dyn quote::ToTokens, ts: &dyn quote::ToTokens| {
        let ts0 = gen_acc_string_pop_ts(acc_ts, ts);
        quote::quote! {
            #ts0
            #acc_ts
        }
    };
    let gen_acc_string_pop_ok_acc_ts = |acc_ts: &dyn quote::ToTokens, ts: &dyn quote::ToTokens| {
        let ts0 = gen_acc_string_pop_ts(acc_ts, ts);
        quote::quote! {
            #ts0
            Ok(#import_ts QpFragment::try_from(#acc_ts).unwrap_or_else(#import_ts QpFragment::from))
        }
    };
    let op_count = 8usize;
    let mut impl_ident_vec_ts = Vec::with_capacity(op_count.saturating_add(2));
    let mut op_routes_ts = Vec::with_capacity(op_count);
    let mut content_ts = Vec::with_capacity(op_count);
    let mut api_client_methods_ts = Vec::with_capacity(op_count);
    let client_sc = quote::format_ident!("client");
    let mut open_api_path_fn_idents = Vec::with_capacity(op_count);
    let mut open_api_schema_types_ts = Vec::with_capacity(op_count.saturating_mul(2));
    let er_enum_d_ts_builder = pg_crud_macros_cmn::ts_helpers::er_enum_d_ts_builder();
    let serde_ser_utoipa_d_ts_builder = macros_helpers::derive_ts_builder::DTsBuilder::new()
        .make_pub()
        .d_debug()
        .d_serde_serialize()
        .d_utoipa_to_schema();
    let ident_prep_pg_er_ucc = naming::prm::SelfPrepPgErUcc::from_tokens(&ident);
    let ident_prep_pg_er_ts = pg_crud_macros_cmn::ts_helpers::er_enum_d_ts_builder().build_enum(
        &proc_macro2::TokenStream::new(),
        &ident_prep_pg_er_ucc,
        &proc_macro2::TokenStream::new(),
        &{
            let ts = quote::quote! {
                #[eo_to_err_string]
                er: sqlx::Error,
                loc: loc_lib::loc::Loc,
            };
            quote::quote! {{
                #CrExtensionIfNotExistsUuidOsspUcc {
                    #ts
                },
                #PrepPgUcc {
                    #ts
                },
            }}
        },
    );
    impl_ident_vec_ts.push({
        let pub_fn_tbl_ts = quote::quote! {
            #MustUse
            pub const fn #TblNameSc() -> &'static str {
                #ident_sc_dq_ts
            }
        };
        let fn_pk_ts = {
            let pk_fi_dq_ts = gen_quotes::dq_ts(&pk_fi);
            quote::quote! {
                const fn #PkSc() -> &'static str {
                    #pk_fi_dq_ts
                }
            }
        };
        let pub_async_fn_prep_extensions_ts = quote::quote! {
            pub async fn #PrepExtensionsSc(#PoolSc: &sqlx::Pool<sqlx::Postgres>) -> Result<(), #ident_prep_pg_er_ucc> {
                if let Err(er) = sqlx::query("create extension if not exists \"uuid-ossp\"").execute(#PoolSc).await {
                    return Err(#ident_prep_pg_er_ucc::#CrExtensionIfNotExistsUuidOsspUcc {
                        er,
                        loc: loc_macros::loc!()
                    });
                }
                Ok(())
            }
        };
        let pub_async_fn_prep_pg_tbl_ts = {
            let prep_pg_cols_fmt = fields.iter().enumerate().fold(
                String::with_capacity(fields.len().saturating_mul(3)),
                |mut acc, (idx, _)| {
                    if idx != 0 {
                        acc.push(',');
                    }
                    acc.push_str("{}");
                    acc
                },
            );
            let prep_pg_dq_ts = gen_quotes::dq_ts(&format!(
                "create table if not exists {{tbl}} ({prep_pg_cols_fmt})"
            ));
            let gen_ft_as_pg_crud_cr_tbl_col_qp_cr_tbl_qp_ts = |ft, fi, is_pk| {
                let is_pk_ts: &dyn quote::ToTokens = if is_pk { &TrueSc } else { &FalseSc };
                let fi_dq_ts = gen_quotes::dq_ts(&fi);
                let ft_pg_type_ts = gen_as_pg_type_path_ts(&ft);
                quote::quote! {
                    #ft_pg_type_ts #CrTblColQpSc(#import_ts SqlColRef::from(&#fi_dq_ts), #import_ts IsPk::from(#is_pk_ts))
                }
            };
            let serde_json_to_string_schemars_schema_for_generic_unwrap_ts = std::iter::once(
                gen_ft_as_pg_crud_cr_tbl_col_qp_cr_tbl_qp_ts(pk_ft, &pk_field.ident, true),
            )
            .chain(fields_without_pk_iter().map(|el| {
                gen_ft_as_pg_crud_cr_tbl_col_qp_cr_tbl_qp_ts(&el.type0, &el.ident, false)
            }));
            quote::quote! {
                pub async fn #PrepPgTblSc(#PoolSc: &sqlx::Pool<sqlx::Postgres>, tbl: &str) -> Result<(), #ident_prep_pg_er_ucc> {
                    if let Err(er) = sqlx::query(&format!(
                        #prep_pg_dq_ts,
                        #(#serde_json_to_string_schemars_schema_for_generic_unwrap_ts),*
                    )).execute(#PoolSc).await {
                        return Err(#ident_prep_pg_er_ucc::#PrepPgUcc {
                            er,
                            loc: loc_macros::loc!()
                        });
                    }
                    Ok(())
                }
            }
        };
        let pub_async_fn_prep_pg_ts = quote::quote! {
            pub async fn #PrepPgSc(#PoolSc: &sqlx::Pool<sqlx::Postgres>) -> Result<(), #ident_prep_pg_er_ucc> {
                Self::#PrepExtensionsSc(#PoolSc).await?;
                Self::#PrepPgTblSc(#PoolSc, #ident_sc_dq_ts).await?;
                Ok(())
            }
        };
        let pub_fn_allow_methods_ts = {
            let http_method_ts = quote::quote! {http::Method};
            quote::quote! {
                #MustUse
                pub const fn allow_methods() -> [#http_method_ts;4] {[
                    #http_method_ts::GET,
                    #http_method_ts::POST,
                    #http_method_ts::PATCH,
                    #http_method_ts::DELETE
                ]}
            }
        };
        let fn_gen_sel_qp_ts = {
            let vrts_ts = gen_fields_named_with_comma_ts(&|el: &macros_helpers::field_data::SynField| {
                let fi_ucc_ts = naming_cmn::ToTokensToUccTs::case_or_panic(&el.ident);
                let init_ts = {
                    let fi_string_dq_ts = gen_quotes::dq_ts(&el.ident);
                    let as_pg_crud_pg_type_pg_type_ts = gen_as_pg_type_path_ts(&el.type0);
                    let ts0 = gen_match_ok_err_short_ts(
                        &quote::quote! {#as_pg_crud_pg_type_pg_type_ts #SelQpSc(
                            #ColSc,
                            #import_ts SqlColRef::from(&#fi_string_dq_ts)
                        )},
                        &quote::quote! {v_820e1163},
                        &quote::quote! {{
                            return Err(#Er0);
                        }},
                    );
                    quote::quote! {=> #ts0}
                };
                quote::quote! {#ident_sel_ucc::#fi_ucc_ts(#ColSc) #init_ts}
            });
            let ts0 = gen_acc_string_pop_ok_acc_ts(
                &quote::quote! {acc},
                &quote::quote! {
                    for el in #SelSc.to_vec() {
                        acc.push_str(&match el {
                            #vrts_ts
                        });
                        acc.push(',');
                    }
                },
            );
            quote::quote! {
                fn #GenSelQpSc(#sel_borrow_pg_crud_not_empty_unq_vec_ident_sel_ts) -> Result<#import_ts QpFragment, #import_ts #QpErUcc> {
                    #ts0
                }
            }
        };
        quote::quote! {
            #pub_fn_tbl_ts
            #fn_pk_ts
            #pub_async_fn_prep_extensions_ts
            #pub_async_fn_prep_pg_tbl_ts
            #pub_async_fn_prep_pg_ts
            #pub_fn_allow_methods_ts
            #fn_gen_sel_qp_ts
        }
    });
    let wrap_into_axum_res_ts = |axum_json_ts: &dyn quote::ToTokens,
                                 status_code_ts: &dyn quote::ToTokens,
                                 add_return: &AddReturn| {
        let return_ts = match add_return {
            AddReturn::False => quote::quote! {res},
            AddReturn::True => quote::quote! {return res;},
        };
        quote::quote! {
            let mut res = axum::response::IntoResponse::into_response(
                axum::Json(#axum_json_ts)
            );
            *res.status_mut() = #status_code_ts;
            #return_ts
        }
    };
    let gen_ident_op_suffix_ts: &dyn Fn(&Op, &str) -> proc_macro2::TokenStream = &|op, suffix| {
        let ident_op_suffix = quote::format_ident!("{ident}{op}{suffix}");
        quote::quote! {#ident_op_suffix}
    };
    let gen_ident_op_er_ucc = |op: &Op| gen_ident_op_suffix_ts(op, "Er");
    let gen_ident_op_res_vrts_ucc = |op: &Op| gen_ident_op_suffix_ts(op, "ResVrts");
    let gen_init_ts: &dyn Fn(
        &SynVrt,
        &'static std::panic::Location<'_>,
    ) -> proc_macro2::TokenStream = &|syn_vrt, loc| -> proc_macro2::TokenStream {
        let vrt_ident = &syn_vrt.vrt.ident;
        let fields_ts = if let syn::Fields::Named(v) = &syn_vrt.vrt.fields {
            v.named.iter().enumerate().map(|(i, el)| {
                let fi = &el.ident;
                let Some(fi_ref) = fi.as_ref() else {
                    return compile_error_ts(CompileErrorMsg(
                        "edbbd08a: expected named field ident",
                    ))
                    .into();
                };
                if *fi_ref == LocSc.to_string() {
                    macros_helpers::gen_field_loc_new_ts::gen_field_loc_new_ts(
                        macros_helpers::gen_field_loc_new_ts::FieldLocFile::from(loc.file()),
                        macros_helpers::gen_field_loc_new_ts::FieldLocLine::from(loc.line()),
                        macros_helpers::gen_field_loc_new_ts::FieldLocCol::from(loc.column()),
                    )
                    .into()
                } else {
                    let er_incr_sc = naming::prm::ErSelfSc::from_display(&i);
                    quote::quote! {#fi: #er_incr_sc}
                }
            })
        } else {
            return compile_error_ts(CompileErrorMsg("10773d36: expected named variant fields"))
                .into();
        };
        quote::quote! {
            #vrt_ident {
                #(#fields_ts),*
            }
        }
    };
    let gen_op_er_init_eprintln_res_ts: &dyn Fn(
        &Op,
        &SynVrt,
        &'static std::panic::Location<'_>,
    ) -> proc_macro2::TokenStream = &|op, syn_vrt, loc| -> proc_macro2::TokenStream {
        let ident_op_er_ucc = gen_ident_op_er_ucc(op);
        let ident_op_res_vrts_ucc = gen_ident_op_res_vrts_ucc(op);
        let syn_vrt_init_ts = gen_init_ts(syn_vrt, loc);
        let ts = wrap_into_axum_res_ts(
            &quote::quote! {#ident_op_res_vrts_ucc::#FromHSc(#ErSc)},
            &match syn_vrt.get_opt_status_code() {
                Some(v) => v.to_http_status_code_ts(),
                None => {
                    return compile_error_ts(CompileErrorMsg(
                        "81efa954: status code attr not found",
                    ))
                    .into();
                }
            },
            &AddReturn::True,
        );
        quote::quote! {
            let #ErSc = #ident_op_er_ucc::#syn_vrt_init_ts;
            #ts
        }
    };
    let new_syn_vrt = |vrt_name: &dyn std::fmt::Display,
                       status_code: Option<macros_helpers::status_code::StatusCode>,
                       vrt_fields: Vec<(
        macros_helpers::loc_data::LocFieldAttr,
        &dyn std::fmt::Display,
        macros_helpers::gen_simple_syn_punct::SynPathSegments,
    )>,
                       is_loc_first|
     -> SynVrt {
        SynVrt {
            vrt: syn::Variant {
                attrs: {
                    let mut attrs = Vec::with_capacity(1);
                    if let Some(v) = status_code.as_ref() {
                        let mut segments = syn::punctuated::Punctuated::new();
                        segments.push(syn::PathSegment {
                            ident: syn::Ident::new(
                                &naming_cmn::AsRefStrToScStr::case(v),
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
                ident: syn::Ident::new(&vrt_name.to_string(), proc_macro2::Span::call_site()),
                fields: syn::Fields::Named(syn::FieldsNamed {
                    brace_token: syn::token::Brace::default(),
                    named: {
                        let initial_fields = if is_loc_first {
                            let mut named_fields_acc = syn::punctuated::Punctuated::new();
                            named_fields_acc
                                .push_value(macros_helpers::loc_syn_field::loc_syn_field().into());
                            named_fields_acc.push_punct(syn::token::Comma {
                                spans: [proc_macro2::Span::call_site()],
                            });
                            named_fields_acc
                        } else {
                            syn::punctuated::Punctuated::new()
                        };
                        let mut named_fields_acc = vrt_fields.into_iter().fold(
                            initial_fields,
                            |mut named_fields_acc, el| {
                                named_fields_acc.push_value(syn::Field {
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
                                                        macros_helpers::attr_ident_str::AttrIdentStr::attr_ident_str(&el.0).as_ref(),
                                                        proc_macro2::Span::call_site(),
                                                    ),
                                                    arguments: syn::PathArguments::None,
                                                });
                                                acc0
                                            },
                                        }),
                                    }],
                                    vis: syn::Visibility::Inherited,
                                    mutability: syn::FieldMutability::None,
                                    ident: Some(syn::Ident::new(
                                        &el.1.to_string(),
                                        proc_macro2::Span::call_site(),
                                    )),
                                    colon_token: Some(syn::token::Colon {
                                        spans: [proc_macro2::Span::call_site()],
                                    }),
                                    ty: syn::Type::Path(syn::TypePath {
                                        qself: None,
                                        path: syn::Path {
                                            leading_colon: None,
                                            segments: el.2.into(),
                                        },
                                    }),
                                });
                                named_fields_acc.push_punct(syn::token::Comma {
                                    spans: [proc_macro2::Span::call_site()],
                                });
                                named_fields_acc
                            },
                        );
                        if !is_loc_first {
                            named_fields_acc
                                .push_value(macros_helpers::loc_syn_field::loc_syn_field().into());
                        }
                        named_fields_acc
                    },
                }),
                discriminant: None,
            },
            status_code,
        }
    };
    let qp_syn_vrt = new_syn_vrt(
        &QpUcc,
        Some(macros_helpers::status_code::StatusCode::BadReq400),
        vec![(
            macros_helpers::loc_data::LocFieldAttr::EoLoc,
            &ErSc,
            macros_helpers::gen_simple_syn_punct::gen_simple_syn_punct([
                &PgCrudSc.to_owned(),
                &QpErUcc.to_string(),
            ]),
        )],
        false,
    );
    let gen_sel_qp_prms_payload_sel_ts = |op: &Op| {
        gen_match_ok_err_short_ts(
            &quote::quote! {Self::#GenSelQpSc(&#PrmsSc.#PayloadSc.#SelSc)},
            &quote::quote! {v_357219fb},
            &{
                let ts =
                    gen_op_er_init_eprintln_res_ts(op, &qp_syn_vrt, std::panic::Location::caller());
                quote::quote! {{#ts}}
            },
        )
    };
    let ident_rd_ucc = naming::prm::SelfRdUcc::from_tokens(&ident);
    let gen_v_dcl_ts0 = |ts: &dyn quote::ToTokens| pg_crud_macros_cmn::gen_v_dcl_ts(&import, &ts);
    let gen_v_init_ts0 = |ts: &dyn quote::ToTokens| pg_crud_macros_cmn::gen_v_init_ts(&import, &ts);
    let gen_impl_pg_crud_dflt_some_one_el_for_tokens_no_lt_ts =
        |impl_ident: &dyn quote::ToTokens, ts: &dyn quote::ToTokens| {
            pg_crud_macros_cmn::gen_impl_pg_crud_dflt_some_one_el_ts(
                &impl_ident,
                &proc_macro2::TokenStream::new(),
                &ts,
            )
        };
    let gen_fi_dflt_some_one_el_call_ts =
        |ts: &dyn quote::ToTokens| quote::quote! {#ts: #PgCrudCmnDfltSomeOneElCall};
    let gen_match_qb_or_err_ts = |expr: &dyn quote::ToTokens,
                                  ok_binding: &dyn quote::ToTokens,
                                  err_ts: &dyn quote::ToTokens| {
        gen_match_ok_err_ts(
            &expr,
            &ok_binding,
            &quote::quote! {{
                #QuerySc = #ok_binding;
            }},
            &Er0,
            &quote::quote! {{#err_ts}},
        )
    };
    let gen_if_let_some_ts =
        |ts0: &dyn quote::ToTokens, ts1: &dyn quote::ToTokens, ts2: &dyn quote::ToTokens| {
            quote::quote! {
                if let Some(#ts0) = #ts1 {
                    #ts2
                }
            }
        };
    let ident_cr_ucc = naming::prm::SelfCrUcc::from_tokens(&ident);
    let ident_cr_ts = {
        let ident_cr_ts = macros_helpers::derive_ts_builder::DTsBuilder::new()
            .make_pub()
            .d_debug()
            .d_clone()
            .d_copy()
            .d_serde_serialize()
            .d_serde_deserialize()
            .d_utoipa_to_schema()
            .build_struct(
                &quote::quote! {#[serde(deny_unknown_fields)]},
                &ident_cr_ucc,
                &proc_macro2::TokenStream::new(),
                &{
                    let ts = gen_fields_named_without_pk_with_comma_ts(
                        &|el: &macros_helpers::field_data::SynField| {
                            let fi = &el.ident;
                            let el_syn_field_ty_as_pg_type_cr_ts = gen_as_pg_type_cr_ts(&el.type0);
                            let concrete_cr_ts = gen_concrete_pg_type_role_ts(&el.type0, &CrUcc);
                            quote::quote! {
                                #[schema(value_type = #concrete_cr_ts)]
                                pub #fi: #el_syn_field_ty_as_pg_type_cr_ts
                            }
                        },
                    );
                    quote::quote! {{#ts}}
                },
            );
        let impl_ident_cr_ts = {
            let pk_ft_as_dflt_some_one_el_call_ts = {
                let pk_ft_as_pg_type_cr_ts = gen_as_pg_type_cr_ts(&pk_ft);
                quote::quote! {
                    <
                        #pk_ft_as_pg_type_cr_ts as #import_ts #DfltSomeOneElUcc
                    >::#DfltSomeOneElSc()
                }
            };
            let fn_cr_qp_ts = {
                let gen_match_as_pg_crud_pg_type_pg_type_cr_qp_ts: &dyn Fn(
                    &dyn quote::ToTokens,
                    &dyn quote::ToTokens,
                ) -> proc_macro2::TokenStream = &|ft, ts| {
                        gen_match_ok_err_ts(
                            &{
                                let as_pg_crud_pg_type_pg_type_ts = gen_as_pg_type_path_ts(&ft);
                                quote::quote! {#as_pg_crud_pg_type_pg_type_ts #CrQpSc(
                                    &#ts,
                                    #IncrSc
                                )}
                            },
                            &quote::quote! {v_c3f0b59a},
                            &{
                                let if_write_is_err_ts = macros_helpers::gen_if_write_is_err_ts::gen_if_write_is_err_ts(
                                    &quote::quote! {acc, "{v_c3f0b59a},"},
                                    &return_err_qp_er_write_into_buffer_ts,
                                );
                                quote::quote! {{
                                    #if_write_is_err_ts
                                }}
                            },
                            &Er0,
                            &quote::quote! {{
                                return Err(#Er0);
                            }},
                        )
                    };
                let pk_ts = gen_match_as_pg_crud_pg_type_pg_type_cr_qp_ts(
                    pk_ft,
                    &pk_ft_as_dflt_some_one_el_call_ts,
                );
                let col_incrs_ts = gen_fields_named_without_pk_without_comma_ts(
                    &|el: &macros_helpers::field_data::SynField| {
                        gen_match_as_pg_crud_pg_type_pg_type_cr_qp_ts(&el.type0, &{
                            let el_fi = &el.ident;
                            quote::quote! {self.#el_fi}
                        })
                    },
                );
                let ts = gen_acc_string_pop_ok_acc_ts(
                    &quote::quote! {acc},
                    &quote::quote! {
                        #pk_ts
                        #col_incrs_ts
                    },
                );
                quote::quote! {
                    fn #CrQpSc(&self, #IncrSc: &mut dyn #import_ts QpIncrMut) -> Result<#import_ts QpFragment, #import_ts #QpErUcc> {
                        #ts
                    }
                }
            };
            let fn_cr_qb_ts = {
                let gen_query_as_pg_crud_pg_type_pg_type_cr_qb_ts: &dyn Fn(
                    &dyn quote::ToTokens,
                    &dyn quote::ToTokens,
                ) -> proc_macro2::TokenStream = &|ft, ts| {
                        gen_match_qb_or_err_ts(
                            &{
                                let as_pg_crud_pg_type_pg_type_ts = gen_as_pg_type_path_ts(&ft);
                                quote::quote! {#as_pg_crud_pg_type_pg_type_ts #CrQbSc(#ts,#QuerySc)}
                            },
                            &quote::quote! {v_3c55d2e1},
                            &quote::quote! {return Err(#Er0);},
                        )
                    };
                let pk_ts = gen_query_as_pg_crud_pg_type_pg_type_cr_qb_ts(
                    pk_ft,
                    &pk_ft_as_dflt_some_one_el_call_ts,
                );
                let binded_query_modifications_ts = gen_fields_named_without_pk_without_comma_ts(
                    &|el: &macros_helpers::field_data::SynField| {
                        gen_query_as_pg_crud_pg_type_pg_type_cr_qb_ts(&el.type0, &{
                            let fi = &el.ident;
                            quote::quote! {self.#fi}
                        })
                    },
                );
                quote::quote! {
                    fn #CrQbSc(self, mut #QuerySc: #import_ts SqlxPostgresQuery<'_>) -> Result<#import_ts SqlxPostgresQuery<'_>, #import_ts SqlxPostgresQueryBindEr> {
                        #pk_ts
                        #binded_query_modifications_ts
                        Ok(#QuerySc)
                    }
                }
            };
            quote::quote! {
                #AllowClippyArbitrarySrcItemOrdering
                impl #ident_cr_ucc {
                    #fn_cr_qp_ts
                    #fn_cr_qb_ts
                }
            }
        };
        let impl_pg_crud_dflt_some_one_el_for_ident_cr_ts =
            gen_impl_pg_crud_dflt_some_one_el_for_tokens_no_lt_ts(&ident_cr_ucc, &{
                let fields_init_without_pk_with_dflt_some_one_el_ts =
                    gen_fields_named_without_pk_with_comma_ts(
                        &|el: &macros_helpers::field_data::SynField| {
                            gen_fi_dflt_some_one_el_call_ts(&el.ident)
                        },
                    );
                quote::quote! {
                    Self{#fields_init_without_pk_with_dflt_some_one_el_ts}
                }
            });
        quote::quote! {
            #ident_cr_ts
            #impl_ident_cr_ts
            #impl_pg_crud_dflt_some_one_el_for_ident_cr_ts
        }
    };
    let gen_no_fields_provided_er_ts = |er_ucc: &dyn quote::ToTokens| {
        pg_crud_macros_cmn::ts_helpers::er_enum_d_ts_builder().build_enum(
            &proc_macro2::TokenStream::new(),
            er_ucc,
            &proc_macro2::TokenStream::new(),
            &quote::quote! {{
                #NoFieldsProvidedUcc {
                    #[eo_to_err_string]
                    loc: loc_lib::loc::Loc,
                }
            }},
        )
    };
    let ident_wh_ucc = naming::prm::SelfWhManyUcc::from_tokens(&ident);
    let ident_wh_try_new_er_ucc = naming::prm::SelfWhManyTryNewErUcc::from_tokens(&ident);
    let ident_wh_ts = {
        let fields_schema_dcl_ts = gen_fields_named_with_comma_ts(
            &|el: &macros_helpers::field_data::SynField| -> proc_macro2::TokenStream {
                let fi = &el.ident;
                let el_syn_field_ty_as_pg_type_wh_ts = gen_as_pg_type_wh_ts(&el.type0);
                let concrete_wh_ts = gen_concrete_pg_type_role_ts(&el.type0, &WhUcc);
                let field_type_ts = pg_crud_macros_cmn::gen_opt_type_dcl_ts(
                    &quote::quote! {#import_ts PgTypeWh<#el_syn_field_ty_as_pg_type_wh_ts>},
                );
                quote::quote! {
                    #[schema(inline, value_type = Option<#import_ts PgTypeWh<#concrete_wh_ts>>, nullable = false)]
                    #fi: #field_type_ts
                }
            },
        );
        let fields_dcl_ts = gen_fields_named_with_comma_ts(
            &|el: &macros_helpers::field_data::SynField| -> proc_macro2::TokenStream {
                let fi = &el.ident;
                let el_syn_field_ty_as_pg_type_wh_ts = gen_as_pg_type_wh_ts(&el.type0);
                let opt_pg_type_wh_syn_field_ty_as_pg_type_wh_ts =
                    pg_crud_macros_cmn::gen_opt_type_dcl_ts(
                        &quote::quote! {#import_ts PgTypeWh<#el_syn_field_ty_as_pg_type_wh_ts>},
                    );
                quote::quote! {
                    #fi: #opt_pg_type_wh_syn_field_ty_as_pg_type_wh_ts
                }
            },
        );
        let ident_wh_ts = {
            let ident_wh_struct_ts = macros_helpers::derive_ts_builder::DTsBuilder::new()
                .make_pub()
                .d_debug()
                .d_clone()
                .d_serde_serialize()
                .d_utoipa_to_schema()
                .build_struct(
                    &proc_macro2::TokenStream::new(),
                    &ident_wh_ucc,
                    &proc_macro2::TokenStream::new(),
                    &quote::quote! {{#fields_schema_dcl_ts}},
                );
            quote::quote! {
                #AllowClippyArbitrarySrcItemOrdering
                #ident_wh_struct_ts
            }
        };
        let ident_wh_try_new_er_ts = gen_no_fields_provided_er_ts(&ident_wh_try_new_er_ucc);
        let impl_pub_try_new_for_ident_wh_ts =
            macros_helpers::gen_new_or_try_new::gen_impl_pub_try_new_for_ident_ts(
                &proc_macro2::TokenStream::new(),
                &ident_wh_ucc,
                &fields_dcl_ts,
                &ident_wh_try_new_er_ucc,
                &{
                    let gen_fields_ts = |add_borrow: AddBorrow| {
                        gen_fields_named_with_comma_ts(
                        &|el: &macros_helpers::field_data::SynField| -> proc_macro2::TokenStream {
                            let fi = &el.ident;
                            quote::quote! {#add_borrow #fi}
                        },
                    )
                    };
                    let fields_ts = gen_fields_ts(AddBorrow::True);
                    let fields_inialization_ts = gen_fields_ts(AddBorrow::False);
                    quote::quote! {
                        if matches!((#fields_ts), (#fields_named_with_comma_none_ts)) {
                            return Err(#ident_wh_try_new_er_ucc::#NoFieldsProvidedUcc {
                                loc: loc_macros::loc!(),
                            });
                        }
                        Ok(Self {#fields_inialization_ts})
                    }
                },
            );
        let impl_de_for_ident_wh_ts = pg_crud_macros_cmn::gen_impl_de_for_struct_by_fields_ts(
            &ident_wh_ucc,
            pg_crud_macros_cmn::SynFieldRefs::from(fields.as_slice()),
            pg_crud_macros_cmn::DeLen::from(fields_len),
            &|_, syn_type| {
                let syn_type_as_pg_type_wh_ts = gen_as_pg_type_wh_ts(&syn_type);
                pg_crud_macros_cmn::gen_opt_type_dcl_ts(
                    &quote::quote! {#import_ts PgTypeWh<#syn_type_as_pg_type_wh_ts>},
                )
            },
        );
        let impl_pg_crud_dflt_some_one_el_for_ident_wh_ts =
            gen_impl_pg_crud_dflt_some_one_el_for_tokens_no_lt_ts(&ident_wh_ucc, &{
                let fields_ts = gen_fields_named_without_comma_ts(
                    &|el: &macros_helpers::field_data::SynField| {
                        let fi = &el.ident;
                        quote::quote! {
                            #fi: Some(
                                #PgCrudCmnDfltSomeOneElCall
                            ),
                        }
                    },
                );
                quote::quote! {Self{#fields_ts}}
            });
        quote::quote! {
            #ident_wh_ts
            #ident_wh_try_new_er_ts
            #impl_pub_try_new_for_ident_wh_ts
            #impl_de_for_ident_wh_ts
            #impl_pg_crud_dflt_some_one_el_for_ident_wh_ts
        }
    };
    let opt_ident_wh_ucc = naming::prm::StdOptOptSelfWhManyUcc::from_tokens(&ident);
    let opt_ident_wh_ts = {
        let opt_ident_wh_ts = macros_helpers::derive_ts_builder::DTsBuilder::new()
            .make_pub()
            .d_debug()
            .d_clone()
            .d_serde_serialize()
            .d_serde_deserialize()
            .d_utoipa_to_schema()
            .build_struct(
                &proc_macro2::TokenStream::new(),
                &opt_ident_wh_ucc,
                &proc_macro2::TokenStream::new(),
                &{
                    let opt_ident_rd_ids_stdrt_nn_ts =
                        pg_crud_macros_cmn::gen_opt_type_dcl_ts(&ident_wh_ucc);
                    quote::quote! {(#opt_ident_rd_ids_stdrt_nn_ts);}
                },
            );
        let impl_opt_ident_wh_accessors_ts = {
            let opt_ident_wh_inner_ts = pg_crud_macros_cmn::gen_opt_type_dcl_ts(&ident_wh_ucc);
            quote::quote! {
                impl #opt_ident_wh_ucc {
                    #[must_use]
                    pub const fn as_ref(&self) -> Option<&#ident_wh_ucc> {
                        self.0.as_ref()
                    }
                    #[must_use]
                    pub fn into_option(self) -> #opt_ident_wh_inner_ts {
                        self.0
                    }
                }
            }
        };
        let impl_pg_type_wh_flt_for_opt_ident_wh_ts =
            pg_crud_macros_cmn::impl_pg_type_wh_flt_for_ident_ts(
                &quote::quote! {<'lt>},
                &opt_ident_wh_ucc,
                &proc_macro2::TokenStream::new(),
                &pg_crud_macros_cmn::IncrPrmUndrscr::False,
                &pg_crud_macros_cmn::ColPrmUndrscr::True,
                &pg_crud_macros_cmn::AddOprtrUndrscr::True,
                &{
                    let extra_prms_modification_ts = fields.iter().enumerate().map(|(i, el)| {
                    let fi = &el.ident;
                    gen_if_let_some_ts(
                        &quote::quote! {v_da0f0616},
                        &quote::quote! {&#VSc.#fi},
                        &gen_match_ok_err_ts(
                            &{
                                let fi_dq_ts = gen_quotes::dq_ts(&fi);
                                quote::quote! {#import_ts PgTypeWhFlt::qp(
                                    v_da0f0616,
                                    incr,
                                    #import_ts SqlColRef::from(&#fi_dq_ts),
                                    #import_ts AddOprtr::from(is_first_push_to_extra_prms_already_happend),
                                )}
                            },
                            &quote::quote! {v_9e3f8fdd},
                            &{
                                let ts = if i == fields_len_without_pk {
                                    proc_macro2::TokenStream::new()
                                } else {
                                    quote::quote! {is_first_push_to_extra_prms_already_happend = true;}
                                };
                                quote::quote! {{
                                    #ExtraPrmsSc.push_str(&v_9e3f8fdd);
                                    #ts
                                }}
                            },
                            &Er0,
                            &quote::quote! {{
                                return Err(#Er0);
                            }},
                        ),
                    )
                });
                    quote::quote! {
                        Ok(#import_ts QpFragment::try_from(match self.as_ref() {
                            Some(#VSc) => {
                                let mut #ExtraPrmsSc = #StringTs::from("where");
                                let mut is_first_push_to_extra_prms_already_happend = false;
                                #(#extra_prms_modification_ts)*
                                #ExtraPrmsSc
                            },
                            None => #StringTs::default()
                        }).unwrap_or_else(#import_ts QpFragment::from))
                    }
                },
                &pg_crud_macros_cmn::IsQbMut::True,
                &{
                    let ts = gen_if_let_some_ts(
                        &quote::quote! {v_27176ffb},
                        &quote::quote! {self.into_option()},
                        &gen_fields_named_without_comma_ts(
                            &|el: &macros_helpers::field_data::SynField| {
                                let fi = &el.ident;
                                gen_if_let_some_ts(
                                    &quote::quote! {v_b12d6fe0},
                                    &quote::quote! {v_27176ffb.#fi},
                                    &gen_match_qb_or_err_ts(
                                        &quote::quote! {#import_ts PgTypeWhFlt::qb(v_b12d6fe0, #QuerySc)},
                                        &quote::quote! {v_edaee3b2},
                                        &quote::quote! {return Err(#Er0);},
                                    ),
                                )
                            },
                        ),
                    );
                    quote::quote! {
                        #ts
                        Ok(#QuerySc)
                    }
                },
                &pg_crud_macros_cmn::Import::PgCrudCmn,
            );
        let impl_pg_crud_dflt_some_one_el_for_opt_ident_wh_ts =
            gen_impl_pg_crud_dflt_some_one_el_for_tokens_no_lt_ts(
                &opt_ident_wh_ucc,
                &quote::quote! {Self(Some(#PgCrudCmnDfltSomeOneElCall))},
            );
        quote::quote! {
            #opt_ident_wh_ts
            #impl_opt_ident_wh_accessors_ts
            #impl_pg_type_wh_flt_for_opt_ident_wh_ts
            #impl_pg_crud_dflt_some_one_el_for_opt_ident_wh_ts
        }
    };
    let pub_wh_opt_ident_wh_ts = quote::quote! {pub #WhManySc: #opt_ident_wh_ucc};
    let wh_many_pg_crud_dflt_some_one_el_call_ts = gen_fi_dflt_some_one_el_call_ts(&WhManySc);
    let gen_rd_or_dm_extra_prms_init_ts = |rm_or_dm: &RmOrDm| {
        gen_match_ok_err_short_ts(
            &quote::quote! {#import_ts PgTypeWhFlt::qp(
                &#PrmsSc.#PayloadSc.#WhManySc,
                &mut #IncrSc,
                #import_ts SqlColRef::from(&""),//useless //todo check if can be optimized
                #import_ts AddOprtr::from(false)//useless
            )},
            &quote::quote! {v_d1627695},
            &{
                let op_er_init_eprintln_rm_or_dm_ts = gen_op_er_init_eprintln_res_ts(
                    &Op::from(rm_or_dm),
                    &qp_syn_vrt,
                    std::panic::Location::caller(),
                );
                quote::quote! {{
                    #op_er_init_eprintln_rm_or_dm_ts
                }}
            },
        )
    };
    let macros_helpers_loc_field_attr_eo_to_err_string_serde =
        macros_helpers::loc_data::LocFieldAttr::EoToErrStringSerde;
    let string_syn_punct = macros_helpers::gen_simple_syn_punct::gen_simple_syn_punct(["String"]);
    let try_bind_syn_vrt = new_syn_vrt(
        &TryBindUcc,
        Some(macros_helpers::status_code::StatusCode::InternalServerEr500),
        vec![(
            macros_helpers_loc_field_attr_eo_to_err_string_serde,
            &TryBindSc,
            string_syn_punct.clone(),
        )],
        false,
    );
    let gen_query_pg_type_wh_flt_qb_prms_payload_wh_query_ts = |op: &Op| {
        gen_match_qb_or_err_ts(
            &quote::quote! {#import_ts PgTypeWhFlt::qb(#PrmsSc.#PayloadSc.#WhManySc, #import_ts SqlxPostgresQuery::from(#QuerySc)).map(#import_ts SqlxPostgresQuery::into_inner).map_err(#import_ts SqlxPostgresQueryBindEr::into_inner)},
            &quote::quote! {v_03a58371},
            &gen_op_er_init_eprintln_res_ts(op, &try_bind_syn_vrt, std::panic::Location::caller()),
        )
    };
    let try_from_sqlx_pg_pg_row_with_not_empty_unq_vec_ident_sel_sc =
        naming::prm::TryFromSqlxPgPgRowWithNotEmptyUnqVecSelfSelSc::from_display(&ident);
    let simple_syn_punct_sqlx_error =
        macros_helpers::gen_simple_syn_punct::gen_simple_syn_punct(["sqlx", "Error"]);
    let macros_helpers_loc_field_attr_eo_to_err_string =
        macros_helpers::loc_data::LocFieldAttr::EoToErrString;
    let pg_syn_vrt = new_syn_vrt(
        &PgUcc,
        Some(macros_helpers::status_code::StatusCode::InternalServerEr500),
        vec![(
            macros_helpers_loc_field_attr_eo_to_err_string,
            &PgSc,
            simple_syn_punct_sqlx_error.clone(),
        )],
        false,
    );
    let gen_match_ident_rd_try_from_sqlx_pg_pg_row_with_not_empty_unq_vec_ident_sel_ts =
        |rm_or_ro: &RmOrRo| {
            gen_match_ok_err_short_ts(
                &quote::quote! {#ident_rd_ucc::#try_from_sqlx_pg_pg_row_with_not_empty_unq_vec_ident_sel_sc(
                    &v_b27d7d79,
                    &#PrmsSc.#PayloadSc.#SelSc
                )},
                &quote::quote! {v_90535a1d},
                &{
                    let op_er_init_eprintln_rm_or_ro_ts = gen_op_er_init_eprintln_res_ts(
                        &Op::from(rm_or_ro),
                        &pg_syn_vrt,
                        std::panic::Location::caller(),
                    );
                    quote::quote! {{
                        #op_er_init_eprintln_rm_or_ro_ts
                    }}
                },
            )
        };
    let sel_ts = {
        let ident_sel_ts = {
            let ident_sel_enum_ts = pg_crud_macros_cmn::ts_helpers::cmn_d_ts_builder()
            .d_copy()
            .d_eq()
            .d_std_hash_hash()
            .d_utoipa_to_schema()
            .build_enum(
                &proc_macro2::TokenStream::new(),
                &ident_sel_ucc,
                &proc_macro2::TokenStream::new(),
                &{
                    let vrts = gen_fields_named_with_comma_ts(&|el: &macros_helpers::field_data::SynField| {
                        let serde_ident_ts = gen_quotes::dq_ts(&el.ident);
                        let fi_ucc_ts = naming_cmn::ToTokensToUccTs::case_or_panic(&el.ident);
                        let el_syn_field_ty_as_pg_type_sel_ts = gen_as_pg_type_sel_ts(&el.type0);
                        let concrete_sel_ts = gen_concrete_pg_type_role_ts(&el.type0, &SelUcc);
                        quote::quote! {
                            #[serde(rename(serialize = #serde_ident_ts, deserialize = #serde_ident_ts))]
                            #fi_ucc_ts(#[schema(value_type = #concrete_sel_ts)] #el_syn_field_ty_as_pg_type_sel_ts)
                        }
                    });
                    quote::quote! {{#vrts}}
                }
            );
            quote::quote! {
                #AllowClippyArbitrarySrcItemOrdering
                #ident_sel_enum_ts
            }
        };
        let impl_display_for_ident_sel_ts =
            macros_helpers::gen_impl_display_ts::gen_impl_display_ts(
                &proc_macro2::TokenStream::new(),
                &ident_sel_ucc,
                &proc_macro2::TokenStream::new(),
                &quote::quote! {write!(f, "{}", serde_json::to_string(&self).unwrap_or_else(|el_2636212f|format!("cannot serialize into json: {el_2636212f:?}")))},
            );
        let impl_loc_lib_to_err_string_for_ident_sel_ts =
            pg_crud_macros_cmn::gen_impl_to_err_string_no_generics_ts(
                &ident_sel_ucc,
                &quote::quote! {format!("{self}")},
            );
        let impl_pg_crud_all_vrts_dflt_some_one_el_for_ident_sel_ts =
            pg_crud_macros_cmn::gen_impl_pg_crud_cmn_all_vrts_dflt_some_one_el_ts(
                &ident_sel_ucc,
                &{
                    let els_ts = gen_fields_named_with_comma_ts(
                        &|el: &macros_helpers::field_data::SynField| {
                            let fi_ucc_ts = naming_cmn::ToTokensToUccTs::case_or_panic(&el.ident);
                            quote::quote! {
                                Self::#fi_ucc_ts(#PgCrudCmnDfltSomeOneElCall)
                            }
                        },
                    );
                    quote::quote! {vec![#els_ts]}
                },
            );
        quote::quote! {
            #ident_sel_ts
            #impl_display_for_ident_sel_ts
            #impl_loc_lib_to_err_string_for_ident_sel_ts
            #impl_pg_crud_all_vrts_dflt_some_one_el_for_ident_sel_ts
        }
    };
    let sel_pg_crud_dflt_some_one_el_call_ts = gen_fi_dflt_some_one_el_call_ts(&SelSc);
    let ident_rd_ts = {
        let ident_rd_ts = {
            let ident_rd_struct_ts = macros_helpers::derive_ts_builder::DTsBuilder::new()
                .make_pub()
                .d_debug()
                .d_partial_eq()
                .d_serde_serialize()
                .d_serde_deserialize()
                .d_utoipa_to_schema()
                .build_struct(
                    &proc_macro2::TokenStream::new(),
                    &ident_rd_ucc,
                    &proc_macro2::TokenStream::new(),
                    &{
                        let field_opt_pk_ts = {
                            let opt_v_pk_ft_as_pg_type_rd_ts =
                                pg_crud_macros_cmn::gen_opt_type_dcl_ts(&gen_v_dcl_ts0(
                                    &gen_as_pg_type_rd_ts(&pk_ft),
                                ));
                            let concrete_pk_rd_ts = gen_concrete_pg_type_role_ts(pk_ft, &RdUcc);
                            quote::quote! {
                                #FieldAttrSerdeSkipSerializingIfOptIsNone
                                #[schema(inline, value_type = Option<#import_ts V<#concrete_pk_rd_ts>>, nullable = false)]
                                pub #pk_fi: #opt_v_pk_ft_as_pg_type_rd_ts
                            }
                        };
                        let fields_opts_without_pk_ts = gen_fields_named_without_pk_with_comma_ts(
                            &|el: &macros_helpers::field_data::SynField| -> proc_macro2::TokenStream {
                                let field_vis = &el.vis;
                                let fi = &el.ident;
                                let opt_v_ft_as_pg_type_rd_ts =
                                    pg_crud_macros_cmn::gen_opt_type_dcl_ts(&gen_v_dcl_ts0(
                                        &gen_as_pg_type_rd_ts(&el.type0),
                                    ));
                                let concrete_rd_ts = gen_concrete_pg_type_role_ts(&el.type0, &RdUcc);
                                quote::quote! {
                                    #FieldAttrSerdeSkipSerializingIfOptIsNone
                                    #[schema(inline, value_type = Option<#import_ts V<#concrete_rd_ts>>, nullable = false)]
                                    #field_vis #fi: #opt_v_ft_as_pg_type_rd_ts
                                }
                            },
                        );
                        quote::quote! {{
                            #field_opt_pk_ts,
                            #fields_opts_without_pk_ts
                        }}
                    },
                );
            quote::quote! {
                #AllowClippyArbitrarySrcItemOrdering
                #ident_rd_struct_ts
            }
        };
        let impl_ident_rd_ts = {
            let fn_try_from_sqlx_pg_pg_row_with_not_empty_unq_vec_ident_sel_ts = {
                let dcl_pk_ts = {
                    let opt_v_pk_ft_as_pk_ts = pg_crud_macros_cmn::gen_opt_type_dcl_ts(
                        &gen_v_dcl_ts0(&pk_ft_as_pg_type_rd_ucc),
                    );
                    quote::quote! {
                        let mut #pk_fi: #opt_v_pk_ft_as_pk_ts = None;
                    }
                };
                let dcl_without_pk_ts = gen_fields_named_without_pk_without_comma_ts(
                    &|el: &macros_helpers::field_data::SynField| {
                        let fi = &el.ident;
                        let opt_v_ft_as_pg_type_rd_ts = pg_crud_macros_cmn::gen_opt_type_dcl_ts(
                            &gen_v_dcl_ts0(&gen_as_pg_type_rd_ts(&el.type0)),
                        );
                        quote::quote! {
                            let mut #fi: #opt_v_ft_as_pg_type_rd_ts = None;
                        }
                    },
                );
                let gen_assign_ts = |vrt_ucc_ts: &dyn quote::ToTokens,
                                     pg_type_rd_ts: &dyn quote::ToTokens,
                                     fi_string_dq_ts: &dyn quote::ToTokens,
                                     fi: &dyn quote::ToTokens| {
                    let ts = gen_match_ok_err_ts(
                        &quote::quote! {sqlx::Row::try_get::<
                            #pg_type_rd_ts,
                            #RefStr
                        >(
                            #VSc,
                            #fi_string_dq_ts
                        )},
                        &quote::quote! {v_470178a2},
                        &quote::quote! {{
                            #fi = Some(#import_ts #VUcc { #VSc: v_470178a2 });
                        }},
                        &Er0,
                        &quote::quote! {{
                            return Err(#Er0);
                        }},
                    );
                    quote::quote! {#ident_sel_ucc::#vrt_ucc_ts(_) => #ts}
                };
                let (assign_vrt_pk_ts, assign_vrts_without_pk_ts) = {
                    (
                        gen_assign_ts(
                            &pk_fi_ucc_ts,
                            &pk_ft_as_pg_type_rd_ucc,
                            &gen_quotes::dq_ts(&pk_fi),
                            &pk_fi,
                        ),
                        fields_without_pk_iter().map(|el| {
                            gen_assign_ts(
                                &naming_cmn::ToTokensToUccTs::case_or_panic(&el.ident),
                                &gen_as_pg_type_rd_ts(&el.type0),
                                &gen_quotes::dq_ts(&el.ident),
                                &el.ident,
                            )
                        }),
                    )
                };
                let fields_init_ts = fields.iter().map(|el| el.ident.as_ref());
                quote::quote! {
                    fn #try_from_sqlx_pg_pg_row_with_not_empty_unq_vec_ident_sel_sc(
                        #VSc: &sqlx::postgres::PgRow,
                        #sel_borrow_pg_crud_not_empty_unq_vec_ident_sel_ts
                    ) -> Result<Self, sqlx::Error> {
                        #dcl_pk_ts
                        #dcl_without_pk_ts
                        for el_dca9f0b7 in #SelSc.to_vec() {
                            match el_dca9f0b7 {
                                #assign_vrt_pk_ts,
                                #(#assign_vrts_without_pk_ts),*
                            }
                        }
                        Ok(Self {#(#fields_init_ts),*})
                    }
                }
            };
            quote::quote! {
                impl #ident_rd_ucc {
                    #fn_try_from_sqlx_pg_pg_row_with_not_empty_unq_vec_ident_sel_ts
                }
            }
        };
        quote::quote! {
            #ident_rd_ts
            #impl_ident_rd_ts
        }
    };
    let ident_rd_ids_ts = {
        let ident_rd_ids_ts = {
            let ident_rd_ids_struct_ts = pg_crud_macros_cmn::ts_helpers::cmn_d_ts_builder()
                .d_utoipa_to_schema()
                .build_struct(
                    &proc_macro2::TokenStream::new(),
                    &ident_rd_ids_ucc,
                    &proc_macro2::TokenStream::new(),
                    &{
                        enum WrapIntoOpt {
                            False,
                            True,
                        }
                        let gen_field_ts =
                            |fi: &dyn quote::ToTokens,
                             ft: &macros_helpers::field_data::SynFieldType,
                             wrap_into_opt: &WrapIntoOpt| {
                                let ft_ts = match &wrap_into_opt {
                                    WrapIntoOpt::False => gen_as_pg_type_rd_ids_ts(&ft),
                                    WrapIntoOpt::True => pg_crud_macros_cmn::gen_opt_type_dcl_ts(
                                        &gen_as_pg_type_rd_ids_ts(&ft),
                                    )
                                    .into(),
                                };
                                let schema_attr_ts = match wrap_into_opt {
                                    WrapIntoOpt::False => {
                                        let concrete_rd_ids_ts = gen_concrete_pg_type_role_ts(ft, &RdIdsUcc);
                                        quote::quote! {#[schema(value_type = #concrete_rd_ids_ts)]}
                                    },
                                    WrapIntoOpt::True => quote::quote! {#[schema(inline, value_type = Option<#import_ts NonPkPgTypeRdIds>, nullable = false)]},
                                };
                                quote::quote! {#schema_attr_ts pub #fi: #ft_ts}
                            };
                        let pk_ts = gen_field_ts(&pk_fi, pk_ft, &WrapIntoOpt::False);
                        let ts = gen_fields_named_without_pk_with_comma_ts(
                            &|el: &macros_helpers::field_data::SynField| {
                                gen_field_ts(&el.ident, &el.type0, &WrapIntoOpt::True)
                            },
                        );
                        quote::quote! {{
                            #pk_ts,
                            #ts
                        }}
                    },
                );
            quote::quote! {
                #AllowClippyArbitrarySrcItemOrdering
                #ident_rd_ids_struct_ts
            }
        };
        let impl_sqlx_row_for_ident_rd_ids_ts = {
            let undescore_undrscr_row = quote::quote! {__row};
            let wh_fts_ts = gen_fields_named_with_comma_ts(
                &|el: &macros_helpers::field_data::SynField| {
                    let ft = &el.type0;
                    let el_syn_field_ty_as_pg_type_rd_ids_ts = gen_as_pg_type_rd_ids_ts(&ft);
                    quote::quote! {
                        #el_syn_field_ty_as_pg_type_rd_ids_ts: ::sqlx::decode::Decode<'lt, R::Database>
                    }
                },
            );
            let pk_ts = {
                let el_syn_field_ty_as_pg_type_rd_ids_ts = gen_as_pg_type_rd_ids_ts(&pk_ft);
                let fi_dq_ts = gen_quotes::dq_ts(&pk_fi);
                let ts = gen_match_ok_err_short_ts(
                    &quote::quote! {sqlx::Row::try_get::<#el_syn_field_ty_as_pg_type_rd_ids_ts, &str>(
                        #undescore_undrscr_row,
                        #fi_dq_ts
                    )},
                    &quote::quote! {v_283179dd},
                    &quote::quote! {{
                        return Err(#Er0);
                    }},
                );
                quote::quote! {
                    let #pk_fi = #ts;
                }
            };
            let fields_init_ts = gen_fields_named_without_pk_without_comma_ts(
                &|el: &macros_helpers::field_data::SynField| {
                    let fi = &el.ident;
                    let ft = &el.type0;
                    let fi_dq_ts = gen_quotes::dq_ts(&quote::quote! {#fi});
                    let el_syn_field_ty_as_pg_type_rd_ids_ts = gen_as_pg_type_rd_ids_ts(&ft);
                    quote::quote! {
                        let #fi = sqlx::Row::try_get::<
                            #el_syn_field_ty_as_pg_type_rd_ids_ts,
                            &str
                        >(#undescore_undrscr_row, #fi_dq_ts).ok();
                    }
                },
            );
            let self_fields_ts =
                gen_fields_named_with_comma_ts(&|el: &macros_helpers::field_data::SynField| {
                    let fi = &el.ident;
                    quote::quote! {#fi}
                });
            quote::quote! {
                impl<'lt, R: ::sqlx::Row<Database = sqlx::Postgres>> ::sqlx::FromRow<'lt, R> for #ident_rd_ids_ucc
                where
                    &'lt ::std::primitive::str: ::sqlx::ColumnIndex<R>,
                    #wh_fts_ts
                {
                    fn from_row(#undescore_undrscr_row: &'lt R) -> ::sqlx::Result<Self> {
                        #pk_ts
                        #fields_init_ts
                        Ok(Self { #self_fields_ts })
                    }
                }
            }
        };
        quote::quote! {
            #ident_rd_ids_ts
            #impl_sqlx_row_for_ident_rd_ids_ts
        }
    };
    let gen_ident_try_op_er_ucc = |op: &Op| {
        let ident_try_op_er = quote::format_ident!("{ident}Try{op}Er");
        quote::quote! {#ident_try_op_er}
    };
    let ident_try_rm_er_ucc = gen_ident_try_op_er_ucc(&Op::Rm);
    let gen_ident_op_er_with_serde_ucc = |op: &Op| gen_ident_op_suffix_ts(op, "ErWithSerde");
    let pg_crud_order_by_ts = quote::quote! {#import_ts #OrderByUcc};
    let ident_upd_ucc = naming::prm::SelfUpdUcc::from_tokens(&ident);
    let ident_um_prms_ucc = naming::prm::SelfUmPrmsUcc::from_tokens(&ident);
    let ident_um_payload_ucc = naming::prm::SelfUmPayloadUcc::from_tokens(&ident);
    let ident_upd_try_new_er_ucc = naming::prm::SelfUpdTryNewErUcc::from_tokens(&ident);
    let ident_upd_for_query_ucc = naming::prm::SelfUpdForQueryUcc::from_tokens(&ident);
    let path_v_ts = quote::quote! {pg_crud_cmn::#VUcc};
    let ident_upd_ts = {
        let gen_opt_v_ft_as_pg_type_upd_ts: &dyn Fn(
            &dyn quote::ToTokens,
        ) -> macros_helpers::generated_rust_ts::GeneratedRustTs = &|syn_type| {
            let syn_type_as_pg_type_upd_ts = gen_as_pg_type_upd_ts(&syn_type);
            pg_crud_macros_cmn::gen_opt_type_dcl_ts(
                &quote::quote! {#path_v_ts<#syn_type_as_pg_type_upd_ts>},
            )
        };
        let fields_dcl_ts = {
            let fields_named_without_pk_ts = gen_fields_named_without_pk_with_comma_ts(
                &|el: &macros_helpers::field_data::SynField| -> proc_macro2::TokenStream {
                    let fi = &el.ident;
                    let opt_v_ft_as_pg_type_upd_ts = gen_opt_v_ft_as_pg_type_upd_ts(&el.type0);
                    quote::quote! {
                        #fi: #opt_v_ft_as_pg_type_upd_ts
                    }
                },
            );
            quote::quote! {
                #pk_fi: #pk_ft_upd_ts,
                #fields_named_without_pk_ts
            }
        };
        let fields_schema_dcl_ts = {
            let fields_named_without_pk_ts = gen_fields_named_without_pk_with_comma_ts(
                &|el: &macros_helpers::field_data::SynField| -> proc_macro2::TokenStream {
                    let fi = &el.ident;
                    let opt_v_ft_as_pg_type_upd_ts = gen_opt_v_ft_as_pg_type_upd_ts(&el.type0);
                    let concrete_upd_ts = gen_concrete_pg_type_role_ts(&el.type0, &UpdUcc);
                    quote::quote! {
                        #[schema(inline, value_type = Option<#path_v_ts<#concrete_upd_ts>>, nullable = false)]
                        #fi: #opt_v_ft_as_pg_type_upd_ts
                    }
                },
            );
            let concrete_pk_upd_ts = gen_concrete_pg_type_role_ts(pk_ft, &UpdUcc);
            quote::quote! {
                #[schema(value_type = #concrete_pk_upd_ts)]
                #pk_fi: #pk_ft_upd_ts,
                #fields_named_without_pk_ts
            }
        };
        let ident_upd_ts = {
            let ident_upd_struct_ts = serde_ser_utoipa_d_ts_builder.build_struct(
                &proc_macro2::TokenStream::new(),
                &ident_upd_ucc,
                &proc_macro2::TokenStream::new(),
                &quote::quote! {{#fields_schema_dcl_ts}},
            );
            quote::quote! {
                #AllowClippyArbitrarySrcItemOrdering
                #ident_upd_struct_ts
            }
        };
        let ident_upd_try_new_er_ts = gen_no_fields_provided_er_ts(&ident_upd_try_new_er_ucc);
        let impl_pub_try_new_for_ident_upd_ts =
            macros_helpers::gen_new_or_try_new::gen_impl_pub_try_new_for_ident_ts(
                &quote::quote! {#[allow(clippy::redundant_pattern_matching)]}, //todo check if 1 then different logic
                &ident_upd_ucc,
                &fields_dcl_ts,
                &ident_upd_try_new_er_ucc,
                &{
                    let (left_ts, right_ts) = {
                        let gen_ts = |ts: &dyn quote::ToTokens| {
                            pg_crud_macros_cmn::mb_wrap_into_braces_ts(
                                ts,
                                pg_crud_macros_cmn::WrapIntoBraces::from(fields_len_without_pk > 1),
                            )
                        };
                        (
                        gen_ts(&gen_fields_named_without_pk_with_comma_ts(
                            &|el: &macros_helpers::field_data::SynField| -> proc_macro2::TokenStream {
                                let fi = &el.ident;
                                quote::quote! {&#fi}
                            },
                        )),
                        gen_ts(&fields_named_without_pk_with_comma_none_ts),
                    )
                    };
                    let fields_inialization_ts = gen_fields_named_with_comma_ts(
                        &|el: &macros_helpers::field_data::SynField| -> proc_macro2::TokenStream {
                            let fi = &el.ident;
                            quote::quote! {#fi}
                        },
                    );
                    quote::quote! {
                        if matches!(#left_ts, #right_ts) {
                            return Err(#ident_upd_try_new_er_ucc::#NoFieldsProvidedUcc {
                                loc: loc_macros::loc!(),
                            });
                        }
                        Ok(Self {#fields_inialization_ts})
                    }
                },
            );
        let impl_de_for_ident_upd_ts = pg_crud_macros_cmn::gen_impl_de_for_struct_by_fields_ts(
            &ident_upd_ucc,
            pg_crud_macros_cmn::SynFieldRefs::from(fields.as_slice()),
            pg_crud_macros_cmn::DeLen::from(fields_len),
            &|syn_ident, syn_type| {
                if syn_ident == pk_fi.as_ref() {
                    quote::quote! {#pk_ft_upd_ts}.into()
                } else {
                    gen_opt_v_ft_as_pg_type_upd_ts(syn_type)
                }
            },
        );
        let impl_pg_crud_dflt_some_one_el_for_ident_upd_ts =
            gen_impl_pg_crud_dflt_some_one_el_for_tokens_no_lt_ts(&ident_upd_ucc, &{
                let ts = gen_fi_dflt_some_one_el_call_ts(&pk_fi);
                let fields_without_pk_with_dflt_some_one_el_ts =
                    gen_fields_named_without_pk_with_comma_ts(
                        &|el: &macros_helpers::field_data::SynField| {
                            let fi = &el.ident;
                            let ts0 = gen_v_init_ts0(&PgCrudCmnDfltSomeOneElCall);
                            quote::quote! {#fi: Some(#ts0)}
                        },
                    );
                quote::quote! {Self{
                    #ts,
                    #fields_without_pk_with_dflt_some_one_el_ts
                }}
            });
        quote::quote! {
            #ident_upd_ts
            #ident_upd_try_new_er_ts
            #impl_pub_try_new_for_ident_upd_ts
            #impl_de_for_ident_upd_ts
            #impl_pg_crud_dflt_some_one_el_for_ident_upd_ts
        }
    };
    let ident_upd_for_query_ts = {
        let ident_upd_for_query_ts = {
            let ident_upd_for_query_struct_ts = serde_ser_utoipa_d_ts_builder.build_struct(
                &proc_macro2::TokenStream::new(),
                &ident_upd_for_query_ucc,
                &proc_macro2::TokenStream::new(),
                &{
                    let fields_named_without_pk_ts = gen_fields_named_without_pk_with_comma_ts(
                        &|el: &macros_helpers::field_data::SynField| -> proc_macro2::TokenStream {
                            let fi = &el.ident;
                            let opt_v_ft_as_pg_type_upd_for_query_ts = {
                                let syn_type_as_pg_type_upd_for_query_ts =
                                    gen_as_pg_type_upd_for_query_ts(&el.type0);
                                pg_crud_macros_cmn::gen_opt_type_dcl_ts(
                                    &quote::quote! {#path_v_ts<#syn_type_as_pg_type_upd_for_query_ts>},
                                )
                            };
                            let concrete_upd_for_query_ts = gen_concrete_pg_type_role_ts(&el.type0, &UpdForQueryUcc);
                            quote::quote! {
                                #[schema(inline, value_type = Option<#path_v_ts<#concrete_upd_for_query_ts>>, nullable = false)]
                                #fi: #opt_v_ft_as_pg_type_upd_for_query_ts
                            }
                        },
                    );
                    let concrete_pk_upd_for_query_ts = gen_concrete_pg_type_role_ts(pk_ft, &UpdForQueryUcc);
                    quote::quote! {{
                        #[schema(value_type = #concrete_pk_upd_for_query_ts)]
                        #pk_fi: #pk_ft_upd_for_query_ts,
                        #fields_named_without_pk_ts
                    }}
                },
            );
            quote::quote! {
                #AllowClippyArbitrarySrcItemOrdering
                #ident_upd_for_query_struct_ts
            }
        };
        let impl_ident_upd_for_query_ts = {
            let upd_qp_pk_ts = {
                let ts = gen_match_ok_err_ts(
                    &quote::quote! {#pk_ft_as_pg_type_ts #UpdQpSc(
                        &self.#pk_fi,
                        #import_ts SqlColRef::from(&""),
                        #import_ts SqlColRef::from(&#ident::#PkSc()),
                        #import_ts SqlColRef::from(&""),
                        #IncrSc,
                    )},
                    &VSc,
                    &quote::quote! {Ok(#VSc)},
                    &Er0,
                    &quote::quote! {Err(#Er0)},
                );
                quote::quote! {
                    fn #UpdQpPkSc(&self, #IncrSc: &mut dyn #import_ts QpIncrMut) -> Result<#import_ts QpFragment, #import_ts #QpErUcc> {
                        #ts
                    }
                }
            };
            let upd_qp_fields_ts = gen_fields_named_without_pk_without_comma_ts(
                &|el: &macros_helpers::field_data::SynField| {
                    let fi = &el.ident;
                    let upd_qp_fi_sc = naming::prm::UpdQpSelfSc::from_tokens(&fi);
                    let ft_as_pg_crud_pg_type_pg_type_ts = gen_as_pg_type_path_ts(&el.type0);
                    let ts = gen_match_ok_err_ts(
                        &{
                            let fi_dq_ts = gen_quotes::dq_ts(&fi);
                            quote::quote! {#ft_as_pg_crud_pg_type_pg_type_ts #UpdQpSc(
                                &#VSc.#VSc,
                                #import_ts SqlColRef::from(&#fi_dq_ts),
                                #import_ts SqlColRef::from(&#fi_dq_ts),
                                #import_ts SqlColRef::from(&""),
                                #IncrSc
                            )}
                        },
                        &quote::quote! {v_f75dfd93},
                        &quote::quote! {Ok(v_f75dfd93)},
                        &Er0,
                        &quote::quote! {Err(#Er0)},
                    );
                    quote::quote! {
                        fn #upd_qp_fi_sc(
                            #VSc: &#import_ts V<#ft_as_pg_crud_pg_type_pg_type_ts #UpdForQueryUcc>,
                            #IncrSc: &mut dyn #import_ts QpIncrMut
                        ) -> Result<#import_ts QpFragment, #import_ts #QpErUcc> {
                            #ts
                        }
                    }
                },
            );
            let sel_only_updd_ids_qp_ts = {
                let pk_ts = {
                    let pk_fi_dq_ts = gen_quotes::dq_ts(&pk_fi);
                    let ts = gen_match_ok_err_short_ts(
                        &quote::quote! {#pk_as_pg_type_ts::#SelOnlyUpddIdsQpSc(
                            &self.#pk_fi,
                            #import_ts SqlColRef::from(&#pk_fi_dq_ts),
                            incr,
                        )},
                        &quote::quote! {v},
                        &quote::quote! {{
                            return Err(#Er0);
                        }},
                    );
                    quote::quote! {acc.push_str(&#ts);}
                };
                let ts = fields_without_pk_iter().map(|el| {
                    let fi = &el.ident;
                    gen_if_let_some_ts(&quote::quote! {v_90f79b11}, &quote::quote! {&self.#fi}, &{
                        let ts = gen_match_ok_err_short_ts(
                            &{
                                let fi_dq_ts = gen_quotes::dq_ts(&fi);
                                let ft_as_pg_crud_pg_type_pg_type_ts =
                                    gen_as_pg_type_path_ts(&el.type0);
                                quote::quote! {#ft_as_pg_crud_pg_type_pg_type_ts #SelOnlyUpddIdsQpSc(
                                    &v_90f79b11.#VSc,
                                    #import_ts SqlColRef::from(&#fi_dq_ts),
                                    incr,
                                )}
                            },
                            &quote::quote! {v_47a6f597},
                            &quote::quote! {{
                                return Err(#Er0);
                            }},
                        );
                        quote::quote! {acc.push_str(&#ts);}
                    })
                });
                let ts0 = gen_acc_string_pop_ok_acc_ts(
                    &quote::quote! {acc},
                    &quote::quote! {
                        #pk_ts
                        #(#ts)*
                    },
                );
                quote::quote! {
                    fn #SelOnlyUpddIdsQpSc(&self, #IncrSc: &mut dyn #import_ts QpIncrMut) -> Result<#import_ts QpFragment, #import_ts QpEr> {
                        #ts0
                    }
                }
            };
            let upd_h_ts = gen_from_h_ts(&ident_upd_ucc, &{
                let pk_ft_as_pg_type_upd_for_query_ts = gen_as_pg_type_upd_for_query_ts(&pk_ft);
                let fields_named_without_pk_ts = gen_fields_named_without_pk_with_comma_ts(
                    &|el: &macros_helpers::field_data::SynField| -> proc_macro2::TokenStream {
                        let fi = &el.ident;
                        let ts = gen_v_init_ts0(&{
                            let ft_as_pg_type_upd_for_query_ts =
                                gen_as_pg_type_upd_for_query_ts(&el.type0);
                            quote::quote! {#ft_as_pg_type_upd_for_query_ts::from(v_0e64c53a.#VSc)}
                        });
                        quote::quote! {#fi: #VSc.#fi.map(|v_0e64c53a| #ts)}
                    },
                );
                quote::quote! {
                    Self {
                        #pk_fi: #pk_ft_as_pg_type_upd_for_query_ts::from(#VSc.#pk_fi),
                        #fields_named_without_pk_ts
                    }
                }
            });
            quote::quote! {
                #AllowClippyArbitrarySrcItemOrdering
                impl #ident_upd_for_query_ucc {
                    #upd_qp_pk_ts
                    #upd_qp_fields_ts
                    #sel_only_updd_ids_qp_ts
                    #upd_h_ts
                }
            }
        };
        quote::quote! {
            #ident_upd_for_query_ts
            #impl_ident_upd_for_query_ts
        }
    };
    let gen_match_upd_qp_pk_ts = |op: &Op, ts: &dyn quote::ToTokens| {
        gen_match_ok_err_short_ts(
            &quote::quote! {#ts.#UpdQpPkSc(&mut #IncrSc)},
            &quote::quote! {v_f269a3b2},
            &{
                let op_er_init_eprintln_upd_qp_pk_ts =
                    gen_op_er_init_eprintln_res_ts(op, &qp_syn_vrt, std::panic::Location::caller());
                quote::quote! {{
                    #op_er_init_eprintln_upd_qp_pk_ts
                }}
            },
        )
    };
    let row_and_rollback_syn_vrt = new_syn_vrt(
        &RowAndRollbackUcc,
        Some(macros_helpers::status_code::StatusCode::InternalServerEr500),
        vec![
            (
                macros_helpers_loc_field_attr_eo_to_err_string,
                &RowSc,
                simple_syn_punct_sqlx_error.clone(),
            ),
            (
                macros_helpers_loc_field_attr_eo_to_err_string,
                &RollbackSc,
                simple_syn_punct_sqlx_error,
            ),
        ],
        false,
    );
    let sqlx_query_sqlx_pg_ts = quote::quote! {sqlx::query::<sqlx::Postgres>};
    let (pg_crud_pg_type_wh_flt_qp_ts, pg_crud_pg_type_wh_flt_qb_ts) = {
        let gen_ts = |ts: &dyn quote::ToTokens| quote::quote! {#import_ts PgTypeWhFlt::#ts};
        (gen_ts(&QpSc), gen_ts(&QbSc))
    };
    let vec_struct_opts_ident_ts = pg_crud_macros_cmn::gen_vec_tokens_dcl_ts(&ident_rd_ucc);
    let not_unq_field_syn_vrt = new_syn_vrt(
        &NotUnqFieldUcc,
        Some(macros_helpers::status_code::StatusCode::BadReq400),
        vec![(
            macros_helpers_loc_field_attr_eo_to_err_string_serde,
            &NotUnqFieldSc,
            macros_helpers::gen_simple_syn_punct::gen_simple_syn_punct(
                [&ident_sel_ucc.to_string()],
            ),
        )],
        true,
    );
    let simple_syn_punct_serde_error =
        macros_helpers::gen_simple_syn_punct::gen_simple_syn_punct(["serde_json", "Error"]);
    let serde_json_to_string_syn_vrt = new_syn_vrt(
        &SerdeJsonToStringUcc,
        None,
        vec![(
            macros_helpers_loc_field_attr_eo_to_err_string,
            &SerdeJsonToStringSc,
            simple_syn_punct_serde_error.clone(),
        )],
        false,
    );
    let simple_syn_punct_reqwest_error =
        macros_helpers::gen_simple_syn_punct::gen_simple_syn_punct(["reqwest", "Error"]);
    let failed_to_get_res_text_syn_vrt = new_syn_vrt(
        &FailedToGetResTextUcc,
        Some(macros_helpers::status_code::StatusCode::BadReq400),
        vec![
            (
                macros_helpers_loc_field_attr_eo_to_err_string,
                &StatusCodeSc,
                macros_helpers::gen_simple_syn_punct::gen_simple_syn_punct([
                    "reqwest",
                    "StatusCode",
                ]),
            ),
            (
                macros_helpers_loc_field_attr_eo_to_err_string,
                &HeadersSc,
                macros_helpers::gen_simple_syn_punct::gen_simple_syn_punct([
                    "reqwest",
                    "header",
                    "HeaderMap",
                ]),
            ),
            (
                macros_helpers_loc_field_attr_eo_to_err_string,
                &ReqwestSc,
                simple_syn_punct_reqwest_error.clone(),
            ),
        ],
        false,
    );
    let deserialize_res_syn_vrt = new_syn_vrt(
        &DeResUcc,
        None,
        vec![
            (
                macros_helpers_loc_field_attr_eo_to_err_string,
                &StatusCodeSc,
                macros_helpers::gen_simple_syn_punct::gen_simple_syn_punct([
                    "reqwest",
                    "StatusCode",
                ]),
            ),
            (
                macros_helpers_loc_field_attr_eo_to_err_string,
                &HeadersSc,
                macros_helpers::gen_simple_syn_punct::gen_simple_syn_punct([
                    "reqwest",
                    "header",
                    "HeaderMap",
                ]),
            ),
            (
                macros_helpers_loc_field_attr_eo_to_err_string_serde,
                &ResTextSc,
                string_syn_punct,
            ),
            (
                macros_helpers_loc_field_attr_eo_to_err_string,
                &SerdeSc,
                simple_syn_punct_serde_error.clone(),
            ),
        ],
        false,
    );
    let reqwest_syn_vrt = new_syn_vrt(
        &ReqwestUcc,
        None,
        vec![(
            macros_helpers_loc_field_attr_eo_to_err_string,
            &ReqwestSc,
            simple_syn_punct_reqwest_error,
        )],
        false,
    );
    let check_body_size_syn_vrt = new_syn_vrt(
        &CheckBodySizeUcc,
        Some(macros_helpers::status_code::StatusCode::PayloadTooLarge413),
        vec![(
            macros_helpers::loc_data::LocFieldAttr::EoLoc,
            &CheckBodySizeSc,
            macros_helpers::gen_simple_syn_punct::gen_simple_syn_punct([
                "route_validators",
                "check_body_size",
                &BodySizeErUcc.to_string(),
            ]),
        )],
        false,
    );
    let serde_json_syn_vrt = new_syn_vrt(
        &SerdeJsonUcc,
        Some(macros_helpers::status_code::StatusCode::BadReq400),
        vec![(
            macros_helpers_loc_field_attr_eo_to_err_string,
            &SerdeJsonSc,
            simple_syn_punct_serde_error,
        )],
        false,
    );
    let header_cnt_type_app_json_not_found_syn_vrt = new_syn_vrt(
        &HeaderContentTypeAppJsonNotFoundUcc,
        Some(macros_helpers::status_code::StatusCode::BadReq400),
        Vec::<(
            macros_helpers::loc_data::LocFieldAttr,
            &'static dyn std::fmt::Display,
            macros_helpers::gen_simple_syn_punct::SynPathSegments,
        )>::default(),
        false,
    );
    let cmn_http_req_syn_vrts = {
        vec![
            GenPgTblVariantRef::Syn(serde_json_to_string_syn_vrt.get_syn_vrt()),
            GenPgTblVariantRef::Syn(failed_to_get_res_text_syn_vrt.get_syn_vrt()),
            GenPgTblVariantRef::Syn(deserialize_res_syn_vrt.get_syn_vrt()),
            GenPgTblVariantRef::Syn(reqwest_syn_vrt.get_syn_vrt()),
        ]
    };
    let empty_logic_ts = proc_macro2::TokenStream::new();
    let gen_logic_ts = |gen_pg_tbl_attr| -> &proc_macro2::TokenStream {
        gen_pg_tbl_input_model
            .logic_ts_by_attr
            .get(&gen_pg_tbl_attr)
            .unwrap_or(&empty_logic_ts)
    };
    let cmn_route_syn_vrts = {
        let opt_cmn_er_vrts = gen_pg_tbl_input_model
            .er_vrts_by_attr
            .get(&GenPgTblAttr::CmnErVrts);
        let mut acc =
            Vec::with_capacity(4usize.saturating_add(opt_cmn_er_vrts.map_or(0usize, Vec::len)));
        acc.push(GenPgTblVariantRef::Syn(
            check_body_size_syn_vrt.get_syn_vrt(),
        ));
        acc.push(GenPgTblVariantRef::Syn(pg_syn_vrt.get_syn_vrt()));
        acc.push(GenPgTblVariantRef::Syn(serde_json_syn_vrt.get_syn_vrt()));
        acc.push(GenPgTblVariantRef::Syn(
            header_cnt_type_app_json_not_found_syn_vrt.get_syn_vrt(),
        ));
        if let Some(vrts) = opt_cmn_er_vrts {
            acc.extend(vrts.iter().map(GenPgTblVariantRef::Model));
        }
        acc
    };
    let gen_pub_h_ts = |is_pub| {
        if is_pub {
            quote::quote! {pub}
        } else {
            proc_macro2::TokenStream::new()
        }
    };
    let gen_pub_h_pk_fi_pk_inn_type_h_ts = |ts: &dyn quote::ToTokens| {
        let is_pub = true;
        let pub_h_ts = gen_pub_h_ts(is_pub);
        quote::quote! {#pub_h_ts #pk_fi: #ts}
    };
    let gen_match_pg_transaction_rollback_await_ts = |op: &Op, loc| {
        let op_er_init_pg_rollback_ts = gen_op_er_init_eprintln_res_ts(op, &pg_syn_vrt, loc);
        let row_and_rollback_syn_vrt_er_init_eprintln_res_creation_ts =
            gen_op_er_init_eprintln_res_ts(op, &row_and_rollback_syn_vrt, loc);
        quote::quote! {{
            if let Err(#Er1) = #ExecutorSc.#RollbackSc().await {
                #row_and_rollback_syn_vrt_er_init_eprintln_res_creation_ts
            }
            #op_er_init_pg_rollback_ts
        }}
    };
    let gen_drop_rows_match_pg_transaction_rollback_await_h_ts = |op: &Op, loc| {
        let match_pg_transaction_rollback_await_ts =
            gen_match_pg_transaction_rollback_await_ts(op, loc);
        quote::quote! {
            drop(#RowsSc);
            #match_pg_transaction_rollback_await_ts
        }
    };
    let wrap_into_v_ts = |ts: &dyn quote::ToTokens| {
        quote::quote! {
            let #VSc = {
                #ts
            };
        }
    };
    let gen_fetch_ts = |fetch_ts: &dyn quote::ToTokens,
                        some_ts: &dyn quote::ToTokens,
                        er_ts: &dyn quote::ToTokens,
                        should_wrap_into_v: &ShouldWrapIntoV| {
        let ts = {
            let ts = gen_match_ok_err_ts(
                &quote::quote! {futures::TryStreamExt::try_next(&mut #RowsSc).await},
                &quote::quote! {v_19f3d6e1},
                &quote::quote! {match v_19f3d6e1 {
                    Some(v_b27d7d79) => #some_ts,
                    None => None,
                }},
                &Er0,
                &quote::quote! {{
                    #er_ts
                }},
            );
            quote::quote! {
                let mut #RowsSc = #BindedQuerySc.fetch(#fetch_ts.as_mut());
                let mut acc_d16ac269 = Vec::new();
                while let Some(v_d9cc2c36) = #ts {
                    acc_d16ac269.push(v_d9cc2c36);
                }
                acc_d16ac269
            }
        };
        match should_wrap_into_v {
            ShouldWrapIntoV::False => ts,
            ShouldWrapIntoV::True => wrap_into_v_ts(&ts),
        }
    };
    let gen_fetch_one_ts = |fetch_ts: &dyn quote::ToTokens,
                            ok_ts: &dyn quote::ToTokens,
                            er_ts: &dyn quote::ToTokens| {
        gen_match_ok_err_ts(
            &quote::quote! {#BindedQuerySc.fetch_one(#fetch_ts.as_mut()).await},
            &quote::quote! {v_b27d7d79},
            &quote::quote! {{
                #ok_ts
            }},
            &Er0,
            &quote::quote! {{
                #er_ts
            }},
        )
    };
    let gen_sqlx_row_try_get_pk_ts =
        |sqlx_row_try_get_type_ts: &dyn quote::ToTokens,
         ok_ts: &dyn quote::ToTokens,
         err_ts: &dyn quote::ToTokens| {
            gen_match_ok_err_ts(
                &quote::quote! {#SqlxRow::try_get::<
                    #sqlx_row_try_get_type_ts,
                    #RefStr
                >(&v_b27d7d79, Self::#PkSc())},
                &quote::quote! {v_69ecb6a9},
                &ok_ts,
                &Er0,
                &quote::quote! {{
                    #err_ts
                }},
            )
        };
    let wrap_into_pg_transaction_begin_commit_ts = |op: &Op, ts: &dyn quote::ToTokens| {
        let pg_transaction_begin_ts = {
            let op_er_init_pg_begin_ts =
                gen_op_er_init_eprintln_res_ts(op, &pg_syn_vrt, std::panic::Location::caller());
            let ts0 = gen_match_ok_err_short_ts(
                &quote::quote! {#SqlxAcquire::#BeginSc(#ExecutorAcquireSc).await},
                &quote::quote! {v_1aaca28f},
                &quote::quote! {{#op_er_init_pg_begin_ts}},
            );
            quote::quote! {let mut #ExecutorSc = #ts0;}
        };
        let pg_transaction_commit_ts = {
            let pg_syn_vrt_er_init_eprintln_res_creation_ts =
                gen_op_er_init_eprintln_res_ts(op, &pg_syn_vrt, std::panic::Location::caller());
            quote::quote! {
                if let Err(#Er0) = #ExecutorSc.#CommitSc().await {
                    #pg_syn_vrt_er_init_eprintln_res_creation_ts
                }
            }
        };
        quote::quote! {
            #pg_transaction_begin_ts
            #ts
            #pg_transaction_commit_ts
            #VSc
        }
    };
    let gen_loc_attr_view_ts = |fi_ref: SynGenPgTblIdentRef<'_>,
                                loc_attr: GenPgTblVariantLocAttr|
     -> proc_macro2::TokenStream {
        let fi = fi_ref.get();
        if *fi == *LocSc.to_string() {
            proc_macro2::TokenStream::new()
        } else {
            loc_attr.get().map_or_else(
                || compile_error_ts(CompileErrorMsg("d1003b2e: loc field attr not found")).into(),
                |v| v.to_attr_view_ts().into(),
            )
        }
    };
    let gen_loc_vrt_ts: &dyn Fn(GenPgTblVariantRef<'_>) -> proc_macro2::TokenStream =
        &|er_vrt| -> proc_macro2::TokenStream {
            let vrt_ident = er_vrt.ident();
            match er_vrt {
                GenPgTblVariantRef::Syn(syn_vrt) => {
                    let syn::Fields::Named(fields_named) = &syn_vrt.fields else {
                        return compile_error_ts(CompileErrorMsg(
                            "2acd4725: expected named variant fields",
                        ))
                        .into();
                    };
                    let fields_mapped_into_ts = fields_named.named.iter().map(|field| {
                        let Some(fi) = field.ident.as_ref() else {
                            return compile_error_ts(CompileErrorMsg(
                                "a21dc807: expected named field ident",
                            ))
                            .into();
                        };
                        let parsed_loc_attr = match gen_pg_tbl_syn_field_loc_attr_stage(
                            SynGenPgTblFieldRef::from(field),
                        ) {
                            Ok(v) => v,
                            Err(er) => return er.into(),
                        };
                        let loc_attr_ts = gen_loc_attr_view_ts(
                            SynGenPgTblIdentRef::from(fi),
                            GenPgTblVariantLocAttr::from(parsed_loc_attr),
                        );
                        let ft = &field.ty;
                        quote::quote! {
                            #loc_attr_ts
                            #fi: #ft
                        }
                    });
                    quote::quote! {
                        #vrt_ident {
                            #(#fields_mapped_into_ts),*
                        }
                    }
                }
                GenPgTblVariantRef::Model(model_vrt) => {
                    let fields_mapped_into_ts = model_vrt.fields.iter().map(|field| {
                        let fi = &field.ident;
                        let loc_attr = gen_loc_attr_view_ts(
                            SynGenPgTblIdentRef::from(fi),
                            GenPgTblVariantLocAttr::from(field.loc_attr),
                        );
                        let ft = &field.type0;
                        quote::quote! {
                            #loc_attr
                            #fi: #ft
                        }
                    });
                    quote::quote! {
                        #vrt_ident {
                            #(#fields_mapped_into_ts),*
                        }
                    }
                }
            }
        };
    let gen_serde_field_ts = |fi_ref: SynGenPgTblIdentRef<'_>,
                              ty_ref: SynGenPgTblTypeRef<'_>,
                              loc_attr: GenPgTblVariantLocAttr|
     -> macros_helpers::generated_rust_ts::GeneratedRustTs {
        let fi = fi_ref.get();
        let ty = ty_ref.get();
        let string_ts = token_patterns::StringTs;
        let with_serde_ucc = naming::WithSerdeUcc;
        let hash_map_ucc = naming::HashMapUcc;
        let ts = if *fi == *LocSc.to_string() {
            quote::quote! {#LocSc: loc_lib::loc::Loc}
        } else {
            let get_hashmap_args = || {
                let segments = if let syn::Type::Path(syn_type_path) = ty {
                    &syn_type_path.path.segments
                } else {
                    return None;
                };
                let last_segment = segments.iter().next_back()?;
                assert!(last_segment.ident == hash_map_ucc.to_string(), "60f0795d");
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
            let el_type_ts = quote::quote! {#ty};
            let Some(parsed_loc_attr) = loc_attr.get() else {
                return compile_error_ts(CompileErrorMsg("b9f53bee: loc field attr not found"));
            };
            let el_type_with_serde_ts = match parsed_loc_attr {
                macros_helpers::loc_data::LocFieldAttr::EoToErrString => quote::quote! {#string_ts},
                macros_helpers::loc_data::LocFieldAttr::EoToErrStringSerde
                | macros_helpers::loc_data::LocFieldAttr::EoVecToErrStringSerde => el_type_ts,
                macros_helpers::loc_data::LocFieldAttr::EoLoc => {
                    match format!("{el_type_ts}{with_serde_ucc}")
                        .parse::<proc_macro2::TokenStream>()
                    {
                        Ok(parsed_ts) => parsed_ts,
                        Err(er) => {
                            return compile_error_ts(CompileErrorMsg(&format!("201dc0a4: {er}")));
                        }
                    }
                }
                macros_helpers::loc_data::LocFieldAttr::EoVecToErrString => {
                    quote::quote! {Vec<#string_ts>}
                }
                macros_helpers::loc_data::LocFieldAttr::EoVecLoc => {
                    let segments = if let syn::Type::Path(v0) = ty {
                        &v0.path.segments
                    } else {
                        return compile_error_ts(CompileErrorMsg("8d93bf20: expected path type"));
                    };
                    assert!(segments.len() == 1, "8c6c5e9d");
                    let Some(first_segment) = segments.iter().next() else {
                        return compile_error_ts(CompileErrorMsg(
                            "595050cf: expected first path segment",
                        ));
                    };
                    let syn::PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments {
                        args,
                        ..
                    }) = &first_segment.arguments
                    else {
                        return compile_error_ts(CompileErrorMsg(
                            "07c6ab44: expected angle bracketed args",
                        ));
                    };
                    assert!(args.len() == 1, "5bf19c5d");
                    let Some(first_arg) = args.iter().next() else {
                        return compile_error_ts(CompileErrorMsg(
                            "e9b33787: expected first generic arg",
                        ));
                    };
                    let el_vec_type_with_serde_ts =
                        match format!("{}{}", quote::quote! {#first_arg}, with_serde_ucc,)
                            .parse::<proc_macro2::TokenStream>()
                        {
                            Ok(parsed_ts) => parsed_ts,
                            Err(er) => {
                                return compile_error_ts(CompileErrorMsg(&format!(
                                    "22c364b9: {er}"
                                )));
                            }
                        };
                    quote::quote! {Vec<#el_vec_type_with_serde_ts>}
                }
                macros_helpers::loc_data::LocFieldAttr::EoHashMapKStringVToErrString => {
                    if get_hashmap_args().is_none() {
                        return compile_error_ts(CompileErrorMsg(
                            "c1d03b71: expected HashMap<K, T>",
                        ));
                    }
                    quote::quote! {std::collections::HashMap<#string_ts, #string_ts>}
                }
                macros_helpers::loc_data::LocFieldAttr::EoHashMapKStringVToErrStringSerde => {
                    let Some((_, second_argument)) = get_hashmap_args() else {
                        return compile_error_ts(CompileErrorMsg(
                            "e9c6a7d2: expected HashMap<K, T>",
                        ));
                    };
                    quote::quote! {std::collections::HashMap<#string_ts, #second_argument>}
                }
                macros_helpers::loc_data::LocFieldAttr::EoHashMapKStringVLoc => {
                    let Some((_, second_argument)) = get_hashmap_args() else {
                        return compile_error_ts(CompileErrorMsg(
                            "c828da34: expected HashMap<K, T>",
                        ));
                    };
                    let el_hashmap_v_type_with_serde_ts =
                        match format!("{}{}", quote::quote! {#second_argument}, with_serde_ucc)
                            .parse::<proc_macro2::TokenStream>()
                        {
                            Ok(parsed_ts) => parsed_ts,
                            Err(er) => {
                                return compile_error_ts(CompileErrorMsg(&format!(
                                    "86307dbc: {er}"
                                )));
                            }
                        };
                    quote::quote! {std::collections::HashMap<#string_ts, #el_hashmap_v_type_with_serde_ts>}
                }
            };
            quote::quote! {#fi: #el_type_with_serde_ts}
        };
        macros_helpers::generated_rust_ts::GeneratedRustTs::from(quote::quote! {#ts,})
    };
    let gen_serde_version_of_named_gen_pg_tbl_vrt_ts =
        |er_vrt: GenPgTblVariantRef<'_>| -> proc_macro2::TokenStream {
            let vrt_ident = er_vrt.ident();
            match er_vrt {
                GenPgTblVariantRef::Syn(syn_vrt) => {
                    let syn::Fields::Named(fields_named) = &syn_vrt.fields else {
                        return compile_error_ts(CompileErrorMsg(
                            "79b0f231: expected named variant fields",
                        ))
                        .into();
                    };
                    let fields_with_serde_ts = fields_named.named.iter().map(|field| {
                        let Some(fi) = field.ident.as_ref() else {
                            return compile_error_ts(CompileErrorMsg(
                                "438aa90e: expected named field ident",
                            ));
                        };
                        let loc_attr = match gen_pg_tbl_syn_field_loc_attr_stage(
                            SynGenPgTblFieldRef::from(field),
                        ) {
                            Ok(v) => v,
                            Err(er) => return er,
                        };
                        gen_serde_field_ts(
                            SynGenPgTblIdentRef::from(fi),
                            SynGenPgTblTypeRef::from(&field.ty),
                            GenPgTblVariantLocAttr::from(loc_attr),
                        )
                    });
                    quote::quote! {
                        #vrt_ident {
                            #(#fields_with_serde_ts)*
                        }
                    }
                }
                GenPgTblVariantRef::Model(model_vrt) => {
                    let fields_with_serde_ts = model_vrt.fields.iter().map(|field| {
                        gen_serde_field_ts(
                            SynGenPgTblIdentRef::from(&field.ident),
                            SynGenPgTblTypeRef::from(&field.type0),
                            GenPgTblVariantLocAttr::from(field.loc_attr),
                        )
                    });
                    quote::quote! {
                        #vrt_ident {
                            #(#fields_with_serde_ts)*
                        }
                    }
                }
            }
        };
    let gen_ident_op_payload_ucc = |op: &Op| match &op {
        Op::Co => quote::quote! {#ident_cr_ucc},
        Op::Uo => quote::quote! {#ident_upd_ucc},
        Op::Cm | Op::Rm | Op::Ro | Op::Um | Op::Dm | Op::Dlo => {
            gen_ident_op_suffix_ts(op, &PayloadUcc.to_string())
        }
    };
    let gen_ident_op_prms_ucc = |op: &Op| gen_ident_op_suffix_ts(op, "Prms");
    let std_sync_arc_combination_of_app_state_logic_traits_ts =
        quote::quote! {std::sync::Arc<dyn pg_tbl::CombinationOfAppStateLogicTraits>};
    let gen_op_result_type_ts = |op: &Op| -> &dyn quote::ToTokens {
        match op {
            Op::Rm => &vec_struct_opts_ident_ts,
            Op::Ro => &ident_rd_ucc,
            Op::Dm => &vec_pk_ft_rd_ts,
            Op::Dlo => &pk_ft_as_pg_type_rd_ucc,
            Op::Co | Op::Uo => &ident_rd_ids_ucc,
            Op::Cm | Op::Um => &vec_ident_rd_ids_ts,
        }
    };
    let pk_ft_orgn_ts = if let syn::Type::Path(type_path) = &**pk_ft {
        let Some(source_last_segment) = type_path.path.segments.last() else {
            return compile_error_ts(CompileErrorMsg(
                "6d0adac1: cloned primary key type path has no segments",
            ));
        };
        let orgn_ident = quote::format_ident!(
            "{}",
            naming::prm::SelfOrgnUcc::from_tokens(&source_last_segment.ident).to_string()
        );
        let mut orgn_type_path = type_path.clone();
        let Some(last_segment) = orgn_type_path.path.segments.last_mut() else {
            return compile_error_ts(CompileErrorMsg(
                "e7408836: primary key type path has no segments",
            ));
        };
        last_segment.ident = orgn_ident;
        quote::quote! {#orgn_type_path}
    } else {
        return compile_error_ts(CompileErrorMsg("2ad2130d: primary key type must be a path"));
    };
    let pk_ft_upd_for_query_open_api_ts = gen_as_pg_type_upd_for_query_ts(&pk_ft);
    fields.iter().fold((), |(), field| {
        let roles: [&dyn quote::ToTokens; 6] =
            [&CrUcc, &RdUcc, &SelUcc, &UpdUcc, &UpdForQueryUcc, &WhUcc];
        roles.into_iter().fold((), |(), role| {
            open_api_schema_types_ts.push(gen_concrete_pg_type_role_ts(&field.type0, role));
        });
        let orgn_role = quote::format_ident!("Orgn");
        open_api_schema_types_ts.push(gen_concrete_pg_type_role_ts(&field.type0, &orgn_role));
    });
    open_api_schema_types_ts.push(gen_concrete_pg_type_role_ts(pk_ft, &RdIdsUcc));
    open_api_schema_types_ts.extend([
        quote::quote! {#ident_cr_ucc},
        quote::quote! {#ident_wh_ucc},
        quote::quote! {#opt_ident_wh_ucc},
        quote::quote! {#ident_sel_ucc},
        quote::quote! {#ident_rd_ucc},
        quote::quote! {#ident_rd_ids_ucc},
        quote::quote! {#ident_upd_ucc},
        quote::quote! {#ident_upd_for_query_ucc},
        quote::quote! {#pk_ft_as_pg_type_rd_ucc},
        quote::quote! {#pk_ft_upd_ts},
        quote::quote! {#pk_ft_orgn_ts},
        quote::quote! {#pk_ft_upd_for_query_open_api_ts},
        quote::quote! {loc_lib::loc::Loc},
        quote::quote! {loc_lib::loc::LocCol},
        quote::quote! {loc_lib::loc::LocCommit},
        quote::quote! {loc_lib::loc::LocFile},
        quote::quote! {loc_lib::loc::LocLine},
        quote::quote! {loc_lib::loc::Occr},
        quote::quote! {loc_lib::loc::StdLocDuration},
        quote::quote! {pg_crud_cmn::NotEmptyUnqVec<#ident_sel_ucc>},
        quote::quote! {pg_crud_cmn::Order},
        quote::quote! {pg_crud_cmn::Oprtr},
        quote::quote! {pg_crud_cmn::OrderBy<#ident_sel_ucc>},
        quote::quote! {pg_crud_cmn::PgCrudStringWrapperTryFromStringEr},
        quote::quote! {pg_crud_cmn::PgnBase},
        quote::quote! {pg_crud_cmn::PgnLimit},
        quote::quote! {pg_crud_cmn::PgnOffset},
        quote::quote! {pg_crud_cmn::PgnStartsWithZero},
        quote::quote! {pg_crud_cmn::QpErWithSerde},
        quote::quote! {route_validators::check_body_size::BodySizeErWithSerde},
        quote::quote! {route_validators::check_body_size::BodySizeLimitBytes},
    ]);
    OpDsc::ALL
    .iter()
    .fold((), |(), op_dsc| {
        let op = &op_dsc.op;
        let op_h_sc_ts = op.self_h_sc_ts();
        let op_sc_ts = op.self_sc_ts();
        let op_sc_string = op.self_sc_str();
        let open_api_path_fn_ident = quote::format_ident!(
            "{}_{}_open_api",
            ident_sc_string,
            op_sc_string
        );
        let open_api_path = format!("/{ident_sc_string}/{op_sc_string}");
        let open_api_path_dq_ts = gen_quotes::dq_ts(&open_api_path);
        let open_api_tag_dq_ts = gen_quotes::dq_ts(&ident_sc_string);
        let open_api_method_ts = match op_dsc.http_method {
            OpHttpMethod::Post => quote::quote! {post},
            OpHttpMethod::Patch => quote::quote! {patch},
            OpHttpMethod::Delete => quote::quote! {delete},
        };
        let open_api_status_ts = if op_dsc.success_status_code
            == macros_helpers::status_code::StatusCode::Crd201
        {
            quote::quote! {201}
        } else {
            quote::quote! {200}
        };
        let open_api_payload_type_ts = gen_ident_op_payload_ucc(op);
        let open_api_response_type_ts = gen_ident_op_res_vrts_ucc(op);
        let ident_op_prms_ucc = gen_ident_op_prms_ucc(op);
        let ident_try_op_er_ucc = gen_ident_try_op_er_ucc(op);
        let result_ok_type_ts = gen_op_result_type_ts(op);
        let try_op_h_sc_ts = op.try_self_h_sc_ts();
        let op_client_method_sc_ts = op.self_sc_ts();
        api_client_methods_ts.push(quote::quote! {
            pub async fn #op_client_method_sc_ts(
                &self,
                #PrmsSc: #ident_op_prms_ucc,
            ) -> Result<#result_ok_type_ts, #ident_try_op_er_ucc> {
                #ident::#try_op_h_sc_ts(
                    &self.client,
                    self.endpoint.as_url().as_str(),
                    #PrmsSc,
                    #ident::#TblNameSc(),
                ).await
            }
        });
        open_api_path_fn_idents.push(open_api_path_fn_ident.clone());
        open_api_schema_types_ts.push(open_api_payload_type_ts.clone());
        open_api_schema_types_ts.push(open_api_response_type_ts.clone());
        let open_api_path_fn_ts = quote::quote! {
            #[allow(dead_code)]
            #[utoipa::path(
                #open_api_method_ts,
                path = #open_api_path_dq_ts,
                operation_id = #op_sc_string,
                tag = #open_api_tag_dq_ts,
                request_body = #open_api_payload_type_ts,
                responses(
                    (status = #open_api_status_ts, description = "Successful response", body = #open_api_response_type_ts),
                    (status = 400, description = "Invalid request", body = #open_api_response_type_ts),
                    (status = 413, description = "Request body is too large", body = #open_api_response_type_ts),
                    (status = 500, description = "Internal server error", body = #open_api_response_type_ts)
                )
            )]
            fn #open_api_path_fn_ident() {}
        };
        let gen_for_el_in_upd_for_query_vec_ts = |ts: &dyn quote::ToTokens| {
            quote::quote! {
                for el_a72f3eac in &#UpdForQueryVecSc {
                    #ts
                }
            }
        };
        let op_er_init_qp_ts =
            gen_op_er_init_eprintln_res_ts(op, &qp_syn_vrt, std::panic::Location::caller());
        let gen_match_ok_err_upd_ts = |ts0: &dyn quote::ToTokens, ts1: &dyn quote::ToTokens| {
            gen_match_ok_err_short_ts(&ts0, &ts1, &quote::quote! {{#op_er_init_qp_ts}})
        };
        let gen_for_el_in_upd_for_query_vec_fi_ts =
            |fi: &dyn quote::ToTokens, ts0: &dyn quote::ToTokens, ts1: &dyn quote::ToTokens| {
                gen_for_el_in_upd_for_query_vec_ts(&gen_if_let_some_ts(
                    &ts0,
                    &quote::quote! {&el_a72f3eac.#fi},
                    &ts1,
                ))
            };
        let type_vrts_from_req_res_syn_vrts = {
            let er_vrts_len = gen_pg_tbl_input_model
                .er_vrts_by_attr
                .get(&op.gen_pg_tbl_attr_er_vrts())
                .map_or(0usize, Vec::len);
            let mut acc = Vec::with_capacity(
                cmn_route_syn_vrts
                    .len()
                    .saturating_add(er_vrts_len)
                    .saturating_add(4usize),
            );
            acc.extend_from_slice(cmn_route_syn_vrts.as_slice());
            if let Op::Rm | Op::Ro = &op {
                acc.push(GenPgTblVariantRef::Syn(
                    not_unq_field_syn_vrt.get_syn_vrt(),
                ));
            }
            if let Op::Cm | Op::Rm | Op::Ro | Op::Co | Op::Um | Op::Uo | Op::Dm = &op {
                acc.push(GenPgTblVariantRef::Syn(qp_syn_vrt.get_syn_vrt()));
            }
            if let Op::Cm | Op::Dlo | Op::Co | Op::Um | Op::Uo | Op::Dm = &op {
                acc.push(GenPgTblVariantRef::Syn(
                    row_and_rollback_syn_vrt.get_syn_vrt(),
                ));
            }
            acc.push(GenPgTblVariantRef::Syn(try_bind_syn_vrt.get_syn_vrt()));
            if let Some(vrts) = gen_pg_tbl_input_model
                .er_vrts_by_attr
                .get(&op.gen_pg_tbl_attr_er_vrts())
            {
                acc.extend(vrts.iter().map(GenPgTblVariantRef::Model));
            }
            acc
        };
        op_routes_ts.push({
            let method_ts = match &op {
                Op::Cm |
                Op::Co |
                Op::Rm |
                Op::Ro => quote::quote! {post},
                Op::Um |
                Op::Uo => quote::quote! {patch},
                Op::Dm |
                Op::Dlo => quote::quote! {delete},
            };
            let op_payload_example_sc =
                op.op_payload_example_sc();
            let (
                slash_op_dq_ts,
                slash_op_payload_example_dq_ts
            ) = {
                let gen_ts = |
                    v: &dyn std::fmt::Display
                | gen_quotes::dq_ts(&format!("/{v}"));
                (
                    gen_ts(&op.self_sc_str()),
                    gen_ts(&op_payload_example_sc)
                )
            };
            quote::quote! {
                .route(#slash_op_dq_ts, axum::routing::#method_ts({
                    let tbl_owned = tbl.to_owned();
                    let requests_metric = metrics::counter!("pg_tbl_requests_total", "table" => tbl_owned.clone(), "operation" => #op_sc_string);
                    let duration_metric = metrics::histogram!("pg_tbl_request_duration_seconds", "table" => tbl_owned.clone(), "operation" => #op_sc_string);
                    let response_200_metric = metrics::counter!("pg_tbl_responses_total", "table" => tbl_owned.clone(), "operation" => #op_sc_string, "status" => "200");
                    let response_201_metric = metrics::counter!("pg_tbl_responses_total", "table" => tbl_owned.clone(), "operation" => #op_sc_string, "status" => "201");
                    let response_400_metric = metrics::counter!("pg_tbl_responses_total", "table" => tbl_owned.clone(), "operation" => #op_sc_string, "status" => "400");
                    let response_413_metric = metrics::counter!("pg_tbl_responses_total", "table" => tbl_owned.clone(), "operation" => #op_sc_string, "status" => "413");
                    let response_500_metric = metrics::counter!("pg_tbl_responses_total", "table" => tbl_owned.clone(), "operation" => #op_sc_string, "status" => "500");
                    let response_other_metric = metrics::counter!("pg_tbl_responses_total", "table" => tbl_owned.clone(), "operation" => #op_sc_string, "status" => "other");
                    async move |
                        app_state_99328dfe: axum::extract::State<#std_sync_arc_combination_of_app_state_logic_traits_ts>,
                        req: axum::extract::Request
                    | {
                        let started_at = std::time::Instant::now();
                        requests_metric.increment(1u64);
                        let response = tracing::Instrument::instrument(
                            Self::#op_h_sc_ts(app_state_99328dfe, req, &tbl_owned),
                            tracing::info_span!(
                                "pg_tbl.operation",
                                table = %tbl_owned,
                                operation = #op_sc_string,
                            ),
                        ).await;
                        duration_metric.record(started_at.elapsed().as_secs_f64());
                        match response.status().as_u16() {
                            200u16 => response_200_metric.increment(1u64),
                            201u16 => response_201_metric.increment(1u64),
                            400u16 => response_400_metric.increment(1u64),
                            413u16 => response_413_metric.increment(1u64),
                            500u16 => response_500_metric.increment(1u64),
                            _ => response_other_metric.increment(1u64),
                        }
                        response
                    }
                }))
                .route(#slash_op_payload_example_dq_ts, axum::routing::get(async move||Self::#op_payload_example_sc()))
            }
        });
        impl_ident_vec_ts.push({
            let try_op_ts = {
                let try_op_sc_ts = op.try_self_sc_ts();
                let payload_ts = {
                    let ts = gen_match_ok_err_short_ts(
                        &quote::quote! {serde_json::to_string(&#PrmsSc.#PayloadSc)},
                        &quote::quote! {v_1772a83e},
                        &{
                            let ts = gen_init_ts(&serde_json_to_string_syn_vrt, std::panic::Location::caller());
                            quote::quote! {{
                                return Err(#ident_try_op_er_ucc::#ts);
                            }}
                        },
                    );
                    quote::quote! {
                        let #PayloadSc = {
                            #ts
                        };
                    }
                };
                let url_ts = {
                    let format_ts = gen_quotes::dq_ts(&format!(
                        "{{endpoint_loc}}/{{tbl}}/{}",
                        op.self_sc_str()
                    ));
                    quote::quote! {let #UrlSc = format!(#format_ts);}
                };
                let future_ts = {
                    let op_http_method_sc_ts =
                        naming_cmn::AsRefStrToScTs::case_or_panic(&op_dsc.http_method);
                    let commit_header_addition_ts = quote::quote! {
                        .header(
                            &"commit".to_owned(),
                            git_info::PROJECT_GIT_INFO.commit.as_ref(),
                        )
                    };
                    let app_json_dq_ts = gen_quotes::dq_ts(&"application/json");
                    let content_type_app_json_header_addition_ts = quote::quote! {
                        .header(reqwest::header::CONTENT_TYPE, #app_json_dq_ts)
                    };
                    quote::quote! {
                        let #FutureSc = #client_sc
                            .#op_http_method_sc_ts(&#UrlSc)
                            #commit_header_addition_ts
                            #content_type_app_json_header_addition_ts
                            .#BodySc(#PayloadSc)
                            .send();
                    }
                };
                let res_ts = {
                    let ts =
                        gen_match_ok_err_short_ts(&quote::quote! {#FutureSc.await}, &quote::quote! {v_180559e9}, &{
                            let ts = gen_init_ts(&reqwest_syn_vrt, std::panic::Location::caller());
                            quote::quote! {{
                                return Err(#ident_try_op_er_ucc::#ts);
                            }}
                        });
                    quote::quote! {let #ResSc = #ts;}
                };
                let er_0_res_status_ts = quote::quote! {
                    let #Er0 = #ResSc.status();
                };
                let headers_ts = quote::quote! {
                    let #Er1 = #ResSc.headers().clone();
                };
                let res_text_ts = {
                    let ts = gen_match_ok_err_ts(
                        &quote::quote! {#ResSc.text().await},
                        &quote::quote! {v_6a62b2b9},
                        &quote::quote! {v_6a62b2b9},
                        &Er2,
                        &{
                            let failed_to_get_res_text_syn_vrt_init_ts =
                                gen_init_ts(&failed_to_get_res_text_syn_vrt, std::panic::Location::caller());
                            quote::quote! {{
                                return Err(#ident_try_op_er_ucc::#failed_to_get_res_text_syn_vrt_init_ts);
                            }}
                        },
                    );
                    quote::quote! {let #Er2 = #ts;}
                };
                let ident_op_res_vrts_ucc = gen_ident_op_res_vrts_ucc(op);
                let expected_res_ts = {
                    let deserialize_res_syn_vrt_init_ts =
                        gen_init_ts(&deserialize_res_syn_vrt, std::panic::Location::caller());
                    let ts = gen_match_ok_err_ts(
                        &quote::quote! {serde_json::from_str::<#ident_op_res_vrts_ucc>(&#Er2)},
                        &quote::quote! {v_563d2a75},
                        &quote::quote! {v_563d2a75},
                        &Er3,
                        &quote::quote! {{
                            return Err(#ident_try_op_er_ucc::#deserialize_res_syn_vrt_init_ts);
                        }},
                    );
                    quote::quote! {let #ExpectedResSc = #ts;}
                };
                let try_op_logic_er_with_serde_ucc =
                    gen_ident_op_er_with_serde_ucc(op);
                let op_er_with_serde_sc = &op.op_er_with_serde_sc();
                let try_op_logic_er_with_serde_ts = {
                    let try_op_logic_res_vrts_to_try_op_logic_er_with_serde = type_vrts_from_req_res_syn_vrts.iter().map(|el| {
                            let vrt_ident = el.ident();
                            let fields_idents_ts = match *el {
                                GenPgTblVariantRef::Syn(syn_vrt) => {
                                    let syn::Fields::Named(fields_named) = &syn_vrt.fields else {
                                        return compile_error_ts(CompileErrorMsg(
                                            "8dcafc1c: expected named variant fields",
                                        ))
                                        .into();
                                    };
                                    let fields_idents = fields_named.named.iter().map(|field| &field.ident);
                                    quote::quote! {#(#fields_idents),*}
                                }
                                GenPgTblVariantRef::Model(model_vrt) => {
                                    let fields_idents = model_vrt.fields.iter().map(|field| &field.ident);
                                    quote::quote! {#(#fields_idents),*}
                                }
                            };
                            quote::quote! {
                                #ident_op_res_vrts_ucc::#vrt_ident {
                                    #fields_idents_ts
                                } => #try_op_logic_er_with_serde_ucc::#vrt_ident { #fields_idents_ts }
                            }
                        });
                    quote::quote! {
                        let #op_er_with_serde_sc = match #ExpectedResSc {
                            #ident_op_res_vrts_ucc::#DesirableUcc(#VSc) => {
                                return Ok(#VSc);
                            },
                            #(#try_op_logic_res_vrts_to_try_op_logic_er_with_serde),*
                        };
                    }
                };
                let return_er_ts = {
                    let field_loc_new_ts = macros_helpers::gen_field_loc_new_ts::gen_field_loc_new_ts(
                        macros_helpers::gen_field_loc_new_ts::FieldLocFile::from(file!()),
                        macros_helpers::gen_field_loc_new_ts::FieldLocLine::from(line!()),
                        macros_helpers::gen_field_loc_new_ts::FieldLocCol::from(column!()),
                    );
                    quote::quote! {
                        Err(#ident_try_op_er_ucc::#try_op_logic_er_with_serde_ucc {
                            #op_er_with_serde_sc,
                            #field_loc_new_ts,
                        })
                    }
                };
                quote::quote! {
                    #[allow(clippy::single_call_fn)]
                    async fn #try_op_h_sc_ts(
                        #client_sc: &reqwest::Client,
                        #EndpointLocSc: #RefStr,
                        #PrmsSc: #ident_op_prms_ucc,
                        #TblSc: &str,
                    ) -> Result<#result_ok_type_ts, #ident_try_op_er_ucc> {
                        #payload_ts
                        #url_ts
                        #future_ts
                        #res_ts
                        #er_0_res_status_ts
                        #headers_ts
                        #res_text_ts
                        #expected_res_ts
                        #try_op_logic_er_with_serde_ts
                        #return_er_ts
                    }
                    pub async fn #try_op_sc_ts(
                        #EndpointLocSc: #RefStr,
                        #PrmsSc: #ident_op_prms_ucc
                    ) -> Result<#result_ok_type_ts, #ident_try_op_er_ucc> {
                        let #client_sc = reqwest::Client::new();
                        Self::#try_op_h_sc_ts(
                            &#client_sc,
                            #EndpointLocSc,
                            #PrmsSc,
                            #self_tbl_name_call_ts
                        ).await
                    }
                }
            };
            let op_h_ts = {
                let req_parts_preparation_ts = {
                    let ts0 = &gen_op_er_init_eprintln_res_ts(
                        op,
                        &header_cnt_type_app_json_not_found_syn_vrt,
                        std::panic::Location::caller(),
                    );
                    let ts1 = gen_match_ok_err_short_ts(
                        &quote::quote! {route_validators::check_body_size::check_body_size(#BodySc, *#AppStateSc.get_maximum_size_of_http_body_in_bytes()).await},
                        &quote::quote! {v_cfac9140},
                        &{
                            let ts = gen_op_er_init_eprintln_res_ts(
                                op,
                                &check_body_size_syn_vrt,
                                std::panic::Location::caller(),
                            );
                            quote::quote! {{#ts}}
                        },
                    );
                    quote::quote! {
                        let (parts, #BodySc) = #ReqSc.into_parts();
                        let headers = parts.headers;
                        if !matches!(
                            headers.get(http::header::CONTENT_TYPE),
                            Some(v_e3f6eecd) if v_e3f6eecd == http::header::HeaderValue::from_static("application/json")
                        ) {
                            #ts0
                        }
                        let body_bytes = #ts1;
                    }
                };
                let extra_validators_ts = {
                    let cmn_logic_ts = gen_logic_ts(GenPgTblAttr::CmnLogic);
                    let op_logic_ts = gen_logic_ts(op.gen_pg_tbl_attr_logic());
                    quote::quote! {
                        #cmn_logic_ts
                        #op_logic_ts
                    }
                };
                let prms_logic_ts = {
                    let prms_logic_ts0 = {
                        //todo in case of large type there is a stackoverflow. for example it was a 3.5md json file gend by cm_payload_example. 3400 fields = success. 16000 = stackoverflow
                        let ts = gen_match_ok_err_short_ts(
                            &{
                                let ident_op_payload_ucc =
                                    gen_ident_op_payload_ucc(op);
                                quote::quote! {serde_json::from_slice::<#ident_op_payload_ucc>(&#BodyBytesSc)}
                            },
                            &quote::quote! {v_9e6fcd2d},
                            &{
                                let ts = gen_op_er_init_eprintln_res_ts(
                                    op,
                                    &serde_json_syn_vrt,
                                    std::panic::Location::caller(),
                                );
                                quote::quote! {{#ts}}
                            },
                        );
                        quote::quote! {
                            let #PrmsSc = #ident_op_prms_ucc {
                                #PayloadSc: #ts
                            };
                        }
                    };
                    match &op {
                        Op::Cm
                        | Op::Co
                        | Op::Rm
                        | Op::Ro
                        | Op::Dm
                        | Op::Dlo => prms_logic_ts0,
                        Op::Um => quote::quote! {
                            #prms_logic_ts0
                            let #UpdForQueryVecSc = #PrmsSc.#PayloadSc.into_vec().into_iter()
                            .map(#ident_upd_for_query_ucc::#FromHSc)
                            .collect::<Vec<#ident_upd_for_query_ucc>>();
                        },
                        Op::Uo => quote::quote! {
                            #prms_logic_ts0
                            let #UpdForQuerySc = #ident_upd_for_query_ucc::#FromHSc(#PrmsSc.#PayloadSc);
                        },
                    }
                };
                let query_string_ts = {
                    let gen_match_ok_err_qp_ts =
                        |ts0: &dyn quote::ToTokens, ts1: &dyn quote::ToTokens, ts2: &dyn quote::ToTokens, ts3: &dyn quote::ToTokens| {
                            gen_match_ok_err_ts(&ts0, &ts1, &ts2, &ts3, &quote::quote! {{#op_er_init_qp_ts}})
                        };
                    let write_into_buffer_qp_syn_vrt_er_init_eprintln_res_creation_ts = {
                        let qp_er_write_into_buffer_ts =
                            pg_crud_macros_cmn::gen_qp_er_write_into_buffer_ts(import);
                        quote::quote! {
                            let #Er0 = #qp_er_write_into_buffer_ts;
                            #op_er_init_qp_ts
                        }
                    };
                    let incr_init_ts = quote::quote! {let mut #IncrSc: u64 = 0;};
                    let col_names_dq_ts = gen_quotes::dq_ts(&{
                        let mut acc = fields.iter().fold(
                            String::with_capacity(fields.len().saturating_mul(32)),
                            |mut acc0, el| {
                                assert!(
                                    std::fmt::Write::write_fmt(
                                        &mut acc0,
                                        format_args!("{}", el.ident),
                                    )
                                    .is_ok(),
                                    "b9fe50dc",
                                );
                                acc0.push(',');
                                acc0
                            },
                        );
                        let _: Option<char> = acc.pop();
                        acc
                    });
                    let sel_only_ids_qp_ts = {
                        let sel_only_ids_qp_init_ts = fields.iter().map(|el: &macros_helpers::field_data::SynField| gen_match_ok_err_qp_ts(
                            &{
                                let fi_dq_ts = gen_quotes::dq_ts(&el.ident);
                                let ft_as_pg_crud_pg_type_pg_type_ts = gen_as_pg_type_path_ts(&el.type0);
                                quote::quote! {#ft_as_pg_crud_pg_type_pg_type_ts #SelOnlyIdsQpSc(#import_ts SqlColRef::from(&#fi_dq_ts))}
                            },
                            &quote::quote! {v_aa341baf},
                            &quote::quote! {{
                                acc_a35168d8.push_str(&v_aa341baf);
                            }},
                            &Er0
                        ));
                        let ts0 = gen_acc_string_pop_acc_ts(
                            &quote::quote! {acc_a35168d8},
                            &quote::quote! {#(#sel_only_ids_qp_init_ts)*},
                        );
                        quote::quote! {{#ts0}}
                    };
                    let gen_if_write_is_err_short_ts = |ts: &dyn quote::ToTokens| {
                        macros_helpers::gen_if_write_is_err_ts::gen_if_write_is_err_ts(
                            &ts,
                            &write_into_buffer_qp_syn_vrt_er_init_eprintln_res_creation_ts,
                        )
                    };
                    let gen_sel_only_updd_ids_qp_ts =
                        |ts: &dyn quote::ToTokens| quote::quote! {#ts.#SelOnlyUpddIdsQpSc(&mut #IncrSc)};
                    match &op {
                        Op::Cm => {
                            let if_write_is_err_ts = gen_if_write_is_err_short_ts(&quote::quote! {
                                acc_8a58994e,
                                "({v_f4fdd10d}),"
                            });
                            let ts0 = gen_acc_string_pop_acc_ts(&quote::quote! {acc_8a58994e}, &{
                                let ts = gen_match_ok_err_qp_ts(
                                    &quote::quote! {el_1651705d.#CrQpSc(&mut #IncrSc)},
                                    &quote::quote! {v_f4fdd10d},
                                    &quote::quote! {{
                                        #if_write_is_err_ts
                                    }},
                                    &Er0,
                                );
                                quote::quote! {
                                    for el_1651705d in #PrmsSc.#PayloadSc.as_slice() {
                                        #ts
                                    }
                                }
                            });
                            quote::quote! {pg_tbl::gen_cm_query_string(
                                pg_tbl::PgTblNameRef::from(#TblSc),
                                pg_tbl::PgTblSqlFragmentRef::from(#col_names_dq_ts),
                                pg_tbl::PgTblSqlFragmentRef::from(&{
                                    #incr_init_ts
                                    #ts0
                                }),
                                pg_tbl::PgTblSqlFragmentRef::from(&#sel_only_ids_qp_ts)
                            )}
                        }
                        Op::Co => {
                            let ts = gen_match_ok_err_upd_ts(
                                &quote::quote! {#PrmsSc.#PayloadSc.#CrQpSc(&mut 0)},
                                &quote::quote! {v_3267d57d},
                            );
                            quote::quote! {
                                pg_tbl::gen_co_query_string(
                                    pg_tbl::PgTblNameRef::from(#TblSc),
                                    pg_tbl::PgTblSqlFragmentRef::from(#col_names_dq_ts),
                                    pg_tbl::PgTblSqlFragmentRef::from(&#ts),
                                    pg_tbl::PgTblSqlFragmentRef::from(&#sel_only_ids_qp_ts)
                                )
                            }
                        }
                        Op::Rm => {
                            let sel_qp_prms_payload_sel_ts =
                                gen_sel_qp_prms_payload_sel_ts(op);
                            let extra_prms_init_ts = gen_rd_or_dm_extra_prms_init_ts(
                                &RmOrDm::Rm,
                            );
                            let extra_prms_order_by_h_ts =
                                gen_quotes::dq_ts(&format!("{{}}{OrderSc} {BySc} {{}} {{}}"));
                            let pk_fi_dq_ts = gen_quotes::dq_ts(&pk_fi);
                            let order_by_col_match_ts =
                                gen_fields_named_with_comma_ts(&|el: &macros_helpers::field_data::SynField| {
                                    let fi_ucc = naming_cmn::ToTokensToUccTs::case_or_panic(&el.ident);
                                    let fi_dq_ts = gen_quotes::dq_ts(&el.ident);
                                    quote::quote! {
                                        #ident_sel_ucc::#fi_ucc(_) => #fi_dq_ts
                                    }
                                });
                            let (if_write_is_err_curly_braces_0_ts, if_write_is_err_curly_braces_1_ts) = {
                                let gen_if_write_is_err_curly_braces_short_ts = |ts: &dyn quote::ToTokens| {
                                    macros_helpers::gen_if_write_is_err_ts::gen_if_write_is_err_ts(
                                    &ts,
                                    &write_into_buffer_qp_syn_vrt_er_init_eprintln_res_creation_ts
                                )
                                };
                                (
                                    gen_if_write_is_err_curly_braces_short_ts(&quote::quote! {
                                        #ExtraPrmsSc,
                                        #extra_prms_order_by_h_ts,
                                        #PrefixSc,
                                        match &#PrmsSc.#PayloadSc.#OrderBySc.#ColSc {
                                            #order_by_col_match_ts
                                        },
                                        &order_691f662a
                                    }),
                                    gen_if_write_is_err_curly_braces_short_ts(&{
                                        let ts = gen_match_ok_err_upd_ts(
                                            &quote::quote! {#pg_crud_pg_type_wh_flt_qp_ts(
                                                &#PrmsSc.#PayloadSc.pgn,
                                                &mut #IncrSc,
                                                #import_ts SqlColRef::from(&""),
                                                #import_ts AddOprtr::from(bool::default())
                                            )},
                                            &quote::quote! {v_742be6cf},
                                        );
                                        quote::quote! {
                                            #ExtraPrmsSc,
                                            "{prefix}{}",
                                            #ts
                                        }
                                    }),
                                )
                            };
                            let if_write_is_err_order_tie_ts = macros_helpers::gen_if_write_is_err_ts::gen_if_write_is_err_ts(
                                &quote::quote! {
                                    #ExtraPrmsSc,
                                    ", {} {}",
                                    #pk_fi_dq_ts,
                                    order_691f662a
                                },
                                &write_into_buffer_qp_syn_vrt_er_init_eprintln_res_creation_ts,
                            );
                            quote::quote! {pg_tbl::gen_rm_query_string(
                                pg_tbl::PgTblNameRef::from(#TblSc),
                                pg_tbl::PgTblSqlFragmentRef::from(&#sel_qp_prms_payload_sel_ts),
                                pg_tbl::PgTblSqlFragmentRef::from(&{
                                    #incr_init_ts
                                    let mut #ExtraPrmsSc = #extra_prms_init_ts;
                                    let #PrefixSc = if extra_prms.is_empty() {""} else {" "};
                                    let order_691f662a = match #PrmsSc.#PayloadSc.#OrderBySc.#OrderSc.as_ref() {
                                        Some(#import_ts Order::Asc) | None => "asc",
                                        Some(#import_ts Order::Desc) => "desc",
                                    };
                                    #if_write_is_err_curly_braces_0_ts
                                    if !matches!(
                                        &#PrmsSc.#PayloadSc.#OrderBySc.#ColSc,
                                        #ident_sel_ucc::#pk_fi_ucc_ts(_)
                                    ) {
                                        #if_write_is_err_order_tie_ts
                                    }
                                    #if_write_is_err_curly_braces_1_ts
                                    #ExtraPrmsSc
                                })
                            )}
                        }
                        Op::Ro => {
                            let sel_qp_prms_payload_sel_ts =
                                gen_sel_qp_prms_payload_sel_ts(op);
                            let ts = gen_match_ok_err_upd_ts(
                                &quote::quote! {#pg_crud_pg_type_wh_flt_qp_ts(
                                    &#PrmsSc.#PayloadSc.#pk_fi,
                                    &mut 0,
                                    #import_ts SqlColRef::from(&Self::#PkSc()),
                                    #import_ts AddOprtr::from(false)
                                )},
                                &quote::quote! {v_be9e7b7d},
                            );
                            quote::quote! {pg_tbl::gen_ro_query_string(
                                pg_tbl::PgTblNameRef::from(#TblSc),
                                pg_tbl::PgTblSqlFragmentRef::from(&#sel_qp_prms_payload_sel_ts),
                                pg_tbl::PgTblSqlFragmentRef::from(&#ts)
                            )}
                        }
                        Op::Um => {
                            let gen_match_upd_qp_pk_op_ts =
                                |ts: &dyn quote::ToTokens| gen_match_upd_qp_pk_ts(op, &ts);
                            let ts0 = gen_acc_string_pop_acc_ts(
                                &quote::quote! {acc_b86a253a},
                                &gen_fields_named_without_pk_without_comma_ts(&|el: &macros_helpers::field_data::SynField| {
                                    let fi = &el.ident;
                                    let fi_dq_ts = gen_quotes::dq_ts(&fi);
                                    let is_fi_upd_exists_sc = naming::prm::IsSelfUpdExistSc::from_tokens(&fi);
                                    let upd_qp_fi_sc = naming::prm::UpdQpSelfSc::from_tokens(&fi);
                                    let for_el_upd_fi_exists_ts = gen_for_el_in_upd_for_query_vec_ts(&quote::quote! {
                                        if el_a72f3eac.#fi.is_some() {
                                            #is_fi_upd_exists_sc = true;
                                            break;
                                        }
                                    });
                                    let for_el_upd_fi_qp_ts = gen_for_el_in_upd_for_query_vec_fi_ts(
                                        &fi,
                                        &quote::quote! {v_3ea04126},
                                        &{
                                            let ts0 = gen_match_ok_err_upd_ts(
                                                &quote::quote! {el_a72f3eac.#UpdQpPkSc(&mut #IncrSc)},
                                                &quote::quote! {v_00890100},
                                            );
                                            let ts1 = gen_match_ok_err_upd_ts(
                                                &quote::quote! {#ident_upd_for_query_ucc::#upd_qp_fi_sc(v_3ea04126, &mut #IncrSc)},
                                                &quote::quote! {v_8797585c},
                                            );
                                            quote::quote! {
                                                acc_8ad06c8c.push_str(&pg_tbl::#GenWhenColIdThenVUmQpSc(
                                                    pg_tbl::PgTblSqlFragmentRef::from(Self::#PkSc()),
                                                    pg_tbl::PgTblSqlFragmentRef::from(&#ts0),
                                                    pg_tbl::PgTblSqlFragmentRef::from(&#ts1)
                                                ));
                                            }
                                        },
                                    );
                                    quote::quote! {
                                        {
                                            let mut #is_fi_upd_exists_sc = false;
                                            #for_el_upd_fi_exists_ts
                                            if #is_fi_upd_exists_sc {
                                                acc_b86a253a.push_str(&
                                                    pg_tbl::gen_col_eqs_case_acc_else_col_end_comma_um_qp(
                                                        pg_tbl::PgTblSqlFragmentRef::from(#fi_dq_ts),
                                                        pg_tbl::PgTblSqlFragmentRef::from(&{
                                                            let mut acc_8ad06c8c = #StringTs::default();
                                                            #for_el_upd_fi_qp_ts
                                                            acc_8ad06c8c
                                                        })
                                                    )
                                                );
                                            }
                                        }
                                    }
                                }),
                            );
                            let ts1 = gen_acc_string_pop_acc_ts(
                                &quote::quote! {acc_a95eb175},
                                &gen_for_el_in_upd_for_query_vec_ts(&gen_if_write_is_err_short_ts(
                                    &{
                                        let match_upd_qp_pk_op_ts =
                                            gen_match_upd_qp_pk_op_ts(
                                                &quote::quote! {el_a72f3eac},
                                            );
                                        quote::quote! {
                                            acc_a95eb175,
                                            "{},",
                                            #match_upd_qp_pk_op_ts
                                        }
                                    },
                                )),
                            );
                            let for_el_sel_only_updd_ids_qp_ts =
                                gen_for_el_in_upd_for_query_vec_ts(&gen_match_ok_err_qp_ts(
                                    &gen_sel_only_updd_ids_qp_ts(&quote::quote! {el_a72f3eac}),
                                    &quote::quote! {v_4f536654},
                                    &quote::quote! {{
                                        acc_fd44b0aa.push_str(&v_4f536654);
                                    }},
                                    &Er0,
                                ));
                            quote::quote! {
                                {
                                    #incr_init_ts
                                    let els = {
                                        #ts0
                                    };
                                    let pks = {
                                        #ts1
                                    };
                                    let return_cols = {
                                        let mut acc_fd44b0aa = String::with_capacity(#UpdForQueryVecSc.len().saturating_mul(32));
                                        #for_el_sel_only_updd_ids_qp_ts
                                        acc_fd44b0aa
                                    };
                                    pg_tbl::gen_um_query_string(
                                        pg_tbl::PgTblNameRef::from(#TblSc),
                                        pg_tbl::PgTblSqlFragmentRef::from(&els),
                                        pg_tbl::PgTblSqlFragmentRef::from(Self::#PkSc()),
                                        pg_tbl::PgTblSqlFragmentRef::from(&pks),
                                        pg_tbl::PgTblSqlFragmentRef::from(&return_cols)
                                    )
                                }
                            }
                        }
                        Op::Uo => {
                            let extra_prms_modification_ts = gen_fields_named_without_pk_without_comma_ts(
                                &|el: &macros_helpers::field_data::SynField| {
                                    let fi = &el.ident;
                                    let fi_dq_ts = gen_quotes::dq_ts(&fi);
                                    let gen_col_queals_v_comma_uo_qp_sc =
                                        naming::GenColQuealsVCommaUoQpSc;
                                    let upd_qp_fi_sc = naming::prm::UpdQpSelfSc::from_tokens(&fi);
                                    gen_if_let_some_ts(
                                        &quote::quote! {v_2d144436},
                                        &quote::quote! {&#UpdForQuerySc.#fi},
                                        &{
                                            let ts = gen_match_ok_err_upd_ts(
                                                &quote::quote! {#ident_upd_for_query_ucc::#upd_qp_fi_sc(v_2d144436, &mut #IncrSc)},
                                                &quote::quote! {v_1ec12051},
                                            );
                                            quote::quote! {
                                                acc_683e37b8.push_str(&pg_tbl::#gen_col_queals_v_comma_uo_qp_sc(
                                                    pg_tbl::PgTblSqlFragmentRef::from(#fi_dq_ts),
                                                    pg_tbl::PgTblSqlFragmentRef::from(&#ts)
                                                ));
                                            }
                                        },
                                    )
                                },
                            );
                            let extra_prms_pk_modification_ts =
                                gen_match_upd_qp_pk_ts(op, &quote::quote! {#UpdForQuerySc});
                            let acc_string_pop_cols_ts = gen_acc_string_pop_acc_ts(
                                &quote::quote! {acc_683e37b8},
                                &extra_prms_modification_ts,
                            );
                            let ts = gen_match_ok_err_upd_ts(
                                &gen_sel_only_updd_ids_qp_ts(&UpdForQuerySc),
                                &quote::quote! {v_7f0d86a1},
                            );
                            quote::quote! {
                                {
                                    #incr_init_ts
                                    let #ColsSc = {
                                        #acc_string_pop_cols_ts
                                    };
                                    let #PkQpSc = #extra_prms_pk_modification_ts;
                                    let return_cols = #ts;
                                    pg_tbl::gen_uo_query_string(
                                        pg_tbl::PgTblNameRef::from(#TblSc),
                                        pg_tbl::PgTblSqlFragmentRef::from(&#ColsSc),
                                        pg_tbl::PgTblSqlFragmentRef::from(Self::#PkSc()),
                                        pg_tbl::PgTblSqlFragmentRef::from(&#PkQpSc),
                                        pg_tbl::PgTblSqlFragmentRef::from(&return_cols)
                                    )
                                }
                            }
                        }
                        Op::Dm => {
                            let extra_prms_init_ts = gen_rd_or_dm_extra_prms_init_ts(
                                &RmOrDm::Dm,
                            );
                            quote::quote! {pg_tbl::gen_dm_query_string(
                                pg_tbl::PgTblNameRef::from(#TblSc),
                                pg_tbl::PgTblSqlFragmentRef::from(&{
                                    #incr_init_ts
                                    #extra_prms_init_ts
                                }),
                                pg_tbl::PgTblSqlFragmentRef::from(Self::#PkSc()),
                            )}
                        }
                        Op::Dlo => quote::quote! {pg_tbl::gen_dlo_query_string(
                            pg_tbl::PgTblNameRef::from(#TblSc),
                            pg_tbl::PgTblSqlFragmentRef::from(Self::#PkSc()),
                        )},
                    }
                };
                let binded_query_ts = {
                    let op_er_init_try_bind_ts = gen_op_er_init_eprintln_res_ts(
                        op,
                        &try_bind_syn_vrt,
                        std::panic::Location::caller(),
                    );
                    let gen_match_qb_or_err_short_ts =
                        |ts0: &dyn quote::ToTokens, ts1: &dyn quote::ToTokens| {
                            gen_match_qb_or_err_ts(&ts0, &ts1, &op_er_init_try_bind_ts)
                        };
                    match &op {
                        Op::Cm => {
                            let ts = gen_match_qb_or_err_short_ts(
                                &quote::quote! {el_7f862135.#CrQbSc(#import_ts SqlxPostgresQuery::from(#QuerySc)).map(#import_ts SqlxPostgresQuery::into_inner).map_err(#import_ts SqlxPostgresQueryBindEr::into_inner)},
                                &quote::quote! {v_011a3eb4},
                            );
                            quote::quote! {
                                for el_7f862135 in #PrmsSc.#PayloadSc.into_vec() {
                                    #ts
                                }
                            }
                        }
                        Op::Co => gen_match_qb_or_err_short_ts(
                            &quote::quote! {#PrmsSc.#PayloadSc.#CrQbSc(#import_ts SqlxPostgresQuery::from(#QuerySc)).map(#import_ts SqlxPostgresQuery::into_inner).map_err(#import_ts SqlxPostgresQueryBindEr::into_inner)},
                            &quote::quote! {v_06f852cd},
                        ),
                        Op::Rm => {
                            let query_pg_type_wh_flt_qb_prms_payload_wh_query_ts = gen_query_pg_type_wh_flt_qb_prms_payload_wh_query_ts(op);
                            let ts = gen_match_qb_or_err_short_ts(
                                &quote::quote! {#pg_crud_pg_type_wh_flt_qb_ts(
                                    #PrmsSc.#PayloadSc.pgn,
                                    #import_ts SqlxPostgresQuery::from(#QuerySc),
                                ).map(#import_ts SqlxPostgresQuery::into_inner).map_err(#import_ts SqlxPostgresQueryBindEr::into_inner)},
                                &quote::quote! {v_9f7e487b},
                            );
                            quote::quote! {
                                #query_pg_type_wh_flt_qb_prms_payload_wh_query_ts
                                #ts
                            }
                        }
                        Op::Ro => gen_match_qb_or_err_short_ts(
                            &quote::quote! {#pg_crud_pg_type_wh_flt_qb_ts(
                                #PrmsSc.#PayloadSc.#pk_fi,
                                #import_ts SqlxPostgresQuery::from(#QuerySc)
                            ).map(#import_ts SqlxPostgresQuery::into_inner).map_err(#import_ts SqlxPostgresQueryBindEr::into_inner)},
                            &quote::quote! {v_80ee6983},
                        ),
                        Op::Um => {
                            let fields_named_without_pk_upd_assign_ts =
                                gen_fields_named_without_pk_without_comma_ts(&|el: &macros_helpers::field_data::SynField| {
                                    gen_for_el_in_upd_for_query_vec_fi_ts(
                                        &el.ident,
                                        &quote::quote! {v_2edaa480},
                                        &{
                                            let ts = gen_match_qb_or_err_short_ts(
                                                &{
                                                    let as_pg_crud_pg_type_pg_type_ts =
                                                        gen_as_pg_type_path_ts(&el.type0);
                                                    quote::quote! {#as_pg_crud_pg_type_pg_type_ts #UpdQbSc(
                                                        v_2edaa480.#VSc.clone(),
                                                        #import_ts SqlxPostgresQuery::from(#QuerySc),
                                                    ).map(#import_ts SqlxPostgresQuery::into_inner).map_err(#import_ts SqlxPostgresQueryBindEr::into_inner)}
                                                },
                                                &quote::quote! {v_600e67dc},
                                            );
                                            quote::quote! {
                                                if let Err(er_981062db) = #QuerySc.try_bind(el_a72f3eac.#pk_fi) {
                                                    let #Er0 = er_981062db.to_string();
                                                    #op_er_init_try_bind_ts
                                                }
                                                #ts
                                            }
                                        },
                                    )
                                });
                            let pk_upd_assign_ts = gen_for_el_in_upd_for_query_vec_ts(
                                &gen_match_qb_or_err_short_ts(
                                    &quote::quote! {#pk_ft_as_pg_type_ts #UpdQbSc(
                                        el_a72f3eac.#pk_fi,
                                        #import_ts SqlxPostgresQuery::from(#QuerySc),
                                    ).map(#import_ts SqlxPostgresQuery::into_inner).map_err(#import_ts SqlxPostgresQueryBindEr::into_inner)},
                                    &quote::quote! {v_c40a4522},
                                ),
                            );
                            let binded_query_sel_only_updd_ids_qb_ts =
                                gen_fields_named_without_pk_without_comma_ts(&|el: &macros_helpers::field_data::SynField| {
                                    gen_for_el_in_upd_for_query_vec_fi_ts(
                                        &el.ident,
                                        &quote::quote! {v_47030ac2},
                                        &gen_match_qb_or_err_short_ts(
                                            &{
                                                let as_pg_crud_pg_type_pg_type_ts =
                                                    gen_as_pg_type_path_ts(&el.type0);
                                                quote::quote! {#as_pg_crud_pg_type_pg_type_ts sel_only_updd_ids_qb(
                                                    &v_47030ac2.#VSc,
                                                    #import_ts SqlxPostgresQuery::from(#QuerySc)
                                                ).map(#import_ts SqlxPostgresQuery::into_inner).map_err(#import_ts SqlxPostgresQueryBindEr::into_inner)}
                                            },
                                            &quote::quote! {v_c5b79b95},
                                        ),
                                    )
                                });
                            quote::quote! {
                                #fields_named_without_pk_upd_assign_ts
                                #pk_upd_assign_ts
                                #binded_query_sel_only_updd_ids_qb_ts
                            }
                        }
                        Op::Uo => {
                            let gen_binded_query_ts =
                                |var_name, method_name| {
                                    gen_fields_named_without_pk_without_comma_ts(&|el: &macros_helpers::field_data::SynField| {
                                        gen_if_let_some_ts(
                                            &var_name,
                                            &{
                                                let fi = &el.ident;
                                                quote::quote! {&#UpdForQuerySc.#fi}
                                            },
                                            &gen_match_qb_or_err_short_ts(
                                                &{
                                                    let as_pg_crud_pg_type_pg_type_ts =
                                                        gen_as_pg_type_path_ts(&el.type0);
                                                    quote::quote! {#as_pg_crud_pg_type_pg_type_ts #method_name}
                                                },
                                                &quote::quote! {v_result},
                                            ),
                                        )
                                    })
                                };
                            let binded_query_modifications_ts = gen_binded_query_ts(
                                quote::quote! {v_ed87c152},
                                quote::quote! {#UpdQbSc(v_ed87c152.#VSc.clone(), #import_ts SqlxPostgresQuery::from(#QuerySc)).map(#import_ts SqlxPostgresQuery::into_inner).map_err(#import_ts SqlxPostgresQueryBindEr::into_inner)},
                            );
                            let binded_query_pk_modification_ts = gen_match_qb_or_err_short_ts(
                                &quote::quote! {#pk_ft_as_pg_type_ts #UpdQbSc(
                                    #UpdForQuerySc.#pk_fi,
                                    #import_ts SqlxPostgresQuery::from(#QuerySc),
                                ).map(#import_ts SqlxPostgresQuery::into_inner).map_err(#import_ts SqlxPostgresQueryBindEr::into_inner)},
                                &quote::quote! {v_d64bac39},
                            );
                            let binded_query_sel_only_updd_ids_qb_ts = gen_binded_query_ts(
                                quote::quote! {v_b2902425},
                                quote::quote! {sel_only_updd_ids_qb(&v_b2902425.#VSc, #import_ts SqlxPostgresQuery::from(#QuerySc)).map(#import_ts SqlxPostgresQuery::into_inner).map_err(#import_ts SqlxPostgresQueryBindEr::into_inner)},
                            );
                            quote::quote! {
                                #binded_query_modifications_ts
                                #binded_query_pk_modification_ts
                                #binded_query_sel_only_updd_ids_qb_ts
                            }
                        }
                        Op::Dm => {
                            gen_query_pg_type_wh_flt_qb_prms_payload_wh_query_ts(
                                op,
                            )
                        }
                        Op::Dlo => gen_match_qb_or_err_short_ts(
                            &quote::quote! {#import_ts PgTypeWhFlt::qb(
                                #PrmsSc.#PayloadSc.#pk_fi,
                                #import_ts SqlxPostgresQuery::from(#QuerySc)
                            ).map(#import_ts SqlxPostgresQuery::into_inner).map_err(#import_ts SqlxPostgresQueryBindEr::into_inner)},
                            &quote::quote! {v_3099ea0f},
                        ),
                    }
                };
                let acquire_pool_and_connection_ts = {
                    let pg_syn_vrt_er_init_eprintln_res_creation_ts =
                        gen_op_er_init_eprintln_res_ts(
                            op,
                            &pg_syn_vrt,
                            std::panic::Location::caller(),
                        );
                    let ts = gen_match_ok_err_short_ts(
                        &quote::quote! {#AppStateSc.get_sqlx_pg_pool().as_ref().acquire().await},
                        &quote::quote! {v_4535ee48},
                        &quote::quote! {{
                            #pg_syn_vrt_er_init_eprintln_res_creation_ts
                        }},
                    );
                    let ts0 = gen_match_ok_err_short_ts(
                        &quote::quote! {sqlx::Acquire::acquire(&mut #PoolConnectionSc).await},
                        &quote::quote! {v_61ae8f84},
                        &quote::quote! {{
                            #pg_syn_vrt_er_init_eprintln_res_creation_ts
                        }},
                    );
                    quote::quote! {
                        let mut #PoolConnectionSc = #ts;
                        let #ExecutorAcquireSc = #ts0;
                    }
                };
                let pg_logic_ts = {
                    let gen_match_ident_rd_ids_as_from_row_from_row_ts = |ts: &dyn quote::ToTokens| {
                        gen_match_ok_err_short_ts(
                            &quote::quote! {<#ident_rd_ids_ucc as sqlx::FromRow<'_, sqlx::postgres::PgRow>>::from_row(&v_b27d7d79)},
                            &quote::quote! {v_33759463},
                            &ts,
                        )
                    };
                    let gen_cr_upd_dm_fetch_ts =
                        |cr_or_upd_or_dm: &CrOrUpdOrDm| {
                            let op_cr_upd_dm = Op::from(cr_or_upd_or_dm);
                            gen_fetch_ts(
                                &ExecutorSc,
                                &match &cr_or_upd_or_dm {
                                    CrOrUpdOrDm::Cr
                                    | CrOrUpdOrDm::Upd => {
                                        let ts = gen_match_ident_rd_ids_as_from_row_from_row_ts(&gen_drop_rows_match_pg_transaction_rollback_await_h_ts(
                                            &op_cr_upd_dm,
                                            std::panic::Location::caller(),
                                        ));
                                        quote::quote! {Some(#ts)}
                                    }
                                    CrOrUpdOrDm::Del => gen_sqlx_row_try_get_pk_ts(
                                        &pk_ft_as_pg_type_rd_ucc,
                                        &quote::quote! {Some(v_69ecb6a9)},
                                        &gen_drop_rows_match_pg_transaction_rollback_await_h_ts(
                                            &op_cr_upd_dm,
                                            std::panic::Location::caller(),
                                        ),
                                    ),
                                },
                                &gen_drop_rows_match_pg_transaction_rollback_await_h_ts(
                                    &op_cr_upd_dm,
                                    std::panic::Location::caller(),
                                ),
                                &ShouldWrapIntoV::True,
                            )
                        };
                    let gen_cr_upd_dlo_fetch_ts =
                        |cr_or_upd_or_dlo: &CrOrUpdOrDlo| {
                            wrap_into_v_ts(&{
                                let op0 = Op::from(cr_or_upd_or_dlo);
                                let ts = gen_match_pg_transaction_rollback_await_ts(
                                    &op0,
                                    std::panic::Location::caller(),
                                );
                                gen_fetch_one_ts(
                                    &ExecutorSc,
                                    &match cr_or_upd_or_dlo {
                                        CrOrUpdOrDlo::Cr | CrOrUpdOrDlo::Upd => gen_match_ident_rd_ids_as_from_row_from_row_ts(&ts),
                                        CrOrUpdOrDlo::Del => gen_sqlx_row_try_get_pk_ts(
                                            &quote::quote! {#pk_ft_as_pg_type_rd_ucc},
                                            &quote::quote! {v_69ecb6a9},
                                            &ts,
                                        ),
                                    },
                                    &ts,
                                )
                            })
                        };
                    match &op {
                        Op::Cm => wrap_into_pg_transaction_begin_commit_ts(
                            op,
                            &gen_cr_upd_dm_fetch_ts(&CrOrUpdOrDm::Cr),
                        ),
                        Op::Co => wrap_into_pg_transaction_begin_commit_ts(
                            op,
                            &gen_cr_upd_dlo_fetch_ts(&CrOrUpdOrDlo::Cr),
                        ),
                        Op::Rm => {
                            let fetch_ts = gen_fetch_ts(
                                &ExecutorAcquireSc,
                                &{
                                    let match_ident_rd_try_from_sqlx_pg_pg_row_with_not_empty_unq_vec_ident_sel_ts = gen_match_ident_rd_try_from_sqlx_pg_pg_row_with_not_empty_unq_vec_ident_sel_ts(&RmOrRo::Rm);
                                    quote::quote! {Some(#match_ident_rd_try_from_sqlx_pg_pg_row_with_not_empty_unq_vec_ident_sel_ts)}
                                },
                                &gen_op_er_init_eprintln_res_ts(
                                    op,
                                    &pg_syn_vrt,
                                    std::panic::Location::caller(),
                                ),
                                &ShouldWrapIntoV::False,
                            );
                            quote::quote! {{
                                #fetch_ts
                            }}
                        },
                        Op::Ro => gen_fetch_one_ts(
                            &ExecutorAcquireSc,
                            &gen_match_ident_rd_try_from_sqlx_pg_pg_row_with_not_empty_unq_vec_ident_sel_ts(&RmOrRo::Ro),
                            &gen_op_er_init_eprintln_res_ts(op, &pg_syn_vrt, std::panic::Location::caller()),
                        ),
                        Op::Um => wrap_into_pg_transaction_begin_commit_ts(
                            op,
                            &gen_cr_upd_dm_fetch_ts(&CrOrUpdOrDm::Upd),
                        ),
                        Op::Uo => wrap_into_pg_transaction_begin_commit_ts(
                            op,
                            &gen_cr_upd_dlo_fetch_ts(&CrOrUpdOrDlo::Upd),
                        ),
                        Op::Dm => wrap_into_pg_transaction_begin_commit_ts(
                            op,
                            &gen_cr_upd_dm_fetch_ts(&CrOrUpdOrDm::Del),
                        ),
                        Op::Dlo => wrap_into_pg_transaction_begin_commit_ts(
                            op,
                            &gen_cr_upd_dlo_fetch_ts(&CrOrUpdOrDlo::Del),
                        ),
                    }
                };
                let wraped_into_axum_res_ts = wrap_into_axum_res_ts(
                    &{
                        let ident_op_res_vrts_ucc = gen_ident_op_res_vrts_ucc(op);
                        quote::quote! {#ident_op_res_vrts_ucc::#DesirableUcc(#VSc)}
                    },
                    &op.desirable_status_code().to_http_status_code_ts(),
                    &AddReturn::False,
                );
                quote::quote! {
                    #[allow(clippy::single_call_fn)]
                    async fn #op_h_sc_ts(
                        #AppStateSc: axum::extract::State<#std_sync_arc_combination_of_app_state_logic_traits_ts>,
                        #ReqSc: axum::extract::Request,
                        #TblSc: &str,
                    ) -> axum::response::Response {
                        #req_parts_preparation_ts
                        #extra_validators_ts
                        #prms_logic_ts
                        let #QueryStringSc = #query_string_ts;
                        //println!("{}", #QueryStringSc);
                        let #BindedQuerySc = {
                            let mut #QuerySc = #sqlx_query_sqlx_pg_ts(&#QueryStringSc);
                            #binded_query_ts
                            #QuerySc
                        };
                        #acquire_pool_and_connection_ts
                        let #VSc = {
                            #pg_logic_ts
                        };
                        #wraped_into_axum_res_ts
                    }
                }
            };
            let op_ts = {
                quote::quote! {
                    #[allow(clippy::single_call_fn)]
                    pub async fn #op_sc_ts(
                        #AppStateSc: axum::extract::State<#std_sync_arc_combination_of_app_state_logic_traits_ts>,
                        #ReqSc: axum::extract::Request,
                    ) -> axum::response::Response {
                        Self::#op_h_sc_ts(#AppStateSc, #ReqSc, #self_tbl_name_call_ts).await
                    }
                }
            };
            let op_payload_example_ts = {
                let op_payload_example_sc = op.op_payload_example_sc();
                let ts = wrap_into_axum_res_ts(
                    &{
                        let ident_op_payload_ucc = gen_ident_op_payload_ucc(op);
                        quote::quote! {<#ident_op_payload_ucc as #import_ts #DfltSomeOneElUcc>::#DfltSomeOneElSc()}
                    },
                    &quote::quote! {http::StatusCode::OK},
                    &AddReturn::False,
                );
                quote::quote! {
                    #MustUse
                    pub fn #op_payload_example_sc() -> axum::response::Response {
                        #ts
                    }
                }
            };
            quote::quote! {
                #op_h_ts
                #op_ts
                #try_op_ts
                #op_payload_example_ts
            }
        });
        content_ts.push({
            let payload_ts = {
                let gen_prms_payload_and_dflt_ts =
                    |dcl_ts: &dyn quote::ToTokens, dflt_init_ts: &dyn quote::ToTokens| {
                        let ident_op_payload_ucc = gen_ident_op_payload_ucc(op);
                        let ident_op_payload_ts = {
                            let (derive_clone, derive_copy) = op.derive_clone_and_copy();
                            let payload_builder_without_deserialize = macros_helpers::derive_ts_builder::DTsBuilder::new()
                                .make_pub()
                                .d_debug()
                                .d_clone_if(derive_clone)
                                .d_copy_if(derive_copy)
                                .d_serde_serialize();
                            let payload_builder = if matches!(op, Op::Cm)
                                && gen_pg_tbl_input_model.config.cm_max_items.is_some()
                            {
                                payload_builder_without_deserialize
                            } else {
                                payload_builder_without_deserialize.d_serde_deserialize()
                            };
                            let ident_op_payload_struct_ts = payload_builder
                                .d_utoipa_to_schema()
                                .build_struct(&quote::quote! {#[serde(deny_unknown_fields)]},&ident_op_payload_ucc, &proc_macro2::TokenStream::new(), &dcl_ts);
                            quote::quote! {
                                #AllowClippyArbitrarySrcItemOrdering
                                #ident_op_payload_struct_ts
                            }
                        };
                        let impl_pg_crud_dflt_some_one_el_for_op_payload_ts =
                            gen_impl_pg_crud_dflt_some_one_el_for_tokens_no_lt_ts(
                                &ident_op_payload_ucc,
                                &quote::quote! {Self #dflt_init_ts},
                            );
                        quote::quote! {
                            #ident_op_payload_ts
                            #impl_pg_crud_dflt_some_one_el_for_op_payload_ts
                        }
                    };
                match &op {
                    Op::Cm => {
                        let ident_op_payload_ucc = gen_ident_op_payload_ucc(op);
                        let vec_ident_cr_ts =
                            pg_crud_macros_cmn::gen_vec_tokens_dcl_ts(&ident_cr_ucc);
                        let payload_ts = gen_prms_payload_and_dflt_ts(
                            &quote::quote! {(#vec_ident_cr_ts);},
                            &quote::quote! {(vec![#PgCrudCmnDfltSomeOneElCall])},
                        );
                        let limited_deserialize_ts = gen_pg_tbl_input_model.config.cm_max_items.map_or_else(
                            proc_macro2::TokenStream::new,
                            |limit| {
                                let limit_value = limit.0;
                                quote::quote! {
                                impl<'de> serde::Deserialize<'de> for #ident_op_payload_ucc {
                                    fn deserialize<Deserializer>(deserializer: Deserializer) -> Result<Self, Deserializer::Error>
                                    where
                                        Deserializer: serde::Deserializer<'de>,
                                    {
                                        let raw = <#vec_ident_cr_ts as serde::Deserialize>::deserialize(deserializer)?;
                                        if raw.len() > #limit_value {
                                            return Err(serde::de::Error::custom(format!("bulk create item count {} exceeds limit {}", raw.len(), #limit_value)));
                                        }
                                        Ok(Self(raw))
                                    }
                                }
                            }
                            },
                        );
                        quote::quote! {
                            #payload_ts
                            #limited_deserialize_ts
                            impl #ident_op_payload_ucc {
                                #[must_use]
                                pub const fn as_slice(&self) -> &[#ident_cr_ucc] {
                                    self.0.as_slice()
                                }
                                #[must_use]
                                pub fn into_vec(self) -> #vec_ident_cr_ts {
                                    self.0
                                }
                            }
                        }
                    }
                    Op::Rm => gen_prms_payload_and_dflt_ts(
                        &quote::quote! {{
                            #pub_wh_opt_ident_wh_ts,
                            #pub_sel_pg_crud_not_empty_unq_vec_ident_sel_ts,
                            pub #OrderBySc: #pg_crud_order_by_ts<#ident_sel_ucc>,
                            pub #PgnSc: #import_ts PgnStartsWithZero,
                        }},
                        &{
                            let ts = gen_fi_dflt_some_one_el_call_ts(&PgnSc);
                            quote::quote! {{
                                #wh_many_pg_crud_dflt_some_one_el_call_ts,
                                #sel_pg_crud_dflt_some_one_el_call_ts,
                                #OrderBySc: #import_ts OrderBy {
                                    #ColSc: #ident_sel_ucc::#pk_fi_ucc_ts(
                                        #PgCrudCmnDfltSomeOneElCall
                                    ),
                                    #OrderSc: Some(
                                        #PgCrudCmnDfltSomeOneElCall
                                    ),
                                },
                                #ts,
                            }}
                        },
                    ),
                    Op::Ro => gen_prms_payload_and_dflt_ts(
                        &{
                            let pub_h_pk_fi_pk_inn_type_h_ts =
                                gen_pub_h_pk_fi_pk_inn_type_h_ts(
                                    &naming::prm::SelfRdUcc::from_type_last_segment(pk_ft),
                                );
                            quote::quote! {{
                                #pub_h_pk_fi_pk_inn_type_h_ts,
                                #pub_sel_pg_crud_not_empty_unq_vec_ident_sel_ts,
                            }}
                        },
                        &{
                            let ts = gen_fi_dflt_some_one_el_call_ts(&pk_fi);
                            quote::quote! {{
                                #ts,
                                #sel_pg_crud_dflt_some_one_el_call_ts
                            }}
                        },
                    ),
                    Op::Um => {
                        let ident_op_payload_ucc = gen_ident_op_payload_ucc(op);
                        let vec_ident_upd_ts = pg_crud_macros_cmn::gen_vec_tokens_dcl_ts(&ident_upd_ucc);
                        let ident_op_payload_vec_ts = serde_ser_utoipa_d_ts_builder
                            .build_struct(
                                &proc_macro2::TokenStream::new(),
                                &ident_op_payload_ucc,
                                &proc_macro2::TokenStream::new(),
                                &quote::quote! {(#vec_ident_upd_ts);},
                            );
                        let ident_op_payload_try_new_er_ucc =
                            gen_ident_op_suffix_ts(op, "PayloadTryNewEr");
                        let ident_op_payload_try_new_er_ts = pg_crud_macros_cmn::ts_helpers::er_enum_d_ts_builder()
                        .build_enum(
                                &proc_macro2::TokenStream::new(),
                                &ident_op_payload_try_new_er_ucc,
                                &proc_macro2::TokenStream::new(),
                                &quote::quote! {{
                                    #NotUnqPkUcc {
                                        #[eo_to_err_string]
                                        #NotUnqPkSc: #pk_ft_upd_ts,
                                        #[eo_to_err_string]
                                        loc: loc_lib::loc::Loc,
                                    }
                                }},
                            );
                        let impl_pub_try_new_for_ident_op_payload_ts = quote::quote! {
                            impl #ident_op_payload_ucc {
                                #[must_use]
                                pub fn into_vec(self) -> #vec_ident_upd_ts {
                                    self.0
                                }
                                pub fn try_new(
                                    #VSc: #vec_ident_upd_ts,
                                ) -> Result<Self, #ident_op_payload_try_new_er_ucc> {
                                let mut acc_6bf275fc = std::collections::HashSet::with_capacity(#VSc.len());
                                for el_35facc3a in &#VSc {
                                    if !acc_6bf275fc.insert(&el_35facc3a.#pk_fi) {
                                        return Err(#ident_op_payload_try_new_er_ucc::#NotUnqPkUcc {
                                            #NotUnqPkSc: el_35facc3a.#pk_fi,
                                            loc: loc_macros::loc!(),
                                        });
                                    }
                                }
                                Ok(Self(#VSc))
                                }
                            }
                        };
                        let um_item_limit_check_ts = gen_pg_tbl_input_model.config.um_max_items.map_or_else(
                            proc_macro2::TokenStream::new,
                            |limit| {
                                let limit_value = limit.0;
                                quote::quote! {
                                    if raw.len() > #limit_value {
                                        return Err(_serde::de::Error::custom(format!("bulk update item count {} exceeds limit {}", raw.len(), #limit_value)));
                                    }
                                }
                            },
                        );
                        let impl_de_for_ident_um_payload_ts = quote::quote! {
                            #[allow(unused_qualifications)]
                            #[allow(clippy::absolute_paths)]
                            #AllowClippyArbitrarySrcItemOrdering
                            const _: () = {
                                #[allow(unused_extern_crates, clippy::useless_attribute)]
                                extern crate serde as _serde;
                                #[automatically_derived]
                                impl<'de> _serde::Deserialize<'de> for #ident_op_payload_ucc {
                                    fn deserialize<__D>(__deserializer: __D) -> Result<Self, __D::Error>
                                    where
                                        __D: _serde::Deserializer<'de>,
                                    {
                                        let raw = <#vec_ident_upd_ts as _serde::Deserialize>::deserialize(__deserializer)?;
                                        #um_item_limit_check_ts
                                        Self::try_new(raw).map_err(|er| _serde::de::Error::custom(format!("{er:?}")))
                                    }
                                }
                            };
                        };
                        let impl_pg_crud_dflt_some_one_el_for_op_payload_ts =
                            gen_impl_pg_crud_dflt_some_one_el_for_tokens_no_lt_ts(
                                &ident_op_payload_ucc,
                                &quote::quote! {
                                    Self(vec![#PgCrudCmnDfltSomeOneElCall])
                                },
                            );
                        quote::quote! {
                            #ident_op_payload_vec_ts
                            #ident_op_payload_try_new_er_ts
                            #impl_pub_try_new_for_ident_op_payload_ts
                            #impl_de_for_ident_um_payload_ts
                            #impl_pg_crud_dflt_some_one_el_for_op_payload_ts
                        }
                    },
                    Op::Dm => gen_prms_payload_and_dflt_ts(
                        &quote::quote! {{#pub_wh_opt_ident_wh_ts}},
                        &quote::quote! {{#wh_many_pg_crud_dflt_some_one_el_call_ts}},
                    ),
                    Op::Dlo => gen_prms_payload_and_dflt_ts(
                        &{
                            let ts = gen_pub_h_pk_fi_pk_inn_type_h_ts(
                                &naming::prm::SelfRdUcc::from_type_last_segment(pk_ft),
                            );
                            quote::quote! {{#ts}}
                        },
                        &{
                            let ts = gen_fi_dflt_some_one_el_call_ts(&pk_fi);
                            quote::quote! {{#ts}}
                        },
                    ),
                    Op::Co | Op::Uo => proc_macro2::TokenStream::new(),
                }
            };
            let prms_ts = {
                let (derive_clone, derive_copy) = op.derive_prms_clone_and_copy();
                let ident_op_prms_struct_ts = macros_helpers::derive_ts_builder::DTsBuilder::new()
                    .make_pub()
                    .d_debug()
                    .d_clone_if(derive_clone)
                    .d_copy_if(derive_copy)
                    .build_struct(&proc_macro2::TokenStream::new(),&gen_ident_op_prms_ucc(op), &proc_macro2::TokenStream::new(), &{
                        let ident_op_payload_ucc = gen_ident_op_payload_ucc(op);
                        quote::quote! {{
                            pub #PayloadSc: #ident_op_payload_ucc,
                        }}
                    });
                quote::quote! {
                    #AllowClippyArbitrarySrcItemOrdering
                    #ident_op_prms_struct_ts
                }
            };
            let op_ts = {
                let ident_op_res_vrts_ucc = gen_ident_op_res_vrts_ucc(op);
                let ident_try_op_logic_res_vrts_ts = {
                    let ident_op_res_vrts_enum_ts = macros_helpers::derive_ts_builder::DTsBuilder::new()
                        .make_pub()
                        .d_debug()
                        .d_serde_serialize()
                        .d_serde_deserialize()
                        .d_utoipa_to_schema()
                        .build_enum(&proc_macro2::TokenStream::new(), &ident_op_res_vrts_ucc, &proc_macro2::TokenStream::new(), &{
                            let vrts_ts = type_vrts_from_req_res_syn_vrts
                                .iter()
                                .copied()
                                .map(gen_serde_version_of_named_gen_pg_tbl_vrt_ts);
                            let desirable_type_ts = gen_op_result_type_ts(op);
                            quote::quote! {{
                                #DesirableUcc(#desirable_type_ts),
                                #(#vrts_ts),*
                            }}
                        });
                    quote::quote! {
                        #AllowClippyArbitrarySrcItemOrdering
                        #ident_op_res_vrts_enum_ts
                    }
                };
                let ident_op_er_ucc = gen_ident_op_er_ucc(op);
                let impl_ident_op_res_vrts_ts = {
                    let from_h_ts = gen_from_h_ts(&ident_op_er_ucc, &{
                        let vrts_ts = type_vrts_from_req_res_syn_vrts.iter().map(|el| {
                            let vrt_ident = el.ident();
                            let fields_mapped_into_ts = match *el {
                                GenPgTblVariantRef::Syn(syn_vrt) => {
                                    let syn::Fields::Named(fields_named) = &syn_vrt.fields else {
                                        return compile_error_ts(CompileErrorMsg(
                                            "10764d2b: expected named variant fields",
                                        )).into();
                                    };
                                    let fields_ts = fields_named.named.iter().map(|field| &field.ident);
                                    quote::quote! {#(#fields_ts),*}
                                }
                                GenPgTblVariantRef::Model(model_vrt) => {
                                    let fields_ts = model_vrt.fields.iter().map(|field| &field.ident);
                                    quote::quote! {#(#fields_ts),*}
                                }
                            };
                            let ident_op_er_with_serde_ucc =
                                gen_ident_op_er_with_serde_ucc(op);
                            quote::quote! {
                                #ident_op_er_with_serde_ucc::#vrt_ident {
                                    #fields_mapped_into_ts
                                } => Self::#vrt_ident {
                                    #fields_mapped_into_ts
                                }
                            }
                        });
                        quote::quote! {
                            match #VSc.#IntoSerdeVersionSc() {
                                #(#vrts_ts),*
                            }
                        }
                    });
                    quote::quote! {
                        impl #ident_op_res_vrts_ucc {
                            #from_h_ts
                        }
                    }
                };
                let ident_op_er_ts = {
                    let ident_op_er_enum_ts = pg_crud_macros_cmn::ts_helpers::er_enum_d_ts_builder()
                        .build_enum(&proc_macro2::TokenStream::new(), &ident_op_er_ucc, &proc_macro2::TokenStream::new(), &{
                            let vrts_ts = type_vrts_from_req_res_syn_vrts
                                .iter()
                                .copied()
                                .map(gen_loc_vrt_ts);
                            quote::quote! {{#(#vrts_ts),*}}
                        });
                    quote::quote! {
                        #AllowClippyArbitrarySrcItemOrdering
                        #ident_op_er_enum_ts
                    }
                };
                quote::quote! {
                    #ident_try_op_logic_res_vrts_ts
                    #impl_ident_op_res_vrts_ts
                    #ident_op_er_ts
                }
            };
            let try_op_ts = {
                let enum_ts = pg_crud_macros_cmn::ts_helpers::er_enum_d_ts_builder()
                        .build_enum(&proc_macro2::TokenStream::new(), &gen_ident_try_op_er_ucc(op), &proc_macro2::TokenStream::new(), &{
                        let mut syn_vrts = Vec::with_capacity(cmn_http_req_syn_vrts.len().saturating_add(1usize));
                        syn_vrts.extend_from_slice(cmn_http_req_syn_vrts.as_slice());
                        if let Op::Rm | Op::Ro = &op {
                            syn_vrts.push(GenPgTblVariantRef::Syn(not_unq_field_syn_vrt.get_syn_vrt()));
                        }
                        let ident_op_er_with_serde_ucc =
                            gen_ident_op_er_with_serde_ucc(op);
                        let op_er_with_serde_syn_vrt = new_syn_vrt(
                            &ident_op_er_with_serde_ucc,
                            None,
                            vec![(
                                macros_helpers_loc_field_attr_eo_to_err_string,
                                &op.op_er_with_serde_sc(),
                                macros_helpers::gen_simple_syn_punct::gen_simple_syn_punct([
                                    &ident_op_er_with_serde_ucc.to_string()
                                ]),
                            )],
                            false,
                        );
                        let vrts_ts = syn_vrts
                            .iter()
                            .copied()
                            .chain(std::iter::once(GenPgTblVariantRef::Syn(op_er_with_serde_syn_vrt.get_syn_vrt())))
                            .map(gen_loc_vrt_ts);
                        quote::quote! {{#(#vrts_ts),*}}
                    });
                quote::quote! {
                    #AllowClippyArbitrarySrcItemOrdering
                    #enum_ts
                }
            };
            quote::quote! {
                #payload_ts
                #prms_ts
                #op_ts
                #try_op_ts
                #open_api_path_fn_ts
            }
        });
    });
    let ident_api_endpoint_ucc = quote::format_ident!("{}ApiEndpoint", ident);
    let ident_api_client_ucc = quote::format_ident!("{}ApiClient", ident);
    let ident_api_client_ts = quote::quote! {
        #[derive(Clone, Debug)]
        pub struct #ident_api_endpoint_ucc(reqwest::Url);
        impl #ident_api_endpoint_ucc {
            #[must_use]
            pub const fn as_url(&self) -> &reqwest::Url {
                &self.0
            }
        }
        impl From<reqwest::Url> for #ident_api_endpoint_ucc {
            fn from(value: reqwest::Url) -> Self {
                Self(value)
            }
        }
        #[derive(Clone, Debug)]
        pub struct #ident_api_client_ucc {
            client: reqwest::Client,
            endpoint: #ident_api_endpoint_ucc,
        }
        impl #ident_api_client_ucc {
            #[must_use]
            pub const fn new(client: reqwest::Client, endpoint: #ident_api_endpoint_ucc) -> Self {
                Self { client, endpoint }
            }
            #(#api_client_methods_ts)*
        }
    };
    let ident_auth_requirement_ucc = quote::format_ident!("{}AuthenticationRequirement", ident);
    let ident_http_method_ucc = quote::format_ident!("{}HttpMethod", ident);
    let ident_operation_ucc = quote::format_ident!("{}Operation", ident);
    let ident_route_contract_ucc = quote::format_ident!("{}RouteContract", ident);
    let ident_success_status_ucc = quote::format_ident!("{}SuccessStatus", ident);
    let route_contract_items_ts = OpDsc::ALL.iter().map(|op_dsc| {
        let operation = quote::format_ident!("{}", op_dsc.op.to_string());
        let http_method = match op_dsc.http_method {
            OpHttpMethod::Post => quote::format_ident!("Post"),
            OpHttpMethod::Patch => quote::format_ident!("Patch"),
            OpHttpMethod::Delete => quote::format_ident!("Delete"),
        };
        let success_status =
            if op_dsc.success_status_code == macros_helpers::status_code::StatusCode::Crd201 {
                quote::format_ident!("Code201")
            } else {
                quote::format_ident!("Code200")
            };
        quote::quote! {
            #ident_route_contract_ucc::new(
                #ident_auth_requirement_ucc::Public,
                #ident_http_method_ucc::#http_method,
                #ident_operation_ucc::#operation,
                #ident_success_status_ucc::#success_status,
            )
        }
    });
    let ident_route_contract_ts = quote::quote! {
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        pub enum #ident_auth_requirement_ucc {
            Public,
        }
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        pub enum #ident_http_method_ucc {
            Delete,
            Patch,
            Post,
        }
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        pub enum #ident_operation_ucc {
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
        pub enum #ident_success_status_ucc {
            Code200,
            Code201,
        }
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        pub struct #ident_route_contract_ucc {
            authentication: #ident_auth_requirement_ucc,
            http_method: #ident_http_method_ucc,
            operation: #ident_operation_ucc,
            success_status: #ident_success_status_ucc,
        }
        impl #ident_route_contract_ucc {
            pub const ALL: [Self; 8] = [#(#route_contract_items_ts),*];
            #[must_use]
            pub const fn authentication(self) -> #ident_auth_requirement_ucc {
                self.authentication
            }
            #[must_use]
            pub const fn http_method(self) -> #ident_http_method_ucc {
                self.http_method
            }
            #[must_use]
            pub const fn new(authentication: #ident_auth_requirement_ucc, http_method: #ident_http_method_ucc, operation: #ident_operation_ucc, success_status: #ident_success_status_ucc) -> Self {
                Self { authentication, http_method, operation, success_status }
            }
            #[must_use]
            pub const fn operation(self) -> #ident_operation_ucc {
                self.operation
            }
            #[must_use]
            pub const fn success_status(self) -> #ident_success_status_ucc {
                self.success_status
            }
        }
    };
    let ident_open_api_ucc = quote::format_ident!("{}OpenApi", ident);
    let gen_role_schema_items_ts = |role: &dyn quote::ToTokens| {
        fields
            .iter()
            .map(|field| {
                let role_type_ts = gen_as_pg_type_tokens_ts(&field.type0, role);
                quote::quote! {<#role_type_ts as utoipa::ToSchema>::schema().1}
            })
            .collect::<Vec<_>>()
    };
    let rd_schema_items_ts = gen_role_schema_items_ts(&RdUcc);
    let sel_schema_items_ts = gen_role_schema_items_ts(&SelUcc);
    let gen_filter_schema_items_ts = |filter_ucc: &dyn quote::ToTokens| {
        fields
            .iter()
            .map(|field| {
                let tt_type_ts = gen_as_pg_type_tokens_ts(&field.type0, &naming::TtUcc);
                quote::quote! {<wh_flts::#filter_ucc<#tt_type_ts> as utoipa::ToSchema>::schema().1}
            })
            .collect::<Vec<_>>()
    };
    let gen_ordered_filter_schema_items_ts = |filter_ucc: &dyn quote::ToTokens| {
        fields
            .iter()
            .map(|field| {
                let tt_type_ts =
                    gen_concrete_stdrt_nn_pg_type_role_ts(&field.type0, &naming::TtUcc);
                quote::quote! {<wh_flts::#filter_ucc<#tt_type_ts> as utoipa::ToSchema>::schema().1}
            })
            .collect::<Vec<_>>()
    };
    let eq_filter_schema_items_ts = gen_filter_schema_items_ts(&quote::format_ident!("PgTypeWhEq"));
    let btwn_filter_schema_items_ts =
        gen_ordered_filter_schema_items_ts(&quote::format_ident!("PgTypeWhBtwn"));
    let greater_than_filter_schema_items_ts =
        gen_ordered_filter_schema_items_ts(&quote::format_ident!("PgTypeWhGreaterThan"));
    let in_filter_schema_items_ts = gen_filter_schema_items_ts(&quote::format_ident!("PgTypeWhIn"));
    let in_value_schema_items_ts = fields.iter().map(|field| {
        let tt_type_ts = gen_as_pg_type_tokens_ts(&field.type0, &naming::TtUcc);
        quote::quote! {<wh_flts::PgTypeNotEmptyUnqVec<#tt_type_ts> as utoipa::ToSchema>::schema().1}
    }).collect::<Vec<_>>();
    let ident_open_api_ts = quote::quote! {
        #[allow(clippy::needless_for_each)] // generated utoipa 4 registration uses iterator callbacks internally
        #[derive(utoipa::OpenApi)]
        #[openapi(
            paths(#(#open_api_path_fn_idents),*),
            components(schemas(#(#open_api_schema_types_ts),*)),
            tags((name = #ident_sc_string, description = "Generated CRUD API"))
        )]
        pub struct #ident_open_api_ucc;
        #[allow(clippy::needless_for_each)] // recursive schema-reference normalization is clearer as iterator traversal
        impl #ident_open_api_ucc {
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
                if let Some(components) = open_api.components.as_mut() {
                    components.schemas.insert("pg_crud_cmn.PgType.Rd".to_owned(), utoipa::openapi::schema::Schema::from(utoipa::openapi::OneOfBuilder::new()#(.item(#rd_schema_items_ts))*.build()).into());
                    components.schemas.insert("pg_crud_cmn.PgType.Sel".to_owned(), utoipa::openapi::schema::Schema::from(utoipa::openapi::OneOfBuilder::new()#(.item(#sel_schema_items_ts))*.build()).into());
                    components.schemas.insert("wh_flts.PgTypeWhEq".to_owned(), utoipa::openapi::schema::Schema::from(utoipa::openapi::OneOfBuilder::new()#(.item(#eq_filter_schema_items_ts))*.build()).into());
                    components.schemas.insert("wh_flts.PgTypeWhBtwn".to_owned(), utoipa::openapi::schema::Schema::from(utoipa::openapi::OneOfBuilder::new()#(.item(#btwn_filter_schema_items_ts))*.build()).into());
                    components.schemas.insert("wh_flts.PgTypeWhGreaterThan".to_owned(), utoipa::openapi::schema::Schema::from(utoipa::openapi::OneOfBuilder::new()#(.item(#greater_than_filter_schema_items_ts))*.build()).into());
                    components.schemas.insert("wh_flts.PgTypeWhIn".to_owned(), utoipa::openapi::schema::Schema::from(utoipa::openapi::OneOfBuilder::new()#(.item(#in_filter_schema_items_ts))*.build()).into());
                    components.schemas.insert("PgTypeNotEmptyUnqVec".to_owned(), utoipa::openapi::schema::Schema::from(utoipa::openapi::OneOfBuilder::new()#(.item(#in_value_schema_items_ts))*.build()).into());
                }
                let mut refs = std::collections::BTreeSet::new();
                if let Ok(value) = serde_json::to_value(&open_api) {
                    collect_refs(&value, &mut refs);
                }
                if let Some(components) = open_api.components.as_mut() {
                    refs.into_iter().for_each(|name| {
                        if !components.schemas.contains_key(&name) {
                            let suffix = name.rsplit('.').next().unwrap_or(name.as_str());
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
    let generated_contract_tests_ts = {
        let round_trip_tests_ts = OpDsc::ALL.iter().map(|op_dsc| {
            let op = &op_dsc.op;
            let payload_type_ts = gen_ident_op_payload_ucc(op);
            let test_ident = quote::format_ident!(
                "{}_{}_payload_json_round_trip",
                ident_sc_string,
                op.self_sc_str()
            );
            let normalize_default_filter_ts = if matches!(op, Op::Rm | Op::Dm) {
                quote::quote! {
                    serialized.as_object_mut().expect("58c97ca7").insert(
                        "wh_many".to_owned(),
                        serde_json::Value::Null,
                    );
                }
            } else {
                proc_macro2::TokenStream::new()
            };
            quote::quote! {
                #[test]
                fn #test_ident() {
                    let original: #payload_type_ts = pg_crud_cmn::DfltSomeOneEl::dflt_some_one_el();
                    let mut serialized = serde_json::to_value(&original).expect("84094d13");
                    #normalize_default_filter_ts
                    let deserialized = serde_json::from_value::<#payload_type_ts>(serialized.clone()).expect("b388de0c");
                    let round_trip = serde_json::to_value(deserialized).expect("570ac825");
                    assert_eq!(round_trip, serialized);
                }
            }
        });
        let unknown_field_tests_ts = [Op::Rm, Op::Ro, Op::Dm, Op::Dlo].into_iter().map(|op| {
            let payload_type_ts = gen_ident_op_payload_ucc(&op);
            let test_ident = quote::format_ident!(
                "{}_{}_payload_rejects_unknown_field",
                ident_sc_string,
                op.self_sc_str()
            );
            quote::quote! {
                #[test]
                fn #test_ident() {
                    let original: #payload_type_ts = pg_crud_cmn::DfltSomeOneEl::dflt_some_one_el();
                    let mut serialized = serde_json::to_value(original).expect("aeedc9e8");
                    serialized.as_object_mut().expect("b9d4b58e").insert(
                        "unknown_field".to_owned(),
                        serde_json::Value::Bool(true),
                    );
                    assert!(serde_json::from_value::<#payload_type_ts>(serialized).is_err());
                }
            }
        });
        let contract_tests_mod_ident = quote::format_ident!("{}_contract_tests", ident_sc_string);
        let api_client_owns_reusable_client_test_ident =
            quote::format_ident!("{}_api_client_owns_reusable_client", ident_sc_string);
        let read_query_negative_contracts_test_ident =
            quote::format_ident!("{}_read_query_negative_contracts", ident_sc_string);
        let route_open_api_parity_test_ident =
            quote::format_ident!("{}_route_open_api_parity", ident_sc_string);
        let ident_rm_payload_ucc = gen_ident_op_payload_ucc(&Op::Rm);
        let route_open_api_parity_assertions_ts = OpDsc::ALL.iter().map(|op_dsc| {
            let path = format!("/{ident_sc_string}/{}", op_dsc.op.self_sc_str());
            let method = match op_dsc.http_method {
                OpHttpMethod::Post => "post",
                OpHttpMethod::Patch => "patch",
                OpHttpMethod::Delete => "delete",
            };
            quote::quote! {
                assert!(document.pointer(&format!("/paths/{}/{}", #path.replace('/', "~1"), #method)).is_some());
            }
        });
        let bulk_limit_tests_ts = [
            (Op::Cm, gen_pg_tbl_input_model.config.cm_max_items),
            (Op::Um, gen_pg_tbl_input_model.config.um_max_items),
        ]
        .into_iter()
        .filter_map(|(op, optional_limit)| {
            let configured_limit = optional_limit?;
            let limit_value = configured_limit.0;
            let payload_type_ts = gen_ident_op_payload_ucc(&op);
            let test_ident = quote::format_ident!(
                "{}_{}_payload_enforces_item_limit",
                ident_sc_string,
                op.self_sc_str()
            );
            Some(quote::quote! {
                #[test]
                fn #test_ident() {
                    let original: #payload_type_ts = pg_crud_cmn::DfltSomeOneEl::dflt_some_one_el();
                    let original_value = serde_json::to_value(original).expect("d4d4cc0d");
                    let item = original_value.as_array().and_then(|items| items.first()).cloned().expect("79b00707");
                    assert!(serde_json::from_value::<#payload_type_ts>(original_value).is_ok());
                    let above_limit = serde_json::Value::Array(std::iter::repeat_n(item, #limit_value.saturating_add(1usize)).collect());
                    match serde_json::from_value::<#payload_type_ts>(above_limit) {
                        Ok(_) => panic!("1a74209c"),
                        Err(error) => assert!(error.to_string().contains("exceeds limit")),
                    }
                }
            })
        });
        quote::quote! {
            #[cfg(test)]
            mod #contract_tests_mod_ident {
                use super::*;
                #[test]
                fn #api_client_owns_reusable_client_test_ident() {
                    let url = reqwest::Url::parse("http://127.0.0.1:3000/").expect("ca76d3e6");
                    let endpoint = #ident_api_endpoint_ucc::from(url.clone());
                    assert_eq!(endpoint.as_url(), &url);
                    let client = #ident_api_client_ucc::new(reqwest::Client::new(), endpoint);
                    assert!(format!("{client:?}").contains(stringify!(#ident_api_client_ucc)));
                }
                #[test]
                fn #read_query_negative_contracts_test_ident() {
                    let original: #ident_rm_payload_ucc = pg_crud_cmn::DfltSomeOneEl::dflt_some_one_el();
                    let serialized = serde_json::to_value(original).expect("bbb88adf");
                    let mut empty_filter_payload = serialized.clone();
                    empty_filter_payload.as_object_mut().expect("aa1919f0").insert("wh_many".to_owned(), serde_json::json!({}));
                    assert!(serde_json::from_value::<#ident_rm_payload_ucc>(empty_filter_payload).is_err());
                    let mut unknown_filter_payload = serialized.clone();
                    unknown_filter_payload.as_object_mut().expect("42671a58").insert("wh_many".to_owned(), serde_json::json!({"unknown_field": null}));
                    assert!(serde_json::from_value::<#ident_rm_payload_ucc>(unknown_filter_payload).is_err());
                    let wh_many = serialized.get("wh_many").and_then(serde_json::Value::as_object).expect("e0b089c7");
                    let (field_name, field_filter) = wh_many.iter().next().expect("5d781d42");
                    let filters = field_filter.get("v").and_then(serde_json::Value::as_array).expect("2ca9da9a");
                    let mut multi_operator = filters.first().and_then(serde_json::Value::as_object).cloned().expect("3a86c2c9");
                    let (second_operator_name, second_operator_value) = filters.get(1usize).and_then(serde_json::Value::as_object).and_then(|value| value.iter().next()).expect("8589f0ef");
                    multi_operator.insert(second_operator_name.clone(), second_operator_value.clone());
                    let mut multi_operator_field_filter = field_filter.clone();
                    let multi_operator_filters = multi_operator_field_filter.as_object_mut().and_then(|value| value.get_mut("v")).and_then(serde_json::Value::as_array_mut).expect("5df08753");
                    multi_operator_filters.clear();
                    multi_operator_filters.push(serde_json::Value::Object(multi_operator));
                    let mut multi_operator_payload = serialized.clone();
                    let mut multi_operator_wh_many = serde_json::Map::new();
                    multi_operator_wh_many.insert(field_name.clone(), multi_operator_field_filter);
                    multi_operator_payload.as_object_mut().expect("c92118fe").insert("wh_many".to_owned(), serde_json::Value::Object(multi_operator_wh_many));
                    assert!(serde_json::from_value::<#ident_rm_payload_ucc>(multi_operator_payload).is_err());
                    let duplicate_filter_json = format!("{{\"{field_name}\":{field_filter},\"{field_name}\":{field_filter}}}");
                    assert!(serde_json::from_str::<#ident_wh_ucc>(&duplicate_filter_json).is_err());
                    let mut cursor_payload = serialized;
                    cursor_payload.as_object_mut().expect("c12f9360").insert("cursor".to_owned(), serde_json::Value::String("forbidden".to_owned()));
                    assert!(serde_json::from_value::<#ident_rm_payload_ucc>(cursor_payload).is_err());
                }
                #[test]
                fn #route_open_api_parity_test_ident() {
                    let document = serde_json::to_value(#ident_open_api_ucc::open_api()).expect("eb512de9");
                    assert_eq!(#ident_route_contract_ucc::ALL.len(), 8usize);
                    assert!(#ident_route_contract_ucc::ALL.into_iter().all(|contract| contract.authentication() == #ident_auth_requirement_ucc::Public));
                    #(#route_open_api_parity_assertions_ts)*
                }
                #(#round_trip_tests_ts)*
                #(#unknown_field_tests_ts)*
                #(#bulk_limit_tests_ts)*
            }
        }
    };
    impl_ident_vec_ts.push(quote::quote! {
        pub fn #RoutesSc(#AppStateSc: #std_sync_arc_combination_of_app_state_logic_traits_ts) -> axum::Router {
            Self::#RoutesHSc(#AppStateSc, #self_tbl_name_call_ts)
        }
    });
    let (oprtr_or_ts, oprtr_and_ts) = {
        let oprtr_ts = quote::quote! {#import_ts Oprtr::};
        (quote::quote! {#oprtr_ts Or}, quote::quote! {#oprtr_ts And})
    };
    let generated_ident_tests_ts = {
        fn gen_assert_ts(
            ts0: &dyn quote::ToTokens,
            ts1: &dyn quote::ToTokens,
        ) -> proc_macro2::TokenStream {
            quote::quote! {assert!(#ts0,#ts1);}
        }
        fn gen_assert_eq_ts(
            ts0: &dyn quote::ToTokens,
            ts1: &dyn quote::ToTokens,
            ts2: &dyn quote::ToTokens,
        ) -> proc_macro2::TokenStream {
            quote::quote! {assert_eq!(#ts0,#ts1,#ts2);}
        }
        let gen_pk_wh_eq_ts = |ts0: &dyn quote::ToTokens| {
            quote::quote! {
                #pk_ft_as_pg_type_wh_ts::Eq(
                    #import_ts PgTypeWhEq {
                        oprtr: #oprtr_or_ts,
                        #VSc: #ts0,
                    },
                )
            }
        };
        let gen_pk_wh_eq_new_ts =
            |ts0: &dyn quote::ToTokens| gen_pk_wh_eq_ts(&quote::quote! {#pk_ft_tt_ts::new(#ts0)});
        let pk_wh_eq_uuid_new_v_ts = gen_pk_wh_eq_new_ts(&quote::quote! {uuid::Uuid::new_v4()});
        let gen_pk_wh_eq_into_inn_ts = |ts0: &dyn quote::ToTokens| {
            gen_pk_wh_eq_new_ts(&quote::quote! {#pk_as_pg_type_ts::into_inn(#ts0)})
        };
        let ident_tests_sc = naming::prm::SelfTestsSc::from_display(&ident);
        let ident_dq_ts = gen_quotes::dq_ts(&naming_cmn::DisplayToScStr::case(&ident));
        let ident_cm_prms_ucc = gen_ident_op_prms_ucc(&Op::Cm);
        let ident_rm_prms_ucc = gen_ident_op_prms_ucc(&Op::Rm);
        let ident_cm_payload_ucc = gen_ident_op_payload_ucc(&Op::Cm);
        let ident_rm_payload_ucc = gen_ident_op_payload_ucc(&Op::Rm);
        let ident_co_prms_ucc = gen_ident_op_prms_ucc(&Op::Co);
        let ident_ro_prms_ucc = gen_ident_op_prms_ucc(&Op::Ro);
        let ident_ro_payload_ucc = gen_ident_op_payload_ucc(&Op::Ro);
        let ident_uo_prms_ucc = gen_ident_op_prms_ucc(&Op::Uo);
        let config_path_ts = quote::quote! {server_config::Config};
        let undrscr_unused_ts = quote::quote! {_unused};
        //todo mb remove it?\
        let gen_some_pg_type_wh_try_new_ts =
            |oprtr_ts: &dyn quote::ToTokens, ts: &dyn quote::ToTokens| {
                quote::quote! {
                    Some(
                        #import_ts PgTypeWh::try_new(
                            #oprtr_ts,
                            #ts
                        ).expect("6b0491b2"),
                    )
                }
            };
        let gen_some_pg_type_wh_try_new_and_ts =
            |ts: &dyn quote::ToTokens| gen_some_pg_type_wh_try_new_ts(&oprtr_and_ts, ts);
        let gen_pg_type_wh_try_new_pk_ts = quote::quote! {
            #import_ts PgTypeWh::try_new(
                oprtr,
                vec
            ).expect("fd20ad6d")
        };
        let ident_cr_dflt_fields_init_without_pk_ts = gen_fields_named_without_pk_with_comma_ts(
            &|el: &macros_helpers::field_data::SynField| {
                let fi = &el.ident;
                let ft_as_pg_type_cr_ts = gen_as_pg_type_cr_ts(&el.type0);
                quote::quote! {
                    #fi: <#ft_as_pg_type_cr_ts as #import_ts DfltSomeOneEl>::dflt_some_one_el()
                }
            },
        );
        let fields_none_init_ts = gen_fields_named_without_pk_with_comma_ts(
            &|el: &macros_helpers::field_data::SynField| {
                let fi = &el.ident;
                quote::quote! {#fi: None}
            },
        );
        //todo instead of first dropping tbl - check if its not exists. if exists Test must fail
        let sel_dflt_all_with_max_page_size_not_empty_unq_vec_ts = {
            let ts = gen_fields_named_with_comma_ts(
                &|el: &macros_helpers::field_data::SynField| {
                    let fi = &el.ident;
                    let ft = &el.type0;
                    let fi_ucc = naming_cmn::ToTokensToUccTs::case_or_panic(&fi);
                    quote::quote! {
                        #ident_sel_ucc::#fi_ucc(
                            <<#ft as #import_ts PgType>::Sel as #import_ts #DfltSomeOneElMaxPageSizeUcc>::#DfltSomeOneElMaxPageSizeSc()
                        )
                    }
                },
            );
            quote::quote! {
                let sel_dflt_all_with_max_page_size = #import_ts NotEmptyUnqVec::try_new_by_hash(vec![
                    #ts
                ]).expect("5e82ac66");
            }
        };
        let pk_ft_as_pg_type_pk_ts = quote::quote! {<#pk_ft as #import_ts PgTypePk>::};
        let gen_pk_ft_as_pg_type_pk_method_call_ts =
            |method_ts: &dyn quote::ToTokens, ts0: &dyn quote::ToTokens| {
                quote::quote! {#pk_ft_as_pg_type_pk_ts #method_ts(#ts0)}
            };
        let rd_ids_el_pk_fi_ts = quote::quote! {rd_ids_el_937c5af3.#pk_fi};
        let (
            pk_ft_rd_ids_into_rd_el_43ab7fb5_pk_fi_ts,
            pk_ft_rd_ids_into_rd_rd_ids_from_try_co_pk_fi_ts,
            pk_ft_rd_only_is_into_rd_rd_ids_el_pk_fi_ts,
            pk_ft_rd_ids_into_rd_rd_ids_from_co_pk_fi_ts,
        ) = {
            let gen_ts = |ts: &dyn quote::ToTokens| {
                gen_pk_ft_as_pg_type_pk_method_call_ts(&RdIdsIntoRdSc, &ts)
            };
            (
                gen_ts(&quote::quote! {el_43ab7fb5.#pk_fi}),
                gen_ts(&quote::quote! {rd_ids_from_try_co.#pk_fi}),
                gen_ts(&rd_ids_el_pk_fi_ts),
                gen_ts(&quote::quote! {rd_ids_from_co.#pk_fi}),
            )
        };
        let pk_wh_eq_iter_map_ts = {
            let ts = gen_pk_wh_eq_into_inn_ts(&pk_ft_rd_ids_into_rd_el_43ab7fb5_pk_fi_ts);
            quote::quote! {.iter().map(|el_43ab7fb5| #ts)}
        };
        let pk_ft_as_pg_type_upd_as_pg_type_pk_rd_ids_into_upd_ts = {
            let method_call_ts =
                gen_pk_ft_as_pg_type_pk_method_call_ts(&RdIdsIntoUpdSc, &rd_ids_el_pk_fi_ts);
            quote::quote! {#pk_as_pg_type_ts::Upd::from(#method_call_ts)}
        };
        let (
            fi_rd_ids_and_cr_into_opt_v_rd_rd_ids_and_cr_ts,
            fi_rd_ids_and_cr_into_opt_v_rd_rd_ids_from_try_co_ident_cr_ts,
            fi_rd_ids_and_cr_into_opt_v_rd_rd_ids_from_co_cr_ts,
            fi_rd_ids_and_cr_into_opt_v_rd_rd_ids_from_co_clone_ident_cr_clone_ts,
        ) = {
            enum AddDotClone {
                False,
                True,
            }
            let gen_ts = |rd_ids_ts: &dyn quote::ToTokens,
                          cr_ts: &dyn quote::ToTokens,
                          add_dot_clone: &AddDotClone| {
                gen_fields_named_without_pk_with_comma_ts(
                    &|el: &macros_helpers::field_data::SynField| {
                        let fi = &el.ident;
                        let mb_dot_clone_ts = match &add_dot_clone {
                            AddDotClone::False => proc_macro2::TokenStream::new(),
                            AddDotClone::True => quote::quote! {.clone()},
                        };
                        let ft_ts = gen_as_pg_type_test_cases_path_ts(&el.type0);
                        quote::quote! {
                            #fi: #ft_ts rd_ids_and_cr_into_opt_v_rd(
                                #rd_ids_ts.#fi #mb_dot_clone_ts.expect("f967434c"),
                                #cr_ts.#fi #mb_dot_clone_ts
                            )
                        }
                    },
                )
            };
            let ident_cr_name_ts = quote::quote! {ident_cr};
            let rd_ids_from_co_name_ts = quote::quote! {rd_ids_from_co};
            (
                gen_ts(&RdIdsSc, &CrSc, &AddDotClone::False),
                gen_ts(
                    &quote::quote! {rd_ids_from_try_co},
                    &ident_cr_name_ts,
                    &AddDotClone::False,
                ),
                gen_ts(
                    &rd_ids_from_co_name_ts,
                    &quote::quote! {ident_cr_dflt},
                    &AddDotClone::False,
                ),
                gen_ts(
                    &rd_ids_from_co_name_ts,
                    &ident_cr_name_ts,
                    &AddDotClone::True,
                ),
            )
        };
        let opt_ident_wh_fields_none_ts = gen_fields_named_without_pk_with_comma_ts(
            &|el: &macros_helpers::field_data::SynField| {
                let fi = &el.ident;
                quote::quote! {#fi: None}
            },
        );
        let sel_dflt_all_with_max_page_size_clone_ts =
            quote::quote! {sel_dflt_all_with_max_page_size.clone()};
        let cmn_rd_ids_from_co_ts = {
            let pk_rd_ts = quote::quote! {pk_rd};
            let pk_rd_clone_ts = quote::quote! {pk_rd.clone()};
            let ts = gen_v_init_ts0(&pk_rd_clone_ts);
            let assert_eq_ro_pk_ts = gen_assert_eq_ts(
                &quote::quote! {
                    #ident_rd_ucc {
                        #pk_fi: Some(#ts),
                        #fields_none_init_ts
                    }
                },
                &quote::quote! {
                    gen_ident_try_ro_h_pk(
                        &#UrlSc,
                        #pk_rd_clone_ts,
                        #SelPkSc.clone(),
                        &tbl_init
                    )
                    .await
                    .expect("36b95e96")
                },
                &quote::quote! {"3d9f2ec0"},
            );
            let assert_eq_dlo_pk_ts = gen_assert_eq_ts(
                &quote::quote! {
                    gen_try_dlo_h(
                        &url,
                        #pk_rd_clone_ts,
                        &tbl_init,
                    ).await.expect("4d96d385")
                },
                &quote::quote! {#pk_rd_clone_ts},
                &quote::quote! {"26e2058b"},
            );
            quote::quote! {
                let #CmnRdIdsFromCoSc = {
                    let rd_ids_from_try_co = gen_rd_ids_from_try_co_dflt(&#UrlSc, &tbl_init).await;
                    let pk_rd = #pk_ft_rd_ids_into_rd_rd_ids_from_try_co_pk_fi_ts;
                    #assert_eq_ro_pk_ts
                    #assert_eq_dlo_pk_ts
                    gen_check_no_rows_from_ident_try_ro_h_pk(
                        &url,
                        #pk_rd_ts,
                        #sel_dflt_all_with_max_page_size_clone_ts,
                        &tbl_init,
                    ).await;
                    rd_ids_from_try_co
                };
            }
        };
        let gen_ident_cr_ts: &dyn Fn(
            &syn::Ident,
            &dyn quote::ToTokens,
        ) -> proc_macro2::TokenStream = &|fi, ts| {
            gen_fields_named_without_pk_with_comma_ts(
                &|el: &macros_helpers::field_data::SynField| {
                    let fi0 = &el.ident;
                    let ft0 = &el.type0;
                    let ts0 = if fi == fi0.as_ref() {
                        quote::quote! {#ts}
                    } else {
                        let ts1 = gen_as_pg_type_path_ts(&ft0);
                        quote::quote! {<#ts1 Cr as #import_ts DfltSomeOneEl>::dflt_some_one_el()}
                    };
                    quote::quote! {#fi0: #ts0}
                },
            )
        };
        let gen_ident_cr_cnt_el_id_ts: &dyn Fn(
            &syn::Ident,
            &dyn quote::ToTokens,
        ) -> proc_macro2::TokenStream = &|fi, el_ts| gen_ident_cr_ts(fi, &el_ts);
        let gen_ident_cr_cnt_el_ts: &dyn Fn(&syn::Ident) -> proc_macro2::TokenStream =
            &|fi| gen_ident_cr_ts(fi, &ElSc);
        let gen_tbl_test_name_fi_ts: &dyn Fn(&str, &syn::Ident) -> proc_macro2::TokenStream =
            &|test_name, fi| {
                let tbl_test_name_fi = quote::format_ident!("tbl_{test_name}_{fi}");
                quote::quote! {#tbl_test_name_fi}
            };
        let tbl_fields_test_name_count = 4usize;
        let mut tbl_fis_init_vec_ts =
            Vec::with_capacity(fields_len_without_pk.saturating_mul(tbl_fields_test_name_count));
        let mut tbl_test_name_fis_vec_ts =
            Vec::with_capacity(fields_len_without_pk.saturating_mul(tbl_fields_test_name_count));
        let fill_tbl_fis_vec_ts: &mut dyn FnMut(Vec<&str>) = &mut |test_names| {
            test_names.into_iter().fold((), |(), el0| {
                let gen_init_variable_name_ts: &dyn Fn(&syn::Ident) -> proc_macro2::TokenStream =
                    &|fi| {
                        let init_variable_name = quote::format_ident!("tbl_{el0}_{fi}");
                        quote::quote! {#init_variable_name}
                    };
                tbl_fis_init_vec_ts.push(gen_fields_named_without_pk_without_comma_ts(
                    &|el: &macros_helpers::field_data::SynField| {
                        let fi = &el.ident;
                        let init_variable_name_ts = gen_init_variable_name_ts(fi);
                        let format_ts = gen_quotes::dq_ts(&format!("{el0}_{fi}"));
                        quote::quote! {
                            let #init_variable_name_ts = add_tbl_postfix(#format_ts);
                        }
                    },
                ));
                tbl_test_name_fis_vec_ts.push(gen_fields_named_without_pk_without_comma_ts(
                    &|el1: &macros_helpers::field_data::SynField| {
                        let fi = &el1.ident;
                        let init_variable_name_ts = gen_init_variable_name_ts(fi);
                        quote::quote! {&#init_variable_name_ts,}
                    },
                ));
            });
        };
        let tbl_rd_ids_and_cr_into_wh_eq_name = "8e427ad7";
        let tbl_rd_ids_and_cr_into_vec_wh_eq_using_fields_name = "eb24448c";
        let tbl_rd_ids_and_cr_into_opt_vec_wh_eq_to_field_name = "9ac6d79a";
        let tbl_rd_ids_and_tt_into_pg_type_opt_wh_greater_than_name = "5a52af33";
        fill_tbl_fis_vec_ts(vec![
            &tbl_rd_ids_and_cr_into_wh_eq_name,
            &tbl_rd_ids_and_cr_into_vec_wh_eq_using_fields_name,
            &tbl_rd_ids_and_cr_into_opt_vec_wh_eq_to_field_name,
            &tbl_rd_ids_and_tt_into_pg_type_opt_wh_greater_than_name,
        ]);
        let sel_dflt_all_with_max_page_size_cloned_clone_ts =
            quote::quote! {sel_dflt_all_with_max_page_size_cloned.clone()};
        let rd_ids_to_2_dims_vec_rd_inn_acc_fields_ts =
            gen_fields_named_without_pk_without_comma_ts(
                &|el: &macros_helpers::field_data::SynField| {
                    let fi = &el.ident;
                    let fi_rd_ids_to_2_dims_vec_rd_inn_acc_sc =
                        naming::prm::SelfRdIdsTo2DimsVecRdInnAccSc::from_tokens(&fi);
                    let ident_cr_dflts_for_col_rd_ids_to_2_dims_vec_rd_inn_ts =
                        gen_fields_named_without_pk_without_comma_ts(
                            &|el0: &macros_helpers::field_data::SynField| {
                                let fi0 = &el0.ident;
                                let ft0 = &el0.type0;
                                if fi == fi0 {
                                    gen_if_let_some_ts(
                                        &quote::quote! {v_a5f7e6cd},
                                        &quote::quote! {&cmn_rd_ids_from_co.#fi0},
                                        &{
                                            let ft_ts = gen_as_pg_type_test_cases_path_ts(&ft0);
                                            quote::quote! {
                                                let rd_ids_to_2_dims_vec_rd_inn_8ef7d00b = #ft_ts rd_ids_to_2_dims_vec_rd_inn(v_a5f7e6cd);
                                                acc_458cda9e.reserve(
                                                    rd_ids_to_2_dims_vec_rd_inn_8ef7d00b
                                                        .iter()
                                                        .map(Vec::len)
                                                        .sum::<usize>()
                                                );
                                                for el_b3522b7d in rd_ids_to_2_dims_vec_rd_inn_8ef7d00b {
                                                    for _ in el_b3522b7d {
                                                        acc_458cda9e.push(ident_cr_dflt.clone());
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
                        let #fi_rd_ids_to_2_dims_vec_rd_inn_acc_sc = {
                            let mut acc_458cda9e = Vec::new();
                            #ident_cr_dflts_for_col_rd_ids_to_2_dims_vec_rd_inn_ts
                            acc_458cda9e
                        };
                    }
                },
            );
        let gen_wh_pk_or_ts = |vec_ts: &dyn quote::ToTokens| {
            quote::quote! {
                gen_ident_wh_pk_others_none(
                    Some(
                        gen_pg_type_wh_try_new_pk(
                            #oprtr_or_ts,
                            #vec_ts
                        )
                    )
                )
            }
        };
        let pk_sort_cmp_ts = quote::quote! {
            |first, second| match (&first.#pk_fi, &second.#pk_fi) {
                (Some(first_h), Some(second_h)) => first_h.#VSc.cmp(&second_h.#VSc),
                _ => panic!("0f1d45ed"),
            }
        };
        let gen_rd_ids_els_ts = {
            let ident_rd_fields_init_without_pk_ts = gen_fields_named_without_pk_with_comma_ts(
                &|syn_field: &macros_helpers::field_data::SynField| {
                    let fi = &syn_field.ident;
                    let ts = gen_v_init_ts0(&PgCrudCmnDfltSomeOneElCall);
                    let ts0 = gen_as_pg_type_test_cases_path_ts(&syn_field.type0);
                    quote::quote! {
                        #fi: el_f108da5a.#fi.as_ref().map_or_else(
                            || Some(#ts),
                            #ts0 rd_ids_to_opt_v_rd_dflt_some_one_el
                        )
                    }
                },
            );
            let wh_pk_or_rd_ids_els_ts =
                gen_wh_pk_or_ts(&quote::quote! {rd_ids_els_efeed554 #pk_wh_eq_iter_map_ts});
            let assert_eq_rd_ids_els_ts = gen_assert_eq_ts(
                &quote::quote! {
                    itertools::Itertools::sorted_by(
                        rd_ids_els_efeed554.iter().map(|el_f108da5a| {
                            #ident_rd_ucc {
                                #pk_fi: #pk_as_pg_type_test_cases_path_ts rd_ids_to_opt_v_rd_dflt_some_one_el(
                                    &el_f108da5a.#pk_fi
                                ),
                                #ident_rd_fields_init_without_pk_ts
                            }
                        }),
                        #pk_sort_cmp_ts
                    ).collect::<Vec<#ident_rd_ucc>>()
                },
                &quote::quote! {
                    itertools::Itertools::sorted_by(
                        gen_try_rm_order_by_pk_with_big_pgn(
                            url,
                            #wh_pk_or_rd_ids_els_ts,
                            #sel_dflt_all_with_max_page_size_clone_ts,
                            tbl_9c259e1c
                        )
                        .await
                        .expect("097d5e7d")
                        .into_iter(),
                        #pk_sort_cmp_ts
                    )
                    .collect::<Vec<#ident_rd_ucc>>()
                },
                &quote::quote! {"50198a7f"},
            );
            quote::quote! {
                async fn gen_rd_ids_els_8a1ef027(
                    url: &str,
                    tbl_9c259e1c: &str,
                    sel_dflt_all_with_max_page_size: #import_ts NotEmptyUnqVec<#ident_sel_ucc>,
                    rd_ids_to_2_dims_vec_rd_inn_acc: Vec<#ident_cr_ucc>
                ) -> Vec<#ident_rd_ids_ucc> {
                    const CM_CHUNK_SIZE_2EE9377B: usize = 25;
                    const CM_CONCURRENCY_7CCFD82D: usize = 5;
                    let rd_ids_to_2_dims_vec_rd_inn_acc_len = rd_ids_to_2_dims_vec_rd_inn_acc.len();
                    let rd_ids_els_efeed554 = futures::StreamExt::fold(
                        futures::StreamExt::buffer_unordered(
                            futures::stream::iter(
                                itertools::Itertools::chunks(
                                    rd_ids_to_2_dims_vec_rd_inn_acc.into_iter(),
                                    CM_CHUNK_SIZE_2EE9377B,
                                )
                                .into_iter()
                                .map(|el_6f515764| el_6f515764.collect::<Vec<#ident_cr_ucc>>())
                                .map(|el_8e425cb1| async move { #ident::try_cm_h(
                                    url,
                                    #ident_cm_prms_ucc {
                                        payload: #ident_cm_payload_ucc(el_8e425cb1)
                                    },
                                    tbl_9c259e1c
                                ).await.expect("38a24e7a") })
                            ),
                            CM_CONCURRENCY_7CCFD82D
                        ),
                        Vec::with_capacity(rd_ids_to_2_dims_vec_rd_inn_acc_len),
                        |mut acc_a33fb452, rd_ids_78f10a3d| async move {
                            acc_a33fb452.extend(rd_ids_78f10a3d);
                            acc_a33fb452
                        }
                    )
                    .await;
                    #assert_eq_rd_ids_els_ts
                    rd_ids_els_efeed554
                }
            }
        };
        let gen_ft_opt_vec_cr_ts: &dyn Fn(&syn::Type) -> proc_macro2::TokenStream = &|ft| {
            let ts = gen_as_pg_type_test_cases_path_ts(ft);
            quote::quote! {#ts #OptVecCrSc()}
        };
        let gen_ft_opt_vec_cr_or_vec_ts: &dyn Fn(&syn::Type) -> proc_macro2::TokenStream = &|ft| {
            let ts = gen_ft_opt_vec_cr_ts(ft);
            quote::quote! {#ts.unwrap_or(Vec::new())}
        };
        let gen_ident_ft_opt_vec_cr_or_vec_ts: &dyn Fn(
            &syn::Ident,
            &syn::Type,
        ) -> proc_macro2::TokenStream = &|_, ft| gen_ft_opt_vec_cr_or_vec_ts(ft);
        let gen_for_in_1_2_ts = |el_ts: &dyn quote::ToTokens, ts: &dyn quote::ToTokens| {
            quote::quote! {
                for #el_ts in [1,2] {
                    #ts
                }
            }
        };
        let gen_vec_pk_sorted_rd_ts = |ts: &dyn quote::ToTokens| {
            quote::quote! {itertools::Itertools::sorted(#ts).collect::<Vec<#pk_ft_as_pg_type_rd_ts>>()}
        };
        let vec_pk_sorted_rd_ts = gen_vec_pk_sorted_rd_ts(&quote::quote! {
            rd_ids_from_try_cm
            .into_iter()
            .map(|el_43ab7fb5| {
                #pk_ft_rd_ids_into_rd_el_43ab7fb5_pk_fi_ts
            })
        });
        let gen_try_dm_h_ts = |ts: &dyn quote::ToTokens, tbl_ts: &dyn quote::ToTokens| {
            quote::quote! {
                #ident::try_dm_h(
                    &url_cloned,
                    #ident_dm_prms_ucc {
                        //todo rewrite it using new\try_new?
                        payload: #ident_dm_payload_ucc {
                            wh_many: #opt_ident_wh_ucc(Some(
                                #ident_wh_ucc {
                                    #ts
                                }
                            ))
                        }
                    },
                    &#tbl_ts
                )
                .await
                .expect("716e470e")
            }
        };
        let gen_rd_ids_from_try_dm_ts = |ts: &dyn quote::ToTokens| {
            quote::quote! {
                let rd_ids_from_try_dm = #ts;
            }
        };
        let gen_rd_ids_from_try_dm_sorted_pk_ts =
            |tbl_ts: &dyn quote::ToTokens, some_ts: &dyn quote::ToTokens| {
                gen_rd_ids_from_try_dm_ts(&gen_vec_pk_sorted_rd_ts(&{
                    let ts = gen_try_dm_h_ts(
                        &quote::quote! {
                            #pk_fi: Some(#some_ts),
                            #opt_ident_wh_fields_none_ts
                        },
                        &tbl_ts,
                    );
                    quote::quote! {#ts.into_iter()}
                }))
            };
        let gen_acc_push_future_ts =
            |ts0: &dyn quote::ToTokens, ts1: &dyn quote::ToTokens, ts2: &dyn quote::ToTokens| {
                quote::quote! {
                    let #ts0 = #ts1.clone();
                    let url_cloned = url.clone();
                    let sel_dflt_all_with_max_page_size_cloned = #sel_dflt_all_with_max_page_size_clone_ts;
                    acc_9189f86e.push(futures::FutureExt::boxed(async move {
                        #ts2
                    }));
                }
            };
        let vec_rd_from_rd_ids_with_cr_ts = quote::quote! {
            gen_vec_ident_rd_from_vec_ident_rd_ids_with_vec_ident_cr(
                rd_ids_from_try_cm.clone(),
                ident_vec_cr.clone()
            )
        };
        let cm_tests_ts = {
            let cm_tests_ts = gen_fields_named_without_pk_without_comma_ts(
                &|el: &macros_helpers::field_data::SynField| {
                    let fi = &el.ident;
                    let ft = &el.type0;
                    let cm_ident_cr_cnt_el_id_ts =
                        gen_ident_cr_cnt_el_id_ts(fi, &quote::quote! {el_03a4f4ee});
                    let ft_opt_vec_cr_or_vec_ts = gen_ft_opt_vec_cr_or_vec_ts(ft);
                    let wh_pk_or_rd_ids_cm_ts =
                        gen_wh_pk_or_ts(&quote::quote! {rd_ids_from_try_cm #pk_wh_eq_iter_map_ts});
                    let assert_eq_cm_rm_ts = gen_assert_eq_ts(
                        &vec_rd_from_rd_ids_with_cr_ts,
                        &quote::quote! {
                            gen_try_rm_order_by_pk_with_big_pgn(
                                &url_cloned,
                                #wh_pk_or_rd_ids_cm_ts,
                                #sel_dflt_all_with_max_page_size_cloned_clone_ts,
                                &tbl_cm_cloned
                            ).await.expect("bdb72341")
                        },
                        &quote::quote! {"d19bbbf6"},
                    );
                    let assert_eq_cm_dm_pks_ts = gen_assert_eq_ts(
                        &quote::quote! {rd_ids_from_try_dm},
                        &vec_pk_sorted_rd_ts,
                        &quote::quote! {"f58f5572"},
                    );
                    let assert_cm_dm_empty_ts = gen_assert_ts(
                        &{
                            let ts = gen_pk_wh_eq_into_inn_ts(&quote::quote! {el_a37bca54.clone()});
                            let wh_pk_or_dm_ts = gen_wh_pk_or_ts(&quote::quote! {
                                {
                                    let mut acc_87ea12c9 = Vec::with_capacity(rd_ids_from_try_dm.len());
                                    for el_a37bca54 in &rd_ids_from_try_dm {
                                        acc_87ea12c9.push(#ts);
                                    }
                                    acc_87ea12c9
                                }
                            });
                            quote::quote! {
                                gen_try_rm_order_by_pk_with_big_pgn(
                                    &url_cloned,
                                    #wh_pk_or_dm_ts,
                                    #sel_dflt_all_with_max_page_size_cloned_clone_ts,
                                    &tbl_cm_cloned
                                ).await
                                .expect("24ab86d6")
                                .is_empty()
                            }
                        },
                        &quote::quote! {"4e88679a"},
                    );
                    let cm_rd_ids_from_try_dm_sorted_pk_ts = gen_rd_ids_from_try_dm_sorted_pk_ts(
                        &quote::quote! {tbl_cm_cloned},
                        &quote::quote! {
                            gen_pg_type_wh_try_new_or_pks(&rd_ids_from_try_cm)
                        },
                    );
                    let cm_acc_push_future_ts = gen_acc_push_future_ts(
                        &quote::quote! {tbl_cm_cloned},
                        &quote::quote! {tbl_cm},
                        &quote::quote! {
                            let ident_vec_cr = {
                                let mut acc_92d248f7 = Vec::with_capacity(el_fce0969c.len());
                                for el_03a4f4ee in el_fce0969c {
                                    acc_92d248f7.push(#ident_cr_ucc {
                                        #cm_ident_cr_cnt_el_id_ts
                                    });
                                }
                                acc_92d248f7
                            };
                            let rd_ids_from_try_cm = #ident::try_cm_h(
                                &url_cloned,
                                #ident_cm_prms_ucc {
                                    #PayloadSc: #ident_cm_payload_ucc(ident_vec_cr.clone())
                                },
                                &tbl_cm_cloned
                            ).await.expect("5eecedc4");
                            #assert_eq_cm_rm_ts
                            #cm_rd_ids_from_try_dm_sorted_pk_ts
                            #assert_eq_cm_dm_pks_ts
                            #assert_cm_dm_empty_ts
                        },
                    );
                    quote::quote! {
                        const CM_CHUNK_SIZE_A13F7C92: usize = 10;
                        for el_fce0969c in #ft_opt_vec_cr_or_vec_ts.chunks(CM_CHUNK_SIZE_A13F7C92) {
                            #cm_acc_push_future_ts
                        }
                    }
                },
            );
            quote::quote! {#cm_tests_ts}
        };
        let co_tests_ts = {
            let co_tests_ts = gen_fields_named_without_pk_without_comma_ts(
                &|el: &macros_helpers::field_data::SynField| {
                    let fi = &el.ident;
                    let ft = &el.type0;
                    let co_ident_cr_cnt_el_id_ts =
                        gen_ident_cr_cnt_el_id_ts(fi, &quote::quote! {el_7632d698});
                    let ts = gen_v_init_ts0(&pk_ft_rd_ids_into_rd_rd_ids_from_try_co_pk_fi_ts);
                    let ft_opt_vec_cr_or_vec_ts = gen_ft_opt_vec_cr_or_vec_ts(ft);
                    let assert_eq_co_ro_pk_ts = gen_assert_eq_ts(
                        &quote::quote! {
                            #ident_rd_ucc {
                                #pk_fi: Some(#ts),
                                #fi_rd_ids_and_cr_into_opt_v_rd_rd_ids_from_try_co_ident_cr_ts
                            }
                        },
                        &quote::quote! {
                            gen_ident_try_ro_h_pk(
                                &url_cloned,
                                #pk_ft_rd_ids_into_rd_rd_ids_from_try_co_pk_fi_ts,
                                #sel_dflt_all_with_max_page_size_cloned_clone_ts,
                                &tbl_co_cloned
                            )
                            .await
                            .expect("f8e1cb88")
                        },
                        &quote::quote! {"5f2adbed"},
                    );
                    let assert_eq_co_dlo_pk_ts = gen_assert_eq_ts(
                        &quote::quote! {
                            gen_try_dlo_h(
                                &url_cloned,
                                #pk_ft_rd_ids_into_rd_rd_ids_from_try_co_pk_fi_ts,
                                &tbl_co_cloned
                            ).await.expect("20d5a40a")
                        },
                        &quote::quote! {#pk_ft_rd_ids_into_rd_rd_ids_from_try_co_pk_fi_ts},
                        &quote::quote! {"4f563faf"},
                    );
                    let co_acc_push_future_ts = gen_acc_push_future_ts(
                        &quote::quote! {tbl_co_cloned},
                        &quote::quote! {tbl_co},
                        &quote::quote! {
                            let ident_cr = #ident_cr_ucc {
                                #co_ident_cr_cnt_el_id_ts
                            };
                            let rd_ids_from_try_co = gen_rd_ids_from_try_co(
                                &url_cloned,
                                ident_cr.clone(),
                                &tbl_co_cloned
                            ).await;
                            #assert_eq_co_ro_pk_ts
                            #assert_eq_co_dlo_pk_ts
                            gen_check_no_rows_from_ident_try_ro_h_pk(
                                &url_cloned,
                                #pk_ft_rd_ids_into_rd_rd_ids_from_try_co_pk_fi_ts,
                                sel_dflt_all_with_max_page_size_cloned,
                                &tbl_co_cloned,
                            ).await;
                        },
                    );
                    quote::quote! {
                        for el_7632d698 in #ft_opt_vec_cr_or_vec_ts {
                            #co_acc_push_future_ts
                        }
                    }
                },
            );
            quote::quote! {#co_tests_ts}
        };
        let add_co_dflt_and_del_after_just_to_add_some_data_to_be_sure_it_will_not_return_from_the_test_query_ts =
            |ts: &dyn quote::ToTokens| {
                quote::quote! {
                    let rd_ids_from_try_co = gen_rd_ids_from_try_co_dflt(
                        &url_cloned,
                        &tbl_7e35b1ce
                    ).await;
                    #ts
                    let _: #pk_ft_as_pg_type_rd_ts = gen_try_dlo_h(
                        &url_cloned,
                        #pk_ft_rd_ids_into_rd_rd_ids_from_try_co_pk_fi_ts,
                        &tbl_7e35b1ce
                    ).await.expect("93b4bf61");
                    gen_check_no_rows_from_ident_try_ro_h_pk(
                        &url_cloned,
                        #pk_ft_rd_ids_into_rd_rd_ids_from_try_co_pk_fi_ts,
                        sel_dflt_all_with_max_page_size_cloned,
                        &tbl_7e35b1ce,
                    ).await;
                }
            };
        let rm_tests_ts = {
            //todo extra rm checks
            let wh_pk_or_repeat_uuid_ts = gen_wh_pk_or_ts(&quote::quote! {
                std::iter::repeat_with(|| #pk_wh_eq_uuid_new_v_ts)
                .take(el_30614c66)
            });
            let test_rm_by_non_existent_pks_ts = gen_for_in_1_2_ts(
                &quote::quote! {el_30614c66},
                &gen_acc_push_future_ts(
                    &quote::quote! {tbl_7e35b1ce},
                    &quote::quote! {tbl_test_rm_by_non_existent_pks},
                    &add_co_dflt_and_del_after_just_to_add_some_data_to_be_sure_it_will_not_return_from_the_test_query_ts(&gen_assert_ts(
                        &quote::quote! {
                            gen_try_rm_order_by_pk_with_big_pgn(
                                &url_cloned,
                                #wh_pk_or_repeat_uuid_ts,
                                sel_dflt_all_with_max_page_size_cloned.clone(),
                                &tbl_7e35b1ce
                            ).await
                            .expect("e661c49b")
                            .is_empty()
                        },
                        &quote::quote! {"06df4025"}
                    ))
                )
            );
            let test_rm_by_eq_to_crd_pks_ts = gen_for_in_1_2_ts(&quote::quote! {el_a636d084}, &{
                let ts = gen_acc_push_future_ts(
                        &quote::quote! {tbl_7e35b1ce},
                        &quote::quote! {tbl_test_rm_by_eq_to_crd_pks},
                        &add_co_dflt_and_del_after_just_to_add_some_data_to_be_sure_it_will_not_return_from_the_test_query_ts(&{
                            let wh_pk_or_rd_ids_cm_ts = gen_wh_pk_or_ts(&quote::quote! {rd_ids_from_try_cm #pk_wh_eq_iter_map_ts});
                            let assert_eq_rm_crd_pks_ts = gen_assert_eq_ts(
                                &vec_rd_from_rd_ids_with_cr_ts,
                                &quote::quote! {
                                    gen_try_rm_order_by_pk_with_big_pgn(
                                        &url_cloned,
                                        #wh_pk_or_rd_ids_cm_ts,
                                        sel_dflt_all_with_max_page_size_cloned.clone(),
                                        &tbl_7e35b1ce
                                    ).await.expect("b8efe770")
                                },
                                &quote::quote! {"er 3b2cf1f5-2c4e-4908-ba66-f4af84fe0893"},
                            );
                            let assert_eq_rm_dm_pks_ts = gen_assert_eq_ts(
                                &quote::quote! {rd_ids_from_try_dm},
                                &vec_pk_sorted_rd_ts,
                                &quote::quote! {"ebbbea6e"},
                            );
                            let assert_rm_dm_empty_ts = gen_assert_ts(
                                &{
                                    let ts = gen_pk_wh_eq_into_inn_ts(&quote::quote! {el_1e9c87ce.clone()});
                                    let wh_pk_or_dm_ts = gen_wh_pk_or_ts(&quote::quote! {
                                        rd_ids_from_try_dm
                                        .iter()
                                        .map(|el_1e9c87ce| #ts)
                                    });
                                    quote::quote! {
                                        gen_try_rm_order_by_pk_with_big_pgn(
                                            &url_cloned,
                                            #wh_pk_or_dm_ts,
                                            sel_dflt_all_with_max_page_size_cloned.clone(),
                                            &tbl_7e35b1ce
                                        ).await
                                        .expect("1f079962")
                                        .is_empty()
                                    }
                                },
                                &quote::quote! {"d79c0af3"}
                            );
                            let rm_rd_ids_from_try_dm_sorted_pk_ts = gen_rd_ids_from_try_dm_sorted_pk_ts(
                                &quote::quote! {tbl_7e35b1ce},
                                &quote::quote! {
                                    gen_pg_type_wh_try_new_or_pks(&rd_ids_from_try_cm)
                                }
                            );
                            quote::quote! {
                                let ident_vec_cr = std::iter::repeat_n(
                                    ident_cr_dflt_cloned.clone(),
                                    el_a636d084
                                ).collect::<Vec<#ident_cr_ucc>>();
                                let rd_ids_from_try_cm = #ident::try_cm_h(
                                    &url_cloned,
                                    #ident_cm_prms_ucc {
                                        payload: #ident_cm_payload_ucc(ident_vec_cr.clone())
                                    },
                                    &tbl_7e35b1ce
                                ).await.expect("d775179f");
                                #assert_eq_rm_crd_pks_ts
                                #rm_rd_ids_from_try_dm_sorted_pk_ts
                                #assert_eq_rm_dm_pks_ts
                                #assert_rm_dm_empty_ts
                            }
                        })
                    );
                quote::quote! {
                    let ident_cr_dflt_cloned = ident_cr_dflt.clone();
                    #ts
                }
            });
            let gen_rd_ids_and_cr_into_wh_assert_eq_ts = |ts: &dyn quote::ToTokens| {
                gen_assert_eq_ts(
                    &quote::quote! {vec![
                        #ident_rd_ucc {
                            #pk_fi: #pk_as_pg_type_test_cases_path_ts rd_ids_to_opt_v_rd_dflt_some_one_el(
                                &rd_ids_from_co.#pk_fi
                            ),
                            #fi_rd_ids_and_cr_into_opt_v_rd_rd_ids_from_co_clone_ident_cr_clone_ts
                        }
                    ]},
                    &quote::quote! {
                        gen_try_rm_order_by_pk_with_big_pgn(
                            &url_cloned,
                            #ident_wh_ucc::try_new(#ts).expect("83c2d430"),
                            #sel_dflt_all_with_max_page_size_cloned_clone_ts,
                            &tbl_7e35b1ce
                        ).await.expect("c3e316c0")
                    },
                    &quote::quote! {"ee8d232d"},
                )
            };
            let gen_rd_test_ts: &dyn Fn(
                &str,
                &dyn Fn(&syn::Ident, &syn::Type) -> proc_macro2::TokenStream,
                &dyn Fn(&syn::Ident) -> proc_macro2::TokenStream,
                &dyn Fn(&macros_helpers::field_data::SynField) -> proc_macro2::TokenStream,
            ) -> proc_macro2::TokenStream = &|test_name, gen_method_call_ts, gen_cr_ts, gen_ts| {
                gen_fields_named_without_pk_without_comma_ts(
                    &|el: &macros_helpers::field_data::SynField| {
                        let fi = &el.ident;
                        let ft = &el.type0;
                        let method_call_ts = gen_method_call_ts(fi, ft);
                        let tbl_test_name_fi_ts = gen_tbl_test_name_fi_ts(test_name, fi);
                        let wh_ident_cr_ts = gen_cr_ts(fi);
                        let ts = gen_ts(el);
                        let assert_eq_wh_dm_pks_ts = gen_assert_eq_ts(
                            &quote::quote! {rd_ids_from_try_dm},
                            &quote::quote! {vec![#pk_ft_rd_ids_into_rd_rd_ids_from_co_pk_fi_ts]},
                            &quote::quote! {"9fc29fa5"},
                        );
                        let assert_wh_dm_empty_ts = gen_assert_ts(
                            &{
                                let wh_pk_wh_eq_into_inn_ts = gen_pk_wh_eq_into_inn_ts(
                                    &pk_ft_rd_ids_into_rd_rd_ids_from_co_pk_fi_ts,
                                );
                                let wh_pk_or_co_ts = gen_wh_pk_or_ts(
                                    &quote::quote! {vec![#wh_pk_wh_eq_into_inn_ts]},
                                );
                                quote::quote! {
                                    gen_try_rm_order_by_pk_with_big_pgn(
                                        &url_cloned,
                                        #wh_pk_or_co_ts,
                                        #sel_dflt_all_with_max_page_size_cloned_clone_ts,
                                        &tbl_7e35b1ce
                                    ).await
                                    .expect("1817b67a")
                                    .is_empty()
                                }
                            },
                            &quote::quote! {"38187925"},
                        );
                        let wh_pk_wh_eq_co_ts =
                            gen_pk_wh_eq_into_inn_ts(&pk_ft_rd_ids_into_rd_rd_ids_from_co_pk_fi_ts);
                        let wh_rd_ids_from_try_dm_sorted_pk_ts =
                            gen_rd_ids_from_try_dm_sorted_pk_ts(
                                &quote::quote! {tbl_7e35b1ce},
                                &quote::quote! {
                                    gen_pg_type_wh_try_new_pk(
                                        #oprtr_or_ts,
                                        vec![#wh_pk_wh_eq_co_ts]
                                    )
                                },
                            );
                        let wh_acc_push_future_ts = gen_acc_push_future_ts(
                            &quote::quote! {tbl_7e35b1ce},
                            &tbl_test_name_fi_ts,
                            &quote::quote! {
                                let ident_cr = #ident_cr_ucc {
                                    #wh_ident_cr_ts
                                };
                                let rd_ids_from_co = gen_rd_ids_from_try_co(
                                    &url_cloned,
                                    ident_cr.clone(),
                                    &tbl_7e35b1ce
                                ).await;
                                #ts
                                #wh_rd_ids_from_try_dm_sorted_pk_ts
                                #assert_eq_wh_dm_pks_ts
                                #assert_wh_dm_empty_ts
                            },
                        );
                        quote::quote! {
                            for #ElSc in #method_call_ts {
                                #wh_acc_push_future_ts
                            }
                        }
                    },
                )
            };
            let some_pk_wh_init_ts = quote::quote! {
                Some(
                    gen_pg_type_wh_try_new_pk(
                        #oprtr_and_ts,
                        vec![
                            #pk_as_pg_type_test_cases_path_ts rd_ids_and_cr_into_wh_eq(
                                rd_ids_from_co.#pk_fi,
                                #PgCrudCmnDfltSomeOneElCall
                            )
                        ]
                    )
                )
            };
            let gen_fi_wh_ts: &dyn Fn(
                &syn::Ident,
                &dyn quote::ToTokens,
            ) -> proc_macro2::TokenStream = &|fi, ts| {
                gen_fields_named_with_comma_ts(&|el0: &macros_helpers::field_data::SynField| {
                    let fi0 = &el0.ident;
                    if pk_fi == fi0 {
                        some_pk_wh_init_ts.clone()
                    } else if fi0.as_ref() == fi {
                        gen_some_pg_type_wh_try_new_and_ts(&ts)
                    } else {
                        none_ts.clone()
                    }
                })
            };
            let gen_for_each_assert_eq_ts: &dyn Fn(
                &dyn quote::ToTokens,
                &dyn quote::ToTokens,
                &syn::Ident,
            ) -> proc_macro2::TokenStream = &|v_ts, el_ts, fi| {
                let vec_el_ts = quote::quote! {vec![#el_ts]};
                let assert_eq_ts =
                    gen_rd_ids_and_cr_into_wh_assert_eq_ts(&gen_fi_wh_ts(fi, &vec_el_ts));
                quote::quote! {
                    for #el_ts in #v_ts.into_vec() {
                        #assert_eq_ts
                    }
                }
            };
            let (rd_ids_and_cr_into_wh_eq_ts, rd_ids_and_cr_into_vec_wh_eq_using_fields_ts) = {
                let gen_ts =
                    |test_name, eq_or_eq_using_fields: &pg_crud_macros_cmn::EqOrEqUsingFields| {
                        gen_rd_test_ts(
                            test_name,
                            &gen_ident_ft_opt_vec_cr_or_vec_ts,
                            &gen_ident_cr_cnt_el_ts,
                            &|el: &macros_helpers::field_data::SynField| {
                                let fi = &el.ident;
                                gen_rd_ids_and_cr_into_wh_assert_eq_ts(
                                    &gen_fields_named_with_comma_ts(
                                        &|el0: &macros_helpers::field_data::SynField| {
                                            let fi0 = &el0.ident;
                                            let ft0 = &el0.type0;
                                            if fi0 == pk_fi {
                                                some_pk_wh_init_ts.clone()
                                            } else if fi0 == fi {
                                                let method_ts = {
                                                    let method_ts: &dyn quote::ToTokens =
                                                match &eq_or_eq_using_fields {
                                                    pg_crud_macros_cmn::EqOrEqUsingFields::Eq => &RdIdsAndCrIntoWhEqSc,
                                                    pg_crud_macros_cmn::EqOrEqUsingFields::EqUsingFields => {
                                                        &RdIdsAndCrIntoVecWhEqUsingFieldsSc
                                                    }
                                                };
                                                    let ts0 =
                                                        gen_as_pg_type_test_cases_path_ts(&ft0);
                                                    quote::quote! {
                                                        #ts0 #method_ts(
                                                            rd_ids_from_co.#fi0.clone().expect("11c3740b"),
                                                            ident_cr.#fi0.clone()
                                                        )
                                                    }
                                                };
                                                match &eq_or_eq_using_fields {
                                            pg_crud_macros_cmn::EqOrEqUsingFields::Eq => {
                                                gen_some_pg_type_wh_try_new_and_ts(&quote::quote! {
                                                    vec![#method_ts]
                                                })
                                            }
                                            pg_crud_macros_cmn::EqOrEqUsingFields::EqUsingFields => {
                                                quote::quote! {
                                                    Some(#import_ts PgTypeWh::new(
                                                        #oprtr_and_ts,
                                                        #method_ts
                                                    ))
                                                }
                                            }
                                        }
                                            } else {
                                                none_ts.clone()
                                            }
                                        },
                                    ),
                                )
                            },
                        )
                    };
                (
                    gen_ts(
                        tbl_rd_ids_and_cr_into_wh_eq_name,
                        &pg_crud_macros_cmn::EqOrEqUsingFields::Eq,
                    ),
                    gen_ts(
                        tbl_rd_ids_and_cr_into_vec_wh_eq_using_fields_name,
                        &pg_crud_macros_cmn::EqOrEqUsingFields::EqUsingFields,
                    ),
                )
            };
            let rd_ids_and_cr_into_opt_vec_wh_eq_to_field_ts = gen_rd_test_ts(
                tbl_rd_ids_and_cr_into_opt_vec_wh_eq_to_field_name,
                &gen_ident_ft_opt_vec_cr_or_vec_ts,
                &gen_ident_cr_cnt_el_ts,
                &|el: &macros_helpers::field_data::SynField| {
                    let fi = &el.ident;
                    gen_if_let_some_ts(
                        &quote::quote! {v_d5cd3c70},
                        &{
                            let ft_ts = gen_as_pg_type_test_cases_path_ts(&el.type0);
                            quote::quote! {#ft_ts #RdIdsAndCrIntoOptVecWhEqToFieldSc(
                                rd_ids_from_co.#fi.clone().expect("65cef584"),
                                ident_cr.#fi.clone()
                            )}
                        },
                        &gen_for_each_assert_eq_ts(
                            &quote::quote! {v_d5cd3c70},
                            &quote::quote! {el_48a3d976},
                            fi,
                        ),
                    )
                },
            );
            let rd_ids_and_tt_into_pg_type_opt_wh_greater_than_ts = gen_rd_test_ts(
                tbl_rd_ids_and_tt_into_pg_type_opt_wh_greater_than_name,
                &|_, ft| {
                    quote::quote! {
                        <#ft as #import_ts PgTypeTestCases>::#PgTypeOptVecWhGreaterThanTestSc()
                        .map_or_else(Vec::new, Into::into)
                    }
                },
                &|fi| gen_ident_cr_ts(fi, &quote::quote! {#ElSc.#CrSc}),
                &|el: &macros_helpers::field_data::SynField| {
                    let fi = &el.ident;
                    gen_if_let_some_ts(
                        &quote::quote! {v_60baba1f},
                        &{
                            let ft_ts = gen_as_pg_type_test_cases_path_ts(&el.type0);
                            quote::quote! {#ft_ts #RdIdsAndTtIntoPgTypeOptWhGreaterThanSc(
                                #ElSc.vrt,
                                rd_ids_from_co.#fi.clone().expect("c8d34556"),
                                #ElSc.greater_than,
                            )}
                        },
                        &gen_rd_ids_and_cr_into_wh_assert_eq_ts(&gen_fi_wh_ts(
                            fi,
                            &quote::quote! {vec![v_60baba1f]},
                        )),
                    )
                },
            );
            quote::quote! {
                #test_rm_by_non_existent_pks_ts
                #test_rm_by_eq_to_crd_pks_ts
                #rd_ids_and_cr_into_wh_eq_ts
                #rd_ids_and_cr_into_vec_wh_eq_using_fields_ts
                #rd_ids_and_cr_into_opt_vec_wh_eq_to_field_ts
                #rd_ids_and_tt_into_pg_type_opt_wh_greater_than_ts
            }
        };
        let ro_tests_ts = gen_acc_push_future_ts(
            &quote::quote! {tbl_ro_cloned},
            &quote::quote! {tbl_ro},
            &quote::quote! {
                    gen_check_no_rows_from_ident_try_ro_h_pk(
                        &url_cloned,
                        #pk_ft_as_pg_type_rd_ts::new(uuid::Uuid::new_v4()),
                        #sel_dflt_all_with_max_page_size_cloned_clone_ts,
                        &tbl_ro_cloned,
                    ).await;
            },
        );
        let gen_ident_rd_init_ts = |ts: &dyn quote::ToTokens| {
            let ts0 = gen_v_init_ts0(&pk_ft_rd_only_is_into_rd_rd_ids_el_pk_fi_ts);
            quote::quote! {#ident_rd_ucc {
                #pk_fi: Some(#ts0),
                #ts
            }}
        };
        let gen_rd_inn_into_upd_ts =
            |fi: &dyn quote::ToTokens,
             ft: &dyn quote::ToTokens,
             ft_ts: &dyn quote::ToTokens,
             i_ts: &dyn quote::ToTokens| {
                let ts = gen_as_pg_type_test_cases_path_ts(&ft);
                quote::quote! {
                    let upd = #ts rd_inn_into_upd_with_new_or_try_new_unwraped({
                        let mut i_e0d2f9db: usize = 0;
                        let mut opt_test_case = None;
                        for el_3a9a65ee in #ft_ts rd_ids_to_2_dims_vec_rd_inn(
                            &rd_ids_el_937c5af3.#fi.clone().expect("c4d98a71")
                        ) {
                            let mut should_break = false;
                            for el_bb734c11 in el_3a9a65ee {
                                if i_e0d2f9db == #i_ts {
                                    opt_test_case = Some(el_bb734c11);
                                    should_break = true;
                                    break;
                                }
                                i_e0d2f9db = i_e0d2f9db.checked_add(1).expect("326274d1");
                            }
                            if should_break {
                                break;
                            }
                        }
                        opt_test_case.expect("bd79056e")
                    });
                }
            };
        let gen_rd_ids_upper_fields_init_without_pk_ts: &dyn Fn(
            &syn::Ident,
        )
            -> proc_macro2::TokenStream = &|fi| {
            gen_fields_named_without_pk_with_comma_ts(
                &|syn_field: &macros_helpers::field_data::SynField| {
                    let fi0 = &syn_field.ident;
                    let ts = if fi == fi0.as_ref() {
                        let ts0 = gen_as_pg_type_test_cases_path_ts(&syn_field.type0);
                        quote::quote! {Some(#ts0 upd_to_rd_ids(&upd))}
                    } else {
                        quote::quote! {None}
                    };
                    quote::quote! {#fi0: #ts}
                },
            )
        };
        let gen_upd_prms_init_without_pk_ts: &dyn Fn(&syn::Ident) -> proc_macro2::TokenStream =
            &|fi| {
                gen_fields_named_without_pk_with_comma_ts(
                    &|syn_field: &macros_helpers::field_data::SynField| {
                        let fi0 = &syn_field.ident;
                        if fi == fi0.as_ref() {
                            let ts = gen_v_init_ts0(&quote::quote! {#UpdSc.clone()});
                            quote::quote! {Some(#ts)}
                        } else {
                            none_ts.clone()
                        }
                    },
                )
            };
        let gen_rd_fields_after_upd_ts: &dyn Fn(
            &syn::Ident,
            &dyn Fn(&syn::Ident) -> proc_macro2::TokenStream,
            &str,
            &str,
        ) -> proc_macro2::TokenStream = &|fi, else_fn, expect_uuid_0, expect_uuid_1| {
            gen_fields_named_without_pk_with_comma_ts(
                &|syn_field: &macros_helpers::field_data::SynField| {
                    let fi0 = &syn_field.ident;
                    let ts = if fi == fi0.as_ref() {
                        let ts0 = gen_v_init_ts0(&{
                            let ts1 = gen_as_pg_type_test_cases_path_ts(&syn_field.type0);
                            let expect_0 = gen_quotes::dq_ts(&expect_uuid_0);
                            let expect_1 = gen_quotes::dq_ts(&expect_uuid_1);
                            quote::quote! {
                                #ts1 previous_rd_and_opt_upd_into_rd(
                                    #ts1 rd_ids_to_opt_v_rd_dflt_some_one_el(
                                        &rd_ids_el_937c5af3.#fi0.clone().expect(#expect_0)
                                    ).expect(#expect_1).#VSc,
                                    Some(#UpdSc.clone())
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
        let um_tests_ts = {
            //todo add Test for trying to upd empty vec
            let um_only_one_col_tests_ts = gen_fields_named_without_pk_without_comma_ts(
                &|el: &macros_helpers::field_data::SynField| {
                    let fi = &el.ident;
                    let ft = &el.type0;
                    let ft_ts = gen_as_pg_type_test_cases_path_ts(ft);
                    let is_fields_without_pk_len_greater_than_one = fields_len_without_pk > 1;
                    let mb_previous_rd_ts = if is_fields_without_pk_len_greater_than_one {
                        let ts =
                            gen_pk_wh_eq_into_inn_ts(&pk_ft_rd_only_is_into_rd_rd_ids_el_pk_fi_ts);
                        let wh_pk_or_um_ts = gen_wh_pk_or_ts(&quote::quote! {vec![#ts]});
                        quote::quote! {
                            let previous_rd = itertools::Itertools::sorted_by(
                                gen_try_rm_order_by_pk_with_big_pgn(
                                    &url_cloned,
                                    #wh_pk_or_um_ts,
                                    #sel_dflt_all_with_max_page_size_cloned_clone_ts,
                                    &tbl_um_cloned
                                )
                                .await
                                .expect("540ec737")
                                .into_iter(),
                                #pk_sort_cmp_ts
                            );
                        }
                    } else {
                        proc_macro2::TokenStream::new()
                    };
                    let fi_rd_ids_to_2_dims_vec_rd_inn_acc_sc =
                        naming::prm::SelfRdIdsTo2DimsVecRdInnAccSc::from_tokens(&fi);
                    let ident_rd_ids_upper_fields_init_without_pk_ts =
                        gen_rd_ids_upper_fields_init_without_pk_ts(fi);
                    let ident_upd_prms_init_without_pk_ts = gen_upd_prms_init_without_pk_ts(fi);
                    let ident_rd_fields_init_without_pk_after_uo_ts = gen_rd_fields_after_upd_ts(
                        fi,
                        &|fi0| quote::quote! {el_a6bc6b2f.#fi0},
                        "96213542",
                        "bf0d6f55",
                    );
                    let expected_rm_ts = {
                        let ts = gen_ident_rd_init_ts(&ident_rd_fields_init_without_pk_after_uo_ts);
                        if is_fields_without_pk_len_greater_than_one {
                            quote::quote! {previous_rd.into_iter().map(|el_a6bc6b2f|#ts).collect::<Vec<#ident_rd_ucc>>()}
                        } else {
                            quote::quote! {vec![#ts]}
                        }
                    };
                    let um_rd_inn_into_upd_ts =
                        gen_rd_inn_into_upd_ts(&fi, &ft, &ft_ts, &quote::quote! {i_7f181188});
                    let assert_eq_um_rd_ids_ts = gen_assert_eq_ts(
                        &quote::quote! {vec![
                            #ident_rd_ids_ucc {
                                #pk_fi: rd_ids_el_937c5af3.#pk_fi,
                                #ident_rd_ids_upper_fields_init_without_pk_ts
                            }
                        ]},
                        &quote::quote! {
                            #ident::try_um_h(
                                &url_cloned,
                                #ident_um_prms_ucc {
                                    payload: #ident_um_payload_ucc::try_new(vec![
                                        #ident_upd_ucc::try_new(
                                            #pk_ft_as_pg_type_upd_as_pg_type_pk_rd_ids_into_upd_ts,
                                            #ident_upd_prms_init_without_pk_ts
                                        ).expect("42dc87b3")
                                    ]).expect("69e1bd8a")
                                },
                                &tbl_um_cloned
                            ).await.expect("d2de0bd6")
                        },
                        &quote::quote! {"34bfb3c7"},
                    );
                    let assert_eq_um_rm_ts = gen_assert_eq_ts(
                        &quote::quote! {{#expected_rm_ts}},
                        &{
                            let ts = gen_pk_wh_eq_into_inn_ts(
                                &pk_ft_rd_only_is_into_rd_rd_ids_el_pk_fi_ts,
                            );
                            let wh_pk_or_um_ts = gen_wh_pk_or_ts(&quote::quote! {vec![#ts]});
                            quote::quote! {
                                itertools::Itertools::sorted_by(
                                    gen_try_rm_order_by_pk_with_big_pgn(
                                        &url_cloned,
                                        #wh_pk_or_um_ts,
                                        sel_dflt_all_with_max_page_size_cloned,
                                        &tbl_um_cloned
                                    )
                                    .await
                                    .expect("25c561e2")
                                    .into_iter(),
                                    #pk_sort_cmp_ts
                                ).collect::<Vec<#ident_rd_ucc>>()
                            }
                        },
                        &quote::quote! {"ae2a2da5"},
                    );
                    let um_acc_push_future_ts = gen_acc_push_future_ts(
                        &quote::quote! {tbl_um_cloned},
                        &quote::quote! {tbl_um},
                        &quote::quote! {
                            #mb_previous_rd_ts
                            #um_rd_inn_into_upd_ts
                            #assert_eq_um_rd_ids_ts
                            #assert_eq_um_rm_ts
                        },
                    );
                    quote::quote! {
                        for (i_7f181188, rd_ids_el_937c5af3) in gen_rd_ids_els_8a1ef027(
                            &url,
                            &tbl_um,
                            #sel_dflt_all_with_max_page_size_clone_ts,
                            #fi_rd_ids_to_2_dims_vec_rd_inn_acc_sc.clone()
                        ).await.into_iter().enumerate() {
                            #um_acc_push_future_ts
                        }
                    }
                },
            );
            quote::quote! {#um_only_one_col_tests_ts}
        };
        let uo_tests_ts = {
            let uo_only_one_col_tests_ts = gen_fields_named_without_pk_without_comma_ts(
                &|el: &macros_helpers::field_data::SynField| {
                    let fi = &el.ident;
                    let ft = &el.type0;
                    let ft_ts = gen_as_pg_type_test_cases_path_ts(ft);
                    let mb_previous_rd_ts = if fields_len_without_pk > 1 {
                        quote::quote! {
                            let previous_rd = gen_ident_try_ro_h_pk(
                                &url_cloned,
                                #pk_ft_rd_only_is_into_rd_rd_ids_el_pk_fi_ts,
                                #sel_dflt_all_with_max_page_size_cloned_clone_ts,
                                &tbl_uo_cloned
                            )
                            .await.expect("e6998b47");
                        }
                    } else {
                        proc_macro2::TokenStream::new()
                    };
                    let fi_rd_ids_to_2_dims_vec_rd_inn_acc_sc =
                        naming::prm::SelfRdIdsTo2DimsVecRdInnAccSc::from_tokens(&fi);
                    let ident_rd_ids_upper_fields_init_without_pk_ts =
                        gen_rd_ids_upper_fields_init_without_pk_ts(fi);
                    let ident_upd_prms_init_without_pk_ts = gen_upd_prms_init_without_pk_ts(fi);
                    let ident_rd_fields_init_without_pk_after_uo_ts = gen_rd_fields_after_upd_ts(
                        fi,
                        &|fi0| quote::quote! {previous_rd.#fi0},
                        "4f19d0d2",
                        "c7685b19",
                    );
                    let uo_rd_inn_into_upd_ts =
                        gen_rd_inn_into_upd_ts(&fi, &ft, &ft_ts, &quote::quote! {i_26824592});
                    let assert_eq_uo_rd_ids_ts = gen_assert_eq_ts(
                        &quote::quote! {#ident_rd_ids_ucc {
                            #pk_fi: rd_ids_el_937c5af3.#pk_fi,
                            #ident_rd_ids_upper_fields_init_without_pk_ts
                        }},
                        &quote::quote! {
                            #ident::try_uo_h(
                                &url_cloned,
                                #ident_uo_prms_ucc {
                                    payload: #ident_upd_ucc::try_new(
                                        #pk_ft_as_pg_type_upd_as_pg_type_pk_rd_ids_into_upd_ts,
                                        #ident_upd_prms_init_without_pk_ts
                                    ).expect("0e5d65a5")//todo add col ident
                                },
                                &tbl_uo_cloned
                            ).await.expect("4d755542")
                        },
                        &quote::quote! {"564de31c"},
                    );
                    let assert_eq_uo_ro_ts = gen_assert_eq_ts(
                        &gen_ident_rd_init_ts(&ident_rd_fields_init_without_pk_after_uo_ts),
                        &quote::quote! {
                            gen_ident_try_ro_h_pk(
                                &url_cloned,
                                #pk_ft_rd_only_is_into_rd_rd_ids_el_pk_fi_ts,
                                sel_dflt_all_with_max_page_size_cloned,
                                &tbl_uo_cloned
                            )
                            .await.expect("75894c76")
                        },
                        &quote::quote! {"d5dec823"},
                    );
                    let uo_acc_push_future_ts = gen_acc_push_future_ts(
                        &quote::quote! {tbl_uo_cloned},
                        &quote::quote! {tbl_uo},
                        &quote::quote! {
                            #mb_previous_rd_ts
                            #uo_rd_inn_into_upd_ts
                            #assert_eq_uo_rd_ids_ts
                            #assert_eq_uo_ro_ts
                        },
                    );
                    quote::quote! {
                        for (i_26824592, rd_ids_el_937c5af3) in gen_rd_ids_els_8a1ef027(
                            &url,
                            &tbl_uo,
                            #sel_dflt_all_with_max_page_size_clone_ts,
                            #fi_rd_ids_to_2_dims_vec_rd_inn_acc_sc
                        ).await.into_iter().enumerate() {
                            #uo_acc_push_future_ts
                        }
                    }
                },
            );
            quote::quote! {#uo_only_one_col_tests_ts}
        };
        let dm_tests_ts = {
            let test_dm_by_non_existent_pks_ts = gen_for_in_1_2_ts(
                &quote::quote! {el_39819198},
                &gen_acc_push_future_ts(
                    &quote::quote! {tbl_7e35b1ce},
                    &quote::quote! {tbl_test_rm_by_eq_to_crd_pks},
                    &add_co_dflt_and_del_after_just_to_add_some_data_to_be_sure_it_will_not_return_from_the_test_query_ts(&gen_assert_ts(
                        &{
                            let ts = gen_try_dm_h_ts(
                                &quote::quote! {
                                    #pk_fi: Some(
                                        gen_pg_type_wh_try_new_pk(
                                            #oprtr_or_ts,
                                            std::iter::repeat_with(|| #pk_wh_eq_uuid_new_v_ts)
                                            .take(el_39819198)
                                        )
                                    ),
                                    #fields_none_init_ts
                                },
                                &quote::quote! {tbl_7e35b1ce}
                            );
                            quote::quote! {#ts.is_empty()}
                        },
                        &quote::quote! {"51d14103"}
                    ))
                )
            );
            let test_dm_by_pks_ts = gen_for_in_1_2_ts(&quote::quote! {el_56409d32}, &{
                let dm_acc_push_future_ts = gen_acc_push_future_ts(
                    &quote::quote! {tbl_7e35b1ce},
                    &quote::quote! {tbl_test_rm_by_eq_to_crd_pks},//todo is tbl name correct?
                    &add_co_dflt_and_del_after_just_to_add_some_data_to_be_sure_it_will_not_return_from_the_test_query_ts(&{
                        let assert_eq_dm_rd_ids_ts = gen_assert_eq_ts(
                            &quote::quote! {rd_ids_from_try_dm},
                            &quote::quote! {{
                                rd_ids_from_try_cm.iter().map(|el_ba0f6b1c|
                                    #pk_as_pg_type_test_cases_path_ts rd_ids_to_opt_v_rd_dflt_some_one_el(
                                        &el_ba0f6b1c.#pk_fi
                                    ).expect("3ee5ee86").#VSc
                                ).collect::<Vec<#pk_ft_as_pg_type_rd_ts>>()
                            }},
                            &quote::quote! {"db5e88a6"}
                        );
                        let assert_dm_empty_ts = gen_assert_ts(
                            &{
                                let ts = gen_pk_wh_eq_ts(&gen_pk_ft_as_pg_type_pk_method_call_ts(&RdIntoTtSc, &quote::quote! {el_adcc8db3}));
                                let wh_pk_or_dm_ts = gen_wh_pk_or_ts(&quote::quote! {
                                    rd_ids_from_try_dm.into_iter().map(|el_adcc8db3| #ts)
                                });
                                quote::quote! {
                                    gen_try_rm_order_by_pk_with_big_pgn(
                                        &url_cloned,
                                        #wh_pk_or_dm_ts,
                                        sel_dflt_all_with_max_page_size_cloned.clone(),
                                        &tbl_7e35b1ce
                                    ).await
                                    .expect("bcb79917")
                                    .is_empty()
                                }
                            },
                            &quote::quote! {"77f038b0"}
                        );
                        let dm_pk_wh_eq_ts = gen_pk_wh_eq_ts(&gen_pk_ft_as_pg_type_pk_method_call_ts(
                            &RdIdsIntoTtSc,
                            &quote::quote! {el_3bb88958.#pk_fi},
                        ));
                        let dm_rd_ids_from_try_dm_ts = gen_rd_ids_from_try_dm_ts(&gen_try_dm_h_ts(
                            &quote::quote! {
                                #pk_fi: Some(
                                    gen_pg_type_wh_try_new_pk(
                                        #oprtr_or_ts,
                                        rd_ids_from_try_cm.iter().map(|el_3bb88958| #dm_pk_wh_eq_ts)
                                    )
                                ),
                                #fields_none_init_ts
                            },
                            &quote::quote! {tbl_7e35b1ce}
                        ));
                        quote::quote! {
                            let rd_ids_from_try_cm = #ident::try_cm_h(
                                &url_cloned,
                                #ident_cm_prms_ucc {
                                    payload: #ident_cm_payload_ucc(
                                        std::iter::repeat_n(ident_cr_dflt_cloned, el_56409d32).collect()
                                    )
                                },
                                &tbl_7e35b1ce
                            ).await.expect("b8695890");
                            #dm_rd_ids_from_try_dm_ts
                            #assert_eq_dm_rd_ids_ts
                            #assert_dm_empty_ts
                        }
                    })
                );
                quote::quote! {
                    let ident_cr_dflt_cloned = ident_cr_dflt.clone();
                    #dm_acc_push_future_ts
                }
            });
            quote::quote! {
                #test_dm_by_non_existent_pks_ts
                #test_dm_by_pks_ts
            }
        };
        let dlo_tests_ts = {
            let ts = gen_v_init_ts0(&pk_ft_rd_ids_into_rd_rd_ids_from_co_pk_fi_ts);
            let assert_eq_dlo_ro_pk_ts = gen_assert_eq_ts(
                &quote::quote! {#ident_rd_ucc {
                    #pk_fi: Some(#ts),
                    #fi_rd_ids_and_cr_into_opt_v_rd_rd_ids_from_co_cr_ts
                }},
                &quote::quote! {
                    gen_ident_try_ro_h_pk(
                        &url,
                        #pk_ft_rd_ids_into_rd_rd_ids_from_co_pk_fi_ts,
                        #sel_dflt_all_with_max_page_size_cloned_clone_ts,
                        &tbl_dlo_cloned
                    )
                    .await.expect("c8c44c89")
                },
                &quote::quote! {"86ef08ae"},
            );
            let assert_eq_dlo_del_pk_ts = gen_assert_eq_ts(
                &quote::quote! {
                    gen_try_dlo_h(
                        &url,
                        #pk_ft_rd_ids_into_rd_rd_ids_from_co_pk_fi_ts,
                        &tbl_dlo_cloned
                    ).await.expect("7e1d1a70")
                },
                &quote::quote! {#pk_ft_rd_ids_into_rd_rd_ids_from_co_pk_fi_ts},
                &quote::quote! {"99f81971"},
            );
            let assert_dlo_no_rows_ts = gen_assert_ts(
                &quote::quote! {pg == no_rows_by_a_query_that_expected_to_return_at_least_one_row()},
                &quote::quote! {"c9261bb8"},
            );
            gen_acc_push_future_ts(
                &quote::quote! {tbl_dlo_cloned},
                &quote::quote! {tbl_dlo},
                &quote::quote! {
                        if let Err(#ErSc) = gen_try_dlo_h(
                            &url_cloned,
                            #pk_ft_as_pg_type_rd_ts::new(uuid::Uuid::new_v4()),
                            &tbl_dlo_cloned
                        ).await {
                            if let #ident_try_dlo_er_ucc::#ident_dlo_er_with_serde_ucc {
                                dlo_er_with_serde,
                                ..
                            } = #ErSc {
                                if let #ident_dlo_er_with_serde_ucc::Pg {
                                    pg,
                                    ..
                                } = dlo_er_with_serde {
                                    #assert_dlo_no_rows_ts
                                } else {
                                    panic!("e63b27a3");
                                }
                            } else {
                                panic!("47a8e0d9")
                            }
                        } else {
                            panic!("9be62f9f")
                        }
                        let rd_ids_from_co = gen_rd_ids_from_try_co_dflt(&url_cloned, &tbl_dlo_cloned).await;
                        #assert_eq_dlo_ro_pk_ts
                        #assert_eq_dlo_del_pk_ts
                        gen_check_no_rows_from_ident_try_ro_h_pk(
                            &url_cloned,
                            #pk_ft_rd_ids_into_rd_rd_ids_from_co_pk_fi_ts,
                            #sel_dflt_all_with_max_page_size_cloned_clone_ts,
                            &tbl_dlo_cloned,
                        ).await;
                },
            )
        };
        let assert_tbl_name_len_ts =
            gen_assert_ts(&quote::quote! {v.len() <= 63}, &quote::quote! {"77f9bfb7"});
        let pk_wh_eq_into_inn_rd_ids_ts = gen_pk_wh_eq_into_inn_ts(&quote::quote! {
            #pk_ft_as_pg_type_pk_ts rd_ids_into_rd(el_9530b728.#pk_fi)
        });
        let size_of_ts = {
            let ts = gen_assert_eq_ts(
                &quote::quote! {std::mem::size_of::<#ident>()},
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
        let gen_ident_wh_pk_others_none_fn_ts = quote::quote! {
            fn gen_ident_wh_pk_others_none(
                opt_pg_type_wh: Option<#import_ts PgTypeWh<#pk_ft_as_pg_type_wh_ts>>,
            ) -> #ident_wh_ucc {
                #ident_wh_ucc::try_new(
                    opt_pg_type_wh,
                    #fields_named_without_pk_with_comma_none_ts
                )
                .expect("5fb2b219")
            }
        };
        let gen_pg_type_wh_try_new_pk_fn_ts = quote::quote! {
            fn gen_pg_type_wh_try_new_pk<T>(
                oprtr: #import_ts Oprtr,
                vec: T,
            ) -> #import_ts PgTypeWh<#pk_ft_as_pg_type_wh_ts>
            where
                T: IntoIterator<Item = #pk_ft_as_pg_type_wh_ts>,
            {
                let vec = vec.into_iter().collect::<Vec<#pk_ft_as_pg_type_wh_ts>>();
                #gen_pg_type_wh_try_new_pk_ts
            }
        };
        let gen_pg_type_wh_try_new_or_pks_fn_ts = quote::quote! {
            fn gen_pg_type_wh_try_new_or_pks(
                vec_rd_ids: &[#ident_rd_ids_ucc]
            ) -> #import_ts PgTypeWh<#pk_ft_as_pg_type_wh_ts> {
                gen_pg_type_wh_try_new_pk(
                    #oprtr_or_ts,
                    vec_rd_ids.iter().map(|el_9530b728| #pk_wh_eq_into_inn_rd_ids_ts)
                )
            }
        };
        let gen_try_rm_order_by_pk_with_big_pgn_fn_ts = quote::quote! {
            async fn gen_try_rm_order_by_pk_with_big_pgn(
                endpoint_loc: &str,
                ident_wh_6b1fab92: #ident_wh_ucc,
                sel: #import_ts NotEmptyUnqVec<#ident_sel_ucc>,
                tbl: &str
            ) -> Result<Vec<#ident_rd_ucc>, #ident_try_rm_er_ucc> {
                #ident::try_rm_h(
                    endpoint_loc,
                    #ident_rm_prms_ucc {
                        payload: #ident_rm_payload_ucc {
                            wh_many: #opt_ident_wh_ucc(Some(
                                ident_wh_6b1fab92
                            )),
                            sel,
                            order_by: #import_ts OrderBy {
                                col: #ident_sel_ucc::#pk_fi_ucc_ts(
                                    #pk_ft_as_pg_type_sel_ts::default()
                                ),
                                order: Some(#import_ts Order::Asc)
                            },
                            pgn: #import_ts PgnStartsWithZero::try_new(10000, 0).expect("b0cdf0cb"),
                        }
                    },
                    tbl
                )
                .await
            }
        };
        let gen_ident_try_ro_h_pk_fn_ts = quote::quote! {
            async fn gen_ident_try_ro_h_pk(
                url: &str,
                pk_col: #pk_ft_as_pg_type_rd_ts,
                sel: #import_ts NotEmptyUnqVec<#ident_sel_ucc>,
                tbl: &str,
            ) -> Result<#ident_rd_ucc, #ident_try_ro_er_ucc> {
                #ident::try_ro_h(
                    url,
                    #ident_ro_prms_ucc {
                        payload: #ident_ro_payload_ucc {
                            pk_col,
                            sel,
                        },
                    },
                    tbl,
                )
                .await
            }
        };
        let gen_check_no_rows_from_ident_try_ro_h_pk_fn_ts = {
            let ts = gen_assert_ts(
                &quote::quote! {pg == no_rows_by_a_query_that_expected_to_return_at_least_one_row()},
                &quote::quote! {"58b9a6a4"},
            );
            quote::quote! {
                async fn gen_check_no_rows_from_ident_try_ro_h_pk(
                    url: &str,
                    pk_col: #pk_ft_as_pg_type_rd_ts,
                    sel: #import_ts NotEmptyUnqVec<#ident_sel_ucc>,
                    tbl: &str,
                ) {
                    if let Err(#ErSc) = gen_ident_try_ro_h_pk(
                        url,
                        pk_col,
                        sel,
                        tbl
                    ).await {
                        if let #ident_try_ro_er_ucc::#ident_ro_er_with_serde_ucc {
                            ro_er_with_serde,
                            ..
                        } = er {
                            if let #ident_ro_er_with_serde_ucc::Pg { pg, .. } = ro_er_with_serde {
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
        let ident_cr_dflt_fn_ts = quote::quote! {
            fn ident_cr_dflt() -> #ident_cr_ucc {
                #ident_cr_ucc {
                    #ident_cr_dflt_fields_init_without_pk_ts
                }
            }
        };
        let gen_rd_ids_from_try_co_fn_ts = quote::quote! {
            async fn gen_rd_ids_from_try_co(
                #UrlSc: &str,
                #PayloadSc: #ident_cr_ucc,
                tbl: &str,
            ) -> #ident_rd_ids_ucc {
                #ident::try_co_h(
                    #UrlSc,
                    #ident_co_prms_ucc {
                        #PayloadSc
                    },
                    tbl
                ).await.expect("32e30b87")
            }
        };
        let gen_rd_ids_from_try_co_dflt_fn_ts = quote::quote! {
            async fn gen_rd_ids_from_try_co_dflt(
                #UrlSc: &str,
                tbl: &str,
            ) -> #ident_rd_ids_ucc {
                gen_rd_ids_from_try_co(
                    #UrlSc,
                    ident_cr_dflt(),
                    tbl
                ).await
            }
        };
        let gen_try_dlo_h_fn_ts = quote::quote! {
            async fn gen_try_dlo_h(
                #UrlSc: &str,
                #pk_fi: #pk_ft_as_pg_type_rd_ts,
                tbl: &str,
            ) -> Result<#pk_ft_as_pg_type_rd_ts, #ident_try_dlo_er_ucc> {
                #ident::try_dlo_h(
                    #UrlSc,
                    #ident_dlo_prms_ucc {
                        payload: #ident_dlo_payload_ucc {
                            #pk_fi
                        }
                    },
                    tbl
                ).await
            }
        };
        let no_rows_by_a_query_that_expected_to_return_at_least_one_row_fn_ts = quote::quote! {
            fn no_rows_by_a_query_that_expected_to_return_at_least_one_row() -> &'static str {
                "no rows returned by a query that expected to return at least one row"
            }
        };
        let gen_vec_ident_rd_from_vec_ident_rd_ids_with_vec_ident_cr_fn_ts = {
            let ts = gen_assert_eq_ts(
                &quote::quote! {rd_ids_from_try_cm.len()},
                &quote::quote! {ident_vec_cr.len()},
                &quote::quote! {"88fb286c"},
            );
            quote::quote! {
                fn gen_vec_ident_rd_from_vec_ident_rd_ids_with_vec_ident_cr(
                    rd_ids_from_try_cm: Vec<#ident_rd_ids_ucc>,
                    ident_vec_cr: Vec<#ident_cr_ucc>
                ) -> Vec<#ident_rd_ucc> {
                    let mut acc_1debe8fb = Vec::with_capacity(rd_ids_from_try_cm.len());
                    #ts
                    for (rd_ids, cr) in rd_ids_from_try_cm.into_iter().zip(ident_vec_cr) {
                        acc_1debe8fb.push(#ident_rd_ucc {
                            #pk_fi: #pk_as_pg_type_test_cases_path_ts rd_ids_to_opt_v_rd_dflt_some_one_el(
                                &rd_ids.#pk_fi
                            ),
                            #fi_rd_ids_and_cr_into_opt_v_rd_rd_ids_and_cr_ts
                        });
                    }
                    acc_1debe8fb.sort_by(#pk_sort_cmp_ts);
                    acc_1debe8fb
                }
            }
        };
        quote::quote! {
            #[cfg(test)]
            mod #ident_tests_sc {
                use super::*;
                #size_of_ts
                #[test]
                fn crud() {
                    #gen_ident_wh_pk_others_none_fn_ts
                    #gen_pg_type_wh_try_new_pk_fn_ts
                    #gen_pg_type_wh_try_new_or_pks_fn_ts
                    #gen_try_rm_order_by_pk_with_big_pgn_fn_ts
                    #gen_ident_try_ro_h_pk_fn_ts
                    #gen_check_no_rows_from_ident_try_ro_h_pk_fn_ts
                    #ident_cr_dflt_fn_ts
                    #gen_rd_ids_from_try_co_fn_ts
                    #gen_rd_ids_from_try_co_dflt_fn_ts
                    #gen_try_dlo_h_fn_ts
                    #no_rows_by_a_query_that_expected_to_return_at_least_one_row_fn_ts
                    #gen_vec_ident_rd_from_vec_ident_rd_ids_with_vec_ident_cr_fn_ts
                    #gen_rd_ids_els_ts
                    tracing_subscriber::fmt::init();
                    tokio::runtime::Builder::new_multi_thread().worker_threads(num_cpus::get()).enable_all().build().expect("38823c21").block_on(async {
                        //todo mb refactor
                        let mut #ConfigSc = #config_path_ts {
                            service_socket_address: <config_lib::ServiceSocketAddress as config_lib::TryFromStdEnvVarOk>::try_from_std_env_var_ok(config_lib::StdEnvVarOk(
                                "127.0.0.1:0".to_owned()
                            )).expect("b5b3915a").0,
                            database_url: <config_lib::DatabaseUrl as config_lib::TryFromStdEnvVarOk>::try_from_std_env_var_ok(config_lib::StdEnvVarOk(
                                "postgres://postgres:postgres@127.0.0.1:5432/postgres?connect_timeout=10".to_owned()
                            )).expect("f9c20f05").0,
                            timezone: <config_lib::ChronoTimezone as config_lib::TryFromStdEnvVarOk>::try_from_std_env_var_ok(config_lib::StdEnvVarOk(
                                "10800".to_owned()
                            )).expect("d00d8998").0,
                            tracing_level: <config_lib::TracingLevel as config_lib::TryFromStdEnvVarOk>::try_from_std_env_var_ok(config_lib::StdEnvVarOk(
                                "er".to_owned()
                            )).expect("957178c9").0,
                            src_place_type: <config_lib::SrcPlaceType as config_lib::TryFromStdEnvVarOk>::try_from_std_env_var_ok(config_lib::StdEnvVarOk(
                                "src".to_owned()
                            )).expect("bec0950e").0,
                            enable_api_git_commit_check: <config_lib::EnableApiGitCommitCheck as config_lib::TryFromStdEnvVarOk>::try_from_std_env_var_ok(config_lib::StdEnvVarOk(
                                "true".to_owned()
                            )).expect("31f02640").0,
                            maximum_size_of_http_body_in_bytes: <config_lib::MaximumSizeOfHttpBodyInBytes as config_lib::TryFromStdEnvVarOk>::try_from_std_env_var_ok(config_lib::StdEnvVarOk(
                                "1048576000".to_owned()
                            )).expect("93b2f818").0,
                            pg_pool_max_connections: <config_lib::PgPoolMaxConnections as config_lib::TryFromStdEnvVarOk>::try_from_std_env_var_ok(config_lib::StdEnvVarOk(
                                "50".to_owned()
                            )).expect("7c4e9f12").0,
                            cors_allow_origin: <config_lib::CorsAllowOrigin as config_lib::TryFromStdEnvVarOk>::try_from_std_env_var_ok(config_lib::StdEnvVarOk(
                                "http://127.0.0.1".to_owned()
                            )).expect("a1b2c3d4").0,
                        };
                        let #PgPoolSc = sqlx::postgres::PgPoolOptions::new()
                        .max_connections(50)
                        .connect(secrecy::ExposeSecret::expose_secret(app_state::GetDatabaseUrl::get_database_url(&#ConfigSc)))
                        .await.expect("e3044bb9");
                        let tcp_listener = tokio::net::TcpListener::bind(app_state::GetServiceSocketAddress::get_service_socket_address(&#ConfigSc)).await.expect("663ae29e");
                        let actual_service_socket_address = tcp_listener.local_addr().expect("f31a9d0c");
                        #ConfigSc.service_socket_address = actual_service_socket_address;
                        let #UrlSc: std::sync::Arc<str> = std::sync::Arc::from(format!("http://{actual_service_socket_address}"));
                        let tbl = #ident_dq_ts;
                        let add_tbl_postfix = |postfix|{
                            let v = format!("{tbl}_{postfix}");
                            #assert_tbl_name_len_ts
                            std::sync::Arc::<str>::from(v)
                        };
                        let tbl_init = add_tbl_postfix("init");
                        let tbl_cm = add_tbl_postfix("cm");
                        let tbl_co = add_tbl_postfix("co");
                        let tbl_test_rm_by_non_existent_pks = add_tbl_postfix("Test_rm_by_non_existent_pks");
                        let tbl_test_rm_by_eq_to_crd_pks = add_tbl_postfix("Test_rm_by_eq_to_crd_pks");
                        #(#tbl_fis_init_vec_ts)*
                        let tbl_ro = add_tbl_postfix("ro");
                        let tbl_um = add_tbl_postfix("um");
                        let tbl_uo = add_tbl_postfix("uo");
                        let tbl_dm = add_tbl_postfix("dm");
                        let tbl_dlo = add_tbl_postfix("dlo");
                        let tbl_names = [
                            &tbl_init,
                            &tbl_cm,
                            &tbl_co,
                            &tbl_test_rm_by_non_existent_pks,
                            &tbl_test_rm_by_eq_to_crd_pks,
                            #(#tbl_test_name_fis_vec_ts)*
                            &tbl_ro,
                            &tbl_um,
                            &tbl_uo,
                            &tbl_dm,
                            &tbl_dlo,
                        ];
                        let drop_all_test_tbls = async ||{
                            let _unused = futures::future::try_join_all(
                                tbl_names
                                .iter()
                                .map(|tbl_name|{
                                    let pg_pool_3b948340 = &pg_pool;
                                    async move {
                                        sqlx::query(&format!("drop table if exists {tbl_name}")).execute(pg_pool_3b948340).await
                                    }
                                })
                            )
                            .await
                            .expect("b9c1eb2e");
                        };
                        drop_all_test_tbls().await;
                        #ident::prep_extensions(&#PgPoolSc).await.expect("0633ff48");
                        //do not make it concrnt. would be pg er: "duplicate k v violates unique constraint \"pg_class_relname_nsp_index\""
                        for el_dac43b91 in tbl_names {
                            #ident::prep_pg_tbl(
                                &#PgPoolSc,
                                el_dac43b91,
                            ).await.expect("c7952247");
                        }
                        let #PgPoolForTokioSpawnSyncMoveSc = #PgPoolSc.clone();
                        let tbl_names_cloned = tbl_names.map(|el_26b304d1| std::sync::Arc::<str>::clone(el_26b304d1));
                        let (started_tx, started_rx) = tokio::sync::oneshot::channel();
                        let #undrscr_unused_ts = tokio::spawn(async move {
                            let #AppStateSc = std::sync::Arc::new(server_app_state::ServerAppState {
                                #PgPoolSc: app_state::SqlxPgPool::from(#PgPoolForTokioSpawnSyncMoveSc.clone()),
                                #ConfigSc,
                                project_git_info: &git_info::PROJECT_GIT_INFO,
                            });
                            started_tx.send(()).expect("431a6f8d");
                            axum::serve(
                                tcp_listener,
                                {
                                    let mut router = axum::Router::new()
                                        .merge(#ident::routes(std::sync::Arc::<server_app_state::ServerAppState<'_>>::clone(&app_state)));
                                    for el_ef09f2b0 in tbl_names_cloned {
                                        router = router.merge(#ident::routes_h(std::sync::Arc::<server_app_state::ServerAppState<'_>>::clone(&app_state), &el_ef09f2b0));
                                    }
                                    router.into_make_service()
                                },
                            )
                            .await
                            .expect("71c1bc30");
                        });
                        started_rx.await.expect("87003141");
                        let #SelPkSc = #import_ts NotEmptyUnqVec::try_new_by_hash(vec![
                            #ident_sel_ucc::#pk_fi_ucc_ts(
                                #pk_ft_as_pg_type_sel_ts::default(),
                            )
                        ])
                        .expect("0776170e");
                        let #IdentCrDfltSc = ident_cr_dflt();
                        #sel_dflt_all_with_max_page_size_not_empty_unq_vec_ts
                        #cmn_rd_ids_from_co_ts
                        #rd_ids_to_2_dims_vec_rd_inn_acc_fields_ts
                        const TEST_FUTURE_CONCURRENCY_D281414B: usize = 100;
                        const TEST_FUTURE_BASE_CAPACITY_7C87B2A1: usize = #fields_len_without_pk;
                        futures::StreamExt::for_each_concurrent(
                            futures::stream::iter({
                                let mut acc_9189f86e: Vec<futures::future::BoxFuture<'static, ()>> = Vec::with_capacity(
                                    TEST_FUTURE_BASE_CAPACITY_7C87B2A1
                                        .saturating_mul(16)
                                        .saturating_add(6)
                                );
                                #cm_tests_ts
                                #co_tests_ts
                                #rm_tests_ts
                                #ro_tests_ts
                                #um_tests_ts
                                #uo_tests_ts
                                #dm_tests_ts
                                #dlo_tests_ts
                                acc_9189f86e
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
    let ident_tests_ts = emit_gen_pg_tbl_tests_stage(
        &gen_pg_tbl_input_model.config,
        ProcMacro2GenPgTblTestsTs(generated_ident_tests_ts),
    )
    .into_inner();
    let cmn_ts = quote::quote! {
        #ident_prep_pg_er_ts
        #ident_cr_ts
        #ident_wh_ts
        #opt_ident_wh_ts
        #sel_ts
        #ident_rd_ts
        #ident_rd_ids_ts
        #ident_upd_ts
        #ident_upd_for_query_ts
    };
    let gend = {
        let ident_gen_pg_tbl_mod_sc = naming::prm::SelfGenPgTblModSc::from_tokens(&ident);
        let impl_and_content_ts = quote::quote! {
                #AllowClippyArbitrarySrcItemOrdering
                impl #ident {
                    #(#impl_ident_vec_ts)*
                    #[allow(clippy::single_call_fn)]
                    fn #RoutesHSc(#AppStateSc: #std_sync_arc_combination_of_app_state_logic_traits_ts, #TblSc: &str) -> axum::Router {
                        axum::Router::new().nest(
                            &format!("/{tbl}"),
                            axum::Router::new()
                            #(#op_routes_ts)*
                            .with_state(#AppStateSc)
                        )
                    }
                }
                #(#content_ts)*
                #ident_api_client_ts
                #ident_route_contract_ts
                #ident_open_api_ts
                #cmn_ts
                #generated_contract_tests_ts
                #ident_tests_ts
        };
        quote::quote! {
            #[allow(unused_qualifications)]
            #[allow(clippy::absolute_paths)]
            mod #ident_gen_pg_tbl_mod_sc {
                use super::#ident;
                #impl_and_content_ts
            }
            pub use #ident_gen_pg_tbl_mod_sc::*;
        }
    };
    emit_gen_pg_tbl_final_stage(
        &gen_pg_tbl_input_model.config,
        &ProcMacro2GenPgTblCmnTs(cmn_ts),
        ProcMacro2GenPgTblWholeTs(gend),
    )
}
