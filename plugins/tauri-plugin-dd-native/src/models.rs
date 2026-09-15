use serde::{Deserialize, Serialize};

/// What the web frontend sends when it wants a compressed file saved.
/// `data_base64` is the compressed file's bytes — base64 travels cleanly
/// over the Tauri IPC bridge's JSON-shaped invoke() calls; see
/// js/native-bridge.js for the size tradeoff this implies and why it's
/// fine for this app's already-enforced file-size ceiling.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SaveFilePayload {
    pub filename: String,
    pub mime_type: String,
    pub data_base64: String,
}

/// What comes back after a successful native save. `uri` is the
/// content:// URI Android's FileProvider issued (usable for sharing);
/// `path` is a human-readable path for display only.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SaveFileResponse {
    pub ok: bool,
    pub uri: Option<String>,
    pub path: Option<String>,
}
