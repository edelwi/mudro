pub struct AuthorCreate {
    pub author_name: String,
}

pub struct AuthorUpdate {
    pub author_name: String
}

#[derive(Debug)]
pub struct Author {
    pub id: i32,
    pub author_name: String
}