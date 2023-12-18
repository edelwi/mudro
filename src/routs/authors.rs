use crate::crud::author::{
    create_author, delete_author, read_author, read_authors, read_random_author, update_author,
};
use crate::models::author::{Author, AuthorCreate, AuthorResponse, AuthorUpdate};
use crate::models::parameter::LimitOffset;
use crate::AppState;
use actix_web::{delete, get, patch, post, web, HttpResponse, Responder};
use serde_json;

fn make_author_response(author: &Author) -> serde_json::Value {
    let author_response = serde_json::json!(
    {
        "status": "success",
        "data": serde_json::json!({
            "author": author_to_response(&author)
        }
        )});
    return author_response;
}

#[post("/authors")]
pub async fn new_author(
    data: web::Data<AppState>,
    body: web::Json<AuthorCreate>,
) -> impl Responder {
    let new_author = create_author(&body, &data.db).await;
    match new_author {
        Ok(new_author) => {
            let author_response = make_author_response(&new_author);

            return HttpResponse::Ok().json(author_response);
        }
        Err(error) => {
            // match error.kind() {

            // }
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

#[patch("/authors/{id}")]
pub async fn upd_author(
    data: web::Data<AppState>,
    id: web::Path<(String,)>,
    body: web::Json<AuthorUpdate>,
) -> impl Responder {
    let (id,) = id.into_inner();

    let author_id = match id.parse() {
        Ok(author_id) => author_id,
        Err(e) => {
            return HttpResponse::BadRequest()
                .json(serde_json::json!({"status": "error","message": format!("{:?}", e)}));
        }
    };

    let updated_author = update_author(&body, author_id, &data.db).await;
    match updated_author {
        Ok(updated_author) => {
            let author_response = make_author_response(&updated_author);
            return HttpResponse::Ok().json(author_response);
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

#[get("/authors/random")]
pub async fn get_random_author(data: web::Data<AppState>) -> impl Responder {
    let result = read_random_author(&data.db).await;

    match result {
        Ok(author) => {
            let author_response = make_author_response(&author);

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

#[get("/authors/{id}")]
pub async fn get_author(data: web::Data<AppState>, id: web::Path<(String,)>) -> impl Responder {
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
            let author_response = make_author_response(&author);

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

#[delete("/authors/{id}")]
pub async fn del_author(data: web::Data<AppState>, id: web::Path<(String,)>) -> impl Responder {
    let (id,) = id.into_inner();

    let author_id = match id.parse() {
        Ok(author_id) => author_id,
        Err(e) => {
            return HttpResponse::InternalServerError()
                .json(serde_json::json!({"status": "error","message": format!("{:?}", e)}));
        }
    };

    let result = delete_author(author_id, &data.db).await;

    match result {
        Ok(author) => {
            let author_response = make_author_response(&author);

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

#[get("/authors")]
pub async fn get_authors(
    data: web::Data<AppState>,
    params: web::Query<LimitOffset>,
) -> impl Responder {
    let limit = params.limit.unwrap_or(100) as i32;
    let offset = params.offset.unwrap_or(0) as i32;

    let result = read_authors(&data.db, offset, limit).await;
    // let total_count_result = total_authors(&data.db).await;

    // let total_count = match total_count_result {
    //     Ok(total_count) => total_count,
    //     Err(e) => {
    //         return HttpResponse::InternalServerError()
    //             .json(serde_json::json!({"status": "error","message": format!("{:?}", e)}));
    //     }
    // };

    match result {
        Ok(authors) => {
            let authors_responses = authors
                .into_iter()
                .map(|author| author_to_response(&author))
                .collect::<Vec<AuthorResponse>>();
            let authors_response = serde_json::json!(
                {
                    "status": "success",
                    "data": authors_responses,
                    // "meta": {
                    //     "totalCount": total_count as u64
                    // }
                }
            );

            return HttpResponse::Ok().json(authors_response);
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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_author_response() {
        let author = Author {
            id: 1,
            author_name: String::from("John Doe"),
        };

        let author_response = make_author_response(&author);

        assert_eq!(*author_response.get("status").unwrap(), "success");
        let data = author_response.get("data").unwrap().get("author").unwrap();
        // print!("{:?}", data);
        assert_eq!(data.get("author_name").unwrap(), "John Doe");
        assert_eq!(data.get("id").unwrap(), 1);
    }

}