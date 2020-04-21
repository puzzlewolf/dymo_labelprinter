use actix_web::middleware::Logger;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use env_logger::Env;
use serde::Deserialize;

use dymo_label::picture;

#[macro_use]
extern crate log;

#[derive(Deserialize, Debug)]
struct FormData {
    label_text: String,
}

#[post("/print/text/{label}")]
async fn print_text(param: web::Path<String>) -> impl Responder {
    let result = handle_print_text(param.to_string());
    match result {
        Ok(_) => HttpResponse::Ok().content_type("text/plain").body("Go get your label!"),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/preview/text/{label}")]
async fn preview_text(param: web::Path<String>) -> impl Responder {
    let result = handle_preview_text(param.to_string());
    match result {
        Ok(image) => HttpResponse::Ok().content_type("image/png").body(image),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[post("/form")]
async fn form(form: web::Form<FormData>) -> impl Responder {
    info!("Form Data: {:?}!", form);
    HttpResponse::Ok()
        .content_type("text/plain")
        .body(format!("Form Data: {:?}!", form))
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../static/index.html"))
}


fn handle_print_text(label_text: String) -> Result<(), Box<dyn std::error::Error>> {
    info!("text: {}", label_text);
    let pic = picture::create_image(&label_text, "Ubuntu")?;

    let bw_pic = picture::convert_to_bw(&pic, 128)?;
    dymo_label::print_label(&bw_pic)
}


fn handle_preview_text(label_text: String) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    info!("text: {}", label_text);
    let pic = picture::create_image(&label_text, "Ubuntu")?;

    let bw_pic = picture::convert_to_bw(&pic, 128)?;
    picture::encode_png(&bw_pic)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    env_logger::from_env(Env::default().default_filter_or("debug")).init();

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .service(index)
            .service(form)
            .service(preview_text)
            .service(print_text)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
