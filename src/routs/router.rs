use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use crate::crud::author::{read_author};
use crate::models::author::{Author, AuthorResponse};
use crate::AppState;
use std::error::Error;

#[get("/author/{id}")]
pub async fn get_author(
    data: web::Data<AppState>,
    id: web::Path<(String,)>
) -> impl Responder {
    let (id,) = id.into_inner();

    let author_id = match id.parse() {
        Ok(author_id) => author_id,
        Err(e) => {
            return HttpResponse::InternalServerError()
                .json(serde_json::json!({"status": "error","message": format!("{:?}", e)}));
        }
    };
     

    let result = read_author(author_id, &data.db).await;

    match result {
        Ok(author) => {
            let author_response = serde_json::json!(
                {
                    "status": "success",
                    "data": serde_json::json!({
                        "author": author_to_response(&author)
                    }
                    )});

            return HttpResponse::Ok().json(author_response);
        }
        // Err(sqlx::Error::RowNotFound) => {
        //     return HttpResponse::NotFound().json(
        //     serde_json::json!({"status": "fail","message": format!("Author with ID: {} not found", author_id)}),
        //     );
        // }
        Err(e) => {
            return HttpResponse::InternalServerError()
                .json(serde_json::json!({"status": "error","message": format!("{:?}", e)}));
        }
    };
}

fn author_to_response(author: &Author) -> AuthorResponse {
    AuthorResponse {
        id: author.id.to_owned(),
        author_name: author.author_name.to_owned(),
    }
}

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api")
        .service(get_author);

    conf.service(scope);
}