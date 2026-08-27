#[path = "domain_types/display_plus_to_tokens.rs"]
mod display_plus_to_tokens;
#[path = "domain_types/hash_map.rs"]
mod hash_map;
#[path = "domain_types/hash_map_snake_case.rs"]
mod hash_map_snake_case;
#[path = "domain_types/hash_map_upper_camel_case.rs"]
mod hash_map_upper_camel_case;
#[path = "parameter.rs"]
pub mod parameter;
#[path = "domain_types/swagger_url_path_prefix.rs"]
mod swagger_url_path_prefix;
#[path = "domain_types/swagger_url_path_self_quotes_str.rs"]
mod swagger_url_path_self_quotes_str;
#[path = "domain_types/swagger_url_path_self_quotes_str_value.rs"]
mod swagger_url_path_self_quotes_str_value;
#[path = "domain_types/swagger_url_path_self_quotes_token_stream.rs"]
mod swagger_url_path_self_quotes_token_stream;
#[path = "domain_types/swagger_url_path_self_quotes_token_stream_value.rs"]
mod swagger_url_path_self_quotes_token_stream_value;

pub use display_plus_to_tokens::DisplayPlusToTokens;
pub use hash_map::HashMap;
pub use hash_map_snake_case::HashMapSnakeCase;
pub use hash_map_upper_camel_case::HashMapUpperCamelCase;
pub use swagger_url_path_prefix::SwaggerUrlPathPrefix;
pub use swagger_url_path_self_quotes_str::SwaggerUrlPathSelfQuotesStr;
pub use swagger_url_path_self_quotes_str_value::SwaggerUrlPathSelfQuotesStrValue;
pub use swagger_url_path_self_quotes_token_stream::SwaggerUrlPathSelfQuotesTokenStream;
pub use swagger_url_path_self_quotes_token_stream_value::SwaggerUrlPathSelfQuotesTokenStreamValue;

