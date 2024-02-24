use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

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

/// # 状态
/// 状态是应用内所有路由和资源共享的。状态可以通过 web::Data<T> 提取器访问，其中 T 是状态的类型。状态也可以被中间件访问
struct AppState {
    app_name: String,
}

#[get("/data")]
async fn data(data: web::Data<AppState>) -> String {
    let app_name = &data.app_name;
    format!("Hello {}!", app_name)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Hello, world!");
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            // scop：命名空间
            .service(
                web::scope("/app")
                    // 这里访问hello   http://ip:port/app/  必须以 / 结尾
                    .service(hello)
                    .service(echo)
                    // http://ip:port/app/index.html
                    .route("/index.html", web::get().to(index)),
            )
            // 添加这个路由
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
