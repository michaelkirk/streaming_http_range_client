use crate::{HttpClient, Reader, ReaderSource, ReqStats, Result};
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
#[async_trait(?Send)]
impl ReaderSource for LocalBytesClient {
    async fn get_byte_range(&self, mut range: Range<u64>) -> Result<Reader> {
        self.clamp_range(&mut range);
        let usize_range = range.start as usize..range.end as usize;
        let range_data = self.bytes[usize_range].to_owned();
        Ok(Box::pin(std::io::Cursor::new(range_data)))
    }

    async fn get_byte_range_from(&self, mut range: RangeFrom<u64>) -> Result<Reader> {
        self.clamp_range_from(&mut range);
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

impl LocalBytesClient {
    /// When requesting bytes _beyond_ a file's length, a web server typically ignores the extra
    /// request and just returns up until the end of the file.
    ///
    /// This method does similarly for our test client, otherwise we'll get an array OOB panic.
    fn clamp_range(&self, range: &mut Range<u64>) {
        let len = self.bytes.len() as u64;
        if range.start > len {
            debug!(
                "clamping request start to filesize {start} -> {len}",
                start = range.start
            );
            range.start = len;
        }
        if range.end > len {
            debug!(
                "clamping request end to filesize {end} -> {len}",
                end = range.end
            );
            range.end = len;
        }
    }
    fn clamp_range_from(&self, range: &mut RangeFrom<u64>) {
        let len = self.bytes.len() as u64;
        if range.start > len {
            debug!(
                "clamping request start to filesize {start} -> {len}",
                start = range.start
            );
            range.start = len;
        }
    }
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
            reader: crate::empty(),
            pos: 0,
            range: None,
            stats: ReqStats::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::io::AsyncReadExt;

    #[tokio::test]
    async fn clamping() {
        let bytes = [b'a', b'b', b'c'];
        let mut client = HttpClient::test_client(&bytes);

        client.set_range(1..3).await.unwrap();
        let mut output = vec![];
        client.read_to_end(&mut output).await.unwrap();
        assert_eq!(output, [b'b', b'c']);

        client.set_range(1..4).await.unwrap();
        let mut output = vec![];
        client.read_to_end(&mut output).await.unwrap();
        assert_eq!(output, [b'b', b'c']);
    }
}
