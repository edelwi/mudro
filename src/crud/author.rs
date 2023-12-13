use std::error::Error;
use sqlx::{Row, pool};
// provides `try_next`
// use futures::TryStreamExt;
use crate::models;

use models::author::{Author, AuthorCreate, AuthorUpdate};

pub async fn create_author(author: &AuthorCreate, pool: &sqlx::PgPool) -> Result<Author, Box<dyn Error>> {
    let query = r#"INSERT INTO author (author_name) VALUES ($1) RETURNING id, author_name"#;

    let row = sqlx::query(query)
        .bind(&author.author_name)
        .fetch_one(pool)
        .await?;

    

    let record = Author {
        id: row.get("id"),
        author_name: row.get("author_name")
    };
    println!("{:?}",record);

    Ok(record)
}

pub async fn update_author(author: &AuthorUpdate, id: i32, pool: &sqlx::PgPool) -> Result<Author, Box<dyn Error>> {
    let query = format!("UPDATE author SET author_name = $1 WHERE id = $2 RETURNING id, author_name");

    let row = sqlx::query(&query)
        .bind(&author.author_name)
        .bind(id)
        .fetch_one(pool)
        // .execute(pool)
        .await?;

    let record = Author {
        id: row.get("id"),
        author_name: row.get("author_name")
    };

    Ok(record)
}

pub async fn read_author(id: i32, pool: &sqlx::PgPool) -> Result<Author, Box<dyn Error>> {
    let query = format!("SELECT id, author_name FROM author WHERE id = $1;");  // python сказка просто
 
    let row = sqlx::query(&query)
        .bind(id)
        .fetch_one(pool)
        .await?;

    let record = Author {
        id: row.get("id"),
        author_name: row.get("author_name")
    };

    Ok(record)
}

pub async fn read_authors(pool: &sqlx::PgPool, offset: i32, limit: i32) -> Result<Vec<Author>, Box<dyn Error>> {
    let query = format!("SELECT id, author_name FROM author ORDER BY author_name offset $1 LIMIT $2;");  // python сказка просто
 
    let rows = sqlx::query(&query)
        .bind(offset)
        .bind(limit)
        .fetch_all(pool)
        .await?;

    let records = rows.into_iter().map(|row| 
        Author {
            id: row.get("id"),
            author_name: row.get("author_name")
        }).collect();

    Ok(records)
}

pub async fn read_random_author(pool: &sqlx::PgPool) -> Result<Author, Box<dyn Error>> {
    let query = format!("SELECT id, author_name FROM author ORDER BY random() LIMIT 1;");  // python сказка просто
 
    let row = sqlx::query(&query)
        .fetch_one(pool)
        .await?;

    let record = Author {
        id: row.get("id"),
        author_name: row.get("author_name")
    };

    Ok(record)
}