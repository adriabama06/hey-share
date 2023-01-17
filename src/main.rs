mod config;
mod files;

use once_cell::sync::Lazy;

use tokio::time::{interval, Duration};

use actix_files::NamedFile;
use actix_web::{
    App, HttpServer,

    get, post, web,
    
    Responder, HttpResponse
};

use std::path::Path;

static mut FILESROUTE: Lazy<files::RouteFiles> = Lazy::new(files::RouteFiles::default);

#[get("/")]
async fn base() -> impl Responder {
    NamedFile::open("404.html")
}

#[get("/{path}")]
async fn index(path: web::Path<String>) -> impl Responder {
    let objective = String::from(path.as_str());
    
    unsafe {
        if FILESROUTE.files.contains_key(&objective) {
            let objective_filename = FILESROUTE.files.get(&objective).unwrap();

            return NamedFile::open(
                Path::new(config::CONFIG.files).join(objective_filename)
            );
        }
    }

    return NamedFile::open("404.html");
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    config::load_config();

    unsafe {
        let mut interval = interval(Duration::from_secs(1));

        tokio::spawn(async move {
            loop {
                interval.tick().await;
                FILESROUTE.load_all(String::from(config::CONFIG.routes));
            }
        });

        FILESROUTE.load_all(String::from(config::CONFIG.routes));

        HttpServer::new(
            || App::new()
            .service(base)
            .service(index)
            .service(echo)
        )
        .bind(("0.0.0.0", config::CONFIG.port))?
        .run()
        .await
    }
}