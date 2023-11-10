use crate::{empty, HttpClient, Reader, ReaderSource, ReqStats, Result};
use async_trait::async_trait;
use std::ops::{Range, RangeFrom};

/// Helpful for tests
#[async_trait]
impl ReaderSource for Vec<u8> {
    async fn get_byte_range(&self, range: Range<u64>) -> Result<Reader> {
        let usize_range = range.start as usize..range.end as usize;
        let range_data = self[usize_range].to_owned();
        Ok(Box::pin(std::io::Cursor::new(range_data)))
    }

    async fn get_byte_range_from(&self, range: RangeFrom<u64>) -> Result<Reader> {
        let usize_range = range.start as usize..;
        let range_data = self[usize_range].to_owned();
        Ok(Box::pin(std::io::Cursor::new(range_data)))
    }

    fn boxed_clone(&self) -> Box<dyn ReaderSource> {
        Box::new(self.clone())
    }
}

impl HttpClient {
    pub fn test_client(data: &[u8]) -> Self {
        Self {
            client: Box::new(data.to_vec()),
            reader: empty(),
            pos: 0,
            range: None,
            stats: ReqStats::default(),
        }
    }
}
