pub struct QuoteCreate {
    pub text: String,
    pub author_id: i32
}

pub struct QuoteUpdate {
    pub text: String,
    pub author_id: i32
}

#[derive(Debug)]
pub struct Quote {
    pub id: i32,
    pub text: String,
    pub author_id: i32
}

#[derive(Debug)]
pub struct QuoteWithAuthor {
    pub id: i32,
    pub text: String,
    pub author_id: i32,
    pub author_name: String,
}