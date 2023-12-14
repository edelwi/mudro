use std::error::Error;
extern crate dotenv;

mod crud {
    pub mod author;
    pub mod quote;
}

mod models {
    pub mod author;
    pub mod quote;
}

use dotenv::dotenv;
use std::env;
use crud::author::{read_author, read_authors, create_author, read_random_author, delete_author};
use crud::quote::{create_quote, read_quote, read_quote_with_author, read_quotes, read_quotes_with_author, update_quote, read_random_quote, read_random_quote_with_author, delete_quote};

use models::author::{Author, AuthorCreate, AuthorUpdate};
use models::quote::{Quote, QuoteCreate, QuoteUpdate, QuoteWithAuthor};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    let db_url = "DATABASE_URL";
    let url = env::var(db_url).expect(&format!("Expected {} to be set", db_url));
    let pool = sqlx::postgres::PgPool::connect(url.as_str()).await?;


    println!("Try to read author 1");
    let a = read_author(1, &pool).await?;
    println!("{:?}", a);

    let result = read_authors(&pool, 5, 10);

    for row in result.await.iter() {
        println!("{:?}", row);
    }
    // let sum: i32 = res.get("sum");
    // println!("1 + 1 = {}", sum);

    // let new_rec = create_author(
    //     &AuthorCreate {
    //         author_name: "John Doe".into()
    //     }, &pool).await?;
    // println!("{:?}", new_rec);

    for i in 0..5 {
        println!("Random author #{}: {:?}", i+1, read_random_author(&pool).await?)
    }

    // let del_rec = delete_author(36, &pool).await?;
    // println!("Deleted record: {:?}", del_rec);

    // let new_quote = create_quote(
    //     &QuoteCreate {
    //         text: "Test quote".into(),
    //         author_id: 34,
    //     }, &pool
    // ).await?;
    // print!("{:?}", new_quote);

    let q1 = read_quote(5, &pool).await?;
    println!("Quote 5 : {:?}", q1);

    let q2 = read_quote_with_author(6, &pool).await?;
    println!("QuoteWithAuthor 6 : {:?}", q2);

    let q3s = read_quotes(&pool, 5, 5);
    for row2 in q3s.await.iter() {
        println!(">> {:?}", row2);
    }

    // let up_rec = update_quote(
    //     &QuoteUpdate {
    //         text: "Something new!!!".into(),
    //         author_id: 34
    //     }, 41, &pool).await?;
    // println!("{:?}", up_rec);

    let q4s = read_quotes_with_author(&pool, 5, 5);
    for row in q4s.await.iter() {
        println!(">> {:?}", row);
    }

    for i in 0..5 {
        println!("Random quote #{}: {:?}", i+1, read_random_quote(&pool).await?)
    }

    for i in 0..5 {
        println!("Random quote 2 #{}: {:?}", i+1, read_random_quote_with_author(&pool).await?)
    }

    let del_q = delete_quote(41, &pool).await?;
    println!("Deleted record: {:?}", del_q);

    Ok(())
}
