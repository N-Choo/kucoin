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

- built on `async` and `reqwest`
- **Spot trading**
  - Market & limit orders
  - Batch orders
  - Order cancellation
  - Retrieve open orders
  - OCO orders (take-profit + stop-loss)

- **Wallet & deposits**
  - Deposit history with filters
  - Lookup by transaction hash


- **Sub-accounts**
  - List sub-accounts
  - Create API keys
  - Configure permissions & IP whitelists
  - Query sub-account balances


- **Typed requests**
  - Builder-pattern request structs
  - Compile-time parameter validation


---

## Installation

```toml
[dependencies]
kucoin = "0.4.0"
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

## Deposits

```rust
use kucoin::types::deposit::{DepositHistoryRequest, DepositStatus};

async fn deposits(client: &KuCoinClient) {
    let req = DepositHistoryRequest::new("SOL")
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
        .set_expire(Expire::Never);

    client.sub_account().add_api(req).await.unwrap();
}
```

```rust
async fn list_subs(client: &KuCoinClient) {
    let res = client.sub_account().fetchall().await.unwrap();
    println!("{:#?}", res.data);
}
```

```rust
async fn balance(client: &KuCoinClient, uid: &str) {
    let res = client.sub_account().balance(uid).await.unwrap();
    println!("{:#?}", res.data);
}
```

---


## 🌍 Production Usage

This crate serves as the production trading core for **CoinBot**, an automated trading platform.

- High-frequency workloads
- Real-time order execution
- Enterprise sub-account orchestration

Project: https://coinbot.locker

---


<div align="center">
<sub>Built with precision in Rust 🦀 Contributions are welcome </sub>
</div>
