use actix_web::http::header::ContentType;
use actix_web::http::StatusCode;
use actix_web::HttpResponse;
use actix_web_flash_messages::IncomingFlashMessages;
use std::fmt::Write;

pub async fn publish_newsletter_form(flash_messages: IncomingFlashMessages) -> HttpResponse {
    let mut msg_html = String::new();
    for m in flash_messages.iter() {
        write!(msg_html, "<p><i>{}</i></p>", m.content()).unwrap();
    }

    let html_body = format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta http-equiv="content-type" content="text/html; charset=utf-8">
    <title>Submit Newsletter</title>
</head>
<body>
    {msg_html}
    <form action="/admin/newsletters" method="post">
        <label>
            Title
            <br />
            <input type="text" name="title" placeholder="Newsletter title" />
        </label>
        <br />
        <label>
            HTML Content
            <br />
            <textarea name="html_content" placeholder="Newsletter content as HTML"></textarea>
        </label>
        <br />
        <label>
            Text Content
            <br />
            <textarea name="text_content" placeholder="Newsletter content as plain text"></textarea>
        </label>
        <br />
        <button type="submit">Submit newsletter</button>
    </form>
    <p><a href="/admin/dashboard">&lt;- Back</a></p>
</body>
</html>"#
    );

    HttpResponse::build(StatusCode::OK)
        .content_type(ContentType::html())
        .body(html_body)
}
