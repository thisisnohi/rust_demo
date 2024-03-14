use crate::errors::MyError;
use crate::models::teacher::{CreateTeacher, Teacher, UpdateTeacher};
use sqlx::PgPool;

pub async fn get_all_teachers_db(pool: &PgPool) -> Result<Vec<Teacher>, MyError> {
    // let rows: Vec<Teacher> = sqlx::query_as!(
    //     Teacher,
    //     r#"select * from Teacher"#
    // )
    // .fetch_all(pool)
    // .await?;

    let rows = sqlx::query!("select id, name, picture_url, profile from teacher")
        .fetch_all(pool)
        .await?;

    let teachers: Vec<Teacher> = rows
        .iter()
        .map(|item| Teacher {
            id: item.id,
            name: item.name.clone(),
            picture_url: item.picture_url.clone(),
            profile: item.profile.clone(),
        })
        .collect();

    match teachers.len() {
        0 => Err(MyError::NotFound("Not found records".into())),
        _ => Ok(teachers),
    }
}

pub async fn get_teacher_detail_db(pool: &PgPool, id: i32) -> Result<Teacher, MyError> {
    // let rows: Vec<Teacher> = sqlx::query_as!(
    //     Teacher,
    //     r#"select * from Teacher"#
    // )
    // .fetch_all(pool)
    // .await?;

    let teacher = sqlx::query!(
        "select id, name, picture_url, profile from teacher where id = $1",
        id
    )
    .fetch_one(pool)
    .await
    .map(|item| Teacher {
        id: item.id,
        name: item.name.clone(),
        picture_url: item.picture_url.clone(),
        profile: item.profile.clone(),
    })
    .map_err(|_err| MyError::NotFound("Not found records".into()))?;

    Ok(teacher)
}

pub async fn post_teacher_db(
    pool: &PgPool,
    create_teacher: CreateTeacher,
) -> Result<Teacher, MyError> {
    let teacher = sqlx::query!(
        r#"INSERT INTO teacher(name, picture_url, profile)
           VALUES ($1, $2, $3)
           RETURNING id, name, picture_url, profile
           "#,
        create_teacher.name,
        create_teacher.picture_url,
        create_teacher.profile
    )
    .fetch_one(pool)
    .await?;

    Ok(Teacher {
        id: teacher.id,
        name: teacher.name,
        picture_url: teacher.picture_url,
        profile: teacher.profile,
    })
}

pub async fn update_teacher_detail_db(
    pool: &PgPool,
    id: i32,
    update_teacher: UpdateTeacher,
) -> Result<Teacher, MyError> {
    let teacher = get_teacher_detail_db(pool, id).await?;

    let temp = Teacher {
        id: teacher.id,
        name: if let Some(name) = update_teacher.name {
            name
        } else {
            teacher.name
        },
        picture_url: if let Some(picture_url) = update_teacher.picture_url {
            picture_url
        } else {
            teacher.picture_url
        },
        profile: if let Some(profile) = update_teacher.profile {
            profile
        } else {
            teacher.profile
        },
    };

    let teacher = sqlx::query!(
        r#"UPDATE teacher SET name = $1, picture_url = $2, profile = $3
          WHERE id = $4
           RETURNING id, name, picture_url, profile
           "#,
        temp.name,
        temp.picture_url,
        temp.profile,
        id
    )
    .fetch_one(pool)
    .await
    .map(|teacher| Teacher {
        id: teacher.id,
        name: teacher.name,
        picture_url: teacher.picture_url,
        profile: teacher.profile,
    })
    .map_err(|_err| MyError::NotFound("Teacher id not found".into()))?;

    Ok(teacher)
}

pub async fn delete_teacher_db(pool: &PgPool, teacher_id: i32) -> Result<Teacher, MyError> {
    // 查询id对应记录
    let record = get_teacher_detail_db(pool, teacher_id).await?;

    let count = sqlx::query(&format!("DELETE FROM teacher WHERE id = {}", teacher_id))
        .execute(pool)
        .await
        .map_err(|_err| MyError::DBError("Unable to delete teacher".into()))?;
    println!("删除记录数:{:?}", count);

    Ok(record)
}
