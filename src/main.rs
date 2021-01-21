use std::env;
use std::net::Ipv4Addr;
use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Responder, middleware::Logger, web::{self, Bytes}};
use env_logger::Env;
mod functions;

async fn default_service(req: HttpRequest, bytes: Bytes) -> impl Responder {
    println!("{} {}", req.method(), req.path());
    println!("{:?}", String::from_utf8(bytes.to_vec()).unwrap());
    HttpResponse::Ok()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port_key = "FUNCTIONS_CUSTOMHANDLER_PORT";
    let port: u16 = match env::var(port_key) {
        Ok(val) => val.parse().expect("Custom Handler port is not a number!"),
        Err(_) => 3000,
    };

    env_logger::Builder::from_env(Env::default().default_filter_or("info")).build();

    HttpServer::new(|| {
        App::new()
            .app_data(
                // Json extractor configuration for this resource.
                web::JsonConfig::default()
                    .content_type(|mime| mime.subtype() == mime::JSON)
            )
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .service(functions::greet_handler)
            .service(functions::my_timer)
            .default_service(web::route().to(default_service))
    })
    .bind((Ipv4Addr::UNSPECIFIED, port))?
    .run()
    .await
}