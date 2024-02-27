use std::io;
use std::io::{Error, ErrorKind};

use actix_files::NamedFile;
use actix_web::body::BoxBody;
use actix_web::http::header::ContentType;
use actix_web::http::StatusCode;
use actix_web::{error, get, web, App, HttpRequest, HttpResponse, HttpServer};
use actix_web::middleware::Logger;
use derive_more::{Display, Error};
use log::{debug, info, warn};

/// 错误处理
fn index(_req: HttpRequest) -> io::Result<NamedFile> {
    Ok(NamedFile::open("static/index.html")?)
}

/// 自定义错误响应示例

#[derive(Debug, Display, Error)]
#[display(fmt = "my error: {}", name)]
struct MyError {
    name: &'static str,
}

impl error::ResponseError for MyError {}

/// 自定义异常
#[get("/my-error")]
async fn my_error() -> Result<&'static str, MyError> {
    Err(MyError { name: "test" })
}

/// 自定义异常2
#[derive(Debug, Display, Error)]
enum MyError2 {
    #[display(fmt = "internal error")]
    InternalError,

    #[display(fmt = "bad request")]
    BadClientData,

    #[display(fmt = "Timeout")]
    Timeout,
}

impl error::ResponseError for MyError2 {
    fn status_code(&self) -> StatusCode {
        info!("status_code1: {}", self);
        println!("status_code2: {}", self);
        match self {
            MyError2::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            MyError2::BadClientData => StatusCode::BAD_REQUEST,
            MyError2::Timeout => StatusCode::GATEWAY_TIMEOUT,
        }
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        info!("error_response1:{}", self.status_code());
        println!("error_response2:{}", self.status_code());
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }
}

#[get("/my-error-type")]
async fn my_error_type() -> Result<&'static str, MyError2> {
    Err(MyError2::BadClientData)
}

/// 推荐的错误处理方式
///  面向用户的错误

#[derive(Debug, Display, Error)]
enum UserError {
    #[display(fmt = "validation error on field {}", field)]
    ValidationError { field: String },
    // 内部错误，隐藏细节后返回用户
    #[display(fmt = "An internal error occurred. Please try again later.")]
    InternalError,
}

impl error::ResponseError for UserError {
    fn status_code(&self) -> StatusCode {
        info!("info status_code1: {}", self);
        println!("println status_code2: {}", self);
        match self {
            UserError::ValidationError{ field} => StatusCode::BAD_REQUEST,
            UserError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        info!("info error_response1:{}", self.status_code());
        println!("println error_response2:{}", self.status_code());
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }
}

/// 错误辅助函数
/// 使用 map_err 将未实现 ResponseError 特质的 MyError 转换为 400（错误请求）
#[get("/err-fn")]
async fn err_fn() -> actix_web::Result<String> {
    println!("======err_fn==========");
    warn!(
        "warn err_fn1: {}",
        MyError {
            name: "错误辅助函数"
        }
    );
    info!(
        "info err_fn1: {}",
        MyError {
            name: "错误辅助函数"
        }
    );
    let err = Err(MyError {
        name: "错误辅助函数",
    });
    debug!(
        "debug err_fn1: {}",
        MyError {
            name: "错误辅助函数"
        }
    );
    // 返回
    err.map_err(|e| error::ErrorBadRequest(e))
}

/// 推荐的错误处理方式
#[get("/user-err")]
async fn user_err() -> Result<&'static str, UserError> {
    println!("user-err");
    do_thing_that_fails().map_err(|_e| UserError::InternalError)?;
    println!("没有出现异常????");
    Ok("success")
}

fn do_thing_that_fails() -> Result<(), std::io::Error> {
    return Err( Error::new(ErrorKind::Other, "Something was wrong"))
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

        App::new().wrap(logger).app_data(json_config).service(
            web::scope("/error")
                // http://127.0.0.1:8080/error/my-error
                .service(my_error)
                // http://127.0.0.1:8080/error/my-error-type
                .service(my_error_type)
                // http://127.0.0.1:8080/error/err-fn
                .service(err_fn)
                // http://127.0.0.1:8080/error/user-err
                .service(user_err),
        )
    })
    .bind("localhost:8080")?
    .run()
    .await
}
