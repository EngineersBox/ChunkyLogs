use std::fmt;

pub struct EncoderError {
    pub message: String
}

impl fmt::Display for EncoderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "could not import store: {}", self.message)
    }
}

pub struct DecoderError {
    pub message: String
}

impl fmt::Display for DecoderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "could not import store: {}", self.message)
    }
}

pub struct TranscoderError {
    pub message: String
}

impl fmt::Display for TranscoderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "could not import store: {}", self.message)
    }
}