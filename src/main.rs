
use http_body_util::{Empty, Full};
use hyper::{Method, Request};
use hyper::body::Bytes;
use hyper_util::rt::{TokioExecutor, TokioIo};
use tokio::net::TcpStream;
use http_body_util::BodyExt;
use hyper_tls::HttpsConnector;
use hyper_util::client::legacy::Client;
use tokio::io::{AsyncWriteExt as _, self};
use hyper::body;
use url::Url;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {

    let https = HttpsConnector::new();
    let client = Client::builder(TokioExecutor::new()).build::<_, Empty<Bytes>>(https);
    let mut url = Url::parse("https://api.hh.ru/vacancies")?;
    url.query_pairs_mut().append_pair("text", "rust");

    let req: Request<Empty<Bytes>> = Request::builder()
        .method(Method::GET)
        .uri(url.as_str())
        .header("User-Agent", "hh-parser 1.0")
        .body(Empty::new())
        .expect("request builder");
    let res = client.request(req).await?;
    let body = res.collect().await.unwrap().to_bytes();
    println!("{}", String::from_utf8(body.to_vec()).unwrap());

    Ok(())
}
