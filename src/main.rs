use actix_web::{web, App, HttpServer};

use crate::routes::write::write_route;
use crate::write_queue::start_global_write_system;

mod structs;
mod routes;
mod write_queue;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let global_tx = start_global_write_system();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(global_tx.clone()))
            .route("/write", web::post().to(write_route))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
