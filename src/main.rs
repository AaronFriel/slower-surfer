#[async_std::main]
async fn main() {
    async_std::task::spawn(async {
        run_handler(8001).await.unwrap();
    });

    let mut value: String = "0".to_string();
    loop {
        value = call_handler(value).await.unwrap();
    }
}

async fn call_handler(
    request_body: String,
) -> Result<String, Box<dyn std::error::Error + Send + Sync + 'static>> {
    let now = std::time::Instant::now();
    let mut res = surf::post("http://127.0.0.1:8001/")
        .body_string(dbg!(request_body))
        .set_header("content-type", "bar")
        .await?;
    let response_body = res.body_string().await?;
    dbg!(now.elapsed().as_millis());

    Ok(dbg!(response_body))
}

async fn run_handler(port: u16) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let mut server = tide::new();
    server.at("/").post(adding_handler_wrapper);
    let addr = format!("127.0.0.1:{}", port);
    server.listen(addr).await?;

    Ok(())
}

async fn adding_handler_wrapper(req: tide::Request<()>) -> tide::Response {
    match adding_handler(req).await {
        Ok(res) => res,
        Err(_) => tide::Response::new(500),
    }
}

async fn adding_handler(
    mut req: tide::Request<()>,
) -> Result<tide::Response, Box<dyn std::error::Error + Send + Sync + 'static>> {
    let request_body = req.body_string().await?;
    let content_type = req.header("content-type").unwrap_or("foo");

    let value: i32 = request_body.parse()?;

    let response_body = (value + 1).to_string();

    Ok(tide::Response::new(200)
        .body_string(response_body)
        .set_header("content-type", content_type))
}
