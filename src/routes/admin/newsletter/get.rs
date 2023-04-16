use actix_web::{http::header::ContentType, HttpResponse};
use actix_web_flash_messages::IncomingFlashMessages;
use std::fmt::Write;

pub async fn publish_newsletter_form(
    flash_message: IncomingFlashMessages,
) -> Result<HttpResponse, actix_web::Error> {
    let mut msg_html = String::new();

    for m in flash_message.iter() {
        writeln!(msg_html, "<p><i>{}</i></p>", m.content()).unwrap();
    }

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(format!(
            r#"<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="UTF-8" />
    <meta http-equiv="X-UA-Compatible" content="IE=edge" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <meta http-equiv="content-type" content="text/html; charset=utf-8" />
    <title>Send newsletter issue</title>
  </head>
  <body>
    {msg_html}
    <form action="/admin/newsletters" method="post">
      <label>
        Title
        <input type="text" placeholder="Enter title" name="title" />
      </label>

      <label>
        Content
        <textarea
          name="content"
          cols="30"
          rows="10"
          placeholder="Enter content"
        ></textarea>
      </label>

      <button type="submit">Login</button>
    </form>
  </body>
</html>"#
        )))
}
