use crate::crud::quote::{
    create_quote, delete_quote, read_quote, read_quotes, read_random_quote_with_author,
    update_quote,
};
use crate::models::parameter::LimitOffset;
use crate::models::quote::{
    Quote, QuoteCreate, QuoteResponse, QuoteUpdate, QuoteWithAuthor, QuoteWithAuthorResponse,
};
use crate::AppState;
use actix_web::{delete, get, patch, post, web, HttpResponse, Responder};
use serde_json;
use log::{error, info, warn};

fn make_quote_short_response(quote: &Quote) -> serde_json::Value {
    let quote_response = serde_json::json!(
    {
        "status": "success",
        "data": serde_json::json!({
            "quote": quote_short_to_response(&quote)
        }
        )});
    return quote_response;
}

fn make_quote_response(quote: &QuoteWithAuthor) -> serde_json::Value {
    let quote_response = serde_json::json!(
    {
        "status": "success",
        "data": serde_json::json!({
            "quote": quote_to_response(&quote)
        }
        )});
    return quote_response;
}

#[post("/quotes")]
pub async fn new_quote(data: web::Data<AppState>, body: web::Json<QuoteCreate>) -> impl Responder {
    let new_quote = create_quote(&body, &data.db).await;
    match new_quote {
        Ok(new_quote) => {
            let quote_response = make_quote_short_response(&new_quote);

            return HttpResponse::Ok().json(quote_response);
        }
        Err(e) => {
            let err_message = format!("{:?}", e);
            if err_message.contains("Duplicate entry") {
                warn!("Duplicate entry: {:?}, {}", e, e);
                return HttpResponse::BadRequest()
                    .json(serde_json::json!({"status": "error","message": "Quota already exists."}))
            }
            else {
                warn!("Unexpected Error: {:?} {}", e, e);
                return HttpResponse::Conflict()
                    .json(serde_json::json!({"status": "error","message": "Something went wrong."}));
            }
        }
    }
}

#[patch("/quotes/{id}")]
pub async fn upd_quote(
    data: web::Data<AppState>,
    id: web::Path<(String,)>,
    body: web::Json<QuoteUpdate>,
) -> impl Responder {
    let (id,) = id.into_inner();

    let quote_id = match id.parse() {
        Ok(quote_id) => quote_id,
        Err(e) => {
            let err_message = format!("{}", e);
            if err_message.starts_with("invalid digit found") {
                warn!("Inwalid ID: {} -> {}", e, id);
                return HttpResponse::BadRequest()
                .json(serde_json::json!({"status": "error","message": "Wrong ID."}));
            } else {
                warn!("Unknown error: {}", e);
                return HttpResponse::Conflict()
                    .json(serde_json::json!({"status": "error","message": "Something went wrong."}));
            }
        }
    };

    let updated_quote = update_quote(&body, quote_id, &data.db).await;
    match updated_quote {
        Ok(updated_quote) => {
            let quote_response = make_quote_short_response(&updated_quote);

            return HttpResponse::Ok().json(quote_response);
        }
        Err(e) => {
            let err_message = format!("{:?}", e);
            if err_message.contains("RowNotFound") {
                warn!("Row not found: {:?}, {}", e, e);
                return HttpResponse::NotAcceptable()
                    .json(serde_json::json!({"status": "error","message": "ID not exists."}))
            }
            else if err_message.contains("Duplicate entry") {
                warn!("Duplicate entry: {:?}, {}", e, e);
                return HttpResponse::BadRequest()
                    .json(serde_json::json!({"status": "error","message": "Quota already exists."}))
            }
            else {
                warn!("Unexpected Error: {:?} {}", e, e);
                return HttpResponse::Conflict()
                    .json(serde_json::json!({"status": "error","message": "Something went wrong."}));
            }
        }
    }
}

#[get("/quotes/random")]
pub async fn get_random_quote(data: web::Data<AppState>) -> impl Responder {
    let result = read_random_quote_with_author(&data.db).await;

    match result {
        Ok(quote) => {
            let quote_response = make_quote_response(&quote);

            return HttpResponse::Ok().json(quote_response);
        }
        Err(e) => {
            let err_message = format!("{:?}", e);
            if err_message.contains("RowNotFound") {
                warn!("Row not found: {:?}, {}", e, e);
                return HttpResponse::NotAcceptable()
                    .json(serde_json::json!({"status": "error","message": "ID not exists."}))
            }
            else {
                warn!("Unexpected Error: {:?} {}", e, e);
                return HttpResponse::Conflict()
                    .json(serde_json::json!({"status": "error","message": "Something went wrong."}));
            }
        }
    };
}

#[get("/quotes/{id}")]
pub async fn get_quote(data: web::Data<AppState>, id: web::Path<(String,)>) -> impl Responder {
    let (id,) = id.into_inner();

    let quote_id = match id.parse() {
        Ok(quote_id) => quote_id,
        Err(e) => {
            let err_message = format!("{}", e);
            if err_message.starts_with("invalid digit found") {
                warn!("Inwalid ID: {} -> {}", e, id);
                return HttpResponse::BadRequest()
                .json(serde_json::json!({"status": "error","message": "Wrong ID."}));
            } else {
                warn!("Unknown error: {}", e);
                return HttpResponse::Conflict()
                    .json(serde_json::json!({"status": "error","message": "Something went wrong."}));
            }
        }
    };

    let result = read_quote(quote_id, &data.db).await;

    match result {
        Ok(quote) => {
            let quote_response = make_quote_short_response(&quote);

            return HttpResponse::Ok().json(quote_response);
        }
        Err(e) => {
            let err_message = format!("{:?}", e);
            if err_message.contains("RowNotFound") {
                warn!("Row not found: {:?}, {}", e, e);
                return HttpResponse::NotAcceptable()
                    .json(serde_json::json!({"status": "error","message": "ID not exists."}))
            }
            else {
                warn!("Unexpected Error: {:?} {}", e, e);
                return HttpResponse::Conflict()
                    .json(serde_json::json!({"status": "error","message": "Something went wrong."}));
            }
        }
    };
}

