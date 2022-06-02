# lambda-apigateway-response &emsp; [![Crates Badge]][crates.io] [![Docs Badge]][docs.rs] [![License: Apache]][Apache 2.0] [![License: MIT]][MIT]

[Crates Badge]: https://img.shields.io/crates/v/lambda-apigateway-response.svg
[crates.io]: https://crates.io/crates/lambda-apigateway-response
[Docs Badge]: https://docs.rs/lambda-apigateway-response/badge.svg
[docs.rs]: https://docs.rs/lambda-apigateway-response
[License: Apache]: https://img.shields.io/badge/License-Apache_2.0-blue.svg
[Apache 2.0]: https://opensource.org/licenses/Apache-2.0
[License: MIT]: https://img.shields.io/badge/License-MIT-yellow.svg
[MIT]: https://opensource.org/licenses/MIT

A response object for aws-lambda-rust-runtime, when the lambda function integrated into API Gateway.

# Example

```rust
use lambda_apigateway_response::{
    http::StatusCode,
    types::{
        Headers,
        MultiValueHeaders,
    },
    Response,
};
use lambda_runtime::{
    Error as LambdaError,
    LambdaEvent,
};
use serde_json::json;

type LambdaResult<T> = Result<T, LambdaError>;

async fn handler(
    _event: LambdaEvent<serde_json::Value>,
) -> LambdaResult<Response<serde_json::Value>> {
    let res = Response {
        status_code: StatusCode::OK,
        body: json!({
            "message": "Hello world!",
        }),
        headers: Headers::new(),
        multi_value_headers: MultiValueHeaders::new(),
        is_base64_encoded: true,
    };

    Ok(res)
}

#[tokio::main]
async fn main() -> LambdaResult<()> {
    let handler_fn = lambda_runtime::service_fn(handler);
    lambda_runtime::run(handler_fn).await?;

    Ok(())
}
```
