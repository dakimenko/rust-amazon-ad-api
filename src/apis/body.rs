use serde::Serialize;

/// Trait for converting various input types into a JSON request body string.
pub trait IntoRequestBody {
    fn into_body(self) -> Result<String, anyhow::Error>;
}

impl IntoRequestBody for serde_json::Value {
    fn into_body(self) -> Result<String, anyhow::Error> {
        serde_json::to_string(&self).map_err(Into::into)
    }
}

impl IntoRequestBody for Vec<serde_json::Value> {
    fn into_body(self) -> Result<String, anyhow::Error> {
        serde_json::to_string(&self).map_err(Into::into)
    }
}

impl IntoRequestBody for String {
    fn into_body(self) -> Result<String, anyhow::Error> {
        // Try parsing as JSON first — if valid JSON string, use as-is.
        // Otherwise, try reading as a file path.
        if serde_json::from_str::<serde_json::Value>(&self).is_ok() {
            return Ok(self);
        }

        // Assume it's a file path
        let content = std::fs::read_to_string(&self)?;
        if serde_json::from_str::<serde_json::Value>(&content).is_ok() {
            Ok(content)
        } else {
            Err(anyhow::anyhow!("String is neither valid JSON nor a readable file path"))
        }
    }
}

impl IntoRequestBody for &str {
    fn into_body(self) -> Result<String, anyhow::Error> {
        self.to_string().into_body()
    }
}

/// Convert any serializable type into a JSON request body string.
pub fn to_json_body<T: Serialize>(value: &T) -> Result<String, anyhow::Error> {
    serde_json::to_string(value).map_err(Into::into)
}
