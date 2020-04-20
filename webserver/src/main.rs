use actix_web::{get, post, web, App, HttpServer, HttpResponse, Responder};
use actix_web::middleware::Logger;
use env_logger::Env;

use dymo_label::picture;

#[macro_use]
extern crate log;

#[get("/preview/text/{label}")]
async fn text(param: web::Path<String>) -> impl Responder {
    let result = handle_text(param.to_string());
    match result {
        Ok(image) => {
            HttpResponse::Ok()
                .content_type("image/png")
                .body(image)
        },
        Err(_) => {
            HttpResponse::InternalServerError().finish()
        },
    }
}

fn handle_text(label_text: String) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    //let label_text = "foo";
    info!("text: {}", label_text);
    let pic = picture::create_image(&label_text, "Ubuntu")?;

    let bw_pic = picture::convert_to_bw(&pic, 128)?;
    let encoded_img = picture::encode_png(&bw_pic)?;
    //bw_pic.save("preview.png")?;
    Ok(encoded_img)
}


#[actix_rt::main]
async fn main() -> std::io::Result<()> {
	env_logger::from_env(Env::default().default_filter_or("info")).init();

    HttpServer::new(|| App::new()
                    .wrap(Logger::default())
                    .wrap(Logger::new("%a %{User-Agent}i"))
                    .service(text))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
