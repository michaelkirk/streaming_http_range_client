use crate::{empty, HttpClient, Reader, ReaderSource, ReqStats, Result};
use async_trait::async_trait;
use std::fmt::{Debug, Formatter};
use std::ops::{Range, RangeFrom};

pub(crate) struct ByteFormatter<'a>(pub &'a [u8]);

impl Debug for ByteFormatter<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "0x{:02X?}", self.0)
    }
}

/// Helpful for tests
#[async_trait]
impl ReaderSource for LocalBytesClient {
    async fn get_byte_range(&self, range: Range<u64>) -> Result<Reader> {
        let usize_range = range.start as usize..range.end as usize;
        let range_data = self.bytes[usize_range].to_owned();
        Ok(Box::pin(std::io::Cursor::new(range_data)))
    }

    async fn get_byte_range_from(&self, range: RangeFrom<u64>) -> Result<Reader> {
        let usize_range = range.start as usize..;
        let range_data = self.bytes[usize_range].to_owned();
        Ok(Box::pin(std::io::Cursor::new(range_data)))
    }

    fn boxed_clone(&self) -> Box<dyn ReaderSource> {
        Box::new(self.clone())
    }
}

#[derive(Clone)]
struct LocalBytesClient {
    bytes: Vec<u8>,
}
impl Debug for LocalBytesClient {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LocalBytesClient")
            .field("bytes", &ByteFormatter(&self.bytes))
            .finish()
    }
}

impl HttpClient {
    pub fn test_client(data: &[u8]) -> Self {
        Self {
            client: Box::new(LocalBytesClient {
                bytes: data.to_vec(),
            }),
            reader: empty(),
            pos: 0,
            range: None,
            stats: ReqStats::default(),
        }
    }
}
