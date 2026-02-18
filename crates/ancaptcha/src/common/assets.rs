//! Management and retrieval of embedded visual assets.

include!(concat!(env!("OUT_DIR"), "/asset_registry.rs"));

/// Returns the complete registry of embedded visual assets.
#[must_use]
pub const fn get_embedded_assets() -> &'static [(&'static str, &'static [u8])] {
    EMBEDDED_ASSETS
}

/// Retrieves the raw byte data of an embedded asset by its unique identifier.
#[must_use]
pub fn get_asset_by_name(name: &str) -> Option<&'static [u8]> {
    EMBEDDED_ASSETS
        .iter()
        .find(|(n, _)| *n == name)
        .map(|(_, data)| *data)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn embedded_assets_presence() {
        let assets = get_embedded_assets();
        assert!(!assets.is_empty());

        for (name, data) in assets {
            assert!(!name.is_empty());
            assert!(!data.is_empty());
        }
    }

    #[test]
    fn runtime_base64_size_limits() {
        for (name, data) in get_embedded_assets() {
            let b64 = crate::common::b64_encode_std(data);
            assert!(
                b64.len() <= 5120,
                "Base64 for asset {} is too large: {} bytes (target <= 5120)",
                name,
                b64.len()
            );
        }
    }

    #[test]
    fn missing_asset_returns_none() {
        let result = get_asset_by_name("nonexistent");
        assert!(result.is_none());
    }
}
