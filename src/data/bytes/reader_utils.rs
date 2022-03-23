use std::fs::File;
use std::io::{BufReader, IoSliceMut, Read};
use crate::data::bytes::exceptions::reader_utils_exceptions;

pub fn read_bytes_handled(buf: &mut BufReader<&File>, target: &mut Vec<IoSliceMut>, target_length: usize) -> Result<usize, reader_utils_exceptions::HandledBufferReadError> {
    return match buf.read_vectored(target) {
        Ok(n) => {
            if n != target_length {
                return Err(reader_utils_exceptions::HandledBufferReadError{
                    message: format!(
                        "Expected to read {} bytes, got {} bytes",
                        target_length,
                        n,
                    ),
                });
            }
            return Ok(n);
        }
        Err(e) => Err(reader_utils_exceptions::HandledBufferReadError{
            message: e.to_string(),
        }),
    };
}