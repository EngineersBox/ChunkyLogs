#[macro_export]
macro_rules! reify{
    ($(#[$struct_attribute:meta])? $struct_vis_spec:vis struct $name:ident {
        $($(#[$field_attribute:meta])? $field_vis_spec:vis $field_name:ident: $field_type:ty,)*
    }) => {
        $(#[$struct_attribute])?
        $struct_vis_spec struct $name {
            $($field_vis_spec $field_name: $field_type,)*
        }
        impl $name {
            #[allow(dead_code)]
            pub fn get_field_attribute_map() -> std::collections::HashMap<String, String> {
                return core::convert::From::from([
                    $((
                        stringify!($field_name).to_string(),
                        stringify!($($field_attribute)?).to_string().replace(" ", "")
                    ),)*
                ]);
            }
            #[allow(dead_code)]
            pub fn get_field_attribute(field_name_prm: &str) -> Result<Option<String>, crate::compiler::errors::proc_macro_errors::StructFieldNotFoundError> {
                return match field_name_prm {
                    $(stringify!($field_name) => {
                        let attr_value: String = stringify!($($field_attribute)?).to_string();
                        return Ok(if attr_value.is_empty() { None } else { Some(attr_value) });
                    },)*
                    _ => Err(crate::compiler::errors::proc_macro_errors::StructFieldNotFoundError{
                        struct_name: stringify!($name).to_string(),
                        field_name: field_name_prm.to_string(),
                    }),
                };
            }
            #[allow(dead_code)]
            pub fn get_field_attribute_typed<T: std::str::FromStr>(field_name_prm: &str) -> Result<Option<T>, crate::compiler::errors::proc_macro_errors::TypedAttributeRetrievalError> {
                let attr: Option<String> = match $name::get_field_attribute(field_name_prm) {
                    Ok(v) => v,
                    Err(e) => return Err(crate::compiler::errors::proc_macro_errors::TypedAttributeRetrievalError{
                        message: e.field_name,
                    }),
                };
                if attr.is_none() {
                    return Ok(None);
                }
                let attr_value: String = attr.unwrap();
                return match attr_value.parse::<T>() {
                    Ok(v) => Ok(Some(v)),
                    Err(_) => Err(crate::compiler::errors::proc_macro_errors::TypedAttributeRetrievalError{
                        message: attr_value,
                    }),
                }
            }
            #[allow(dead_code)]
            pub fn get_field(&self, field_name_prm: &str) -> Result<Box<&dyn std::any::Any>, crate::compiler::errors::proc_macro_errors::StructFieldNotFoundError> {
                return match field_name_prm {
                    $(stringify!($field_name) => Ok(Box::new(&self.$field_name)),)*
                    _ => Err(crate::compiler::errors::proc_macro_errors::StructFieldNotFoundError{
                        struct_name: stringify!($name).to_string(),
                        field_name: field_name_prm.to_string(),
                    }),
                }
            }
            #[allow(dead_code)]
            pub fn get_field_typed<T: 'static>(&self, field_name_prm: &str) -> Result<Box<&T>, crate::compiler::errors::proc_macro_errors::StructFieldNotFoundError> {
                let boxed_field_value: Box<&dyn std::any::Any> = match self.get_field(field_name_prm) {
                    Ok(v) => v,
                    Err(e) => return Err(e),
                };
                return match boxed_field_value.downcast_ref() {
                    Some(v) => Ok(Box::new(v)),
                    None => Err(crate::compiler::errors::proc_macro_errors::StructFieldNotFoundError{
                        struct_name: stringify!($name).to_string(),
                        field_name: field_name_prm.to_string(),
                    })
                }
            }
        }
    }
}