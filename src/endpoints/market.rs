//! Public market-data endpoints (`/api/v1/market`, `/api/v1/symbols`).

use crate::{
    client::rest::KuCoinClient,
    types::{
        KuCoinResponse,
        market::{Level1, SymbolInfo},
    },
    utils::errors::KucoinResults,
};

/// Public market-data operations for a [`KuCoinClient`].
pub struct MarketHandler<'a> {
    pub client: &'a KuCoinClient,
}

impl MarketHandler<'_> {
    /// Last traded price for `symbol`.
    /// Endpoint: `GET /api/v1/market/orderbook/level1?symbol={symbol}`.
    ///
    /// Weight 1, so preferred over `allTickers` when only a few symbols are
    /// needed. Margin trades on the same order book as spot, so this serves
    /// both.
    pub async fn get_last_price(&self, symbol: &str) -> KucoinResults<KuCoinResponse<Level1>> {
        let endpoint = format!("/api/v1/market/orderbook/level1?symbol={symbol}");
        let res = self
            .client
            .send::<KuCoinResponse<Level1>>("GET", "", &endpoint)
            .await?;
        Ok(res)
    }

    /// Trading-pair metadata for `symbol`, or `None` when the exchange does not
    /// list it. Endpoint: `GET /api/v1/symbols`.
    ///
    /// The endpoint has no per-symbol variant, so the full list is fetched and
    /// filtered client-side.
    pub async fn get_symbol(&self, symbol: &str) -> KucoinResults<Option<SymbolInfo>> {
        let res = self
            .client
            .send::<KuCoinResponse<Vec<SymbolInfo>>>("GET", "", "/api/v1/symbols")
            .await?;
        Ok(res
            .data
            .and_then(|symbols| symbols.into_iter().find(|s| s.symbol == symbol)))
    }
}
