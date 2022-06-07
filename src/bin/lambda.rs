use lambda_http::{run, service_fn, Error, IntoResponse, Request, RequestExt, Response};
use dioxus::prelude::*;
use dioxus::ssr;
use brein_rs::recipe::Recipe;
use brein_rs::recipes::Recipes;
use brein_rs::nav_bar::NavBar;

pub fn Foo(cx: Scope) -> Element {
    cx.render(rsx! { "foobar" })
}

pub fn SampleComponent(cx: Scope) -> Element {
  cx.render(rsx! { 
      Recipes {},
      NavBar {},
    })
}

/// This is the main body for the function.
/// Write your code inside it.
/// There are some code example in the following URLs:
/// - https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/lambda-http/examples
async fn function_handler(event: Request) -> Result<impl IntoResponse, Error> {
    // Extract some useful information from the request
    let mut app = VirtualDom::new(SampleComponent);
    let _ = app.rebuild();
    let body = dioxus::ssr::render_vdom(&app);

    // Return something that implements IntoResponse.
    // It will be serialized to the right response event automatically by the runtime
    let resp = Response::builder()
        .status(200)
        .header("content-type", "text/html")
        .body(body)
        .map_err(Box::new)?;
    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    run(service_fn(function_handler)).await
}
