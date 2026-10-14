use base64::{Engine, engine::general_purpose::STANDARD};
use ethers::core::k256::sha2::Sha256;
use hmac::{self, Hmac, Mac};

/// Use API-Secret to encrypt the prehash string {timestamp+method+endpoint+body} with sha256 HMAC.
///
/// # Parameters
/// - api_secret: KC-API Secret key.
/// - timestamp : Must be consistent with the 'KC-API-TIMESTAMP' field in the request header.
/// - method    : HTTP-method in UPPER-CASE (POST, GET...).
/// - endpoint  : API-link.
/// - payload   : The request body is a JSON string and need to be the same with the parameters passed by the API.
///
/// # Returns
/// - Encoded prehash as base64
pub fn encrypt_prehash(
    api_secret: &str,
    timestamp: &str,
    method: &str,
    endpoint: &str,
    body: &str,
) -> String {
    // Initialize HMAC-sha256 with secret api key.
    let mut mac = Hmac::<Sha256>::new_from_slice(api_secret.as_bytes())
        .expect("Use API-Secret to encrypt the prehash");
    // Add prehash-string to be hashed.
    let prehash = format!("{}{}{}{}", timestamp, method, endpoint, body);
    mac.update(prehash.as_bytes());

    // Compute HMAC & encode it as base64
    let result = mac.finalize();
    let sign = result.into_bytes();
    STANDARD.encode(sign)
}

/// Encrypt passphrase with HMAC-sha256 via API-Secret.
///
/// # Parameters
/// - api_secret: KC-API Secret key.
/// - passphrase: KC-pass.
///
/// # Returns
/// - Encrypted passcode as base64.
pub fn encrypt_pass(api_secret: String, passphrase: String) -> String {
    // Initialize HMAC-sha256 with secret api key.
    let mut mac = Hmac::<Sha256>::new_from_slice(api_secret.as_bytes())
        .expect("Use API-Secret to encrypt the passphrase");
    // include prehash-string to be hashed.
    mac.update(passphrase.as_bytes());

    // Compute HMAC & encode it as base64
    let result = mac.finalize();
    let sign = result.into_bytes();
    STANDARD.encode(sign)
}

#[cfg(test)]
mod test {
    use super::*;
    use std::time::SystemTime;
    use std::time::UNIX_EPOCH;

    #[test]
    fn test_hmac() {
        // Setup data needed to generate HMAC-SHA256
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Clock may have gone backwards")
            .as_millis()
            .to_string();
        let payload = r#"{"clientOid": "235b7471-0190-4e10-a4cf-953c83a06af5", "side": "sell", "symbol": "ETH-USDT", "type": "market", "isIsolated": false, "funds": "1"}"#;

        // Generate encode 64 sign msg
        let en64_sign = encrypt_prehash(
            "1a422807-19f5-4e8f-9135-b89707845621",
            &timestamp,
            "POST",
            "/api/v3/hf/margin/order",
            payload,
        );

        println!("{}", en64_sign);
    }

    #[test]
    fn test_passphrase() {
        let _passphrase = "910988".to_string();
        let en64_pass = encrypt_pass(
            "1a922807-19f5-4e6c-9135-b89707845621".to_string(),
            _passphrase,
        );
        println!("{}", en64_pass);
    }
}
