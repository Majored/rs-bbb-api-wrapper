# mcm_api_wrapper
[![GitHub license](https://img.shields.io/badge/license-MIT-007ec6)](https://github.com/Majored/rs-mcm-api-wrapper/blob/main/LICENSE)
[![Crates.io](https://img.shields.io/crates/v/mcm_api_wrapper)](https://crates.io/crates/mcm_api_wrapper)
[![docs.rs](https://img.shields.io/docsrs/mcm_api_wrapper)](https://docs.rs/mcm_api_wrapper/)

An asynchronous Rust wrapper for MC-Market's [HTTP API](https://www.mc-market.org/wiki/ultimate-api/).

- Built on [reqwest](https://github.com/seanmonstar/reqwest)/[hyper](https://github.com/hyperium/hyper) - a fast and correct HTTP implementation.
- Full coverage of the API with a fully asynchronous design using the [tokio](https://github.com/tokio-rs/tokio) runtime.
- Requests are queued and may be dynamically delayed to stay within rate limiting rules.

## Installation & Usage

```toml
[dependencies]
mcm_api_wrapper = "1.0.0"
```

An extensive list of [examples](https://github.com/Majored/rs-mcm-api-wrapper/tree/main/examples) can be found under the `/examples` directory.

```Rust
use mcm_rust_api_wrapper::{APIWrapper, APIToken};
...

let token = APIToken::Private(String::from("y6xWrGkAzh8Gp4qBWFMG7tDyB+zB+Lub"));
    
let wrapper = APIWrapper::new(token).await.unwrap();
let member = wrapper.members().fetch(87939).await.unwrap();

assert_eq!("Harry", member.username());
```

## Issues & Support
Whether you're wanting to report a bug you've come across during use of this wrapper or are seeking general help/assistance, please utilise the [issues tracker](https://github.com/Majored/rs-mcm-api-wrapper/issues) and tag your issue appropriately during creation.

I try to respond to issues within a reasonable timeframe.
