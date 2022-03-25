#[macro_export]
macro_rules! reflective_attributes{
    ($struct_vis_spec:vis struct $name:ident {
        $($(#[$field_attribute:meta])? $field_vis_spec:vis $field_name:ident: $field_type:ty,)*
    }) => {
        $struct_vis_spec struct $name {
            $($field_vis_spec $field_name: $field_type,)*
        }
        impl $name {
            pub fn get_field_attribute_mappings() -> std::collections::HashMap<String, String> {
                return core::convert::From::from([
                    $((
                        stringify!($field_name).to_string(),
                        stringify!($($field_attribute)?).to_string().replace(" ", "")
                    ),)*
                ]);
            }
            pub fn get_field_attribute(field_name_prm: &str) -> Result<&'static str, crate::compiler::errors::proc_macro_errors::StructFieldNotFoundError> {
                let fields: Vec<&str> = vec![$(stringify!($field_name,$($field_attribute)?)),*];
                let mut field_attr: &'static str = "@@FNF@@";

                fields.iter().for_each(|field_str| {
                    let parts : Vec<&str> = field_str.split(", ").collect();
                    if parts.len() == 2 && parts[0] == field_name_prm{
                        field_attr = parts[1];
                    }
                });
                if field_attr == "@@FNF@@" {
                    return Err(crate::compiler::errors::proc_macro_errors::StructFieldNotFoundError{
                        struct_name: stringify!($name).to_string(),
                        field_name: field_name_prm.to_string(),
                    })
                }
                return Ok(field_attr);
            }
            pub fn get_field_attribute_typed<T: std::str::FromStr>(field_name_prm: &str) -> Result<T, crate::compiler::errors::proc_macro_errors::TypedAttributeRetrievalError> {
                let attr: &'static str = match $name::get_field_attribute(field_name_prm) {
                    Ok(v) => v,
                    Err(e) => return Err(crate::compiler::errors::proc_macro_errors::TypedAttributeRetrievalError{
                        message: e.field_name,
                    }),
                };
                return match attr.parse::<T>() {
                    Ok(v) => Ok(v),
                    Err(e) => Err(crate::compiler::errors::proc_macro_errors::TypedAttributeRetrievalError{
                        message: attr.to_string(),
                    }),
                }
            }
        }
    }
}