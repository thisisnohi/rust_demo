use std::ops::Deref;
use actix_web::{error, get, post, web, App, HttpResponse, HttpServer, Result};
use serde::Deserialize;

/// Query（查询参数提取器）
/// Query<T> 提供了请求查询参数的提取功能。它底层使用了 serde_urlencoded crate。

#[derive(Deserialize, Debug)]
struct Info {
    username: String,
}

#[get("/")]
async fn index(info: web::Query<Info>) -> String {
    println!("{:?}", info);
    format!("Welcom {}!", info.username)
}

/// JSON（JSON格式的数据提取器）
/// `curl -X POST -H "Content-Type:application/json" http://127.0.0.1:8080/submit -d '{"username": "NOHI"}'`
#[post("/submit")]
async fn submit(info: web::Json<Info>) -> Result<String> {
    Ok(format!("Welcom {}!", info.username))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 启动应用
    HttpServer::new(|| {
        // JsonConfig 配置
        let json_config = web::JsonConfig::default()
            // 限制4096byte 4kB
            .limit(4096)
            .error_handler(|err, _req| {
                error::InternalError::from_response(err, HttpResponse::Conflict().finish()).into()
            });

        App::new().app_data(json_config).service(index).service(submit)
    })
    .bind("localhost:8080")?
    .run()
    .await
}
