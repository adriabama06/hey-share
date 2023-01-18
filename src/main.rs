mod config;
mod files;

use tokio::time::{interval, Duration};

use actix_multipart::Multipart;
use actix_files::NamedFile;
use actix_web::{
    App, HttpServer,

    get, post, web,
    
    Responder, HttpResponse, HttpRequest
};

use futures_util::TryStreamExt as _;

use std::{path::Path, io::Write, fs};

#[get("/")]
async fn base() -> impl Responder {
    NamedFile::open("404.html")
}

#[get("/{path}")]
async fn index(path: web::Path<String>) -> impl Responder {
    let objective = String::from(path.as_str());
    
    unsafe {
        if files::FILESROUTE.contains_key(&objective) {
            let objective_filename = files::FILESROUTE.get(&objective).unwrap();

            return NamedFile::open(
                Path::new(&config::CONFIG.files).join(objective_filename)
            );
        }
    }

    return NamedFile::open("404.html");
}

#[post("/upload")]
async fn upload(mut payload: Multipart) -> impl Responder {
    while let Some(mut field) = payload.try_next().await.unwrap() {
        let content_disposition = field.content_disposition();

        let filename = content_disposition.get_filename().unwrap();
        unsafe {
            let filepath = Path::new(&config::CONFIG.files).join(filename);

            let mut f = web::block(|| std::fs::File::create(filepath)).await.unwrap().unwrap();

            // Field in turn is stream of *Bytes* object
            while let Some(chunk) = field.try_next().await.unwrap() {
                // filesystem operations are blocking, we have to use threadpool
                f = web::block(move || f.write_all(&chunk).map(|_| f)).await.unwrap().unwrap();
            }
        }
    }

    HttpResponse::Ok()
}

#[post("/route")]
async fn route(req: HttpRequest, item: web::Json<files::RouteFile>) -> impl Responder {
    unsafe {
        if req.headers().get("secret").unwrap().to_str().unwrap().to_string() != config::CONFIG.secret {
            return HttpResponse::Ok().body("Invalid secret");
        }
    }

    if item.url == "" || item.file == "" {
        return HttpResponse::Ok().body("Content not completed");
    }

    unsafe {
        fs::File::create(
            Path::new(&config::CONFIG.routes)
            .join(&item.url)
        )
        .unwrap()
        .write_all(serde_json::to_string(&item).unwrap().as_str().as_bytes())
        .unwrap();
    }

    HttpResponse::Ok().body("Route created")
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
                files::load_all(String::from(config::CONFIG.routes.clone()));
            }
        });

        files::load_all(String::from(config::CONFIG.routes.clone()));

        HttpServer::new(|| {
                App::new()
                .service(base)
                .service(index)
                .service(upload)
                .service(route)
                .service(echo)
            }
        )
        .bind(("0.0.0.0", config::CONFIG.port))?
        .run()
        .await
    }
}