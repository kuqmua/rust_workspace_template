mod admin_entity_id_from_i64;
pub mod admin_entity_id_try_from_i64_error;
pub mod admin_resource_text;
pub mod admin_role_record_id;
pub mod admin_socket_addr;
pub mod admin_user_record_id;
pub mod secrecy_admin_string;
pub mod std_admin_bool;
pub mod std_admin_str_ref;
pub mod std_admin_string;
#[cfg(test)]
pub mod tests_domain_types;
pub mod uuid_admin_value;

const _: fn(&str) -> Result<(), bounded_types::bounded_value_error::BoundedValueError> =
    bounded_types::bounded_string::BoundedString::<0, 0>::validate_str;
