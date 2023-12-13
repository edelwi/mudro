use std::error::Error;
extern crate dotenv;

mod crud {
    pub mod author;
}

mod models {
    pub mod author;
}

use dotenv::dotenv;
use std::env;
use crud::author::{read_author, read_authors, create_author, read_random_author};

use models::author::{Author, AuthorCreate, AuthorUpdate};

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
    //         author_name: "John Doe 2".into()
    //     }, &pool).await?;
    // println!("{:?}", new_rec);

    for i in 0..5 {
        println!("Random author #{}: {:?}", i+1, read_random_author(&pool).await?)
    }

    Ok(())
}
