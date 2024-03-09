use actix_web::middleware::Logger;
use actix_web::{error, post, web, App, Error, HttpRequest, HttpResponse, HttpServer, Result};
use futures::StreamExt;
use log::info;
use serde::{Deserialize, Serialize};

/// # 请求
/// 每一个应用都应该经过充分的测试。Actix Web 提供了单元测试和集成测试的工具。
/// ## 单元测试
/// 对于单元测试，actix-web 提供了一个请求构建器类型。TestRequest 实现了类似构建器的模式。
/// 你可以使用 to_http_request() 生成 HttpRequest 实例，并使用它调用你的处理器。

#[cfg(test)]
mod test {
    use super::*;
    use actix_web::{
        http::{self, header::ContentType},
        test,
    };

    #[test]
    async fn test_index_ok() {
        let req = test::TestRequest::default()
            .insert_header(ContentType::plaintext())
            .to_http_request();
        let resp = index(req).await.unwrap();
        println!("resp:{}", resp)
    }
}

async fn index(_req: HttpRequest) -> Result<String> {
    println!("index");
    Ok(format!("Welcome to the actix-web framework!"))
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
        App::new().wrap(logger).route("/", web::get().to(index))
    })
    .bind("localhost:8080")?
    .run()
    .await
}
