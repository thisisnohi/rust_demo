use actix_web::middleware::Logger;
use actix_web::{error, post, web, App, Error, HttpResponse, HttpServer, Result};
use futures::StreamExt;
use log::info;
use serde::{Deserialize, Serialize};

/// # 请求
/// ## JSON 请求

#[derive(Deserialize, Debug)]
struct Info {
    username: String,
}

async fn index(info: web::Json<Info>) -> Result<String> {
    println!("index");
    info!("Info: {:?}", info);
    Ok(format!("Welcome {}!", info.username))
}

// 将请求加载到内在中，然后反序列化
#[derive(Serialize, Deserialize)]
struct MyObj {
    name: String,
    number: i32,
}

const MAX_SIZE: usize = 262_144;

#[post("/index")]
async fn index_manual(mut payload: web::Payload) -> Result<HttpResponse, Error> {
    println!("println index_manual");
    info!("info index_manual");

    // payload is a stream of Bytes objects
    let mut body = web::BytesMut::new();
    while let Some(chunk) = payload.next().await {
        let chunk = chunk?;
        // limit max size of in-memory payload
        if (body.len() + chunk.len()) > MAX_SIZE {
            return Err(error::ErrorBadRequest("overflow"));
        }
        body.extend_from_slice(&chunk);
    }
    // body is loaded, now we can deserialize serde-json
    let obj = serde_json::from_slice::<MyObj>(&body)?;
    Ok(HttpResponse::Ok().json(obj))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 设置日志参数
    std::env::set_var("RUST_LOG", "info");
    std::env::set_var("RUST_BACKTRACE", "1");
    // 日期初始化
    env_logger::init();

    // 启动应用
    HttpServer::new(|| {
        // 加载日志
        let logger = Logger::default();
        // JsonConfig 配置
        let json_config = web::JsonConfig::default()
            // 限制4096byte 4kB
            .limit(4096)
            .error_handler(|err, _req| {
                error::InternalError::from_response(err, HttpResponse::Conflict().finish()).into()
            });

        App::new()
            .wrap(logger)
            .app_data(json_config)
            // `curl -X POST -H "Content-Type:application/json"  http://127.0.0.1:8080 -d '{"username":"111"}'`
            .route("/", web::post().to(index))
            // `curl -X POST -H "Content-Type:application/json"  http://127.0.0.1:8080/index -d '{"name":"Jad", "number": 128}'`
            .service(index_manual)
        // .default_service(
        //     web::route()
        //         .guard(guard::Not(guard::Get()))
        //         .to(HttpResponse::MethodNotAllowed),
        // )
    })
    .bind("localhost:8080")?
    .run()
    .await
}
