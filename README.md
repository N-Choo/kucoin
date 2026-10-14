<div align="center">

# 🦀 KuCoin API-Wraper

Optimizing for high-frequency trading, automated bots, and enterprise sub-account management.

[![Crates.io](https://img.shields.io/crates/v/kucoin?style=flat-square&color=dea584)](https://crates.io/crates/kucoin)
[![Docs.rs](https://img.shields.io/docsrs/kucoin?style=flat-square&color=4d76ae)](https://docs.rs/kucoin)
[![License](https://img.shields.io/badge/license-MIT-green?style=flat-square)](LICENSE)

[Documentation](https://docs.rs/kucoin) ·
[Report Bug](https://github.com/your-username/kucoin/issues) ·
[Request Feature](https://github.com/your-username/kucoin/issues)

</div>

---

## Features

Built on `async` and `reqwest`.

<table>
<tr>
<td valign="top" width="50%">

**Spot trading**

- Market & limit orders
- Batch orders
- Order cancellation
- Retrieve open orders
- OCO orders (take-profit + stop-loss)

</td>
<td valign="top" width="50%">

**Margin trading**

- Cross & isolated margin orders
- Auto-borrow / auto-repay
- Order validation without execution (`order/test`)
- Order lookup by id or clientOid

</td>
</tr>
<tr>
<td valign="top" width="50%">

**Wallet & deposits**

- Deposit history with filters
- Lookup by transaction hash

</td>
<td valign="top" width="50%">

**Sub-accounts**

- List sub-accounts
- Create API keys
- Configure permissions & IP whitelists
- Query sub-account balances

</td>
</tr>
<tr>
<td valign="top" width="50%">

**Typed requests**

- Builder-pattern request structs
- Compile-time parameter validation

</td>
<td valign="top" width="50%">

**Market data**

- Last traded price (`level1`)
- Trading-pair metadata (`symbols`)

</td>
</tr>
</table>

---

## Installation

```toml
[dependencies]
kucoin = "0.8.0"
tokio = { version = "1", features = ["full"] }
dotenv = "0.15" # optional
```

---

## Quick Start

```
project-root/
├── client/        # Authentication & HTTP layer
├── endpoints/     # KuCoin API endpoints
├── types/         # Request and response models
└── utils/         # HMAC-SHA256, system time, error types
```

### Client Initialization

```rust
use std::env;
use kucoin::client::rest::{Credentials, KuCoinClient};

#[tokio::main]
async fn main() {
    let credentials = Credentials::new(
        &env::var("API_KEY").expect("API_KEY not set"),
        &env::var("API_SECRET").expect("API_SECRET not set"),
        &env::var("API_PASSPHRASE").expect("API_PASSPHRASE not set"),
    );

    let client = KuCoinClient::new(credentials);
}
```

---

## Spot Trading

### Place an Order

```rust
use kucoin::types::spot::{SpotOrderRequest, TradeType, Side};

async fn place_order(client: &KuCoinClient) {
    let order = SpotOrderRequest::new(TradeType::Market, "BTC-USDT", Side::Buy)
        .set_funds(100.0)
        .set_remark("example-order");

    client.spot().place_order(order).await.unwrap();
}
```

---

### Batch Orders

```rust
use kucoin::types::spot::{BatchSpotContract, SpotOrderRequest, TradeType, Side};

async fn batch_orders(client: &KuCoinClient) {
    let batch = BatchSpotContract::new()
        .add_order(
            SpotOrderRequest::new(TradeType::Market, "BTC-USDT", Side::Buy)
                .set_funds(50.0),
        )
        .add_order(
            SpotOrderRequest::new(TradeType::Market, "SOL-USDT", Side::Buy)
                .set_funds(20.0),
        );

    client.spot().place_multi_orders(batch).await.unwrap();
}
```

---

### Cancel & List Orders

```rust
use kucoin::types::spot::SpotCancelRequest;

async fn cancel(client: &KuCoinClient, order_id: &str) {
    let req = SpotCancelRequest::new(order_id, 0.0, "BTC-USDT");
    client.spot().cancel_order(req).await.unwrap();
}

async fn open_orders(client: &KuCoinClient) {
    let res = client.spot().list_orders_open("BTC-USDT").await.unwrap();
    println!("{:#?}", res.data);
}
```

---

### Get Order By Id

```rust
async fn get_order(client: &KuCoinClient, order_id: &str) {
    let res = client.spot().get_order(order_id, "BTC-USDT").await.unwrap();

    if let Some(order) = res.data {
        println!(
            "active: {} (in order book: {})",
            order.active, order.in_order_book
        );
    }
}
```

---

### OCO Orders (Take-Profit + Stop-Loss)

Place a single one-cancels-other order pairing a take-profit limit with a
stop-loss; when one leg fills or is cancelled, the other is cancelled too.

```rust
use kucoin::types::spot::{OcoOrderRequest, Side};

async fn place_oco(client: &KuCoinClient, qty: f64) {
    let oco = OcoOrderRequest::new("SOL-USDT", Side::Sell)
        .set_price(130.0)       // take-profit limit price
        .set_size(qty)          // base quantity (your filled position)
        .set_stop_price(100.0)  // stop-loss trigger price
        .set_limit_price(99.5)  // stop-loss limit price
        .set_remark("tp-sl");

    let res = client.spot().place_oco_order(oco).await.unwrap();
    if let Some(data) = res.data {
        println!("OCO order id: {}", data.order_id);
    }
}
```

`OcoOrderRequest::new` auto-generates a `client_oid` (UUID) and defaults
`trade_type` to `"TRADE"` (spot). Set an explicit id with
`.set_client_oid("...")` when you want idempotency across retries.

---

## Margin Trading

Cross and isolated margin orders via `POST /api/v3/hf/margin/order`. One
endpoint covers buy, sell, and close (closing is an order on the opposite
side). Enable `auto_borrow`/`auto_repay` to trade without pre-funding the asset.

### Place a Margin Order

```rust
use kucoin::types::margin::MarginOrderRequest;
use kucoin::types::spot::{Side, TradeType};

async fn margin_buy(client: &KuCoinClient) {
    let mut order = MarginOrderRequest::new(TradeType::Market, "BTC-USDT", Side::Buy);
    order.funds = Some("100".to_string()); // quote-asset amount
    order.auto_borrow = Some(true);

    let res = client.margin().place_order(order).await.unwrap();
    if let Some(data) = res.data {
        println!("Margin order id: {}", data.order_id);
    }
}
```

### Isolated Limit Order

```rust
async fn margin_limit(client: &KuCoinClient) {
    let mut order = MarginOrderRequest::new(TradeType::Limit, "ETH-USDT", Side::Sell);
    order.price = Some("4200".to_string());
    order.size = Some("0.5".to_string());
    order.is_isolated = Some(true);
    order.auto_repay = Some(true);

    client.margin().place_order(order).await.unwrap();
}
```

### Validate an Order Without Executing

`MarginOrderRequest` uses public fields rather than builder methods; set
`size`, `funds`, `price`, `is_isolated`, `auto_borrow`, `auto_repay`, and
`time_in_force` directly on the struct.

```rust
async fn margin_dry_run(client: &KuCoinClient) {
    let mut order = MarginOrderRequest::new(TradeType::Market, "XRP-USDT", Side::Sell);
    order.size = Some("10".to_string());

    // Never reaches the matching engine; validates signing and parameters.
    client.margin().place_order_test(order).await.unwrap();
}
```

### Get Order Status

Fetch an order's status by its exchange id.

```rust
use kucoin::types::margin::MarginOrderStatus;

let res = client.margin().get_order("order-id", "BTC-USDT").await.unwrap();
if let Some(MarginOrderStatus { active, deal_size, .. }) = res.data {
    println!("active: {active}, filled: {deal_size}");
}
```

### Look Up by clientOid

Resolve the clientOid you assigned to the exchange order id. A missing order is
returned as an error for which `is_not_found()` is `true` (KuCoin answers `404`,
`code:126043`).

```rust
use kucoin::types::margin::MarginOrderId;

match client.margin().get_order_by_client_oid("my-oid", "BTC-USDT").await {
    Ok(res) => {
        if let Some(MarginOrderId { id }) = res.data {
            println!("exchange id: {id}");
        }
    }
    Err(e) if e.is_not_found() => println!("order was never placed"),
    Err(e) => return Err(e.into()),
}
```

---

## Market Data

Public endpoints — no API key needed. Margin trades on the same order book as
spot, so these serve both.

```rust
use kucoin::types::market::{Level1, SymbolInfo};

// Last traded price (weight 1)
let last = client.market().get_last_price("BTC-USDT").await.unwrap();
if let Some(Level1 { price }) = last.data {
    println!("last price: {price}");
}

// Trading-pair metadata, or `None` when the pair is not listed
if let Some(SymbolInfo { base_min_size, base_increment, .. }) =
    client.market().get_symbol("BTC-USDT").await.unwrap()
{
    println!("min size {base_min_size}, step {base_increment}");
}
```

---

## Deposits

```rust
use kucoin::types::deposit::{DepositHistoryRequest, DepositStatus};

async fn deposits(client: &KuCoinClient) {
    let req = DepositHistoryRequest::new(Some("SOL".to_string()))
        .set_status(DepositStatus::Success)
        .set_page_size(20);

    client.deposit().history(req).await.unwrap();
}
```

```rust
async fn lookup(client: &KuCoinClient, tx: &str) {
    let res = client.deposit().by_tx_hash(tx).await.unwrap();
    println!("{:#?}", res);
}
```

---

## Sub-Accounts

```rust
use kucoin::types::sup_account::{SubAccRequest, Expire};

async fn create_sub(client: &KuCoinClient) {
    let req = SubAccRequest::new("SubUser01", "Trading Bot", "StrongPass123!")
        .set_permission("General,Spot")
        .add_ipwhitelist("192.168.1.1")
        .set_expire(Expire::NeverExpire);

    client.sub_acc().add_api(req).await.unwrap();
}
```

```rust
async fn list_subs(client: &KuCoinClient) {
    let res = client.sub_acc().fetchall().await.unwrap();
    println!("{:#?}", res.data);
}
```

```rust
async fn balance(client: &KuCoinClient, uid: &str) {
    let res = client.sub_acc().balance(uid).await.unwrap();
    println!("{:#?}", res.data);
}
```

---

## Error Handling

Fallible calls return the crate error enum, which exposes HTTP status helpers so
you can classify failures without depending on `reqwest` directly:

```rust
match client.margin().get_order_by_client_oid(oid, symbol).await {
    Err(e) if e.is_not_found() => { /* order does not exist */ }
    Err(e) if e.is_rate_limited() => { /* back off and retry */ }
    other => { other?; }
}
```

---
