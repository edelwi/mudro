use crate::routs::authors::{
    del_author, get_author, get_authors, get_random_author, new_author, upd_author,
};
use crate::routs::quotes::{
    del_quote, get_quote, get_quotes, get_random_quote, new_quote, upd_quote,
};
use actix_web::web;

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api")
        .service(get_random_author)
        .service(get_author)
        .service(get_authors)
        .service(new_author)
        .service(upd_author)
        .service(del_author)
        .service(get_random_quote)
        .service(get_quotes)
        .service(new_quote)
        .service(upd_quote)
        .service(get_quote)
        .service(del_quote);

    conf.service(scope);
}
