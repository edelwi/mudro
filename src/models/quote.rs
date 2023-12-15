use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct QuoteCreate {
    pub text: String,
    pub author_id: i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct QuoteUpdate {
    pub text: String,
    pub author_id: i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Quote {
    pub id: i32,
    pub text: String,
    pub author_id: i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct QuoteWithAuthor {
    pub id: i32,
    pub text: String,
    pub author_id: i32,
    pub author_name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct QuoteResponse {
    pub id: i32,
    pub text: String,
    pub author_id: i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct QuoteWithAuthorResponse {
    pub id: i32,
    pub text: String,
    pub author_id: i32,
    pub author_name: String,
}
