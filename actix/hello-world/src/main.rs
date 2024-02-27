use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use std::sync::Mutex;

// 路由宏方式注册
#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello, world!")
}

// 路由宏方式注册
#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    println!("{}", req_body);
    HttpResponse::Ok().body(req_body)
}

// 采用手工注册，而不是路由宏方式注册
async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

async fn index() -> impl Responder {
    "Hello world! \nthis is index"
}

/// ## 状态
/// 状态是应用内所有路由和资源共享的。状态可以通过 web::Data<T> 提取器访问，其中 T 是状态的类型。状态也可以被中间件访问
struct AppState {
    app_name: String,
}

#[get("/data")]
async fn data(data: web::Data<AppState>) -> String {
    let app_name = &data.app_name;
    format!("Hello {}!", app_name)
}

/// ## 共享可变状态
struct AppStateWithCounter {
    counter: Mutex<i32>,
}

async fn counter(dd: web::Data<AppStateWithCounter>) -> String {
    let mut counter = dd.counter.lock().unwrap();
    *counter += 1;
    format!("Counter: {}", counter)
}

/// ## 配置
fn scoped_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/test")
            .route(web::get().to(|| async { HttpResponse::Ok().body("test") }))
            .route(web::head().to(HttpResponse::MethodNotAllowed)),
    );
}

fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/app")
            .route(web::get().to(|| async { HttpResponse::Ok().body("app") }))
            .route(web::head().to(HttpResponse::MethodNotAllowed)),
    );
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Hello, world!");
    // 计数
    let count = web::Data::new(AppStateWithCounter {
        counter: Mutex::new(0),
    });

    HttpServer::new(move || {
        App::new()
            .service(hello)
            .service(echo)
            // scop：命名空间
            .service(
                // 路由作用域
                web::scope("/scop")
                    // 这里访问hello   http://ip:port/scop/  必须以 / 结尾
                    .service(hello)
                    .service(echo)
                    // http://ip:port/app/index.html
                    .route("/index.html", web::get().to(index)),
            )
            // 添加这个路由
            .route("/hey", web::get().to(manual_hello))
            // 将状态传递给应用并启动应用：
            .app_data(web::Data::new(AppState {
                app_name: String::from("actix-web"),
            }))
            // http://127.0.0.1:8080/data
            .service(data)
            // 共享可变状态
            .app_data(count.clone())
            // http://127.0.0.1:8080/counter
            .route("/counter", web::get().to(counter))
            // 配置
            .configure(config)
            .service(web::scope("/api"))
            .configure(scoped_config)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
