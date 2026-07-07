# Domain type policy implementation plan
1. Add the policy document first.
   - Define the target rule: struct fields, enum variant fields, function parameters, method parameters, and return types must use types declared in this repository.
   - Define why: public and internal signatures should carry domain meaning instead of raw external or primitive types.
   - Define the migration path: wrap raw values in newtype domain structs and initialize them through `From` or `TryFrom`.
2. Build an AST-based style test skeleton.
   - Parse every Rust source file with `syn`.
   - Collect repository-declared type names from `struct`, `enum`, `union`, and trait declarations.
   - Treat generic type parameters declared on the current item as allowed.
   - Skip code under `#[cfg(test)]` and `#[cfg(feature = "test-utils")]`.
3. Add explicit exceptions before enforcing the rule.
   - Allow `#[proc_macro]`, `#[proc_macro_derive]`, and `#[proc_macro_attribute]` functions to use `proc_macro::TokenStream` in parameters and return values.
   - Allow single-field tuple newtypes such as `struct MyStruct(u32);` to wrap a raw primitive or external type.
   - Allow Rust structural syntax that is not a domain value by itself: references, lifetimes, tuples, arrays, slices, `impl Trait`, and `dyn Trait`; their nested concrete types are still checked where applicable.
   - Allow function generic parameters such as `T` when `T` is declared on the function, impl, trait, struct, or enum being checked.
   - Allow methods inside `impl SomeTrait for Type` because those signatures are constrained by the implemented trait contract; keep checking free functions, inherent methods, trait method declarations, struct fields, and enum variant fields.
4. Start with module-scoped enforcement.
   - Enable the test for a small explicit source allowlist.
   - Run the test and use its error output as the migration list.
   - Convert raw fields and signatures in that source set to domain wrappers.
5. Expand enforcement iteratively.
   - Add more source roots to the enforced set only after the previous set passes.
   - Prefer existing domain crates for wrappers when ownership is clear.
   - Put shared wrappers in an existing shared crate or a new dedicated crate only when the same concept is reused across crates.
6. Finish with workspace-wide enforcement.
   - Remove the module/source allowlist.
   - Keep only semantic exceptions from step 3.
   - Ensure the error message says to use repository domain wrapper types and initialize them with `From` or `TryFrom`.
7. Verify before completion.
   - Run `cargo fmt`.
   - Run `cargo clippy --all-targets --all-features -- -D warnings`.
   - Run `cargo test --features test-utils`.
   - Run the new domain-type policy test directly with `--nocapture` to confirm its diagnostics are readable.
