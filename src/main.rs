#[macro_use]
extern crate log;

use actix_web::{web, App, HttpRequest, HttpServer, Responder};
use actix_web::middleware::Logger as actix_Logger;
use std::thread;
use std::time::{Duration};
use std::env;

extern crate slog_envlogger;
extern crate slog_stdlog;
extern crate rand;

use rand::Rng;

async fn greet(req: HttpRequest) -> impl Responder {
    let mut rng = rand::thread_rng();
    let n: u8 = rng.gen();
    info!("Taking request {}", n);
    let name = req.match_info().get("name").unwrap_or("World");
    thread::sleep(Duration::from_secs(5));
    info!("Answering {}", n);
    format!("Hello {}!", &name)
}

fn main() {
    if env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "actix_web=info,t_rust=info");
    }
    let _guard = slog_envlogger::init().unwrap();
    actix().unwrap()
}

#[actix_rt::main]
async fn actix() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(actix_Logger::new(
                "API %a %r %s %b %{Referer}i %{User-Agent}i %T",
            ))
            .route("/", web::get().to(greet))
            .route("/{name}", web::get().to(greet))
    })
    .client_timeout(0)
    .bind("127.0.0.1:8063")?
    .run()
    .await
}