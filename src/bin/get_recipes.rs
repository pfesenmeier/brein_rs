use lambda_http::{run, service_fn, Error, IntoResponse, Request, RequestExt, Response};

/// There are some code example in the following URLs:
/// - https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/lambda-http/examples
async fn function_handler(event: Request) -> Result<impl IntoResponse, Error> {
    // Return something that implements IntoResponse.
    // It will be serialized to the right response event automatically by the runtime
    let get_recipes_bucket_name = env!("GET_RECIPES_BUCKET_NAME");
    let resp = Response::builder()
        .status(200)
        .header("content-type", "text/html")
        .body("hello world")
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
