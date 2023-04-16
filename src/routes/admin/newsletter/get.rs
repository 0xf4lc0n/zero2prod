use actix_web::{http::header::ContentType, web, HttpResponse};

use crate::authentication::UserId;

pub async fn send_newsletter_issue_form(
    _user_id: web::ReqData<UserId>,
) -> Result<HttpResponse, actix_web::Error> {
    let html = include_str!("./send_newsletter.html");

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(html))
}