naming_macros::generate_upper_camel_case_and_snake_case_str_and_token_stream!([
    ["primary", "key"],
    ["serde"],
    ["with", "serde"],
    ["location"],
    ["failed", "to", "get", "res", "text"],
    ["de", "res"],
    ["status", "code"],
    ["res", "text"],
    ["order", "by"],
    ["not", "unique", "primary", "key"],
    ["into", "serde", "version"],
    ["app", "state"],
    ["query", "part", "error"],
    ["serde", "json", "to", "string"],
    ["endpoint", "location"],
    ["query", "string"],
    ["binded", "query"],
    ["not", "unique", "field"],
    ["body", "bytes"],
    ["check", "body", "size"],
    ["expected", "res"],
    ["pg", "crud"],
    ["cm", "error", "variants"],
    ["co", "error", "variants"],
    ["rm", "error", "variants"],
    ["ro", "error", "variants"],
    ["um", "error", "variants"],
    ["uo", "error", "variants"],
    ["dm", "error", "variants"],
    ["dlo", "error", "variants"],
    ["common", "error", "variants"],
    ["cm", "logic"],
    ["co", "logic"],
    ["rm", "logic"],
    ["ro", "logic"],
    ["um", "logic"],
    ["uo", "logic"],
    ["dm", "logic"],
    ["dlo", "logic"],
    ["common", "logic"],
    ["row", "and", "rollback"],
    ["serde", "json"],
    ["pool", "connection"],
    ["in"],
    ["as"],
    ["where"],
    ["error"],
    ["string"],
    ["parameters"],
    ["payload"],
    ["element"],
    ["value"],
    ["req"],
    ["res"],
    ["config"],
    ["is"],
    ["to"],
    ["column"],
    ["select"],
    ["order"],
    ["by"],
    ["not"],
    ["desirable"],
    ["rollback"],
    ["fields"],
    ["commit"],
    ["begin"],
    ["query"],
    ["update"],
    ["delete"],
    ["and"],
    ["row"],
    ["pool"],
    ["reqwest"],
    ["headers"],
    ["or"],
    ["asc"],
    ["desc"],
    ["optional"],
    ["read"],
    ["read", "inner"],
    ["body"],
    ["pg"],
    ["increment"],
    ["url"],
    ["future"],
    ["end"],
    ["rows"],
    ["executor"],
    ["prefix"],
    ["id"],
    ["pagination"],
    ["std", "optional", "optional", "obj", "accumulator"],
    ["not", "unique", "id", "in", "json", "delete", "array"],
    [
        "not", "unique", "id", "in", "json", "update", "and", "delete", "arrs"
    ],
    ["all", "fields", "are", "none"],
    ["self"],
    ["create", "query", "part"],
    ["create", "query", "bind"],
    ["select", "query", "part"],
    ["column", "field"],
    ["column", "field", "for", "error", "message"],
    ["field"],
    ["create"],
    ["dotenv"],
    ["std", "env", "var", "error"],
    ["env", "var", "name"],
    ["try", "from", "std", "env", "var", "ok"],
    ["table", "name"],
    ["default", "some", "one", "element"],
    ["all", "variants", "default", "some", "one", "element"],
    ["location", "lib"],
    ["pub"],
    ["self", "create"],
    ["self", "select"],
    ["self", "read"],
    ["pg", "type"],
    ["true"],
    ["false"],
    ["update", "query", "part"],
    ["update", "query", "bind"],
    ["query", "part"],
    ["query", "bind"],
    ["between"],
    ["add", "operator"],
    ["eq"],
    ["greater", "than"],
    ["eq", "to", "encoded", "string", "representation"],
    ["find", "ranges", "within", "given", "range"],
    [
        "find", "ranges", "that", "fully", "contain", "the", "given", "range"
    ],
    ["start"],
    ["strictly", "to", "left", "of", "range"],
    ["strictly", "to", "right", "of", "range"],
    ["included", "lower", "bound"],
    ["excluded", "upper", "bound"],
    ["greater", "than", "included", "lower", "bound"],
    ["greater", "than", "excluded", "upper", "bound"],
    ["overlap", "with", "range"],
    ["adjacent", "with", "range"],
    ["range", "len"],
    ["before"],
    ["current", "date"],
    ["current", "time"],
    ["greater", "than", "current", "date"],
    ["greater", "than", "current", "time"],
    ["current", "timestamp"],
    ["greater", "than", "current", "timestamp"],
    ["self", "where"],
    ["dimension", "one", "contains", "all", "els", "of", "array"],
    ["dimension", "two", "contains", "all", "els", "of", "array"],
    [
        "dimension",
        "three",
        "contains",
        "all",
        "els",
        "of",
        "array"
    ],
    ["dimension", "four", "contains", "all", "els", "of", "array"],
    ["dimension", "one", "overlaps", "with", "array"],
    ["dimension", "two", "overlaps", "with", "array"],
    ["dimension", "three", "overlaps", "with", "array"],
    ["dimension", "four", "overlaps", "with", "array"],
    ["dimension", "one", "len", "eq"],
    ["dimension", "two", "len", "eq"],
    ["dimension", "three", "len", "eq"],
    ["dimension", "four", "len", "eq"],
    ["dimension", "one", "len", "greater", "than"],
    ["dimension", "two", "len", "greater", "than"],
    ["dimension", "three", "len", "greater", "than"],
    ["dimension", "four", "len", "greater", "than"],
    ["dimension", "one", "all", "els", "eq"],
    ["dimension", "two", "all", "els", "eq"],
    ["dimension", "three", "all", "els", "eq"],
    ["dimension", "four", "all", "els", "eq"],
    ["dimension", "one", "contains", "element", "greater", "than"],
    ["dimension", "two", "contains", "element", "greater", "than"],
    [
        "dimension",
        "three",
        "contains",
        "element",
        "greater",
        "than"
    ],
    [
        "dimension",
        "four",
        "contains",
        "element",
        "greater",
        "than"
    ],
    ["dimension", "one", "all", "els", "greater", "than"],
    ["dimension", "two", "all", "els", "greater", "than"],
    ["dimension", "three", "all", "els", "greater", "than"],
    ["dimension", "four", "all", "els", "greater", "than"],
    ["months"],
    ["days"],
    ["microseconds"],
    ["date"],
    ["time"],
    ["pg", "type", "where", "filter"],
    ["is", "primary", "key"],
    ["create", "table", "column", "query", "part"],
    ["table", "type"],
    ["mut"],
    ["boolean"],
    ["number"],
    ["vec", "of"],
    ["array", "of"],
    ["with", "id"],
    ["regex"],
    ["dimension", "one", "regex"],
    ["dimension", "two", "regex"],
    ["dimension", "three", "regex"],
    ["dimension", "four", "regex"],
    ["dimension", "one", "contains", "element", "regex"],
    ["dimension", "two", "contains", "element", "regex"],
    ["dimension", "three", "contains", "element", "regex"],
    ["dimension", "four", "contains", "element", "regex"],
    ["dimension", "one", "all", "els", "regex"],
    ["dimension", "two", "all", "els", "regex"],
    ["dimension", "three", "all", "els", "regex"],
    ["dimension", "four", "all", "els", "regex"],
    ["dimension", "one", "eq"],
    ["dimension", "two", "eq"],
    ["dimension", "three", "eq"],
    ["dimension", "four", "eq"],
    ["dimension", "one", "greater", "than"],
    ["dimension", "two", "greater", "than"],
    ["dimension", "three", "greater", "than"],
    ["dimension", "four", "greater", "than"],
    ["dimension", "one", "in"],
    ["dimension", "two", "in"],
    ["dimension", "three", "in"],
    ["dimension", "four", "in"],
    ["dimension", "one", "between"],
    ["dimension", "two", "between"],
    ["dimension", "three", "between"],
    ["dimension", "four", "between"],
    ["dimension", "one", "before"],
    ["dimension", "one", "current", "date"],
    ["dimension", "one", "greater", "than", "current", "date"],
    ["dimension", "one", "current", "timestamp"],
    [
        "dimension",
        "one",
        "greater",
        "than",
        "current",
        "timestamp"
    ],
    ["dimension", "one", "current", "time"],
    ["dimension", "one", "greater", "than", "current", "time"],
    [
        "dimension",
        "one",
        "eq",
        "to",
        "encoded",
        "string",
        "representation"
    ],
    [
        "dimension",
        "one",
        "find",
        "ranges",
        "within",
        "given",
        "range"
    ],
    [
        "dimension",
        "one",
        "find",
        "ranges",
        "that",
        "fully",
        "contain",
        "the",
        "given",
        "range"
    ],
    ["dimension", "one", "strictly", "to", "left", "of", "range"],
    ["dimension", "one", "strictly", "to", "right", "of", "range"],
    ["dimension", "one", "included", "lower", "bound"],
    ["dimension", "one", "excluded", "upper", "bound"],
    [
        "dimension",
        "one",
        "greater",
        "than",
        "included",
        "lower",
        "bound"
    ],
    [
        "dimension",
        "one",
        "greater",
        "than",
        "excluded",
        "upper",
        "bound"
    ],
    ["dimension", "one", "overlap", "with", "range"],
    ["dimension", "one", "adjacent", "with", "range"],
    ["dimension", "one", "range", "len"],
    ["dimensions"],
    ["dimensions", "ies"],
    ["len", "eq"],
    ["len", "greater", "than"],
    ["contains", "all", "els", "of", "array"],
    ["overlaps", "with", "array"],
    ["contains", "element", "regex"],
    ["all", "els", "regex"],
    ["all", "els", "eq"],
    ["contains", "element", "greater", "than"],
    ["all", "els", "greater", "than"],
    ["create", "extension", "if", "not", "exists", "uuid", "ossp"],
    ["prep", "pg"],
    ["prep", "pg", "table"],
    ["header", "content", "type", "app", "json", "not", "found"],
    ["where", "many"],
    ["no", "fields", "provided"],
    ["extra", "parameters"],
    ["generate", "select", "query", "part"],
    ["update", "query", "part", "primary", "key"],
    [
        "generate", "column", "eq", "v", "comma", "uo", "query", "part"
    ],
    ["primary", "key", "query", "part"],
    ["cols"],
    [
        "generate", "when", "column", "id", "then", "v", "um", "query", "part"
    ],
    ["contains", "null", "byte"],
    ["pg", "type", "test", "cases"],
    ["included", "start", "greater", "than", "included", "end"],
    ["included", "start", "greater", "than", "excluded", "end"],
    ["excluded", "start", "greater", "than", "included", "end"],
    ["excluded", "start", "greater", "than", "excluded", "end"],
    ["included", "end", "cannot", "be", "max"],
    ["earlier", "date", "not", "supported"],
    ["earliest", "supported", "date"],
    [
        "invalid",
        "hour",
        "or",
        "minute",
        "or",
        "second",
        "or",
        "microsecond"
    ],
    ["hour"],
    ["min"],
    ["sec"],
    ["micro"],
    ["minute"],
    ["second"],
    ["microsecond"],
    ["nanosecond", "precision", "is", "not", "supported"],
    ["date", "naive"],
    ["nanosecond"],
    ["included"],
    ["excluded"],
    ["unbounded"],
    ["normalize"],
    ["new"],
    ["try", "new"],
    ["pg", "pool"],
    ["pg", "pool", "for", "tokio", "spawn", "sync", "move"],
    ["identifier", "create", "default"],
    ["select", "primary", "key"],
    ["select", "query", "part", "pg", "type"],
    ["read", "ids"],
    ["select", "only", "ids", "query", "part"],
    ["select", "only", "updated", "ids", "query", "part"],
    ["create", "update", "delete", "are", "empty"],
    ["update", "to", "read", "ids"],
    ["self", "read", "ids", "h"],
    ["common", "read", "ids", "from", "co"],
    ["generate", "pg", "table", "primary", "key"],
    ["try", "bind"],
    ["select", "only", "updated", "ids", "query", "bind"],
    ["create", "for", "query"],
    [
        "read", "ids", "to", "optional", "v", "read", "default", "some", "one", "element"
    ],
    ["optional", "update"],
    ["select", "only", "created", "ids", "query", "part"],
    ["select", "only", "created", "ids", "query", "bind"],
    ["update", "for", "query"],
    ["update", "for", "query", "vec"],
    [
        "read", "ids", "and", "create", "into", "optional", "v", "read"
    ],
    ["default", "some", "one", "element", "max", "page", "size"],
    [
        "all", "variants", "default", "some", "one", "element", "max", "page", "size"
    ],
    ["ids", "are", "not", "unique"],
    ["pg", "type", "primary", "key"],
    ["pg", "type", "not", "primary", "key"],
    ["read", "ids", "and", "create", "into", "where", "eq"],
    ["read", "ids", "and", "create", "into", "read"],
    ["read", "ids", "and", "create", "into", "table", "type"],
    [
        "read", "inner", "into", "read", "with", "new", "or", "try", "new", "unwraped"
    ],
    [
        "read", "inner", "into", "update", "with", "new", "or", "try", "new", "unwraped"
    ],
    ["read", "ids", "into", "optional", "v", "read", "inner"],
    [
        "previous", "read", "and", "optional", "update", "into", "read"
    ],
    [
        "read", "ids", "and", "create", "into", "vec", "where", "eq", "using", "fields"
    ],
    ["eq", "operator"],
    ["pg", "type", "eq", "operator"],
    ["read", "ids", "into", "table", "type"],
    ["read", "ids", "into", "read"],
    ["read", "ids", "into", "update"],
    ["read", "into", "table", "type"],
    ["optional", "vec", "create"],
    ["read", "ids", "to2", "dimensions", "vec", "read", "inner"],
    [
        "read", "ids", "and", "create", "into", "optional", "vec", "where", "eq", "to", "field"
    ],
    [
        "create",
        "into",
        "pg",
        "type",
        "optional",
        "vec",
        "where",
        "dimension",
        "one",
        "eq"
    ],
    [
        "read", "ids", "and", "table", "type", "into", "pg", "type", "optional", "where",
        "greater", "than"
    ],
    [
        "pg", "type", "optional", "vec", "where", "greater", "than", "test"
    ],
    ["prep", "extensions"],
    ["table"],
    ["routes"],
    ["routes", "h"],
    ["from", "h"],
    ["executor", "acquire"],
    ["generate", "pg", "types", "mod"],
    ["to", "err", "string"],
    ["body", "size", "error"],
    ["max"],
    ["near", "zero"],
    ["negative", "less", "typical"],
    ["negative", "more", "typical"],
    ["positive", "less", "typical"],
    ["positive", "more", "typical"],
    ["v"],
    ["not", "uuid"]
]);
#[cfg(test)]
mod tests {
    #[test]
    fn generated_static_names_preserve_both_cases_and_tokens() {
        assert_eq!(super::PrimaryKeyUpperCamelCase.to_string(), "PrimaryKey");
        assert_eq!(super::PrimaryKeySnakeCase.to_string(), "primary_key");
        let upper_camel = super::PrimaryKeyUpperCamelCase;
        let snake = super::PrimaryKeySnakeCase;
        assert_eq!(quote::quote!(#upper_camel).to_string(), "PrimaryKey");
        assert_eq!(quote::quote!(#snake).to_string(), "primary_key");
        assert_eq!(super::HashMapUpperCamelCase.to_string(), "HashMap");
        assert_eq!(super::HashMapSnakeCase.to_string(), "hashmap");
    }

    #[test]
    fn parameterized_names_preserve_display_token_and_type_path_context() {
        let display = super::parameter::SelfPayloadUpperCamelCase::from_display(
            &constants_str::VALUE_BCB2F337,
        );
        assert_eq!(display.to_string(), "TableExamplePayload");
        let tokens =
            super::parameter::SelfPayloadSnakeCase::from_tokens(&quote::quote!(TableExample));
        assert_eq!(tokens.to_string(), "table_example_payload");
        let qualified_type: syn::Type = syn::parse_quote!(crate::model::TableExample);
        let qualified_name =
            super::parameter::SelfPayloadUpperCamelCase::from_type_last_segment(&qualified_type);
        assert_eq!(
            qualified_name.to_string(),
            "crate::model::TableExamplePayload"
        );
    }

    #[test]
    fn swagger_path_helpers_quote_snake_case_paths() {
        let name = String::from(constants_str::VALUE_DECD817E);
        let path = super::SwaggerUrlPathSelfQuotesStr::swagger_url_path_self_quotes_str(
            &name,
            super::SwaggerUrlPathPrefix::from(constants_str::SERVICE),
        );
        assert_eq!(path.as_ref(), "\"/service/table_example\"");
        let tokens =
            super::SwaggerUrlPathSelfQuotesTokenStream::swagger_url_path_self_quotes_token_stream(
                &name,
                super::SwaggerUrlPathPrefix::from(constants_str::SERVICE),
            );
        assert_eq!(
            quote::quote!(#tokens).to_string(),
            "\"/service/table_example\""
        );
    }
}
