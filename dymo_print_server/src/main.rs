use actix_web::middleware::Logger;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use env_logger::Env;
use serde::{Deserialize};
use structopt::StructOpt;
use std::net::{Ipv4Addr, SocketAddr};
//use serde_urlencoded;

use dymo_print::picture;
use dymo_print::fonts as dymofonts;

#[macro_use]
extern crate log;

#[derive(Debug, StructOpt)]
#[structopt(about = "A very simple application to use a Dymo printer remotely.")]
struct Opt {
    /// Set port
    #[structopt(short = "p", long = "port", default_value = "8080")]
    port: u16,

    /// Set address
    #[structopt(short = "a", long = "address", default_value = "127.0.0.1")]
    address: Ipv4Addr,
}

#[derive(Deserialize, Debug)]
struct LabelData {
    text: String,
    font: String,
}

//#[derive(Deserialize, Debug)]
//struct ImageFormData {
//    #[serde(deserialize_with = "serde_urlencoded::from_str")]
//    label_image: Vec<u8>,
//}

//#[get("/preview/image/{image}")]
//async fn preview_image(param: web::Path<Vec<u8>>) -> impl Responder {
//    let result = handle_preview_image(&param.into_inner());
//    match result {
//        Ok(img) => HttpResponse::Ok().content_type("image/png").body(img),
//        Err(err) => error_response(err),
//    }
//}

#[get("/preview/text/{font}/{text}")]
async fn preview_text(data: web::Path<LabelData>) -> impl Responder {
    debug!("Path Data: {:?}!", data);
    let result = handle_preview_text(&data);
    match result {
        Ok(img) => HttpResponse::Ok().content_type("image/png").body(img),
        Err(err) => error_response(err),
    }
}

#[post("/print/text")]
async fn print_text(form: web::Form<LabelData>) -> impl Responder {
    debug!("Form Data: {:?}!", form);
    let result = handle_print_text(&form);
    match result {
        Ok(_) => HttpResponse::Ok()
            .content_type("text/plain; charset=utf-8")
            .body("Go get your label!"),
        Err(err) => error_response(err),
    }
}

#[get("/fonts")]
async fn fonts() -> impl Responder {
    let fonts = dymofonts::get_fonts().unwrap();
    let fonts_json = serde_json::to_string(&fonts).unwrap();
    HttpResponse::Ok()
        .content_type("application/json; charset=utf-8")
        .body(fonts_json)
}

#[get("/")]
async fn text() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../static/text.html"))
}

//#[get("/image")]
//async fn image() -> impl Responder {
//    HttpResponse::Ok()
//        .content_type("text/html; charset=utf-8")
//        .body(include_str!("../static/image.html"))
//}

#[get("/static/site.css")]
async fn css() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/css; charset=utf-8")
        .body(include_str!("../static/site.css"))
}

fn handle_print_text(label_data: &LabelData) -> Result<(), Box<dyn std::error::Error>> {
    info!("label data: {:?}", label_data);
    let bw_pic = picture::create_bw_image(&label_data.text, &label_data.font, 128)?;

    dymo_print::print_label(&bw_pic)
}

fn handle_preview_text(label_data: &LabelData) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    info!("label data: {:?}", label_data);
    let bw_pic = picture::create_bw_image(&label_data.text, &label_data.font, 128)?;

    picture::encode_png(&bw_pic)
}

//fn handle_preview_image(label_image: &Vec<u8>) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
//    //info!("label text: {}", label_text);
//    let bw_pic = picture::convert_memory_bw_image(&label_image, 128)?;
//
//    picture::encode_png(&bw_pic)
//}

fn error_response(err: Box<dyn std::error::Error>) -> HttpResponse {
    error!("{}", err);
    HttpResponse::InternalServerError().body(err.to_string())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let opt = Opt::from_args();
    let bind_address = SocketAddr::from((opt.address, opt.port));

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .service(css)
            .service(text)
            .service(fonts)
//            .service(image)
            .service(preview_text)
            .service(print_text)
    })
    .bind(bind_address)?
    .run()
    .await
}
