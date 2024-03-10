use std::{io, sync::Mutex};

#[path = "../handlers.rs"]
mod handlers;

#[path = "../router.rs"]
mod router;

#[path = "../state.rs"]
mod state;

#[path = "../models.rs"]
mod models;

use actix_web::{web, App, HttpServer};
use router::*;
use state::AppState;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    let shared_data = web::Data::new(AppState {
        health_check_response: "I'm OK".to_string(),
        visit_count: Mutex::new(0),
        courses: Mutex::new(vec![])
    });

    let app = move || {
        App::new()
            .app_data(shared_data.clone())
            // curl localhost:3000/health
            .configure(general_routes)
            // 
            // 添加课程：curl -X POST localhost:3000/courses/ -H "Content-Type:application/json" -d '{"teacher_id":3,"name":"First course"}'
            // 获取老师所有的课程：curl localhost:3000/courses/1
            // 获取老师的某一课程： curl localhost:3000/courses/1/2
            .configure(course_routes)
    };

    HttpServer::new(app).bind("127.0.0.1:3000")?.run().await
}
