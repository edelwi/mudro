use serde::{Deserialize, Serialize};

pub struct AuthorCreate {
    pub author_name: String,
}

pub struct AuthorUpdate {
    pub author_name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Author {
    pub id: i32,
    pub author_name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AuthorResponse {
    pub id: i32,
    pub author_name: String,
}
