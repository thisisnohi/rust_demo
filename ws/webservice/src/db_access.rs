use crate::models::Course;
use sqlx::PgPool;

pub async fn get_all_courses_db(pool: &PgPool) -> Vec<Course> {
    let rows = sqlx::query!(
        r#"select id, teacher_id, name, time 
            from course 
            "#
    )
    .fetch_all(pool)
    .await
    .unwrap();

    let rs = rows
        .iter()
        .map(|r| Course {
            teacher_id: r.teacher_id,
            id: Some(r.id),
            name: r.name.clone(),
            time: Some(chrono::NaiveDateTime::from(r.time.unwrap())),
        })
        .collect();

    // 返回
    rs
}

pub async fn get_courses_for_teacher_db(pool: &PgPool, teacher_id: i32) -> Vec<Course> {
    let rows = sqlx::query!(
        r#"select id, teacher_id, name, time 
            from course 
            where teacher_id = $1
            "#,
        teacher_id
    )
    .fetch_all(pool)
    .await
    .unwrap();

    let rs = rows
        .iter()
        .map(|r| Course {
            teacher_id: r.teacher_id,
            id: Some(r.id),
            name: r.name.clone(),
            time: Some(chrono::NaiveDateTime::from(r.time.unwrap())),
        })
        .collect();

    // 返回
    rs
}

pub async fn get_courses_detail_db(pool: &PgPool, teacher_id: i32, course_id: i32) -> Course {
    let rows = sqlx::query!(
        r#"select id, teacher_id, name, time 
            from course 
            where teacher_id = $1 and id = $2
            "#,
        teacher_id,
        course_id
    )
    // 只获取一条数据
    .fetch_one(pool)
    .await
    .unwrap();

    Course {
        teacher_id: rows.teacher_id,
        id: Some(rows.id),
        name: rows.name.clone(),
        time: Some(chrono::NaiveDateTime::from(rows.time.unwrap())),
    }
}

pub async fn post_new_course_db(pool: &PgPool, new_course: Course) -> Course {
    let row = sqlx::query!(
        r#"
           INSERT INTO course (id, teacher_id, name) VALUES ($1, $2, $3)
           RETURNING id, teacher_id, name, time
        "#,
        new_course.id,
        new_course.teacher_id,
        new_course.name,
    )
    // 只获取一条数据
    .fetch_one(pool)
    .await
    .unwrap();

    Course {
        teacher_id: row.teacher_id,
        id: Some(row.id),
        name: row.name.clone(),
        time: Some(chrono::NaiveDateTime::from(row.time.unwrap())),
    }
}
