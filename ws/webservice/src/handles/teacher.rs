use actix_web::{web, HttpResponse};

use crate::{
    db_access::teacher::{
        delete_teacher_db, get_all_teachers_db, get_teacher_detail_db, post_teacher_db,
        update_teacher_detail_db,
    },
    errors::MyError,
    models::teacher::{CreateTeacher, UpdateTeacher},
    state::AppState,
};

pub async fn get_all_teacher(app_state: web::Data<AppState>) -> Result<HttpResponse, MyError> {
    get_all_teachers_db(&app_state.db)
        .await
        .map(|teachers| HttpResponse::Ok().json(teachers))
}

pub async fn get_teacher_details(
    app_state: web::Data<AppState>,
    path: web::Path<i32>,
) -> Result<HttpResponse, MyError> {
    let teacher_id = path.into_inner();

    get_teacher_detail_db(&app_state.db, teacher_id.into())
        .await
        .map(|teacher| HttpResponse::Ok().json(teacher))
}

pub async fn post_new_teacher(
    app_state: web::Data<AppState>,
    create_teacher: web::Json<CreateTeacher>,
) -> Result<HttpResponse, MyError> {
    post_teacher_db(&app_state.db, CreateTeacher::from(create_teacher))
        .await
        .map(|teacher| HttpResponse::Ok().json(teacher))
}

pub async fn update_teacher_detail(
    app_state: web::Data<AppState>,
    path: web::Path<i32>,
    update_teacher: web::Json<UpdateTeacher>,
) -> Result<HttpResponse, MyError> {
    let teacher_id = path.into_inner();

    update_teacher_detail_db(
        &app_state.db,
        teacher_id,
        UpdateTeacher::from(update_teacher),
    )
    .await
    .map(|teacher| HttpResponse::Ok().json(teacher))
}

pub async fn delete_teacher(
    app_state: web::Data<AppState>,
    path: web::Path<i32>,
) -> Result<HttpResponse, MyError> {
    let teacher_id = path.into_inner();
    delete_teacher_db(&app_state.db, teacher_id)
        .await
        .map(|teacher| HttpResponse::Ok().json(teacher))
}
