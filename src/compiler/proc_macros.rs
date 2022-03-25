#[macro_export]
macro_rules! reflective_attributes {
    ($struct_vis_spec:vis struct $name:ident {
        $($(#[$field_attribute:meta])? $field_vis_spec:vis $field_name:ident: $field_type:ty,)*
    }) => {
        $struct_vis_spec struct $name {
            $($field_vis_spec $field_name: $field_type,)*
        }

        impl $name {
            fn get_field_attribute(field_name_prm : &str) -> Result<&'static str, crate::compiler::errors::proc_macro_errors::StructFieldNotFoundError> {
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
        }
    }
}