use std::fmt;

pub struct ChunkProcessingException {
    pub message: String
}

impl fmt::Display for ChunkProcessingException {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "could not process bytes for chunk: {}", self.message)
    }
}