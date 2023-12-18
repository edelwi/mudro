use crate::models;
use sqlx::Row;
use std::error::Error;

use models::author::{Author, AuthorCreate, AuthorUpdate};
// use mockall::{automock};

// #[automock]
// pub trait PoolMock {
//     fn create_author(&self) -> Result<Author, Box<dyn Error>>;
// }

pub async fn create_author(
    author: &AuthorCreate,
    pool: &sqlx::PgPool,
) -> Result<Author, Box<dyn Error>> {
    let query = r#"INSERT INTO author (author_name) VALUES ($1) RETURNING id, author_name"#;

    let row = sqlx::query(query)
        .bind(&author.author_name)
        .fetch_one(pool)
        .await?;

    let record = Author {
        id: row.get("id"),
        author_name: row.get("author_name"),
    };

    Ok(record)
}

pub async fn update_author(
    author: &AuthorUpdate,
    id: i32,
    pool: &sqlx::PgPool,
) -> Result<Author, Box<dyn Error>> {
    let query =
        format!("UPDATE author SET author_name = $1 WHERE id = $2 RETURNING id, author_name");

    let row = sqlx::query(&query)
        .bind(&author.author_name)
        .bind(id)
        .fetch_one(pool)
        // .execute(pool)
        .await?;

    let record = Author {
        id: row.get("id"),
        author_name: row.get("author_name"),
    };

    Ok(record)
}

pub async fn read_author(id: i32, pool: &sqlx::PgPool) -> Result<Author, Box<dyn Error>> {
    let query = format!("SELECT id, author_name FROM author WHERE id = $1;"); // python сказка просто

    let row = sqlx::query(&query).bind(id).fetch_one(pool).await?;

    let record = Author {
        id: row.get("id"),
        author_name: row.get("author_name"),
    };

    Ok(record)
}

pub async fn read_authors(
    pool: &sqlx::PgPool,
    offset: i32,
    limit: i32,
) -> Result<Vec<Author>, Box<dyn Error>> {
    let query =
        format!("SELECT id, author_name FROM author ORDER BY author_name offset $1 LIMIT $2;"); // python сказка просто

    let rows = sqlx::query(&query)
        .bind(offset)
        .bind(limit)
        .fetch_all(pool)
        .await?;

    let records = rows
        .into_iter()
        .map(|row| Author {
            id: row.get("id"),
            author_name: row.get("author_name"),
        })
        .collect();

    Ok(records)
}

pub async fn read_random_author(pool: &sqlx::PgPool) -> Result<Author, Box<dyn Error>> {
    let query = format!("SELECT id, author_name FROM author ORDER BY random() LIMIT 1;"); // python сказка просто

    let row = sqlx::query(&query).fetch_one(pool).await?;

    let record = Author {
        id: row.get("id"),
        author_name: row.get("author_name"),
    };

    Ok(record)
}

pub async fn delete_author(id: i32, pool: &sqlx::PgPool) -> Result<Author, Box<dyn Error>> {
    let query = format!("DELETE FROM author WHERE id = $1 RETURNING id, author_name");

    let row = sqlx::query(&query).bind(id).fetch_one(pool).await?;

    let record = Author {
        id: row.get("id"),
        author_name: row.get("author_name"),
    };

    Ok(record)
}

// pub async fn total_authors(pool: &sqlx::PgPool) -> Result<i32, Box<dyn Error>> {
//     let query = format!(r#"SELECT COUNT(*) FROM author"#);

//     let row = sqlx::query(&query).fetch_one(pool).await?;

//     let authors = row.try_get("count")?;

//     Ok(authors)
// }

// async fn total_authors(pool: &sqlx::PgPool) -> Result<i32, Box<dyn Error>> {
//     let mut conn = pool.acquire().await?;

//     let authors: i64 = sqlx::query("SELECT COUNT(*) FROM authors")
//         .fetch_one(&mut conn)
//         .try_map(|row: PgRow| row.try_get("count"))
//         .await?;

//     Ok(authors)
// }



// #[cfg(test)]
// mod tests {
//     use super::*;
//     use mockall::predicate::*;
//     use sqlx::postgres::PgPool;

//     // Generate a mock for the database pool
//     auto_mock! {
//         PoolMock;
//     }

//     // Test function with mock
//     #[tokio::test]
//     async fn test_create_author() {
//         // Create a mock for the database pool
//         let pool_mock = MockPool::new();

//         // Set up expectations for the mock
//         pool_mock.expect_get_pool().return_const(&PgPool);

//         let author = AuthorCreate{
//             author_name: "John Doe".to_string(),
//         };

//         // Call the CRUD function with the mock
//         let result = create_author(&author, &pool_mock).await;

//         // Assert that the result is as expected
//         assert!(result.u.is_ok());
//     }
// }