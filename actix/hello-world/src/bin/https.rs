use actix_web::{get, App, HttpRequest, HttpServer, Responder};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

/// 测试https

#[get("/")]
async fn index(_req: HttpRequest) -> impl Responder {
    "Welcome to the HTTPS server!"
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
