use crate::routs::authors::{
    get_author, get_random_author, get_authors, new_author, upd_author, del_author
};
use crate::routs::quotes::{
    get_random_quote, new_quote, upd_quote, get_quote, del_quote, get_quotes
};
use actix_web::{web};

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
        .service(del_quote)
        ;

    conf.service(scope);
}