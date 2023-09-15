use actix_files as fs;
use std::env;
use std::{
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
    time::Instant,
};

use actix::*;
use actix_web::{middleware::Logger, web, Error, HttpRequest, HttpResponse, Responder};
use actix_web_actors::ws;

mod server;
mod session;
/// Entry point for our websocket route
async fn chat_route(
    req: HttpRequest,
    stream: web::Payload,
    srv: web::Data<Addr<server::ChatServer>>,
) -> Result<HttpResponse, Error> {
    ws::start(
        session::WsChatSession {
            id: 0,
            hb: Instant::now(),
            room: "main".to_owned(),
            name: None,
            addr: srv.get_ref().clone(),
        },
        &req,
        stream,
    )
}

async fn get_count(count: web::Data<AtomicUsize>) -> impl Responder {
    let current_count = count.load(Ordering::SeqCst);
    format!("Visitors: {current_count}")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let app_state = Arc::new(AtomicUsize::new(0));
    let server = server::ChatServer::new(app_state.clone()).start();
    log::info!("starting HTTP server at http://localhost:8080");

    use actix_web::{App, HttpServer};
    let args: Vec<String> = env::args().collect();
    let root_dir: String = args[1].clone();
    HttpServer::new(move || {
        App::new()
            .service(fs::Files::new("/", &root_dir).show_files_listing())
            .app_data(web::Data::from(app_state.clone()))
            .app_data(web::Data::new(server.clone()))
            .route("/count", web::get().to(get_count))
            .route("/ws", web::get().to(chat_route))
            .wrap(Logger::default())
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
