#![allow(
    clippy::arbitrary_source_item_ordering,
    reason = "status conversion implementations remain grouped before test-only modules"
)]
// The owner module retains lint-sensitive semantics from the original implementation.
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(
    Debug,
    strum_macros::Display,
    PartialEq,
    Eq,
    Clone,
    Copy,
    Hash,
    naming_macros::AsRefStrEnumWithUnitFieldsToUpperCamelCaseStr,
    naming_macros::AsRefStrEnumWithUnitFieldsToSnakeCaseStr,
    optimal_memory_layout::OptimalMemoryLayout,
)]
pub enum StatusCode {
    Continue100,
    SwitchingProtocols101,
    Processing102,
    Ok200,
    Created201,
    Accepted202,
    NonAuthoritativeInformation203,
    NoContent204,
    ResetContent205,
    PartialContent206,
    MultiStatus207,
    AlreadyReported208,
    ImUsed226,
    MultipleChoices300,
    MovedPermanently301,
    Found302,
    SeeOther303,
    NotModified304,
    UseProxy305,
    TemporaryRedirect307,
    PermanentRedirect308,
    BadReq400,
    Unauthorized401,
    PaymentRequired402,
    Forbidden403,
    NotFound404,
    MethodNotAllowed405,
    NotAcceptable406,
    ProxyAuthenticationRequired407,
    ReqTimeout408,
    Conflict409,
    Gone410,
    LengthRequired411,
    PreconditionFailed412,
    PayloadTooLarge413,
    UriTooLong414,
    UnsupportedMediaType415,
    RangeNotSatisfiable416,
    ExpectationFailed417,
    ImATeapot418,
    MisdirectedReq421,
    UnprocessableEntity422,
    Locked423,
    FailedDependency424,
    UpgradeRequired426,
    PreconditionRequired428,
    TooManyReqs429,
    ReqHeaderFieldsTooLarge431,
    UnavailableForLegalReasons451,
    InternalServerError500,
    NotImplemented501,
    BadGateway502,
    ServiceUnavailable503,
    GatewayTimeout504,
    HttpVersionNotSupported505,
    VariantAlsoNegotiates506,
    InsufficientStorage507,
    LoopDetected508,
    NotExtended510,
    NetworkAuthenticationRequired511,
}
impl StatusCode {
    #[must_use]
    pub fn to_http_status_code_token_stream(
        &self,
    ) -> crate::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream {
        let ts = match *self {
            Self::Continue100 => quote::quote! {CONTINUE},
            Self::SwitchingProtocols101 => quote::quote! {SWITCHING_PROTOCOLS},
            Self::Processing102 => quote::quote! {PROCESSING},
            Self::Ok200 => quote::quote! {OK},
            Self::Created201 => quote::quote! {CREATED},
            Self::Accepted202 => quote::quote! {ACCEPTED},
            Self::NonAuthoritativeInformation203 => quote::quote! {NON_AUTHORITATIVE_INFORMATION},
            Self::NoContent204 => quote::quote! {NO_CONTENT},
            Self::ResetContent205 => quote::quote! {RESET_CONTENT},
            Self::PartialContent206 => quote::quote! {PARTIAL_CONTENT},
            Self::MultiStatus207 => quote::quote! {MULTI_STATUS},
            Self::AlreadyReported208 => quote::quote! {ALREADY_REPORTED},
            Self::ImUsed226 => quote::quote! {IM_USED},
            Self::MultipleChoices300 => quote::quote! {MULTIPLE_CHOICES},
            Self::MovedPermanently301 => quote::quote! {MOVED_PERMANENTLY},
            Self::Found302 => quote::quote! {FOUND},
            Self::SeeOther303 => quote::quote! {SEE_OTHER},
            Self::NotModified304 => quote::quote! {NOT_MODIFIED},
            Self::UseProxy305 => quote::quote! {USE_PROXY},
            Self::TemporaryRedirect307 => quote::quote! {TEMPORARY_REDIRECT},
            Self::PermanentRedirect308 => quote::quote! {PERMANENT_REDIRECT},
            Self::BadReq400 => quote::quote! {BAD_REQUEST},
            Self::Unauthorized401 => quote::quote! {UNAUTHORIZED},
            Self::PaymentRequired402 => quote::quote! {PAYMENT_REQUIRED},
            Self::Forbidden403 => quote::quote! {FORBIDDEN},
            Self::NotFound404 => quote::quote! {NOT_FOUND},
            Self::MethodNotAllowed405 => quote::quote! {METHOD_NOT_ALLOWED},
            Self::NotAcceptable406 => quote::quote! {NOT_ACCEPTABLE},
            Self::ProxyAuthenticationRequired407 => quote::quote! {PROXY_AUTHENTICATION_REQUIRED},
            Self::ReqTimeout408 => quote::quote! {REQUEST_TIMEOUT},
            Self::Conflict409 => quote::quote! {CONFLICT},
            Self::Gone410 => quote::quote! {GONE},
            Self::LengthRequired411 => quote::quote! {LENGTH_REQUIRED},
            Self::PreconditionFailed412 => quote::quote! {PRECONDITION_FAILED},
            Self::PayloadTooLarge413 => quote::quote! {PAYLOAD_TOO_LARGE},
            Self::UriTooLong414 => quote::quote! {URI_TOO_LONG},
            Self::UnsupportedMediaType415 => quote::quote! {UNSUPPORTED_MEDIA_TYPE},
            Self::RangeNotSatisfiable416 => quote::quote! {RANGE_NOT_SATISFIABLE},
            Self::ExpectationFailed417 => quote::quote! {EXPECTATION_FAILED},
            Self::ImATeapot418 => quote::quote! {IM_A_TEAPOT},
            Self::MisdirectedReq421 => quote::quote! {MISDIRECTED_REQUEST},
            Self::UnprocessableEntity422 => quote::quote! {UNPROCESSABLE_ENTITY},
            Self::Locked423 => quote::quote! {LOCKED},
            Self::FailedDependency424 => quote::quote! {FAILED_DEPENDENCY},
            Self::UpgradeRequired426 => quote::quote! {UPGRADE_REQUIRED},
            Self::PreconditionRequired428 => quote::quote! {PRECONDITION_REQUIRED},
            Self::TooManyReqs429 => quote::quote! {TOO_MANY_REQUESTS},
            Self::ReqHeaderFieldsTooLarge431 => quote::quote! {REQUEST_HEADER_FIELDS_TOO_LARGE},
            Self::UnavailableForLegalReasons451 => quote::quote! {UNAVAILABLE_FOR_LEGAL_REASONS},
            Self::InternalServerError500 => quote::quote! {INTERNAL_SERVER_ERROR},
            Self::NotImplemented501 => quote::quote! {NOT_IMPLEMENTED},
            Self::BadGateway502 => quote::quote! {BAD_GATEWAY},
            Self::ServiceUnavailable503 => quote::quote! {SERVICE_UNAVAILABLE},
            Self::GatewayTimeout504 => quote::quote! {GATEWAY_TIMEOUT},
            Self::HttpVersionNotSupported505 => quote::quote! {HTTP_VERSION_NOT_SUPPORTED},
            Self::VariantAlsoNegotiates506 => quote::quote! {VARIANT_ALSO_NEGOTIATES},
            Self::InsufficientStorage507 => quote::quote! {INSUFFICIENT_STORAGE},
            Self::LoopDetected508 => quote::quote! {LOOP_DETECTED},
            Self::NotExtended510 => quote::quote! {NOT_EXTENDED},
            Self::NetworkAuthenticationRequired511 => {
                quote::quote! {NETWORK_AUTHENTICATION_REQUIRED}
            }
        };
        crate::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream::from(
            quote::quote! {http::StatusCode::#ts},
        )
    }
    #[must_use]
    pub fn to_proc_macro_attr_view_token_stream(
        &self,
    ) -> crate::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream {
        match format!("#[{self}]").parse::<proc_macro2::TokenStream>() {
            Ok(v) => {
                crate::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream::from(v)
            }
            Err(error) => {
                let message = error.to_string();
                crate::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream::from(
                    quote::quote! {compile_error!(#message);},
                )
            }
        }
    }
    #[must_use]
    pub fn to_status_code_description_token_stream(
        &self,
    ) -> crate::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream {
        crate::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream::from(
            match *self {
                Self::Continue100 => quote::quote! {"continue"},
                Self::SwitchingProtocols101 => quote::quote! {"switching protocols"},
                Self::Processing102 => quote::quote! {"processing"},
                Self::Ok200 => quote::quote! {"ok"},
                Self::Created201 => quote::quote! {"created"},
                Self::Accepted202 => quote::quote! {"accepted"},
                Self::NonAuthoritativeInformation203 => {
                    quote::quote! {"non authoritative information"}
                }
                Self::NoContent204 => quote::quote! {"no content"},
                Self::ResetContent205 => quote::quote! {"reset content"},
                Self::PartialContent206 => quote::quote! {"partial content"},
                Self::MultiStatus207 => quote::quote! {"multi status"},
                Self::AlreadyReported208 => quote::quote! {"already reported"},
                Self::ImUsed226 => quote::quote! {"im used"},
                Self::MultipleChoices300 => quote::quote! {"multiple choices"},
                Self::MovedPermanently301 => quote::quote! {"moved permanently"},
                Self::Found302 => quote::quote! {"found"},
                Self::SeeOther303 => quote::quote! {"see other"},
                Self::NotModified304 => quote::quote! {"not modified"},
                Self::UseProxy305 => quote::quote! {"use proxy"},
                Self::TemporaryRedirect307 => quote::quote! {"temporary redirect"},
                Self::PermanentRedirect308 => quote::quote! {"permanent redirect"},
                Self::BadReq400 => quote::quote! {"bad req"},
                Self::Unauthorized401 => quote::quote! {"unauthorized"},
                Self::PaymentRequired402 => quote::quote! {"payment required"},
                Self::Forbidden403 => quote::quote! {"forbidden"},
                Self::NotFound404 => quote::quote! {"not found"},
                Self::MethodNotAllowed405 => quote::quote! {"method not allowed"},
                Self::NotAcceptable406 => quote::quote! {"not acceptable"},
                Self::ProxyAuthenticationRequired407 => {
                    quote::quote! {"proxy authentication required"}
                }
                Self::ReqTimeout408 => quote::quote! {"req timeout"},
                Self::Conflict409 => quote::quote! {"conflict"},
                Self::Gone410 => quote::quote! {"gone"},
                Self::LengthRequired411 => quote::quote! {"len required"},
                Self::PreconditionFailed412 => quote::quote! {"precondition failed"},
                Self::PayloadTooLarge413 => quote::quote! {"payload too large"},
                Self::UriTooLong414 => quote::quote! {"uri too long"},
                Self::UnsupportedMediaType415 => quote::quote! {"unsupported media type"},
                Self::RangeNotSatisfiable416 => quote::quote! {"range not satisfiable"},
                Self::ExpectationFailed417 => quote::quote! {"expectation failed"},
                Self::ImATeapot418 => quote::quote! {"im a teapot"},
                Self::MisdirectedReq421 => quote::quote! {"misdirected req"},
                Self::UnprocessableEntity422 => quote::quote! {"unprocessable entity"},
                Self::Locked423 => quote::quote! {"locked"},
                Self::FailedDependency424 => quote::quote! {"failed dependency"},
                Self::UpgradeRequired426 => quote::quote! {"upgrade required"},
                Self::PreconditionRequired428 => quote::quote! {"precondition required"},
                Self::TooManyReqs429 => quote::quote! {"too many reqs"},
                Self::ReqHeaderFieldsTooLarge431 => {
                    quote::quote! {"req header fields too large"}
                }
                Self::UnavailableForLegalReasons451 => {
                    quote::quote! {"unavailable for legal reasons"}
                }
                Self::InternalServerError500 => quote::quote! {"internal server error"},
                Self::NotImplemented501 => quote::quote! {"not implemented"},
                Self::BadGateway502 => quote::quote! {"bad gateway"},
                Self::ServiceUnavailable503 => quote::quote! {"service unavailable"},
                Self::GatewayTimeout504 => quote::quote! {"gateway timeout"},
                Self::HttpVersionNotSupported505 => {
                    quote::quote! {"http version not supported"}
                }
                Self::VariantAlsoNegotiates506 => quote::quote! {"variant also negotiates"},
                Self::InsufficientStorage507 => quote::quote! {"insufficient storage"},
                Self::LoopDetected508 => quote::quote! {"loop detected"},
                Self::NotExtended510 => quote::quote! {"not extended"},
                Self::NetworkAuthenticationRequired511 => {
                    quote::quote! {"network authentication required"}
                }
            },
        )
    }
    #[must_use]
    pub fn to_status_code_token_stream(
        &self,
    ) -> crate::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream {
        crate::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream::from(
            match *self {
                Self::Continue100 => quote::quote! {100},
                Self::SwitchingProtocols101 => quote::quote! {101},
                Self::Processing102 => quote::quote! {102},
                Self::Ok200 => quote::quote! {200},
                Self::Created201 => quote::quote! {201},
                Self::Accepted202 => quote::quote! {202},
                Self::NonAuthoritativeInformation203 => quote::quote! {203},
                Self::NoContent204 => quote::quote! {204},
                Self::ResetContent205 => quote::quote! {205},
                Self::PartialContent206 => quote::quote! {206},
                Self::MultiStatus207 => quote::quote! {207},
                Self::AlreadyReported208 => quote::quote! {208},
                Self::ImUsed226 => quote::quote! {226},
                Self::MultipleChoices300 => quote::quote! {300},
                Self::MovedPermanently301 => quote::quote! {301},
                Self::Found302 => quote::quote! {302},
                Self::SeeOther303 => quote::quote! {303},
                Self::NotModified304 => quote::quote! {304},
                Self::UseProxy305 => quote::quote! {305},
                Self::TemporaryRedirect307 => quote::quote! {307},
                Self::PermanentRedirect308 => quote::quote! {308},
                Self::BadReq400 => quote::quote! {400},
                Self::Unauthorized401 => quote::quote! {401},
                Self::PaymentRequired402 => quote::quote! {402},
                Self::Forbidden403 => quote::quote! {403},
                Self::NotFound404 => quote::quote! {404},
                Self::MethodNotAllowed405 => quote::quote! {405},
                Self::NotAcceptable406 => quote::quote! {406},
                Self::ProxyAuthenticationRequired407 => quote::quote! {407},
                Self::ReqTimeout408 => quote::quote! {408},
                Self::Conflict409 => quote::quote! {409},
                Self::Gone410 => quote::quote! {410},
                Self::LengthRequired411 => quote::quote! {411},
                Self::PreconditionFailed412 => quote::quote! {412},
                Self::PayloadTooLarge413 => quote::quote! {413},
                Self::UriTooLong414 => quote::quote! {414},
                Self::UnsupportedMediaType415 => quote::quote! {415},
                Self::RangeNotSatisfiable416 => quote::quote! {416},
                Self::ExpectationFailed417 => quote::quote! {417},
                Self::ImATeapot418 => quote::quote! {418},
                Self::MisdirectedReq421 => quote::quote! {421},
                Self::UnprocessableEntity422 => quote::quote! {422},
                Self::Locked423 => quote::quote! {423},
                Self::FailedDependency424 => quote::quote! {424},
                Self::UpgradeRequired426 => quote::quote! {426},
                Self::PreconditionRequired428 => quote::quote! {428},
                Self::TooManyReqs429 => quote::quote! {429},
                Self::ReqHeaderFieldsTooLarge431 => quote::quote! {431},
                Self::UnavailableForLegalReasons451 => quote::quote! {451},
                Self::InternalServerError500 => quote::quote! {500},
                Self::NotImplemented501 => quote::quote! {501},
                Self::BadGateway502 => quote::quote! {502},
                Self::ServiceUnavailable503 => quote::quote! {503},
                Self::GatewayTimeout504 => quote::quote! {504},
                Self::HttpVersionNotSupported505 => quote::quote! {505},
                Self::VariantAlsoNegotiates506 => quote::quote! {506},
                Self::InsufficientStorage507 => quote::quote! {507},
                Self::LoopDetected508 => quote::quote! {508},
                Self::NotExtended510 => quote::quote! {510},
                Self::NetworkAuthenticationRequired511 => quote::quote! {511},
            },
        )
    }
}
impl TryFrom<&String> for StatusCode {
    type Error = ();
    fn try_from(v: &String) -> Result<Self, Self::Error> {
        if v == constants_str::CONTINUE_100 {
            Ok(Self::Continue100)
        } else if v == constants_str::SWITCHING_PROTOCOLS_101 {
            Ok(Self::SwitchingProtocols101)
        } else if v == constants_str::PROCESSING_102 {
            Ok(Self::Processing102)
        } else if v == constants_str::VALUE_200_OK {
            Ok(Self::Ok200)
        } else if v == constants_str::CREATED_201 {
            Ok(Self::Created201)
        } else if v == constants_str::ACCEPTED_202 {
            Ok(Self::Accepted202)
        } else if v == constants_str::NON_AUTHORITATIVE_INFORMATION_203 {
            Ok(Self::NonAuthoritativeInformation203)
        } else if v == constants_str::NO_CNT_204 {
            Ok(Self::NoContent204)
        } else if v == constants_str::RESET_CNT_205 {
            Ok(Self::ResetContent205)
        } else if v == constants_str::PARTIAL_CNT_206 {
            Ok(Self::PartialContent206)
        } else if v == constants_str::MULTI_STATUS_207 {
            Ok(Self::MultiStatus207)
        } else if v == constants_str::ALREADY_REPORTED_208 {
            Ok(Self::AlreadyReported208)
        } else if v == constants_str::IM_USED_226 {
            Ok(Self::ImUsed226)
        } else if v == constants_str::MULTIPLE_CHOICES_300 {
            Ok(Self::MultipleChoices300)
        } else if v == constants_str::MOVED_PERMANENTLY_301 {
            Ok(Self::MovedPermanently301)
        } else if v == constants_str::FOUND_302 {
            Ok(Self::Found302)
        } else if v == constants_str::SEE_OTHER_303 {
            Ok(Self::SeeOther303)
        } else if v == constants_str::NOT_MODIFIED_304 {
            Ok(Self::NotModified304)
        } else if v == constants_str::USE_PROXY_305 {
            Ok(Self::UseProxy305)
        } else if v == constants_str::TEMPORARY_REDIRECT_307 {
            Ok(Self::TemporaryRedirect307)
        } else if v == constants_str::PERMANENT_REDIRECT_308 {
            Ok(Self::PermanentRedirect308)
        } else if v == constants_str::BAD_REQ_400 {
            Ok(Self::BadReq400)
        } else if v == constants_str::UNAUTHORIZED_401 {
            Ok(Self::Unauthorized401)
        } else if v == constants_str::PAYMENT_REQUIRED_402 {
            Ok(Self::PaymentRequired402)
        } else if v == constants_str::FORBIDDEN_403 {
            Ok(Self::Forbidden403)
        } else if v == constants_str::NOT_FOUND_404 {
            Ok(Self::NotFound404)
        } else if v == constants_str::METHOD_NOT_ALLOWED_405 {
            Ok(Self::MethodNotAllowed405)
        } else if v == constants_str::NOT_ACCEPTABLE_406 {
            Ok(Self::NotAcceptable406)
        } else if v == constants_str::PROXY_AUTHENTICATION_REQUIRED_407 {
            Ok(Self::ProxyAuthenticationRequired407)
        } else if v == constants_str::REQ_TIMEOUT_408 {
            Ok(Self::ReqTimeout408)
        } else if v == constants_str::CONFLICT_409 {
            Ok(Self::Conflict409)
        } else if v == constants_str::GONE_410 {
            Ok(Self::Gone410)
        } else if v == constants_str::LENGTH_REQUIRED_411 {
            Ok(Self::LengthRequired411)
        } else if v == constants_str::PRECONDITION_FAILED_412 {
            Ok(Self::PreconditionFailed412)
        } else if v == constants_str::PAYLOAD_TOO_LARGE_413 {
            Ok(Self::PayloadTooLarge413)
        } else if v == constants_str::URI_TOO_LONG_414 {
            Ok(Self::UriTooLong414)
        } else if v == constants_str::UNSUPPORTED_MEDIA_TYPE_415 {
            Ok(Self::UnsupportedMediaType415)
        } else if v == constants_str::RANGE_NOT_SATISFIABLE_416 {
            Ok(Self::RangeNotSatisfiable416)
        } else if v == constants_str::EXPECTATION_FAILED_417 {
            Ok(Self::ExpectationFailed417)
        } else if v == constants_str::IM_A_TEAPOT_418 {
            Ok(Self::ImATeapot418)
        } else if v == constants_str::MISDIRECTED_REQ_421 {
            Ok(Self::MisdirectedReq421)
        } else if v == constants_str::UNPROCESSABLE_ENTITY_422 {
            Ok(Self::UnprocessableEntity422)
        } else if v == constants_str::LOCKED_423 {
            Ok(Self::Locked423)
        } else if v == constants_str::FAILED_DEPENDENCY_424 {
            Ok(Self::FailedDependency424)
        } else if v == constants_str::UPGRADE_REQUIRED_426 {
            Ok(Self::UpgradeRequired426)
        } else if v == constants_str::PRECONDITION_REQUIRED_428 {
            Ok(Self::PreconditionRequired428)
        } else if v == constants_str::TOO_MANY_REQS_429 {
            Ok(Self::TooManyReqs429)
        } else if v == constants_str::REQ_HEADER_FIELDS_TOO_LARGE_431 {
            Ok(Self::ReqHeaderFieldsTooLarge431)
        } else if v == constants_str::UNAVAILABLE_FOR_LEGAL_REASONS_451 {
            Ok(Self::UnavailableForLegalReasons451)
        } else if v == constants_str::INTERNAL_SERVER_ERROR_500 {
            Ok(Self::InternalServerError500)
        } else if v == constants_str::NOT_IMPLEMENTED_501 {
            Ok(Self::NotImplemented501)
        } else if v == constants_str::BAD_GATEWAY_502 {
            Ok(Self::BadGateway502)
        } else if v == constants_str::SERVICE_UNAVAILABLE_503 {
            Ok(Self::ServiceUnavailable503)
        } else if v == constants_str::GATEWAY_TIMEOUT_504 {
            Ok(Self::GatewayTimeout504)
        } else if v == constants_str::HTTP_VERSION_NOT_SUPPORTED_505 {
            Ok(Self::HttpVersionNotSupported505)
        } else if v == constants_str::VARIANT_ALSO_NEGOTIATES_506 {
            Ok(Self::VariantAlsoNegotiates506)
        } else if v == constants_str::INSUFFICIENT_STORAGE_507 {
            Ok(Self::InsufficientStorage507)
        } else if v == constants_str::LOOP_DETECTED_508 {
            Ok(Self::LoopDetected508)
        } else if v == constants_str::NOT_EXTENDED_510 {
            Ok(Self::NotExtended510)
        } else if v == constants_str::NETWORK_AUTHENTICATION_REQUIRED_511 {
            Ok(Self::NetworkAuthenticationRequired511)
        } else {
            Err(())
        }
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn status_code_token_views_are_consistent() {
        let cases = [
            (
                super::StatusCode::Continue100,
                constants_str::VALUE_100,
                constants_str::VALUE_EC8654E0,
                constants_str::VALUE_5638DD6B,
            ),
            (
                super::StatusCode::Ok200,
                constants_str::VALUE_200,
                constants_str::VALUE_54DA51BC,
                constants_str::VALUE_C48B5B1A,
            ),
            (
                super::StatusCode::PermanentRedirect308,
                constants_str::VALUE_48A1706E,
                constants_str::VALUE_E02F9F2F,
                constants_str::VALUE_DE99DE17,
            ),
            (
                super::StatusCode::ImATeapot418,
                constants_str::VALUE_4C8D5B6C,
                constants_str::VALUE_53E089DB,
                constants_str::VALUE_68F422DB,
            ),
            (
                super::StatusCode::NetworkAuthenticationRequired511,
                constants_str::VALUE_2C69BC9B,
                constants_str::VALUE_4A158E80,
                constants_str::VALUE_79DFA927,
            ),
        ];
        cases
            .into_iter()
            .try_for_each(|(status, code, http, description)| {
                assert_eq!(
                    status.to_status_code_token_stream().as_ref().to_string(),
                    code
                );
                assert_eq!(
                    status
                        .to_http_status_code_token_stream()
                        .as_ref()
                        .to_string(),
                    http
                );
                assert_eq!(
                    status
                        .to_status_code_description_token_stream()
                        .as_ref()
                        .to_string(),
                    description
                );
                assert_eq!(
                    status
                        .to_proc_macro_attr_view_token_stream()
                        .as_ref()
                        .to_string(),
                    format!("# [{status}]")
                );
                Ok::<(), ()>(())
            })
            .expect("1f309f5c status_code_token_views_are_consistent invariant must hold");
    }

    #[test]
    fn status_code_parsing_accepts_known_values_and_rejects_unknown() {
        assert_eq!(
            super::StatusCode::try_from(&String::from(constants_str::VALUE_200_OK)),
            Ok(super::StatusCode::Ok200)
        );
        assert_eq!(
            super::StatusCode::try_from(&String::from(
                constants_str::NETWORK_AUTHENTICATION_REQUIRED_511
            )),
            Ok(super::StatusCode::NetworkAuthenticationRequired511)
        );
        assert_eq!(
            super::StatusCode::try_from(&String::from("unknown_status")),
            Err(())
        );
    }

    #[test]
    fn attribute_selection_requires_exactly_one_supported_status() {
        let one: syn::Variant = syn::parse_quote! {
            #[serde(rename = "ignored")]
            #[not_found_404]
            Variant
        };
        assert_eq!(
            crate::only_one::only_one(crate::syn_variant_ref::SynVariantRef::from(&one)),
            Ok(super::StatusCode::NotFound404)
        );
        let missing: syn::Variant = syn::parse_quote! {
            #[unknown]
            Variant
        };
        assert_eq!(
            crate::only_one::only_one(crate::syn_variant_ref::SynVariantRef::from(&missing)),
            Err(crate::only_one_status_code_error::OnlyOneStatusCodeError::NotFound)
        );
        let multiple: syn::Variant = syn::parse_quote! {
            #[not_found_404]
            #[internal_server_error_500]
            Variant
        };
        assert_eq!(
            crate::only_one::only_one(crate::syn_variant_ref::SynVariantRef::from(&multiple)),
            Err(crate::only_one_status_code_error::OnlyOneStatusCodeError::MoreThanOne)
        );
    }
}
