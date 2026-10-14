use secrecy::{ExposeSecret, SecretString};
use serde::de::DeserializeOwned;
use std::str::FromStr;

use crate::{
    endpoints::{
        deposit::DepositHandler, margin::MarginHandler, market::MarketHandler,
        sub_account::SubAccHander, trades::SpotHandler, transfer::TransferHandler,
        withdrawals::WithdrawHandler,
    },
    utils::{
        auth::{encrypt_pass, encrypt_prehash},
        time,
    },
};
use reqwest::{
    Client, Method,
    header::{CONTENT_TYPE, HeaderMap, HeaderValue, InvalidHeaderValue},
};

/// Holds API authentication secrets (Key, Secret, Passphrase).
#[derive(Clone)]
pub struct Credentials {
    key: SecretString,
    secret: SecretString,
    passphrase: SecretString,
}

impl Credentials {
    /// Create a new 'Credentials' instance.
    pub fn new(key: &str, secret: &str, passphrase: &str) -> Self {
        Credentials {
            key: SecretString::new(key.into()),
            secret: SecretString::new(secret.into()),
            passphrase: SecretString::new(passphrase.into()),
        }
    }
}

/// The main HTTP client for interacting with the KuCoin API.
#[derive(Clone)]
pub struct KuCoinClient {
    credentials: Credentials,
    /// The API host URL (e.g., https://api.kucoin.com).
    pub base_link: String,
    http_client: Client,
}

impl KuCoinClient {
    /// New Client
    ///
    /// # Parameters
    /// - base: Host.
    /// - endpoint: Path.
    ///
    /// #Returns Self.
    pub fn new(credentials: Credentials) -> Self {
        KuCoinClient {
            credentials,
            base_link: "https://api.kucoin.com".to_string(),
            http_client: Client::new(),
        }
    }

    /// Redefine credentials.
    pub fn set_credentials(self: &mut Self, credentials: Credentials) -> &mut Self {
        self.credentials = credentials;
        self
    }

    /// Send The Request with Dyn Method.
    /// # Type Parameters
    /// - `T` - The type to deserialize the response into.
    ///
    /// # Parameters
    /// - payload   : Body for HTTP-request.
    /// - method    : HTTP-request method.
    ///
    /// # Returns
    /// * `Ok(T)` - The API response parsed into the requested struct.
    pub async fn send<T: DeserializeOwned>(
        &self,
        method: &str,
        payload: &str,
        endpoint: &str,
    ) -> Result<T, reqwest::Error> {
        let headers = self.get_headers(payload, method, endpoint);
        let method_type = Method::from_str(method).unwrap();
        let url = format!("{}{}", self.base_link, endpoint);

        // Build Dyn Request based on the method_type.
        let response = self
            .http_client
            .request(method_type, url)
            .headers(headers.unwrap())
            .body(payload.to_string())
            .send()
            .await?
            .error_for_status()?;
        response.json::<T>().await
    }

    /// Build headers with generated encoded for KC-API-SIGN and KC-API-PASSPHRASE.
    ///
    /// # Parameters
    /// - payload   : Body for HTTP-request.
    /// - method    : HTTP-request method.
    /// - endpoint  : Service endpoint
    ///
    /// # Returns
    /// - If headers value passed : A set of HTTP headers.
    /// - If not passed: Error msg.
    fn get_headers(
        &self,
        payload: &str,
        method: &str,
        endpoint: &str,
    ) -> Result<HeaderMap, InvalidHeaderValue> {
        // Encrypting
        let timestamp = &time::get_timestamp();
        let sign = encrypt_prehash(
            &self.credentials.secret.expose_secret(),
            timestamp,
            method,
            endpoint,
            payload,
        );

        let passphrase = encrypt_pass(
            self.credentials.secret.expose_secret().to_string(),
            self.credentials.passphrase.expose_secret().to_string(),
        );

        // Build Headers
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        headers.insert(
            "KC-API-KEY",
            HeaderValue::from_str(self.credentials.key.expose_secret().as_str())?,
        );
        headers.insert("KC-API-SIGN", HeaderValue::from_str(sign.as_str())?);
        headers.insert("KC-API-TIMESTAMP", HeaderValue::from_str(&timestamp)?);
        headers.insert(
            "KC-API-PASSPHRASE",
            HeaderValue::from_str(passphrase.as_str())?,
        );
        headers.insert("KC-API-KEY-VERSION", HeaderValue::from_static("3"));
        Ok(headers)
    }

    // --- Modular Accessors ---

    pub fn deposit(&self) -> DepositHandler<'_> {
        DepositHandler { client: self }
    }

    pub fn spot(&self) -> SpotHandler<'_> {
        SpotHandler { client: self }
    }

    pub fn margin(&self) -> MarginHandler<'_> {
        MarginHandler { client: self }
    }

    pub fn market(&self) -> MarketHandler<'_> {
        MarketHandler { client: self }
    }

    pub fn transfer(&self) -> TransferHandler<'_> {
        TransferHandler { client: self }
    }

    pub fn sub_acc(&self) -> SubAccHander<'_> {
        SubAccHander { client: self }
    }

    pub fn withdraw(&self) -> WithdrawHandler<'_> {
        WithdrawHandler { client: self }
    }
}
