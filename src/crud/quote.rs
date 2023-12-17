use crate::models;
use sqlx::Row;
use std::error::Error;

use models::quote::{Quote, QuoteCreate, QuoteUpdate, QuoteWithAuthor};

pub async fn create_quote(
    quote: &QuoteCreate,
    pool: &sqlx::PgPool,
) -> Result<Quote, Box<dyn Error>> {
    let query =
        r#"INSERT INTO quote (text, author_id) VALUES ($1, $2) RETURNING id, text, author_id"#;

    let row = sqlx::query(query)
        .bind(&quote.text)
        .bind(&quote.author_id)
        .fetch_one(pool)
        .await?;

    let record = Quote {
        id: row.get("id"),
        text: row.get("text"),
        author_id: row.get("author_id"),
    };
    println!("{:?}", record);

    Ok(record)
}

pub async fn update_quote(
    quote: &QuoteUpdate,
    id: i32,
    pool: &sqlx::PgPool,
) -> Result<Quote, Box<dyn Error>> {
    let query = format!(
        "UPDATE quote SET text = $1, author_id = $2 WHERE id = $3 RETURNING id, text, author_id"
    );

    let row = sqlx::query(&query)
        .bind(&quote.text)
        .bind(&quote.author_id)
        .bind(id)
        .fetch_one(pool)
        .await?;

    let record = Quote {
        id: row.get("id"),
        text: row.get("text"),
        author_id: row.get("author_id"),
    };

    Ok(record)
}

pub async fn read_quote(id: i32, pool: &sqlx::PgPool) -> Result<Quote, Box<dyn Error>> {
    let query = format!("SELECT id, text, author_id FROM quote WHERE id = $1;");

    let row = sqlx::query(&query).bind(id).fetch_one(pool).await?;

    let record = Quote {
        id: row.get("id"),
        text: row.get("text"),
        author_id: row.get("author_id"),
    };

    Ok(record)
}

pub async fn read_quote_with_author(
    id: i32,
    pool: &sqlx::PgPool,
) -> Result<QuoteWithAuthor, Box<dyn Error>> {
    let query = format!(
        r#"SELECT quote.id, text, author_id, author_name 
                            FROM quote 
                            INNER JOIN author on quote.author_id = author.id
                            WHERE quote.id = $1;"#
    );

    let row = sqlx::query(&query).bind(id).fetch_one(pool).await?;

    let record = QuoteWithAuthor {
        id: row.get("id"),
        text: row.get("text"),
        author_id: row.get("author_id"),
        author_name: row.get("author_name"),
    };

    Ok(record)
}

pub async fn read_quotes(
    pool: &sqlx::PgPool,
    offset: i32,
    limit: i32,
) -> Result<Vec<Quote>, Box<dyn Error>> {
    let query = format!("SELECT id, text, author_id FROM quote ORDER BY text offset $1 LIMIT $2;");

    let rows = sqlx::query(&query)
        .bind(offset)
        .bind(limit)
        .fetch_all(pool)
        .await?;

    let records = rows
        .into_iter()
        .map(|row| Quote {
            id: row.get("id"),
            text: row.get("text"),
            author_id: row.get("author_id"),
        })
        .collect();

    Ok(records)
}

pub async fn read_quotes_with_author(
    pool: &sqlx::PgPool,
    offset: i32,
    limit: i32,
) -> Result<Vec<QuoteWithAuthor>, Box<dyn Error>> {
    let query = format!(
        r#"SELECT quote.id, text, author_id, author_name 
                            FROM quote 
                            INNER JOIN author on quote.author_id = author.id
                            ORDER BY text offset $1 LIMIT $2;"#
    );

    let rows = sqlx::query(&query)
        .bind(offset)
        .bind(limit)
        .fetch_all(pool)
        .await?;

    let records = rows
        .into_iter()
        .map(|row| QuoteWithAuthor {
            id: row.get("id"),
            text: row.get("text"),
            author_id: row.get("author_id"),
            author_name: row.get("author_name"),
        })
        .collect();

    Ok(records)
}

pub async fn read_random_quote(pool: &sqlx::PgPool) -> Result<Quote, Box<dyn Error>> {
    let query = format!("SELECT id, text, author_id FROM quote ORDER BY random() LIMIT 1;");

    let row = sqlx::query(&query).fetch_one(pool).await?;

    let record = Quote {
        id: row.get("id"),
        text: row.get("text"),
        author_id: row.get("author_id"),
    };

    Ok(record)
}

pub async fn read_random_quote_with_author(
    pool: &sqlx::PgPool,
) -> Result<QuoteWithAuthor, Box<dyn Error>> {
    let query = format!(
        "SELECT quote.id as id, text, author_id, author_name 
                            FROM quote 
                            INNER JOIN author on quote.author_id = author.id 
                            ORDER BY random() LIMIT 1;"
    );

    let row = sqlx::query(&query).fetch_one(pool).await?;

    let record = QuoteWithAuthor {
        id: row.get("id"),
        text: row.get("text"),
        author_id: row.get("author_id"),
        author_name: row.get("author_name"),
    };

    Ok(record)
}

pub async fn delete_quote(id: i32, pool: &sqlx::PgPool) -> Result<Quote, Box<dyn Error>> {
    let query = format!("DELETE FROM quote WHERE id = $1 RETURNING id, text, author_id");

    let row = sqlx::query(&query).bind(id).fetch_one(pool).await?;

    let record = Quote {
        id: row.get("id"),
        text: row.get("text"),
        author_id: row.get("author_id"),
    };

    Ok(record)
}
