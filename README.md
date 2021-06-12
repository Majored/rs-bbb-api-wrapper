# -mcm-rust-api-wrapper
A Rust wrapper for MC-Market's HTTP API.

## Example
```
use mcm_rust_api_wrapper::{APIToken, APIWrapper};
...

let token = APIToken::Private(String::from("y6xWrGkAzh8Gp4qBWFMG7tDyB+zB+Lub"));
    
let wrapper = match APIWrapper::build(token).await {
    Ok(wrapper) => wrapper,
    Err(error) => {
        println!("Error: {:?}", error);
        std::process::exit(0);
    }
};

println!("Connected!");
```