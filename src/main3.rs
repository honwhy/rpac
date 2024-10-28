use hyper::{Body, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn};
use std::convert::Infallible;
use hyper::client::Client as HyperClient;

async fn handle_request(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    // Process the incoming request here
    // For this example, we'll just return a simple response
    // 解析请求URL
    let uri = req.uri().to_string();
    println!("Handling request to {}", uri);
    // // 根据PAC规则决定如何处理请求
    // match pac_rules(&uri) {
    //     Some(proxy_url) => {
    //         // 转发到指定代理
    //         forward_to_proxy(req, &proxy_url).await
    //     },
    //     None => {
    //         // 直接发送请求
    //         send_direct(req).await
    //     },
    // }
    Ok(Response::new(Body::from("Hello, Rust HTTP Server!")))
}
// async fn forward_to_proxy(req: Request<Body>, proxy_url: &str) -> Result<Response<Body>, hyper::Error> {
//     let client = HyperClient::new();
//     let response = client.request(req.method().clone(), proxy_url)
//                          .body(Body::from(req.into_body()))
//                          .send()
//                          .await?;
//     Ok(response.into_response())
// }

// async fn send_direct(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
//     let client = HyperClient::new();
//     let response = client.request(req.method().clone(), req.uri().to_string())
//                          .body(req.into_body())
//                          .await?;
//     Ok(response)
// }

fn pac_rules(url: &str) -> Option<String> {
    // 这里是你的PAC规则实现
    // 示例：如果请求的是特定域名，则返回代理地址
    if url.contains("example.com") {
        Some("http://proxy.example.com".to_string())
    } else {
        None
    }
}
#[tokio::main]
async fn main() {
    let make_svc = make_service_fn(|_conn| {
        async {
            Ok::<_, Infallible>(service_fn(handle_request))
        }
    });

    let addr = ([127, 0, 0, 1], 3000).into();
    let server = Server::bind(&addr).serve(make_svc);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}