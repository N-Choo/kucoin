use crate::{
    client::rest::KuCoinClient,
    types::{
        KuCoinResponse,
        deposit::{Deposit, DepositHistoryRequest, DepositList, DepositStatus},
    },
};

pub struct DepositHandler<'a> {
    pub client: &'a KuCoinClient,
}

impl DepositHistoryRequest {
    /// Create a new Deposit query with the mandatory 'currency' field.
    /// Examples currency: BTC,ETH,USDT
    pub fn new(currency: Option<String>) -> Self {
        DepositHistoryRequest {
            currency: currency,
            current_page: None,
            end_at: None,
            page_size: None,
            start_at: None,
            status: None,
        }
    }

    /// Set the status (Chainable).
    pub fn set_status(mut self, status: DepositStatus) -> Self {
        self.status = Some(status);
        self
    }

    /// Set the current page (Chainable).
    pub fn set_current_page(mut self, page: i64) -> Self {
        self.current_page = Some(page);
        self
    }

    /// Set the page size (Chainable).
    /// Note: API usually requires min 10, max 500.
    pub fn set_page_size(mut self, size: i64) -> Self {
        self.page_size = Some(size);
        self
    }

    /// Set the start timestamp in milliseconds (Chainable).
    pub fn set_start_at(mut self, start_at: i64) -> Self {
        self.start_at = Some(start_at);
        self
    }

    /// Set the end timestamp in milliseconds (Chainable).
    pub fn set_end_at(mut self, end_at: i64) -> Self {
        self.end_at = Some(end_at);
        self
    }

    /// This method **mutates** the provided `client` instance by overwriting its:
    /// - `base_link` to `https://api.kucoin.com`
    /// - `endpoint` to `/api/v1/deposits` + deposit query.
    ///
    /// # Argurments
    /// - 'client' - Mutable instance of 'KuCoinClient'
    ///
    /// # Returns
    /// - Request query for deposit in string.
    fn build_endpoint(&self) -> String {
        if let Some(_) = self.currency.clone() {
            let query = serde_urlencoded::to_string(&self).unwrap();
            return format!("/api/v1/deposits?{}", query);
        }
        "/api/v1/deposits".to_string()
    }
}

impl<'a> DepositHandler<'a> {
    pub async fn history(
        &self,
        filter: DepositHistoryRequest,
    ) -> Result<KuCoinResponse<DepositList>, reqwest::Error> {
        // Build endpoint
        let endpoint = filter.build_endpoint();
        self.client
            .send::<KuCoinResponse<DepositList>>("GET", "", &endpoint)
            .await
    }

    pub async fn by_tx_hash(&self, signature: &str) -> Result<Option<Deposit>, reqwest::Error> {
        let filter = DepositHistoryRequest::new(None);
        let deposit_log = self.history(filter).await?;

        let items = match deposit_log.data {
            Some(data) => data.items,
            None => return Ok(None),
        };

        let target_item = items
            .into_iter()
            .find(|item| item.wallet_tx_id.as_deref() == Some(signature));
        Ok(target_item)
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use crate::client::rest::Credentials;
    use std::env;

    #[tokio::test]
    async fn test_get_deposits() {
        // 1. Setup Credentials
        let credentials = Credentials::new(
            &env::var("api_key").unwrap(),
            &env::var("api_secret").unwrap(),
            &env::var("api_passphrase").unwrap(),
        );

        // 2. Initialize Client
        let client = KuCoinClient::new(credentials);

        // 3. configure search_filter.
        let search_filter = DepositHistoryRequest::new(Some("SOL".to_string()))
            .set_status(DepositStatus::Success)
            .set_page_size(20); // 20 rows per page.

        // 4. Fetch deposit history for client.
        let res = client.deposit().history(search_filter).await;
        println!("Deposit history: {:#?}", res);
    }

    #[tokio::test]
    async fn test_transaction_lookup() {
        // 1. Setup Credentials
        let credentials = Credentials::new(
            &env::var("api_key").unwrap(),
            &env::var("api_secret").unwrap(),
            &env::var("api_passphrase").unwrap(),
        );

        // 2. Initialize Client
        let client = KuCoinClient::new(credentials);

        // 3. Get a target deposit log.
        let res = client.deposit().by_tx_hash("4h1HgWnEdxjJrnQj3NzzBngEm8oJsyCBHuT8srCK6vfasD2YDuGFkeoZ1G1XwRKRrxN5YBNRZJ36qTu9BYt41aGn").await;
        println!("Deposit history: {:#?}", res);
    }
}
