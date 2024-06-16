use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::path::Path;

async fn serve_file(path: web::Path<(String, String, String)>) -> impl Responder {
    let (dict, subdir, filename) = path.into_inner();
    let filepath = format!("./static/{}/{}/{}", dict, subdir, filename);

    println!("Attempting to serve file: {}", filepath); // Debugging information

    if Path::new(&filepath).exists() {
        match std::fs::read(&filepath) {
            Ok(data) => HttpResponse::Ok()
                .append_header(("Content-Type", "audio/mpeg"))
                .body(data),
            Err(e) => {
                println!("Error reading file: {}", e); // More debugging information
                HttpResponse::InternalServerError().body("Error reading file")
            },
        }
    } else {
        println!("File not found: {}", filepath); // More debugging information
        HttpResponse::NotFound().body("File not found")
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/{dict}/{subdir}/{filename}", web::get().to(serve_file))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}