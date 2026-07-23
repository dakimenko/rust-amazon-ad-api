use bytes::Bytes;

/// Format for downloaded content.
#[derive(Debug, Clone)]
pub enum DownloadFormat {
    /// Raw decompressed bytes (decompresses gzip if detected).
    Raw,
    /// Force gzip decompression to raw bytes buffer.
    Gzip,
    /// Parse decompressed bytes as `serde_json::Value`.
    JsonValue,
    /// Return decompressed bytes as UTF-8 String.
    StringValue,
}

/// Result of a download operation.
#[derive(Debug, Clone)]
pub enum DownloadResult {
    Raw(Bytes),
    JsonValue(serde_json::Value),
    String(String),
}

/// Download and process content from a URL in-memory (no disk writes).
///
/// Handles gzip decompression automatically when the format is `Raw`
/// and the Content-Encoding is gzip, or when the body starts with the
/// gzip magic bytes.
pub async fn download(
    client: &reqwest::Client,
    url: &str,
    format: DownloadFormat,
) -> Result<DownloadResult, anyhow::Error> {
    let response = client.get(url).send().await?;

    if !response.status().is_success() {
        let status = response.status();
        let error_text = response.text().await?;
        return Err(anyhow::anyhow!(
            "Download failed: {} - {}",
            status,
            error_text
        ));
    }

    let is_gzip_header = response
        .headers()
        .get("content-encoding")
        .and_then(|v| v.to_str().ok())
        .map(|v| v.contains("gzip"))
        .unwrap_or(false);

    let body_bytes = response.bytes().await?;
    let is_gzip = matches!(format, DownloadFormat::Gzip)
        || is_gzip_header
        || (body_bytes.len() >= 2 && body_bytes[0] == 0x1f && body_bytes[1] == 0x8b);

    let decompressed = if is_gzip {
        use async_compression::tokio::bufread::GzipDecoder;
        use tokio::io::AsyncReadExt;

        let mut decoder = GzipDecoder::new(&body_bytes[..]);
        let mut buf = Vec::new();
        decoder.read_to_end(&mut buf).await?;
        Bytes::from(buf)
    } else {
        body_bytes
    };

    match format {
        DownloadFormat::Raw | DownloadFormat::Gzip => Ok(DownloadResult::Raw(decompressed)),
        DownloadFormat::JsonValue => {
            let value: serde_json::Value = serde_json::from_slice(&decompressed)?;
            Ok(DownloadResult::JsonValue(value))
        }
        DownloadFormat::StringValue => {
            let s = String::from_utf8(decompressed.to_vec())
                .map_err(|e| anyhow::anyhow!("Invalid UTF-8: {}", e))?;
            Ok(DownloadResult::String(s))
        }
    }
}

/// Extension trait to save downloaded bytes to a file.
#[allow(async_fn_in_trait)]
pub trait SaveToFile {
    async fn save_to_file<P: AsRef<std::path::Path>>(&self, path: P) -> std::io::Result<()>;
}

impl SaveToFile for Bytes {
    async fn save_to_file<P: AsRef<std::path::Path>>(&self, path: P) -> std::io::Result<()> {
        let path = path.as_ref().to_path_buf();
        let bytes = self.clone();
        tokio::task::spawn_blocking(move || std::fs::write(path, bytes))
            .await
            .map_err(|e| std::io::Error::other(e.to_string()))?
    }
}
