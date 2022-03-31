use actix_web::HttpResponse;
use serde_json::json;

pub async fn not_found() -> HttpResponse {
    let err_response = json!({
        "error": "not_found",
        "description": "Requested route does not exist"
    });

    return HttpResponse::NotFound().json(err_response);
}
