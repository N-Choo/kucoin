// Example code that deserializes and serializes the model.
// extern crate serde;
// #[macro_use]
// extern crate serde_derive;
// extern crate serde_json;
//
// use generated_module::SpotContract;
//
// fn main() {
//     let json = r#"{"answer": 42}"#;
//     let model: SpotContract = serde_json::from_str(&json).unwrap();
// }
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchSpotContract {
    /// Order List
    pub order_list: Vec<SpotOrderRequest>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpotOrderRequest {
    /// Order failed after timeout of specified milliseconds, If clientTimestamp +
    /// allowMaxTimeWindow < Gateway received the message time, this order will fail.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_max_time_window: Option<i64>,
    /// Cancel after n seconds, the order timing strategy is GTT, -1 means it will not be
    /// cancelled automatically, the default value is -1
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancel_after: Option<i64>,
    /// Client Order Id，The ClientOid field is a unique ID created by the user（we recommend using
    /// a UUID）, and can only contain numbers, letters, underscores （_）, and hyphens （-）. This
    /// field is returned when order information is obtained. You can use clientOid to tag your
    /// orders. ClientOid is different from the order ID created by the service provider. Please
    /// do not initiate requests using the same clientOid. The maximum length for the ClientOid
    /// is 40 characters.
    ///
    /// Please remember the orderId created by the service provider, it used to check for updates
    /// in order status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_oid: Option<String>,
    /// Equal to KC-API-TIMESTAMP, Need to be defined if allowMaxTimeWindow is specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_timestamp: Option<i64>,
    /// When **type** is market, select one out of two: size or funds
    ///
    /// When placing a market order, the funds field refers to the funds for the priced asset
    /// (the asset name written latter) of the trading pair. The funds must be based on the
    /// quoteIncrement of the trading pair. The quoteIncrement represents the precision of the
    /// trading pair. The funds value for an order must be a multiple of quoteIncrement and must
    /// be between quoteMinSize and quoteMaxSize.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub funds: Option<String>,
    /// [Hidden order](/docs-new/enums-definitions) or not (not shown in order book)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hidden: Option<bool>,
    /// Whether or not only visible portions of orders are shown in [Iceberg
    /// orders](/docs-new/enums-definitions)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iceberg: Option<bool>,
    /// passive order labels, this is disabled when the order timing strategy is IOC or FOK
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_only: Option<bool>,
    /// Specify price for order
    ///
    /// When placing a limit order, the price must be based on priceIncrement for the trading
    /// pair. The price increment (priceIncrement) is the price precision for the trading pair.
    /// For example, for the BTC-USDT trading pair, the priceIncrement is 0.00001000. So the
    /// price for your orders cannot be less than 0.00001000 and must be a multiple of
    /// priceIncrement. Otherwise, the order will return an invalid priceIncrement error.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    /// Order placement remarks, length cannot exceed 20 characters (ASCII)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remark: Option<String>,
    /// specify if the order is to 'buy' or 'sell'
    pub side: Side,
    /// Specify quantity for order
    ///
    /// When **type** is limit, size refers to the amount of trading targets (the asset name
    /// written in front) for the trading pair. Teh Size must be based on the baseIncrement of
    /// the trading pair. The baseIncrement represents the precision for the trading pair. The
    /// size of an order must be a positive-integer multiple of baseIncrement and must be between
    /// baseMinSize and baseMaxSize.
    ///
    /// When **type** is market, select one out of two: size or funds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    /// [Self Trade Prevention](/docs-new/enums-definitions) is divided into four strategies: CN,
    /// CO, CB, and DC.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stp: Option<Stp>,
    /// symbol
    pub symbol: String,
    /// Order tag, length cannot exceed 20 characters (ASCII)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<String>,
    /// [Time in Force](/docs-new/enums-definitions) is a special strategy used during trading to
    /// specify how long an order remains active before execution or expiration. **Market orders
    /// are not supported**. Order fills include self-fills. Default is `GTC`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_in_force: Option<TimeInForce>,
    /// specify if the order is an 'limit' order or 'market' order.
    ///
    /// The type of order you specify when you place your order determines whether or not you
    /// need to request other parameters and also affects the execution of the matching engine.
    ///
    /// When placing a limit order, you must specify a price and size. The system will try to
    /// match the order according to market price or a price better than market price. If the
    /// order cannot be immediately matched, it will stay in the order book until it is matched
    /// or the user cancels.
    ///
    /// Unlike limit orders, the price for market orders fluctuates with market prices. When
    /// placing a market order, you do not need to specify a price, you only need to specify a
    /// quantity. Market orders are filled immediately and will not enter the order book. All
    /// market orders are takers and a taker fee will be charged.
    #[serde(rename = "type")]
    pub spot_contract_type: TradeType,
    /// Maximum visible quantity in iceberg orders
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visible_size: Option<String>,
}

/// specify if the order is to 'buy' or 'sell'
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Side {
    Buy,
    Sell,
}

/// specify if the order is an 'limit' order or 'market' order.
///
/// The type of order you specify when you place your order determines whether or not you
/// need to request other parameters and also affects the execution of the matching engine.
///
/// When placing a limit order, you must specify a price and size. The system will try to
/// match the order according to market price or a price better than market price. If the
/// order cannot be immediately matched, it will stay in the order book until it is matched
/// or the user cancels.
///
/// Unlike limit orders, the price for market orders fluctuates with market prices. When
/// placing a market order, you do not need to specify a price, you only need to specify a
/// quantity. Market orders are filled immediately and will not enter the order book. All
/// market orders are takers and a taker fee will be charged.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TradeType {
    Limit,
    Market,
}

