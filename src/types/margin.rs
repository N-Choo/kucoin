//! Margin order types for KuCoin cross/isolated margin trading.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::types::spot::{Side, TimeInForce, TradeType};

/// Request body for `POST /api/v3/hf/margin/order`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarginOrderRequest {
    /// Client-defined unique order id (a UUID is recommended).
    pub client_oid: String,
    /// Trading pair, e.g. `BTC-USDT`.
    pub symbol: String,
    /// `buy` or `sell`.
    pub side: Side,
    /// `limit` or `market`.
    #[serde(rename = "type")]
    pub order_type: TradeType,
    /// Base-asset quantity. For market orders use `size` XOR `funds`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    /// Quote-asset amount, the market-buy alternative to `size`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub funds: Option<String>,
    /// Limit price.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    /// `true` = isolated margin; absent/`false` = cross margin.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_isolated: Option<bool>,
    /// Auto-borrow any missing asset before placing the order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_borrow: Option<bool>,
    /// Auto-repay borrowed assets when the position closes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_repay: Option<bool>,
    /// Time in force (`GTC`/`GTT`/`IOC`/`FOK`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_in_force: Option<TimeInForce>,
}

impl MarginOrderRequest {
    /// New margin order with a fresh `client_oid`, defaulting to cross margin.
    pub fn new(order_type: TradeType, symbol: &str, side: Side) -> Self {
        MarginOrderRequest {
            client_oid: Uuid::new_v4().to_string(),
            symbol: symbol.to_string(),
            side,
            order_type,
            size: None,
            funds: None,
            price: None,
            is_isolated: None,
            auto_borrow: None,
            auto_repay: None,
            time_in_force: None,
        }
    }
}

/// Response `data` for a placed margin order.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarginOrderData {
    /// Exchange-assigned order id.
    pub order_id: String,
    /// Loan application id, present only when auto-borrow triggered.
    pub loan_apply_id: Option<String>,
    /// Borrowed size, present only when auto-borrow triggered.
    pub borrow_size: Option<String>,
    /// Echoed client order id.
    pub client_oid: Option<String>,
}

/// Subset of `/api/v3/hf/margin/orders/{id}` needed for reconciliation.
///
/// Deliberately lenient: only `active` is required; the rest default, so a
/// `null` in any other response field cannot fail decoding.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarginOrderStatus {
    /// `true` while the order is working; `false` once done or cancelled.
    pub active: bool,
    /// Number of filled base-asset units.
    #[serde(default)]
    pub deal_size: String,
    /// Funds of filled transactions (quote asset).
    #[serde(default)]
    pub deal_funds: String,
    /// Trading fee charged on the fill.
    #[serde(default)]
    pub fee: String,
}

/// Minimal `/api/v3/hf/margin/orders/client-order/{oid}` lookup response.
///
/// Only the exchange-assigned `id` is needed to resolve a client-side order id.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarginOrderId {
    /// Exchange-assigned order id.
    pub id: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serializes_market_sell_with_autoborrow() {
        let mut req = MarginOrderRequest::new(TradeType::Market, "XRP-USDT", Side::Sell);
        req.client_oid = "abc-123".to_string();
        req.size = Some("15.9".to_string());
        req.auto_borrow = Some(true);
        req.auto_repay = Some(true);

        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"symbol\":\"XRP-USDT\""));
        assert!(json.contains("\"side\":\"sell\""));
        assert!(json.contains("\"type\":\"market\""));
        assert!(json.contains("\"clientOid\":\"abc-123\""));
        assert!(json.contains("\"size\":\"15.9\""));
        assert!(json.contains("\"autoBorrow\":true"));
        assert!(json.contains("\"autoRepay\":true"));
        assert!(!json.contains("funds"));
        assert!(!json.contains("isIsolated"));
    }

    #[test]
    fn new_generates_unique_client_oids() {
        let a = MarginOrderRequest::new(TradeType::Market, "XRP-USDT", Side::Sell);
        let b = MarginOrderRequest::new(TradeType::Market, "XRP-USDT", Side::Sell);
        assert!(Uuid::parse_str(&a.client_oid).is_ok());
        assert!(Uuid::parse_str(&b.client_oid).is_ok());
        assert_ne!(a.client_oid, b.client_oid);
    }

    #[test]
    fn serializes_limit_order_with_isolated_and_time_in_force() {
        let mut req = MarginOrderRequest::new(TradeType::Limit, "BTC-USDT", Side::Buy);
        req.client_oid = "cid-limit".to_string();
        req.price = Some("60000".to_string());
        req.size = Some("0.001".to_string());
        req.is_isolated = Some(true);
        req.time_in_force = Some(TimeInForce::Gtc);

        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"type\":\"limit\""));
        assert!(json.contains("\"price\":\"60000\""));
        assert!(json.contains("\"isIsolated\":true"));
        assert!(json.contains("\"timeInForce\":\"GTC\""));
    }

    #[test]
    fn serializes_market_buy_side_and_omits_none() {
        let mut req = MarginOrderRequest::new(TradeType::Market, "SOL-USDT", Side::Buy);
        req.size = Some("0.1".to_string());
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"side\":\"buy\""));
        assert!(!json.contains("price"));
        assert!(!json.contains("autoBorrow"));
    }

    #[test]
    fn parses_margin_order_data_with_borrow_fields() {
        let json = r#"{"orderId":"o-1","loanApplyId":"l-1","borrowSize":"15.9","clientOid":"abc"}"#;
        let data: MarginOrderData = serde_json::from_str(json).unwrap();
        assert_eq!(data.order_id, "o-1");
        assert_eq!(data.loan_apply_id.as_deref(), Some("l-1"));
        assert_eq!(data.borrow_size.as_deref(), Some("15.9"));
    }

    #[test]
    fn parses_margin_order_data_without_borrow_fields() {
        let data: MarginOrderData = serde_json::from_str(r#"{"orderId":"o-2"}"#).unwrap();
        assert_eq!(data.order_id, "o-2");
        assert!(data.loan_apply_id.is_none());
        assert!(data.borrow_size.is_none());
    }

    #[test]
    fn order_status_tolerates_null_unrelated_fields() {
        let json = r#"{
            "active": false,
            "dealSize": "0.072",
            "dealFunds": "7.95",
            "fee": "0.008",
            "visibleSize": null,
            "tax": null,
            "channel": null,
            "clientOid": null
        }"#;
        let status: MarginOrderStatus = serde_json::from_str(json).expect("lenient decode");
        assert!(!status.active);
        assert_eq!(status.deal_size, "0.072");
        assert_eq!(status.deal_funds, "7.95");
        assert_eq!(status.fee, "0.008");
    }

    #[test]
    fn order_status_defaults_missing_reconcile_fields() {
        let status: MarginOrderStatus = serde_json::from_str(r#"{"active": true}"#).unwrap();
        assert!(status.active);
        assert_eq!(status.deal_size, "");
        assert_eq!(status.deal_funds, "");
        assert_eq!(status.fee, "");
    }

    #[test]
    fn parses_client_order_lookup_id() {
        let id: MarginOrderId =
            serde_json::from_str(r#"{"id":"o-9","symbol":"BTC-USDT"}"#).unwrap();
        assert_eq!(id.id, "o-9");
    }
}
