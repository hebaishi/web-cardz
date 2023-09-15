use actix_files as fs;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{App, HttpServer};
    let args: Vec<String> = env::args().collect();
    let root_dir: String = args[1].clone();
    HttpServer::new(move|| App::new().service(fs::Files::new("/", &root_dir).show_files_listing()))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
