use actix_web::{web, HttpResponse};
use chrono::Utc;

use crate::{models::Course, state::AppState};

pub async fn health_check_handler(app_state: web::Data<AppState>) -> HttpResponse {
    let health_check_response = &app_state.health_check_response;
    let mut visit_count = app_state.visit_count.lock().unwrap();

    let response = format!("{} {} times", health_check_response, visit_count);

    *visit_count += 1;

    // 返回
    HttpResponse::Ok().json(&response)
}

pub async fn new_course(
    new_course: web::Json<Course>,
    app_state: web::Data<AppState>,
) -> HttpResponse {
    println!("Recived new course");

    let course_count = app_state
        .courses
        .lock()
        .unwrap()
        .clone()
        .into_iter()
        .filter(|course| course.teacher_id == new_course.teacher_id)
        .collect::<Vec<Course>>()
        .len();
    let new_course = Course {
        teacher_id: new_course.teacher_id,
        id: Some(course_count + 1),
        name: new_course.name.clone(),
        time: Some(Utc::now().naive_utc()),
    };

    app_state.courses.lock().unwrap().push(new_course);

    // 返回
    HttpResponse::Ok().json("Course added")
}

pub async fn get_courses_for_teacher(
    param: web::Path<usize>,
    app_state: web::Data<AppState>,
) -> HttpResponse {
    println!("Recived new course");

    let id = param.into_inner();

    let filter_courses = app_state
        .courses
        .lock()
        .unwrap()
        .clone()
        .into_iter()
        .filter(|item| item.teacher_id == id)
        .collect::<Vec<Course>>();

    if filter_courses.len() > 0 {
        HttpResponse::Ok().json(filter_courses)
    } else {
        // 返回
        HttpResponse::Ok().json("No course found...")
    }
}

pub async fn get_course_detail(
    param: web::Path<(usize,usize)>,
    app_state: web::Data<AppState>,
) -> HttpResponse {
    println!("Recived new course");

    let (teacher_id, course_id) = param.into_inner();

    let filter_courses = app_state
        .courses
        .lock()
        .unwrap()
        .clone()
        .into_iter()
        .find(|item| item.teacher_id == teacher_id && item.id == Some(course_id))
        .ok_or("Course not found");

    if let Ok(course) = filter_courses{
        HttpResponse::Ok().json(course)
    } else {
        // 返回
        HttpResponse::Ok().json("No course found...")
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Mutex;

    use actix_web::http::StatusCode;

    use super::*;

    #[actix_rt::test]
    async fn post_course_test() {
        let course = web::Json(Course {
            teacher_id: 1,
            id: None,
            name: "Test course".into(),
            time: None,
        });

        let app_state = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            courses: Mutex::new(vec![]),
        });

        //
        let resp = new_course(course, app_state).await;

        assert_eq!(resp.status(), StatusCode::OK)
    }


    #[actix_rt::test]
    async fn get_all_courses_success() {
        let course = web::Path::from(1);

        let app_state = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            courses: Mutex::new(vec![]),
        });

        // 
        let resp = get_courses_for_teacher(course, app_state).await;

        assert_eq!(resp.status(), StatusCode::OK)
    }

    #[actix_rt::test]
    async fn test_get_course_detail() {
        let path = web::Path::from((1,1));

        let app_state = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            courses: Mutex::new(vec![]),
        });

        // 
        let resp = get_course_detail(path, app_state).await;
        println!("resp:{:?}", resp);
        
        assert_eq!(resp.status(), StatusCode::OK)
    }
}
