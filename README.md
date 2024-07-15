# streaming_http_range_client

A client for ergonomically streaming HTTP Range requests in Rust.

```
use streaming_http_range_client::HttpClient;
use tokio::io::AsyncReadExt;

use tokio;
let mut new_client = HttpClient::new("https://georust.org");
new_client.set_range(2..14).await.unwrap();

let mut output = String::new();
new_client.read_to_string(&mut output).await.unwrap();

// This `expected_text` may need to be updated someday if someone updates the site.
let expected_text = "DOCTYPE html";
assert_eq!(expected_text, output)
```

This crate was written primarily to serve the needs of
https://github.com/michaelkirk/geomedea, but it might be more generally useful.

