use anyhow::Result;
use spin_analytics::http_component_with_analytics;
use spin_sdk::{
    http::{Request, Response},
    http_component,
};
/// A simple Spin HTTP component.
#[http_component_with_analytics]
fn handle_http_rust_example(_: Request) -> Result<Response> {
    Ok(http::Response::builder()
        .status(200)
        .header("foo", "bar")
        .body(Some("Hello, Fermyon".into()))?)
}
