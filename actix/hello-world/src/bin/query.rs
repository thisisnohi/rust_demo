use actix_web::{get, post, web, App, HttpServer, Responder, Result, HttpRequest};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use serde::Deserialize;

/// 数据提取器

#[derive(Deserialize)]
struct MyInfo {
    id: u32,
    username: String,
}

#[post("/index")]
async fn index(path: web::Path<(String, String)>, json: web::Json<MyInfo>) -> impl Responder {
    let path = path.into_inner();
    format!("{} {} {} {}", path.0, path.1, json.id, json.username)
}

/// Path（路径提取器）
/// Path 可以从请求路径中提取信息。可以提取的路径部分称为“动态段”，并用花括号标记。您可以从路径中反序列化任何变量段。
/// 浏览器：https://127.0.0.1:8080/users/1/nohi
/// 命令行：curl -k https://127.0.0.1:8080/users/1/nohi
#[get("/users/{user_id}/{friend}")]
async fn user_friend(path: web::Path<(u32, String)>) -> Result<String> {
    let (user_id, friend) = path.into_inner();
    Ok(format!("Welcom {}, user_id {}!", friend, user_id))
}
/// 可以将path提到取实现了serde的Deserialize物质的struct中
#[derive(Deserialize,Debug)]
struct PathInfo {
    user_id: u32,
    friend: String,
}
#[get("/users2/{user_id}/{friend}")]
async fn user_friend2(path: web::Path<PathInfo>) -> Result<String> {
    println!("Path :{:?}", path);
    Ok(format!("Welcom {}, user_id {}!", path.friend, path.user_id))
}

/// 在处理函数中按名称查询，参见 match_info 文档
#[get("/users3/{user_id}/{friend}")]
async fn user_friend3(req: HttpRequest) -> Result<String> {
    let name = req.match_info().get("friend").unwrap();
    let user_id = req.match_info().get("user_id").unwrap();
    println!(" user_id: {},name :{}", user_id, name);
    Ok(format!("Welcom {}, user_id {}!", name, user_id))
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
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(user_friend)
            .service(user_friend2)
            .service(user_friend3)
    })
    .bind_openssl("localhost:8080", builder)?
    .run()
    .await
}
