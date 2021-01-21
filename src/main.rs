use std::env;
use std::net::Ipv4Addr;
use actix_web::{App, HttpServer, middleware::Logger};
use env_logger::Env;
mod functions;

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
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .service(functions::greet_handler)
    })
    .bind((Ipv4Addr::UNSPECIFIED, port))?
    .run()
    .await
}