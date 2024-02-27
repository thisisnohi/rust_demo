use actix_web::{App, error, guard, HttpResponse, HttpServer, web};
use actix_web::middleware::Logger;
use log::info;

/// 路由分发
/// App::route() 方法提供了注册路由的简单方式

async fn index() -> HttpResponse{
    info!("index");
    HttpResponse::Ok().body("hello world")
}

async fn get_user() -> HttpResponse{
    info!("get_user");

    HttpResponse::Ok().body("User NOHI")
}

/// 路由配置
/// ResourceHandler::route() 返回一个 Route 对象。路由可以使用类似构建器的模式进行配置。配置方法如下：
///      Route::guard() 注册一个导航守卫。每一个路由都可以注册任意数量的守卫。
///      Route::method() 注册一个请求方法。每个路由都可以注册任意数量的请求方法。
///      Route::to() 注册一个异步的处理函数。只能注册一个处理函数，通常最后一个注册的处理函数会生效。

/// 资源 pattern 语法
///  1. {foo}/bar/baz  == /{foo}/bar/baz
///  2. foo/{baz}/{bar}
///     匹配：
///         foo/1/2        -> Params {'baz': '1', 'bar': '2'}
///         foo/abc/def    -> Params {'baz': 'abc', 'bar': 'def'}
///     不匹配：
///        foo/1/2/        -> No match (trailing slash)
///        bar/abc/def     -> First segment literal mismatch
///  3.  foo/{name}.html  /foo/biz.html   匹配结果将是 Params {'name': 'biz'}
///  4.  foo/{name}.{ext} 路径 /foo/biz.html 将会与上面的模式匹配，匹配结果是 Params {'name': 'biz', 'ext': 'html'}。
///  5. 尾部匹配: foo/{bar}/{tail:.*}
///       匹配： foo/1/2/           -> Params {'bar': '1', 'tail': '2/'}
///             foo/abc/def/a/b/c  -> Params {'bar': 'abc', 'tail': 'def/a/b/c'}


/// 路由作用域
/// /users
/// /users/show
/// /users/show/{id}
///   App::new().service(
///             web::scope("/users")
///                 .service(show_users)
///                 .service(user_detail),
///         )
///

/// 匹配信息
///


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

        App::new().wrap(logger).app_data(json_config)
            .route("/", web::get().to(index))
            .service(
                web::resource("/user/{name}")
                    .guard(guard::Header("content-type", "application/json"))
                    .route(web::get().to(get_user))
                    .route(web::put().to(HttpResponse::Ok))
            ).default_service(web::route().guard(guard::Not(guard::Get()))
            .to(HttpResponse::MethodNotAllowed))
            // 使用如下命令可以查看结果
            // curl -v  -X GET -H "Content-Type:application/json" http://localhost:8080/us/1O
            // Note: Unnecessary use of -X or --request, GET is already inferred.
            // *   Trying 127.0.0.1:8080...
            // * Connected to localhost (127.0.0.1) port 8080 (#0)
            // > GET /us/1O HTTP/1.1
            // > Host: localhost:8080
            // > User-Agent: curl/7.78.0
            // > Accept: */*
            // > Content-Type:application/json
            // >
            // * Mark bundle as not supporting multiuse
            // < HTTP/1.1 405 Method Not Allowed
            // < content-length: 0
            // < date: Tue, 27 Feb 2024 14:04:01 GMT
            // <
            // * Connection #0 to host localhost left intact
    })
    .bind("localhost:8080")?
    .run()
    .await
}
