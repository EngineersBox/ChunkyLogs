use std::collections::HashMap;
use java_properties::read;
use std::fs::File;
use std::io::BufReader;
use crate::try_except_return_default;

use crate::configuration::exceptions;
use std::path::Path;

pub struct Config {
    pub filename: String,
    pub properties: HashMap<String, String>
}

impl Config {
    pub fn new(filename: &str) -> Config {
        Config {
            filename: String::from(filename),
            properties: Default::default()
        }
    }
    pub fn read(&mut self) {
        let path: &Path = Path::new(self.filename.as_str());
        let file: File = match File::open(&path) {
            Err(_) => panic!("{}", exceptions::FileError{filename: self.filename.clone()}),
            Ok(file) => file,
        };

        self.properties = try_except_return_default! {
            read(BufReader::new(file)),
            "Could not read properties",
            HashMap::new()
        };
    }

    pub fn get(&mut self, key: String) -> Result<String, exceptions::ConfigPropertiesError> {
        if key.is_empty() {
            return Err(exceptions::ConfigPropertiesError::InvalidConfigPropertyKeyError{
                0:exceptions::InvalidConfigPropertyKeyError{key},
            });
        }
        let value: Option<&String> = self.properties.get(key.as_str());
        if value.is_none() {
            return Err(exceptions::ConfigPropertiesError::MissingConfigPropertyError{
                0:exceptions::MissingConfigPropertyError{property: key.clone()},
            });
        }
        return Ok(String::from((*value.unwrap()).clone()));
    }
}