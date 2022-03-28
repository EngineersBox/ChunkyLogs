pub trait ToVec<T> {
    fn to_vec(self) -> Vec<T>;
}

impl ToVec<u8> for &[u8] {
    fn to_vec(self) -> Vec<u8> {
        self.to_vec()
    }
}

#[macro_export]
macro_rules! byte_layout {
    (@reader value [$target_field:ident, $byte_parser:ident$(, $endianness:ident)?], $self_accessor:ident, $tail:ident) => {
        match nom::number::complete::$byte_parser::<I,E>$((nom::number::Endianness::$endianness))?($tail) {
            Ok((t, b)) => {
                $tail = t;
                $self_accessor.$target_field = b;
            },
            Err(_) => return Err(crate::compiler::errors::proc_macro_errors::ByteLayoutParsingError{
                type_name: std::any::type_name::<Self>().to_string(),
                field_name: stringify!($target_field).to_string(),
            }),
        };
    };
    (@reader bytes_vec [$target_field_pure:ident, $ref_field_byte_count:ident], $self_accessor:ident, $tail:ident) => {
        match nom::bytes::complete::take::<_, I, E>($self_accessor.$ref_field_byte_count)($tail) {
            Ok((t, b)) => {
                $tail = t;
                $self_accessor.$target_field_pure = b.to_vec();
            },
            Err(_) => return Err(crate::compiler::errors::proc_macro_errors::ByteLayoutParsingError{
                type_name: std::any::type_name::<Self>().to_string(),
                field_name: stringify!($target_field_pure).to_string(),
            }),
        }
    };
    (@reader bytes_vec_lit [$target_field_bytes_vec_lit:ident, $field_byte_count:literal], $self_accessor:ident, $tail:ident) => {
        match nom::bytes::complete::take::<_, I, E>($field_byte_count as usize)($tail) {
            Ok((t, b)) => {
                $tail = t;
                $self_accessor.$target_field_bytes_vec_lit = b.to_vec();
            },
            Err(_) => return Err(crate::compiler::errors::proc_macro_errors::ByteLayoutParsingError{
                type_name: std::any::type_name::<Self>().to_string(),
                field_name: stringify!($target_field_pure).to_string(),
            }),
        }
    };
    (@reader bytes_vec_null_term [$target_field_bytes_vec_nt:ident], $self_accessor:ident, $tail:ident) => {
        $self_accessor.$target_field_bytes_vec_nt = Vec::new();
        loop {
            match nom::bytes::complete::take::<_, I, E>(1usize)($tail) {
                Ok((t, v)) => {
                    $tail = t;
                    let vec_v = v.to_vec();
                    if vec_v.get(0).unwrap() == &0x00u8 {
                        break;
                    }
                    $self_accessor.$target_field_bytes_vec_nt.push(*vec_v.get(0).unwrap());
                },
                Err(_) => return Err(crate::compiler::errors::proc_macro_errors::ByteLayoutParsingError{
                    type_name: std::any::type_name::<Self>().to_string(),
                    field_name: stringify!($target_field_primitive).to_string(),
                }),
            };
        }
    };
    (@reader primitive_vec [$target_field_primitive:ident, $ref_field_primitive_byte_count:ident, $primitive_byte_parser:ident$(, $endianness:ident)?], $self_accessor:ident, $tail:ident) => {
        $self_accessor.$target_field_primitive = Vec::with_capacity($self_accessor.$ref_field_primitive_byte_count as usize);
        for _ in 0..$self_accessor.$ref_field_primitive_byte_count {
            match nom::number::complete::$primitive_byte_parser::<I,E>$((nom::number::Endianness::$endianness))?($tail) {
                Ok((t, v)) => {
                    $tail = t;
                    $self_accessor.$target_field_primitive.push(v);
                },
                Err(_) => return Err(crate::compiler::errors::proc_macro_errors::ByteLayoutParsingError{
                    type_name: std::any::type_name::<Self>().to_string(),
                    field_name: stringify!($target_field_primitive).to_string(),
                }),
            };
        }
    };
    (@reader primitive_vec_lit [$target_field_primitive_lit:ident, $primitive_byte_count_lit:literal, $primitive_byte_parser:ident$(, $endianness:ident)?], $self_accessor:ident, $tail:ident) => {
        $self_accessor.$target_field_primitive_lit = Vec::with_capacity($primitive_byte_count_lit as usize);
        for _ in 0..$primitive_byte_count_lit {
            match nom::number::complete::$primitive_byte_parser::<I,E>$((nom::number::Endianness::$endianness))?($tail) {
                Ok((t, v)) => {
                    $tail = t;
                    $self_accessor.$target_field_primitive_lit.push(v);
                },
                Err(_) => return Err(crate::compiler::errors::proc_macro_errors::ByteLayoutParsingError{
                    type_name: std::any::type_name::<Self>().to_string(),
                    field_name: stringify!($target_field_primitive).to_string(),
                }),
            };
        }
    };
    (@reader composite_vec [$target_field_composite:ident, $ref_field_composite_byte_count:ident, $composite_struct_name:ident], $self_accessor:ident, $tail:ident) => {
        $self_accessor.$target_field_composite = Vec::with_capacity($self_accessor.$ref_field_composite_byte_count as usize);
        for _ in 0..$self_accessor.$ref_field_composite_byte_count {
            let mut other: $composite_struct_name = Default::default();
            match other.parse_bytes::<I,E>($tail) {
                Ok(new_tail) => {
                    $tail = new_tail;
                    $self_accessor.$target_field_composite.push(other);
                },
                Err(e) => return Err(e),
            };
        }
    };
    (@reader composite_vec_lit [$target_field_composite_lit:ident, $composite_byte_count_lit:literal, $composite_struct_name:ident], $self_accessor:ident, $tail:ident) => {
        $self_accessor.$target_field_composite_lit = Vec::with_capacity($composite_byte_count_lit as usize);
        for _ in 0..$composite_byte_count_lit {
            let mut other: $composite_struct_name = Default::default();
            match other.parse_bytes::<I,E>($tail) {
                Ok(new_tail) => {
                    $tail = new_tail;
                    $self_accessor.$target_field_composite_lit.push(other);
                },
                Err(e) => return Err(e),
            };
        }
    };
    (@reader composite [$target_field_composite:ident, $composite_struct_name:ident], $self_accessor:ident, $tail:ident) => {
        let mut other: $composite_struct_name = Default::default();
        match other.parse_bytes::<I,E>($tail) {
            Ok(new_tail) => {
                $tail = new_tail;
                $self_accessor.$target_field_composite = other;
            },
            Err(e) => return Err(e),
        };
    };
    (
        $struct_name:ident
        $($alt:ident [$elem:ident$(, $args:tt)*])+
    ) => {
        impl $struct_name {
            #[allow(dead_code)]
            pub fn parse_bytes<I, E>(&mut self, bytes: I) -> Result<I, crate::compiler::errors::proc_macro_errors::ByteLayoutParsingError>
            where
                I:  nom::InputTakeAtPosition + nom::FindSubstring<I> + nom::InputTake + crate::compiler::byte_unpack::ToVec<u8> + nom::Slice<std::ops::RangeFrom<usize>> + nom::InputIter<Item = u8> + nom::InputLength + Clone,
                E: nom::error::ParseError<I> {
                let mut tail = bytes;
                $(byte_layout!(@reader $alt [$elem$(, $args)*],self,tail);)+
                return Ok(tail);
            }
        }
    }
}