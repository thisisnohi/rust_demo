use actix_web::web;

use crate::handlers::{
    course::{
        delete_course, get_course_detail, get_courses_for_teacher, post_new_course,
        update_course_details,
    },
    general::health_check_handler,
    teacher::{
        delete_teacher, get_all_teacher, get_teacher_details, post_new_teacher,
        update_teacher_detail,
    },
};

pub fn general_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(health_check_handler));
}

pub fn course_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/courses")
            .route("/", web::post().to(post_new_course))
            .route("/{teacher_id}", web::get().to(get_courses_for_teacher))
            .route(
                "/{teacher_id}/{course_id}",
                web::get().to(get_course_detail),
            )
            .route("/{teacher_id}/{course_id}", web::delete().to(delete_course))
            .route(
                "/{teacher_id}/{course_id}",
                web::put().to(update_course_details),
            ),
    );
}

pub fn teacher_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/teacher")
            .route("/", web::get().to(get_all_teacher))
            .route("/", web::post().to(post_new_teacher))
            .route("/{teacher_id}", web::get().to(get_teacher_details))
            .route("/{teacher_id}", web::delete().to(delete_teacher))
            .route("/{teacher_id}", web::put().to(update_teacher_detail)),
    );
}
