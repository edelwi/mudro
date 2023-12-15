
use crate::crud::quote::{
    read_random_quote_with_author, create_quote, update_quote, read_quote, delete_quote, read_quotes};
use crate::models::quote::{
    Quote, QuoteCreate, QuoteUpdate, QuoteWithAuthor, QuoteWithAuthorResponse, QuoteResponse};
use crate::models::parameter::LimitOffset;
use crate::AppState;
use actix_web::error::HttpError;
use actix_web::{delete, get, patch, post, web, App, HttpResponse, HttpServer, Responder};
use std::error::Error;

#[post("/quotes")]
pub async fn new_quote(
    data: web::Data<AppState>,
    body: web::Json<QuoteCreate>,
) -> impl Responder {
    println!("POST /quotes {:?}", &body);

    let new_quote = create_quote(&body, &data.db).await;
    match new_quote {
        Ok(new_quote) => {
            let quote_response = serde_json::json!(
            {
                "status": "success",
                "data": serde_json::json!({
                    "quote": quote_short_to_response(&new_quote)
                }
                )});

            return HttpResponse::Ok().json(quote_response);
        }
        Err(error) => {
            // if e.contains("Duplicate entry") {
            //     return HttpResponse::BadRequest().json(
            //     serde_json::json!({"status": "fail", "message": "Author already exists"}),
            // );
            // }

            return HttpResponse::InternalServerError()
                .json(serde_json::json!({"status": "error", "message": format!("{:?}", error)}));
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
            return HttpResponse::BadRequest()
                .json(serde_json::json!({"status": "error","message": format!("{:?}", e)}));
        }
    };

    println!("PUT /quotes/{} {:?}", quote_id, &body);

    let updated_quote = update_quote(&body, quote_id, &data.db).await;
    match updated_quote {
        Ok(updated_quote) => {
            let quote_response = serde_json::json!(
            {
                "status": "success",
                "data": serde_json::json!({
                    "quote": quote_short_to_response(&updated_quote)
                }
                )});

            return HttpResponse::Ok().json(quote_response);
        }
        Err(error) => {
            // if e.contains("Duplicate entry") {
            //     return HttpResponse::BadRequest().json(
            //     serde_json::json!({"status": "fail", "message": "Quote already exists"}),
            // );
            // }

            return HttpResponse::InternalServerError()
                .json(serde_json::json!({"status": "error", "message": format!("{:?}", error)}));
        }
    }
}

#[get("/quotes/random")]
pub async fn get_random_quote(data: web::Data<AppState>) -> impl Responder {
    println!("GET /quotes/random");

    let result = read_random_quote_with_author(&data.db).await;

    match result {
        Ok(quote) => {
            let quote_response = serde_json::json!(
            {
                "status": "success",
                "data": serde_json::json!({
                    "quote": quote_to_response(&quote)
                }
                )});

            return HttpResponse::Ok().json(quote_response);
        }
        // Err(sqlx::Error::RowNotFound) => {
        //     return HttpResponse::NotFound().json(
        //     serde_json::json!({"status": "fail","message": format!("Quote with ID: {} not found", quote_id)}),
        //     );
        // }
        Err(e) => {
            return HttpResponse::InternalServerError()
                .json(serde_json::json!({"status": "error","message": format!("{:?}", e)}));
        }
    };
}

#[get("/quotes/{id}")]
pub async fn get_quote(data: web::Data<AppState>, id: web::Path<(String,)>) -> impl Responder {
    let (id,) = id.into_inner();

    let quote_id = match id.parse() {
        Ok(quote_id) => quote_id,
        Err(e) => {
            return HttpResponse::InternalServerError()
                .json(serde_json::json!({"status": "error","message": format!("{:?}", e)}));
        }
    };
    println!("GET /quotes/{}", quote_id);

    let result = read_quote(quote_id, &data.db).await;

    match result {
        Ok(quote) => {
            let quote_response = serde_json::json!(
            {
                "status": "success",
                "data": serde_json::json!({
                    "quote": quote_short_to_response(&quote)
                }
                )});

            return HttpResponse::Ok().json(quote_response);
        }
        // Err(sqlx::Error::RowNotFound) => {
        //     return HttpResponse::NotFound().json(
        //     serde_json::json!({"status": "fail","message": format!("Quote with ID: {} not found", quote_id)}),
        //     );
        // }
        Err(e) => {
            return HttpResponse::InternalServerError()
                .json(serde_json::json!({"status": "error","message": format!("{:?}", e)}));
        }
    };
}

#[delete("/quotes/{id}")]
pub async fn del_quote(data: web::Data<AppState>, id: web::Path<(String,)>) -> impl Responder {
    let (id,) = id.into_inner();

    let quote_id = match id.parse() {
        Ok(quote_id) => quote_id,
        Err(e) => {
            return HttpResponse::InternalServerError()
                .json(serde_json::json!({"status": "error","message": format!("{:?}", e)}));
        }
    };
    println!("DELETE /quotes/{}", quote_id);

    let result = delete_quote(quote_id, &data.db).await;

    match result {
        Ok(quote) => {
            let quote_response = serde_json::json!(
            {
                "status": "success",
                "data": serde_json::json!({
                    "quote": quote_short_to_response(&quote)
                }
                )});

            return HttpResponse::Ok().json(quote_response);
        }
        // Err(sqlx::Error::RowNotFound) => {
        //     return HttpResponse::NotFound().json(
        //     serde_json::json!({"status": "fail","message": format!("Quote with ID: {} not found", quote_id)}),
        //     );
        // }
        Err(e) => {
            return HttpResponse::InternalServerError()
                .json(serde_json::json!({"status": "error","message": format!("{:?}", e)}));
        }
    };
}

#[get("/quotes")]
pub async fn get_quotes(
    data: web::Data<AppState>,
    params: web::Query<LimitOffset>,
) -> impl Responder {
    println!("GET /quotes {:?}", params);
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
        // Err(sqlx::Error::RowNotFound) => {
        //     return HttpResponse::NotFound().json(
        //     serde_json::json!({"status": "fail","message": format!("Quote with ID: {} not found", quote_id)}),
        //     );
        // }
        Err(e) => {
            return HttpResponse::InternalServerError()
                .json(serde_json::json!({"status": "error","message": format!("{:?}", e)}));
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
