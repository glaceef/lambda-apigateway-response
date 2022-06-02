//! A response object for aws-lambda-rust-runtime, when the lambda function integrated into API Gateway.
//! 
//! # Example
//! 
//! ```rust
//! use lambda_apigateway_response::{
//!     http::StatusCode,
//!     types::{
//!         Headers,
//!         MultiValueHeaders,
//!     },
//!     Response,
//! };
//! use lambda_runtime::{
//!     Error as LambdaError,
//!     LambdaEvent,
//! };
//! use serde_json::json;
//! 
//! type LambdaResult<T> = Result<T, LambdaError>;
//! 
//! async fn handler(
//!     _event: LambdaEvent<serde_json::Value>,
//! ) -> LambdaResult<Response<serde_json::Value>> {
//!     let res = Response {
//!         status_code: StatusCode::OK,
//!         body: json!({
//!             "message": "Hello world!",
//!         }),
//!         headers: Headers::new(),
//!         multi_value_headers: MultiValueHeaders::new(),
//!         is_base64_encoded: true,
//!     };
//! 
//!     Ok(res)
//! }
//! 
//! #[tokio::main]
//! async fn main() -> LambdaResult<()> {
//!     let handler_fn = lambda_runtime::service_fn(handler);
//!     lambda_runtime::run(handler_fn).await?;
//! 
//!     Ok(())
//! }
//! ```

pub use http;

pub mod response;
pub mod types;

pub use response::Response;
pub use types::*;
