# Rust client for the Bondora API

Bondora API version 1

## Overview

source: https://api.bondora.com/swagger/docs/v1

## Usage

```rust
use bondora::apis::APIClient;
use bondora::apis::AccountApi;
use bondora::models::AuctionRequest;

#[tokio::main]
async fn main() {
    let token = "ZACK0HC58KAOkGCyM3W932YHh2C3h4cr5WqHm2Y076Ek0xRa".to_string();
    let client = APIClient::new(token);

    let result = client.account_get_balance().await.unwrap();
    println!("{:?}", result);
}
```

## Generate your API key

Head over to [https://api.bondora.com/Application](https://api.bondora.com/Application) and register you App with Bondora. After taht you can generate an API key within the UI.