/// [Self Trade Prevention](/docs-new/enums-definitions) is divided into four strategies: CN,
/// CO, CB, and DC.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Stp {
    #[serde(rename = "CB")]
    Cb,
    #[serde(rename = "CN")]
    Cn,
    #[serde(rename = "CO")]
    Co,
    #[serde(rename = "DC")]
    Dc,
}

/// [Time in Force](/docs-new/enums-definitions) is a special strategy used during trading to
/// specify how long an order remains active before execution or expiration. **Market orders
/// are not supported**. Order fills include self-fills. Default is `GTC`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TimeInForce {
    #[serde(rename = "FOK")]
    Fok,
    #[serde(rename = "GTC")]
    Gtc,
    #[serde(rename = "GTT")]
    Gtt,
    #[serde(rename = "IOC")]
    Ioc,
}

/// Cancel Partial Order for Spot request.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpotCancelRequest {
    /// The unique order id generated by the trading system
    pub order_id: String,
    /// The size you want cancel
    pub cancel_size: String,
    /// symbol
    pub symbol: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpotCanceledData {
    /// The size you canceled
    pub cancel_size: String,
    /// order id
    pub order_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchOrderResult {
    pub code: String,
    pub data: Vec<SpotOrderResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpotData {
    /// The user self-defined order id.
    pub client_oid: String,
    /// The unique order id generated by the trading system,which can be used later for further
    /// actions such as canceling the order.
    pub order_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpotOrderResult {
    /// The user self-defined order ID.
    pub client_oid: Option<String>,
    /// Error message
    pub fail_msg: Option<String>,
    /// The unique order ID generated by the trading system, which can be used later for further
    /// actions such as canceling the order.
    pub order_id: Option<String>,
    /// Add order success/failure
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpotDatum {
    /// Order status: true-The status of the order isactive; false-The status of the order is done
    pub active: bool,
    /// A GTT timeInForce that expires in n seconds
    pub cancel_after: i64,
    /// Whether there is a cancellation record for the order.
    pub cancel_exist: bool,
    /// Funds of canceled transactions
    pub cancelled_funds: String,
    /// Number of canceled transactions
    pub cancelled_size: String,
    pub channel: String,
    /// Client Order Id，unique identifier created by the user
    pub client_oid: String,
    pub created_at: i64,
    /// Funds of filled transactions
    pub deal_funds: String,
    /// Number of filled transactions
    pub deal_size: String,
    /// [Handling fees](/docs-new/5327739m0)
    pub fee: String,
    /// currency used to calculate trading fee
    pub fee_currency: String,
    /// Order Funds
    pub funds: String,
    /// Whether its a hidden order.
    pub hidden: bool,
    /// Whether its a iceberg order.
    pub iceberg: bool,
    /// The unique order id generated by the trading system
    pub id: String,
    /// Whether to enter the orderbook: true: enter the orderbook; false: not enter the orderbook
    pub in_order_book: bool,
    pub last_updated_at: i64,
    pub op_type: String,
    /// Whether its a postOnly order.
    pub post_only: bool,
    /// Order price
    pub price: String,
    /// Funds of remain transactions
    pub remain_funds: String,
    /// Number of remain transactions
    pub remain_size: String,
    /// Order placement remarks
    pub remark: Option<String>,
    /// Order side
    pub side: Side,
    /// Order size
    pub size: String,
    /// [Self Trade Prevention](/docs-new/enums-definitions) is divided into four strategies: CN,
    /// CO, CB, and DC.
    pub stp: Option<Stp>,
    /// symbol
    pub symbol: String,
    /// Order tag
    pub tags: Option<String>,
    /// Users in some regions need query this field
    pub tax: String,
    /// [Time in Force](/docs-new/enums-definitions) is a special strategy used during trading to
    /// specify how long an order remains active before execution or expiration. **Market orders
    /// are not supported**. Order fills include self-fills. Default is `GTC`.
    pub time_in_force: TimeInForce,
    /// Trade type, redundancy param
    pub trade_type: String,
    /// Order type
    #[serde(rename = "type")]
    pub datum_type: Type,
    /// Visible size of iceberg order in order book.
    pub visible_size: String,
}

/// Order type
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Type {
    Limit,
    Market,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelAllRes {
    /// The Symbols Failed to cancel
    pub failed_symbols: Vec<FailedSymbol>,
    /// The Symbols Successfully cancelled
    pub succeed_symbols: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FailedSymbol {
    /// error message
    pub error: Option<String>,
    /// symbol
    pub symbol: Option<String>,
}

/// Request body for a Spot OCO (one-cancels-other) order.
///
/// Pairs a take-profit limit order (`price`) with a stop-loss order
/// (`stop_price`/`limit_price`); when one leg fills or is cancelled the other is
/// cancelled automatically.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OcoOrderRequest {
    /// Trading pair, e.g. `BTC-USDT`.
    pub symbol: String,
    /// `buy` or `sell`.
    pub side: Side,
    /// Limit price of the take-profit order.
    pub price: String,
    /// Order quantity (base asset).
    pub size: String,
    /// Client-defined unique order id (a UUID is recommended).
    pub client_oid: String,
    /// Trigger price of the stop-loss order.
    pub stop_price: String,
    /// Limit price of the stop-loss order.
    pub limit_price: String,
    /// Order placement remarks, no longer than 20 ASCII characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remark: Option<String>,
    /// Order source: `TRADE` (spot), `MARGIN_TRADE`, or `MARGIN_ISOLATED_TRADE`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trade_type: Option<String>,
}

/// Response payload for a placed Spot OCO order.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OcoOrderData {
    /// The unique order id generated by the trading system.
    pub order_id: String,
    /// The user-defined client order id, when echoed back.
    #[serde(default)]
    pub client_oid: Option<String>,
}
