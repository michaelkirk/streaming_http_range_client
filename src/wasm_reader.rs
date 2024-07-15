use super::asyncio;
use asyncio::AsyncRead;
use futures_util::Stream;
use std::pin::Pin;
use std::task::{Context, Poll};

/// reqwest::Request::bytes_stream returns an AsyncRead on non-wasm32 targets
/// but on wasm32, we need to implement it ourselves — hence this struct.
pub struct WasmReader {
    bytes_stream: Box<dyn Unpin + Stream<Item = std::io::Result<bytes::Bytes>>>,
    pending: Option<bytes::Bytes>,
}

impl WasmReader {
    pub fn new(
        bytes_stream: Box<dyn Unpin + Stream<Item = std::io::Result<bytes::Bytes>>>,
    ) -> Self {
        Self {
            bytes_stream,
            pending: None,
        }
    }
}

impl AsyncRead for WasmReader {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<std::io::Result<usize>> {
        let mut bytes_written = 0;
        loop {
            if let Some(mut pending) = self.pending.take() {
                let to_copy = pending.len().min(buf.len());
                // copy to_copy bytes from pending into buf
                buf[..to_copy].copy_from_slice(&pending.split_to(to_copy));
                bytes_written += to_copy;
                if !pending.is_empty() {
                    self.pending = Some(pending);
                }
                return Poll::Ready(Ok(bytes_written));
            }

            match Pin::new(self.bytes_stream.as_mut()).poll_next(cx) {
                Poll::Ready(Some(Ok(chunk))) => {
                    assert!(chunk.len() > 0);
                    self.pending = Some(chunk);
                }
                Poll::Ready(Some(Err(e))) => return Poll::Ready(Err(e)),
                Poll::Ready(None) => return Poll::Ready(Ok(bytes_written)),
                Poll::Pending => return Poll::Pending,
            }
        }
    }
}
