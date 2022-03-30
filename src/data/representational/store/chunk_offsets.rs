use std::str::FromStr;
use crate::{byte_layout, reify};

reify!{
    #[derive(Debug,Default,Clone)]
    pub struct ChunkOffsets {
        #[byte_size=4]
        pub sector_index: u32,
        #[byte_size=2]
        pub sector_offset: u16,
    }
}

byte_layout!{
    ChunkOffsets
    value [sector_index, u32, Big]
    value [sector_offset, u16, Big]
}

impl ChunkOffsets {
    fn attr_as_t<T: FromStr>(name: &str) -> Result<T, std::io::Error> {
        let attribute: Option<String> = match Self::get_field_attribute(name) {
            Ok(v) => v,
            Err(e) => return Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                e.to_string(),
            )),
        };
        if attribute.is_none() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Field has no attributes",
            ))
        }
        let attr_string: String = attribute.unwrap();
        let attribute_split: Vec<&str> = attr_string.split("=").collect::<Vec<&str>>();
        let split_val: Option<&&str> = attribute_split.get(1);
        if split_val.is_none() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Field has no attributes",
            ));
        }
        return match split_val.unwrap().parse::<T>() {
            Ok(v) => Ok(v),
            Err(e) => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Could not parse field",
            )),
        };
    }
}