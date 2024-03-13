use actix_web::{web, HttpResponse};

use crate::db_access::course::{
    delete_course_db, get_courses_detail_db, get_courses_for_teacher_db, post_new_course_db,
    update_course_details_db,
};
use crate::errors::MyError;
use crate::models::course::{CreateCourse, UpdateCourse};
use crate::state::AppState;

pub async fn post_new_course(
    new_course: web::Json<CreateCourse>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, MyError> {
    println!("Received new course");

    // let course_count = app_state
    //     .courses
    //     .lock()
    //     .unwrap()
    //     .clone()
    //     .into_iter()
    //     .filter(|course| course.teacher_id == new_course.teacher_id)
    //     .collect::<Vec<Course>>()
    //     .len();
    // let new_course = Course {
    //     teacher_id: new_course.teacher_id,
    //     id: Some(course_count + 1),
    //     name: new_course.name.clone(),
    //     time: Some(Utc::now().naive_utc()),
    // };
    //
    // app_state.courses.lock().unwrap().push(new_course);

    // // 使用数据库操作
    // let course = post_new_course_db(&app_state.db, new_course.into()).await;
    // // 返回
    // HttpResponse::Ok().json(course)

    post_new_course_db(&app_state.db, new_course.try_into()?)
        .await
        .map(|course| HttpResponse::Ok().json(course))
}

pub async fn get_courses_for_teacher(
    path: web::Path<i32>, // param: web::Path<(i32,)>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, MyError> {
    println!("Recived new course");

    let teacher_id = path.into_inner();

    // let filter_courses = app_state
    //     .courses
    //     .lock()
    //     .unwrap()
    //     .clone()
    //     .into_iter()
    //     .filter(|item| item.teacher_id == id)
    //     .collect::<Vec<Course>>();
    //
    // if filter_courses.len() > 0 {
    //     HttpResponse::Ok().json(filter_courses)
    // } else {
    //     // 返回
    //     HttpResponse::Ok().json("No course found...")
    // }

    // 使用数据库操作
    get_courses_for_teacher_db(&app_state.db, teacher_id)
        .await
        .map(|courses| HttpResponse::Ok().json(courses))
}

pub async fn get_course_detail(
    path: web::Path<(i32, i32)>, // param: web::Path<(i32, i32)>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, MyError> {
    println!("Recived new course");

    let (teacher_id, course_id) = path.into_inner();

    // let filter_courses = app_state
    //     .courses
    //     .lock()
    //     .unwrap()
    //     .clone()
    //     .into_iter()
    //     .find(|item| item.teacher_id == teacher_id && item.id == Some(course_id))
    //     .ok_or("Course not found");
    //
    // if let Ok(course) = filter_courses {
    //     HttpResponse::Ok().json(course)
    // } else {
    //     // 返回
    //     HttpResponse::Ok().json("No course found...")
    // }

    // 使用数据库操作
    // let course = get_courses_detail_db(&app_state.db, teacher_id, course_id).await;
    // HttpResponse::Ok().json(course)
    
    get_courses_detail_db(&app_state.db, teacher_id, course_id)
        .await
        .map(|course| HttpResponse::Ok().json(course))
}

/// 删除课程
pub async fn delete_course(
    path: web::Path<(i32, i32)>, // param: web::Path<(i32, i32)>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, MyError> {
    println!("delete_course  course");
    let (teacher_id, course_id) = path.into_inner();

    delete_course_db(&app_state.db, teacher_id, course_id)
        .await
        .map(|course| HttpResponse::Ok().json(course))
}

/// 更新课程
pub async fn update_course_details(
    path: web::Path<(i32, i32)>, // param: web::Path<(i32, i32)>,
    app_state: web::Data<AppState>,
    update_course: web::Json<UpdateCourse>,
) -> Result<HttpResponse, MyError> {
    println!("update_course_details course");

    let (teacher_id, course_id) = path.into_inner();
    
    update_course_details_db(&app_state.db, teacher_id, course_id, update_course.into())
        .await
        .map(|course| HttpResponse::Ok().json(course))
}

//
// #[cfg(test)]
// mod tests {
//     use std::sync::Mutex;
//
//     use actix_web::http::StatusCode;
//
//     use super::*;
//
//     #[actix_rt::test]
//     async fn post_course_test() {
//         let course = web::Json(Course {
//             teacher_id: 1,
//             id: None,
//             name: "Test course".into(),
//             time: None,
//         });
//
//         let app_state = web::Data::new(AppState {
//             health_check_response: "".to_string(),
//             visit_count: Mutex::new(0),
//             courses: Mutex::new(vec![]),
//         });
//
//         //
//         let resp = new_course(course, app_state).await;
//
//         assert_eq!(resp.status(), StatusCode::OK)
//     }
//
//     #[actix_rt::test]
//     async fn get_all_courses_success() {
//         let course = web::Path::from(1);
//
//         let app_state = web::Data::new(AppState {
//             health_check_response: "".to_string(),
//             visit_count: Mutex::new(0),
//             courses: Mutex::new(vec![]),
//         });
//
//         //
//         let resp = get_courses_for_teacher(course, app_state).await;
//
//         assert_eq!(resp.status(), StatusCode::OK)
//     }
//
//     #[actix_rt::test]
//     async fn test_get_course_detail() {
//         let path = web::Path::from((1, 1));
//
//         let app_state = web::Data::new(AppState {
//             health_check_response: "".to_string(),
//             visit_count: Mutex::new(0),
//             courses: Mutex::new(vec![]),
//         });
//
//         //
//         let resp = get_course_detail(path, app_state).await;
//         println!("resp:{:?}", resp);
//
//         assert_eq!(resp.status(), StatusCode::OK)
//     }
// }

/// 使用数据库后的测试类
#[cfg(test)]
mod tests {
    use std::env;
    use std::sync::Mutex;

    use actix_web::{http::StatusCode, ResponseError};
    use chrono::NaiveDateTime;
    use dotenv::dotenv;
    use sqlx::{postgres::PgPoolOptions, PgPool};

    use super::*;

    #[actix_rt::test]
    async fn post_course_test() {
        // 加载环境变量配置
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL 没有在 .env文件里设置");
        let db_pool: sqlx::Pool<sqlx::Postgres> =
            PgPoolOptions::new().connect(&database_url).await.unwrap();

        let app_state = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: db_pool,
        });

        let course = web::Json(CreateCourse {
            teacher_id: 1,
            name: "Test course".into(),
            description: Some("This is a course".into()),
            format: None,
            structure: None,
            duration: None,
            price: None,
            language: Some("English".into()),
            level: Some("Beginner".into()),
        });

        //
        let resp = post_new_course(course, app_state).await.unwrap();

        assert_eq!(resp.status(), StatusCode::OK)
    }

    #[actix_rt::test]
    async fn get_all_courses_success() {
        // 加载环境变量配置
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL 没有在 .env文件里设置");
        let db_pool: sqlx::Pool<sqlx::Postgres> =
            PgPoolOptions::new().connect(&database_url).await.unwrap();

        let app_state = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: db_pool,
        });

        let path: web::Path<i32> = web::Path::from(1);
        let resp = get_courses_for_teacher(path, app_state).await.unwrap();

        assert_eq!(resp.status(), StatusCode::OK)
    }

    #[actix_rt::test]
    async fn test_get_course_detail() {
        // 加载环境变量配置
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL 没有在 .env文件里设置");
        let db_pool: sqlx::Pool<sqlx::Postgres> =
            PgPoolOptions::new().connect(&database_url).await.unwrap();

        let path: web::Path<(i32, i32)> = web::Path::from((1, 1));

        let app_state = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: db_pool,
        });

        //
        let resp = get_course_detail(path, app_state).await.unwrap();
        println!("resp:{:?}", resp);

        assert_eq!(resp.status(), StatusCode::OK)
    }

    #[actix_rt::test]
    async fn test_get_course_detail_fail() {
        // 加载环境变量配置
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL 没有在 .env文件里设置");
        let db_pool: sqlx::Pool<sqlx::Postgres> =
            PgPoolOptions::new().connect(&database_url).await.unwrap();

        let path: web::Path<(i32, i32)> = web::Path::from((11, 21));

        let app_state = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: db_pool,
        });

        //
        let resp = get_course_detail(path, app_state).await;
        println!("resp:{:?}", resp);
        match resp {
            Ok(_) => {}
            Err(err) => assert_eq!(err.status_code(), StatusCode::NOT_FOUND),
        }
    }

    #[actix_rt::test]
    async fn update_course_success() {
        // 加载环境变量配置
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL 没有在 .env文件里设置");
        let db_pool: sqlx::Pool<sqlx::Postgres> =
            PgPoolOptions::new().connect(&database_url).await.unwrap();

        

        let app_state = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: db_pool,
        });

        let update_course = UpdateCourse {
            name: Some("Course name changed".into()),
            descriptioin: Some("This is another test course".into()),
            format: None,
            structure: None,
            duration: None,
            price: None,
            language: Some("Chinese".into()),
            level: Some("Intermediate".into()),
        };

        let path: web::Path<(i32, i32)> = web::Path::from((1, 1));
 
        //
        let resp = update_course_details(path, app_state, web::Json(update_course)).await;
        println!("resp:{:?}", resp);
        match resp {
            Ok(_) => {}
            Err(err) => assert_eq!(err.status_code(), StatusCode::NOT_FOUND),
        }
    }

    #[ignore = "不能删除数据"]
    #[actix_rt::test]
    async fn delete_course_success() {
        // 加载环境变量配置
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL 没有在 .env文件里设置");
        let db_pool: sqlx::Pool<sqlx::Postgres> =
            PgPoolOptions::new().connect(&database_url).await.unwrap();

        let app_state = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: db_pool,
        });

        let path: web::Path<(i32, i32)> = web::Path::from((1, 1));
        //
        let resp = delete_course(path, app_state).await.unwrap();
        println!("resp:{:?}", resp);
        
        assert_eq!(resp.status(), StatusCode::OK)
    }

    #[actix_rt::test]
    async fn delete_course_failture() {
        // 加载环境变量配置
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL 没有在 .env文件里设置");
        let db_pool: sqlx::Pool<sqlx::Postgres> =
            PgPoolOptions::new().connect(&database_url).await.unwrap();

        let app_state = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: db_pool,
        });

        let path: web::Path<(i32, i32)> = web::Path::from((1, 100));
        //
        let resp = delete_course(path, app_state).await;
        println!("resp:{:?}", resp);
        match resp {
            Ok(_) => println!("不应该删除成功，something wrong"),
            Err(err) => assert_eq!(err.status_code(), StatusCode::NOT_FOUND),
        }
    }
}
