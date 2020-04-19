use actix_web::{get, web, App, HttpServer, Responder};
use actix_web::middleware::Logger;
use env_logger::Env;

#[macro_use]
extern crate log;

#[get("/{id}/{name}/index.html")]
async fn index(info: web::Path<(u32, String)>) -> impl Responder {
    info!("{:?}", info);
    format!("Hello {}! id:{}", info.1, info.0)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
	env_logger::from_env(Env::default().default_filter_or("info")).init();

    HttpServer::new(|| App::new()
                    .wrap(Logger::default())
                    .wrap(Logger::new("%a %{User-Agent}i"))
                    .service(index))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
