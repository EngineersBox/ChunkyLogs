#[macro_export]
macro_rules! byte_layout {
    (@inner value [$target_field:ident, $byte_count:ident]) => {

    };
    (@inner pure_vec [$target_field_pure:ident, $ref_field_byte_count:ident]) => {

    };
    (@inner typed_vec [$target_field_composite:ident, $ref_field_composite_byte_count:ident, $composite_struct_name:ident]) => {

    };
    (
        $struct_name:ident
        $($alt:ident [$elem:ident$(, $args:tt)+])+
    ) => {
        #[allow(dead_code)]
        impl $struct_name {
            pub fn parse_bytes(bytes: &Vec<u8>) {
                $(byte_layout!(@inner $alt [$elem$(, $args)+]);)+
            }
        }
    }
}