use actix_cors::Cors;
use sqlx::postgres::PgPoolOptions;
use std::{env, io, sync::Mutex};

#[path = "../handles/mod.rs"]
mod handlers;

#[path = "../router.rs"]
mod router;

#[path = "../state.rs"]
mod state;

#[path = "../models/mod.rs"]
mod models;

#[path = "../dbaccess/mod.rs"]
mod db_access;

#[path = "../errors.rs"]
mod errors;


use actix_web::{http, web, App, HttpServer};
use dotenv::dotenv;
use router::*;
use state::AppState;

///
/// 运行`cargo run --bin teacher-service`
#[actix_rt::main]
async fn main() -> io::Result<()> {
    // 加载环境变量配置
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL 没有在 .env文件里设置");

    let db_pool: sqlx::Pool<sqlx::Postgres> =
        PgPoolOptions::new().connect(&database_url).await.unwrap();

    let shared_data = web::Data::new(AppState {
        health_check_response: "I'm OK".to_string(),
        visit_count: Mutex::new(0),
        // courses: Mutex::new(vec![]),
        db: db_pool,
    });

    let app = move || {
        // 支持跨域
        let cors = Cors::default()
           .allowed_origin("http://localhost:8080;")
        .allowed_origin_fn(|origin, _req_head| {
            origin.as_bytes().starts_with(b"http://localhost")
        }).allowed_methods(vec!["GET", "POST"])
        .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
        .allowed_header(http::header::CONTENT_TYPE)
        .max_age(36000);

    

        App::new()
            .app_data(shared_data.clone())
            // curl localhost:3000/health
            .configure(general_routes)
            //
            // 添加课程：curl -X POST localhost:3000/courses/ -H "Content-Type:application/json" -d '{"teacher_id":3,"name":"First course"}'
            // 获取老师所有的课程：curl localhost:3000/courses/1
            // 获取老师的某一课程： curl localhost:3000/courses/1/2
            .configure(course_routes)

            // 教师管理服务
            // 获取所有课程： curl localhost:3000/teacher/
            // 添加课程： curl -X POST localhost:3000/teacher/ -H "Content-Type:application/json" -d '{"picture_url": "purl","name":"First course", "profile": "profile"}'
            // 获取课程详情: curl localhost:3000/teacher/1
            // 更新课程详情: curl -X PUT localhost:3000/teacher/1 -H "Content-Type:application/json" -d '{"picture_url": "purl","name":"First course", "profile": "还是原来的程集小学吗?"}'
            // 删除课程： curl -X DELETE localhost:3000/teacher/3
            .wrap(cors)
            .configure(teacher_routes)
    };

    HttpServer::new(app).bind("127.0.0.1:3000")?.run().await
}
