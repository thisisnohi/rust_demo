use actix_web::{get, App, HttpRequest, HttpServer, Responder, web, post};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use serde::{Deserialize};

/// 数据提取器


#[derive(Deserialize)]
struct MyInfo{
    id: u32,
    username: String,
}


#[post("/index")]
async fn index(path: web::Path<(String, String)>, json: web::Json<MyInfo>) -> impl Responder {
    let path = path.into_inner();
    format!("{} {} {} {}", path.0, path.1, json.id, json.username)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 创建自签名证书
    // `openssl req -x509 -newkey rsa:4096 -nodes -keyout key.pem -out cert.pem -days 365 -subj '/CN=localhost'`

    // 加载TLS keys
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file("key.pem", SslFiletype::PEM)
        .unwrap();
    builder.set_certificate_chain_file("cert.pem").unwrap();

    // 启动应用
    HttpServer::new(|| App::new().service(index))
        .bind_openssl("localhost:8080", builder)?
        .run()
        .await
}
