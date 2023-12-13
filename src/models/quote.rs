struct QuoteCreate {
    text: String,
    author_id: u32
}

struct QuoteUpdate {
    text: String,
    author_id: u32
}

struct Quote {
    id: u32,
    text: String,
    author_id: u32
}