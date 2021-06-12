# MC-Market Rust API Wrapper
[![Documentation Status](https://img.shields.io/badge/docs-0.1.0-4d76ae)](https://majored.pw/docs)
[![GitHub license](https://img.shields.io/badge/license-MIT-007ec6)](https://github.com/Majored/mcm-rust-api-wrapper/blob/main/LICENSE)

An (in-progress) asynchronous Rust wrapper for MC-Market's HTTP API (https://www.mc-market.org/wiki/ultimate-api/).

* Built on [reqwest](https://github.com/seanmonstar/reqwest)/[hyper](https://github.com/hyperium/hyper) - a fast and correct HTTP implementation.
* Full coverage of the API with a fully asynchronous design using the [tokio](https://github.com/tokio-rs/tokio) runtime.
* Requests are queued and may be dynamically delayed to stay within rate limiting rules.
* Pre-generated and hosted `rustdoc` documentation.

## Installation & Usage
As Cargo allows the inclusion of dependencies directly from Git repsositories, doing so is likely the easiest way to get started with this wrapper:
```Toml
[dependencies]
mcm-rust-api-wrapper = { git = "https://github.com/Majored/mcm-rust-api-wrapper" }
```

Alternatively, cloning this repository locally and directing Cargo to it would also be a relatively painless way to get started:
```Toml
[dependencies]
mcm-rust-api-wrapper = { path = "../mcm-rust-api-wrapper" }
```

---

```Rust
use mcm_rust_api_wrapper::{APIWrapper, APIToken};
...

let token = APIToken::Private(String::from("y6xWrGkAzh8Gp4qBWFMG7tDyB+zB+Lub"));
    
let wrapper = APIWrapper::build(token).await.unwrap();
let member = wrapper.fetch_member(87939).await.unwrap();

assert_eq!("Harry", member.username());
```

## Issues & Support
Whether you're wanting to report a bug you've come across during use of this wrapper or are seeking general help/assistance, please utilise the [issues tracker](https://github.com/Majored/mcm-rust-api-wrapper/issues) and tag your issue appropriately during creation.

I try to respond to issues as fast as possible.
