use sqlx::PgPool;

use crate::errors::MyError;
use crate::models::course::{Course, CreateCourse, UpdateCourse};

pub async fn get_all_courses_db(pool: &PgPool) -> Result<Vec<Course>, MyError> {
    let rows: Vec<Course> = sqlx::query_as!(
        Course,
        r#"select *
            from course 
            "#
    )
    .fetch_all(pool)
    .await?;

    Ok(rows)
}

pub async fn get_courses_for_teacher_db(
    pool: &PgPool,
    teacher_id: i32,
) -> Result<Vec<Course>, MyError> {
    // 修改前写法
    // let rows = sqlx::query!(
    //     r#"select id, teacher_id, name, time
    //         from course
    //         where teacher_id = $1
    //         "#,
    //     teacher_id
    // )
    // .fetch_all(pool)
    // .await?;

    // let rs:Vec<Course> = rows
    //     .iter()
    //     .map(|r| Course {
    //         teacher_id: r.teacher_id,
    //         id: Some(r.id),
    //         name: r.name.clone(),
    //         time: Some(chrono::NaiveDateTime::from(r.time.unwrap())),
    //     })
    //     .collect();

    // // 返回
    // match rs.len() {
    //     0 => Err(MyError::NotFound(("Course not found for teacher".into()))),
    //     _ => Ok(rs),
    // }

    // 修改后写法  sqlx::FromRow 作用
    let rows: Vec<Course> = sqlx::query_as!(
        Course,
        r#"select *
            from course 
            where teacher_id = $1
            "#,
        teacher_id
    )
    .fetch_all(pool)
    .await?;

    Ok(rows)
}

pub async fn get_courses_detail_db(
    pool: &PgPool,
    teacher_id: i32,
    course_id: i32,
) -> Result<Course, MyError> {
    // let rows = sqlx::query!(
    //     r#"select id, teacher_id, name, time
    //         from course
    //         where teacher_id = $1 and id = $2
    //         "#,
    //     teacher_id,
    //     course_id
    // )
    // // 只获取一条数据
    // .fetch_one(pool)
    // .await;

    // if let Ok(rows) = rows{
    //     Ok(Course {
    //         teacher_id: rows.teacher_id,
    //         id: Some(rows.id),
    //         name: rows.name.clone(),
    //         time: Some(chrono::NaiveDateTime::from(rows.time.unwrap())),
    //     })
    // } else{
    //     Err(MyError::NotFound("Course id not found".into()))
    // }

    let rows: Option<Course> = sqlx::query_as!(
        Course,
        r#"select *
            from course 
            where teacher_id = $1 and id = $2
            "#,
        teacher_id,
        course_id
    )
    // 只获取一条数据
    .fetch_optional(pool)
    .await?;

    if let Some(course) = rows {
        Ok(course)
    } else {
        Err(MyError::NotFound("Course id not found".into()))
    }
}

pub async fn post_new_course_db(pool: &PgPool, new_course: CreateCourse) -> Result<Course, MyError> {
    // 获取最大id + 1
    let all_course = get_all_courses_db(&pool).await.unwrap();
    let id: i32 = match all_course.len() {
        0 => 1,
        _ => {
            all_course
                .iter()
                .reduce(|a, b| if a.id > b.id { a } else { b })
                .unwrap()
                .id
                + 1
        }
    };

    println!("新增ID:{}", id);

    // 新增时没有id 和 time
    let row = sqlx::query_as!(
        Course,
        r#"
           INSERT INTO course 
           (teacher_id, name, description, format, structure, duration, price, language, level, id) 
           VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
           RETURNING id, teacher_id, name, time, description, format, structure, duration, price, language, level
        "#,
        new_course.teacher_id,
        new_course.name,
        new_course.description,
        new_course.format,
        new_course.structure,
        new_course.duration,
        new_course.price,
        new_course.language,
        new_course.level,
        id
    )
    // 只获取一条数据
    .fetch_one(pool)
    .await?;

    Ok(row)
}

pub async fn delete_course_db(pool: &PgPool, teacher_id: i32, id: i32) -> Result<String, MyError> {
    let rows = sqlx::query!(
        r#"
        DELETE FROM course WHERE teacher_id = $1 and id = $2
        "#,
        teacher_id,
        id
    )
    // 只获取一条数据
    .execute(pool)
    .await?;

    Ok(format!("Delete {:?} record", rows))
}



pub async fn update_course_details_db(pool: &PgPool, teacher_id: i32, id: i32, update_course: UpdateCourse) -> Result<Course, MyError> {
    let db_info = sqlx::query_as!(
        Course,
        r#"
        SELECT * FROM course WHERE teacher_id = $1 and id = $2
        "#,
        teacher_id,
        id
    )
    // 只获取一条数据
    .fetch_one(pool)
    .await
    .map_err(|_err| MyError::NotFound("Course id not foud".into()))?
    ;


    let name = if let Some(name) = update_course.name{
        name
    } else {
        db_info.name
    };

    let description = if let Some(description) = update_course.descriptioin{
        description
    } else {
        db_info.description.unwrap_or_default()
    };

    let format = if let Some(format) = update_course.format{
        format
    } else {
        db_info.format.unwrap_or_default()
    };

    let structure = if let Some(structure) = update_course.structure{
        structure
    } else {
        db_info.structure.unwrap_or_default()
    };

    let duration = if let Some(duration) = update_course.duration{
        duration
    } else {
        db_info.duration.unwrap_or_default()
    };
    let level = if let Some(level) = update_course.level{
        level
    } else {
        db_info.level.unwrap_or_default()
    };

    let language = if let Some(language) = update_course.language{
        language
    } else {
        db_info.language.unwrap_or_default()
    };

    let price = if let Some(price) = update_course.price{
        price
    } else {
        db_info.price.unwrap_or_default()
    };


    // 组装sql语句
    let course_row = sqlx::query_as!(
        Course,
        "
        UPDATE course SET 
         name = $1, description = $2, format = $3, structure = $4, duration = $5, price = $6, language = $7, level = $8
        where teacher_id = $9 and id = $10
        RETURNING id, teacher_id, name, time, description, format, structure, duration, price, language, level
        ",
        name, description, format, structure, duration, price, language, level
        , teacher_id, id
    ).fetch_one(pool).await;

    if let Ok(course_row) = course_row {
        Ok(course_row)
    } else {
        Err(MyError::NotFound("Course id not found".into()))
    }

}
