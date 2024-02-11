use actix_web::HttpResponse;

pub async fn submit_newsletter_form() -> HttpResponse {
    HttpResponse::Ok().finish()
}
