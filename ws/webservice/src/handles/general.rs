use actix_web::{web, HttpResponse};

use crate::{db_access::course::get_all_courses_db, state::AppState};


pub async fn health_check_handler(app_state: web::Data<AppState>) -> HttpResponse {
    let health_check_response = &app_state.health_check_response;
    let mut visit_count = app_state.visit_count.lock().unwrap();

    // 使用数据库操作
    let course = get_all_courses_db(&app_state.db).await.unwrap();

    let response = format!(
        "{} {} times  course: {:?}",
        health_check_response, visit_count, course
    );

    *visit_count += 1;

    // 返回
    HttpResponse::Ok().json(&response)
}