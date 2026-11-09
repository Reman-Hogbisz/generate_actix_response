# generate_actix_response

JSON response macros for Actix Web. The crate owns its response envelope and has no dependency on an application's models crate. It remains Actix-specific.

## Use

```toml
[dependencies]
generate_actix_response = { git = "https://github.com/Reman-Hogbisz/generate_actix_response", branch = "main" }
```

```rust
use generate_actix_response::{generate_ok_json_response, generate_not_found_json_response_with_message};

let response = generate_ok_json_response!(vec!["hello"]);
// HTTP 200: {"status":200,"message":"OK","data":["hello"]}
let response = generate_not_found_json_response_with_message!("Record missing");
// HTTP 404: {"status":404,"message":"Record missing"}
```

All existing status-specific macro names are retained. Each `generate_<status>_json_response!` accepts either no arguments or a data value. Each `generate_<status>_json_response_with_message!` accepts a message and optional data. `make_json_response!` accepts a numeric status, optional message, and optional data. `generate_response_content!` accepts the same arguments and returns `(StatusCode, StandardResponse<T>)` instead of an HTTP response. Invalid numeric statuses panic; negative or overflowing values are rejected rather than truncated.

`StandardResponse<T>` exposes `status: i32`, `message: String`, and `data: Option<T>`; absent data is omitted from JSON. Payloads only need `serde::Serialize` when generating HTTP responses. Enable the optional `utoipa` feature for OpenAPI schema support.

Macros resolve Actix and the response type through this crate, so consumers do not need direct Actix or Serde dependencies just to invoke them. Renaming this dependency in Cargo.toml is supported.

## Compatibility

Existing response macro names and JSON shapes are preserved. `generate_response_content!` returns this crate's `StandardResponse<T>`.

Crates.io publishing is disabled pending a release and licensing decision.

## Validate

```sh
cargo fmt --all -- --check
cargo build --locked --all-features
cargo test --locked
cargo test --locked --all-features
cargo clippy --locked --all-targets --all-features -- -D warnings
cargo run --locked --manifest-path tests/renamed/Cargo.toml
```