#[delete("/quotes/{id}")]
pub async fn del_quote(data: web::Data<AppState>, id: web::Path<(String,)>) -> impl Responder {
    let (id,) = id.into_inner();

    let quote_id = match id.parse() {
        Ok(quote_id) => quote_id,
        Err(e) => {
            let err_message = format!("{}", e);
            if err_message.starts_with("invalid digit found") {
                warn!("Inwalid ID: {} -> {}", e, id);
                return HttpResponse::BadRequest()
                .json(serde_json::json!({"status": "error","message": "Wrong ID."}));
            } else {
                warn!("Unknown error: {}", e);
                return HttpResponse::Conflict()
                    .json(serde_json::json!({"status": "error","message": "Something went wrong."}));
            }
        }
    };

    let result = delete_quote(quote_id, &data.db).await;

    match result {
        Ok(quote) => {
            let quote_response = make_quote_short_response(&quote);

            return HttpResponse::Ok().json(quote_response);
        }
        Err(e) => {
            let err_message = format!("{:?}", e);
            if err_message.contains("RowNotFound") {
                warn!("Row not found: {:?}, {}", e, e);
                return HttpResponse::NotAcceptable()
                    .json(serde_json::json!({"status": "error","message": "ID not exists."}))
            }
            else {
                warn!("Unexpected Error: {:?} {}", e, e);
                return HttpResponse::Conflict()
                    .json(serde_json::json!({"status": "error","message": "Something went wrong."}));
            }
        }
    };
}

#[get("/quotes")]
pub async fn get_quotes(
    data: web::Data<AppState>,
    params: web::Query<LimitOffset>,
) -> impl Responder {
    let limit = params.limit.unwrap_or(100) as i32;
    let offset = params.offset.unwrap_or(0) as i32;

    let result = read_quotes(&data.db, offset, limit).await;
    // let total_count_result = total_quotes(&data.db).await;

    // let total_count = match total_count_result {
    //     Ok(total_count) => total_count,
    //     Err(e) => {
    //         return HttpResponse::InternalServerError()
    //             .json(serde_json::json!({"status": "error","message": format!("{:?}", e)}));
    //     }
    // };

    match result {
        Ok(quotes) => {
            let quotes_responses = quotes
                .into_iter()
                .map(|quote| quote_short_to_response(&quote))
                .collect::<Vec<QuoteResponse>>();
            let quotes_response = serde_json::json!(
                {
                    "status": "success",
                    "data": quotes_responses,
                    // "meta": {
                    //     "totalCount": total_count as u64
                    // }
                }
            );

            return HttpResponse::Ok().json(quotes_response);
        }
        Err(e) => {
            let err_message = format!("{:?}", e);
            if err_message.contains("RowNotFound") {
                warn!("Row not found: {:?}, {}", e, e);
                return HttpResponse::NotAcceptable()
                    .json(serde_json::json!({"status": "error","message": "ID not exists."}))
            }
            else {
                warn!("Unexpected Error: {:?} {}", e, e);
                return HttpResponse::Conflict()
                    .json(serde_json::json!({"status": "error","message": "Something went wrong."}));
            }
        }
    };
}

fn quote_to_response(quote: &QuoteWithAuthor) -> QuoteWithAuthorResponse {
    QuoteWithAuthorResponse {
        id: quote.id.to_owned(),
        text: quote.text.to_owned(),
        author_id: quote.author_id.to_owned(),
        author_name: quote.author_name.to_owned(),
    }
}

fn quote_short_to_response(quote: &Quote) -> QuoteResponse {
    QuoteResponse {
        id: quote.id.to_owned(),
        text: quote.text.to_owned(),
        author_id: quote.author_id.to_owned(),
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_quote_short_response() {
        let quote = Quote {
            id: 1,
            text: String::from("Some wise thot."),
            author_id: 5,
        };

        let quote_response = make_quote_short_response(&quote);

        assert_eq!(*quote_response.get("status").unwrap(), "success");
        let data = quote_response.get("data").unwrap().get("quote").unwrap();
        // print!("{:?}", data);
        assert_eq!(data.get("text").unwrap(), "Some wise thot.");
        assert_eq!(data.get("id").unwrap(), 1);
        assert_eq!(data.get("author_id").unwrap(), 5);
    }

    #[test]
    fn test_make_quote_response() {
        let quote = QuoteWithAuthor {
            id: 1,
            text: String::from("Some wise thot."),
            author_id: 5,
            author_name: String::from("John Doe"),
        };

        let quote_response = make_quote_response(&quote);

        assert_eq!(*quote_response.get("status").unwrap(), "success");
        let data = quote_response.get("data").unwrap().get("quote").unwrap();
        // print!("{:?}", data);
        assert_eq!(data.get("text").unwrap(), "Some wise thot.");
        assert_eq!(data.get("id").unwrap(), 1);
        assert_eq!(data.get("author_id").unwrap(), 5);
        assert_eq!(data.get("author_name").unwrap(), "John Doe");
    }
}
