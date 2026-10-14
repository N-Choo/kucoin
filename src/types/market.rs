//! Public market-data types shared by spot and margin trading.

use serde::{Deserialize, Serialize};

/// Last-price payload for `GET /api/v1/market/orderbook/level1`.
///
/// Margin trades on the same order book as spot, so this shared endpoint is the
/// source of last prices for both.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Level1 {
    /// Last traded price, as a decimal string.
    pub price: String,
}

/// Minimal trading-pair metadata from `GET /api/v1/symbols`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SymbolInfo {
    /// Trading pair, e.g. `BTC-USDT`.
    pub symbol: String,
    /// Minimum size step for the base asset, as a decimal string.
    #[serde(default)]
    pub base_increment: String,
    /// Minimum order size for the base asset, as a decimal string.
    #[serde(default)]
    pub base_min_size: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_level1_last_price() {
        let json = r#"{
            "code": "200000",
            "data": {
                "sequence": "1",
                "price": "83600.5",
                "size": "0.1",
                "bestAsk": "83601",
                "bestAskSize": "0.5",
                "bestBid": "83600",
                "bestBidSize": "0.4",
                "time": 1700000000000
            }
        }"#;
        let res: crate::types::KuCoinResponse<Level1> = serde_json::from_str(json).unwrap();
        assert_eq!(res.data.unwrap().price, "83600.5");
    }

    #[test]
    fn parses_symbol_info() {
        let json = r#"{
            "symbol": "BTC-USDT",
            "name": "BTC-USDT",
            "baseCurrency": "BTC",
            "quoteCurrency": "USDT",
            "baseIncrement": "0.00000001",
            "baseMinSize": "0.000001",
            "enableTrading": true
        }"#;
        let info: SymbolInfo = serde_json::from_str(json).unwrap();
        assert_eq!(info.symbol, "BTC-USDT");
        assert_eq!(info.base_increment, "0.00000001");
        assert_eq!(info.base_min_size, "0.000001");
    }

    #[test]
    fn defaults_missing_symbol_sizing_fields() {
        let info: SymbolInfo = serde_json::from_str(r#"{"symbol":"XRP-USDT"}"#).unwrap();
        assert_eq!(info.base_increment, "");
        assert_eq!(info.base_min_size, "");
    }
}
