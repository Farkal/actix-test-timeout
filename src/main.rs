#[macro_use]
extern crate log;

use actix_web::{web, App, HttpRequest, HttpServer, Responder};
use actix_web::middleware::Logger as actix_Logger;
use std::env;

extern crate slog_envlogger;
extern crate slog_stdlog;
extern crate rand;

use rand::Rng;

const SIZE: usize = 100_000_000;

fn heavy_process() -> Result<(), ()> {
    // let mut sub_results = vec![0.0f64; SIZE];
    let mut divisor = 1.0;
    let mut result: f64;
    let mut tmp = 0.0;

    for i in 0..SIZE {
        tmp = 4.0 / divisor;
        divisor += 2.0;
    }

    result = 0.0;

    for i in 0..SIZE {
        if i % 2 == 0 {
            result += tmp;
        }
        else {
            result -= tmp;
        }
    }
    info!("{}", result);
    Ok(())
    // thread::sleep(Duration::from_secs(5));
}

async fn greet(req: HttpRequest) -> impl Responder {
    let mut rng = rand::thread_rng();
    let n: u8 = rng.gen();
    info!("Taking request {}", n);
    let name = req.match_info().get("name").unwrap_or("World");
    info!("Answering {}", n);
    web::block(heavy_process).await.expect("FAIL");
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