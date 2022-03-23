use std::fs::File;
use std::io;
use std::io::{BufReader, Read};
use std::path::Path;
use crate::{Byte, Chunk, LogGroup};
use crate::data::store::exceptions::store_exceptions;

enum LogStoreImportState {
    LOADED,
    FRESH,
}

const LOG_STORE_LENGTH_OFFSET: usize = 0;
const LOG_STORE_NEWEST_OFFSET_OFFSET: usize = LOG_STORE_LENGTH_OFFSET + 8;
const LOG_STORE_DATA_OFFSET: usize = LOG_STORE_NEWEST_OFFSET_OFFSET + 8;

pub struct LogStore {
    filepath: String,
    state: LogStoreImportState,
    pub length: u64,
    pub current_chunk_offset: u64,
    pub current_chunk: Chunk,
    pub current_log_group: LogGroup,
}

impl LogStore {
    pub fn with_filepath(&self, filepath: &str) -> LogStore {
        let mut store: LogStore = LogStore{
            filepath: filepath.parse().unwrap(),
            state: LogStoreImportState::FRESH,
            length: 0,
            current_chunk_offset: 0,
            current_chunk: Chunk::new(),
            current_log_group: LogGroup::new(),
        };

        return store;
    }
    pub fn import_latest(&mut self) -> Result<(), store_exceptions::StoreImportError> {
        let path: &Path = Path::new(&self.filepath);
        if !path.exists() {
            self.state = LogStoreImportState::LOADED;
            return Ok(());
        }
        let file_result: io::Result<File> = File::open(path);
        if file_result.is_err() {
            return Err(store_exceptions::StoreImportError{
                message: format!(
                    "Unable to open file: {}",
                    self.filepath
                ),
            });
        }
        return match self.convert_bytes_to_chunks(&file_result.unwrap()) {
            Ok(_) => {
                self.state = LogStoreImportState::LOADED;
                return Ok(());
            },
            Err(e) => Err(store_exceptions::StoreImportError{
                message: e.message,
            }),
        };
    }
    fn convert_bytes_to_chunks(&mut self, file: &File) -> Result<(), store_exceptions::StoreConvertError> {
        let mut buf: BufReader<&File> = BufReader::new(file);
        let mut header: [Byte; LOG_STORE_DATA_OFFSET] = [0; LOG_STORE_DATA_OFFSET];
        // Read header
        match buf.read(&mut header) {
            Ok(n) => {
                if n != LOG_STORE_DATA_OFFSET {
                    return Err(store_exceptions::StoreConvertError{
                        message: format!(
                            "Expected to read {} bytes, got {} bytes",
                            LOG_STORE_DATA_OFFSET,
                            n,
                        ),
                    })
                }
            },
            Err(e) => return Err(store_exceptions::StoreConvertError{
                message: e.to_string(),
            }),
        };

        // Process header
        let mut length_bytes: [Byte; 8] = [0; 8];
        length_bytes.copy_from_slice(&header[LOG_STORE_LENGTH_OFFSET..LOG_STORE_NEWEST_OFFSET_OFFSET]);
        self.length = u64::from_be_bytes(length_bytes);

        let mut current_offset_bytes: [Byte; 8] = [0; 8];
        current_offset_bytes.copy_from_slice(&header[LOG_STORE_NEWEST_OFFSET_OFFSET..LOG_STORE_DATA_OFFSET]);
        self.current_chunk_offset = u64::from_be_bytes(current_offset_bytes);

        // Read current chunk
        match buf.seek_relative(self.current_chunk_offset as i64) {
            Ok(_) => {}
            Err(e) => return Err(store_exceptions::StoreConvertError{
                message: e.to_string(),
            }),
        };
        match Chunk::from_bytes_buffer(&mut buf) {
            Ok(chunk) => self.current_chunk = chunk,
            Err(e) => return Err(store_exceptions::StoreConvertError{
                message: e.to_string(),
            }),
        };

        return Ok(());
    }
}