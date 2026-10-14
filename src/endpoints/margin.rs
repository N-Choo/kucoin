//! KuCoin cross/isolated margin order endpoints (`/api/v3/hf/margin`).

use crate::{
    client::rest::KuCoinClient,
    types::{
        KuCoinResponse,
        margin::{MarginOrderData, MarginOrderId, MarginOrderRequest, MarginOrderStatus},
    },
    utils::errors::KucoinResults,
};

/// Margin order-status endpoint: `/api/v3/hf/margin/orders/{id}?symbol={symbol}`.
fn order_endpoint(order_id: &str, symbol: &str) -> String {
    format!("/api/v3/hf/margin/orders/{order_id}?symbol={symbol}")
}

/// Margin clientOid endpoint:
/// `/api/v3/hf/margin/orders/client-order/{oid}?symbol={symbol}`.
fn client_order_endpoint(client_oid: &str, symbol: &str) -> String {
    format!("/api/v3/hf/margin/orders/client-order/{client_oid}?symbol={symbol}")
}

/// Margin order operations for a [`KuCoinClient`].
pub struct MarginHandler<'a> {
    pub client: &'a KuCoinClient,
}

impl MarginHandler<'_> {
    /// Place a margin order. Endpoint: `POST /api/v3/hf/margin/order`.
    ///
    /// One endpoint covers buy, sell, and close: closing is an order on the
    /// opposite side. Set `auto_borrow`/`auto_repay` to trade unbacked.
    pub async fn place_order(
        &self,
        order: MarginOrderRequest,
    ) -> KucoinResults<KuCoinResponse<MarginOrderData>> {
        self.submit("/api/v3/hf/margin/order", order).await
    }

    /// Validate a margin order without executing it.
    /// Endpoint: `POST /api/v3/hf/margin/order/test`.
    ///
    /// The request and response are identical to [`Self::place_order`], but the
    /// order never enters the matching engine and cannot be queried afterwards.
    /// Use it to verify signing, parameters, and fund/hold availability before
    /// committing a real order.
    pub async fn place_order_test(
        &self,
        order: MarginOrderRequest,
    ) -> KucoinResults<KuCoinResponse<MarginOrderData>> {
        self.submit("/api/v3/hf/margin/order/test", order).await
    }

    /// POST `order` to `endpoint`; the order and order/test endpoints share the
    /// exact same request and response wire format.
    async fn submit(
        &self,
        endpoint: &str,
        order: MarginOrderRequest,
    ) -> KucoinResults<KuCoinResponse<MarginOrderData>> {
        let body = serde_json::to_string(&order)?;
        let res = self
            .client
            .send::<KuCoinResponse<MarginOrderData>>("POST", &body, endpoint)
            .await?;
        Ok(res)
    }

    /// Fetch a margin order's status by its exchange id.
    /// Endpoint: `GET /api/v3/hf/margin/orders/{id}?symbol={symbol}`.
    pub async fn get_order(
        &self,
        order_id: &str,
        symbol: &str,
    ) -> KucoinResults<KuCoinResponse<MarginOrderStatus>> {
        let res = self
            .client
            .send::<KuCoinResponse<MarginOrderStatus>>("GET", "", &order_endpoint(order_id, symbol))
            .await?;
        Ok(res)
    }

    /// Fetch a margin order by the clientOid we assigned.
    /// Endpoint: `GET /api/v3/hf/margin/orders/client-order/{oid}?symbol={symbol}`.
    ///
    /// A missing order is surfaced as an error for which
    /// [`KucoinErrors::is_not_found`](crate::utils::errors::KucoinErrors::is_not_found)
    /// is `true` (KuCoin answers `404`, `code:126043`), not as `Ok(None)`.
    pub async fn get_order_by_client_oid(
        &self,
        client_oid: &str,
        symbol: &str,
    ) -> KucoinResults<KuCoinResponse<MarginOrderId>> {
        let res = self
            .client
            .send::<KuCoinResponse<MarginOrderId>>(
                "GET",
                "",
                &client_order_endpoint(client_oid, symbol),
            )
            .await?;
        Ok(res)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::client::rest::Credentials;
    use crate::types::spot::{Side, TradeType};
    use std::env;

    /// A client built from `api_key`/`api_secret`/`api_passphrase`, or `None`
    /// when those are unset.
    fn client() -> Option<KuCoinClient> {
        let key = env::var("api_key").ok()?;
        let secret = env::var("api_secret").ok()?;
        let passphrase = env::var("api_passphrase").ok()?;
        Some(KuCoinClient::new(Credentials::new(
            &key,
            &secret,
            &passphrase,
        )))
    }

    /// Live smoke test: the exchange must accept — but not execute — a market
    /// entry and a market exit, exercising auth signing, request
    /// serialization, the borrow/repay flags, and response parsing.
    ///
    /// Ignored by default: needs funded margin credentials in
    /// `api_key`/`api_secret`/`api_passphrase`. Run with:
    /// `cargo test -p kucoin -- --ignored order_test_accepts_market_orders`.
    #[tokio::test]
    #[ignore = "live: needs funded margin credentials"]
    async fn order_test_accepts_market_orders() {
        let Some(client) = client() else { return };

        // Long (no borrow) and short (auto-borrow/repay) entry intents.
        for (side, borrow) in [(Side::Buy, false), (Side::Sell, true)] {
            let mut req = MarginOrderRequest::new(TradeType::Market, "BTC-USDT", side);
            req.size = Some("0.00001".to_string());
            req.auto_borrow = Some(borrow);
            req.auto_repay = Some(borrow);

            let res = client
                .margin()
                .place_order_test(req)
                .await
                .expect("order/test request should reach the exchange");

            assert_eq!(
                res.code, "200000",
                "order/test rejected: code={} msg={:?}",
                res.code, res.msg
            );
            assert!(res.data.is_some(), "order/test returned no data");
        }
    }

    #[test]
    fn builds_margin_lookup_endpoints() {
        assert_eq!(
            order_endpoint("abc", "XRP-USDT"),
            "/api/v3/hf/margin/orders/abc?symbol=XRP-USDT"
        );
        assert_eq!(
            client_order_endpoint("abc", "XRP-USDT"),
            "/api/v3/hf/margin/orders/client-order/abc?symbol=XRP-USDT"
        );
    }
}
