use std::fmt;

pub(crate) struct FileError {
    pub(crate) filename: String
}

impl fmt::Display for FileError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "could not read file: {}", self.filename)
    }
}

pub enum ConfigPropertiesError {
    MissingConfigPropertyError(MissingConfigPropertyError),
    InvalidConfigPropertyKeyError(InvalidConfigPropertyKeyError)
}

pub struct MissingConfigPropertyError {
    pub property: String
}

impl fmt::Display for MissingConfigPropertyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "property does not exist in configuration: {}", self.property.clone())
    }
}

pub struct InvalidConfigPropertyKeyError {
    pub key: String
}

impl fmt::Display for InvalidConfigPropertyKeyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid configuration properties key: {}", self.key)
    }
}