use thiserror::Error as ThisError;

/// Defines Error types.
#[derive(ThisError, Debug)]
pub enum KucoinErrors {
    /// Contents doesn't match the Structure elements  
    #[error("SERDE-JSON-ERROR: {0}")]
    JSONError(#[from] serde_json::Error),

    #[error("Account tag is required for {0} ISOLATED account")]
    MissingIsolatedTag(String),

    #[error("REQWEST-ERROR: {0}")]
    ReqwestError(#[from] reqwest::Error),
}

impl KucoinErrors {
    /// Underlying HTTP status, when the failure came from the transport layer.
    ///
    /// Lets callers classify responses (e.g. a `404` means an order does not
    /// exist) without depending on `reqwest` types directly.
    pub fn status(&self) -> Option<reqwest::StatusCode> {
        match self {
            KucoinErrors::ReqwestError(e) => e.status(),
            _ => None,
        }
    }

    /// The exchange answered `404 Not Found`.
    pub fn is_not_found(&self) -> bool {
        self.status() == Some(reqwest::StatusCode::NOT_FOUND)
    }

    /// The exchange answered `429 Too Many Requests`.
    pub fn is_rate_limited(&self) -> bool {
        self.status() == Some(reqwest::StatusCode::TOO_MANY_REQUESTS)
    }
}
/// Alias Type for Results with Error Handler
pub type KucoinResults<T> = Result<T, KucoinErrors>;
